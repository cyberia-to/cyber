# Cyber: From Go to a Sovereign Rust Operating System

## Complete Technical Blueprint

**Version**: 2.0 · March 2026
**Scope**: go-cyber v7 → CosmWasm contracts → Rust host → Rs language → CyberOS
**Timeline**: 12–18 months of phased, governance-driven upgrades

---

## Part I — Where We Are

### 1.1 The Problem

go-cyber v7 runs ~13,400 lines of custom logic on top of ~695,000 lines of Go infrastructure. The ratio is 1:52. The chain depends on NVIDIA CUDA for GPU-accelerated PageRank, creating vendor lock that limits the validator set to NVIDIA GPUs only.

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

**CUDA vendor lock** — Only NVIDIA GPUs can validate. AMD, Intel, Apple Silicon excluded. Limits decentralization.

**CGO FFI overhead** — The CUDA rank module uses CGO. Every contract call crosses Go→Rust→Go boundaries via wasmd/wasmvm FFI. Breaks cross-compilation, prevents static linking.

**Go GC pauses** — Non-deterministic garbage collection pauses affect block timing. Tolerable today, bottleneck at scale.

**Upgrade friction** — Every Cosmos SDK upgrade requires rebasing 695K lines of Go. Custom modules tightly coupled to SDK internals.

### 1.2 The Goal

A single `cargo build` producing a complete validator binary. Zero custom Go. Any GPU. Foundation for a purpose-built operating system.

### 1.3 Strategy

Six phases, each producing a working chain. No hard forks — all transitions via standard Cosmos governance proposals. Each phase is independently valuable even if later phases are delayed.

| Phase | Name | Duration | Deliverable |
|-------|------|----------|-------------|
| 0 | Interface Definition | 4 weeks | `cyber-interfaces` crate — zero implementation, all types |
| 1 | CosmWasm Migration | 15 weeks | All custom modules → CosmWasm contracts on wasmd |
| 2 | wgpu Rank Engine | 8 weeks (parallel) | CUDA → wgpu, any GPU vendor |
| 3 | Rust Host | 16 weeks + 4 buffer | Replace Go SDK with minimal Rust framework |
| 4 | Hardening | 8 weeks | Audit, optimize, document |
| 5 | Rs + CyberOS | Ongoing | Domain-specific language, sovereign OS |

---

## Part II — Interface Layer

### 2.1 The Key Insight

Define ALL interactions as CosmWasm messages and queries. The backend (Go or Rust) becomes a swappable implementation detail. Contracts written in Phase 1 run unchanged through Phase 3 and into CyberOS.

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

#[cw_serde]
pub struct Link {
    pub particle_from: String,
    pub particle_to: String,
}

#[cw_serde]
pub struct RankResponse {
    pub rank: Uint128,  // Fixed-point, 18 decimal places
}
```

### 2.2 Interface Stability Contract

Contracts program against these interfaces. Phase 1 resolves them through Go SDK module wrappers. Phase 3 resolves them through native Rust modules. The contracts never change.

```
Phase 1:  Contract → CyberMsg::Cyberlink → wasmd dispatch → Go x/graph keeper
Phase 3:  Contract → CyberMsg::Cyberlink → Rust dispatch  → Rust graph store
Phase 5:  Contract → CyberMsg::Cyberlink → Rs cell call   → KnowledgeGraph cell
```

Same message. Same contract binary. Three completely different backends.

---

## Part III — Phase 0+1: CosmWasm Contract Migration

### 3.1 Module-by-Module Plan

Every custom Go module migrates to a CosmWasm contract. Where Go-level hooks are unavoidable (AnteHandler, sudo), thin Go wrappers remain until Phase 3 eliminates them.

#### x/graph → cw-graph (~800 lines Rust)

Storage: `Map<(String, String), LinkMeta>` for cyberlinks, `Map<String, Vec<String>>` for adjacency lists.

No Go wrapper needed — pure contract. Graph queries (`backlinks`, `search`) implemented as contract queries. The native rank engine reads the graph state directly from contract storage via sudo.

#### x/rank → rank-engine (Rust native) + cw-rank-verifier (~400 lines CW)

Rank cannot live in a contract — GPU compute is not available inside Wasm. Architecture:

```
rank-engine (native Rust, runs every block via sudo)
    ├── Reads graph state from cw-graph storage
    ├── Computes PageRank on GPU (wgpu)
    ├── Generates rank Merkle tree
    └── Pushes rank root + top-K ranks into cw-rank-verifier via sudo

cw-rank-verifier (CosmWasm contract)
    ├── Stores current rank Merkle root
    ├── Stores top-K rank values
    ├── Verifies Merkle proofs for individual rank lookups
    └── Exposes query interface: ParticleRank { particle } → rank
```

#### x/bandwidth → cw-bandwidth (~600 lines CW) + Go AnteHandler wrapper

Bandwidth accounting lives in a contract. But rate limiting must happen BEFORE transaction execution (in the AnteHandler). Thin Go wrapper in Phase 1:

```go
// Phase 1: ~50 lines Go wrapper
func (bw BandwidthAnteHandler) AnteHandle(ctx sdk.Context, tx sdk.Tx, ...) {
    // Query cw-bandwidth contract for sender's remaining bandwidth
    remaining := queryContract(ctx, bwContractAddr, BandwidthQuery{Load{sender}})
    if remaining < txCost {
        return ErrInsufficientBandwidth
    }
    // Deduct via contract execute
    executeContract(ctx, bwContractAddr, BandwidthMsg{Deduct{sender, txCost}})
}
```

Phase 3 replaces this with a Rust AnteHandler calling the same contract.

#### x/resources → cw-resources (~500 lines CW)

Investmint logic: lock HYDROGEN for a period, receive VOLT (bandwidth) and AMPERE (rank weight). Pure contract with no Go hooks. Uses TokenFactory to mint/burn resource tokens.

#### x/dmn → cw-dmn (~500 lines CW)

Autonomous programs ("thoughts") — contracts that execute on a cron schedule. Requires wasmd cron module integration (or `cw-croncat`). Each thought is a stored (contract_addr, msg, schedule) tuple.

#### x/grid → cw-grid (~400 lines CW)

Energy routing — delegation of bandwidth/resources to other addresses. Pure contract, no hooks needed.

#### x/liquidity → cw-liquidity (~1,200 lines CW)

AMM with MEV protection. Fork from `cw-dex` or `astroport` contracts. Heaviest single contract but well-understood domain.

### 3.2 Phase 1 Weekly Timeline

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

**Team**: 2–3 Rust developers, 1 Go developer (part-time for wrappers)

### 3.3 State Migration

Single governance upgrade transaction at a coordinated block height:

```go
func CreateUpgradeHandler(...) upgradetypes.UpgradeHandler {
    return func(ctx sdk.Context, plan upgradetypes.Plan, fromVM module.VersionMap) (...) {
        // 1. Store all contract code
        graphCodeID := storeCode(ctx, wasmKeeper, "cw_graph.wasm")
        rankCodeID  := storeCode(ctx, wasmKeeper, "cw_rank_verifier.wasm")
        // ... all contracts

        // 2. Export existing Go state
        graphState := graphKeeper.ExportGenesis(ctx)
        bwState    := bandwidthKeeper.ExportGenesis(ctx)
        // ...

        // 3. Instantiate contracts with migrated state
        graphAddr := instantiate(ctx, wasmKeeper, graphCodeID, GraphInstantiateMsg{
            Links: convertLinks(graphState.Links),
        })

        // 4. Register contract addresses in app params
        app.SetContractAddress("graph", graphAddr)
        // ...

        // 5. Disable old Go modules
        // Old x/graph, x/bandwidth etc. no longer process messages

        return fromVM, nil
    }
}
```

**Rollback**: Revert governance proposal, validators switch back to old binary. State is in Go modules until upgrade height.

---

## Part IV — Phase 2: wgpu Rank Engine

### 4.1 CUDA → wgpu

Replace NVIDIA-only CUDA kernels with cross-vendor wgpu compute. WebGPU standard guarantees identical integer arithmetic on all compliant GPUs.

```rust
// rank-engine/src/compute.rs
use wgpu;

pub struct RankEngine {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    // GPU buffers
    adjacency_buf: wgpu::Buffer,   // CSR format graph
    rank_buf: wgpu::Buffer,        // Current rank vector
    new_rank_buf: wgpu::Buffer,    // Next iteration output
}

impl RankEngine {
    pub fn compute_rank(&self, iterations: u32) -> Vec<u64> {
        for _ in 0..iterations {
            let mut encoder = self.device.create_command_encoder(&Default::default());
            {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                pass.set_pipeline(&self.pipeline);
                pass.set_bind_group(0, &self.bind_group, &[]);
                pass.dispatch_workgroups((self.num_nodes + 255) / 256, 1, 1);
            }
            self.queue.submit(std::iter::once(encoder.finish()));
            std::mem::swap(&mut self.rank_buf, &mut self.new_rank_buf);
        }
        // Readback results
        self.readback_ranks()
    }
}
```

### 4.2 WGSL Compute Shader (Deterministic PageRank)

```wgsl
// rank-engine/src/pagerank.wgsl
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
            incoming_rank = incoming_rank + (current_rank[src] * weight) / (out_degree * u64(graph.scale));
        }
    }

    let damping = u64(graph.damping_factor);
    let scale = u64(graph.scale);
    let teleport = (scale - damping) * scale / u64(graph.num_nodes);
    next_rank[node] = teleport + (damping * incoming_rank) / scale;
}
```

**Determinism guarantee**: Integer-only arithmetic (u32/u64). WGSL spec defines integer operations identically across all compliant implementations. Adjacency list sorted for deterministic reduction order. Cross-vendor testing suite validates bit-exact results across NVIDIA, AMD, Intel, Apple Silicon.

### 4.3 Timeline

| Week | Task | Deliverable |
|------|------|-------------|
| 1–2 | wgpu setup, headless compute pipeline | GPU init + basic dispatch |
| 3–4 | WGSL PageRank shader, fixed-point arithmetic | Compute shader + tests |
| 5 | CSR graph management, GPU upload/readback | Graph data structures |
| 6 | Merkle tree integration | Rank proofs |
| 7 | Determinism testing across GPU vendors | Cross-vendor test suite |
| 8 | Integration with Go host (Phase 1) or Rust host (Phase 3) | Bridge |

---

## Part V — Phase 3: Rust Host

This is the hardest phase. Four Go dependencies must be replaced with Rust.

### 5.1 CometBFT — Stays as Go Sidecar

There is no production Rust BFT consensus engine. Every Rust chain in the Cosmos ecosystem — Penumbra, Namada, Nomic — runs CometBFT as a separate Go process communicating via ABCI over TCP.

```
┌─────────────────────┐     TCP/26658      ┌──────────────────────┐
│  CometBFT (Go)      │ ◄────────────────► │  cyber-node (Rust)   │
│  - P2P networking    │     ABCI protocol  │  - State machine     │
│  - Mempool           │                    │  - CosmWasm VM       │
│  - Consensus rounds  │                    │  - Rank engine       │
│  - Block production  │                    │  - Bank, staking     │
└─────────────────────┘                     └──────────────────────┘
```

CometBFT is infrastructure — like Linux under the binary. Unmodified, audited, stock releases. Zero maintenance burden for Bostrom team. The Rust app owns all state, all logic, all upgrades.

**Implementation**: tower-abci (Penumbra's async ABCI server on Tower + tokio):

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

**Target**: CometBFT v0.38+ (ABCI++) — enables `PrepareProposal` for Rust-controlled block construction.

### 5.2 cosmos-sdk — Minimal Custom Rust Framework

No existing Rust framework fits directly:

| Project | Status | Why Not |
|---------|--------|---------|
| **Gears** (Rumos) | Early, 31★ | No IBC, no CosmWasm, incomplete |
| **Orga** (Turbofish/Nomic) | Production | No CosmWasm, custom IBC, different paradigm |
| **Penumbra** | Production | UTXO model, no CosmWasm |
| **Namada** | Production | Deep privacy coupling, not a framework |

Instead: build a **minimal Rust application framework** that only implements what Bostrom actually uses. After Phase 1 moves business logic to contracts, the native modules shrink dramatically.

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

**Store decision**: Jellyfish Merkle tree (JMT). Wider ecosystem support than Merk (Penumbra, Sovereign Labs, Astria all use JMT). ICS-23 proof adapter exists. Store is behind a trait — can benchmark Merk later and swap.

### 5.3 ibc-go → ibc-rs

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

**Critical issue**: Switching from IAVL to JMT changes the proof format. IBC relayers verify proofs from the counterparty chain. Solution: JMT proofs wrapped in ICS-23 `CommitmentProof` format (Penumbra's approach). The root hash is correct, the proof verifies — the internal tree structure doesn't matter to IBC.

### 5.4 wasmd → cosmwasm-vm Native

The CosmWasm VM is already Rust. Currently Go calls it via FFI (wasmvm). In Phase 3, direct embedding:

```rust
// Direct Rust — no Go, no FFI
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

        // Recursive message dispatch
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

### 5.5 IAVL → JMT State Migration

At the Phase 3 upgrade height:

```
1. HALT chain at governance-approved height
2. Export: iterate all IAVL leaves → KV dump (deterministic, ordered)
3. Import: insert all KV pairs into JMT
4. Compute new JMT root hash
5. Resume with Rust binary + CometBFT sidecar
6. IBC channels: governance proposal on counterparty chains to update light client
```

Risk: IBC proof format change. Counterparty chains must update their Bostrom light client. Mitigation: extensive testnet rehearsal, coordinate with relayer operators, potentially maintain dual proof support during transition.

### 5.6 Phase 3 Complete Architecture

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

### 5.7 Phase 3 Timeline

| Week | Task | Deliverable |
|------|------|-------------|
| 1–2 | ABCI server + CometBFT integration | tower-abci setup, block processing |
| 3–4 | Store layer (JMT + RocksDB + multi-store) | State persistence, ICS-23 proofs |
| 5–6 | Auth module (accounts, signatures, sequences) | Transaction validation |
| 7–8 | Bank module (balances, transfers, TokenFactory) | Token operations |
| 9–10 | Staking module (validators, delegations, rewards) | Validator set management |
| 11–12 | Wasm host (cosmwasm-vm, message dispatch) | Contract execution |
| 13–14 | IBC integration (ibc-rs contexts, proofs) | Cross-chain communication |
| 15 | Rank engine + graph store integration | GPU compute, cyberlinks |
| 16 | State migration testing, IAVL → JMT | Testnet rehearsal |
| +4 | Buffer: IBC coordination, cross-vendor testing | Mainnet readiness |

**Team**: 3–4 Rust developers. Ideal: experience with Namada, Penumbra, or Nomic codebases.

**Rollback**: Reverse state migration (JMT → IAVL). This is why extensive testnet validation is critical before Phase 3 mainnet.

---

## Part VI — Phase 4: Hardening

### 6.1 Performance Optimization

Profile with `perf` and `flamegraph`. Optimize hot paths: tx deserialization, state reads, CosmWasm VM calls. Tune RocksDB for blockchain workload. Target: >1000 TPS for cyberlink operations, <2GB RAM for full node.

### 6.2 Security Audit

External audit of all native modules (bank, staking, auth, IBC integration). Formal verification of fixed-point arithmetic in rank engine. Fuzz testing of all message handlers. Property-based state machine testing for invariants.

### 6.3 Ecosystem Tooling

Rust CLI (replacing Go CLI). Docker images. Ansible/Terraform deployment scripts. Prometheus metrics + Grafana dashboards. Block explorer compatibility via API-compatible gRPC endpoints.

### 6.4 Duration

8 weeks. Can overlap with Phase 3 final testing.

---

## Part VII — Phase 5a: Rs Language

Rs is a strict superset of Rust — all valid Rust is valid Rs. New capabilities are behind `#![edition = "rs"]` and implemented as attribute macros or compiler extensions. Rs adds four features missing from standard Rust that are essential for deterministic, real-time, bare-metal blockchain systems.

### 7.1 Bounded Async

Rust's `async fn` creates futures with no deadline. A forgotten `.await` on a network read blocks forever. In consensus, this costs real money (slashing).

```rust
// Standard Rust async — still valid
async fn standard_function() -> Result<()> { /* ... */ }

// Rs bounded async — deadline in the signature
async(100ms) fn read_block(lba: u64) -> Result<Block> {
    let data = device.read(lba).await;  // .await inherits 100ms deadline
    Ok(Block::from(data))
}

// The compiler enforces: if the future doesn't resolve within 100ms,
// it returns Err(DeadlineExceeded). No silent hangs. No forgotten timeouts.
```

Under `#![edition = "rs"]`, the compiler rejects unbounded `.await` in any function with a deadline ancestor. Deadlines propagate: if `f` has a 100ms deadline, any `g.await` inside `f` inherits a deadline ≤ 100ms.

### 7.2 Deterministic Functions

Blockchain consensus requires identical output on every node. Rust doesn't guarantee this.

```rust
#[deterministic]
fn compute_rank(weights: &[FixedPoint<u128, 18>]) -> FixedPoint<u128, 18> {
    let mut sum = FixedPoint::ZERO;
    for w in weights {
        sum = sum.checked_add(*w)?;  // checked arithmetic required
    }
    sum
}
```

Inside `#[deterministic]`, the compiler rejects:

| Rejected | Reason |
|----------|--------|
| `f32`, `f64` | Non-deterministic across platforms |
| Unchecked `+`, `-`, `*` | Overflow differs debug/release |
| `HashMap` iteration | Order is non-deterministic |
| `std::time::Instant` | Wall clock varies |
| `rand::*` | Randomness |
| Raw pointer arithmetic | Addresses vary |
| Non-`#[deterministic]` calls | Transitivity |

What IS allowed: `BTreeMap`, `Vec`, `checked_add/mul/div`, `FixedPoint`, `Cid`, all `#[deterministic]` functions.

### 7.3 Typed Registers (MMIO without Unsafe)

OS kernels need to interact with hardware registers. Standard Rust requires `unsafe` for raw pointer dereference. Rs provides a safe abstraction:

```rust
#[register(base = 0x4000_0000, bank_size = 0x100)]
mod net_dma {
    #[reg(offset = 0x00, access = "rw")]
    pub struct Control {
        #[field(bits = 0..1)]
        pub enabled: bool,
        #[field(bits = 1..2)]
        pub interrupt_on_complete: bool,
    }

    #[reg(offset = 0x08, access = "ro")]
    pub struct Status {
        #[field(bits = 0..1)]
        pub tx_complete: bool,
    }
}

// Usage — fully safe, compiler-checked
fn init_dma(regs: &net_dma::Registers) {
    regs.control.write(Control { enabled: true, interrupt_on_complete: true });
    while !regs.status.read().tx_complete {}  // no unsafe anywhere
}
```

The macro generates safe wrappers around `volatile_read`/`volatile_write`. The `unsafe` exists in generated code — auditable, minimal, correct by construction. User code never touches `unsafe`.

### 7.4 Cell Declarations

Self-contained, hot-swappable OS modules with resource budgets and state migration:

```rust
cell! {
    name: KnowledgeGraph,
    version: 2,
    budget: 1500ms,
    heartbeat: 1s,

    state {
        links: BoundedMap<Cid, Cyberlink, 10_000_000>,
        agent_links: BoundedMap<Address, BoundedVec<Cid, 100_000>, 1_000_000>,
        link_count: u64,
    }

    epoch_state {
        new_links_this_epoch: BoundedVec<Cyberlink, 50_000>,
    }

    pub fn cyberlink(&mut self, from: Cid, to: Cid, agent: Address) -> Result<()> {
        self.state.links.insert((from, to), Cyberlink { from, to, agent, height: current_height() })?;
        self.epoch_state.new_links_this_epoch.push(Cyberlink { from, to, agent, height: current_height() })?;
        self.state.link_count += 1;
        Ok(())
    }

    migrate from v1 {
        links: old.links,
        agent_links: BoundedMap::new(),  // new field in v2
        link_count: old.links.len() as u64,
    }
}
```

Generated code provides: init/shutdown lifecycle, health reporting, budget enforcement, state serialization, migration harness, hot-swap protocol (freeze → export → replace → import → resume).

### 7.5 Rs Standard Library Extensions

| Module | Purpose |
|--------|---------|
| `rs::fixed_point` | Fixed-point arithmetic for deterministic consensus math |
| `rs::bounded` | `BoundedVec<T, N>`, `BoundedMap<K, V, N>` — compile-time capacity limits |
| `rs::channel` | Bounded, backpressured channels (no unbounded queues ever) |
| `rs::cid` | Content Identifier (CID) as first-class type |
| `rs::arena` | Region-based arena allocator for epoch-scoped allocations |

### 7.6 Compiler Implementation

Rs is implemented as a rustc patch (not a fork). Three phases:

**Phase 1 — Library only (works today)**: `#[deterministic]` as proc macro, `cell!` as declarative macro, `BoundedVec`/`FixedPoint` as library types. ~2,000 lines.

**Phase 2 — Compiler patch**: `async(Nms)` deadline enforcement, `register!` hardware safety proofs, cross-crate determinism checking. ~8,000 lines of rustc changes.

**Phase 3 — Upstream proposal**: RFC for bounded async, IEC 61508 safety argument for `register!`.

---

## Part VIII — Phase 5b: CyberOS

CyberOS is a purpose-built operating system for running Bostrom validators. Not a general-purpose OS. It does exactly one thing: run a decentralized superintelligence node with maximum reliability, security, and performance.

### 8.1 Design Axioms

1. **No processes** — cells (hot-swappable, budget-constrained modules)
2. **No file system** — content-addressed storage (Merkle trees, CIDs)
3. **No users** — cryptographic agents (addresses, signatures)
4. **No TCP/IP stack** — three purpose-built protocols (gossip, consensus, query)
5. **Bounded liveness** — every async operation has a deadline, no hangs ever
6. **Zero unsafe** — hardware abstraction via typed registers, not raw pointers

### 8.2 Kernel Architecture

```
┌──────────────────────────────────────────────────────┐
│  Application Cells                                    │
│  ┌──────────┐ ┌──────────┐ ┌──────┐ ┌───────────┐  │
│  │Consensus │ │Knowledge │ │Rank  │ │Bandwidth  │  │
│  │Cell      │ │Graph Cell│ │Cell  │ │Cell       │  │
│  └──────────┘ └──────────┘ └──────┘ └───────────┘  │
│  ┌──────────┐ ┌──────────┐ ┌──────┐                 │
│  │Query Cell│ │Wasm Cell │ │IBC   │                 │
│  │(gRPC/    │ │(CosmWasm)│ │Cell  │                 │
│  │ QUIC)    │ │          │ │      │                 │
│  └──────────┘ └──────────┘ └──────┘                 │
├──────────────────────────────────────────────────────┤
│  Epoch Budget Allocator                               │
│  ┌────────────────────────────────────────────────┐  │
│  │ Total epoch: 6000ms per block                  │  │
│  │ Consensus: 2000ms (hard) ← cannot be starved   │  │
│  │ Execution: 3000ms (firm) ← tx processing       │  │
│  │ Background: 1000ms (soft) ← rank, indexing     │  │
│  └────────────────────────────────────────────────┘  │
├──────────────────────────────────────────────────────┤
│  Rs Runtime (Bounded Liveness + Deterministic Core)   │
├──────────────────────────────────────────────────────┤
│  Neural Drivers (LLM-generated, trait-conformant)     │
│  ┌───────┐ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐    │
│  │NVMe   │ │Net   │ │GPU   │ │UART  │ │Timer │    │
│  └───────┘ └──────┘ └──────┘ └──────┘ └──────┘    │
├──────────────────────────────────────────────────────┤
│  Hardware (x86_64, aarch64, riscv64)                  │
└──────────────────────────────────────────────────────┘
```

### 8.3 Neural Drivers

Hardware drivers are 70%+ of Linux's 30M lines. CyberOS inverts this: **drivers are generated by LLMs against stable trait contracts**.

```rust
/// Every block device must implement this trait.
/// LLMs generate implementations from datasheets.
pub trait BlockDevice: BoundedLiveness + Send + Sync {
    const BLOCK_SIZE: u32;
    const MAX_IO_LATENCY: Duration;

    async fn read_blocks(&self, lba: u64, buf: &mut [u8]) -> Result<usize>;
    async fn write_blocks(&self, lba: u64, buf: &[u8]) -> Result<usize>;
    fn capacity_blocks(&self) -> u64;
    async fn flush(&self) -> Result<()>;
    fn health(&self) -> DeviceHealth;
}
```

The Hardware Abstraction Layer is ~3,000 lines of trait definitions. Each driver implementation is ~200-500 lines, generated from datasheets + reference code by LLM, validated against a conformance test suite. Drivers that pass the test suite are submitted to the network. Validators vote on driver adoption. Network-wide driver improvement without centralized development.

### 8.4 Purpose-Built Networking

No general TCP/IP stack. Three protocols, each a separate cell:

| Protocol | Purpose | Transport |
|----------|---------|-----------|
| **Gossip** | Propagate txs and blocks to peers | UDP/QUIC |
| **Consensus** | Validator voting, proposals | UDP/QUIC |
| **Query** | Client requests (rank, graph, proofs) | QUIC streams |

~10-15K lines vs ~100K+ for full TCP/IP + HTTP + TLS. Each protocol cell has independent budget. Gossip cannot starve consensus. Queries cannot starve gossip.

### 8.5 Content-Addressed Storage

No file system. No paths, no inodes, no directories.

```rust
trait ContentStore: BoundedLiveness {
    async fn get(&self, cid: &Cid) -> Result<Option<Block>>;
    async fn put(&self, data: &[u8]) -> Result<Cid>;
    async fn merkle_root(&self) -> Hash;
    async fn prove_inclusion(&self, cid: &Cid) -> Result<MerkleProof>;
}
```

All persistent data is content-addressed: blockchain state in Merkle trees, knowledge graph indexed by CIDs, blocks indexed by height and hash. No open/close/read/write/seek. Append-only, Merkle-proven.

### 8.6 CyberOS vs Existing Approaches

| | Linux + Go (current) | CyberOS |
|---|---|---|
| Kernel | 30M lines, general purpose | ~50K lines, purpose-built |
| Drivers | Hand-written by specialists | LLM-generated, trait-conformant |
| File system | ext4, VFS, inodes | Content-addressed Merkle storage |
| Networking | Full TCP/IP stack | 3 purpose-built protocols |
| Processes | Unix processes, unlimited | Cells, budget-constrained |
| Async | Unbounded futures | Bounded liveness, deadlines |
| Safety | unsafe permitted everywhere | Zero-unsafe userspace (typed registers) |
| Updates | Package manager, reboot | Hot-swap cells, governance-driven |

---

## Part IX — Consolidated Plan

### 9.1 Full Timeline

```
Month 1:      Phase 0 (interfaces)
              Phase 2 starts (wgpu, parallel)
Month 1–4:    Phase 1 (CosmWasm migration)
Month 3:      Phase 2 completes (wgpu rank engine)
Month 5–9:    Phase 3 (Rust host)
Month 9–10:   Phase 4 (hardening)
Month 10–12:  Phase 3 mainnet, Phase 4 completes
Month 12+:    Phase 5 (Rs language + CyberOS, ongoing)
```

### 9.2 Governance Upgrade Sequence

Each phase is a separate governance proposal. Validators must achieve 2/3+ approval.

| Upgrade | Binary Change | State Change | Rollback |
|---------|---------------|--------------|----------|
| Phase 1 | wasmd binary with CW contracts | Go state → contract state | Revert to old binary |
| Phase 2 | wgpu rank engine replaces CUDA | None (ranks recomputed) | Revert binary (CUDA fallback) |
| Phase 3 | Rust binary + CometBFT sidecar | IAVL → JMT migration | Reverse migration (complex) |
| Phase 4 | Optimized Rust binary | None | Revert to Phase 3 binary |
| Phase 5 | CyberOS binary | Module → cell migration | Revert to Phase 4 binary |

### 9.3 Resource Estimates

| Phase | Team | Duration | Custom Lines |
|-------|------|----------|--------------|
| 0 | 1 Rust dev | 4 weeks | ~500 (interfaces) |
| 1 | 2–3 Rust + 1 Go | 15 weeks | ~6,700 (contracts + Go wrappers) |
| 2 | 1 GPU/Rust dev | 8 weeks | ~2,000 (rank engine) |
| 3 | 3–4 Rust devs | 16+4 weeks | ~18,000 (Rust host) |
| 4 | 2–3 Rust devs | 8 weeks | ~2,000 (tooling, tests) |
| 5 | 2–4 Rust devs | Ongoing | ~15,000+ (Rs compiler + CyberOS) |

### 9.4 Hardware Requirements After Migration

| Resource | Current (go-cyber) | After Phase 3 | After CyberOS |
|----------|-------------------|---------------|----------------|
| GPU | NVIDIA only (CUDA) | Any vendor (wgpu) | Any vendor |
| RAM | ~8GB | ~4GB (no GC overhead) | ~2GB (no OS overhead) |
| CPU | 4+ cores | 2+ cores | 2+ cores |
| Disk | ~500GB | ~300GB (compact store) | ~200GB |
| Binary | ~100MB (Go) | ~30MB (Rust) | ~15MB (no OS) |
| Processes | 1 (Go, embedded CometBFT) | 2 (Rust + CometBFT) | 1 (CyberOS) |

### 9.5 Risk Matrix

| Risk | Phase | Severity | Likelihood | Mitigation |
|------|-------|----------|------------|------------|
| CosmWasm gas limits hit during migration | 1 | Medium | Medium | Pre-test all contract ops, optimize storage layout |
| wgpu determinism failure on exotic GPUs | 2 | High | Low | Integer-only arithmetic, cross-vendor test suite |
| JMT ICS-23 proof incompatibility | 3 | Critical | Medium | Test with Hermes relayer early; Penumbra solved this |
| Staking module bugs (slashing, rewards) | 3 | Critical | Medium | Port Orga's O(1) staking; property testing |
| IAVL→JMT state migration data loss | 3 | Critical | Low | Multiple testnet rehearsals, verify every KV pair |
| Phase 3 rollback complexity | 3 | High | Low | Extensive testnet, parallel chain option |
| Rs compiler acceptance by Rust community | 5 | Medium | Medium | Start as proc macros, prove value before compiler patch |
| CyberOS driver coverage gaps | 5 | Medium | High | LLM generation covers common hardware; start with virtio |

### 9.6 Success Criteria

#### Per-Phase Gates

| Phase | Gate (must pass before mainnet) |
|-------|-------------------------------|
| 0 | All interfaces compile, no implementation dependencies |
| 1 | All 7 modules pass behavioral equivalence tests vs Go modules |
| 2 | Rank output bit-identical across NVIDIA, AMD, Intel GPUs |
| 3 | Full IBC roundtrip works (send + receive + ack on testnet) |
| 4 | External audit clean, no critical findings |
| 5 | CyberOS boots on 3+ hardware platforms, passes all Phase 3 tests |

#### Final State

After all phases, Bostrom achieves:

- **Zero custom Go code** in the critical path (CometBFT is external, standard, replaceable)
- **Any GPU** can validate (NVIDIA, AMD, Intel, Apple Silicon)
- **Single `cargo build`** produces the complete node binary
- **~18,000 lines** custom Rust application (vs 695,000 lines Go infrastructure)
- **Foundation for CyberOS**: cell architecture, Rs language, sovereign OS
- **Full IBC compatibility**: interchain communication preserved
- **Full API compatibility**: existing tools, wallets, explorers continue working
- **Improved performance**: no GC pauses, no FFI overhead, native Rust speed
- **Improved security**: Rust memory safety, 45x smaller attack surface

### 9.7 Honest Acknowledgments

**"Zero Go" means**: zero custom Go code maintained by Bostrom team. CometBFT remains a Go sidecar until Phase 5 (CyberOS) potentially replaces it with a Rust/Rs consensus engine. This is how Penumbra, Namada, and Nomic operate. It is the industry standard.

**No production precedent for**: pure Rust CosmWasm host (Phase 3), deterministic wgpu rank engine at scale (Phase 2), LLM-generated OS drivers in production (Phase 5). These are engineering firsts, not assembly of existing parts.

**The 12-month estimate is optimistic**. Phase 3 alone may take 6 months with a small team. CyberOS (Phase 5) is a multi-year effort. Each phase is independently valuable — the plan degrades gracefully if later phases are delayed.

---

## Appendix A — Crate Dependencies

```toml
[workspace.dependencies]
# Consensus interface
tower-abci = "0.16"
tendermint = "0.38"
tendermint-proto = "0.38"

# IBC
ibc = "0.54"
ibc-proto = "0.48"

# CosmWasm
cosmwasm-vm = "2.2"
cosmwasm-std = "2.2"
cosmwasm-schema = "2.2"

# State storage
jmt = "0.10"
rocksdb = "0.22"

# GPU compute
wgpu = "24"

# Networking / RPC
tonic = "0.12"
tokio = { version = "1", features = ["full"] }
prost = "0.13"

# Cryptography
k256 = "0.13"
ed25519-dalek = "2.1"
sha2 = "0.10"

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
borsh = "1.5"
```

## Appendix B — Existing Ecosystem Maturity

| Component | Crate | Maturity | Production Use | Risk |
|-----------|-------|----------|----------------|------|
| ABCI server | tower-abci | ✅ Stable | Penumbra, Namada, Fendermint | Low |
| Tendermint types | tendermint-rs | ✅ Stable | Penumbra, Namada, Nomic, many | Low |
| Merkle tree | jmt | ✅ Stable | Penumbra, Sovereign Labs, Astria | Low |
| IBC protocol | ibc-rs | ✅ Maturing | Namada, ibc-go test parity | Medium |
| CosmWasm VM | cosmwasm-vm | ✅ Stable | All CW chains (via FFI today) | Low |
| GPU compute | wgpu | ✅ Stable | Firefox, Bevy, many | Low |
| Protobuf | prost | ✅ Stable | Tonic ecosystem, widespread | Low |
| gRPC | tonic | ✅ Stable | Widespread | Low |
| Rust Cosmos SDK | Gears | 🔴 Early | AZKR-chain (dev only) | High |
| Native CW host | — | 🔴 None | No production example | High |

## Appendix C — Reference Architectures

### Penumbra (production Rust Cosmos chain)
```
CometBFT v0.37 (external Go binary)
    ↓ ABCI (tower-abci, async)
Penumbra pd (Rust binary)
    ├── tower-abci
    ├── penumbra-storage (async state, JMT)
    ├── ibc-types + penumbra-ibc (async IBC)
    ├── Custom modules (shielded pool, DEX, staking, governance)
    └── UTXO model (not account-based)
```

### Namada (production Rust Cosmos chain)
```
CometBFT v0.37.16 (external Go binary)
    ↓ ABCI (tower-abci)
Namada (Rust binary)
    ├── tower-abci
    ├── Custom state machine
    ├── ibc-rs (full IBC)
    ├── MASP (multi-asset shielded pool)
    └── PoS, governance, slashing
```

### Nomic (production Rust Cosmos chain)
```
CometBFT (external Go binary)
    ↓ ABCI (abci2, custom)
Nomic (Rust binary)
    ├── Orga framework (custom Rust SDK)
    ├── Merk (high-performance Merkle AVL, 2-20x faster than JMT)
    ├── Custom IBC
    ├── Bitcoin bridge
    └── PoS staking (O(1) delegations)
```

### Bostrom Phase 3 (planned)
```
CometBFT v0.38+ (external Go binary)
    ↓ ABCI++ (tower-abci, async)
cyber-node (Rust binary)
    ├── tower-abci + tendermint-rs
    ├── cyber-sdk (minimal: auth, bank, staking, store)
    ├── ibc-rs (full IBC)
    ├── cosmwasm-vm (native, no FFI)
    ├── rank-engine (wgpu, any GPU)
    ├── graph-store (native cyberlinks)
    └── CosmWasm contracts (unchanged from Phase 1)
```
