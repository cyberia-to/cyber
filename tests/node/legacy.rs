use super::*;

fn write_genesis(home: &Path) {
    std::fs::write(home.join("genesis.json"), b"{\"chain_id\":\"spacepussy-test\",\"genesis_time\":42,\"engine\":\"cybergraph+bbg\",\"protocol\":\"soft3/spacepussy-test/v1\"}\n").unwrap();
}

#[test]
fn explicit_legacy_import_preserves_old_frame_bytes_and_checks_source_on_restart() {
    let (donor_home, donor_rpc) = initialized();
    let mut donor = Server::start(donor_home.path());
    donor.ready(&donor_rpc);
    post(
        &donor_rpc,
        "/v1/link",
        &serde_json::json!({"neuron":"alice", "from":"zheng", "to":"pussy", "amount":100}),
    );
    let frame = wire(&donor_rpc, 0);
    // This one-link frame uses a two-byte length. Remove the two optional
    // empty collections to exercise the oldest complete legacy encoding.
    assert_eq!(&frame[..3], &[0x1f, b'!', b'b']);
    assert_ne!(frame[3] & 0x80, 0);
    assert_eq!(frame[4] & 0x80, 0);
    let length = frame.len() - 5 - 8;
    let mut old = vec![
        0x1f,
        b'!',
        b'b',
        (length as u8 & 0x7f) | 0x80,
        (length >> 7) as u8,
    ];
    old.extend_from_slice(&frame[5..frame.len() - 8]);
    let intent = super::durable::intent_frame();
    ureq::post(&format!("{donor_rpc}/v1/frame"))
        .send_bytes(&intent)
        .unwrap();
    old.extend(intent);
    let root = get(&donor_rpc, "/root");
    let genesis = std::fs::read(donor_home.path().join("genesis.json")).unwrap();
    drop(donor);

    let (home, rpc) = initialized();
    std::fs::write(home.path().join("genesis.json"), &genesis).unwrap();
    std::fs::write(home.path().join("log"), &old).unwrap();
    std::fs::write(home.path().join("block_meta"), b"1 99 100 100\n").unwrap();
    rejected_start(home.path());
    assert!(!home.path().join("bbg").exists());
    let imported: Value =
        serde_json::from_str(&success(cyber(home.path(), &["storage", "import-legacy"]))).unwrap();
    assert_eq!(imported["events"], 2);
    assert_eq!(imported["signals"], 1);
    assert_eq!(imported["root"], root.trim());
    assert_eq!(std::fs::read(home.path().join("log")).unwrap(), old);
    assert_eq!(
        std::fs::read(home.path().join("genesis.json")).unwrap(),
        genesis
    );
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    assert_eq!(get(&rpc, "/root"), root);
    assert_eq!(wire(&rpc, 0), old);
    assert_eq!(field(&get(&rpc, "/balance/alice"), "balance"), "100");
    assert_eq!(field(&get(&rpc, "/block/1"), "time"), "0");
    assert!(!cyber(home.path(), &["storage", "import-legacy"])
        .status
        .success());
    post(
        &rpc,
        "/v1/pay",
        &serde_json::json!({"neuron":"alice", "to":"bob", "amount":30, "request_id":"after-import"}),
    );
    let log = wire(&rpc, 0);
    assert!(log.starts_with(&old));
    assert_eq!(wire(&rpc, old.len()), log[old.len()..]);
    let root = get(&rpc, "/root");
    drop(server);
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    assert_eq!(get(&rpc, "/root"), root);
    assert_eq!(wire(&rpc, 0), log);
    drop(server);

    let mut changed = old.clone();
    changed.push(0);
    std::fs::write(home.path().join("log"), &changed).unwrap();
    rejected_start(home.path());
    assert_eq!(std::fs::read(home.path().join("log")).unwrap(), changed);
    std::fs::write(home.path().join("log"), &old).unwrap();
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    assert_eq!(get(&rpc, "/root"), root);
}

#[test]
fn malformed_legacy_logs_fail_without_source_rewrite_or_ready_destination() {
    let valid = super::durable::intent_frame();
    let invalid_order = foculus::Signal {
        neuron: [1; 32],
        network: foculus::SELF_NETWORK,
        links: vec![],
        delta_pi: vec![],
        box_moves: vec![],
        prev: [0; 32],
        step: 9,
        height: 0,
        proof: None,
    };
    let malformed = [
        vec![0x1f, b'!', b'b', 0x80],
        [valid.as_slice(), b"trailing garbage"].concat(),
        valid[..valid.len() - 1].to_vec(),
        foculus::encode_signal_frame(&invalid_order),
    ];
    for log in malformed {
        let (home, _) = initialized();
        write_genesis(home.path());
        std::fs::write(home.path().join("log"), &log).unwrap();
        assert!(!cyber(home.path(), &["storage", "import-legacy"])
            .status
            .success());
        assert_eq!(std::fs::read(home.path().join("log")).unwrap(), log);
        rejected_start(home.path());
    }
}

#[test]
fn committed_block_root_mismatch_prevents_binary_readiness() {
    use bbg::storage::database::{Backend, Database, RecordDomain};
    let (home, rpc) = initialized();
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    let receipt = post(
        &rpc,
        "/v1/link",
        &serde_json::json!({"neuron":"alice","from":"hello","to":"cyber"}),
    );
    let root = get(&rpc, "/root");
    drop(server);
    let key = 1u64.to_be_bytes();
    let database = home.path().join("bbg");
    let original = {
        let db = Database::open(&database, Backend::Ssd).unwrap();
        let original = db
            .read_record(RecordDomain::NativeBlocks, &key, 4096)
            .unwrap()
            .unwrap();
        assert_eq!(&original[..4], b"CGB\x01");
        let encoded_root: String = original[36..68]
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(encoded_root, receipt["root"].as_str().unwrap());
        let mut changed = original.clone();
        changed[36] ^= 1; // Preserve framing and signal; disagree only on the committed root.
        db.transaction::<_, bbg::storage::StorageError>(|tx| {
            tx.put_record(RecordDomain::NativeBlocks, &key, &changed)
        })
        .unwrap();
        original
    };
    rejected_start(home.path());
    {
        let db = Database::open(&database, Backend::Ssd).unwrap();
        let retained = db
            .read_record(RecordDomain::NativeBlocks, &key, 4096)
            .unwrap()
            .unwrap();
        assert_ne!(retained, original);
        db.transaction::<_, bbg::storage::StorageError>(|tx| {
            tx.put_record(RecordDomain::NativeBlocks, &key, &original)
        })
        .unwrap();
    }
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    assert_eq!(get(&rpc, "/root"), root);
}

#[test]
fn genesis_corruption_or_identity_change_fails_startup_and_preserves_bytes() {
    let (home, rpc) = initialized();
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    post(
        &rpc,
        "/v1/link",
        &serde_json::json!({"neuron":"alice", "from":"hello", "to":"cyber"}),
    );
    let root = get(&rpc, "/root");
    drop(server);
    let path = home.path().join("genesis.json");
    let original = std::fs::read(&path).unwrap();
    for bytes in [b"{\"genesis_time\":123 garbage".to_vec(), b"{}".to_vec()] {
        std::fs::write(&path, &bytes).unwrap();
        rejected_start(home.path());
        assert_eq!(std::fs::read(&path).unwrap(), bytes);
    }
    let mut changed: Value = serde_json::from_slice(&original).unwrap();
    changed["genesis_time"] = (changed["genesis_time"].as_u64().unwrap() + 1).into();
    let bytes = changed.to_string().into_bytes();
    std::fs::write(&path, &bytes).unwrap();
    rejected_start(home.path());
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    std::fs::remove_file(&path).unwrap();
    rejected_start(home.path());
    assert!(!path.exists());
    // Formatting changes preserve the pinned canonical genesis.
    let original: Value = serde_json::from_slice(&original).unwrap();
    std::fs::write(&path, original.to_string()).unwrap();
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    assert_eq!(get(&rpc, "/root"), root);
}
