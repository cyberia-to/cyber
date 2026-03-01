# go-cyber → Rust: Complete Migration Plan

## Executive Summary

This document describes the complete migration path from `go-cyber` (Go/CUDA) to a pure Rust stack. The migration proceeds in four phases, progressively eliminating Go dependencies while maintaining chain continuity through standard Cosmos SDK governance upgrades. The end state is a single `cargo build` binary with zero Go code, cross-vendor GPU support via `wgpu`, and a clear path to CyberOS.

**Current state**: ~13,400 lines custom Go + 3.5% CUDA, NVIDIA-only, ~680K lines Go dependencies  
**End state**: ~9,500 lines Rust (contracts + rank engine), any GPU, single binary

---

## 1. Current Architecture: go-cyber v7.0.1

```
go-cyber v7.0.1
├── Cosmos SDK v0.47.12        Go, ~500K lines in dependencies
├── CometBFT v0.37.8           Go, ~100K lines
├── IBC-go v7.6.0              Go, ~80K lines
├── wasmd v0.45.0              Go host + CosmWasm 1.5.2 Rust VM
│
├── x/graph      — cyberlinks storage (CID → CID)           ~1,800 lines Go
├── x/rank       — Token-Weighted PageRank, GPU/CUDA         ~3,500 lines Go + CUDA
├── x/bandwidth  — resource metering for cyberlinks (VOLT)   ~1,200 lines Go
├── x/resources  — investmint (HYDROGEN → VOLT/AMPERE)       ~1,000 lines Go
├── x/dmn        — autonomous programs / thoughts             ~700 lines Go
├── x/grid       — energy routing (VOLT & AMPERE)             ~800 lines Go
├── x/liquidity  — InterChain AMM                            ~2,500 lines Go
│
├── merkle/      — custom Merkle tree for rank proofs         ~400 lines Go
├── plugins/     — GPU computation bindings                   ~500 lines Go
└── app wiring   — app.go, ante handlers, genesis            ~1,500 lines Go

Total custom: ~13,400 lines Go + ~500 lines CUDA
Language split: 93.3% Go, 3.5% CUDA, 3.2% other
GPU requirement: NVIDIA only (CUDA vendor lock)
```

### Critical Dependencies

| Dependency | Language | Lines | Role |
|---|---|---|---|
| Cosmos SDK v0.47.12 | Go | ~500K | Application framework |
| CometBFT v0.37.8 | Go | ~100K | Consensus engine |
| IBC-go v7.6.0 | Go | ~80K | Cross-chain communication |
| wasmd v0.45.0 | Go+Rust | ~15K Go | CosmWasm host |
| wasmvm | Rust (FFI) | ~20K | WASM execution engine |

### Module Dependency Graph

```
                    ┌─────────────────────┐
                    │  BeginBlocker /      │
                    │  EndBlocker (Go)     │
                    └───┬──────────┬───────┘
                        │          │
                   ┌────▼────┐ ┌───▼─────┐
                   │ x/rank  │ │ x/dmn   │
                   │ (CUDA)  │ │ (cron)  │
                   └────┬────┘ └───┬─────┘
                        │         │
                    ┌───▼─────────▼───┐
                    │  AnteHandler    │
                    │  (Go only)      │
                    └───────┬─────────┘
                            │
                    ┌───────▼─────────┐
                    │  x/bandwidth    │
                    └───────┬─────────┘
                            │
          ┌─────────────────┼─────────────────┐
          │                 │                  │
    ┌─────▼─────┐   ┌──────▼──────┐   ┌──────▼──────┐
    │ x/graph   │   │ x/resources │   │ x/grid      │
    │ (CRUD)    │   │ (tokenomics)│   │ (routing)   │
    └─────┬─────┘   └─────────────┘   └─────────────┘
          │
    ┌─────▼──────┐
    │x/liquidity │
    └────────────┘
```

---

## 2. CosmWasm Boundary Analysis

### What CosmWasm CAN Do

CosmWasm contracts have access to the full Cosmos SDK via messages and queries:

```rust
// Standard CosmWasm message types — already cover all SDK modules
CosmosMsg::Bank(BankMsg::Send { .. })           // x/bank
CosmosMsg::Staking(StakingMsg::Delegate { .. }) // x/staking
CosmosMsg::Distribution(DistMsg::SetWithdrawAddress { .. })
CosmosMsg::Gov(GovMsg::Vote { .. })
CosmosMsg::Ibc(IbcMsg::Transfer { .. })
CosmosMsg::Wasm(WasmMsg::Execute { .. })

// Escape hatch: ANY protobuf message via Stargate
CosmosMsg::Stargate { type_url: String, value: Binary }

// Same for queries
QueryRequest::Bank(BankQuery::Balance { .. })
QueryRequest::Staking(StakingQuery::Validators { .. })
QueryRequest::Stargate { path: String, data: Binary }
```

100% of Cosmos SDK functionality is accessible from CosmWasm through Stargate messages.

### What CosmWasm CANNOT Do

| Capability | Required By | Workaround |
|---|---|---|
| BeginBlocker / EndBlocker | x/rank, x/dmn | Neutron x/cron module → sudo calls |
| AnteHandler modification | x/bandwidth | Thin Go wrapper (~100 lines) calls CW query |
| GPU access | x/rank | Native plugin or wgpu rank-engine |
| Native token minting | x/resources | TokenFactory module (Osmosis pattern) |
| Self-execution | x/dmn auto-trigger | External cron → sudo entry point |

### Solution Map

```
Problem                    Solution                        Go code needed
─────────────────────────  ──────────────────────────────  ──────────────
BeginBlocker/EndBlocker    x/cron module (Neutron)         0 (plug-and-play)
AnteHandler                Go wrapper → CW query           ~100 lines
Token minting              TokenFactory (Osmosis)          0 (plug-and-play)
GPU PageRank               Phase 1: Go+CUDA native         ~300 lines
                           Phase 2: wgpu Rust native       0
Auto-execution             x/cron → sudo on contract       0
```

---

## 3. Module-by-Module Migration

### 3.1 x/graph → cw-graph

**Function**: Stores cyberlinks (CID_from → CID_to) tied to neuron accounts.

**Complexity**: 🟢 Simple — pure CRUD operations

**Estimated effort**: 2–3 weeks, ~1,200 lines Rust

```rust
// cw-graph/src/msg.rs
#[cw_serde]
pub struct Cyberlink {
    pub from: String,  // CID (particle)
    pub to: String,    // CID (particle)
}

#[cw_serde]
pub enum ExecuteMsg {
    Cyberlink { links: Vec<Cyberlink> },
    // bandwidth check delegated to cw-bandwidth via inter-contract query
}

#[cw_serde]
pub enum QueryMsg {
    GraphStats {},
    ParticleLinks { cid: String, pagination: Option<PageRequest> },
    IsLinked { from: String, to: String, agent: String },
    InLinks { cid: String, pagination: Option<PageRequest> },
    OutLinks { cid: String, pagination: Option<PageRequest> },
}
```

**State migration**: Genesis export from Go IAVL → JSON → cw-graph instantiate msg. All cyberlinks loaded at init.

**Dependencies**: Queries cw-bandwidth for rate limiting before accepting cyberlinks.

---

### 3.2 x/rank → rank-engine + cw-rank-verifier

**Function**: Computes Token-Weighted PageRank on GPU. The core computation of the network.

**Complexity**: 🔴 Architectural blocker — cannot run in CosmWasm (no GPU, no EndBlocker)

**Solution**: Split into two components:

#### cw-rank-verifier (CosmWasm contract, ~800 lines Rust)

Stores rank results and serves queries. Receives updates via sudo from the native rank module.

```rust
// cw-rank-verifier/src/msg.rs
#[cw_serde]
pub enum SudoMsg {
    UpdateRanks {
        ranks: Vec<(String, Uint128)>,  // CID → rank value
        merkle_root: Binary,
        epoch: u64,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Rank { cid: String },
    RankWithProof { cid: String },
    TopRanked { limit: u32 },
    Search { cid: String, page: Option<PageRequest> },
    RankParams {},
    CurrentEpoch {},
}
```

#### rank-engine (native Rust crate, ~2,000 lines)

GPU PageRank computation using `wgpu`. Replaces Go+CUDA entirely.

```rust
// rank-engine/src/lib.rs
pub struct RankEngine {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    graph_buffer: wgpu::Buffer,     // CSR adjacency matrix
    ranks_buffer: wgpu::Buffer,     // current rank vector
    weights_buffer: wgpu::Buffer,   // stake weights
    params: RankParams,
}

impl RankEngine {
    pub fn new() -> Result<Self> {
        // Request any GPU via wgpu — works on Vulkan, Metal, DX12, OpenGL ES
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        }).await?;
        let (device, queue) = adapter.request_device(&Default::default(), None).await?;
        // Load WGSL compute shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("pagerank"),
            source: wgpu::ShaderSource::Wgsl(include_str!("pagerank.wgsl").into()),
        });
        // ... pipeline setup
    }

    pub fn compute_epoch(&mut self, graph: &AdjacencyCSR, weights: &[u64]) -> Result<RankResult> {
        self.upload_graph(graph)?;
        self.upload_weights(weights)?;
        for _ in 0..self.params.convergence_iterations {
            self.dispatch_pagerank_iteration()?;
        }
        let ranks = self.readback_ranks()?;
        let merkle_root = build_merkle_tree(&ranks);
        Ok(RankResult { ranks, merkle_root, epoch: self.epoch })
    }
}
```

#### WGSL Compute Shader (deterministic PageRank)

```wgsl
// rank-engine/src/pagerank.wgsl
// Deterministic fixed-point PageRank — identical results on ALL GPUs

struct RankParams {
    damping: u32,      // 850000 = 0.85 in fixed-point (scale 1000000)
    num_nodes: u32,
    scale: u32,        // 1000000
    teleport: u32,     // (1 - damping) * scale / num_nodes
}

@group(0) @binding(0) var<storage, read> adj_offsets: array<u32>;
@group(0) @binding(1) var<storage, read> adj_targets: array<u32>;
@group(0) @binding(2) var<storage, read> edge_weights: array<u32>;
@group(0) @binding(3) var<storage, read> current_rank: array<u32>;
@group(0) @binding(4) var<storage, read_write> next_rank: array<u32>;
@group(0) @binding(5) var<uniform> params: RankParams;

@compute @workgroup_size(256)
fn pagerank_iteration(@builtin(global_invocation_id) gid: vec3<u32>) {
    let node = gid.x;
    if (node >= params.num_nodes) { return; }

    let start = adj_offsets[node];
    let end = adj_offsets[node + 1u];

    // Accumulate in u64 to prevent overflow
    var sum: u64 = 0u;
    for (var i = start; i < end; i = i + 1u) {
        let source = adj_targets[i];
        let weight = edge_weights[i];
        let source_rank = current_rank[source];
        let out_degree = adj_offsets[source + 1u] - adj_offsets[source];

        if (out_degree > 0u) {
            // Pure integer arithmetic — deterministic across all GPU vendors
            sum = sum + u64(source_rank) * u64(weight) / u64(out_degree);
        }
    }

    // Damping: new_rank = teleport + damping * sum / scale
    let damped = u64(params.damping) * sum / u64(params.scale);
    next_rank[node] = u32(u64(params.teleport) + damped);
}
```

**Determinism guarantee**: All arithmetic is integer-only (u32/u64). No floats. No vendor-specific rounding. WGSL integer operations are defined by the WebGPU spec to produce identical results on all compliant implementations. The adjacency list is sorted, ensuring deterministic reduction order.

**Hardware support**: wgpu abstracts across Vulkan (Linux/Windows/Android), Metal (macOS/iOS), DX12 (Windows), OpenGL ES (fallback). Works on NVIDIA, AMD, Intel, Apple Silicon, Qualcomm Adreno. Headless operation — no display server required.

**Migration path**:
- Phase 1: Go+CUDA native module (unchanged, ~300 lines Go wrapper)
- Phase 2: wgpu rank-engine as Rust crate, integrated via FFI bridge (~100 lines Go)
- Phase 3: Native integration in Rust host, zero Go

---

### 3.3 x/bandwidth → cw-bandwidth + Go AnteHandler wrapper

**Function**: Rate limiting for cyberlink operations based on VOLT stake.

**Complexity**: 🟡 Medium — logic ports cleanly, but enforcement requires AnteHandler

**Estimated effort**: 3 weeks, ~800 lines Rust (contract) + ~100 lines Go (AnteHandler)

```rust
// cw-bandwidth/src/msg.rs
#[cw_serde]
pub enum ExecuteMsg {
    // Called by cw-graph before accepting cyberlinks
    ConsumeBandwidth { agent: String, cost: Uint128 },
    // Called via sudo from EndBlocker to recover bandwidth
    RecoverBandwidth { block_time: u64 },
}

#[cw_serde]
pub enum QueryMsg {
    // Called by Go AnteHandler to check before tx execution
    CheckBandwidth { agent: String, required: Uint128 },
    AgentBandwidth { agent: String },
    BandwidthParams {},
    TotalBandwidth {},
}
```

```go
// Thin Go AnteHandler wrapper (~100 lines)
// Delegates all logic to cw-bandwidth contract
func (bw BandwidthAnteHandler) AnteHandle(ctx sdk.Context, tx sdk.Tx, ...) {
    for _, msg := range tx.GetMsgs() {
        if cyberlinkMsg, ok := msg.(*types.MsgCyberlink); ok {
            query := fmt.Sprintf(`{"check_bandwidth":{"agent":"%s","required":"%d"}}`,
                cyberlinkMsg.Neuron, estimateCost(cyberlinkMsg))
            result := bw.wasmKeeper.QuerySmart(ctx, bw.bandwidthContract, []byte(query))
            if !result.Allowed {
                return ctx, sdkerrors.Wrap(sdkerrors.ErrUnauthorized, "insufficient bandwidth")
            }
        }
    }
    return next(ctx, tx, simulate)
}
```

---

### 3.4 x/resources → cw-resources

**Function**: Investmint — lock HYDROGEN for a period, receive VOLT or AMPERE.

**Complexity**: 🟢 Simple with TokenFactory

**Estimated effort**: 2–3 weeks, ~600 lines Rust

```rust
// cw-resources/src/msg.rs
#[cw_serde]
pub enum ExecuteMsg {
    Investmint {
        resource: Resource,      // Volt or Ampere
        amount: Uint128,         // HYDROGEN to lock
        length: u64,             // lock period in blocks
    },
    Uninvestmint {
        agent: String,
        resource: Resource,
    },
}

#[cw_serde]
pub enum Resource {
    Volt,
    Ampere,
}
```

Uses TokenFactory for native token minting: contract creates `factory/{contract_address}/volt` and `factory/{contract_address}/ampere` denoms, mints and burns freely.

**Dependency**: TokenFactory module from Osmosis (Go, plug-and-play, no custom code).

---

### 3.5 x/dmn → cw-dmn

**Function**: Autonomous programs — scheduled contract executions ("thoughts").

**Complexity**: 🟡 Medium — requires cron trigger

**Estimated effort**: 2 weeks, ~500 lines Rust

```rust
// cw-dmn/src/msg.rs
#[cw_serde]
pub enum ExecuteMsg {
    CreateThought {
        program: String,         // target contract address
        trigger: Trigger,        // execution condition
        load: Binary,            // message to send
        name: String,
    },
    RemoveThought { name: String },
    ChangeThoughtInput { name: String, load: Binary },
    ChangeThoughtPeriod { name: String, period: u64 },
}

#[cw_serde]
pub enum SudoMsg {
    // Called by x/cron module each block
    ExecuteThoughts { block_height: u64, block_time: u64 },
}

#[cw_serde]
pub struct Trigger {
    pub period: u64,            // execute every N blocks
    pub block: u64,             // next execution block
}
```

**Dependency**: Neutron x/cron module (Go, plug-and-play). Calls `sudo { "execute_thoughts": { ... } }` on cw-dmn every block.

---

### 3.6 x/grid → cw-grid

**Function**: Energy routing — delegate VOLT/AMPERE from one neuron to another.

**Complexity**: 🟢 Simple — escrow + delegation accounting

**Estimated effort**: 1–2 weeks, ~500 lines Rust

```rust
// cw-grid/src/msg.rs
#[cw_serde]
pub enum ExecuteMsg {
    CreateRoute { destination: String, alias: String },
    EditRoute { destination: String, alias: String },
    DeleteRoute { destination: String },
    EditRouteAlias { destination: String, alias: String },
}

#[cw_serde]
pub enum QueryMsg {
    SourceRoutes { source: String },
    DestinationRoutes { destination: String },
    Route { source: String, destination: String },
    RoutedEnergy { source: String, destination: String },
}
```

---

### 3.7 x/liquidity → cw-liquidity (fork)

**Function**: AMM with MEV protection.

**Complexity**: 🟢 Ready-made solution exists

**Estimated effort**: 1 week (fork + config)

Use existing production CosmWasm AMM: Astroport, TerraSwap, or Osmosis contracts. Fork, configure denoms, deploy.

---

## 4. Target Architectures by Phase

### Phase 1: cw-cyber on wasmd

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
    └── cyber-merkle           Merkle tree for rank proofs (~300 lines)
```

**Result**: Custom Go reduced from ~13,400 to ~900 lines. All business logic in Rust.

### Phase 2: wgpu rank-engine (parallel with Phase 1)

```
rank-engine (standalone Rust crate)
├── src/
│   ├── lib.rs                GPU abstraction, pipeline setup
│   ├── pagerank.wgsl         WGSL compute shader (fixed-point)
│   ├── graph.rs              CSR adjacency matrix management
│   ├── merkle.rs             Rank Merkle tree
│   └── ffi.rs                C-ABI exports for Go FFI bridge
├── Cargo.toml                wgpu, bytemuck, blake3
└── tests/
    ├── determinism.rs         Cross-GPU determinism verification
    └── correctness.rs         PageRank correctness against reference impl
```

Integration into Phase 1 binary via Go FFI bridge:

```go
// ~100 lines: Go FFI bridge to Rust rank-engine
// #cgo LDFLAGS: -lrank_engine
// #include "rank_engine.h"
import "C"

func (k Keeper) ComputeRank(graph []byte) ([]byte, error) {
    result := C.rank_engine_compute(
        (*C.uint8_t)(unsafe.Pointer(&graph[0])),
        C.size_t(len(graph)),
    )
    // ...
}
```

**Result**: CUDA eliminated. Any GPU works. Custom Go still ~900 lines.

### Phase 3: Full Rust host

```
cw-cyber v3 (zero Go)
├── Rust binary (single cargo build)
│   ├── tendermint-abci       tower-abci or CometBFT via gRPC
│   ├── ibc-rs                Rust IBC implementation (Hermes team)
│   ├── cosmwasm-vm           native Rust, no FFI
│   ├── rank-engine           wgpu, native integration
│   ├── state-store           jellyfish-merkle or custom
│   ├── bank                  minimal token accounting
│   ├── staking               validator set management
│   └── governance            DAO-DAO contracts or custom
│
├── CosmWasm contracts (unchanged from Phase 1)
│   └── ... same contracts, zero modifications
│
└── Zero Go. Zero CGO. Zero CUDA.
    Single binary. Any GPU. cargo build.
```

### Phase 4: Rs / CyberOS edition

```
CyberOS
├── Rs runtime
│   ├── Cell scheduler (bounded async)
│   ├── Consensus cell
│   ├── Rank cell (wgpu)
│   ├── Graph cell
│   ├── WasmRuntime cell (runs CW contracts)
│   └── Neural driver cells
│
├── CW contracts (from Phase 1, run inside WasmRuntime cell)
│
└── Rs language extensions
    ├── #[deterministic] functions
    ├── cell! {} declarations
    ├── async(Nms) bounded futures
    └── #[epoch] scoped state
```

---

## 5. Interface Abstraction Layer

The key architectural insight: define ALL interactions as CosmWasm messages/queries. The backend (Go or Rust) becomes swappable behind the interface.

```rust
// cyber-interfaces/src/lib.rs — trait crate, imported by all contracts

/// Messages that cyber contracts can send
#[cw_serde]
pub enum CyberMsg {
    Graph(GraphMsg),
    Rank(RankMsg),
    Bandwidth(BandwidthMsg),
    Resources(ResourcesMsg),
    Grid(GridMsg),
    Dmn(DmnMsg),
}

/// Queries that cyber contracts can make
#[cw_serde]
pub enum CyberQuery {
    Graph(GraphQuery),
    Rank(RankQuery),
    Bandwidth(BandwidthQuery),
    Resources(ResourcesQuery),
    Grid(GridQuery),
}

// Each module defines its interface:
#[cw_serde]
pub enum GraphMsg {
    Cyberlink { links: Vec<Cyberlink> },
}

#[cw_serde]
pub enum RankQuery {
    Rank { cid: String },
    Search { cid: String, page: Option<PageRequest> },
    TopRanked { limit: u32 },
    IsLinkExist { from: String, to: String },
}
```

Contracts program against interfaces. Phase 1 resolves them through Go SDK. Phase 3 resolves them through Rust modules. The contracts never change.

---

## 6. State Migration

### Single Governance Upgrade Transaction

```
Block N:     go-cyber v7 running (Go modules hold all state)
                ↓ governance proposal: SoftwareUpgrade "cw-cyber-v1"
Block N+1:   chain halts at upgrade height
                ↓ validators switch binary
Block N+2:   cw-cyber v1 starts
                ↓ upgrade handler executes:
                │
                │  1. Export x/graph state → instantiate cw-graph
                │     (all cyberlinks loaded into contract storage)
                │
                │  2. Export x/bandwidth state → instantiate cw-bandwidth
                │     (all agent bandwidth balances)
                │
                │  3. Export x/resources state → instantiate cw-resources
                │     (all active investmint positions)
                │     Create tokenfactory denoms for VOLT, AMPERE
                │
                │  4. Export x/grid state → instantiate cw-grid
                │     (all energy routes)
                │
                │  5. Export x/dmn state → instantiate cw-dmn
                │     (all registered thoughts)
                │
                │  6. Export x/rank state → instantiate cw-rank-verifier
                │     (latest rank snapshot + merkle root)
                │
                │  7. Register contract addresses in app state
                │  8. Remove old Go module stores
                │
Block N+3:   normal operation — all queries routed to CW contracts
```

This is a standard Cosmos SDK chain upgrade. One governance vote. Validators update their binary. State migrates automatically during InitGenesis of the new binary.

### Migration Script Structure

```go
// app/upgrades/cw_cyber_v1.go
func CreateUpgradeHandler(
    mm *module.Manager,
    configurator module.Configurator,
    wasmKeeper wasm.Keeper,
    graphKeeper graph.Keeper,
    // ... other keepers
) upgradetypes.UpgradeHandler {
    return func(ctx sdk.Context, plan upgradetypes.Plan, fromVM module.VersionMap) (module.VersionMap, error) {
        // 1. Store contract code
        graphCodeID := storeCode(ctx, wasmKeeper, "cw_graph.wasm")
        // ...

        // 2. Export existing state
        graphState := graphKeeper.ExportGenesis(ctx)

        // 3. Instantiate contracts with migrated state
        graphAddr := instantiate(ctx, wasmKeeper, graphCodeID, GraphInstantiateMsg{
            Links: convertLinks(graphState.Links),
            Params: convertParams(graphState.Params),
        })

        // 4. Register contract addresses
        app.SetContractAddress("graph", graphAddr)
        // ...

        return fromVM, nil
    }
}
```

---

## 7. Existing Rust Ecosystem

Components that already exist in production Rust and can be used directly:

| Component | Go (current) | Rust alternative | Status |
|---|---|---|---|
| Consensus | CometBFT | tendermint-rs (Informal Systems) | Production — used by Penumbra, Namada |
| ABCI interface | Go ABCI server | tower-abci | Production |
| IBC | ibc-go | ibc-rs (Hermes) | Production |
| Light client | Go tendermint-light-client | tendermint-light-client-rs | Production |
| State store | IAVL tree | jellyfish-merkle (Aptos origin) | Production |
| Governance | x/gov | DAO-DAO (CosmWasm) | Production |
| Protobuf | protoc-gen-go | prost | Production |
| gRPC | grpc-go | tonic | Production |
| Cryptography | Go crypto | ring, ed25519-dalek, k256 | Production |
| CosmWasm VM | wasmvm (Go↔Rust FFI) | cosmwasm-vm (native Rust) | Production — already Rust |
| GPU compute | CUDA | wgpu + WGSL | Production — used by Firefox, Bevy |

**~70% of the required Rust infrastructure already exists in production.** Phase 3 is assembly, not invention.

---

## 8. Timeline and Resource Estimates

### Phase 1: cw-cyber on wasmd (15 weeks)

| Week | Task | Deliverable |
|---|---|---|
| 1–2 | Fork wasmd, integrate tokenfactory + cron | Build system, localbostrom |
| 3–4 | cw-graph: cyberlinks storage + queries | Contract + tests |
| 5 | cw-grid: energy routing | Contract + tests |
| 6–7 | cw-resources: investmint + tokenfactory | Contract + tests |
| 8–9 | cw-bandwidth: accounting + Go AnteHandler wrapper | Contract + thin Go |
| 10 | cw-dmn: thoughts + cron integration | Contract + tests |
| 11 | cw-rank-verifier + Go rank sudo bridge | Contract + Go bridge |
| 12 | cw-liquidity: fork AMM | Deploy config |
| 13 | Genesis migration script | Export → import test |
| 14 | Integration testing on testnet | Full system test |
| 15 | Validator coordination + mainnet governance | Mainnet upgrade |

**Team**: 2–3 Rust developers, 1 Go developer (part-time for wrappers)

### Phase 2: wgpu rank-engine (8 weeks, parallel with Phase 1)

| Week | Task | Deliverable |
|---|---|---|
| 1–2 | wgpu setup, headless compute pipeline | GPU init + basic dispatch |
| 3–4 | WGSL PageRank shader, fixed-point arithmetic | Compute shader + tests |
| 5 | CSR graph management, GPU upload/readback | Graph data structures |
| 6 | Merkle tree integration | Rank proofs |
| 7 | Determinism testing across GPU vendors | Cross-vendor test suite |
| 8 | FFI bridge to Go host (or native integration) | Integration |

**Team**: 1 GPU/Rust developer

### Phase 3: Full Rust host (12 weeks)

| Week | Task | Deliverable |
|---|---|---|
| 1–3 | ABCI server (tower-abci + CometBFT gRPC) | Block processing |
| 4–6 | State store (jellyfish-merkle or custom) | Persistent state |
| 7–8 | CosmWasm VM integration (native) | Contract execution |
| 9–10 | Bank, staking, governance modules | Core SDK |
| 11 | IBC integration (ibc-rs) | Cross-chain |
| 12 | Migration, testing, deployment | Mainnet upgrade |

**Team**: 3–4 Rust developers

### Phase 4: Rs/CyberOS (ongoing)

Cell-based architecture, bounded async, neural drivers. No fixed timeline — this is the research frontier.

### Total: Phases 1–3 = ~35 weeks to zero Go

Phases 1 and 2 run in parallel, so effective timeline:

```
Month 1–4:   Phase 1 (cw-cyber) + Phase 2 (wgpu rank) in parallel
Month 4:     Mainnet upgrade to cw-cyber v1, CUDA eliminated
Month 5–7:   Phase 3 (Rust host)
Month 7:     Mainnet upgrade to cw-cyber v3, zero Go
Month 8+:    Phase 4 (CyberOS)
```

---

## 9. Risk Analysis

| Risk | Impact | Probability | Mitigation |
|---|---|---|---|
| wgpu determinism variance across GPUs | Consensus failure | Low | Fixed-point integer-only arithmetic; exhaustive cross-vendor testing |
| CosmWasm gas limits for large state migrations | Failed genesis | Medium | Batch instantiation; paginated state loading |
| CometBFT ↔ Rust ABCI compatibility | Phase 3 blocker | Low | Already proven by Penumbra, Namada |
| Rank computation performance regression | Slower blocks | Medium | Benchmark wgpu vs CUDA early; wgpu supports same parallelism |
| State migration data loss | Chain halt | Low | Extensive testnet rehearsal; rollback plan |
| TokenFactory incompatibility with VOLT/AMPERE | Token economics break | Low | Test on devnet; denom migration if needed |

---

## 10. Strategic Value

### Immediate (Phase 1)

- **Unified codebase**: All business logic in Rust, single language for contracts and future OS
- **Forkless upgrades**: Governance-driven contract migration, no binary recompilation
- **Community access**: Anyone can write Cyber extensions in Rust
- **LLM-friendly**: Rust contracts are easier for AI-assisted development than Go modules

### Medium-term (Phase 2)

- **GPU liberation**: Any GPU works — NVIDIA, AMD, Intel, Apple Silicon
- **Validator accessibility**: Dramatically lower hardware barrier for validators
- **Pure Rust build**: No CGO, no CUDA toolkit, no multi-language toolchain

### Long-term (Phase 3+)

- **CyberOS foundation**: Same contracts run in CyberOS via WasmRuntime cell
- **Single binary**: One `cargo build` produces the entire node
- **Cell architecture**: Hot-swappable modules, bounded async, deterministic compute
- **Path to Rs**: Compile-time guarantees (determinism, deadlines) that CosmWasm cannot provide

### Metrics

| Metric | go-cyber v7 | cw-cyber v1 | cw-cyber v3 |
|---|---|---|---|
| Custom Go lines | 13,400 | 900 | 0 |
| Custom Rust lines | 0 | 6,700 | 9,500 |
| CUDA lines | 500 | 500 → 0 | 0 |
| Go dependencies | ~680K lines | ~680K lines | 0 |
| GPU requirement | NVIDIA only | NVIDIA → any | Any |
| Build toolchain | Go + CUDA + CGO | Go + Rust | Rust only |
| Upgrade mechanism | Binary replace | Governance migrate | Governance migrate |
| Contract language | Go | Rust | Rust |
| Path to CyberOS | None | Direct | Native |

---

## 11. Open Questions

1. **Rank oracle trust model** (Phase 2 interim): If rank is computed off-chain before full Rust host, who submits results? Validator multisig? Governance whitelist?

2. **wgpu fallback**: What happens on nodes with no GPU? CPU software renderer (wgpu supports this) — acceptable for light nodes but not validators.

3. **IBC compatibility**: Ensuring ibc-rs maintains packet-level compatibility with ibc-go during Phase 3 transition.

4. **State store migration**: IAVL → jellyfish-merkle requires proof-format migration. IBC relayers must handle both proof types during transition.

5. **ZK rank verification** (future): PageRank in ZK circuits for trustless off-chain computation. Research-level — monitor SP1, Risc0, Jolt for feasibility.

---

## 12. Conclusion

The migration from go-cyber to a pure Rust stack is achievable in approximately 7 months through four phases. The critical insight is the **interface abstraction**: by defining all module interactions as CosmWasm messages and queries, the backend becomes a swappable implementation detail. Contracts written in Phase 1 will run unchanged through Phase 3 and into CyberOS.

The `wgpu` breakthrough eliminates the CUDA vendor lock, transforming the validator hardware requirement from "NVIDIA GPU + CUDA toolkit" to "any GPU + `cargo build`." Combined with fixed-point integer arithmetic in WGSL shaders, this achieves deterministic cross-vendor GPU computation — the core technical challenge identified in the December 2024 WebGPU VM design.

Each phase delivers standalone value:
- **Phase 1** removes custom Go, enabling Rust-only development
- **Phase 2** removes NVIDIA dependency, democratizing validation
- **Phase 3** removes Go entirely, achieving a single-binary Rust node
- **Phase 4** transforms the node into CyberOS with cell-based architecture

One governance proposal per phase. Standard Cosmos upgrade mechanism. No revolutionary risk — evolutionary migration.
