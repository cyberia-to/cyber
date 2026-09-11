use fs2::FileExt;
use serde_json::Value;
use std::{
    net::TcpListener,
    path::Path,
    process::{Child, Command, Output, Stdio},
    time::{Duration, Instant},
};

fn binary() -> std::path::PathBuf {
    std::env::var_os("CYBER_TEST_BINARY")
        .map(Into::into)
        .unwrap_or_else(|| env!("CARGO_BIN_EXE_cyber").into())
}

fn cyber(home: &Path, args: &[&str]) -> Output {
    Command::new(binary())
        .arg("--home")
        .arg(home)
        .args(args)
        .output()
        .unwrap()
}

fn success(output: Output) -> String {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

struct Server(Child);
impl Server {
    fn start(home: &Path) -> Self {
        Self(
            Command::new(binary())
                .arg("--home")
                .arg(home)
                .arg("node")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .unwrap(),
        )
    }

    fn ready(&mut self, rpc: &str) {
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            assert!(
                self.0.try_wait().unwrap().is_none(),
                "node exited before readiness"
            );
            if ureq::get(&format!("{rpc}/health"))
                .timeout(Duration::from_millis(200))
                .call()
                .is_ok()
            {
                return;
            }
            assert!(Instant::now() < deadline, "node readiness timed out");
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

#[test]
fn cyb_wire_write_restart_and_exclusive_home() {
    let home = tempfile::tempdir().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let bind = listener.local_addr().unwrap().to_string();
    drop(listener);
    success(cyber(home.path(), &["init", "--bind", &bind]));
    let descriptor: Value = serde_json::from_str(&success(cyber(home.path(), &["cyb"]))).unwrap();
    let rpc = descriptor["rpc"].as_str().unwrap();
    let mut server = Server::start(home.path());
    server.ready(rpc);

    // Check the home lock itself, independently of the occupied TCP port.
    let lock = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(home.path().join("node.lock"))
        .unwrap();
    assert!(lock.try_lock_exclusive().is_err());

    let mut competing = Server::start(home.path());
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        if let Some(status) = competing.0.try_wait().unwrap() {
            assert!(!status.success());
            break;
        }
        assert!(
            Instant::now() < deadline,
            "second node accepted the same home"
        );
        std::thread::sleep(Duration::from_millis(25));
    }

    let receipt: Value = ureq::post(&format!("{rpc}/v1/link"))
        .timeout(Duration::from_secs(5))
        .send_json(serde_json::json!({"neuron":"alice", "from":"hello", "to":"cyber", "amount":1, "valence":0}))
        .unwrap().into_json().unwrap();
    assert_eq!(receipt["ok"], true);
    assert_eq!(receipt["height"], 1);
    let status: Value =
        serde_json::from_str(&success(cyber(home.path(), &["status", "--json"]))).unwrap();
    assert_eq!(status["height"], 1);
    assert_eq!(status["bbg-root"], receipt["root"]);
    // cyb's network client consumes these exact cybermark fields.
    let wire = success(cyber(home.path(), &["status"]));
    assert!(wire.lines().any(|l| l == "height: 1"));
    assert!(wire.contains(&format!("bbg-root: {}", receipt["root"].as_str().unwrap())));
    for route in ["/blocks", "/block/1", "/stats", "/log"] {
        assert!(ureq::get(&format!("{rpc}{route}"))
            .timeout(Duration::from_secs(5))
            .call()
            .is_ok());
    }
    drop(server);
    let mut restarted = Server::start(home.path());
    restarted.ready(rpc);
    let after: Value =
        serde_json::from_str(&success(cyber(home.path(), &["status", "--json"]))).unwrap();
    assert_eq!(after["height"], status["height"]);
    assert_eq!(after["bbg-root"], status["bbg-root"]);
    assert_eq!(after["signals"], status["signals"]);
}

#[test]
fn config_is_explicit_validated_and_never_overwritten() {
    let home = tempfile::tempdir().unwrap();
    assert!(!cyber(home.path(), &["node"]).status.success());
    assert!(!cyber(home.path(), &["init", "--bind", "0.0.0.0:7780"])
        .status
        .success());
    assert!(!home.path().join("config.toml").exists());
    success(cyber(home.path(), &["init", "--moniker", "my node"]));
    let original = std::fs::read(home.path().join("config.toml")).unwrap();
    assert!(!cyber(home.path(), &["init"]).status.success());
    assert_eq!(
        std::fs::read(home.path().join("config.toml")).unwrap(),
        original
    );
    assert!(!cyber(home.path(), &["init", "--bind"]).status.success());
    assert!(success(cyber(home.path(), &["config"])).contains("my node"));
    std::fs::write(home.path().join("config.toml"), "version=99").unwrap();
    assert!(!cyber(home.path(), &["node"]).status.success());
}
