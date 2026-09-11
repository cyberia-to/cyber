//! Release-readiness probes against an actual executable, using disposable homes.
//! Exit 1 means unmet product requirements; exit 2 means the audit could not run.
use serde_json::{json, Value};
use std::{
    error::Error,
    fs,
    net::TcpListener,
    path::{Path, PathBuf},
    process::{Child, Command, Output, Stdio},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

struct Fixture {
    home: tempfile::TempDir,
    binary: PathBuf,
    rpc: String,
}

impl Fixture {
    fn new(binary: &Path) -> Result<Self> {
        let home = tempfile::tempdir()?;
        let reservation = TcpListener::bind("127.0.0.1:0")?;
        let bind = reservation.local_addr()?.to_string();
        let fixture = Self {
            home,
            binary: binary.to_owned(),
            rpc: format!("http://{bind}"),
        };
        checked(fixture.command(&["init", "--bind", &bind])?)?;
        drop(reservation);
        Ok(fixture)
    }

    fn command(&self, args: &[&str]) -> Result<Output> {
        Ok(Command::new(&self.binary)
            .arg("--home")
            .arg(self.home.path())
            .args(args)
            .output()?)
    }

    fn start(&self) -> Result<Server> {
        Ok(Server(
            Command::new(&self.binary)
                .arg("--home")
                .arg(self.home.path())
                .arg("node")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()?,
        ))
    }

    fn status(&self) -> Result<Value> {
        Ok(serde_json::from_str(&checked(
            self.command(&["status", "--json"])?,
        )?)?)
    }

    fn link(&self, from: &str, to: &str, amount: u64) -> Result<Value> {
        let request = ureq::post(&format!("{}/v1/link", self.rpc))
            .timeout(Duration::from_secs(3))
            .send_json(json!({"neuron":"audit-neuron", "from":from, "to":to,
                "amount":amount, "valence":0}));
        let response = match request {
            Ok(response) | Err(ureq::Error::Status(_, response)) => response,
            Err(error) => return Err(error.into()),
        };
        let code = response.status();
        let body = response.into_string()?;
        Ok(json!({"http":code, "body":serde_json::from_str::<Value>(&body).unwrap_or(json!(body))}))
    }
}

struct Server(Child);
impl Server {
    fn ready(&mut self, rpc: &str) -> Result<bool> {
        let deadline = Instant::now() + Duration::from_secs(8);
        loop {
            if self.0.try_wait()?.is_some() {
                return Ok(false);
            }
            if ureq::get(&format!("{rpc}/health"))
                .timeout(Duration::from_millis(200))
                .call()
                .is_ok()
            {
                return Ok(true);
            }
            if Instant::now() >= deadline {
                return Err("readiness timed out".into());
            }
            std::thread::sleep(Duration::from_millis(25));
        }
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn checked(output: Output) -> Result<String> {
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned().into());
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn running(fixture: &Fixture) -> Result<Server> {
    let mut server = fixture.start()?;
    if !server.ready(&fixture.rpc)? {
        return Err("node exited before readiness".into());
    }
    Ok(server)
}

fn gate(name: &str, passed: bool, observed: Value) -> Value {
    json!({"gate":name, "passed":passed, "observed":observed})
}

fn audit(binary: &Path) -> Result<Vec<Value>> {
    let mut gates = Vec::new();
    {
        let fixture = Fixture::new(binary)?;
        let descriptor: Value = serde_json::from_str(&checked(fixture.command(&["cyb"])?)?)?;
        gates.push(gate(
            "embedded_joy",
            descriptor["capabilities"]["joy_worker"] == true,
            descriptor.clone(),
        ));
        let worker = fixture.command(&["worker", "status"])?;
        gates.push(gate(
            "worker_management",
            worker.status.success(),
            json!({
            "exit":worker.status.code(), "stderr":String::from_utf8_lossy(&worker.stderr)}),
        ));
        let config_path = fixture.home.path().join("config.toml");
        let config = fs::read_to_string(&config_path)?;
        fs::write(
            &config_path,
            config.replace("spacepussy-test", "audit-network-alpha"),
        )?;
        let configured = fixture.command(&["config"])?;
        gates.push(gate("additional_network_configuration", configured.status.success(), json!({
            "exit":configured.status.code(), "stderr":String::from_utf8_lossy(&configured.stderr),
            "scope":"Changing the existing network field; a full VM/OS/genesis descriptor is not implemented."})));
    }
    {
        let fixture = Fixture::new(binary)?;
        let server = running(&fixture)?;
        let receipt = fixture.link("hello", "cyber", 1)?;
        let before = fixture.status()?;
        drop(server);
        let _server = running(&fixture)?;
        let after = fixture.status()?;
        gates.push(gate(
            "normal_write_replay",
            receipt["body"]["ok"] == true
                && before["height"] == 1
                && before["height"] == after["height"]
                && before["bbg-root"] == after["bbg-root"],
            json!({"receipt":receipt,"before":before,"after":after}),
        ));
    }
    {
        let fixture = Fixture::new(binary)?;
        let server = running(&fixture)?;
        // A directory at the journal path reliably makes append fail, even as root.
        fs::create_dir(fixture.home.path().join("log"))?;
        let receipt = fixture.link("write-error", "must-survive", 1)?;
        let live = fixture.status()?;
        drop(server);
        let mut restarted = fixture.start()?;
        let ready = restarted.ready(&fixture.rpc)?;
        let replay = if ready {
            fixture.status()?
        } else {
            Value::Null
        };
        let refused = receipt["http"].as_u64().is_some_and(|s| s >= 400);
        gates.push(gate(
            "journal_error_refuses_ack_and_mutation",
            refused && live["height"] == 0,
            json!({"fault":"directory at journal path after startup", "receipt":receipt,
                "live":live,"restart_ready":ready,"replay":replay}),
        ));
    }
    {
        let fixture = Fixture::new(binary)?;
        let server = running(&fixture)?;
        fixture.link("truncation", "must-detect", 1)?;
        let before = fixture.status()?;
        drop(server);
        let path = fixture.home.path().join("log");
        let length = fs::metadata(&path)?.len();
        if length == 0 {
            return Err("no journal produced before truncation probe".into());
        }
        fs::OpenOptions::new()
            .write(true)
            .open(path)?
            .set_len(length - 1)?;
        let mut restarted = fixture.start()?;
        let ready = restarted.ready(&fixture.rpc)?;
        let after = if ready {
            fixture.status()?
        } else {
            Value::Null
        };
        let exit = restarted.0.try_wait()?;
        gates.push(gate(
            "truncated_journal_fails_closed",
            !ready && exit.is_some_and(|status| !status.success()),
            json!({
            "original_bytes":length, "truncated_bytes":length-1,"before":before,
            "restart_ready":ready,"exit":exit.and_then(|status| status.code()),"after":after}),
        ));
    }
    {
        let fixture = Fixture::new(binary)?;
        let _server = running(&fixture)?;
        let first = fixture.link("same-json", "retried", 1)?;
        let second = fixture.link("same-json", "retried", 1)?;
        let after = fixture.status()?;
        gates.push(gate("json_relay_retry_is_idempotent", after["height"] == 1,
            json!({"first":first,"second":second,"after":after,
                "scope":"Replay of the identical JSON body as after a lost response; bridge has no stable request identity."})));
    }
    {
        let fixture = Fixture::new(binary)?;
        let _server = running(&fixture)?;
        let receipt = fixture.link("zheng", "pussy", 1000000)?;
        let balance = ureq::get(&format!("{}/balance/audit-neuron", fixture.rpc))
            .timeout(Duration::from_secs(3))
            .call()?
            .into_string()?;
        let issued = balance
            .lines()
            .find_map(|line| line.strip_prefix("balance: "))
            .ok_or("balance document lacks balance")?
            .parse::<u64>()?;
        gates.push(gate(
            "unproved_reward_rejected",
            issued == 0,
            json!({"unsigned_unproved_request_amount":1000000,"receipt":receipt,"issued":issued}),
        ));
    }
    Ok(gates)
}

fn run() -> Result<bool> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 2 {
        return Err("usage: readiness BINARY REPORT.json".into());
    }
    let binary = fs::canonicalize(&args[0])?;
    let original_binary = fs::read(&binary)?;
    let checksum_output = checked(
        Command::new("shasum")
            .args(["-a", "256"])
            .arg(&binary)
            .output()?,
    )?;
    let sha256 = checksum_output
        .split_whitespace()
        .next()
        .ok_or("missing binary checksum")?;
    let gates = audit(&binary)?;
    if fs::read(&binary)? != original_binary {
        return Err("binary changed during audit; rerun on an immutable artifact".into());
    }
    let failed = gates.iter().filter(|g| g["passed"] == false).count();
    let report = json!({"schema":"cyber/readiness-audit/v1", "binary":binary,
        "sha256":sha256,
        "unix_time":SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
        "version":checked(Command::new(&binary).arg("--version").output()?)?.trim(),
        "scope":"Selected architecture and failure probes; passing these alone does not establish production readiness.",
        "checked_gates_pass":failed == 0,"failed":failed,"checked":gates.len(),"gates":gates});
    if let Some(parent) = Path::new(&args[1])
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)?;
    }
    fs::write(&args[1], serde_json::to_string_pretty(&report)? + "\n")?;
    for entry in report["gates"].as_array().ok_or("missing gates")? {
        println!(
            "{} {}",
            if entry["passed"] == true {
                "PASS"
            } else {
                "FAIL"
            },
            entry["gate"].as_str().unwrap_or("unknown")
        );
    }
    println!(
        "{failed} unmet requirements; report: {}",
        Path::new(&args[1]).display()
    );
    Ok(failed == 0)
}

fn main() {
    match run() {
        Ok(true) => {}
        Ok(false) => std::process::exit(1),
        Err(error) => {
            eprintln!("audit error: {error}");
            std::process::exit(2);
        }
    }
}
