//! Qualify an actual previous writer against a candidate reader and writer.
//! cargo run --locked --example node_storage_compat -- PREVIOUS_BINARY CURRENT_BINARY
use serde_json::{json, Value};
use std::{
    net::TcpListener,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    time::{Duration, Instant},
};

struct Server(Child);
impl Server {
    fn start(binary: &Path, home: &Path) -> Self {
        Self(
            Command::new(binary)
                .arg("--home")
                .arg(home)
                .arg("node")
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .spawn()
                .unwrap(),
        )
    }
    fn ready(&mut self, rpc: &str) {
        let deadline = Instant::now() + Duration::from_secs(15);
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
    fn refused(mut self) {
        use std::io::Read;
        let deadline = Instant::now() + Duration::from_secs(15);
        loop {
            if let Some(status) = self.0.try_wait().unwrap() {
                assert!(
                    !status.success(),
                    "previous writer accepted the upgraded store"
                );
                let mut error = String::new();
                self.0
                    .stderr
                    .take()
                    .unwrap()
                    .read_to_string(&mut error)
                    .unwrap();
                assert!(
                    error.contains("exact state mismatch"),
                    "unexpected refusal: {error}"
                );
                return;
            }
            assert!(
                Instant::now() < deadline,
                "previous writer did not refuse the upgraded store"
            );
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
fn invoke(binary: &Path, home: &Path, args: &[&str]) -> String {
    let output = Command::new(binary)
        .arg("--home")
        .arg(home)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}
fn status(binary: &Path, home: &Path) -> Value {
    serde_json::from_str(&invoke(binary, home, &["status", "--json"])).unwrap()
}
fn write(rpc: &str, request: &str, amount: u64) -> Value {
    ureq::post(&format!("{rpc}/v1/link")).timeout(Duration::from_secs(5))
        .set("Idempotency-Key", request)
        .send_json(json!({"neuron":"compat-alice", "from":"zheng", "to":"pussy", "amount":amount, "valence":1}))
        .unwrap().into_json().unwrap()
}
fn run(previous: &Path, current: &Path, empty: bool) {
    let home = tempfile::tempdir().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let bind = listener.local_addr().unwrap().to_string();
    drop(listener);
    let rpc = format!("http://{bind}");
    invoke(previous, home.path(), &["init", "--bind", &bind]);
    let config = std::fs::read(home.path().join("config.toml")).unwrap();
    let mut old = Server::start(previous, home.path());
    old.ready(&rpc);
    let original = (!empty).then(|| write(&rpc, "compat-original", 25));
    let before = status(previous, home.path());
    drop(old);

    let mut candidate = Server::start(current, home.path());
    candidate.ready(&rpc);
    assert_eq!(status(current, home.path()), before);
    if let Some(ref receipt) = original {
        assert_eq!(&write(&rpc, "compat-original", 25), receipt);
    }
    drop(candidate);

    // Read-only recovery and exact retries preserve the old reader's ability to open.
    let mut old = Server::start(previous, home.path());
    old.ready(&rpc);
    assert_eq!(status(previous, home.path()), before);
    drop(old);

    let mut candidate = Server::start(current, home.path());
    candidate.ready(&rpc);
    let next = write(&rpc, "compat-next", 3);
    let after = status(current, home.path());
    drop(candidate);
    Server::start(previous, home.path()).refused();
    let mut restarted = Server::start(current, home.path());
    restarted.ready(&rpc);
    assert_eq!(status(current, home.path()), after);
    assert_eq!(write(&rpc, "compat-next", 3), next);
    if let Some(receipt) = original {
        assert_eq!(write(&rpc, "compat-original", 25), receipt);
    }
    assert_eq!(
        std::fs::read(home.path().join("config.toml")).unwrap(),
        config
    );
    println!("{} legacy store: preserved state/receipts, delayed version upgrade, old-writer refusal and restart passed",
        if empty { "empty" } else { "populated" });
}
fn main() {
    let args: Vec<PathBuf> = std::env::args_os().skip(1).map(Into::into).collect();
    assert_eq!(args.len(), 2, "expected PREVIOUS_BINARY CURRENT_BINARY");
    for path in &args {
        assert!(path.is_file(), "binary missing: {}", path.display());
    }
    for empty in [true, false] {
        run(&args[0], &args[1], empty);
    }
}
