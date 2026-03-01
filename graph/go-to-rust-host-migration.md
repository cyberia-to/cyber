# Phase 3 Deep Dive: Go → Rust Host Migration

## The Four Go Dependencies

go-cyber currently depends on four major Go codebases. Each requires a distinct replacement strategy.

```
go-cyber binary (single Go process)
├── cosmos-sdk v0.47.x    ~180K lines Go    App framework (bank, staking, gov, auth...)
├── CometBFT v0.37.x      ~100K lines Go    BFT consensus engine + P2P + mempool
├── ibc-go v7.x            ~60K lines Go     Inter-Blockchain Communication protocol
└── wasmd v0.45.x          ~15K lines Go     CosmWasm host module (x/wasm)
                           ─────────────
                           ~355K lines Go    Total external Go dependencies
```

The previous migration plan treated Phase 3 as "assembly" — just plug together existing Rust crates. That was wrong. Each component has a different maturity level, different integration pattern, and different risk profile.

---

## 1. CometBFT → Consensus Layer

### Reality Check

There is **no production Rust implementation of Tendermint/CometBFT consensus**. Every Rust blockchain in the Cosmos ecosystem — Penumbra, Namada, Nomic — runs CometBFT as a **separate Go process** communicating over TCP via ABCI.

```
┌─────────────────────┐     TCP/26658      ┌──────────────────────┐
│  CometBFT (Go)      │ ◄────────────────► │  Rust ABCI App       │
│  - P2P networking    │     ABCI protocol  │  - State machine     │
│  - Mempool           │                    │  - Transaction exec  │
│  - Consensus rounds  │                    │  - Query handling    │
│  - Block production  │                    │  - State storage     │
└─────────────────────┘                     └──────────────────────┘
```

### What Exists in Rust

| Crate | Maintainer | What It Does | What It Does NOT Do |
|---|---|---|---|
| `tendermint-rs` | Informal Systems / CometBFT | ABCI types, protobuf, light client, P2P (limited) | Run consensus, produce blocks |
| `tower-abci` | Penumbra | Async ABCI server on Tower/tokio, handles 4 connections | Replace CometBFT |
| `tendermint-abci` | Informal Systems | Sync ABCI server, blocking API | Replace CometBFT |
| `abci2` | Turbofish (Nomic) | Low-level ABCI protocol server | Replace CometBFT |

### Architecture Decision: Two-Process Model

**Phase 3 cannot eliminate the CometBFT Go binary for consensus.** This is not a failure — it is how every serious Rust chain in the ecosystem operates.

```
Deployment: Two processes, one machine
─────────────────────────────────────
$ cometbft start                    # Go binary — consensus, P2P, mempool
$ cyber-node start                  # Rust binary — app logic, state, contracts
                                    # connects to cometbft:26658 via ABCI
```

**Why this is acceptable:**
- CometBFT is an unmodified, audited binary from official releases
- No fork maintenance burden — use stock releases
- The Rust app owns ALL state, ALL logic, ALL contract execution
- CometBFT is a dumb consensus pipe — it knows nothing about cyberlinks, ranks, or bandwidth

**Why NOT rewrite CometBFT in Rust:**
- ~100K lines of battle-tested BFT consensus
- Zero benefit for Bostrom's unique logic (rank, graph, bandwidth are all in the app layer)
- Consensus bugs are existential — not worth the risk for zero functional gain
- If a Rust BFT engine emerges (from Anoma, Penumbra, etc.), it can be swapped in later via the ABCI interface

### Implementation: tower-abci Integration

```rust
// cyber-node/src/main.rs
use tower_abci::{Server, split};
use cyber_app::CyberApp;

#[tokio::main]
async fn main() {
    let app = CyberApp::new(config);
    
    // tower-abci splits into 4 services (consensus, mempool, info, snapshot)
    let (consensus, mempool, info, snapshot) = split::service(app, 1);
    
    let server = Server::builder()
        .consensus(consensus)
        .mempool(mempool)
        .info(info)
        .snapshot(snapshot)
        .finish()
        .unwrap();
    
    // CometBFT connects here
    server.listen_tcp("127.0.0.1:26658").await.unwrap();
}
```

The `CyberApp` implements Tower's `Service<Request>` trait, processing ABCI requests:
- `InitChain` — genesis, contract instantiation
- `PrepareProposal` / `ProcessProposal` — ABCI++ (v0.38+)
- `FinalizeBlock` — execute transactions, update state
- `Commit` — persist state, return app hash
- `Query` — read state (gRPC compatible)
- `CheckTx` — mempool validation (bandwidth checks here)

### CometBFT Version Strategy

Current go-cyber uses CometBFT v0.37 (ABCI v1). Options:

| Version | ABCI | Key Feature | Consideration |
|---|---|---|---|
| v0.37 | v1 | Stable, widely used | Penumbra, Namada use this |
| v0.38 | v2 (ABCI++) | PrepareProposal, ProcessProposal, VoteExtensions | Enables MEV protection, oracle voting |

**Recommendation**: Target v0.38 for ABCI++ — `PrepareProposal` allows the Rust app to control block construction (rank updates, bandwidth scheduling). tower-abci supports v0.37 wire format; v0.38 support via `tendermint-rs` v0.38+.

### Metrics

| Metric | Current (go-cyber) | Phase 3 |
|---|---|---|
| CometBFT Go binary | Embedded in same process | Separate process, stock binary |
| ABCI communication | In-process Go calls | TCP socket (tower-abci) |
| Latency overhead | 0 (same process) | ~0.1ms per ABCI call (loopback TCP) |
| Custom Go for consensus | 0 lines | 0 lines |
| Maintenance burden | Full Cosmos SDK upgrade path | Download new cometbft release |

---

## 2. cosmos-sdk → Rust Application Framework

### What cosmos-sdk Actually Does

The SDK provides the "application shell" — everything between consensus and business logic:

```
cosmos-sdk responsibilities:
├── BaseApp          ABCI routing, transaction decoding, gas metering
├── x/auth           Accounts, signatures, authentication
├── x/bank           Token balances, transfers, supply tracking
├── x/staking        Validator set, delegations, slashing
├── x/gov            Governance proposals, voting
├── x/distribution   Reward distribution
├── x/slashing       Jail/unjail, downtime tracking
├── x/evidence       Double-sign evidence handling
├── x/params         On-chain parameter store
├── x/upgrade        Software upgrade coordination
├── x/crisis         Invariant checking
├── Store            IAVL tree, multistore, commitment
└── AnteHandler      Pre-execution checks (fees, signatures, sequence)
```

This is ~180K lines of Go. It is the **hardest piece to replace** because there is no single production-ready Rust equivalent.

### Existing Rust Alternatives

| Project | Status | Approach | Production? | IBC? | CosmWasm? |
|---|---|---|---|---|---|
| **Gears** (Rumos) | Early, 31★ | Direct Cosmos SDK port in Rust | No | No | No |
| **Orga** (Turbofish/Nomic) | Production | Custom framework, Tendermint-based | Yes (Nomic) | Yes (custom) | No |
| **Penumbra** | Production | Custom UTXO-based, no SDK | Yes | Yes (custom ibc-types) | No |
| **Namada** | Production | Custom account-based, full IBC | Yes | Yes (ibc-rs) | WASM VM (not CW) |

### Analysis: Why None Fit Directly

**Gears** — Most SDK-compatible but too immature. No IBC, no CosmWasm, minimal modules. Would require massive development to be production-ready.

**Orga** — Production-proven (Nomic mainnet), high performance (Merk store claims 2-20x faster than JMT), includes staking, but custom IBC implementation and no CosmWasm. The "Orga way" is to write state machines directly in Rust with derive macros, not through contracts.

**Penumbra** — Wrong model (UTXO, not account-based). Extractable components: tower-abci, JMT, penumbra-storage patterns.

**Namada** — Closest architecturally (Rust + CometBFT + IBC + account model), but deeply coupled to their privacy features (MASP). Not a framework, it's a specific chain.

### The Bostrom Approach: Minimal Custom Framework

Instead of porting 180K lines of cosmos-sdk, build a **minimal Rust app framework** that only implements what Bostrom actually uses. The critical insight: **most SDK modules are already being moved to CosmWasm contracts in Phase 1**.

After Phase 1, what the Rust host actually needs to implement natively:

```
cyber-sdk (Rust, minimal)
├── BaseApp (~2,000 lines)
│   ├── ABCI routing (tower-abci Service impl)
│   ├── Transaction decoding (protobuf via prost)
│   ├── Gas metering (simple counter)
│   └── AnteHandler chain (signatures, bandwidth, fees)
│
├── x/auth (~1,500 lines)  
│   ├── Account storage (address → account)
│   ├── Signature verification (secp256k1 via k256, ed25519-dalek)
│   ├── Sequence numbers (replay protection)
│   └── Module accounts (for contracts)
│
├── x/bank (~1,000 lines)
│   ├── Balance storage (address × denom → amount)
│   ├── Transfer logic (with hooks for bandwidth)
│   ├── Supply tracking
│   └── TokenFactory integration (create/mint/burn denoms)
│
├── x/staking (~2,500 lines)
│   ├── Validator set (bonded, unbonding, unbonded)
│   ├── Delegations (delegate, undelegate, redelegate)
│   ├── Slashing (downtime, double-sign)
│   ├── Reward distribution (block rewards → validators → delegators)
│   └── Validator power updates → CometBFT via EndBlock
│
├── x/wasm-host (~2,000 lines)
│   ├── Contract lifecycle (store, instantiate, execute, migrate, sudo)
│   ├── Gas forwarding (host gas → wasm gas)
│   ├── Message dispatch (contract submessages → host)
│   └── Query routing (contract queries → host modules or other contracts)
│
├── x/upgrade (~500 lines)
│   ├── Upgrade proposal handling
│   ├── Binary halt at upgrade height
│   └── Migration handler registration
│
├── Store (~1,500 lines)
│   ├── Merkle tree (JMT or Merk)
│   ├── Multi-store (one subtree per module)
│   ├── IAVL-compatible proofs (for IBC, or migrate to JMT proofs)
│   └── State snapshots (for fast sync)
│
└── Total: ~11,000 lines Rust
    vs 180,000 lines Go cosmos-sdk
```

**Why so much smaller?**
- No EVM, no Ethermint compatibility
- Governance → CosmWasm contract (cw-governance or DAO-DAO)
- Liquidity → CosmWasm contract (cw-liquidity)
- Distribution → simplified, embedded in staking
- Crisis/evidence → simplified
- Params → contract-based or hardcoded per upgrade
- No legacy amino encoding
- No REST API legacy (gRPC only via tonic)

### Prior Art to Fork/Study

For each module, the best reference implementation:

| Module | Best Reference | Why |
|---|---|---|
| BaseApp / ABCI routing | Namada `namada_node::shell` | Full ABCI++ impl, Rust, production |
| Auth / signatures | Penumbra `penumbra-custody` | Rust crypto, account handling |
| Bank / balances | Orga `orga::coins` | O(1) operations, Rust native |
| Staking | Orga `orga::plugins::staking` | O(1) delegation/rewards, production (Nomic) |
| Store / Merkle | Merk (Nomic) or JMT (Penumbra) | Both production; Merk faster, JMT more ecosystem-compatible |
| Wasm host | cosmwasm-vm directly | Already Rust, just needs host glue |
| Upgrade | Namada upgrade handler | Simple, production-tested |

### Store Decision: Merk vs JMT vs IAVL-compatible

| Store | Origin | Performance | IBC Proofs | Ecosystem |
|---|---|---|---|---|
| IAVL | Cosmos SDK | Baseline | Native (ICS-23) | All Cosmos chains |
| JMT (jellyfish-merkle) | Aptos → Penumbra, Sovereign | ~2x IAVL | Requires adapter | Growing |
| Merk | Nomic/Turbofish | 2-20x IAVL (claimed) | Custom, ICS-23 adapter needed | Nomic only |

**Recommendation**: Start with JMT (wider ecosystem, Penumbra + Sovereign Labs co-develop, ICS-23 proof adapter exists). The store is behind an abstraction trait — can benchmark Merk later and swap if needed.

```rust
// cyber-sdk/src/store/mod.rs
pub trait MerkleStore: Send + Sync {
    fn get(&self, key: &[u8]) -> Option<Vec<u8>>;
    fn set(&mut self, key: &[u8], value: &[u8]);
    fn delete(&mut self, key: &[u8]);
    fn root_hash(&self) -> [u8; 32];
    fn prove(&self, key: &[u8]) -> ICS23Proof;  // IBC-compatible proofs
    fn commit(&mut self) -> [u8; 32];
}

// JMT implementation
pub struct JmtStore { /* jellyfish-merkle + RocksDB */ }
impl MerkleStore for JmtStore { /* ... */ }

// Future: Merk implementation
pub struct MerkStore { /* merk AVL + RocksDB */ }
impl MerkleStore for MerkStore { /* ... */ }
```

### State Migration: IAVL → JMT

This is a critical risk. All existing state is in IAVL tree format. IBC relayers expect IAVL proofs.

**Migration strategy:**
1. At Phase 3 upgrade height, export all KV pairs from IAVL
2. Import into JMT (or Merk)
3. New proofs use JMT format
4. IBC client update: counterparty chains must update their Bostrom light client to accept new proof format
5. Alternatively: implement ICS-23 proof generation that wraps JMT proofs in ICS-23 format (Penumbra does this)

```
Phase 3 upgrade block:
  1. HALT chain
  2. Export: iterate all IAVL leaves → KV dump
  3. Import: insert all KV pairs into JMT
  4. Compute new JMT root hash
  5. Resume with Rust binary
  6. IBC channels: governance proposal on counterparty chains to update client
```

**Risk mitigation**: Run parallel JMT store for 1000+ blocks on testnet. Compare every proof against IAVL to verify correctness before mainnet.

---

## 3. ibc-go → ibc-rs

### Current State of ibc-rs

ibc-rs (`cosmos/ibc-rs`) is the most mature Rust replacement in this migration. Maintained by Informal Systems (same team as Hermes relayer and tendermint-rs).

```
ibc-rs crate structure:
├── ibc                   Re-exports everything
├── ibc-core              Core protocol (connections, channels, packets)
│   ├── ics02-client      Client creation, update, misbehaviour
│   ├── ics03-connection  Connection handshake (4-step)
│   ├── ics04-channel     Channel lifecycle, packet send/recv/ack/timeout
│   └── ics24-host        Host requirements, path definitions
├── ibc-clients           Light client implementations
│   ├── ics07-tendermint  Tendermint light client (most common)
│   └── ics08-wasm        Wasm-based custom light clients
├── ibc-apps              Application layer
│   ├── ics20-transfer    Fungible token transfer
│   └── ics721-nft        NFT transfer
├── ibc-testkit           Testing framework
└── ibc-query             Query utilities
```

### Integration Model

ibc-rs defines two key traits the host chain must implement:

```rust
/// The host provides read-only access to state
pub trait ValidationContext {
    type ClientValidationContext;
    type E: ExtClientValidationContext;
    
    fn host_height(&self) -> Height;
    fn host_timestamp(&self) -> Timestamp;
    fn client_state(&self, client_id: &ClientId) -> Result<Box<dyn ClientState>>;
    fn consensus_state(&self, path: &ClientConsensusStatePath) -> Result<Box<dyn ConsensusState>>;
    fn connection_end(&self, conn_id: &ConnectionId) -> Result<ConnectionEnd>;
    fn channel_end(&self, port_channel_id: &PortChannelId) -> Result<ChannelEnd>;
    // ... ~20 more methods for state reads
}

/// The host provides write access to state  
pub trait ExecutionContext: ValidationContext {
    type E: ExtClientExecutionContext;
    
    fn store_client_state(&mut self, path: ClientStatePath, state: Box<dyn ClientState>);
    fn store_consensus_state(&mut self, path: ClientConsensusStatePath, state: Box<dyn ConsensusState>);
    fn store_connection(&mut self, path: ConnectionPath, conn: ConnectionEnd);
    fn store_channel(&mut self, path: ChannelPath, channel: ChannelEnd);
    fn store_packet_commitment(&mut self, path: CommitmentPath, commitment: PacketCommitment);
    // ... write methods matching validation reads
}
```

The cyber-sdk implements these traits against its own store (JMT or Merk), and ibc-rs handles all the protocol logic.

### What Works vs What Needs Work

| Feature | Status | Notes |
|---|---|---|
| ICS-02 Client | ✅ Production | Create, update, misbehaviour |
| ICS-03 Connection | ✅ Production | Full 4-step handshake |
| ICS-04 Channel | ✅ Production | Ordered + unordered channels |
| ICS-20 Transfer | ✅ Production | Fungible token transfer |
| ICS-07 Tendermint Client | ✅ Production | Used by Namada |
| ICS-08 Wasm Client | ✅ Available | CosmWasm light clients |
| ICS-27 Interchain Accounts | ⚠️ Not in ibc-rs | Only in ibc-go. Would need custom impl or skip |
| ICS-721 NFT Transfer | ✅ Available | Contributed by Heliax/Namada |
| Relayer compatibility | ✅ Hermes | Hermes is Rust, native ibc-rs types |

### IBC Proof Compatibility

**Critical issue**: IBC relayers verify proofs from the counterparty chain. If Bostrom switches from IAVL to JMT, the proof format changes.

**Solution approach (Penumbra's model):**
- ibc-rs uses ICS-23 proof specs
- JMT proofs can be wrapped in ICS-23 `CommitmentProof` format
- The Tendermint light client on counterparty chains verifies the app hash (root hash)
- As long as the root hash is correct and the proof verifies against it, the internal tree structure doesn't matter

```rust
// Proof generation for IBC
impl CyberApp {
    fn ibc_proof(&self, path: &str, height: Height) -> ICS23Proof {
        let jmt_proof = self.store.prove(path.as_bytes());
        // Wrap JMT proof in ICS-23 format
        ics23::CommitmentProof {
            proof: Some(ics23::commitment_proof::Proof::Exist(
                ics23::ExistenceProof {
                    key: path.as_bytes().to_vec(),
                    value: jmt_proof.value,
                    leaf: Some(jmt_proof.leaf_spec()),
                    path: jmt_proof.inner_nodes(),
                }
            ))
        }
    }
}
```

### Penumbra's IBC Divergence (Lesson)

Penumbra couldn't use ibc-rs directly due to their async state model. They built `ibc-types` (minimal domain types) + `penumbra-ibc` (async implementation). Their experience:

- ibc-rs API went through significant churn
- Tight coupling between ibc-rs and synchronous state access
- Solution: `ibc-types` as shared vocabulary, implementation-specific IBC handlers

**For Bostrom**: ibc-rs's `ValidationContext`/`ExecutionContext` split (ADR-005) was specifically designed to support different host architectures (prompted by Namada's needs). Bostrom's synchronous state model fits well. Use ibc-rs directly — don't repeat Penumbra's fork path.

---

## 4. wasmd → Rust CosmWasm Host

### The CosmWasm Stack

```
Current stack (Go + Rust):
──────────────────────────
CW Contracts (.wasm)     ← Pure Rust, compiled to Wasm
        │
cosmwasm-std             ← Rust: contract API (messages, queries, storage)
        │
cosmwasm-vm              ← Rust: Wasmer engine, gas metering, caching  ← ALREADY RUST
        │
wasmvm                   ← Go↔Rust FFI bridge (libwasmvm.so/dylib)    ← ELIMINATED
        │
x/wasm (wasmd)           ← Go: contract lifecycle, permissions, IBC    ← MUST REPLACE
        │
cosmos-sdk               ← Go: BaseApp, bank, staking...               ← MUST REPLACE
```

### What's Already Rust (cosmwasm-vm)

The **entire VM is already Rust**. `cosmwasm-vm` crate provides:
- Wasmer-based Wasm execution engine
- Gas metering (wasm instruction counting)
- Instance caching (pinned contracts stay in memory)
- Import resolution (what the contract can call)
- Memory management (allocate/deallocate across Wasm boundary)

In Phase 3, `cosmwasm-vm` is embedded directly — no FFI bridge:

```rust
// Direct Rust integration — no Go, no FFI, no wasmvm
use cosmwasm_vm::{Cache, CacheOptions, Instance, Backend};

let cache = Cache::new(CacheOptions {
    base_dir: PathBuf::from("/data/wasm"),
    available_capabilities: HashSet::from(["staking", "iterator", "cyber"]),
    memory_cache_size: Size::mebi(512),
    instance_memory_limit: Size::mebi(32),
});
```

### What Must Be Built: x/wasm-host (Rust)

The Go `x/wasm` module (~15K lines) handles everything around the VM. This must be reimplemented in Rust:

```rust
// cyber-sdk/src/wasm_host/mod.rs

pub struct WasmHost {
    cache: cosmwasm_vm::Cache<CyberApi, CyberStorage, CyberQuerier>,
    code_store: CodeStore,        // code_id → wasm bytes
    contract_store: ContractStore, // address → contract info
    state_store: StateStore,       // (address, key) → value
}

impl WasmHost {
    /// Store new Wasm code, return code_id
    pub fn store_code(&mut self, sender: &Addr, wasm: &[u8]) -> Result<u64> {
        // 1. Validate Wasm (magic bytes, section checks)
        // 2. Check capabilities (imports match available)
        // 3. Compile and cache
        // 4. Store code hash → code_id mapping
        // 5. Emit event
    }
    
    /// Instantiate a contract from stored code
    pub fn instantiate(
        &mut self, sender: &Addr, code_id: u64, 
        msg: &[u8], label: &str, funds: &[Coin],
    ) -> Result<(Addr, Response)> {
        // 1. Generate contract address (instantiate2 predictable or hash-based)
        // 2. Create contract info entry
        // 3. Transfer funds from sender to contract
        // 4. Call contract's instantiate entry point via cosmwasm-vm
        // 5. Process response: messages, events, attributes
        // 6. Handle submessages recursively
    }
    
    /// Execute a contract
    pub fn execute(
        &mut self, sender: &Addr, contract: &Addr,
        msg: &[u8], funds: &[Coin],
    ) -> Result<Response> {
        // 1. Load contract info (code_id, admin)
        // 2. Transfer funds
        // 3. Build Env (block info, contract info, transaction info)
        // 4. Call contract's execute entry point
        // 5. Process response + submessages
    }
    
    /// Sudo — privileged call from host (rank updates, epoch hooks)
    pub fn sudo(&mut self, contract: &Addr, msg: &[u8]) -> Result<Response> {
        // No sender, no funds — direct host → contract call
        // Used by: rank-engine pushing rank updates, cron triggers for dmn
    }
    
    /// Migrate contract to new code
    pub fn migrate(
        &mut self, sender: &Addr, contract: &Addr, 
        new_code_id: u64, msg: &[u8],
    ) -> Result<Response> {
        // Only admin can migrate
        // Updates code_id, calls migrate entry point
    }
    
    /// Process contract response (submessages, bank sends, wasm calls)
    fn process_response(&mut self, response: Response) -> Result<Vec<Event>> {
        // This is the complex part — recursive message dispatch:
        // CosmosMsg::Bank(BankMsg::Send{..}) → self.bank.send()
        // CosmosMsg::Wasm(WasmMsg::Execute{..}) → self.execute() (recursive)
        // CosmosMsg::Staking(StakingMsg::Delegate{..}) → self.staking.delegate()
        // CosmosMsg::Custom(CyberMsg::...) → custom cyber message routing
        // SubMsg with reply_on → capture result, call reply entry point
    }
}
```

### Custom Messages: CyberMsg Integration

The most interesting part — Bostrom contracts emit `CyberMsg` custom messages that the host must route:

```rust
// cyber-sdk/src/wasm_host/custom.rs

/// Custom message types that contracts can send to the host
#[cw_serde]
pub enum CyberMsg {
    /// Create a cyberlink (from particle → to particle)
    Cyberlink { links: Vec<Link> },
    /// Investmint bandwidth/resources
    Investmint { amount: Coin, resource: String, length: u64 },
    /// Route energy
    CreateRoute { destination: String, alias: String },
    EditRoute { destination: String, value: Coin },
    DeleteRoute { destination: String },
}

/// Custom query types that contracts can ask the host
#[cw_serde]
pub enum CyberQuery {
    /// Get current rank for a particle
    ParticleRank { particle: String },
    /// Get bandwidth state for an address
    BandwidthLoad { address: String },
    /// Search by particle content
    Search { particle: String, page: Option<u32> },
    /// Get cyberlinks from/to a particle
    Backlinks { particle: String, page: Option<u32> },
}

impl WasmHost {
    fn handle_custom_msg(&mut self, msg: CyberMsg) -> Result<Vec<Event>> {
        match msg {
            CyberMsg::Cyberlink { links } => {
                // Write to graph storage directly (native, not contract)
                self.graph.create_links(&links)?;
                // Trigger rank recalculation flag
                self.rank_dirty = true;
                Ok(vec![Event::new("cyberlink").add_attribute("count", links.len())])
            }
            // ... other handlers
        }
    }
    
    fn handle_custom_query(&self, query: CyberQuery) -> Result<Binary> {
        match query {
            CyberQuery::ParticleRank { particle } => {
                let rank = self.rank_store.get_rank(&particle)?;
                to_json_binary(&RankResponse { rank })
            }
            // ... other handlers
        }
    }
}
```

### Backend Trait for cosmwasm-vm

cosmwasm-vm requires the host to implement a `Backend` providing storage, API, and querier:

```rust
use cosmwasm_vm::{Backend, Storage, Api, Querier};

/// Host-provided storage for each contract instance
struct CyberStorage {
    prefix: Vec<u8>,  // contract address as prefix
    store: Arc<RwLock<JmtStore>>,
}

impl Storage for CyberStorage {
    fn get(&self, key: &[u8]) -> BackendResult<Option<Vec<u8>>> { /* prefixed read */ }
    fn set(&mut self, key: &[u8], value: &[u8]) -> BackendResult<()> { /* prefixed write */ }
    fn remove(&mut self, key: &[u8]) -> BackendResult<()> { /* prefixed delete */ }
    fn range(...) -> BackendResult<Records> { /* prefixed iteration */ }
}

/// Host-provided crypto and address API
struct CyberApi;

impl Api for CyberApi {
    fn addr_validate(&self, input: &str) -> BackendResult<Addr> { /* bech32 validate */ }
    fn addr_canonicalize(&self, input: &str) -> BackendResult<CanonicalAddr> { /* bech32 → bytes */ }
    fn addr_humanize(&self, canonical: &CanonicalAddr) -> BackendResult<Addr> { /* bytes → bech32 */ }
    fn secp256k1_verify(&self, hash: &[u8], sig: &[u8], pk: &[u8]) -> BackendResult<bool> { /* k256 */ }
    fn ed25519_verify(&self, msg: &[u8], sig: &[u8], pk: &[u8]) -> BackendResult<bool> { /* ed25519-dalek */ }
    // ...
}

/// Host-provided querier for cross-contract and host-module queries
struct CyberQuerier {
    bank: Arc<BankModule>,
    staking: Arc<StakingModule>,
    wasm_host: Arc<WasmHost>,  // for cross-contract queries
    custom: Arc<CyberCustomQuerier>,  // rank, graph, bandwidth
}

impl Querier for CyberQuerier {
    fn query_raw(&self, request: &[u8], gas_limit: u64) -> BackendResult<SystemResult<ContractResult<Binary>>> {
        let query: QueryRequest<CyberQuery> = from_json(request)?;
        match query {
            QueryRequest::Bank(bank_query) => self.bank.handle(bank_query),
            QueryRequest::Staking(staking_query) => self.staking.handle(staking_query),
            QueryRequest::Wasm(wasm_query) => self.wasm_host.handle(wasm_query),
            QueryRequest::Custom(cyber_query) => self.custom.handle(cyber_query),
            _ => Err(BackendError::unknown("unsupported query")),
        }
    }
}
```

### IBC Contract Support

wasmd's IBC integration allows contracts to be IBC-enabled (receive/send packets). In Rust:

```rust
impl WasmHost {
    /// Called by ibc-rs when a packet arrives for a wasm-port
    pub fn ibc_packet_receive(
        &mut self, 
        contract: &Addr,
        packet: &Packet,
    ) -> Result<IbcReceiveResponse> {
        let msg = IbcPacketReceiveMsg::new(packet.clone(), relayer);
        let env = self.build_env(contract)?;
        
        // Call contract's ibc_packet_receive entry point
        let response = self.call_ibc_packet_receive(contract, &env, &msg)?;
        
        // Process response (acknowledgement, submessages)
        self.process_ibc_response(response)
    }
    
    // Similarly: ibc_channel_open, ibc_channel_connect, ibc_channel_close,
    //           ibc_packet_ack, ibc_packet_timeout
}
```

---

## 5. Integrated Architecture

### Complete Phase 3 Binary

```
cyber-node (single Rust binary + CometBFT sidecar)
│
├── main.rs
│   └── tower-abci Server setup
│
├── cyber-sdk/                    (~11,000 lines Rust)
│   ├── app.rs                    ABCI routing, tx decoding
│   ├── ante.rs                   AnteHandler: sig verify, bandwidth, gas
│   ├── auth/                     Accounts, sequences, module accounts
│   ├── bank/                     Balances, transfers, TokenFactory
│   ├── staking/                  Validators, delegations, rewards, slashing
│   ├── store/                    JMT multi-store, ICS-23 proofs
│   ├── upgrade/                  Software upgrade handler
│   └── wasm_host/                CosmWasm host (~2,000 lines)
│       ├── lifecycle.rs          store/instantiate/execute/migrate/sudo
│       ├── dispatch.rs           Submessage routing, recursive calls
│       ├── custom.rs             CyberMsg/CyberQuery handlers
│       ├── ibc.rs                IBC packet/channel hooks
│       └── backend.rs            Storage/Api/Querier for cosmwasm-vm
│
├── ibc-integration/              (~1,500 lines Rust)
│   ├── context.rs                ValidationContext + ExecutionContext for ibc-rs
│   ├── router.rs                 Port → module routing (transfer, wasm-ports)
│   └── proofs.rs                 JMT → ICS-23 proof generation
│
├── rank-engine/                  (~2,000 lines Rust)  
│   ├── wgpu_compute.rs           GPU PageRank (WGSL shader)
│   ├── graph.rs                  CSR format, graph updates
│   ├── merkle.rs                 Rank Merkle tree
│   └── integration.rs            BeginBlock trigger, sudo push to cw-rank-verifier
│
├── graph-store/                  (~500 lines Rust)
│   └── Native cyberlink storage (high-performance, not in contract)
│
├── bandwidth/                    (~800 lines Rust)
│   └── Native bandwidth accounting + AnteHandler integration
│
├── Cargo.toml dependencies:
│   ├── tower-abci = "0.16"       ABCI server
│   ├── tendermint = "0.38"       Types, proto, light client
│   ├── ibc = "0.54"              Full IBC protocol
│   ├── cosmwasm-vm = "2.2"       Wasm contract VM
│   ├── jmt = "0.10"              Jellyfish Merkle tree
│   ├── wgpu = "24.0"             GPU compute
│   ├── rocksdb = "0.22"          Storage backend
│   ├── prost = "0.13"            Protobuf
│   ├── tonic = "0.12"            gRPC server
│   ├── k256 = "0.13"             secp256k1
│   └── ed25519-dalek = "2.1"     Ed25519
│
└── Total: ~18,000 lines Rust (vs ~355K lines Go dependencies)

+ CometBFT v0.38.x (stock Go binary, separate process)
```

### Dependency Comparison

| Component | Go (current) | Rust (Phase 3) | Reduction |
|---|---|---|---|
| App framework | cosmos-sdk ~180K lines | cyber-sdk ~11K lines | 94% |
| Consensus | CometBFT ~100K lines | CometBFT ~100K (unchanged, sidecar) | 0% |
| IBC | ibc-go ~60K lines | ibc-rs ~30K lines (dependency) | 50% |
| Wasm host | wasmd ~15K Go + wasmvm FFI | cosmwasm-vm native + wasm_host 2K | 87% |
| Custom Go | 13,400 lines | 0 | 100% |
| **Total custom code** | **13,400 Go** | **~18,000 Rust** | — |
| **Total dependencies** | **~355K Go** | **~130K Rust** + CometBFT | 63% |

### What "Zero Go" Actually Means

Honestly: **zero custom Go code, but CometBFT remains a Go sidecar process.**

```
Zero custom Go:          ✅ No Go code written or maintained by Bostrom team
Zero Go dependencies:    ❌ CometBFT is still Go
Zero Go in main binary:  ✅ cyber-node is pure Rust, single cargo build
Single process:          ❌ Two processes (cyber-node + cometbft)
```

To achieve true single-process:
- **Option A**: Embed CometBFT via CGO (defeats the purpose)
- **Option B**: Wait for a Rust BFT consensus engine to mature
- **Option C**: Phase 4 (CyberOS) — implement custom consensus in Rs

**Recommendation**: Accept the two-process model. CometBFT is battle-tested, unmodified, zero maintenance. The two-process overhead is negligible (~0.1ms per ABCI call over loopback TCP).

---

## 6. Revised Phase 3 Timeline

### Updated: 16 weeks (not 12)

| Week | Task | Deliverable | Reference |
|---|---|---|---|
| 1–2 | ABCI server + CometBFT integration | tower-abci Service, block processing loop | Namada shell, Penumbra pd |
| 3–4 | Store layer (JMT + RocksDB + multi-store) | State persistence, ICS-23 proofs | Penumbra jmt, Nomic Merk |
| 5–6 | Auth module (accounts, signatures, sequences) | Transaction validation | Namada tx verification |
| 7–8 | Bank module (balances, transfers, TokenFactory) | Token operations | Orga coins |
| 9–10 | Staking module (validators, delegations, rewards) | Validator set management | Orga staking |
| 11–12 | Wasm host (cosmwasm-vm integration, message dispatch) | Contract execution | cosmwasm-vm API |
| 13–14 | IBC integration (ibc-rs contexts, proof generation) | Cross-chain communication | Namada IBC integration |
| 15 | Rank engine + graph store native integration | GPU compute, cyberlinks | Phase 2 rank-engine |
| 16 | State migration testing, IAVL → JMT migration | Testnet rehearsal, rollback plan | — |

**+4 weeks buffer** for:
- Cross-vendor testing (different CometBFT versions)
- IBC proof compatibility verification with counterparty chains
- Validator coordination and mainnet governance proposal

**Team**: 3–4 Rust developers with blockchain experience. Ideal: someone from Namada/Penumbra/Nomic ecosystem.

---

## 7. Risk Matrix (Updated)

| Risk | Severity | Likelihood | Mitigation |
|---|---|---|---|
| JMT ICS-23 proof incompatibility with IBC relayers | Critical | Medium | Test with Hermes relayer early; Penumbra has solved this |
| Staking module bugs (slashing, rewards) | Critical | Medium | Port Orga's O(1) staking; extensive property testing |
| cosmwasm-vm API instability | High | Low | Pin version; API is stable since 2.0 |
| CometBFT ABCI version mismatch | High | Low | Use matching tendermint-rs + CometBFT versions |
| State migration data loss (IAVL → JMT) | Critical | Low | Multiple testnet rehearsals; export/import verification |
| Performance regression (Rust host vs Go SDK) | Medium | Low | Rust is typically faster; benchmark before mainnet |
| Missing SDK features (edge cases in bank/staking) | Medium | Medium | Test against existing go-cyber behavior; property tests |
| ICS-27 Interchain Accounts not in ibc-rs | Low | High | Skip for Phase 3; implement later or wait for ibc-rs |

---

## 8. Build System

```toml
# Cargo.toml (workspace)
[workspace]
members = [
    "cyber-node",       # Binary: tower-abci server
    "cyber-sdk",        # Library: app framework modules
    "cyber-ibc",        # Library: ibc-rs integration
    "rank-engine",      # Library: wgpu GPU compute
    "cyber-interfaces", # Library: CyberMsg/CyberQuery types (shared with contracts)
]

[workspace.dependencies]
tower-abci = "0.16"
tendermint = "0.38"
tendermint-proto = "0.38"
ibc = "0.54"
ibc-proto = "0.48"
cosmwasm-vm = "2.2"
cosmwasm-std = "2.2"
jmt = "0.10"
wgpu = "24"
rocksdb = "0.22"
prost = "0.13"
tonic = "0.12"
tokio = { version = "1", features = ["full"] }
k256 = "0.13"
ed25519-dalek = "2.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```bash
# Build everything
cargo build --release

# Run
./target/release/cyber-node --home /data/cyber &
cometbft start --home /data/cometbft --proxy_app tcp://127.0.0.1:26658
```

---

## 9. Honest Assessment

### What Phase 3 Delivers
- All Bostrom-specific logic in Rust (graph, rank, bandwidth, contracts)
- All SDK modules in Rust (auth, bank, staking, wasm host)
- All IBC in Rust (ibc-rs)
- Single `cargo build` for the application binary
- No custom Go code anywhere
- No CGO, no wasmvm FFI

### What Phase 3 Does NOT Deliver
- CometBFT remains a Go process (all serious Rust Cosmos chains accept this)
- Store migration (IAVL → JMT) requires IBC client updates on counterparty chains
- Some SDK edge cases may be missing vs full cosmos-sdk
- ICS-27 Interchain Accounts not available initially

### The Uncomfortable Truth About "Zero Go"
Every production Rust chain in the Cosmos ecosystem runs CometBFT as a Go sidecar. Penumbra. Namada. Nomic. There is no Rust BFT consensus engine ready for production. Claiming "zero Go" while shipping CometBFT alongside is technically misleading — but it is the industry standard and the right engineering choice.

The goal is not "zero Go binaries on disk" but rather:
- Zero Go code maintained by the Bostrom team
- Zero Go compilation in the build pipeline
- Zero Go dependencies in the application
- Rust ownership of all state, logic, and upgrades

CometBFT is infrastructure — like running Linux under your Rust binary. You don't rewrite the kernel.
