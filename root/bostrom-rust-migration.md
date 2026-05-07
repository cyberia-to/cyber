---
tags: cyber, bostrom, migration, rust, research
icon: "\U0001F980"
alias: go-cyber-to-rust-migration, go-to-rust-host-migration, cyber-os-unified-blueprint
---

# Bostrom → Rust: Complete Migration Path

## 1. current architecture

go-cyber v7 runs ~13,400 lines of custom logic on top of ~695,000 lines of Go infrastructure. The ratio is 1:52.

```
go-cyber v7 binary (single Go process)
├── Cosmos SDK v0.47.12      Go     ~500,000 lines   Module system, routing, state
├── CometBFT v0.37.8         Go     ~100,000 lines   BFT consensus, P2P, mempool
├── ibc-go v7.6.0            Go      ~80,000 lines   Inter-Blockchain Communication
├── wasmd v0.45.0            Go+Rust  ~15,000 lines   CosmWasm host (x/wasm)
├── x/graph                  Go        ~1,800 lines   Cyberlink storage (CID → CID)
├── x/rank                   Go+CUDA   ~3,500 lines   Token-weighted PageRank on GPU
├── x/bandwidth              Go        ~1,200 lines   Rate limiting via VOLT tokens
├── x/resources              Go        ~1,000 lines   Investmint (HYDROGEN → VOLT/AMPERE)
├── x/dmn                    Go          ~700 lines   Autonomous programs (thoughts)
├── x/grid                   Go          ~800 lines   Energy routing/delegation
├── x/liquidity              Go        ~2,500 lines   Interchain AMM with MEV protection
└── CUDA kernels             CUDA C      ~470 lines   PageRank matrix operations
```

Four constraints make this architecture untenable long-term:

CUDA vendor lock — Only NVIDIA GPUs can validate. AMD, Intel, Apple Silicon excluded.

CGO FFI overhead — The CUDA rank module uses CGO. Every contract call crosses Go→Rust→Go boundaries via wasmd/wasmvm FFI. Breaks cross-compilation, prevents static linking.

Go GC pauses — Non-deterministic garbage collection pauses affect block timing. Tolerable today, bottleneck at scale.

Upgrade friction — Every [[Cosmos SDK]] upgrade requires rebasing 695K lines of Go. Custom modules tightly coupled to SDK internals.

Goal: a single `cargo build` producing a complete validator binary. Zero custom Go. Any GPU. Foundation for [[CyberOS]].

---

## 2. strategy

Six phases, each producing a working chain. No hard forks — all transitions via standard Cosmos governance proposals. Each phase is independently valuable even if later phases are delayed.

| Phase | Name | Duration | Deliverable |
|-------|------|----------|-------------|
| 0 | Interface Definition | 4 weeks | `cyber-interfaces` crate — zero implementation, all types |
| 1 | CosmWasm Migration | 15 weeks | All custom modules → CosmWasm contracts on wasmd |
| 2 | wgpu Rank Engine | 8 weeks (parallel) | CUDA → [[wgpu]], any GPU vendor |
| 3 | Rust Host | 16 weeks + 4 buffer | Replace Go SDK with minimal Rust framework |
| 4 | Hardening | 8 weeks | Audit, optimize, document |
| 5 | [[Rs]] + [[CyberOS]] | Ongoing | Domain-specific language, sovereign OS |

---

## 3. interface layer

The key architectural insight: define ALL interactions as [[CosmWasm]] messages and queries. The backend (Go or Rust) becomes a swappable implementation detail. Contracts written in Phase 1 run unchanged through Phase 3 and into [[CyberOS]].

```rust
// cyber-interfaces/src/lib.rs — trait crate, imported by all contracts
// Zero implementation. Only types.

#[cw_serde]
pub enum CyberMsg {
    Cyberlink { particle_from: String, particle_to: String },
    CyberlinkBatch { links: Vec<Link> },
    Investmint { amount: Coin, resource: String, length: u64 },
    CreateRoute { destination: String, alias: String },
    EditRoute { destination: String, value: Coin },
    DeleteRoute { destination: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum CyberQuery {
    #[returns(RankResponse)]
    ParticleRank { particle: String },
    #[returns(BandwidthResponse)]
    BandwidthLoad { address: String },
    #[returns(SearchResponse)]
    Search { particle: String, page: Option<u32> },
    #[returns(LinksResponse)]
    Backlinks { particle: String, page: Option<u32> },
}
```

Interface stability: same message, same contract binary, three different backends.

```
Phase 1:  Contract → CyberMsg::Cyberlink → wasmd dispatch → Go x/graph keeper
Phase 3:  Contract → CyberMsg::Cyberlink → Rust dispatch  → Rust graph store
Phase 5:  Contract → CyberMsg::Cyberlink → Rs cell call   → KnowledgeGraph cell
```

---

## 4. Phase 0+1: CosmWasm migration

### 4.1 CosmWasm boundary analysis

[[CosmWasm]] contracts access full [[Cosmos SDK]] functionality via messages and queries, including Stargate escape hatch for any protobuf message. 100% of SDK functionality is reachable.

What CosmWasm cannot do:

| Capability | Required By | Workaround |
|---|---|---|
| BeginBlocker / EndBlocker | x/rank, x/dmn | Neutron x/cron module → sudo calls |
| AnteHandler modification | x/bandwidth | Thin Go wrapper (~100 lines) calls CW query |
| GPU access | x/rank | Native plugin or wgpu rank-engine |
| Native token minting | x/resources | TokenFactory module (Osmosis pattern) |
| Self-execution | x/dmn auto-trigger | External cron → sudo entry point |

### 4.2 module-by-module plan

#### x/graph → cw-graph (~1,200 lines Rust)

Pure CRUD operations. Storage: `Map<(String, String), LinkMeta>` for [[cyberlinks]], `Map<String, Vec<String>>` for adjacency lists. No Go wrapper needed.

```rust
#[cw_serde]
pub enum ExecuteMsg {
    Cyberlink { links: Vec<Cyberlink> },
}

#[cw_serde]
pub enum QueryMsg {
    GraphStats {},
    ParticleLinks { cid: String, pagination: Option<PageRequest> },
    InLinks { cid: String, pagination: Option<PageRequest> },
    OutLinks { cid: String, pagination: Option<PageRequest> },
}
```

#### x/rank → rank-engine (native Rust) + cw-rank-verifier (~800 lines CW)

Rank cannot live in a contract — GPU compute is unavailable inside Wasm. Split into two components:

```
rank-engine (native Rust, runs every block via sudo)
    ├── Reads graph state from cw-graph storage
    ├── Computes PageRank on GPU (wgpu)
    ├── Generates rank Merkle tree
    └── Pushes rank root + top-K ranks into cw-rank-verifier via sudo

cw-rank-verifier (CosmWasm contract)
    ├── Stores current rank Merkle root + top-K values
    ├── Verifies Merkle proofs for individual rank lookups
    └── Exposes query interface: ParticleRank { particle } → rank
```

#### x/bandwidth → cw-bandwidth (~800 lines CW) + Go AnteHandler wrapper

Rate limiting must happen BEFORE transaction execution (in the AnteHandler). Thin Go wrapper in Phase 1 (~100 lines) queries the contract for sender's remaining bandwidth, deducts via contract execute. Phase 3 replaces this with a Rust AnteHandler calling the same contract.

#### x/resources → cw-resources (~600 lines CW)

Investmint: lock HYDROGEN for a period, receive VOLT or AMPERE. Pure contract using TokenFactory for native token minting: `factory/{contract_address}/volt` and `factory/{contract_address}/ampere`.

#### x/dmn → cw-dmn (~500 lines CW)

Autonomous programs ("thoughts") — scheduled contract executions. Requires x/cron (Neutron) calling `sudo { "execute_thoughts": { ... } }` every block.

#### x/grid → cw-grid (~500 lines CW)

Energy routing — delegation of bandwidth/resources. Pure contract, no hooks.

#### x/liquidity → cw-liquidity (fork)

Use existing production CosmWasm AMM: Astroport, TerraSwap, or Osmosis contracts. Fork, configure denoms, deploy.

### 4.3 Phase 1 target architecture

```
cw-cyber v1
├── Go host (~900 lines custom)
│   ├── wasmd (vanilla fork)
│   ├── x/tokenfactory          ← Osmosis, plug-and-play
│   ├── x/cron                  ← Neutron, plug-and-play
│   ├── rank native module      ← Go+CUDA, pushes to CW via sudo (~300 lines)
│   └── bandwidth AnteHandler   ← thin wrapper, queries CW (~100 lines)
│
├── CosmWasm contracts (all Rust)
│   ├── cw-graph                ~1,200 lines
│   ├── cw-rank-verifier          ~800 lines
│   ├── cw-bandwidth              ~800 lines
│   ├── cw-resources              ~600 lines
│   ├── cw-dmn                    ~500 lines
│   ├── cw-grid                   ~500 lines
│   └── cw-liquidity           ~0 (fork)
│
└── Shared Rust libraries
    ├── cyber-interfaces       trait crate, message definitions
    └── cyber-merkle           Merkle tree for rank proofs
```

Custom Go reduced from ~13,400 to ~900 lines. All business logic in Rust.

### 4.4 state migration

Single governance upgrade transaction at a coordinated block height:

```go
func CreateUpgradeHandler(...) upgradetypes.UpgradeHandler {
    return func(ctx sdk.Context, plan upgradetypes.Plan, fromVM module.VersionMap) (...) {
        // 1. Store all contract code
        graphCodeID := storeCode(ctx, wasmKeeper, "cw_graph.wasm")
        // ... all contracts

        // 2. Export existing Go state
        graphState := graphKeeper.ExportGenesis(ctx)
        // ...

        // 3. Instantiate contracts with migrated state
        graphAddr := instantiate(ctx, wasmKeeper, graphCodeID, GraphInstantiateMsg{
            Links: convertLinks(graphState.Links),
        })

        // 4. Register contract addresses in app params
        app.SetContractAddress("graph", graphAddr)

        // 5. Disable old Go modules
        return fromVM, nil
    }
}
```

Rollback: revert governance proposal, validators switch back to old binary.

### 4.5 Phase 1 timeline (15 weeks)

| Week | Task | Deliverable |
|------|------|-------------|
| 1–2 | Fork wasmd, integrate TokenFactory + cron | Build system, localbostrom |
| 3–4 | cw-graph: cyberlinks storage + queries | Contract + tests |
| 5 | cw-grid: energy routing | Contract + tests |
| 6–7 | cw-resources: investmint + TokenFactory | Contract + tests |
| 8–9 | cw-bandwidth: accounting + Go AnteHandler wrapper | Contract + thin Go |
| 10 | cw-dmn: thoughts + cron integration | Contract + tests |
| 11 | cw-rank-verifier + Go rank sudo bridge | Contract + Go bridge |
| 12 | cw-liquidity: fork AMM | Deploy config |
| 13 | Genesis migration script | Export → import test |
| 14 | Integration testing on testnet | Full system test |
| 15 | Validator coordination + mainnet governance | Mainnet upgrade |

Team: 2–3 Rust developers, 1 Go developer (part-time for wrappers)

---

## 5. Phase 2: wgpu rank engine

Replace NVIDIA-only CUDA kernels with cross-vendor [[wgpu]] compute. WebGPU standard guarantees identical integer arithmetic on all compliant GPUs.

```rust
pub struct RankEngine {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    adjacency_buf: wgpu::Buffer,   // CSR format graph
    rank_buf: wgpu::Buffer,        // Current rank vector
    new_rank_buf: wgpu::Buffer,    // Next iteration output
}
```

### 5.1 WGSL compute shader (deterministic PageRank)

```wgsl
// ALL arithmetic is integer-only. No floats. No vendor-specific rounding.

struct GraphData {
    num_nodes: u32,
    damping_factor: u32,   // Fixed-point: 850000 = 0.85
    scale: u32,            // 1000000 = 1.0
}

@group(0) @binding(0) var<storage, read> graph: GraphData;
@group(0) @binding(1) var<storage, read> row_ptr: array<u32>;
@group(0) @binding(2) var<storage, read> col_idx: array<u32>;
@group(0) @binding(3) var<storage, read> weights: array<u32>;
@group(0) @binding(4) var<storage, read> current_rank: array<u64>;
@group(0) @binding(5) var<storage, read_write> next_rank: array<u64>;

@compute @workgroup_size(256)
fn pagerank_step(@builtin(global_invocation_id) gid: vec3<u32>) {
    let node = gid.x;
    if (node >= graph.num_nodes) { return; }

    var incoming_rank: u64 = 0u;
    let start = row_ptr[node];
    let end = row_ptr[node + 1u];

    for (var i = start; i < end; i = i + 1u) {
        let src = col_idx[i];
        let weight = u64(weights[i]);
        let out_degree = u64(row_ptr[src + 1u] - row_ptr[src]);
        if (out_degree > 0u) {
            incoming_rank += (current_rank[src] * weight) / (out_degree * u64(graph.scale));
        }
    }

    let damping = u64(graph.damping_factor);
    let scale = u64(graph.scale);
    let teleport = (scale - damping) * scale / u64(graph.num_nodes);
    next_rank[node] = teleport + (damping * incoming_rank) / scale;
}
```

Determinism guarantee: integer-only arithmetic (u32/u64). WGSL spec defines integer operations identically across all compliant implementations. Adjacency list sorted for deterministic reduction order.

Hardware support: [[wgpu]] abstracts across Vulkan (Linux/Windows/Android), Metal (macOS/iOS), DX12 (Windows), OpenGL ES (fallback). Works on NVIDIA, AMD, Intel, Apple Silicon, Qualcomm Adreno. Headless operation — no display server required.

### 5.2 Phase 2 timeline (8 weeks, parallel with Phase 1)

| Week | Task | Deliverable |
|------|------|-------------|
| 1–2 | wgpu setup, headless compute pipeline | GPU init + basic dispatch |
| 3–4 | WGSL PageRank shader, fixed-point arithmetic | Compute shader + tests |
| 5 | CSR graph management, GPU upload/readback | Graph data structures |
| 6 | Merkle tree integration | Rank proofs |
| 7 | Determinism testing across GPU vendors | Cross-vendor test suite |
| 8 | Integration with Go host (Phase 1) or Rust host (Phase 3) | Bridge |

Team: 1 GPU/Rust developer

---

## 6. Phase 3: Rust host

The hardest phase. Four Go dependencies must be replaced with Rust.

```
go-cyber binary (single Go process)
├── cosmos-sdk v0.47.x    ~180K lines Go    App framework
├── CometBFT v0.37.x      ~100K lines Go    BFT consensus engine
├── ibc-go v7.x            ~60K lines Go     IBC protocol
└── wasmd v0.45.x          ~15K lines Go     CosmWasm host module
                           ─────────────
                           ~355K lines Go
```

### 6.1 CometBFT — stays as Go sidecar

There is no production Rust implementation of Tendermint/CometBFT consensus. Every Rust chain in the Cosmos ecosystem — Penumbra, Namada, Nomic — runs [[CometBFT]] as a separate Go process communicating via ABCI over TCP.

```
┌─────────────────────┐     TCP/26658      ┌──────────────────────┐
│  CometBFT (Go)      │ ◄────────────────► │  cyber-node (Rust)   │
│  - P2P networking    │     ABCI protocol  │  - State machine     │
│  - Mempool           │                    │  - CosmWasm VM       │
│  - Consensus rounds  │                    │  - Rank engine       │
│  - Block production  │                    │  - Bank, staking     │
└─────────────────────┘                     └──────────────────────┘
```

CometBFT is infrastructure — like Linux under the binary. Unmodified, audited, stock releases. Zero maintenance burden. The Rust app owns all state, all logic, all upgrades.

Implementation: tower-abci (Penumbra's async ABCI server on Tower + tokio):

```rust
use tower_abci::{Server, split};

#[tokio::main]
async fn main() {
    let app = CyberApp::new(config);
    let (consensus, mempool, info, snapshot) = split::service(app, 1);

    Server::builder()
        .consensus(consensus)
        .mempool(mempool)
        .info(info)
        .snapshot(snapshot)
        .finish().unwrap()
        .listen_tcp("127.0.0.1:26658").await.unwrap();
}
```

Target: CometBFT v0.38+ (ABCI++) — enables `PrepareProposal` for Rust-controlled block construction.

### 6.2 cosmos-sdk → minimal Rust framework

No existing Rust framework fits directly:

| Project | Status | Why Not |
|---------|--------|---------|
| Gears (Rumos) | Early, 31★ | No IBC, no CosmWasm, incomplete |
| Orga (Turbofish/Nomic) | Production | No CosmWasm, custom IBC, different paradigm |
| Penumbra | Production | UTXO model, no CosmWasm |
| Namada | Production | Deep privacy coupling, not a framework |

Instead: build a minimal Rust application framework that only implements what [[Bostrom]] actually uses. After Phase 1 moves business logic to contracts, the native modules shrink dramatically.

```
cyber-sdk (~11,000 lines Rust vs ~500,000 lines Go cosmos-sdk)
├── BaseApp (~2,000 lines)
│   ├── ABCI routing (tower-abci Service impl)
│   ├── Transaction decoding (prost)
│   ├── Gas metering
│   └── AnteHandler chain (signatures, bandwidth, fees)
│
├── x/auth (~1,500 lines)
│   ├── Account storage, signature verification (k256, ed25519-dalek)
│   ├── Sequence numbers (replay protection)
│   └── Module accounts
│
├── x/bank (~1,000 lines)
│   ├── Balance storage (address × denom → amount)
│   ├── Transfer logic, supply tracking
│   └── TokenFactory integration
│
├── x/staking (~2,500 lines)
│   ├── Validator set, delegations, slashing
│   ├── Reward distribution
│   └── Validator power updates → CometBFT via EndBlock
│
├── x/wasm-host (~2,000 lines)
│   ├── Contract lifecycle (store, instantiate, execute, migrate, sudo)
│   ├── Gas forwarding, message dispatch, query routing
│   └── CyberMsg/CyberQuery custom handler
│
├── x/upgrade (~500 lines)
│
├── Store (~1,500 lines)
│   ├── Jellyfish Merkle tree (JMT) + RocksDB
│   ├── Multi-store (one subtree per module)
│   ├── ICS-23 proof generation (for IBC compatibility)
│   └── State snapshots
│
└── Why 45x smaller?
    ├── No EVM, no Ethermint
    ├── Governance → CosmWasm contract (DAO-DAO)
    ├── Liquidity → CosmWasm contract
    ├── No legacy amino encoding
    ├── No REST API legacy (gRPC only via tonic)
    └── Most business logic already in contracts from Phase 1
```

Store decision: Jellyfish Merkle tree (JMT). Wider ecosystem support (Penumbra, Sovereign Labs, Astria). ICS-23 proof adapter exists. Store is behind a trait — can benchmark Merk later and swap.

Per-module reference implementations:

| Module | Reference | Why |
|---|---|---|
| BaseApp / ABCI routing | Namada `namada_node::shell` | Full ABCI++ impl, Rust, production |
| Auth / signatures | Penumbra `penumbra-custody` | Rust crypto, account handling |
| Bank / balances | Orga `orga::coins` | O(1) operations, production (Nomic) |
| Staking | Orga `orga::plugins::staking` | O(1) delegation/rewards, production |
| Store | JMT (Penumbra) | ICS-23 compatible, widely used |
| Wasm host | cosmwasm-vm directly | Already Rust, just needs host glue |

### 6.3 ibc-go → ibc-rs

The most mature Rust replacement. Maintained by Informal Systems (same team as Hermes relayer). The host implements two traits:

```rust
/// Read-only state access for IBC validation
pub trait ValidationContext {
    fn host_height(&self) -> Height;
    fn host_timestamp(&self) -> Timestamp;
    fn client_state(&self, client_id: &ClientId) -> Result<Box<dyn ClientState>>;
    fn consensus_state(&self, path: &ClientConsensusStatePath) -> Result<Box<dyn ConsensusState>>;
    fn connection_end(&self, conn_id: &ConnectionId) -> Result<ConnectionEnd>;
    fn channel_end(&self, port_id: &PortId, channel_id: &ChannelId) -> Result<ChannelEnd>;
    // ... ~20 methods
}

/// Write access for IBC execution
pub trait ExecutionContext: ValidationContext {
    fn store_client_state(&mut self, path: ClientStatePath, state: Box<dyn ClientState>);
    fn store_connection(&mut self, path: ConnectionPath, conn: ConnectionEnd);
    fn store_channel(&mut self, path: ChannelPath, channel: ChannelEnd);
    fn store_packet_commitment(&mut self, path: CommitmentPath, commitment: PacketCommitment);
    // ... write methods
}
```

IBC proof compatibility: switching from IAVL to JMT changes the proof format. Solution: JMT proofs wrapped in ICS-23 `CommitmentProof` format (Penumbra's approach). The root hash is correct, the proof verifies — the internal tree structure does not matter to IBC.

Penumbra's divergence (lesson): Penumbra forked ibc-rs into `ibc-types` due to async state model mismatch. Bostrom's synchronous state model fits ibc-rs's `ValidationContext`/`ExecutionContext` split (ADR-005) well. Use ibc-rs directly.

ICS-27 Interchain Accounts is not in ibc-rs — skip for Phase 3, implement later.

### 6.4 wasmd → cosmwasm-vm native

The CosmWasm VM is already Rust. Currently Go calls it via FFI (wasmvm). In Phase 3, direct embedding:

```rust
use cosmwasm_vm::{Cache, Instance, Backend};

impl WasmHost {
    fn execute_contract(&mut self, sender: &Addr, contract: &Addr,
                        msg: &[u8], funds: &[Coin]) -> Result<Response> {
        let code = self.code_store.get(contract)?;
        let env = self.build_env(contract)?;
        let info = MessageInfo { sender, funds };

        // cosmwasm-vm natively, zero FFI
        let instance = self.cache.get_instance(&code.checksum, self.backend(contract))?;
        let response = instance.execute(&env, &info, msg)?;

        self.process_response(response)
    }

    fn process_response(&mut self, response: Response) -> Result<Vec<Event>> {
        for msg in response.messages {
            match msg.msg {
                CosmosMsg::Bank(BankMsg::Send { to, amount }) =>
                    self.bank.send(&to, &amount)?,
                CosmosMsg::Wasm(WasmMsg::Execute { contract, msg, funds }) =>
                    self.execute_contract(&contract, &msg, &funds)?,
                CosmosMsg::Custom(CyberMsg::Cyberlink { links }) => {
                    self.graph.create_links(&links)?;
                    self.rank_dirty = true;
                }
                // ... staking, IBC, etc.
            }
        }
        Ok(response.events)
    }
}
```

cosmwasm-vm requires the host to implement a `Backend` providing storage, API, and querier:

```rust
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

/// Host-provided querier routes to bank, staking, wasm, and custom modules
struct CyberQuerier { bank, staking, wasm_host, custom }

impl Querier for CyberQuerier {
    fn query_raw(&self, request: &[u8], gas_limit: u64) -> BackendResult<...> {
        let query: QueryRequest<CyberQuery> = from_json(request)?;
        match query {
            QueryRequest::Bank(q) => self.bank.handle(q),
            QueryRequest::Staking(q) => self.staking.handle(q),
            QueryRequest::Wasm(q) => self.wasm_host.handle(q),
            QueryRequest::Custom(q) => self.custom.handle(q),
            _ => Err(BackendError::unknown("unsupported query")),
        }
    }
}
```

### 6.5 IAVL → JMT state migration

At the Phase 3 upgrade height:

```
1. HALT chain at governance-approved height
2. Export: iterate all IAVL leaves → KV dump (deterministic, ordered)
3. Import: insert all KV pairs into JMT
4. Compute new JMT root hash
5. Resume with Rust binary + CometBFT sidecar
6. IBC channels: governance proposal on counterparty chains to update light client
```

Risk: IBC proof format change. Counterparty chains must update their [[Bostrom]] light client. Mitigation: extensive testnet rehearsal, coordinate with relayer operators, potentially maintain dual proof support during transition.

### 6.6 Phase 3 complete architecture

```
cyber-node (single Rust binary, ~18,000 lines)
├── tower-abci          ABCI server (Penumbra)
├── tendermint-rs        Types, proto (Informal Systems)
├── cyber-sdk            Minimal app framework
│   ├── auth             Accounts, signatures
│   ├── bank             Balances, TokenFactory
│   ├── staking          Validators, delegations
│   ├── wasm-host        CosmWasm host (native)
│   └── store            JMT + RocksDB
├── ibc-rs               Full IBC protocol
├── cosmwasm-vm          Wasm contract VM (native)
├── rank-engine          wgpu GPU compute
├── graph-store          Native cyberlink storage
└── bandwidth            Native rate limiting

+ CometBFT v0.38 (stock Go binary, separate process)
+ CosmWasm contracts (unchanged from Phase 1)
```

### 6.7 Phase 3 timeline (16 weeks + 4 buffer)

| Week | Task | Deliverable | Reference |
|------|------|-------------|-----------|
| 1–2 | ABCI server + CometBFT integration | tower-abci setup | Namada shell, Penumbra pd |
| 3–4 | Store layer (JMT + RocksDB + multi-store) | State persistence, ICS-23 proofs | Penumbra jmt |
| 5–6 | Auth module (accounts, signatures, sequences) | Transaction validation | Namada tx verification |
| 7–8 | Bank module (balances, transfers, TokenFactory) | Token operations | Orga coins |
| 9–10 | Staking module (validators, delegations, rewards) | Validator set management | Orga staking |
| 11–12 | Wasm host (cosmwasm-vm, message dispatch) | Contract execution | cosmwasm-vm API |
| 13–14 | IBC integration (ibc-rs contexts, proofs) | Cross-chain communication | Namada IBC |
| 15 | Rank engine + graph store integration | GPU compute, [[cyberlinks]] | Phase 2 rank-engine |
| 16 | State migration testing, IAVL → JMT | Testnet rehearsal | — |
| +4 | Buffer: IBC coordination, cross-vendor testing | Mainnet readiness | — |

Team: 3–4 Rust developers. Ideal: experience with Namada, Penumbra, or Nomic codebases.

---

## 7. Phase 4: hardening

Performance optimization: profile with `perf` and `flamegraph`. Optimize hot paths: tx deserialization, state reads, [[CosmWasm]] VM calls. Tune RocksDB for blockchain workload. Target: >1000 TPS for cyberlink operations, <2GB RAM for full node.

Security audit: external audit of all native modules (bank, staking, auth, IBC integration). Formal verification of fixed-point arithmetic in rank engine. Fuzz testing of all message handlers. Property-based state machine testing for invariants.

Ecosystem tooling: Rust CLI, Docker images, deployment scripts, Prometheus metrics + Grafana dashboards, block explorer compatibility.

Duration: 8 weeks. Can overlap with Phase 3 final testing.

---

## 8. Phase 5: Rs + CyberOS

[[Rs]] is a strict superset of Rust — all valid Rust is valid Rs. Four new capabilities for deterministic, real-time, bare-metal blockchain systems: bounded async with compile-time deadlines, `#[deterministic]` functions that reject non-deterministic operations, typed MMIO registers without unsafe, and `cell!` declarations for hot-swappable OS modules with resource budgets.

[[CyberOS]] is a purpose-built operating system for running [[Bostrom]] validators. Cells instead of processes, content-addressed storage instead of filesystems, cryptographic agents instead of users, three purpose-built network protocols instead of TCP/IP. Neural drivers generated by models against stable trait contracts.

see [[rs-language-spec]] for the full language specification, [[cyber-os-architecture]] for the complete OS architecture

---

## 9. consolidated plan

### 9.1 full timeline

```
Month 1:      Phase 0 (interfaces) + Phase 2 starts (wgpu, parallel)
Month 1–4:    Phase 1 (CosmWasm migration)
Month 3:      Phase 2 completes (wgpu rank engine)
Month 5–9:    Phase 3 (Rust host)
Month 9–10:   Phase 4 (hardening)
Month 10–12:  Phase 3 mainnet, Phase 4 completes
Month 12+:    Phase 5 (Rs language + CyberOS, ongoing)
```

### 9.2 governance upgrade sequence

| Upgrade | Binary Change | State Change | Rollback |
|---------|---------------|--------------|----------|
| Phase 1 | wasmd binary with CW contracts | Go state → contract state | Revert to old binary |
| Phase 2 | wgpu rank engine replaces CUDA | None (ranks recomputed) | Revert binary (CUDA fallback) |
| Phase 3 | Rust binary + CometBFT sidecar | IAVL → JMT migration | Reverse migration (complex) |
| Phase 4 | Optimized Rust binary | None | Revert to Phase 3 binary |
| Phase 5 | CyberOS binary | Module → cell migration | Revert to Phase 4 binary |

### 9.3 resource estimates

| Phase | Team | Duration | Custom Lines |
|-------|------|----------|--------------|
| 0 | 1 Rust dev | 4 weeks | ~500 (interfaces) |
| 1 | 2–3 Rust + 1 Go | 15 weeks | ~6,700 (contracts + Go wrappers) |
| 2 | 1 GPU/Rust dev | 8 weeks | ~2,000 (rank engine) |
| 3 | 3–4 Rust devs | 16+4 weeks | ~18,000 (Rust host) |
| 4 | 2–3 Rust devs | 8 weeks | ~2,000 (tooling, tests) |
| 5 | 2–4 Rust devs | Ongoing | ~15,000+ (Rs compiler + CyberOS) |

### 9.4 hardware requirements

| Resource | Current (go-cyber) | After Phase 3 | After CyberOS |
|----------|-------------------|---------------|----------------|
| GPU | NVIDIA only (CUDA) | Any vendor (wgpu) | Any vendor |
| RAM | ~8GB | ~4GB (no GC overhead) | ~2GB (no OS overhead) |
| CPU | 4+ cores | 2+ cores | 2+ cores |
| Disk | ~500GB | ~300GB (compact store) | ~200GB |
| Binary | ~100MB (Go) | ~30MB (Rust) | ~15MB (no OS) |

### 9.5 existing Rust ecosystem

~70% of the required infrastructure already exists in production:

| Component | Go (current) | Rust alternative | Status |
|---|---|---|---|
| Consensus | CometBFT | tendermint-rs (Informal Systems) | Production — Penumbra, Namada |
| ABCI interface | Go ABCI server | tower-abci | Production — Penumbra |
| IBC | ibc-go | ibc-rs (Hermes) | Production — Namada |
| State store | IAVL tree | jellyfish-merkle (Aptos origin) | Production — Penumbra, Sovereign |
| Governance | x/gov | DAO-DAO (CosmWasm) | Production |
| Protobuf | protoc-gen-go | prost | Production |
| gRPC | grpc-go | tonic | Production |
| Cryptography | Go crypto | ring, ed25519-dalek, k256 | Production |
| CosmWasm VM | wasmvm (Go↔Rust FFI) | cosmwasm-vm (native Rust) | Production — already Rust |
| GPU compute | CUDA | wgpu + WGSL | Production — Firefox, Bevy |

### 9.6 risk matrix

| Risk | Phase | Severity | Likelihood | Mitigation |
|------|-------|----------|------------|------------|
| CosmWasm gas limits hit during migration | 1 | Medium | Medium | Pre-test all contract ops, optimize storage layout |
| wgpu determinism failure on exotic GPUs | 2 | High | Low | Integer-only arithmetic, cross-vendor test suite |
| JMT ICS-23 proof incompatibility | 3 | Critical | Medium | Test with Hermes relayer early; Penumbra solved this |
| Staking module bugs (slashing, rewards) | 3 | Critical | Medium | Port Orga's O(1) staking; property testing |
| IAVL→JMT state migration data loss | 3 | Critical | Low | Multiple testnet rehearsals, verify every KV pair |
| Phase 3 rollback complexity | 3 | High | Low | Extensive testnet, parallel chain option |
| Rs compiler acceptance by Rust community | 5 | Medium | Medium | Start as proc macros, prove value before compiler patch |
| CyberOS driver coverage gaps | 5 | Medium | High | model generation covers common hardware; start with virtio |

### 9.7 success gates

| Phase | Gate (must pass before mainnet) |
|-------|-------------------------------|
| 0 | All interfaces compile, no implementation dependencies |
| 1 | All 7 modules pass behavioral equivalence tests vs Go modules |
| 2 | Rank output bit-identical across NVIDIA, AMD, Intel GPUs |
| 3 | Full IBC roundtrip works (send + receive + ack on testnet) |
| 4 | External audit clean, no critical findings |
| 5 | CyberOS boots on 3+ hardware platforms, passes all Phase 3 tests |

### 9.8 metrics

| Metric | go-cyber v7 | cw-cyber v1 | cw-cyber v3 |
|---|---|---|---|
| Custom Go lines | 13,400 | 900 | 0 |
| Custom Rust lines | 0 | 6,700 | 18,000 |
| CUDA lines | 500 | 500 → 0 | 0 |
| Go dependencies | ~680K lines | ~680K lines | 0 |
| GPU requirement | NVIDIA only | NVIDIA → any | Any |
| Build toolchain | Go + CUDA + CGO | Go + Rust | Rust only |
| Upgrade mechanism | Binary replace | Governance migrate | Governance migrate |
| Path to CyberOS | None | Direct | Native |

### 9.9 honest acknowledgments

"Zero Go" means: zero custom Go code maintained by [[Bostrom]] team. [[CometBFT]] remains a Go sidecar until Phase 5 ([[CyberOS]]) potentially replaces it with a Rust/Rs consensus engine. This is how Penumbra, Namada, and Nomic operate. It is the industry standard.

No production precedent for: pure Rust CosmWasm host (Phase 3), deterministic wgpu rank engine at scale (Phase 2), model-generated OS drivers in production (Phase 5). These are engineering firsts, not assembly of existing parts.

The 12-month estimate is optimistic. Phase 3 alone may take 6 months with a small team. [[CyberOS]] (Phase 5) is a multi-year effort. Each phase is independently valuable — the plan degrades gracefully if later phases are delayed.

---

## 10. reference architectures

### Penumbra (production Rust Cosmos chain)
```
CometBFT v0.37 (external Go binary)
    ↓ ABCI (tower-abci, async)
Penumbra pd (Rust binary)
    ├── tower-abci
    ├── penumbra-storage (async state, JMT)
    ├── ibc-types + penumbra-ibc (async IBC)
    └── Custom modules (shielded pool, DEX, staking, governance)
```

### Namada (production Rust Cosmos chain)
```
CometBFT v0.37.16 (external Go binary)
    ↓ ABCI (tower-abci)
Namada (Rust binary)
    ├── tower-abci
    ├── Custom state machine
    ├── ibc-rs (full IBC)
    └── PoS, governance, slashing
```

### Nomic (production Rust Cosmos chain)
```
CometBFT (external Go binary)
    ↓ ABCI (abci2, custom)
Nomic (Rust binary)
    ├── Orga framework (custom Rust SDK)
    ├── Merk (high-performance Merkle AVL, 2-20x faster than JMT)
    └── Custom IBC, Bitcoin bridge, PoS staking
```

---

## appendix: workspace dependencies

```toml
[workspace.dependencies]
tower-abci = "0.16"
tendermint = "0.38"
tendermint-proto = "0.38"
ibc = "0.54"
ibc-proto = "0.48"
cosmwasm-vm = "2.2"
cosmwasm-std = "2.2"
jmt = "0.10"
rocksdb = "0.22"
wgpu = "24"
tonic = "0.12"
tokio = { version = "1", features = ["full"] }
prost = "0.13"
k256 = "0.13"
ed25519-dalek = "2.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## appendix: ecosystem maturity

| Component | Crate | Maturity | Production Use | Risk |
|-----------|-------|----------|----------------|------|
| ABCI server | tower-abci | stable | Penumbra, Namada, Fendermint | Low |
| Tendermint types | tendermint-rs | stable | Penumbra, Namada, Nomic | Low |
| Merkle tree | jmt | stable | Penumbra, Sovereign Labs, Astria | Low |
| IBC protocol | ibc-rs | maturing | Namada, ibc-go test parity | Medium |
| CosmWasm VM | cosmwasm-vm | stable | All CW chains (via FFI today) | Low |
| GPU compute | wgpu | stable | Firefox, Bevy | Low |
| Rust Cosmos SDK | Gears | early | AZKR-chain (dev only) | High |
| Native CW host | — | none | No production example | High |

see [[cyber/whitepaper]] for protocol specification, [[cyb-system-architecture]] for browser architecture, [[rs-language-spec]] for Rs language, [[cyber-os-architecture]] for CyberOS