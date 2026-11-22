//! cyber — the phase-1 root chain client (soft3/specs/chain.md).
//!
//! Three things a neuron can do — transfer, cyberlink, claim — signed with
//! the key the spell derives and submitted to a chain node over the HTTP
//! surface of chain-wire §7. Encoding and signing are `soft3::wallet`; this
//! binary is the command line around it.

mod spell;

use std::collections::BTreeMap;
use std::time::Duration;

use serde_json::{json, Value};
use soft3::wallet::{self, hex, Client, Kind, SignedTx, Signer, Tx, TxStatus};

type Result<T> = std::result::Result<T, String>;

const DEFAULT_RPC: &str = "https://cyb.ai/pussy";
const FLAGS: &[&str] = &[
    "--rpc", "--chain-id", "--spell", "--home", "--hrp", "--token", "--fee", "--nonce", "--amount",
    "--valence", "--after", "--limit", "--wait",
];
const SWITCHES: &[&str] = &["--json", "--no-wait", "--help", "--version"];

fn error(e: impl std::fmt::Display) -> String {
    e.to_string()
}

struct Options {
    words: Vec<String>,
    flags: BTreeMap<String, String>,
    switches: Vec<String>,
}

impl Options {
    fn parse(args: impl IntoIterator<Item = String>) -> Result<Self> {
        let mut args = args.into_iter();
        let (mut words, mut flags, mut switches) = (Vec::new(), BTreeMap::new(), Vec::new());
        while let Some(arg) = args.next() {
            if SWITCHES.contains(&arg.as_str()) {
                switches.push(arg);
            } else if let Some((k, v)) = arg.split_once('=').filter(|(k, _)| FLAGS.contains(k)) {
                flags.insert(k.to_string(), v.to_string());
            } else if FLAGS.contains(&arg.as_str()) {
                let value = args.next().ok_or_else(|| format!("missing value for {arg}"))?;
                if flags.insert(arg.clone(), value).is_some() {
                    return Err(format!("duplicate option {arg}"));
                }
            } else if arg.starts_with("--") {
                return Err(format!("unsupported option {arg}; cyber help"));
            } else {
                words.push(arg);
            }
        }
        Ok(Self { words, flags, switches })
    }

    fn value(&self, key: &str) -> Option<&str> {
        self.flags.get(key).map(String::as_str)
    }

    fn on(&self, switch: &str) -> bool {
        self.switches.iter().any(|s| s == switch)
    }

    fn u64(&self, key: &str) -> Result<Option<u64>> {
        self.value(key)
            .map(|v| v.parse::<u64>().map_err(|_| format!("{key} must be an integer, got {v:?}")))
            .transpose()
    }

    fn rpc(&self) -> String {
        self.value("--rpc")
            .map(str::to_string)
            .or_else(|| std::env::var("CYBER_RPC").ok())
            .unwrap_or_else(|| DEFAULT_RPC.into())
    }

    fn client(&self) -> Result<Client> {
        Client::new(&self.rpc()).map_err(error)
    }

    fn chain_id(&self, client: &Client) -> Result<[u8; 32]> {
        let pinned = match self.value("--chain-id").or(std::env::var("CYBER_CHAIN_ID").ok().as_deref()) {
            Some(s) => Some(wallet::parse32(s).map_err(error)?),
            None => None,
        };
        client.chain_id(pinned).map_err(error)
    }

    fn hrp(&self) -> &str {
        self.value("--hrp").unwrap_or("pussy")
    }

    fn signer(&self) -> Result<Signer> {
        let path = spell::path(self.value("--spell"), self.value("--home"))?;
        let words = spell::read(&path)?;
        Signer::from_spell(&words, "").map_err(|e| format!("{}: {e}", path.display()))
    }

    fn token(&self) -> [u8; 32] {
        wallet::token_arg(self.value("--token").unwrap_or("pussy"))
    }

    fn wait(&self) -> Option<Duration> {
        if self.on("--no-wait") {
            return None;
        }
        Some(Duration::from_secs(
            self.value("--wait").and_then(|w| w.parse().ok()).unwrap_or(120),
        ))
    }
}

fn tx_json(tx: &Tx) -> Value {
    match tx {
        Tx::Coinbase { height, to, amount, extra } => json!({
            "kind": "coinbase", "height": height, "to": hex(to), "amount": amount, "extra": hex(extra)
        }),
        Tx::Transfer { from, nonce, token, amount, to, fee } => json!({
            "kind": "transfer", "from": hex(from), "nonce": nonce, "token": hex(token),
            "amount": amount, "to": hex(to), "fee": fee
        }),
        Tx::Cyberlink { from, nonce, token, amount, p, q, valence, fee } => json!({
            "kind": "cyberlink", "from": hex(from), "nonce": nonce, "token": hex(token),
            "amount": amount, "p": hex(p), "q": hex(q), "valence": valence, "fee": fee
        }),
        Tx::Claim { account_id, nonce, fee } => json!({
            "kind": "claim", "account_id": hex(account_id), "nonce": nonce, "fee": fee
        }),
    }
}

/// Sign, submit, and (unless `--no-wait`) wait for inclusion; print the receipt.
fn send(options: &Options, client: &Client, signed: SignedTx) -> Result<()> {
    let mut receipt = json!({
        "schema": "cyber/tx-receipt/1",
        "txid": signed.txid_hex(),
        "chain": hex(&signed.chain),
        "bytes": signed.bytes.len(),
        "tx": tx_json(&signed.tx),
    });
    client.submit(&signed).map_err(error)?;
    receipt["submitted"] = json!(true);
    if !options.on("--json") {
        println!("txid {}", signed.txid_hex());
    }
    match options.wait() {
        None => receipt["status"] = json!("submitted"),
        Some(budget) => match client.wait(&signed.txid, budget).map_err(error)? {
            TxStatus::Included { height, block, .. } => {
                receipt["status"] = json!("included");
                receipt["height"] = json!(height);
                receipt["block"] = json!(block);
                if !options.on("--json") {
                    println!("included at height {height}{}", block.map(|b| format!(" block {b}")).unwrap_or_default());
                }
            }
            other => receipt["status"] = json!(format!("{other:?}")),
        },
    }
    if options.on("--json") {
        println!("{}", serde_json::to_string_pretty(&receipt).map_err(error)?);
    }
    Ok(())
}

fn nonce_for(options: &Options, client: &Client, neuron: &[u8; 32]) -> Result<u64> {
    match options.u64("--nonce")? {
        Some(n) => Ok(n),
        None => client.nonce(neuron).map_err(|e| format!("nonce: {e}")),
    }
}

fn valence(options: &Options) -> Result<i8> {
    let v = options.value("--valence").unwrap_or("0");
    match v {
        "-1" | "0" | "1" | "+1" => Ok(v.trim_start_matches('+').parse().expect("checked")),
        _ => Err(format!("--valence must be -1, 0 or 1, got {v:?}")),
    }
}

fn tx_command(options: &Options, words: &[&str]) -> Result<()> {
    let client = options.client()?;
    if let ["show", txid] = words {
        let txid = wallet::parse32(txid).map_err(error)?;
        let status = client.tx(&txid).map_err(error)?;
        let v = match status {
            TxStatus::Unknown => json!({"txid": hex(&txid), "status": "unknown"}),
            TxStatus::Mempool => json!({"txid": hex(&txid), "status": "mempool"}),
            TxStatus::Included { height, block, tx } => {
                json!({"txid": hex(&txid), "status": "included", "height": height, "block": block, "tx": tx})
            }
        };
        println!("{}", serde_json::to_string_pretty(&v).map_err(error)?);
        return Ok(());
    }
    let signer = options.signer()?;
    let chain = options.chain_id(&client)?;
    let fee = options.u64("--fee")?;
    let signed = match words {
        ["transfer", to, amount] => {
            let amount = amount.parse::<u64>().map_err(|_| format!("amount must be an integer, got {amount:?}"))?;
            let nonce = nonce_for(options, &client, &signer.neuron)?;
            signer.transfer(&chain, nonce, options.token(), amount, wallet::card(to), fee)
        }
        ["link", p, q] => {
            let nonce = nonce_for(options, &client, &signer.neuron)?;
            let amount = options.u64("--amount")?.unwrap_or(0);
            signer.cyberlink(&chain, nonce, wallet::card(p), wallet::card(q), valence(options)?, options.token(), amount, fee)
        }
        ["claim"] => {
            let nonce = nonce_for(options, &client, &signer.neuron)?;
            signer.claim(&chain, nonce, fee)
        }
        _ => return Err("cyber tx transfer TO AMOUNT | link P Q | claim | show TXID".into()),
    }
    .map_err(error)?;
    send(options, &client, signed)
}

fn neuron_command(options: &Options, words: &[&str]) -> Result<()> {
    if words == ["new"] {
        let path = spell::path(options.value("--spell"), options.value("--home"))?;
        let words = Signer::generate_spell().map_err(error)?;
        spell::write_new(&path, &words)?;
        println!("minted {}", path.display());
    }
    let signer = options.signer()?;
    let v = json!({
        "neuron": signer.neuron_hex(),
        "pubkey": hex(&signer.pubkey),
        "account_id": hex(&signer.account_id),
        "address": signer.address(options.hrp()).map_err(error)?,
        "legacy_owner": hex(&signer.legacy_owner(options.hrp())),
        "spell": spell::path(options.value("--spell"), options.value("--home"))?,
    });
    println!("{}", serde_json::to_string_pretty(&v).map_err(error)?);
    Ok(())
}

fn owner_arg(options: &Options, arg: Option<&&str>) -> Result<[u8; 32]> {
    match arg {
        Some(s) => Ok(wallet::card(s)),
        None => Ok(options.signer()?.neuron),
    }
}

fn run(options: Options) -> Result<()> {
    let words: Vec<&str> = options.words.iter().map(String::as_str).collect();
    if options.on("--help") || matches!(words.as_slice(), ["help" | "-h" | "?"]) {
        print!("{}", include_str!("help.txt"));
        return Ok(());
    }
    if options.on("--version") || matches!(words.as_slice(), ["version" | "-V"]) {
        println!("cyber {} (true-cyber) · soft3 {} · chain soft3/chain/1", env!("CARGO_PKG_VERSION"), soft3::VERSION);
        return Ok(());
    }
    match words.as_slice() {
        [] | ["status"] => {
            let s = options.client()?.status().map_err(error)?;
            if options.on("--json") {
                let fields: serde_json::Map<String, Value> =
                    s.fields.iter().map(|(k, v)| (k.clone(), json!(v))).collect();
                println!("{}", serde_json::to_string_pretty(&json!({
                    "schema": "cyber/status/1", "rpc": options.rpc(), "chain": s.chain,
                    "chain_id": s.chain_id.map(|c| hex(&c)), "height": s.height, "tip": s.tip,
                    "root": s.root, "catching_up": s.catching_up, "fields": fields,
                })).map_err(error)?);
            } else {
                println!("chain      {}", s.chain);
                println!("chain-id   {}", s.chain_id.map(|c| hex(&c)).unwrap_or_else(|| "(not published)".into()));
                println!("height     {}", s.height);
                println!("tip        {}", s.tip);
                println!("root       {}", s.root);
                println!("catching   {}", s.catching_up);
                for key in ["protocol", "rule-set", "chainwork", "bits", "supply", "conserved", "mempool", "peers", "mining"] {
                    if let Some(v) = s.field(key) {
                        println!("{key:<10} {v}");
                    }
                }
            }
            Ok(())
        }
        ["balance", rest @ ..] => {
            let client = options.client()?;
            let owner = owner_arg(&options, rest.first())?;
            let token = options.value("--token").map(wallet::token_arg);
            let balance = client.balance(&owner, token.as_ref()).map_err(error)?;
            if options.on("--json") {
                println!("{}", json!({"owner": hex(&owner), "token": token.map(|t| hex(&t)), "balance": balance}));
            } else {
                println!("{balance}");
            }
            Ok(())
        }
        ["balances", rest @ ..] => {
            let owner = owner_arg(&options, rest.first())?;
            print!("{}", options.client()?.balances(&owner).map_err(error)?);
            Ok(())
        }
        ["nonce", rest @ ..] => {
            let neuron = owner_arg(&options, rest.first())?;
            println!("{}", options.client()?.nonce(&neuron).map_err(error)?);
            Ok(())
        }
        ["tip"] => {
            println!("{}", serde_json::to_string_pretty(&options.client()?.tip().map_err(error)?).map_err(error)?);
            Ok(())
        }
        ["block", which] => {
            println!("{}", serde_json::to_string_pretty(&options.client()?.block(which).map_err(error)?).map_err(error)?);
            Ok(())
        }
        ["history"] => {
            let after = options.u64("--after")?;
            let limit = options.u64("--limit")?.unwrap_or(16);
            println!("{}", serde_json::to_string_pretty(&options.client()?.history(after, limit).map_err(error)?).map_err(error)?);
            Ok(())
        }
        ["tx", rest @ ..] => tx_command(&options, rest),
        ["neuron", rest @ ..] => neuron_command(&options, rest),
        ["fees"] => {
            for k in [Kind::Transfer, Kind::Cyberlink, Kind::Claim] {
                println!("{:<10} {:>3} bytes  fee floor {}", k.name(), k.total_bytes(), k.min_fee());
            }
            Ok(())
        }
        _ => Err("unknown command or missing arguments; cyber help".into()),
    }
}

fn main() {
    if let Err(e) = Options::parse(std::env::args().skip(1)).and_then(run) {
        eprintln!("cyber: {e}");
        std::process::exit(1);
    }
}
