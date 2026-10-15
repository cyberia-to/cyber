use std::{
    net::TcpListener,
    path::Path,
    process::{Command, Output},
    time::Duration,
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

fn initialized() -> (tempfile::TempDir, String) {
    let home = tempfile::tempdir().unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let bind = listener.local_addr().unwrap().to_string();
    drop(listener);
    success(cyber(home.path(), &["init", "--bind", &bind]));
    (home, format!("http://{bind}"))
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
