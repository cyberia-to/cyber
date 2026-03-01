---
tags: cyber, bostrom, migration, rust, research
icon: "\U0001F980"
---

# Bostrom → Rust: Complete Migration Path

## From go-cyber to a Pure Rust Superintelligence Node

**Version**: 1.0 · March 2026
**Scope**: Full elimination of Go from the Bostrom blockchain stack
**Target**: Single `cargo build` producing a complete validator binary

---

## Executive Summary

Bostrom (go-cyber v7) runs ~613,000 lines of Go code to support ~13,400 lines of custom logic. The chain depends on NVIDIA CUDA for GPU-accelerated PageRank, creating a vendor lock that limits the validator set. This document describes a phased migration to a pure Rust stack that eliminates Go entirely, removes CUDA dependency via wgpu cross-vendor GPU compute, and establishes the foundation for CyberOS — a sovereign operating system for decentralized superintelligence.

The migration is structured in 5 phases over 12–18 months, with each phase producing a working, upgradeable chain. No phase requires a hard fork — all transitions happen via standard Cosmos governance proposals.

---

## 1. Current Architecture Inventory

### 1.1 Dependency Stack

| Layer | Component | Language | Lines of Code | Role |
|-------|-----------|----------|---------------|------|
| Consensus | CometBFT v0.37.8 | Go | ~100,000 | Block production, BFT voting, P2P gossip |
| Framework | Cosmos SDK v0.47.12 | Go | ~500,000 | Module system, tx routing, state, auth |
| IBC | ibc-go v7.6.0 | Go | ~80,000 | Cross-chain communication |
| Wasm VM | wasmd v0.45.0 | Go + Rust | ~15,000 Go / ~50,000 Rust | CosmWasm smart contract execution |
| Custom | x/graph | Go | ~1,800 | Cyberlink storage (CID → CID) |
| Custom | x/rank | Go + CUDA | ~3,500 | Token-weighted PageRank on GPU |
| Custom | x/bandwidth | Go | ~1,200 | Rate limiting via VOLT tokens |
| Custom | x/resources | Go | ~1,000 | Investmint (HYDROGEN → VOLT/AMPERE) |
| Custom | x/dmn | Go | ~700 | Autonomous programs (thoughts) |
| Custom | x/grid | Go | ~800 | Energy routing/delegation |
| Custom | x/liquidity | Go | ~2,500 | Interchain AMM with MEV protection |
| GPU | CUDA kernels | CUDA C | ~470 | PageRank matrix operations |

**Totals**: ~93.3% Go, ~3.5% CUDA, ~3.2% other. Approximately 13,400 lines of custom code sitting on ~695,000 lines of Go infrastructure.

### 1.2 Critical Dependencies and Constraints

**CUDA vendor lock**: Only NVIDIA GPUs can run validators. Excludes AMD, Intel, Apple Silicon. Limits validator decentralization significantly.

**CGO bridge**: The CUDA rank module uses CGO to call C/CUDA from Go. CGO breaks cross-compilation, complicates builds, introduces undefined behavior at the FFI boundary, and prevents static linking.

**Go garbage collector**: Non-deterministic pauses affect block production timing. Not critical at current load, but becomes a bottleneck under scale.

**wasmd FFI overhead**: Go wasmd calls into Rust cosmwasm-vm via FFI. Every contract call crosses the Go→Rust boundary twice (call + return). In pure Rust, this overhead vanishes.

### 1.3 What We Keep vs. What We Replace

| Component | Action | Rationale |
|-----------|--------|-----------|
| Tendermint BFT consensus algorithm | Keep (re-implement in Rust) | Battle-tested, IBC-compatible |
| ABCI interface | Keep (Rust implementation exists) | Standard protocol, language-agnostic |
| IBC protocol | Keep (Rust implementation exists) | Interchain communication essential |
| CosmWasm VM | Keep (already Rust) | Contract ecosystem, already native |
| Cosmos SDK module system | Replace | Go-specific, not needed in Rust |
| x/bank, x/staking, x/gov etc. | Replace with minimal Rust modules | Only need features Bostrom uses |
| CUDA PageRank | Replace with wgpu | Cross-vendor GPU, pure Rust |
| Go binary and build system | Eliminate | Target: `cargo build` only |

---

## 2. Existing Rust Infrastructure

A critical insight: roughly 70% of the required Rust infrastructure already exists in production. This is not a build-from-scratch effort.

### 2.1 Consensus & Networking

| Crate | Maintainer | Status | Used By |
|-------|-----------|--------|---------|
| `tendermint-rs` | Informal Systems | Production | Penumbra, Namada, Hermes |
| `tower-abci` | Penumbra Labs | Production | Penumbra, Namada, Astria |
| `cometbft-rs` | CometBFT team | Production | Multiple Cosmos chains |
| `ibc-rs` / `ibc-types` | Informal / Penumbra | Production | Hermes relayer |
| `libp2p` | Protocol Labs | Production | Multiple networks |

### 2.2 State & Storage

| Crate | Maintainer | Status | Description |
|-------|-----------|--------|-------------|
| `jmt` (Jellyfish Merkle Tree) | Penumbra + Sovereign | Production | State commitment tree |
| `penumbra-storage` | Penumbra Labs | Production | Async state management |
| `rocksdb` / `sled` | Community | Production | Key-value storage backends |
| `cosmwasm-vm` | Confio | Production | Wasm contract execution (already Rust) |
| `cosmwasm-std` | Confio | Production | Contract standard library |

### 2.3 Cryptography

| Crate | Purpose | Status |
|-------|---------|--------|
| `ed25519-dalek` | Validator signing | Production |
| `k256` / `secp256k1` | User accounts (Cosmos-compatible) | Production |
| `sha2` / `blake3` | Hashing | Production |
| `ring` | TLS, general crypto | Production |
| `cosmwasm-crypto` | CosmWasm verification | Production |

### 2.4 Serialization & RPC

| Crate | Purpose | Status |
|-------|---------|--------|
| `prost` | Protobuf (Cosmos SDK compatibility) | Production |
| `tonic` | gRPC server/client | Production |
| `serde` / `serde_json` | JSON serialization | Production |
| `cosmos-sdk-proto` | Cosmos protobuf types in Rust | Production |

### 2.5 GPU Compute

| Crate | Purpose | Status |
|-------|---------|--------|
| `wgpu` | Cross-vendor GPU abstraction | Production (v24+) |
| `naga` | WGSL shader compiler | Production |

### 2.6 Precedent: Pure Rust Tendermint Chains

Two production chains prove this architecture works:

**Penumbra** — Privacy-focused L1, pure Rust, Tendermint consensus via tower-abci, custom IBC implementation, Jellyfish Merkle Tree for state. Does not use Cosmos SDK. Live on mainnet.

**Namada** — Privacy L1, Rust, CometBFT consensus, custom state machine, IBC-compatible. Also does not use Cosmos SDK. Live on mainnet.

Both chains demonstrate that Cosmos SDK (Go) is not required to participate in the IBC ecosystem. Only the ABCI protocol and IBC protocol compliance are required — both available in Rust.

---

## 3. Migration Phases

### Overview

```
Phase 0   Phase 1          Phase 2         Phase 3         Phase 4         Phase 5
NOW       CW Contracts     wgpu Rank       Rust Host       Unification     CyberOS
(go-cyber (logic → Rust    (CUDA → wgpu    (Go host →      (optimize,      (cell arch,
 v7)       via CosmWasm)    cross-vendor)    Rust host)      audit, harden)  Rs edition)
          
 Go+CUDA   Go+Rust+CUDA    Go+Rust+wgpu    Rust+wgpu       Rust+wgpu       Rs+wgpu
 ~613K Go  ~900 Go custom  ~600 Go custom  0 Go            0 Go            0 Go
           ~6.7K Rust CW   ~6.7K Rust CW   ~15K Rust       ~12K Rust       ~10K Rs
                           ~2K Rust GPU    ~2K Rust GPU    ~2K Rust GPU    ~2K Rs GPU

 4 weeks   15 weeks         8 weeks         12 weeks        8 weeks         ongoing
           ──────────────────────────────────────────────────────────────────────────→
                            ↑ parallel with Phase 1
```

---

### Phase 0: Interface Definition (4 weeks)

**Goal**: Define every interaction boundary as a Rust trait or CosmWasm message type. This is the single most important step — correct interfaces make every subsequent phase a backend swap.

#### 0.1 CosmWasm Message Interfaces

Create a `cyber-interfaces` crate defining typed messages for all custom modules:

```rust
// cyber-interfaces/src/lib.rs
// This crate has ZERO implementation. Only types and traits.

pub mod graph {
    use cosmwasm_schema::{cw_serde, QueryResponses};

    #[cw_serde]
    pub enum ExecuteMsg {
        Cyberlink {
            particle_from: String,  // CIDv0
            particle_to: String,    // CIDv0
        },
        CyberlinkBatch {
            links: Vec<Link>,
        },
        DeleteCyberlink {
            particle_from: String,
            particle_to: String,
        },
    }

    #[cw_serde]
    #[derive(QueryResponses)]
    pub enum QueryMsg {
        #[returns(IsLinkedResponse)]
        IsLinked {
            particle_from: String,
            particle_to: String,
        },
        #[returns(LinksResponse)]
        ParticleLinks {
            particle: String,
            direction: Direction,
            start_after: Option<String>,
            limit: Option<u32>,
        },
        #[returns(GraphStatsResponse)]
        GraphStats {},
    }

    #[cw_serde]
    pub struct Link {
        pub particle_from: String,
        pub particle_to: String,
    }

    #[cw_serde]
    pub enum Direction { From, To, Both }

    // ... response types
}

pub mod rank {
    #[cw_serde]
    pub enum SudoMsg {
        /// Called by the rank engine after each computation cycle
        UpdateRanks {
            merkle_root: String,
            ranks_count: u64,
            computation_block: u64,
        },
    }

    #[cw_serde]
    #[derive(QueryResponses)]
    pub enum QueryMsg {
        #[returns(RankResponse)]
        ParticleRank { particle: String },
        #[returns(SearchResponse)]
        Search {
            particle: String,
            page: Option<u32>,
            per_page: Option<u32>,
        },
        #[returns(RankParamsResponse)]
        RankParams {},
        #[returns(KarmaResponse)]
        AccountKarma { address: String },
    }
}

pub mod bandwidth {
    #[cw_serde]
    pub enum QueryMsg {
        #[returns(BandwidthResponse)]
        AccountBandwidth { address: String },
        #[returns(BandwidthParamsResponse)]
        Params {},
        #[returns(TotalBandwidthResponse)]
        TotalBandwidth {},
    }
    // AnteHandler check interface
    #[cw_serde]
    pub struct CheckBandwidthRequest {
        pub sender: String,
        pub msg_count: u32,
        pub tx_size: u64,
    }
    #[cw_serde]
    pub struct CheckBandwidthResponse {
        pub allow: bool,
        pub remaining: u64,
        pub limit: u64,
    }
}

pub mod resources {
    #[cw_serde]
    pub enum ExecuteMsg {
        Investmint {
            amount: cosmwasm_std::Coin,   // HYDROGEN
            resource: ResourceType,       // VOLT or AMPERE
            length: u64,                  // lock duration in blocks
        },
    }
    #[cw_serde]
    pub enum ResourceType { Volt, Ampere }
}

pub mod dmn {
    #[cw_serde]
    pub enum ExecuteMsg {
        CreateThought {
            program: String,     // contract address
            trigger: Trigger,
            load: Binary,        // message to execute
            name: String,
        },
        RemoveThought { name: String },
        ChangeThoughtInput { name: String, input: Binary },
        ChangeThoughtPeriod { name: String, period: u64 },
        ChangeThoughtBlock { name: String, block: u64 },
    }
    #[cw_serde]
    pub enum Trigger {
        Period { period: u64 },
        Block { block: u64 },
    }
}

pub mod grid {
    #[cw_serde]
    pub enum ExecuteMsg {
        CreateRoute { destination: String, name: String },
        EditRoute { destination: String, value: cosmwasm_std::Coin },
        DeleteRoute { destination: String },
        EditRouteName { destination: String, name: String },
    }
}
```

#### 0.2 Host-Level Trait Abstractions

```rust
// cyber-traits/src/lib.rs
// Defines the ABCI application boundary — implementation-agnostic

pub trait CyberApp: Send + Sync {
    fn info(&self) -> AppInfo;
    fn init_chain(&mut self, genesis: Genesis) -> Result<()>;
    fn begin_block(&mut self, header: BlockHeader) -> Result<Vec<Event>>;
    fn deliver_tx(&mut self, tx: &[u8]) -> Result<TxResult>;
    fn end_block(&mut self, height: u64) -> Result<EndBlockResult>;
    fn commit(&mut self) -> Result<[u8; 32]>;  // app hash
    fn query(&self, path: &str, data: &[u8]) -> Result<QueryResult>;
}

pub trait StateStore: Send + Sync {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<()>;
    fn delete(&mut self, key: &[u8]) -> Result<()>;
    fn root_hash(&self) -> [u8; 32];
    fn prove(&self, key: &[u8]) -> Result<MerkleProof>;
}

pub trait RankEngine: Send + Sync {
    fn load_graph(&mut self, adjacency: &AdjacencyData) -> Result<()>;
    fn load_weights(&mut self, stakes: &StakeWeights) -> Result<()>;
    fn compute(&mut self, params: RankParams) -> Result<RankResult>;
    fn get_rank(&self, cid_index: u64) -> Result<u64>;  // fixed-point rank
    fn merkle_root(&self) -> [u8; 32];
}

pub trait ConsensusEngine: Send + Sync {
    async fn start(&mut self, app: Box<dyn CyberApp>) -> Result<()>;
    async fn stop(&mut self) -> Result<()>;
}
```

#### 0.3 Deliverables

- `cyber-interfaces` crate: ~1,200 lines, published to workspace
- `cyber-traits` crate: ~400 lines, published to workspace
- Integration tests with mock implementations
- Documentation: every message, every query, every error code

**Exit criteria**: Any developer can implement a CyberApp against these traits without knowing whether Go or Rust is behind them.

---

### Phase 1: CosmWasm Contract Migration (15 weeks)

**Goal**: Move all custom business logic from Go modules to CosmWasm contracts (Rust). The Go host becomes a thin runtime.

#### 1.1 Module Migration Map

| Go Module | → CW Contract | Est. Lines (Rust) | Weeks | Dependencies |
|-----------|---------------|-------------------|-------|--------------|
| x/graph | cw-graph | ~1,200 | 2–3 | None |
| x/grid | cw-grid | ~500 | 1–2 | None |
| x/resources | cw-resources | ~600 | 2–3 | TokenFactory module |
| x/liquidity | cw-liquidity | ~2,000 | 2–3 | Fork Astroport/TerraSwap |
| x/bandwidth | cw-bandwidth | ~800 | 2–3 | Go AnteHandler wrapper |
| x/dmn | cw-dmn | ~500 | 2 | Go cron module (x/cron) |
| x/rank | cw-rank-verifier | ~400 | 1–2 | Rank engine (native) |
| **Total** | | **~6,000** | **13–18** | |

#### 1.2 Contract Details

**cw-graph** (simplest, migrate first)

The knowledge graph. Pure CRUD with indexed storage.

```rust
// Storage schema
const LINKS: Map<(&str, &str), LinkMeta> = Map::new("links");
const FROM_INDEX: Map<&str, Vec<String>> = Map::new("from_idx");
const TO_INDEX: Map<&str, Vec<String>> = Map::new("to_idx");
const STATS: Item<GraphStats> = Item::new("stats");

fn execute_cyberlink(
    deps: DepsMut,
    info: MessageInfo,
    particle_from: String,
    particle_to: String,
) -> Result<Response, ContractError> {
    // Validate CID format
    validate_cid(&particle_from)?;
    validate_cid(&particle_to)?;

    // Check duplicate
    if LINKS.has(deps.storage, (&particle_from, &particle_to)) {
        return Err(ContractError::LinkExists {});
    }

    // Store link with metadata
    let meta = LinkMeta {
        neuron: info.sender.clone(),
        height: deps.api.block_height(),
        timestamp: deps.api.block_time(),
    };
    LINKS.save(deps.storage, (&particle_from, &particle_to), &meta)?;

    // Update indices (for rank engine graph reads)
    // ...

    // Update stats
    STATS.update(deps.storage, |mut s| -> StdResult<_> {
        s.cyberlinks += 1;
        Ok(s)
    })?;

    Ok(Response::new()
        .add_attribute("action", "cyberlink")
        .add_attribute("from", particle_from)
        .add_attribute("to", particle_to)
        .add_attribute("neuron", info.sender))
}
```

**cw-rank-verifier** (rank results receiver)

The actual rank computation stays native (GPU). This contract receives and stores results:

```rust
fn sudo_update_ranks(
    deps: DepsMut,
    merkle_root: String,
    ranks_count: u64,
    computation_block: u64,
) -> Result<Response, ContractError> {
    // Only callable via sudo (host chain only)
    let state = RANK_STATE.load(deps.storage)?;

    // Store new rank merkle root
    RANK_STATE.save(deps.storage, &RankState {
        merkle_root: merkle_root.clone(),
        ranks_count,
        last_computation_block: computation_block,
        previous_merkle_root: state.merkle_root,
    })?;

    Ok(Response::new()
        .add_attribute("action", "update_ranks")
        .add_attribute("merkle_root", merkle_root)
        .add_attribute("ranks_count", ranks_count.to_string()))
}
```

Individual rank lookups query the native rank engine via Stargate custom query.

**cw-resources** (investmint)

Uses TokenFactory for minting VOLT and AMPERE:

```rust
fn execute_investmint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    resource: ResourceType,
    length: u64,
) -> Result<Response, ContractError> {
    // Validate HYDROGEN sent
    let hydrogen = must_pay(&info, "hydrogen")?;

    // Calculate mint amount based on halving schedule
    let params = PARAMS.load(deps.storage)?;
    let mint_amount = calculate_mint_amount(hydrogen, length, &params, env.block.height)?;

    // Lock HYDROGEN (escrow in contract)
    let unlock_height = env.block.height + length;
    LOCKS.save(deps.storage, info.sender.as_str(), &Lock {
        amount: hydrogen,
        unlock_height,
    })?;

    // Mint resource token via TokenFactory
    let denom = match resource {
        ResourceType::Volt => "factory/contract_addr/millivolt",
        ResourceType::Ampere => "factory/contract_addr/milliampere",
    };

    let mint_msg = TokenFactoryMsg::Mint {
        denom: denom.into(),
        amount: mint_amount,
        mint_to: info.sender.to_string(),
    };

    Ok(Response::new()
        .add_message(CosmosMsg::Custom(mint_msg))
        .add_attribute("action", "investmint")
        .add_attribute("resource", format!("{:?}", resource))
        .add_attribute("amount", mint_amount.to_string()))
}
```

**cw-bandwidth** (rate limiting logic + Go AnteHandler wrapper)

The accounting logic lives in the contract. The enforcement happens at the Go AnteHandler level via a query to the contract:

```rust
// Contract side: accounting
fn query_check_bandwidth(
    deps: Deps,
    sender: String,
    msg_count: u32,
    tx_size: u64,
) -> StdResult<CheckBandwidthResponse> {
    let params = PARAMS.load(deps.storage)?;
    let account = ACCOUNTS.may_load(deps.storage, &sender)?
        .unwrap_or_default();

    // Load VOLT balance via bank query
    let volt_balance = deps.querier.query_balance(&sender, "millivolt")?;

    // Calculate bandwidth limit proportional to VOLT stake
    let limit = calculate_bandwidth_limit(volt_balance.amount, &params);
    let cost = calculate_tx_cost(msg_count, tx_size, &params);
    let remaining = limit.saturating_sub(account.consumed);

    Ok(CheckBandwidthResponse {
        allow: remaining >= cost,
        remaining,
        limit,
    })
}
```

Go AnteHandler (thin wrapper, ~50 lines):

```go
// This is the ONLY custom Go code for bandwidth enforcement.
// It queries the CW contract to check bandwidth.
func (bw BandwidthAnteHandler) AnteHandle(ctx sdk.Context, tx sdk.Tx, simulate bool, next sdk.AnteHandler) (sdk.Context, error) {
    sender := tx.GetMsgs()[0].GetSigners()[0].String()
    resp := bw.wasmKeeper.QuerySmart(ctx, bw.contractAddr, []byte(fmt.Sprintf(
        `{"check_bandwidth":{"sender":"%s","msg_count":%d,"tx_size":%d}}`,
        sender, len(tx.GetMsgs()), tx.Size(),
    )))
    var result CheckBandwidthResponse
    json.Unmarshal(resp, &result)
    if !result.Allow {
        return ctx, sdkerrors.Wrap(sdkerrors.ErrInsufficientFee, "insufficient bandwidth")
    }
    return next(ctx, tx, simulate)
}
```

**cw-dmn** (autonomous programs)

Thoughts registry. Auto-execution triggered by x/cron module (from Neutron):

```rust
// Stores thought definitions
fn execute_create_thought(
    deps: DepsMut,
    info: MessageInfo,
    program: String,
    trigger: Trigger,
    load: Binary,
    name: String,
) -> Result<Response, ContractError> {
    // Validate program address exists
    deps.api.addr_validate(&program)?;

    THOUGHTS.save(deps.storage, (&info.sender, &name), &Thought {
        program: deps.api.addr_validate(&program)?,
        trigger,
        load,
        owner: info.sender.clone(),
    })?;

    Ok(Response::new().add_attribute("action", "create_thought"))
}

// Called by cron module each block via sudo
fn sudo_tick(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let mut msgs = vec![];
    let height = env.block.height;

    // Iterate active thoughts, fire those whose trigger matches
    for thought in THOUGHTS.range(deps.storage, None, None, Order::Ascending) {
        let (_, t) = thought?;
        if should_fire(&t.trigger, height) {
            msgs.push(WasmMsg::Execute {
                contract_addr: t.program.to_string(),
                msg: t.load.clone(),
                funds: vec![],
            });
        }
    }

    Ok(Response::new().add_messages(msgs))
}
```

#### 1.3 Minimal Go Host After Phase 1

```
wasmd (vanilla fork)
  + x/tokenfactory        (from Osmosis, ~2,000 lines — well-tested)
  + x/cron                (from Neutron, ~500 lines — calls sudo on contracts)
  + x/rank (native)       (existing Go+CUDA, ~3,500 lines — temporary)
  + BandwidthAnteHandler  (~50 lines — queries CW contract)
  + CustomQuerier          (~100 lines — routes rank queries to native module)
  + CustomMsgHandler       (~100 lines — routes resource mints to tokenfactory)
```

**Custom Go code remaining: ~900 lines** (down from 13,400).

Everything else is standard wasmd + community modules.

#### 1.4 Genesis State Migration

```
Block N:     go-cyber v7 processes normally
             ↓ governance proposal passes (SoftwareUpgrade)
Block N+1:   chain halts
             ↓ export genesis state
             ↓ migration script transforms:
               - x/graph state → cw-graph InitMsg + bulk cyberlink data
               - x/rank params → cw-rank-verifier InitMsg
               - x/bandwidth accounts → cw-bandwidth InitMsg
               - x/resources locks → cw-resources InitMsg
               - x/dmn thoughts → cw-dmn InitMsg
               - x/grid routes → cw-grid InitMsg
               - x/liquidity pools → cw-liquidity InitMsg
             ↓ new binary starts with migrated genesis
Block N+2:   cw-cyber v1 operates normally
             All previous state preserved. All queries work.
```

The migration script is a standalone Go binary (~2,000 lines) that reads old genesis JSON, transforms it into new genesis JSON with contract instantiation messages. This is standard Cosmos SDK upgrade practice — Osmosis, Neutron, and Juno have all done similar module-to-contract migrations.

#### 1.5 Testing Strategy

- **Unit tests**: Each contract tested with `cw-multi-test` (in-memory CosmWasm VM)
- **Integration tests**: Full chain binary with migrated testnet state
- **Differential testing**: Run old and new binaries against same block sequence, compare state hashes
- **Testnet**: Deploy to a public testnet with migrated Bostrom state for community testing
- **Audit**: External security review of all contracts before mainnet upgrade

#### 1.6 Deliverables

- 7 CosmWasm contracts, tested and audited
- Modified wasmd fork with tokenfactory, cron, rank module, ante handler
- Genesis migration script with full state coverage tests
- Testnet with migrated state running for minimum 2 weeks
- Governance proposal ready for mainnet

---

### Phase 2: wgpu Rank Engine (8 weeks, parallel with Phase 1)

**Goal**: Replace CUDA with cross-vendor GPU compute via wgpu. Any GPU works — NVIDIA, AMD, Intel, Apple Silicon.

#### 2.1 Why wgpu

| Property | CUDA | wgpu |
|----------|------|------|
| Vendor | NVIDIA only | Any GPU (Vulkan/Metal/DX12/OpenGL ES) |
| Language | CUDA C (proprietary) | WGSL (W3C standard) |
| Rust integration | CGO → C → CUDA (3 boundaries) | Native Rust crate (0 boundaries) |
| Determinism | Float-dependent (non-deterministic) | Fixed-point integers (deterministic by construction) |
| Headless compute | Requires display driver hacks | Native headless support |
| Build | NVIDIA toolkit + nvcc + CGO | `cargo build` |

#### 2.2 Deterministic Fixed-Point Arithmetic

The single biggest technical challenge: GPU floating-point is non-deterministic across vendors and even across driver versions. Consensus requires bit-exact results.

**Solution**: All computation uses fixed-point integer arithmetic. No floats anywhere.

```wgsl
// WGSL compute shader — PageRank iteration (deterministic)
// All values are u64 fixed-point with 18 decimal places
// SCALE = 1_000_000_000_000_000_000 (10^18)

struct Node {
    in_degree: u32,
    out_degree: u32,
    stake_weight: u32,       // fixed-point u32.16
    rank: u32,               // current rank (high 32 bits)
    rank_low: u32,           // current rank (low 32 bits) — together form u64
}

struct Edge {
    from_node: u32,
    to_node: u32,
}

@group(0) @binding(0) var<storage, read>       nodes: array<Node>;
@group(0) @binding(1) var<storage, read>       edges: array<Edge>;
@group(0) @binding(2) var<storage, read>       csr_offsets: array<u32>;
@group(0) @binding(3) var<storage, read_write> new_ranks: array<Node>;
@group(0) @binding(4) var<uniform>             params: RankParams;

struct RankParams {
    damping_num: u32,        // damping factor numerator (e.g. 85)
    damping_den: u32,        // damping factor denominator (e.g. 100)
    tolerance: u32,          // convergence tolerance (fixed-point)
    node_count: u32,
    edge_count: u32,
}

@compute @workgroup_size(256)
fn pagerank_iteration(@builtin(global_invocation_id) gid: vec3<u32>) {
    let idx = gid.x;
    if (idx >= params.node_count) { return; }

    // Accumulate incoming rank contributions
    // Using u64 arithmetic (emulated with two u32s in WGSL)
    var sum_high: u32 = 0u;
    var sum_low: u32 = 0u;

    let start = csr_offsets[idx];
    let end = csr_offsets[idx + 1u];

    for (var i = start; i < end; i = i + 1u) {
        let src = edges[i].from_node;
        let src_rank_high = nodes[src].rank;
        let src_rank_low = nodes[src].rank_low;
        let src_out = nodes[src].out_degree;

        if (src_out > 0u) {
            // Integer division: rank / out_degree
            // Canonical rounding: (a + b/2) / b
            let contrib = u64_div(src_rank_high, src_rank_low, src_out);
            let added = u64_add(sum_high, sum_low, contrib.high, contrib.low);
            sum_high = added.high;
            sum_low = added.low;
        }
    }

    // Apply stake weight
    let weighted = u64_mul_u32(sum_high, sum_low, nodes[idx].stake_weight);

    // Apply damping: new_rank = (1-d)/N + d * weighted_sum
    let base_high: u32 = 0u;
    let base_low: u32 = params.damping_den - params.damping_num; // (1-d) as integer
    // ... full fixed-point damping calculation

    new_ranks[idx].rank = result.high;
    new_ranks[idx].rank_low = result.low;
}
```

#### 2.3 Rank Engine Crate Structure

```
cyber-rank/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Public API implementing RankEngine trait
│   ├── gpu/
│   │   ├── mod.rs          # wgpu device setup, buffer management
│   │   ├── pipeline.rs     # Compute pipeline creation
│   │   ├── buffers.rs      # GPU buffer allocation and transfer
│   │   └── shaders/
│   │       ├── pagerank.wgsl
│   │       ├── normalize.wgsl
│   │       └── reduce.wgsl
│   ├── graph/
│   │   ├── mod.rs          # CSR (Compressed Sparse Row) representation
│   │   ├── csr.rs          # Adjacency matrix construction
│   │   └── weights.rs      # Stake-weighted node values
│   ├── fixed_point.rs      # Deterministic u64.18 arithmetic
│   ├── merkle.rs           # Rank Merkle tree for state commitment
│   └── fallback.rs         # CPU fallback for testing / no-GPU environments
├── tests/
│   ├── determinism.rs      # Cross-platform determinism verification
│   ├── correctness.rs      # Results match known-good reference
│   ├── gpu_vendors.rs      # Multi-vendor comparison tests
│   └── benchmark.rs        # Performance regression tests
└── benches/
    └── pagerank.rs         # Criterion benchmarks
```

#### 2.4 Integration with Go Host (Phase 1–2 bridge)

During the transitional period (Phase 1 Go host + Phase 2 Rust rank engine), the rank engine integrates via FFI:

```rust
// cyber-rank-ffi/src/lib.rs
// Thin FFI bridge: Go calls Rust rank engine
// This replaces the current Go→CGO→CUDA path

#[no_mangle]
pub extern "C" fn rank_engine_new() -> *mut RankEngine { ... }

#[no_mangle]
pub extern "C" fn rank_engine_load_graph(
    engine: *mut RankEngine,
    edges_ptr: *const u8,
    edges_len: usize,
) -> i32 { ... }

#[no_mangle]
pub extern "C" fn rank_engine_compute(
    engine: *mut RankEngine,
    result_ptr: *mut u8,
    result_len: *mut usize,
) -> i32 { ... }

#[no_mangle]
pub extern "C" fn rank_engine_free(engine: *mut RankEngine) { ... }
```

Go side (~50 lines):

```go
// #cgo LDFLAGS: -L${SRCDIR}/target/release -lcyber_rank_ffi
// #include "cyber_rank.h"
import "C"

func (r *RustRankEngine) ComputeRank(graph *Graph) (*RankResult, error) {
    result := C.rank_engine_compute(r.ptr, ...)
    // ...
}
```

This eliminates CUDA entirely. The Go binary links against a Rust `.so`/`.dylib` instead of CUDA libraries.

#### 2.5 Determinism Verification Protocol

Before any mainnet deployment, determinism must be proven across hardware:

1. **Reference dataset**: 100K nodes, 1M edges, known rank values from CPU computation
2. **Test matrix**:
   - NVIDIA (Ampere, Turing, Pascal)
   - AMD (RDNA 3, RDNA 2)
   - Intel Arc (Alchemist)
   - Apple M1/M2/M3 (Metal backend)
   - Mesa/llvmpipe software renderer (CI environments)
3. **Pass criteria**: Bit-exact u64 results across all platforms
4. **Continuous CI**: Every commit tested on at least NVIDIA + AMD + CPU fallback

#### 2.6 Deliverables

- `cyber-rank` crate with wgpu compute shaders
- `cyber-rank-ffi` crate for Go integration (temporary, Phase 1–2 bridge)
- Determinism test suite passing on 3+ GPU vendors
- Performance benchmarks: must match or exceed CUDA throughput
- Documentation: fixed-point arithmetic proofs, shader specifications

---

### Phase 3: Rust Host (12 weeks)

**Goal**: Replace the Go host entirely. Single Rust binary. Zero Go code.

#### 3.1 Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                        cyber-node                             │
│                    (single Rust binary)                        │
│                                                               │
│  ┌─────────────┐  ┌──────────────┐  ┌─────────────────────┐ │
│  │ tower-abci   │  │ tendermint   │  │   gRPC / REST API   │ │
│  │ (ABCI server)│  │  (external   │  │   (tonic server)    │ │
│  │              │  │   process)   │  │                     │ │
│  └──────┬───────┘  └──────────────┘  └─────────┬───────────┘ │
│         │                                       │             │
│  ┌──────▼───────────────────────────────────────▼───────────┐ │
│  │                    CyberApp                               │ │
│  │              (implements Application trait)                │ │
│  │                                                           │ │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌─────────────┐ │ │
│  │  │ tx_router │ │ ante_    │ │ begin/   │ │ query_      │ │ │
│  │  │          │ │ handler  │ │ end_block│ │ handler     │ │ │
│  │  └────┬─────┘ └────┬─────┘ └────┬─────┘ └──────┬──────┘ │ │
│  │       │             │            │               │        │ │
│  │  ┌────▼─────────────▼────────────▼───────────────▼──────┐ │ │
│  │  │                 Module Router                         │ │ │
│  │  │                                                       │ │ │
│  │  │  ┌──────────────────────────────────────────────────┐ │ │ │
│  │  │  │              CosmWasm Runtime                     │ │ │ │
│  │  │  │  ┌─────────┐ ┌─────────┐ ┌──────────┐          │ │ │ │
│  │  │  │  │cw-graph │ │cw-grid  │ │cw-bandw. │ ...      │ │ │ │
│  │  │  │  └─────────┘ └─────────┘ └──────────┘          │ │ │ │
│  │  │  └──────────────────────────────────────────────────┘ │ │ │
│  │  │                                                       │ │ │
│  │  │  ┌──────────────────────────────────────────────────┐ │ │ │
│  │  │  │              Native Modules                       │ │ │ │
│  │  │  │  ┌─────────┐ ┌─────────┐ ┌──────────┐          │ │ │ │
│  │  │  │  │  bank   │ │staking  │ │rank-eng. │ ...      │ │ │ │
│  │  │  │  └─────────┘ └─────────┘ └──────────┘          │ │ │ │
│  │  │  └──────────────────────────────────────────────────┘ │ │ │
│  │  └───────────────────────────────────────────────────────┘ │ │
│  │                                                           │ │
│  │  ┌───────────────────────────────────────────────────────┐ │ │
│  │  │                 State Layer                            │ │ │
│  │  │  ┌───────────────┐  ┌───────────────────────────────┐ │ │ │
│  │  │  │ Jellyfish MT   │  │ RocksDB / sled               │ │ │ │
│  │  │  │ (state commit) │  │ (key-value persistence)      │ │ │ │
│  │  │  └───────────────┘  └───────────────────────────────┘ │ │ │
│  │  └───────────────────────────────────────────────────────┘ │ │
│  └───────────────────────────────────────────────────────────┘ │
│                                                               │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │                  cyber-rank (wgpu)                         │ │
│  │  GPU PageRank engine, runs as background compute task      │ │
│  └───────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────┘
```

#### 3.2 Component Implementation Plan

**3.2.1 ABCI Server** — Use `tower-abci` from Penumbra (production-proven)

CometBFT continues as an external process connected via ABCI socket. This is the same architecture Penumbra and Namada use. CometBFT remains a Go binary, but it is a standard unmodified binary — no custom Go code.

```rust
use tower_abci::{BoxError, Server};

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let app = CyberApp::new(config)?;
    let server = Server::builder()
        .consensus(app.clone())
        .mempool(app.clone())
        .info(app.clone())
        .snapshot(app.clone())
        .finish()?;

    server.listen("127.0.0.1:26658").await?;
    Ok(())
}
```

**3.2.2 Minimal Native Modules**

Bostrom does not need the full Cosmos SDK x/bank, x/staking, x/gov etc. It needs a subset. These modules are implemented as Rust structs implementing simple traits:

| Module | Lines (est.) | Notes |
|--------|-------------|-------|
| `cyber-bank` | ~800 | Balance tracking, send, tokenfactory integration |
| `cyber-staking` | ~2,000 | Delegation, undelegation, validator set management |
| `cyber-distribution` | ~600 | Reward distribution, commission |
| `cyber-gov` | ~500 | Use DAO-DAO contracts for complex governance |
| `cyber-auth` | ~400 | Account management, sequence tracking |
| `cyber-ibc` | ~1,500 | Wrapping ibc-rs / ibc-types for ABCI integration |
| `cyber-wasm` | ~800 | CosmWasm VM integration (cosmwasm-vm is already Rust) |

**Total new Rust code: ~6,600 lines** for the host.

Combined with contracts (~6,000 lines) and rank engine (~2,000 lines), the entire stack is ~14,600 lines of Rust. Compare to the current ~613,000 lines of Go.

**3.2.3 State Migration: Go IAVL → Jellyfish Merkle Tree**

This is the highest-risk step. The state commitment tree changes format.

```
Approach: Snapshot-and-reimport

1. Final Go binary exports full state at halt height
   - Every key-value pair, every module store
   - Output: flat key-value dump (protobuf or JSON)

2. Rust binary reads dump and inserts into JMT
   - Preserves all data, recomputes Merkle roots
   - New app hash derived from JMT root

3. Validators compare new app hash via consensus
   - If 2/3+ agree, chain proceeds
   - Standard CometBFT upgrade mechanism
```

The state format changes but the data is preserved. IBC light client proofs will need one epoch of "trust migration" where counterparty chains update their client state.

**3.2.4 API Compatibility**

The Rust node must expose the same APIs:

- **CometBFT RPC** (port 26657): Handled by CometBFT process (unchanged)
- **gRPC** (port 9090): Implemented with `tonic`, using `cosmos-sdk-proto` message types
- **REST/LCD** (port 1317): gRPC-gateway or lightweight REST handler
- **WebSocket**: Event subscription via CometBFT (unchanged)

Existing clients (cyb.ai, CLI tools, indexers) continue working without modification.

#### 3.3 Workspace Structure

```
cyber-rs/
├── Cargo.toml                  # Workspace root
├── crates/
│   ├── cyber-node/             # Binary entry point
│   ├── cyber-app/              # CyberApp implementation
│   ├── cyber-bank/             # Token management
│   ├── cyber-staking/          # Validator set, delegation
│   ├── cyber-distribution/     # Reward distribution
│   ├── cyber-gov/              # Governance (minimal, delegates to CW)
│   ├── cyber-auth/             # Account management
│   ├── cyber-ibc/              # IBC integration
│   ├── cyber-wasm/             # CosmWasm VM integration
│   ├── cyber-rank/             # wgpu PageRank engine
│   ├── cyber-state/            # JMT + RocksDB state management
│   ├── cyber-interfaces/       # Message type definitions
│   └── cyber-traits/           # Trait abstractions
├── contracts/
│   ├── cw-graph/
│   ├── cw-rank-verifier/
│   ├── cw-bandwidth/
│   ├── cw-resources/
│   ├── cw-dmn/
│   ├── cw-grid/
│   └── cw-liquidity/
├── tools/
│   ├── genesis-migrate/        # State migration tool
│   └── cyber-cli/              # CLI client
└── tests/
    ├── integration/            # Full-chain integration tests
    └── determinism/            # Cross-platform rank determinism
```

Build: `cargo build --release` → single binary `cyber-node` (~30MB).

#### 3.4 CometBFT: The Last Go Process

After Phase 3, the only Go code is CometBFT itself — an unmodified external binary. The Rust node talks to it over a socket (ABCI protocol). This is acceptable because:

- CometBFT is maintained by Informal Systems / CometBFT team
- No custom patches needed
- It's replaceable: if a pure Rust consensus engine emerges (e.g., from Malachite/tendermint-rs), CometBFT can be swapped out
- Penumbra and Namada use this exact architecture in production

The path to 100% Rust including consensus exists (Malachite, a modular BFT engine in Rust, is under development by Informal Systems), but is not required for this migration.

#### 3.5 Deliverables

- `cyber-rs` workspace compiling to single binary
- All native modules implemented and tested
- State migration tool with round-trip verification
- API compatibility test suite (all existing endpoints work)
- Performance benchmarks vs. Go binary
- Testnet running for minimum 4 weeks
- Security audit of native modules
- Governance proposal for mainnet upgrade

---

### Phase 4: Hardening (8 weeks)

**Goal**: Optimize, audit, document. Prepare for long-term maintainability.

#### 4.1 Performance Optimization

- Profile with `perf` and `flamegraph`
- Optimize hot paths: tx deserialization, state reads, CosmWasm VM calls
- Tune RocksDB configuration for blockchain workload
- Benchmark: target >1000 TPS for cyberlink operations
- Memory footprint: target <2GB RAM for full node

#### 4.2 Security Audit

- External audit of all native modules (bank, staking, auth, IBC)
- Formal verification of fixed-point arithmetic in rank engine
- Fuzz testing of all message handlers
- State machine testing: property-based tests for invariants

#### 4.3 Documentation

- Architecture decision records (ADRs) for every design choice
- Operator guide: node setup, backup, upgrade procedures
- Developer guide: how to add new modules, extend the system
- API reference: all gRPC/REST endpoints with examples

#### 4.4 Ecosystem Tooling

- Updated `cyber` CLI in Rust (replacing Go CLI)
- Docker images for easy deployment
- Ansible/Terraform deployment scripts
- Monitoring: Prometheus metrics, Grafana dashboards
- Block explorer compatibility (existing explorers should work via API compatibility)

---

### Phase 5: CyberOS Foundation (ongoing)

**Goal**: Transform the Rust codebase into CyberOS — a sovereign operating system with cell-based hot-swappable modules.

This phase is not a single upgrade but a continuous evolution. Key milestones:

#### 5.1 Cell Architecture

Convert modules into cells — hot-swappable, budget-constrained units:

```rust
// Future: Rs language cell declaration
cell! {
    name: Graph,
    budget: 500ms,

    state {
        links: BTreeMap<(Cid, Cid), LinkMeta>,
        stats: GraphStats,
    }

    pub fn cyberlink(&mut self, from: Cid, to: Cid, neuron: Address) -> Result<()> {
        // ...
    }

    pub fn query_links(&self, particle: &Cid) -> Vec<Link> {
        // ...
    }
}
```

#### 5.2 Governance-Driven Module Upgrades

CosmWasm contracts can already be migrated via governance. Native cells extend this:

```
Governance proposal: "Upgrade Graph cell to v2"
  → Validators download new binary with updated Graph cell
  → Chain halts, state migrates, restarts
  → Zero downtime for all other cells
```

#### 5.3 Neural Drivers & Bounded Async

Autonomous agents that interact with the knowledge graph:

```rust
cell! {
    name: NeuralDriver,
    budget: 2000ms,

    #[deterministic]
    pub async(100ms) fn process_feed(&mut self) -> Result<Vec<Cyberlink>> {
        let recent = self.graph.query_recent(100)?;
        let ranked = self.rank.batch_query(&recent)?;
        // ... generate new links based on graph patterns
    }
}
```

#### 5.4 Rs Language Extensions

The 7 primitives (typed registers, bounded async, deterministic functions, content-addressed types, epoch-scoped state, cell declarations, owned regions) become available as the Rs compiler patch stabilizes. All existing Rust code continues to compile — Rs is a strict superset.

---

## 4. Risk Assessment

### 4.1 Technical Risks

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| wgpu determinism failure on some GPU | Medium | Critical | Extensive multi-vendor testing; CPU fallback always available |
| State migration data loss | Low | Critical | Round-trip verification; parallel testnet with real state |
| IBC compatibility break | Medium | High | Use battle-tested ibc-rs; extensive relay testing with Cosmos Hub |
| Performance regression vs Go | Low | Medium | Benchmark-driven development; Rust typically faster |
| CosmWasm VM behavior difference | Low | Medium | Same cosmwasm-vm crate used in both Go and Rust hosts |
| CometBFT compatibility issue | Low | Low | Standard ABCI protocol; well-documented interface |

### 4.2 Operational Risks

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Validator confusion during upgrade | Medium | Medium | Clear documentation, testnet rehearsal, validator coordination |
| Extended chain halt during migration | Low | High | Time-box migration to 2 hours; rollback procedure ready |
| Community resistance to change | Medium | Medium | Progressive delivery — each phase is independently valuable |
| Key developer unavailability | Medium | High | Document everything; multiple contributors per component |

### 4.3 Go-Free Risk Reduction

Eliminating Go actually reduces several current risks:

- **CGO undefined behavior**: Eliminated. No more FFI boundary between Go and CUDA.
- **Go GC pauses**: Eliminated. Rust has no GC.
- **Go module version conflicts**: Eliminated. `Cargo.lock` manages Rust dependencies deterministically.
- **NVIDIA driver dependency**: Eliminated. wgpu works with any GPU driver.
- **Build reproducibility**: Improved. `cargo build` is more reproducible than Go+CGO+CUDA toolchain.

---

## 5. Resource Estimates

### 5.1 Development Time (sequential, 1–2 developers)

| Phase | Duration | Parallel? | Cumulative |
|-------|----------|-----------|------------|
| Phase 0: Interfaces | 4 weeks | — | 4 weeks |
| Phase 1: CW Contracts | 15 weeks | — | 19 weeks |
| Phase 2: wgpu Rank | 8 weeks | Yes (with Phase 1) | 19 weeks |
| Phase 3: Rust Host | 12 weeks | — | 31 weeks |
| Phase 4: Hardening | 8 weeks | — | 39 weeks |

**Total: ~9 months** with Phases 1 and 2 running in parallel.

With 3–4 developers, this compresses to ~5–6 months.

### 5.2 Lines of Code

| Component | Go (current) | Rust (target) | Ratio |
|-----------|-------------|---------------|-------|
| Custom modules | 13,400 | 6,000 (CW contracts) | 0.45x |
| Host framework | 595,000 (SDK+CometBFT+wasmd) | 6,600 (native modules) | 0.01x |
| GPU rank | 3,500 + 470 CUDA | 2,000 | 0.50x |
| **Total custom** | **13,400** | **14,600** | **1.09x** |
| **Total including framework** | **613,000** | **14,600** | **0.024x** |

The Rust codebase is 42x smaller than Go because it uses existing Rust crates (tendermint-rs, tower-abci, cosmwasm-vm, jmt, tonic, prost) as dependencies rather than vendoring an entire framework.

### 5.3 Hardware Requirements After Migration

| Requirement | Current (go-cyber) | After Migration (cyber-rs) |
|-------------|--------------------|-----------------------------|
| GPU | NVIDIA (CUDA required) | Any GPU (or CPU fallback) |
| RAM | 16GB+ | 4–8GB (estimate) |
| CPU | 4+ cores | 2+ cores |
| Disk | SSD, 200GB+ | SSD, 100GB+ |
| Build toolchain | Go + CGO + CUDA toolkit + nvcc | Rust toolchain only |

---

## 6. Governance & Upgrade Path

### 6.1 Upgrade Sequence

Each phase is a separate governance proposal:

```
Proposal 1: "Upgrade to cw-cyber v1" (Phase 1 completion)
  - Custom logic moves to CosmWasm contracts
  - Go host simplified to ~900 lines custom code
  - CUDA rank temporarily retained
  - Validator binary: Go + Rust contracts

Proposal 2: "Enable wgpu rank engine" (Phase 2 completion)
  - CUDA removed
  - wgpu rank engine activated
  - Validator GPU requirement: any vendor
  - Validator binary: Go + Rust contracts + Rust rank

Proposal 3: "Upgrade to cyber-rs v1" (Phase 3 completion)
  - Go host eliminated
  - Full Rust binary
  - State migrated to JMT
  - Validator binary: Rust only (+ CometBFT external process)
```

### 6.2 Rollback Strategy

Each phase has a rollback plan:

- **Phase 1 rollback**: Revert governance proposal, validators switch back to old binary. Contract state is not lost (it was migrated from Go state).
- **Phase 2 rollback**: Switch back to CUDA rank engine via governance parameter change.
- **Phase 3 rollback**: More complex — requires reverse state migration (JMT → IAVL). This is why extensive testnet validation is critical before Phase 3.

---

## 7. Success Criteria

### Per-Phase Gates

| Phase | Gate | Verification |
|-------|------|--------------|
| 0 | All interfaces defined, mock tests pass | Code review, CI |
| 1 | All contracts deployed on testnet, state migrated, 2 weeks stable | Testnet monitoring |
| 1 | Existing cyb.ai and CLI work without modification | Integration tests |
| 2 | Rank results bit-identical on NVIDIA, AMD, Intel | Multi-vendor CI |
| 2 | Rank computation ≥ CUDA performance | Benchmarks |
| 3 | Full Rust binary running on testnet, 4 weeks stable | Testnet monitoring |
| 3 | All gRPC/REST APIs compatible | API test suite |
| 3 | IBC transfers working with Cosmos Hub testnet | Relay tests |
| 4 | External security audit passed | Audit report |
| 4 | All documentation complete | Review |

### Final State

After all phases complete, Bostrom achieves:

- **Zero Go code** in the critical path (CometBFT is external, standard, replaceable)
- **Any GPU** can validate (NVIDIA, AMD, Intel, Apple Silicon)
- **Single `cargo build`** produces the complete node binary
- **~14,600 lines** of custom Rust (vs. 613,000 lines Go)
- **Foundation for CyberOS**: cell architecture, Rs language, sovereign OS
- **Full IBC compatibility**: interchain communication preserved
- **Full API compatibility**: existing tools continue working
- **Improved performance**: no GC pauses, no FFI overhead, native Rust speed
- **Improved security**: Rust memory safety, smaller attack surface

---

## Appendices

### A. Reference Implementations

| What | Where | Notes |
|------|-------|-------|
| Penumbra (pure Rust Tendermint chain) | github.com/penumbra-zone/penumbra | Architecture reference |
| Namada (pure Rust Tendermint chain) | github.com/anoma/namada | IBC reference |
| tower-abci | github.com/penumbra-zone/tower-abci | ABCI server |
| tendermint-rs | github.com/informalsystems/tendermint-rs | Tendermint client libraries |
| ibc-rs | github.com/cosmos/ibc-rs | IBC protocol |
| jmt | github.com/penumbra-zone/jmt | Jellyfish Merkle Tree |
| cosmwasm-vm | github.com/CosmWasm/cosmwasm | Wasm runtime (already Rust) |
| wgpu | github.com/gfx-rs/wgpu | GPU compute |
| Neutron x/cron | github.com/neutron-org/neutron | Cron module for CW |
| Osmosis TokenFactory | github.com/osmosis-labs/osmosis | Token minting |
| DAO-DAO | github.com/DA0-DA0/dao-contracts | CosmWasm governance |

### B. Decision Log

| Decision | Choice | Rationale |
|----------|--------|-----------|
| State tree | Jellyfish Merkle Tree (not IAVL) | Used by Penumbra, Sovereign, co-developed; async-friendly; simpler than IAVL |
| ABCI server | tower-abci (not raw socket) | Production-proven, async, composable with tower middleware |
| IBC | ibc-rs + ibc-types (not custom) | Standard implementation, maintained by Cosmos ecosystem |
| Consensus | CometBFT external process (not Rust reimplementation) | Lowest risk; proven; replaceable later |
| GPU framework | wgpu (not OpenCL, not Vulkan direct) | Cross-vendor, W3C standard WGSL, native Rust, headless |
| Fixed-point format | u64 with 18 decimal places | Sufficient precision for PageRank, deterministic, no overflow risk with u64 accumulation |
| Contract framework | CosmWasm (not custom Wasm) | Existing ecosystem, tooling, auditing expertise |
| Serialization | Protobuf via prost (not custom) | Cosmos SDK compatibility, existing type definitions |

### C. Glossary

| Term | Definition |
|------|-----------|
| ABCI | Application BlockChain Interface — protocol between consensus engine and application |
| CID | Content Identifier — IPFS content-addressed hash |
| CSR | Compressed Sparse Row — efficient sparse matrix storage for graphs |
| CW / CosmWasm | WebAssembly smart contract framework for Cosmos chains |
| JMT | Jellyfish Merkle Tree — authenticated data structure for state commitment |
| Rs | Strict superset of Rust adding 7 primitives for systems that never reboot |
| wgpu | Rust implementation of WebGPU API for cross-platform GPU compute |
| WGSL | WebGPU Shading Language — W3C standard shader language |
| Fixed-point | Integer arithmetic simulating decimal numbers (e.g., u64 with 18 implicit decimal places) |
