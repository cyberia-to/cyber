use clap::{Parser, Subcommand};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
    net::SocketAddr,
    path::{Path, PathBuf},
};

mod connection;

#[derive(Parser)]
#[command(name = "cyber", version, about = "Cyber local node over soft3")]
struct Cli {
    /// Node directory; defaults to $CYBER_HOME or ~/.cyber/spacepussy-test.
    #[arg(long, global = true)]
    home: Option<PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a configuration. Existing configuration is preserved.
    Init {
        #[arg(long, default_value = "127.0.0.1:7780")]
        bind: SocketAddr,
        #[arg(long, default_value = "cyber-local")]
        moniker: String,
    },
    /// Run the configured local chaosnet node in the foreground.
    #[command(alias = "start")]
    Node,
    /// Read the node's status (this command does not replicate state).
    Status {
        #[arg(long)]
        json: bool,
    },
    /// Print the effective configuration.
    Config,
    /// Print configuration; --live additionally observes the endpoint's native capabilities.
    Cyb {
        #[arg(long)]
        live: bool,
    },
    /// Offline storage operations. Stop the node before migration.
    Storage {
        #[command(subcommand)]
        command: StorageCommand,
    },
    /// Explicit offline activation of authenticated native publication.
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },
}

#[derive(Subcommand)]
enum StorageCommand {
    /// Import exact retained legacy log bytes into BBG; exact retries resume safely.
    ImportLegacy,
}

#[derive(Subcommand)]
enum AuthCommand {
    /// Require signed native writes and retire old writers. This upgrade is permanent.
    Enable {
        /// Import the retained legacy log first, under the same home lock.
        #[arg(long)]
        import_legacy: bool,
    },
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    version: u32,
    network: String,
    bind: SocketAddr,
    moniker: String,
}

impl Config {
    fn validate(&self) -> Result<(), String> {
        if self.version != 1 || self.network != "spacepussy-test" {
            return Err("supported config: version=1, network=spacepussy-test".into());
        }
        if !self.bind.ip().is_loopback() || self.bind.port() == 0 {
            return Err(
                "bind must be a loopback address with a nonzero port for this local node profile"
                    .into(),
            );
        }
        if self.moniker.trim().is_empty()
            || self.moniker.len() > 256
            || self.moniker.chars().any(char::is_control)
        {
            return Err("moniker must be 1..256 bytes and contain no control characters".into());
        }
        Ok(())
    }

    fn rpc(&self) -> String {
        format!("http://{}", self.bind)
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let home = cli
        .home
        .or_else(|| std::env::var_os("CYBER_HOME").map(PathBuf::from))
        .or_else(|| {
            std::env::var_os("HOME").map(|p| PathBuf::from(p).join(".cyber/spacepussy-test"))
        })
        .ok_or("set --home or CYBER_HOME")?;
    let config_path = home.join("config.toml");
    if let Command::Init { bind, moniker } = cli.command {
        let config = Config {
            version: 1,
            network: "spacepussy-test".into(),
            bind,
            moniker,
        };
        config.validate()?;
        let text = toml::to_string_pretty(&config)?;
        fs::create_dir_all(&home)?;
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&config_path)?;
        file.write_all(text.as_bytes())?;
        file.sync_all()?;
        #[cfg(unix)]
        fs::File::open(&home)?.sync_all()?;
        println!("{}", config_path.display());
        return Ok(());
    }
    let config: Config = toml::from_str(
        &read_config(&config_path)
            .map_err(|e| format!("{}: {e}; create it with cyber init", config_path.display()))?,
    )?;
    config.validate()?;
    match cli.command {
        Command::Node => {
            let _locks = lock_home(&home)?;
            soft3::node::run(home, &config.bind.to_string(), &config.moniker)?;
        }
        Command::Config => print!("{}", toml::to_string_pretty(&config)?),
        Command::Cyb { live } => println!(
            "{}",
            serde_json::to_string_pretty(&connection::descriptor(&home, &config, live)?)?
        ),
        Command::Storage {
            command: StorageCommand::ImportLegacy,
        } => {
            let _locks = lock_home(&home)?;
            println!("{}", serde_json::to_string_pretty(&run_import(&home)?)?);
        }
        Command::Auth {
            command: AuthCommand::Enable { import_legacy },
        } => {
            let _locks = lock_home(&home)?;
            let imported = if import_legacy {
                Some(run_import(&home)?)
            } else {
                None
            };
            let network = soft3::node::Node::open(home, config.moniker)?.enable_authentication()?;
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "schema": "cyber/authentication/1", "network": hex(&network),
                    "profile": connection::SIGNED_PROFILE, "authenticated": true,
                    "import": imported, "consensus_finality": false,
                }))?
            );
        }
        Command::Status { json } => {
            let body = connection::read_endpoint(&format!("{}/status", config.rpc()))?;
            let fields = parse_status(&body)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&fields)?);
            } else {
                print!("{body}");
            }
        }
        Command::Init { .. } => unreachable!(),
    }
    Ok(())
}

fn read_config(path: &Path) -> std::io::Result<String> {
    let file = fs::File::open(path)?;
    if !file.metadata()?.is_file() {
        return Err(std::io::Error::other("config must be a regular file"));
    }
    let mut text = String::new();
    file.take(16 * 1024 + 1).read_to_string(&mut text)?;
    if text.len() > 16 * 1024 {
        return Err(std::io::Error::other("config exceeds 16 KiB"));
    }
    Ok(text)
}

// Product node and both offline operations share the same lifetime locks.
// The second lock interoperates with `soft3 auth enable`; BBG also excludes
// independent processes that open the underlying store directly.
fn lock_home(home: &Path) -> Result<Vec<fs::File>, Box<dyn std::error::Error>> {
    let mut locks = Vec::new();
    for name in ["node.lock", "auth-upgrade.lock"] {
        let lock = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(home.join(name))?;
        lock.try_lock_exclusive().map_err(|e| format!("node home already in use or cannot be locked ({name}): {e}; stop the node before migration or authentication activation"))?;
        locks.push(lock);
    }
    Ok(locks)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn run_import(home: &Path) -> std::io::Result<serde_json::Value> {
    let report = soft3::node::import_legacy(home)?;
    Ok(
        serde_json::json!({"schema":"cyber/legacy-import/1", "events":report.events,
        "signals":report.signals, "height":report.height, "root":hex(&report.root),
        "source":hex(&report.source), "source_retained":true}),
    )
}

fn parse_status(body: &str) -> Result<serde_json::Value, String> {
    let mut lines = body.lines();
    if lines.next() != Some("---") {
        return Err("expected cybermark status".into());
    }
    let mut fields = serde_json::Map::new();
    let mut closed = false;
    for line in lines {
        if line == "---" {
            closed = true;
            break;
        }
        let (key, value) = line.split_once(':').ok_or("invalid status field")?;
        if fields
            .insert(key.trim().into(), value.trim().into())
            .is_some()
        {
            return Err("duplicate status field".into());
        }
    }
    if !closed
        || fields.get("particle").and_then(|v| v.as_str()) != Some("status")
        || fields.get("chain").and_then(|v| v.as_str()) != Some("spacepussy-test")
    {
        return Err("unexpected status document or network".into());
    }
    let height = fields
        .get("height")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or("invalid status height")?;
    let root = fields
        .get("bbg-root")
        .and_then(|v| v.as_str())
        .ok_or("missing state root")?;
    if root.len() != 64 || !root.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("invalid state root".into());
    }
    fields.insert("height".into(), height.into());
    Ok(fields.into())
}

fn main() {
    if let Err(e) = run(Cli::parse()) {
        eprintln!("cyber: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::parse_status;

    #[test]
    fn status_requires_a_complete_network_bound_document() {
        let valid = format!(
            "---\nparticle: status\nchain: spacepussy-test\nheight: 0\nbbg-root: {}\n---\n",
            "ab".repeat(32)
        );
        assert_eq!(parse_status(&valid).unwrap()["height"], 0);
        for invalid in [
            valid.replace("spacepussy-test", "bostrom"),
            valid.replace("height: 0", "height: 0\nheight: 1"),
            valid.replace(&"ab".repeat(32), "bad"),
            valid.trim_end_matches("---\n").to_string(),
            "ok".into(),
        ] {
            assert!(parse_status(&invalid).is_err(), "accepted {invalid}");
        }
    }
}
