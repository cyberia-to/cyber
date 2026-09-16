//! Configuration and explicitly requested, bounded endpoint observations.
use crate::Config;
use serde_json::{json, Value};
use std::{io::Read, path::Path, time::Duration};

pub const SIGNED_PROFILE: &str = "neuron/signed-native/1";
const UNSIGNED_PROFILE: &str = "soft3/unsigned-local/1";
const RECEIPT: &str = "/v3/receipt/{subject}/{request}";
const MAX_OBSERVATION: u64 = 64 * 1024;

pub fn read_endpoint(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = ureq::AgentBuilder::new()
        .redirects(0)
        .timeout(Duration::from_secs(5))
        .build()
        .get(url)
        .call()?;
    if response.status() != 200 {
        return Err(format!("expected HTTP 200, received {}", response.status()).into());
    }
    let mut bytes = Vec::new();
    response
        .into_reader()
        .take(MAX_OBSERVATION + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_OBSERVATION {
        return Err("endpoint observation exceeds 64 KiB".into());
    }
    Ok(String::from_utf8(bytes)?)
}

fn observe(config: &Config) -> Result<Value, Box<dyn std::error::Error>> {
    let value: Value =
        serde_json::from_str(&read_endpoint(&format!("{}/capabilities", config.rpc()))?)?;
    let network = value["network"]
        .as_str()
        .ok_or("capabilities missing native network")?;
    let authenticated = value["authenticated"]
        .as_bool()
        .ok_or("capabilities missing authentication state")?;
    let profile = if authenticated {
        SIGNED_PROFILE
    } else {
        UNSIGNED_PROFILE
    };
    let kinds = value["kinds"]
        .as_array()
        .ok_or("capabilities missing action kinds")?;
    let expected = if authenticated {
        vec![json!("native/signal"), json!("native/pay")]
    } else {
        vec![]
    };
    let maximum = value["max_envelope_bytes"]
        .as_u64()
        .ok_or("capabilities missing envelope bound")?;
    if value["schema"] != "neuron/native-capabilities/1"
        || value["profile"] != profile
        || network.len() != 64
        || !network.bytes().all(|b| b.is_ascii_hexdigit())
        || kinds.len() != expected.len()
        || !expected.iter().all(|kind| kinds.contains(kind))
        || maximum == 0
        || maximum > 8 * 1024 * 1024
        || value["idempotency"] != "network-subject-request/1"
        || value["receipt"] != RECEIPT
    {
        return Err("unsupported or inconsistent native capabilities".into());
    }
    // Retain the exact observation for the operator. This is endpoint metadata,
    // not a proof of authority, custody, history completeness or consensus.
    Ok(value)
}

pub fn descriptor(
    home: &Path,
    config: &Config,
    live: bool,
) -> Result<Value, Box<dyn std::error::Error>> {
    let observation = if live { Some(observe(config)?) } else { None };
    let authenticated = observation
        .as_ref()
        .and_then(|v| v["authenticated"].as_bool());
    let network = observation.as_ref().and_then(|v| v["network"].as_str());
    let profile = observation.as_ref().and_then(|v| v["profile"].as_str());
    let mut commands = vec![format!("net set spacepussy-test {}", config.rpc())];
    if authenticated == Some(true) {
        commands.push(format!(
            "net pin spacepussy-test {}",
            network.expect("validated network")
        ));
    }
    Ok(json!({
        "schema":"cyber/connection/v2", "binary":"cyber", "version":env!("CARGO_PKG_VERSION"),
        "network_label":config.network, "network":network, "profile":profile,
        "rpc":config.rpc(), "home":std::fs::canonicalize(home)?, "mode":"local-chaosnet",
        "observation":if live {"endpoint-capabilities"} else {"configuration-only"},
        "status":"/status", "submit":if authenticated == Some(true) {Some("/v3/action")} else {None},
        "endpoints":{"capabilities":"/capabilities", "action":"/v3/action", "receipt":RECEIPT, "history":"/v3/history"},
        "cyb_commands":commands,
        "capabilities":{"graph":true, "replay":true, "peer_sync":false,
            "authenticated_writes":authenticated, "consensus":false, "joy_worker":false},
        "native_capabilities":observation, "receipt_meaning":"endpoint-acceptance", "consensus_finality":false,
    }))
}
