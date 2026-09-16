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
    assert_eq!(descriptor["schema"], "cyber/connection/v2");
    assert_eq!(descriptor["observation"], "configuration-only");
    assert!(descriptor["network"].is_null());
    assert!(descriptor["submit"].is_null());
    assert!(descriptor["capabilities"]["authenticated_writes"].is_null());
    assert!(!home.path().join("genesis.json").exists());
    assert!(!home.path().join("bbg").exists());
    assert!(!cyber(home.path(), &["cyb", "--live"]).status.success());
    let rpc = descriptor["rpc"].as_str().unwrap();
    let mut server = Server::start(home.path());
    server.ready(rpc);
    let live: Value =
        serde_json::from_str(&success(cyber(home.path(), &["cyb", "--live"]))).unwrap();
    assert_eq!(live["profile"], "soft3/unsigned-local/1");
    assert_eq!(live["capabilities"]["authenticated_writes"], false);
    assert!(live["submit"].is_null());
    assert_eq!(live["cyb_commands"].as_array().unwrap().len(), 1);

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

fn initialized() -> (tempfile::TempDir, String) {
    let home = tempfile::tempdir().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let bind = listener.local_addr().unwrap().to_string();
    drop(listener);
    success(cyber(home.path(), &["init", "--bind", &bind]));
    (home, format!("http://{bind}"))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn id(text: &str) -> [u8; 32] {
    assert_eq!(text.len(), 64);
    std::array::from_fn(|i| u8::from_str_radix(&text[i * 2..i * 2 + 2], 16).unwrap())
}

fn signed_signal(network: [u8; 32], amount: u64) -> neuron_model::action::SignedAction {
    use neuron_model::{
        action::{statement_bytes, ActionRequest, SignedAction},
        identity::{ActionContext, NetworkRef, SubjectRef},
    };
    let key = mudra::SigningKey::from_bytes((&[7; 32]).into()).unwrap();
    let subject = mudra::claim::neuron_of(&mudra::cosmos::compressed(key.verifying_key()));
    let signal = cybergraph::Signal {
        neuron: subject,
        network,
        links: vec![cybergraph::CyberlinkRecord {
            neuron: subject,
            from: [1; 32],
            to: [2; 32],
            token: [0; 32],
            amount,
            valence: -7,
            height: 1,
        }],
        delta_pi: vec![],
        box_moves: vec![],
        prev: [0; 32],
        step: 0,
        height: 1,
        proof: None,
    };
    let action = ActionRequest {
        request: [3; 32],
        attachment: [4; 32],
        context: ActionContext {
            subject: SubjectRef::Native(subject),
            network: NetworkRef::Native(network),
            binding_revision: 0,
            prog: None,
            invocation: None,
            policy: [5; 32],
            grant: [6; 32],
        },
        kind: "native/signal".into(),
        payload: foculus::signal_codec::encode_signal(&signal).unwrap(),
    };
    let statement = *hemera::hash(&statement_bytes(&action).unwrap()).as_bytes();
    SignedAction::new(
        action,
        mudra::neuron::sign_statement(&key, subject, statement).unwrap(),
    )
    .unwrap()
}

fn post_signed(
    rpc: &str,
    action: &neuron_model::action::SignedAction,
) -> Result<ureq::Response, ureq::Error> {
    ureq::post(&format!("{rpc}/v3/action"))
        .timeout(Duration::from_secs(5))
        .set("Content-Type", "application/octet-stream")
        .set("Idempotency-Key", &hex(&action.action.request))
        .send_bytes(&action.encode().unwrap())
}

#[test]
fn explicit_authentication_descriptor_signed_receipt_and_restart() {
    let (home, rpc) = initialized();
    let unrelated_key = b"existing operator custody remains untouched";
    std::fs::write(home.path().join("operator.key"), unrelated_key).unwrap();
    let config = std::fs::read(home.path().join("config.toml")).unwrap();
    let enabled: Value =
        serde_json::from_str(&success(cyber(home.path(), &["auth", "enable"]))).unwrap();
    assert_eq!(enabled["profile"], "neuron/signed-native/1");
    assert_eq!(enabled["authenticated"], true);
    assert_eq!(enabled["consensus_finality"], false);
    let network = id(enabled["network"].as_str().unwrap());
    let genesis = std::fs::read(home.path().join("genesis.json/source")).unwrap();
    assert_eq!(
        std::fs::read(home.path().join("operator.key")).unwrap(),
        unrelated_key
    );
    assert_eq!(
        std::fs::read(home.path().join("config.toml")).unwrap(),
        config
    );
    for path in ["keys", "mnemonic", "neuron.key", "soul"] {
        assert!(!home.path().join(path).exists());
    }

    let mut server = Server::start(home.path());
    server.ready(&rpc);
    for args in [
        vec!["auth", "enable"],
        vec!["storage", "import-legacy"],
        vec!["auth", "enable", "--import-legacy"],
    ] {
        let result = cyber(home.path(), &args);
        assert!(!result.status.success());
        assert!(String::from_utf8_lossy(&result.stderr).contains("already in use"));
    }
    let live: Value =
        serde_json::from_str(&success(cyber(home.path(), &["cyb", "--live"]))).unwrap();
    assert_eq!(live["network"], enabled["network"]);
    assert_eq!(live["network_label"], "spacepussy-test");
    assert_eq!(live["submit"], "/v3/action");
    assert_eq!(live["capabilities"]["authenticated_writes"], true);
    assert_eq!(live["receipt_meaning"], "endpoint-acceptance");
    assert_eq!(
        live["cyb_commands"][1],
        format!("net pin spacepussy-test {}", hex(&network))
    );
    assert_eq!(live["endpoints"]["history"], "/v3/history");
    for route in ["/v1/link", "/v1/pay", "/v1/frame", "/v2/frame"] {
        let result = ureq::post(&format!("{rpc}{route}")).send_bytes(b"{}");
        assert!(
            matches!(result, Err(ureq::Error::Status(401, _))),
            "{route}"
        );
    }
    let action = signed_signal(network, 100);
    let receipt: Value = post_signed(&rpc, &action).unwrap().into_json().unwrap();
    assert_eq!(receipt["state"], "accepted");
    assert_eq!(receipt["network"], enabled["network"]);
    assert_eq!(
        post_signed(&rpc, &action)
            .unwrap()
            .into_json::<Value>()
            .unwrap(),
        receipt
    );
    assert!(matches!(
        post_signed(&rpc, &signed_signal(network, 101)),
        Err(ureq::Error::Status(409, _))
    ));
    assert!(matches!(
        post_signed(&rpc, &signed_signal([99; 32], 100)),
        Err(ureq::Error::Status(400, _))
    ));
    let path = format!(
        "{rpc}/v3/receipt/{}/{}",
        receipt["subject"].as_str().unwrap(),
        receipt["request"].as_str().unwrap()
    );
    assert_eq!(
        ureq::get(&path)
            .call()
            .unwrap()
            .into_json::<Value>()
            .unwrap(),
        receipt
    );
    let history: Value = ureq::get(&format!("{rpc}/v3/history?limit=16"))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
    assert_eq!(
        history["entries"][0]["signals"][0],
        hex(&action.action.payload)
    );
    let before = success(cyber(home.path(), &["status", "--json"]));
    drop(server);
    let repeated: Value =
        serde_json::from_str(&success(cyber(home.path(), &["auth", "enable"]))).unwrap();
    assert_eq!(repeated, enabled);
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    assert_eq!(success(cyber(home.path(), &["status", "--json"])), before);
    assert_eq!(
        ureq::get(&path)
            .call()
            .unwrap()
            .into_json::<Value>()
            .unwrap(),
        receipt
    );
    assert_eq!(
        post_signed(&rpc, &action)
            .unwrap()
            .into_json::<Value>()
            .unwrap(),
        receipt
    );
    assert_eq!(
        std::fs::read(home.path().join("genesis.json/source")).unwrap(),
        genesis
    );
    assert_eq!(
        std::fs::read(home.path().join("operator.key")).unwrap(),
        unrelated_key
    );
}

fn legacy(home: &Path) -> (Vec<u8>, Vec<u8>) {
    let genesis = b"{\n  \"chain_id\":\"spacepussy-test\",\n  \"genesis_time\":17,\n  \"engine\":\"cybergraph+bbg\",\n  \"protocol\":\"soft3/spacepussy-test/v1\"\n}\n".to_vec();
    let log = [21, 22]
        .into_iter()
        .flat_map(|n| {
            foculus::encode_signal_frame(&cybergraph::Signal {
                neuron: [n; 32],
                network: cybergraph::SELF_NETWORK,
                links: vec![],
                delta_pi: vec![],
                box_moves: vec![],
                prev: [0; 32],
                step: 0,
                height: 0,
                proof: None,
            })
        })
        .collect::<Vec<_>>();
    std::fs::write(home.join("genesis.json"), &genesis).unwrap();
    std::fs::write(home.join("log"), &log).unwrap();
    (genesis, log)
}

#[test]
fn explicit_legacy_import_and_combined_auth_upgrade_preserve_sources() {
    for combined in [false, true] {
        let (home, rpc) = initialized();
        let (genesis, log) = legacy(home.path());
        let refusal = cyber(home.path(), &["node"]);
        assert!(!refusal.status.success());
        assert!(String::from_utf8_lossy(&refusal.stderr).contains("storage import-legacy"));
        assert!(!cyber(home.path(), &["auth", "enable"]).status.success());
        assert!(!home.path().join("bbg").exists());
        let result: Value = if combined {
            serde_json::from_str(&success(cyber(
                home.path(),
                &["auth", "enable", "--import-legacy"],
            )))
            .unwrap()
        } else {
            let imported: Value =
                serde_json::from_str(&success(cyber(home.path(), &["storage", "import-legacy"])))
                    .unwrap();
            assert_eq!(imported["events"], 2);
            assert_eq!(imported["signals"], 2);
            assert_eq!(imported["source_retained"], true);
            assert_eq!(
                serde_json::from_str::<Value>(&success(cyber(
                    home.path(),
                    &["storage", "import-legacy"]
                )))
                .unwrap(),
                imported
            );
            assert_eq!(
                std::fs::read(home.path().join("genesis.json")).unwrap(),
                genesis
            );
            let mut unsigned = Server::start(home.path());
            unsigned.ready(&rpc);
            let status: Value =
                serde_json::from_str(&success(cyber(home.path(), &["status", "--json"]))).unwrap();
            assert_eq!(status["signals"], "2");
            drop(unsigned);
            serde_json::from_str(&success(cyber(home.path(), &["auth", "enable"]))).unwrap()
        };
        assert_eq!(
            std::fs::read(home.path().join("genesis.json/source")).unwrap(),
            genesis
        );
        assert_eq!(std::fs::read(home.path().join("log")).unwrap(), log);
        let mut server = Server::start(home.path());
        server.ready(&rpc);
        let descriptor: Value =
            serde_json::from_str(&success(cyber(home.path(), &["cyb", "--live"]))).unwrap();
        assert_eq!(descriptor["network"], result["network"]);
        let status: Value =
            serde_json::from_str(&success(cyber(home.path(), &["status", "--json"]))).unwrap();
        assert_eq!(status["signals"], "2");
        let history: Value = ureq::get(&format!("{rpc}/v3/history?limit=16"))
            .call()
            .unwrap()
            .into_json()
            .unwrap();
        let retained = history["entries"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|entry| entry["signals"].as_array().unwrap())
            .map(|s| {
                let text = s.as_str().unwrap();
                let bytes = (0..text.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
                    .collect::<Vec<_>>();
                let signal = foculus::signal_codec::decode_signal(&bytes).unwrap();
                assert_eq!(signal.network, cybergraph::SELF_NETWORK);
                signal.neuron
            })
            .collect::<Vec<_>>();
        assert_eq!(retained, vec![[21; 32], [22; 32]]);
        drop(server);
        std::fs::write(home.path().join("log"), [&log[..], b"bad"].concat()).unwrap();
        assert!(!cyber(home.path(), &["node"]).status.success());
        std::fs::write(home.path().join("log"), &log).unwrap();
        let mut server = Server::start(home.path());
        server.ready(&rpc);
        assert_eq!(
            serde_json::from_str::<Value>(&success(cyber(home.path(), &["status", "--json"])))
                .unwrap(),
            status
        );
    }
}

#[test]
fn malformed_legacy_and_invalid_commands_leave_the_source_and_generation_intact() {
    let (home, _) = initialized();
    let (genesis, mut log) = legacy(home.path());
    log.pop();
    std::fs::write(home.path().join("log"), &log).unwrap();
    for args in [
        vec!["storage", "import-legacy"],
        vec!["auth", "enable", "--import-legacy"],
        vec!["auth", "disable"],
        vec!["auth", "enable", "--home"],
        vec!["storage", "import-legacy", "--unknown"],
    ] {
        assert!(!cyber(home.path(), &args).status.success());
        assert_eq!(
            std::fs::read(home.path().join("genesis.json")).unwrap(),
            genesis
        );
        assert_eq!(std::fs::read(home.path().join("log")).unwrap(), log);
        assert!(!home.path().join("bbg").exists());
    }
}

#[test]
fn live_descriptor_refuses_foreign_metadata_redirects_and_oversized_observations() {
    use std::io::{Read, Write};
    for (status, body) in [
        ("200 OK", serde_json::json!({
            "schema":"neuron/native-capabilities/1", "profile":"foreign/test/1",
            "network":"a".repeat(64), "authenticated":true, "kinds":["native/signal", "native/pay"],
            "max_envelope_bytes":8*1024*1024, "idempotency":"network-subject-request/1",
            "receipt":"/v3/receipt/{subject}/{request}",
        }).to_string()),
        ("200 OK", "x".repeat(64 * 1024 + 1)),
        ("302 Found", String::new()),
    ] {
        let home=tempfile::tempdir().unwrap();
        let listener=TcpListener::bind("127.0.0.1:0").unwrap();
        let address=listener.local_addr().unwrap();
        success(cyber(home.path(), &["init", "--bind", &address.to_string()]));
        let peer=std::thread::spawn(move || {
            let (mut socket, _)=listener.accept().unwrap();
            socket.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
            socket.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
            let mut request=[0u8;4096];
            let read=socket.read(&mut request).unwrap();
            assert!(String::from_utf8_lossy(&request[..read]).starts_with("GET /capabilities "));
            let _=write!(socket,"HTTP/1.1 {status}\r\nContent-Type: application/json\r\nLocation: http://{address}/capabilities\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len());
        });
        let output=cyber(home.path(), &["cyb", "--live"]);
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        peer.join().unwrap();
        assert!(!home.path().join("bbg").exists());
        assert!(!home.path().join("genesis.json").exists());
    }
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
