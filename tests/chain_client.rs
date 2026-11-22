//! The `cyber` binary against a mock chain node that speaks chain-wire §7
//! and validates every submitted transaction the way the kernel's step 7
//! does (decode, signature over the chain-bound statement, exact nonce).
//! Wrong chain id, wrong nonce and a forged key are rejected; a good
//! transfer, link and claim are admitted, included, and reported.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

use soft3::wallet::{self, hex, Tx};

const CHAIN: [u8; 32] = [0x44; 32];
const SPELL: &str =
    "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

#[derive(Default)]
struct State {
    height: u64,
    nonces: std::collections::HashMap<[u8; 32], u64>,
    /// txid → (tx, height included at; 0 = mempool), polls seen
    txs: std::collections::HashMap<[u8; 32], (Tx, u64, u32)>,
    legacy_hits: u32,
}

struct Node {
    url: String,
    state: Arc<Mutex<State>>,
}

fn respond(stream: &mut TcpStream, status: &str, content_type: &str, body: &str) {
    let _ = write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
}

fn handle(mut stream: TcpStream, state: &Mutex<State>) {
    let mut raw = Vec::new();
    let mut buf = [0u8; 4096];
    let (mut head_end, mut content_length) = (None, 0usize);
    loop {
        let n = stream.read(&mut buf).unwrap_or(0);
        if n == 0 {
            break;
        }
        raw.extend_from_slice(&buf[..n]);
        if head_end.is_none() {
            if let Some(i) = raw.windows(4).position(|w| w == b"\r\n\r\n") {
                head_end = Some(i + 4);
                let head = String::from_utf8_lossy(&raw[..i]).to_string();
                content_length = head
                    .lines()
                    .find_map(|l| l.to_ascii_lowercase().strip_prefix("content-length:").map(|v| v.trim().parse().unwrap_or(0)))
                    .unwrap_or(0);
            }
        }
        if let Some(h) = head_end {
            if raw.len() >= h + content_length {
                break;
            }
        }
    }
    let Some(h) = head_end else { return };
    let head = String::from_utf8_lossy(&raw[..h]).to_string();
    let body = raw[h..h + content_length].to_vec();
    let mut line = head.lines().next().unwrap_or("").split_whitespace();
    let (method, path) = (line.next().unwrap_or(""), line.next().unwrap_or(""));
    let mut s = state.lock().unwrap();
    match (method, path) {
        ("GET", "/status") => {
            let root = hex(hemera::hash(&s.height.to_le_bytes()).as_bytes());
            let doc = format!(
                "---\nparticle: status\nchain: pussy-rc\nprotocol: soft3/chain/1\nrule-set: 1\nheight: {}\ntip: {}\ngenesis: {}\nbbg-root: {root}\ncatching-up: false\n---\n",
                s.height,
                "ab".repeat(32),
                hex(&CHAIN)
            );
            respond(&mut stream, "200 OK", "text/plain; charset=utf-8", &doc);
        }
        ("GET", p) if p.starts_with("/nonce/") => {
            let n = wallet::parse32(&p[7..]).map(|id| s.nonces.get(&id).copied().unwrap_or(0)).unwrap_or(0);
            respond(&mut stream, "200 OK", "application/json", &format!("{{\"nonce\":{n}}}"));
        }
        ("GET", p) if p.starts_with("/balance/") => {
            respond(&mut stream, "200 OK", "text/plain", "---\nparticle: balance\nbalance: 1000000\n---\n");
        }
        ("POST", "/tx") => match wallet::verify(&CHAIN, &body) {
            Err(e) => respond(&mut stream, "400 Bad Request", "application/json", &format!("{{\"error\":\"{e}\"}}")),
            Ok((tx, signer)) => {
                let signer = signer.expect("signed kinds only");
                let expected = s.nonces.get(&signer).copied().unwrap_or(0);
                if tx.nonce() != Some(expected) {
                    respond(&mut stream, "400 Bad Request", "application/json", &format!("{{\"error\":\"bad nonce: expected {expected}\"}}"));
                    return;
                }
                let txid = wallet::tx::txid(&CHAIN, &body);
                s.nonces.insert(signer, expected + 1);
                s.txs.insert(txid, (tx, 0, 0));
                respond(&mut stream, "200 OK", "application/json", &format!("{{\"txid\":\"{}\"}}", hex(&txid)));
            }
        },
        ("GET", p) if p.starts_with("/tx/") => {
            let Ok(id) = wallet::parse32(&p[4..]) else {
                respond(&mut stream, "400 Bad Request", "text/plain", "bad txid");
                return;
            };
            let next_height = s.height + 1;
            let answer = match s.txs.get_mut(&id) {
                None => None,
                Some((_, included, polls)) => {
                    *polls += 1;
                    if *polls == 1 {
                        Some(None)
                    } else {
                        if *included == 0 {
                            *included = next_height;
                        }
                        Some(Some(*included))
                    }
                }
            };
            match answer {
                None => respond(&mut stream, "404 Not Found", "application/json", "{\"error\":\"unknown txid\"}"),
                Some(None) => respond(&mut stream, "200 OK", "application/json", "{\"height\":\"mempool\"}"),
                Some(Some(h)) => {
                    s.height = s.height.max(h);
                    respond(&mut stream, "200 OK", "application/json", &format!("{{\"height\":{h},\"block\":\"{}\"}}", "cd".repeat(32)));
                }
            }
        }
        ("POST", "/v1/link" | "/v1/pay") => {
            s.legacy_hits += 1;
            respond(&mut stream, "410 Gone", "application/json", "{\"error\":\"unsigned ingress is gone; POST /tx\"}");
        }
        _ => respond(&mut stream, "404 Not Found", "text/plain", "not found"),
    }
}

fn node() -> Node {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let state = Arc::new(Mutex::new(State { height: 10, ..Default::default() }));
    let shared = state.clone();
    thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            handle(stream, &shared);
        }
    });
    Node { url, state }
}

fn cyber(node: &Node, spell: &std::path::Path, args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_cyber"))
        .args(args)
        .arg("--rpc")
        .arg(&node.url)
        .arg("--spell")
        .arg(spell)
        .arg("--wait")
        .arg("20")
        .env_remove("CYBER_CHAIN_ID")
        .env_remove("CYBER_RPC")
        .output()
        .unwrap();
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

fn spell_file(dir: &tempfile::TempDir) -> std::path::PathBuf {
    let p = dir.path().join("spell");
    std::fs::write(&p, SPELL).unwrap();
    p
}

#[test]
fn transfer_link_and_claim_are_signed_submitted_and_included() {
    let node = node();
    let dir = tempfile::tempdir().unwrap();
    let spell = spell_file(&dir);

    let (ok, out, err) = cyber(&node, &spell, &["status"]);
    assert!(ok, "{err}");
    assert!(out.contains(&format!("chain-id   {}", hex(&CHAIN))), "{out}");
    assert!(out.contains("height     10"), "{out}");

    let (ok, out, err) = cyber(&node, &spell, &["neuron"]);
    assert!(ok, "{err}");
    assert!(out.contains("31284576301bacb9c2cad6379244f856dc32c751c38c3b0af4c5c886fd0ed741"), "{out}");
    assert!(out.contains("pussy19rl4cm2hmr8afy4kldpxz3fka4jguq0a4l8rzm"), "{out}");

    let to = "77".repeat(32);
    let (ok, out, err) = cyber(&node, &spell, &["tx", "transfer", &to, "5", "--json"]);
    assert!(ok, "stdout {out}\nstderr {err}");
    let receipt: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(receipt["status"], "included");
    assert_eq!(receipt["height"], 11);
    assert_eq!(receipt["bytes"], 224);
    assert_eq!(receipt["tx"]["kind"], "transfer");
    assert_eq!(receipt["tx"]["nonce"], 0);
    assert_eq!(receipt["tx"]["fee"], 224);
    assert_eq!(receipt["tx"]["token"], hex(&wallet::tx::token("pussy")));

    let (ok, out, _) = cyber(&node, &spell, &["nonce"]);
    assert!(ok);
    assert_eq!(out.trim(), "1");

    let (ok, out, err) = cyber(&node, &spell, &["tx", "link", "boot", "pussy", "--valence", "1"]);
    assert!(ok, "stdout {out}\nstderr {err}");
    assert!(out.contains("included at height 12"), "{out}");

    let (ok, out, err) = cyber(&node, &spell, &["tx", "claim", "--no-wait", "--json"]);
    assert!(ok, "stdout {out}\nstderr {err}");
    let receipt: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(receipt["status"], "submitted");
    assert_eq!(receipt["tx"]["account_id"], "28ff5c6d57d8cfd492b6fb42614536ed648e01fd");
    assert_eq!(receipt["tx"]["nonce"], 2);

    let txid = receipt["txid"].as_str().unwrap().to_string();
    let (ok, out, _) = cyber(&node, &spell, &["tx", "show", &txid]);
    assert!(ok);
    assert!(out.contains("\"mempool\""), "{out}");

    let s = node.state.lock().unwrap();
    assert_eq!(s.txs.len(), 3);
    let kinds: std::collections::BTreeSet<&str> = s.txs.values().map(|(tx, _, _)| tx.kind().name()).collect();
    assert_eq!(kinds.into_iter().collect::<Vec<_>>(), ["claim", "cyberlink", "transfer"]);
    let link = s.txs.values().find(|(tx, _, _)| tx.kind().name() == "cyberlink").unwrap();
    if let Tx::Cyberlink { p, q, valence, amount, .. } = &link.0 {
        assert_eq!(*p, wallet::tx::token("boot"));
        assert_eq!(*q, wallet::tx::token("pussy"));
        assert_eq!((*valence, *amount), (1, 0));
    }
    assert_eq!(s.legacy_hits, 0, "the client never touches the unsigned ingress");
}

#[test]
fn wrong_nonce_wrong_chain_and_bad_fee_are_rejected() {
    let node = node();
    let dir = tempfile::tempdir().unwrap();
    let spell = spell_file(&dir);
    let to = "77".repeat(32);

    let (ok, _, err) = cyber(&node, &spell, &["tx", "transfer", &to, "5", "--nonce", "7"]);
    assert!(!ok);
    assert!(err.contains("bad nonce: expected 0"), "{err}");

    let (ok, _, err) = cyber(&node, &spell, &["tx", "transfer", &to, "5", "--chain-id", &"45".repeat(32)]);
    assert!(!ok);
    assert!(err.contains("rejected") && err.contains("NSIG1"), "{err}");

    let (ok, _, err) = cyber(&node, &spell, &["tx", "transfer", &to, "5", "--fee", "100"]);
    assert!(!ok);
    assert!(err.contains("below the floor"), "{err}");

    let (ok, _, err) = cyber(&node, &spell, &["tx", "link", "boot", "boot"]);
    assert!(!ok);
    assert!(err.contains("p != q"), "{err}");

    let (ok, _, err) = cyber(&node, &spell, &["tx", "link", "boot", "pussy", "--valence", "2"]);
    assert!(!ok);
    assert!(err.contains("valence"), "{err}");

    // a forged signature: another key cannot author this neuron's transfer
    let other = dir.path().join("other");
    std::fs::write(&other, "legal winner thank year wave sausage worth useful legal winner thank yellow").unwrap();
    let signer = wallet::Signer::from_spell(SPELL, "").unwrap();
    let forged = wallet::Signer::from_spell(
        "legal winner thank year wave sausage worth useful legal winner thank yellow",
        "",
    )
    .unwrap()
    .sign(&CHAIN, Tx::Transfer { from: signer.neuron, nonce: 0, token: [0; 32], amount: 1, to: [1; 32], fee: 224 });
    assert!(forged.is_err());

    assert!(node.state.lock().unwrap().txs.is_empty(), "nothing invalid reached the mempool");
}

#[test]
fn neuron_new_mints_once_and_never_overwrites() {
    let node = node();
    let dir = tempfile::tempdir().unwrap();
    let spell = dir.path().join("fresh");
    let (ok, out, err) = cyber(&node, &spell, &["neuron", "new"]);
    assert!(ok, "{err}");
    assert!(out.contains("minted"), "{out}");
    let words = std::fs::read_to_string(&spell).unwrap();
    assert_eq!(words.split_whitespace().count(), 24);
    let (ok, _, err) = cyber(&node, &spell, &["neuron", "new"]);
    assert!(!ok);
    assert!(err.contains("never overwritten"), "{err}");
    assert_eq!(std::fs::read_to_string(&spell).unwrap(), words);
}
