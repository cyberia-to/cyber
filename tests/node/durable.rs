use super::*;
use std::io::Write;
use std::net::{Shutdown, TcpStream};

#[test]
fn lost_response_retry_and_payment_receipts_survive_sigkill() {
    let (home, rpc) = initialized();
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    let funding = serde_json::json!({"neuron":"alice", "from":"zheng", "to":"pussy", "amount":100, "request_id":"lost-funding"});
    let body = funding.to_string();
    let mut stream = TcpStream::connect(rpc.strip_prefix("http://").unwrap()).unwrap();
    write!(stream, "POST /v1/link HTTP/1.1\r\nHost: localhost\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body).unwrap();
    stream.shutdown(Shutdown::Write).unwrap();
    // The client never reads the acknowledgment. Observe the committed state
    // independently before killing the process; no production kill hook exists.
    drop(stream);
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if field(&get(&rpc, "/status"), "height") == "1" {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "lost-response operation did not commit"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    let funding_root = get(&rpc, "/root").trim().to_owned();
    let payment =
        serde_json::json!({"neuron":"alice", "to":"bob", "amount":40, "request_id":"pay-40"});
    let paid = post(&rpc, "/v1/pay", &payment);
    assert_eq!(paid["height"], 2);
    assert_eq!(paid["balance"], 60);
    let committed_root = get(&rpc, "/root");
    assert!(home.path().join("bbg").is_dir());
    assert!(!home.path().join("log").exists());
    drop(server); // Child::kill is SIGKILL on Unix.

    let mut server = Server::start(home.path());
    server.ready(&rpc);
    assert_eq!(get(&rpc, "/root"), committed_root);
    let retried = post(&rpc, "/v1/link", &funding);
    assert_eq!(retried["height"], 1);
    assert_eq!(retried["root"], funding_root);
    assert_eq!(retried["supply"], 100);
    assert_eq!(post(&rpc, "/v1/pay", &payment), paid);
    assert_eq!(get(&rpc, "/root"), committed_root);
    assert_eq!(field(&get(&rpc, "/balance/alice"), "balance"), "60");
    assert_eq!(field(&get(&rpc, "/balance/bob"), "balance"), "40");

    post(
        &rpc,
        "/v1/pay",
        &serde_json::json!({"neuron":"alice", "to":"bob", "amount":10, "request_id":"pay-10"}),
    );
    assert_eq!(post(&rpc, "/v1/pay", &payment), paid);
    assert_eq!(field(&get(&rpc, "/balance/alice"), "balance"), "50");
    let mut conflict = payment.clone();
    conflict["amount"] = 41.into();
    let error = ureq::post(&format!("{rpc}/v1/pay"))
        .send_json(conflict)
        .unwrap_err();
    assert!(matches!(error, ureq::Error::Status(409, _)));
    assert_eq!(field(&get(&rpc, "/status"), "height"), "3");

    let repeated = serde_json::json!({"neuron":"alice", "from":"hello", "to":"cyber"});
    assert_eq!(post(&rpc, "/v1/link", &repeated)["height"], 4);
    assert_eq!(post(&rpc, "/v1/link", &repeated)["height"], 5);
}

fn send_frame(rpc: &str, bytes: &[u8], id: &str) -> String {
    ureq::post(&format!("{rpc}/v1/frame"))
        .set("Idempotency-Key", id)
        .timeout(Duration::from_secs(10))
        .send_bytes(bytes)
        .unwrap()
        .into_string()
        .unwrap()
}

pub(super) fn intent_frame() -> Vec<u8> {
    // Stable historical tape envelope: 136-byte intent, unsigned LEB128 size.
    let mut frame = vec![0x1f, b'^', b'b', 0x88, 0x01];
    frame.extend([3; 32]);
    frame.extend(10u64.to_le_bytes());
    frame.extend([4; 32]);
    frame.extend([0; 64]);
    frame
}

#[test]
fn native_economics_intents_and_atomic_rejection_survive_restart() {
    let (home, rpc) = initialized();
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    post(
        &rpc,
        "/v1/link",
        &serde_json::json!({"neuron":"alice", "from":"zheng", "to":"pussy", "amount":100}),
    );
    let original_wire = wire(&rpc, 0);
    let mut signal = foculus::decode_signal_frame(&original_wire).unwrap();
    signal.neuron = [2; 32];
    signal.links[0].neuron = signal.neuron;
    let native = foculus::encode_signal_frame(&signal);
    let receipt = send_frame(&rpc, &native, "native-work");
    assert_eq!(field(&receipt, "height"), "2");
    assert_eq!(field(&receipt, "supply"), "200");
    assert_eq!(
        field(
            &get(&rpc, &format!("/balance/{}", "02".repeat(32))),
            "balance"
        ),
        "100"
    );
    assert_eq!(send_frame(&rpc, &native, "native-work"), receipt);
    let root_before_intent = get(&rpc, "/root");
    let intent = intent_frame();
    let intended = send_frame(&rpc, &intent, "native-intent");
    assert_eq!(field(&intended, "height"), "2");
    // Intent retention is outside the current BBG root dimensions.
    assert_eq!(get(&rpc, "/root"), root_before_intent);
    assert_eq!(field(&get(&rpc, "/status"), "intents"), "1");

    let mut first = signal.clone();
    first.neuron = [5; 32];
    first.links[0].neuron = first.neuron;
    let mut second = first.clone();
    second.step = 99;
    let mut batch = foculus::encode_signal_frame(&first);
    batch.extend(foculus::encode_signal_frame(&second));
    let root = get(&rpc, "/root");
    let log = wire(&rpc, 0);
    let error = ureq::post(&format!("{rpc}/v1/frame"))
        .set("Idempotency-Key", "invalid-batch")
        .send_bytes(&batch)
        .unwrap_err();
    assert!(matches!(error, ureq::Error::Status(400, _)));
    assert_eq!(get(&rpc, "/root"), root);
    assert_eq!(wire(&rpc, 0), log);
    assert_eq!(field(&get(&rpc, "/balance/alice"), "supply"), "200");
    let accepted = send_frame(
        &rpc,
        &foculus::encode_signal_frame(&first),
        "valid-after-rejection",
    );
    assert_eq!(field(&accepted, "height"), "3");
    let root = get(&rpc, "/root");
    let log = wire(&rpc, 0);
    assert_eq!(wire(&rpc, original_wire.len()), log[original_wire.len()..]);
    assert_eq!(
        foculus::frames::decode_events_strict(&log).unwrap().len(),
        4
    );
    assert!(get(&rpc, "/blocks").contains("2 "));
    assert!(get(&rpc, "/block/2").contains("supply: 200"));
    drop(server);
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    assert_eq!(get(&rpc, "/root"), root);
    assert_eq!(wire(&rpc, 0), log);
    assert_eq!(field(&get(&rpc, "/balance/alice"), "supply"), "300");
    assert_eq!(field(&get(&rpc, "/status"), "intents"), "1");
    assert_eq!(send_frame(&rpc, &intent, "native-intent"), intended);
    assert_eq!(get(&rpc, "/root"), root);
}

#[test]
fn v2_retains_explicit_network_across_restart_and_refuses_lossy_export() {
    let (home, rpc) = initialized();
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    let signal = foculus::Signal {
        neuron: [7; 32],
        network: [8; 32],
        links: vec![],
        delta_pi: vec![],
        box_moves: vec![],
        prev: [0; 32],
        step: 0,
        height: 0,
        proof: None,
    };
    let encoded = foculus::signal_codec::encode_signal(&signal).unwrap();
    let receipt = ureq::post(&format!("{rpc}/v2/frame"))
        .set("Idempotency-Key", "explicit-network")
        .send_bytes(&encoded)
        .unwrap()
        .into_string()
        .unwrap();
    assert_eq!(field(&receipt, "height"), "1");
    assert!(matches!(
        ureq::get(&format!("{rpc}/log")).call(),
        Err(ureq::Error::Status(422, _))
    ));
    let history: Value = serde_json::from_str(&get(&rpc, "/v2/history")).unwrap();
    let mut operation = b"CGOP\x01\x02\x01\0\0\0\0".to_vec();
    operation.extend((encoded.len() as u32).to_le_bytes());
    operation.extend(&encoded);
    let operation: String = operation.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(history["entries"][0]["operation"], operation);
    let root = get(&rpc, "/root");
    drop(server);
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    assert_eq!(get(&rpc, "/root"), root);
    let recovered_history: Value = serde_json::from_str(&get(&rpc, "/v2/history")).unwrap();
    assert_eq!(recovered_history, history);
    let retry = ureq::post(&format!("{rpc}/v2/frame"))
        .set("Idempotency-Key", "explicit-network")
        .send_bytes(&encoded)
        .unwrap()
        .into_string()
        .unwrap();
    assert_eq!(retry, receipt);
}
