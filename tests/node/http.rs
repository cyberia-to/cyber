use super::*;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

fn raw(rpc: &str, request: &[u8]) -> String {
    let mut stream = TcpStream::connect(rpc.strip_prefix("http://").unwrap()).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    stream.write_all(request).unwrap();
    stream.shutdown(Shutdown::Write).unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    response
}

#[test]
fn incomplete_ambiguous_and_invalid_inputs_never_mutate_the_node() {
    let (home, rpc) = initialized();
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    let root = get(&rpc, "/root");
    let body = r#"{"neuron":"alice","from":"hello","to":"cyber"}"#;
    for header in [
        format!("Content-Length: {}\r\n", body.len() + 5),
        format!(
            "Content-Length: {}\r\nContent-Length: {}\r\n",
            body.len(),
            body.len()
        ),
        format!(
            "Content-Length: {}\r\nTransfer-Encoding: chunked\r\n",
            body.len()
        ),
        "Content-Length: nope\r\n".into(),
        String::new(),
    ] {
        let response = raw(
            &rpc,
            format!("POST /v1/link HTTP/1.1\r\nHost: localhost\r\n{header}\r\n{body}").as_bytes(),
        );
        assert!(response.starts_with("HTTP/1.1 400"), "{response}");
    }
    let response = raw(
        &rpc,
        b"POST /v1/link HTTP/1.1\r\nContent-Length: 8388609\r\n\r\n",
    );
    assert!(response.starts_with("HTTP/1.1 413"));
    for invalid in [
        serde_json::json!({"neuron":"alice","from":"hello","to":"cyber","valence":128}),
        serde_json::json!({"neuron":"alice","from":"hello","to":"cyber","amount":-1}),
        serde_json::json!({"neuron":"alice","from":"hello","to":"cyber","typo":true}),
        serde_json::json!({"neuron":"alice","from":"hello","to":"cyber","request_id":" "}),
    ] {
        assert!(matches!(
            ureq::post(&format!("{rpc}/v1/link")).send_json(invalid),
            Err(ureq::Error::Status(400, _))
        ));
    }
    assert!(matches!(ureq::post(&format!("{rpc}/v1/link")).set("Idempotency-Key", "header")
        .send_json(serde_json::json!({"neuron":"alice","from":"hello","to":"cyber","request_id":"body"})), Err(ureq::Error::Status(400, _))));
    assert_eq!(get(&rpc, "/root"), root);
    assert_eq!(field(&get(&rpc, "/status"), "height"), "0");
    assert_eq!(get(&rpc, "/health"), "ok\n");
}

#[test]
fn returned_request_key_is_reusable_and_history_is_paginated() {
    let (home, rpc) = initialized();
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    let mut body = serde_json::json!({"neuron":"alice","from":"hello","to":"cyber"});
    let receipt = post(&rpc, "/v1/link", &body);
    body["request_id"] = receipt["request_id"].clone();
    assert_eq!(post(&rpc, "/v1/link", &body), receipt);
    let next = post(
        &rpc,
        "/v1/link",
        &serde_json::json!({"neuron":"alice","from":"second","to":"cyber"}),
    );
    let page: Value = serde_json::from_str(&get(&rpc, "/v2/history?limit=1")).unwrap();
    assert_eq!(page["schema"], "cyber/native-history/v1");
    assert_eq!(page["entries"].as_array().unwrap().len(), 1);
    assert_eq!(page["entries"][0]["receipt"], receipt);
    assert!(page["entries"][0]["operation"]
        .as_str()
        .unwrap()
        .starts_with("43474f5001"));
    let cursor = page["next"].as_u64().unwrap();
    let page: Value =
        serde_json::from_str(&get(&rpc, &format!("/v2/history?after={cursor}&limit=1"))).unwrap();
    assert_eq!(page["entries"][0]["receipt"], next);
    let page: Value = serde_json::from_str(&get(
        &rpc,
        &format!("/v2/history?after={}&limit=1", page["next"]),
    ))
    .unwrap();
    assert!(page["entries"].as_array().unwrap().is_empty());
    assert!(page["next"].is_null());
    let root = get(&rpc, "/root");
    drop(server);
    let mut server = Server::start(home.path());
    server.ready(&rpc);
    assert_eq!(post(&rpc, "/v1/link", &body), receipt);
    assert_eq!(get(&rpc, "/root"), root);
}
