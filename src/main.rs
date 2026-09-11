use clap::{Parser, Subcommand};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, net::SocketAddr, path::PathBuf, time::Duration};

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
    /// Print the connection descriptor consumed by launchers and cyb.
    Cyb,
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
            return Err("bind must be a loopback address with a nonzero port; the chaosnet bridge accepts unsigned writes".into());
        }
        if self.moniker.trim().is_empty() || self.moniker.chars().any(char::is_control) {
            return Err("moniker must be nonempty and contain no control characters".into());
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
        println!("{}", config_path.display());
        return Ok(());
    }
    let config: Config = toml::from_str(
        &fs::read_to_string(&config_path)
            .map_err(|e| format!("{}: {e}; create it with cyber init", config_path.display()))?,
    )?;
    config.validate()?;
    match cli.command {
        Command::Node => {
            // Hold the OS lock for the entire server lifetime. It releases on crash too.
            let lock = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open(home.join("node.lock"))?;
            lock.try_lock_exclusive()
                .map_err(|e| format!("node home already in use or cannot be locked: {e}"))?;
            soft3::node::run(home, &config.bind.to_string(), &config.moniker)?;
        }
        Command::Config => print!("{}", toml::to_string_pretty(&config)?),
        Command::Cyb => println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "schema": "cyber/connection/v1",
                "binary": "cyber",
                "version": env!("CARGO_PKG_VERSION"),
                "network": config.network,
                "rpc": config.rpc(),
                "home": fs::canonicalize(&home)?,
                "mode": "local-chaosnet",
                "status": "/status",
                "submit": "/v1/link",
                "cyb_command": format!("net set spacepussy-test {}", config.rpc()),
                "capabilities": {"graph": true, "replay": true, "peer_sync": false,
                    "authenticated_writes": false, "consensus": false, "joy_worker": false}
            }))?
        ),
        Command::Status { json } => {
            let body = ureq::get(&format!("{}/status", config.rpc()))
                .timeout(Duration::from_secs(5))
                .call()?
                .into_string()?;
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
