---
tags: cyber, bostrom, gpu, wgsl, vm, research
icon: "\U0001F3AE"
---

# Cybergraph GPU VM Specification
**Version:** 0.1-experimental  
**Status:** Draft  
**Context:** Native Cosmos SDK module (`x/gpu`) embedded alongside CosmWasm in Bostrom/Cyber blockchain

---

## 1. Overview

The Cybergraph GPU VM is a consensus-embedded GPU compute layer allowing anyone to upload WGSL compute shaders and execute them as on-chain state transitions. Computation results are committed to the cybergraph as content-addressed CIDs, making programs, inputs, and outputs first-class nodes in the knowledge graph.

Unlike off-chain compute markets (Akash, Gensyn), execution results carry the same canonical weight as token balances — they are consensus state, not trusted oracle data.

---

## 2. Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│  Cosmos SDK Validator Process                                     │
│                                                                   │
│  ┌─────────────┐   ┌─────────────────┐   ┌────────────────────┐ │
│  │ x/cybergraph│   │   x/gpu module  │   │     CosmWasm       │ │
│  │             │◄──│                 │◄──│                    │ │
│  │ CID storage │   │ ShaderRegistry  │   │ can call GPU jobs  │ │
│  │ rank state  │   │ JobQueue        │   │ reads output CIDs  │ │
│  └─────────────┘   │ ResultCommitter │   └────────────────────┘ │
│                    └────────┬────────┘                           │
│                             │                                    │
│                    ┌────────▼────────┐                           │
│                    │  wgpu Executor  │                           │
│                    │                 │                           │
│                    │ Vulkan / Metal  │                           │
│                    │ DX12 / GL       │                           │
│                    │ llvmpipe (CPU)  │                           │
│                    └─────────────────┘                           │
└──────────────────────────────────────────────────────────────────┘
```

**Key invariant:** All validator hardware paths (GPU or CPU software fallback) must produce bit-identical output for any shader in the allowed subset. This is enforced by the instruction set restrictions in Section 4.

---

## 3. Message Types

### 3.1 `MsgUploadShader`
```protobuf
message MsgUploadShader {
  string sender        = 1;
  bytes  wgsl_source   = 2;  // UTF-8 WGSL source code
  string description   = 3;  // human readable
  repeated string tags = 4;  // for cybergraph indexing
}

message MsgUploadShaderResponse {
  string shader_cid    = 1;  // content hash of shader, stored in cybergraph
  ShaderProfile profile = 2; // static analysis result
}
```

On receipt:
1. Parse with `naga` → reject if invalid WGSL
2. Run subset validator (Section 4) → reject if uses blocked features
3. Run static analyzer → produce `ShaderProfile` (Section 6)
4. Store shader source as CID in cybergraph
5. Register `ShaderProfile` in `x/gpu` state keyed by CID

### 3.2 `MsgGpuExecute`
```protobuf
message MsgGpuExecute {
  string   sender        = 1;
  string   shader_cid    = 2;  // must exist in ShaderRegistry
  string   input_cid     = 3;  // raw bytes stored in cybergraph
  Dispatch dispatch      = 4;
  uint64   gas_limit     = 5;
  repeated Binding bindings = 6; // additional buffer bindings
  uint64   parent_clock  = 7;  // logical clock dependency (0 = none)
}

message Dispatch {
  uint32 x = 1;
  uint32 y = 2;
  uint32 z = 3;
}

message Binding {
  uint32 group   = 1;
  uint32 binding = 2;
  string cid     = 3;  // data CID for this binding slot
}

message MsgGpuExecuteResponse {
  string job_id      = 1;  // deterministic: hash(shader_cid + input_cid + block_height + sender)
  uint64 logical_clock = 2; // assigned clock value for this job
}
```

Execution is **asynchronous within the block** — job is queued at tx processing time, all GPU jobs execute together before `EndBlock`, results committed to state at `EndBlock`.

### 3.3 `MsgQueryResult`
```protobuf
message MsgQueryResult {
  string job_id = 1;
}

message MsgQueryResultResponse {
  JobStatus status     = 1;  // PENDING | EXECUTING | COMPLETE | FAILED
  string    output_cid = 2;  // set when COMPLETE
  uint64    gas_used   = 3;
  uint64    logical_clock = 4;
  string    error      = 5;  // set when FAILED
}
```

---

## 4. WGSL Instruction Set — Allowed Subset

### 4.1 Types — ALLOWED ✅

| Type | Notes |
|------|-------|
| `i32`, `u32` | Fully deterministic |
| `i64`, `u64` | Fully deterministic (when WGSL extension available) |

| `vec2/3/4<T>` | Integer base types only |
| `mat2x2/3x3/4x4<T>` | Integer base types only |
| `array<T, N>` | Fixed-size |
| `array<T>` | Runtime-sized, with bounds enforcement |
| `struct` | All fields must be allowed types |
| `bool` | Comparisons only |

### 4.2 Integer Operations — ALLOWED ✅ (unconditionally deterministic)

- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Bitwise: `&`, `|`, `^`, `~`, `<<`, `>>`
- Comparison: `<`, `>`, `<=`, `>=`, `==`, `!=`
- Atomics: `atomicAdd`, `atomicSub`, `atomicMax`, `atomicMin`, `atomicAnd`, `atomicOr`, `atomicXor`, `atomicExchange`, `atomicCompareExchangeWeak`
- Builtins: `abs`, `min`, `max`, `clamp`, `countOneBits`, `reverseBits`, `firstLeadingBit`, `firstTrailingBit`

### 4.3 Float Operations — BLOCKED ❌

All float types and operations are excluded in v0.1. This includes `f32`, `f16`, `f64`, all float arithmetic, and all float builtins. Algorithms requiring fractional values must use fixed-point integer representation (e.g. scale by 10^6, operate in `i64`, descale at output boundary).

**Fixed-point pattern for rank:**
```wgsl
// Instead of: rank: f32 = 0.85 * ...
// Use:        rank: i64 = 850000i * ... / 1000000i
// Scale factor declared in shader metadata
```

### 4.4 Control Flow — RULES

| Construct | Rule |
|-----------|------|
| `if / else` | Allowed |
| `switch` | Allowed |
| `for (var i = 0u; i < N; i++)` | Allowed if N is a **literal constant** or **uniform value bounded at upload time** |
| `loop {}` | **Blocked** — no statically provable bound |
| `while (runtime_condition)` | **Blocked** |
| `break`, `continue` | Allowed inside statically bounded `for` |
| `return` | Allowed |
| `discard` | **Blocked** (fragment shader concept) |

All loops must have their bound declared at upload time:
```wgsl
// VALID — literal bound
for (var i = 0u; i < 1024u; i++) { ... }

// VALID — uniform bound, declared max in shader metadata
for (var i = 0u; i < uniforms.iteration_count; i++) { ... }
// requires: iteration_count <= MAX_ITERATIONS declared in ShaderProfile

// INVALID — runtime data bound
for (var i = 0u; i < data[0]; i++) { ... }
```

### 4.5 Memory — RULES

| Feature | Rule |
|---------|------|
| `var<storage, read>` | Allowed |
| `var<storage, read_write>` | Allowed |
| `var<uniform>` | Allowed |
| `var<workgroup>` | Allowed |
| `var<private>` | Allowed |
| `workgroupBarrier()` | Allowed — **logical clock tick** |
| `storageBarrier()` | Allowed — **logical clock tick** |
| `textureBarrier()` | Blocked |
| Array bounds | Must be statically provable OR runtime-checked with gas penalty |

### 4.6 Blocked Features — BLOCKED ❌

| Feature | Reason |
|---------|--------|
| All texture operations | Non-deterministic sampling, meaningless in compute |
| All sampler types | Same |
| Subgroup/wave ops (`subgroupBallot` etc.) | Subgroup size varies per GPU model |
| `dpdx`, `dpdy`, `fwidth` | Fragment shader only |
| Pointer parameters | Excluded for simplicity in v0.1 |
| `atomicCompareExchangeWeak` spin loops | Unbounded — allowed only in bounded `for` |
| `@builtin(sample_index)` etc. | Fragment builtins |

---

## 5. Execution Model

### 5.1 Within a Block

```
BeginBlock
  └── initialize wgpu context if not warm

ProcessTxs (normal Cosmos flow)
  └── MsgGpuExecute → validate gas → enqueue job → assign logical clock

EndBlock
  └── resolve logical clock dependencies (topological sort)
  └── execute jobs in dependency order
  └── for each job:
       ├── bind input buffers from CID store
       ├── dispatch shader
       ├── read output buffer
       ├── hash output → output_cid
       ├── store output_cid in cybergraph
       └── write JobResult to state

Commit
  └── all output CIDs are canonical state
```

### 5.2 Execution Backends

Priority order per validator:

1. **Vulkan** (Linux, Windows) — preferred for performance
2. **Metal** (macOS) — preferred on Apple hardware
3. **DX12** (Windows) — fallback
4. **OpenGL/GLES** via wgpu — fallback
5. **llvmpipe (Mesa software rasterizer)** — **canonical reference, always available**

Validators without GPU hardware run llvmpipe. For the integer-only instruction set, all backends produce bit-identical results. This must be verified in the testnet phase with a cross-backend conformance test suite.

### 5.3 Buffer Layout Contract

All buffers must use explicit layout annotations. No implicit padding allowed:

```wgsl
struct InputData {
  @size(4) count: u32,
  @align(16) data: array<vec4<f32>>,
}
```

The VM enforces that uploaded shaders declare `@size` and `@align` on all struct members. This ensures identical memory interpretation across all backends and platforms.

Endianness: **little-endian only**. Validators on big-endian hardware must byte-swap (in practice irrelevant — all modern GPU hardware is little-endian).

### 5.4 Input/Output Protocol

**Input:**
- Primary input: single CID resolved to raw bytes, bound at `@group(0) @binding(0)`
- Additional bindings: `MsgGpuExecute.bindings` resolved from CIDs
- Uniform parameters: encoded as CBOR in a separate CID, bound at `@group(0) @binding(1)`

**Output:**
- Single output storage buffer at `@group(1) @binding(0)`
- After execution: raw bytes hashed (SHA-256) → output CID
- Output CID stored in cybergraph
- Cyberlink created: `input_cid → [shader_cid] → output_cid`

This means every GPU computation permanently creates a typed edge in the knowledge graph. The graph encodes the full provenance of every result.

---

## 6. Gas Metering

### 6.1 Two-Layer Model

Gas is computed at two levels that multiply together:

```
total_gas = static_complexity × dispatch_volume + memory_gas
```

### 6.2 Static Analysis → `ShaderProfile`

Computed once at `MsgUploadShader`, stored on-chain:

```protobuf
message ShaderProfile {
  repeated TickProfile ticks        = 1;  // one per logical clock interval
  uint64  cost_per_workgroup        = 2;  // sum of all tick costs
  uint64  max_loop_iterations       = 3;  // product of all loop bounds
  uint64  workgroup_shared_bytes    = 4;
  uint64  max_dispatch_x            = 5;  // declared maximums
  uint64  max_dispatch_y            = 6;
  uint64  max_dispatch_z            = 7;
}

message TickProfile {
  uint32 tick_index           = 1;
  uint64 integer_ops          = 2;
  uint64 atomic_ops           = 3;
  uint64 storage_reads_bytes  = 4;  // static upper bound
  uint64 storage_writes_bytes = 5;
  uint64 barrier_type         = 6;  // workgroup=1, storage=2
}
```

**Op weights (governance parameters, initial values):**

| Operation | Gas Weight |
|-----------|-----------|
| Integer arithmetic | 1 |
| Integer division/mod | 4 |
| Storage buffer read (per 4 bytes) | 10 |
| Storage buffer write (per 4 bytes) | 15 |
| Atomic op | 20 |
| `workgroupBarrier()` | 50 |
| `storageBarrier()` | 100 |

```
cost_per_workgroup = Σ_ticks (
    integer_ops × W_INT +
    atomic_ops × W_ATOMIC +
    storage_reads × W_READ +
    storage_writes × W_WRITE +
    barrier_cost
) × max_loop_iterations
```

Branches are priced at **worst-case path** — the more expensive branch is always counted.

### 6.3 Dispatch Gas

```
dispatch_gas = cost_per_workgroup × dispatch_x × dispatch_y × dispatch_z
```

Checked against `gas_limit` before any GPU work begins. If insufficient → tx fails immediately, no GPU execution, gas_used = validation_cost only.

### 6.4 Memory Gas

Charged separately on buffer size regardless of shader logic:

```
memory_gas = (input_bytes + output_bytes + uniform_bytes) × W_MEMORY_BYTE
```

Initial `W_MEMORY_BYTE` = 1 gas per 32 bytes (governance parameter).

### 6.5 Logical Clock Gas Multiplier

Each additional logical clock tick (barrier) adds overhead because it represents a global synchronization that serializes execution. Ticks are already costed individually in the `TickProfile` — the barrier weight captures this.

For dependent job chains (see Section 7), each dependency hop adds a flat `CLOCK_DEPENDENCY_COST` = 500 gas to account for scheduling overhead.

### 6.6 Block GPU Budget

```
max_gpu_gas_per_block = CANONICAL_THROUGHPUT × BLOCK_TIME_MS × GPU_UTILIZATION_TARGET
```

- `CANONICAL_THROUGHPUT` calibrated against reference hardware at genesis
- `GPU_UTILIZATION_TARGET` = 0.5 (50% of block time for GPU, remainder for Cosmos overhead)
- Governance parameter, adjustable

If queued jobs exceed block budget, they are deferred to next block in FIFO order. Gas is not charged for deferred jobs until execution.

---

## 7. Logical Clocks — Causal Ordering

### 7.1 Job Clock Assignment

Every GPU job is assigned a **logical clock value** at queue time:

```
job.logical_clock = max(
    current_block_clock,
    parent_job.logical_clock + 1  // if parent_clock specified in MsgGpuExecute
)
```

The block's logical clock starts at `last_block_final_clock + 1` and advances monotonically.

### 7.2 Dependency Declaration

A job can declare it depends on a previous job's output:

```protobuf
// in MsgGpuExecute
parent_clock: 42  // this job cannot execute before clock 42 is committed
```

The scheduler builds a DAG of jobs within the block. Independent jobs execute in parallel (batched dispatch). Dependent jobs execute in topological order.

```
clock 1: [job_A, job_B, job_C]  → dispatch all in parallel
clock 2: [job_D(depends on A)]  → dispatch after clock 1 committed
clock 2: [job_E(depends on B)]  → dispatch in same batch as job_D
clock 3: [job_F(depends on D,E)] → dispatch after clock 2 committed
```

### 7.3 Cross-Block Dependencies

A job in block N+1 can depend on a job from block N by specifying its `logical_clock`. The scheduler verifies the referenced clock is committed in state before executing the dependent job.

This enables **GPU computation pipelines** across multiple blocks — multi-stage ML inference, iterative refinement, multi-pass graph algorithms — without any special protocol, just logical clock references.

### 7.4 Clock Commitment

At `EndBlock`, the final logical clock value of the block is committed to state:

```
GpuBlockState {
  block_height:    uint64
  final_clock:     uint64
  job_results:     map<job_id, JobResult>
  clock_to_jobs:   map<uint64, []job_id>
}
```

### 7.5 Causal Provenance in the Knowledge Graph

Every cyberlink created by a GPU job carries its logical clock:

```
(input_cid) --[shader_cid @ clock=42]--> (output_cid)
```

The clock value is part of the edge metadata. Queries can reconstruct the full causal history of any CID — which computations produced it, in what order, from what inputs.

---

## 8. Determinism Guarantees

### 8.1 Requirements

A GPU VM execution is deterministic if and only if:
1. Shader uses only allowed instruction subset (Section 4)
2. Buffer layout is fully specified with explicit alignment (Section 5.3)
3. Dispatch dimensions are identical across validators
4. Input CIDs resolve to identical bytes (guaranteed by content addressing)

### 8.2 Conformance Test Suite

Before mainnet, the following must pass across all supported backends:

- Integer arithmetic correctness vs. reference CPU implementation

- Atomic operation ordering under concurrent access
- Barrier synchronization correctness
- Memory layout alignment identity
- Known convergent algorithms (rank) produce identical final vectors

Test runner executes each test shader on Vulkan, Metal, DX12, GL, and llvmpipe and diffs outputs. All must match.

---

## 9. Validator Requirements

### 9.1 Hardware

| Tier | Hardware | Role |
|------|----------|------|
| Full GPU | Vulkan-capable GPU, ≥8GB VRAM | Full performance, preferred |
| Minimal GPU | Any Vulkan-capable GPU | Adequate for current workloads |
| CPU-only | Any x86_64 with Mesa/llvmpipe | Canonical reference, slower |

CPU-only validators participate fully in consensus. They may miss block time budgets for large dispatches — governance sets block GPU budget conservatively enough that llvmpipe validators can keep up.

### 9.2 Software

- `wgpu` (Rust) embedded in node binary
- Mesa/llvmpipe available on all platforms as fallback
- `naga` for shader validation at upload time
- No CUDA/ROCm dependencies — wgpu abstracts all backends

---

## 10. Integration with CosmWasm

CosmWasm contracts can submit GPU jobs and read results:

```rust
// Submit a GPU job from CosmWasm
let msg = GpuExecuteMsg {
    shader_cid: "Qm...",
    input_cid: "Qm...",
    dispatch: [64, 64, 1],
    gas_limit: 1_000_000u64,
    parent_clock: None,
};
let response = deps.querier.query_gpu_execute(msg)?;
let job_id = response.job_id;

// Read result in next block (or same block if after EndBlock)
let result = deps.querier.query_gpu_result(job_id)?;
match result.status {
    JobStatus::Complete => {
        let output_cid = result.output_cid;
        // use output_cid to fetch data or pass to next shader
    }
    JobStatus::Pending => { /* retry next block */ }
}
```

The GPU VM is a **host capability** from CosmWasm's perspective — similar to how CosmWasm calls the bank module. GPU compute becomes composable with any smart contract logic.

---

## 11. Cybergraph Integration

Every GPU interaction creates permanent knowledge graph structure:

```
Shader upload:
  (author_neuron) --[upload]--> (shader_cid)
  (shader_cid) --[implements]--> (algorithm_cid)

Job execution:
  (input_cid) --[shader_cid @ block=N clock=42]--> (output_cid)
  (neuron) --[executed]--> (job_id_cid)

Rank computation:
  (cybergraph_state_cid) --[rank_shader_cid]--> (rank_vector_cid)
```

The rank shader becomes just another shader in the registry. Its periodic execution is triggered by a native module at epoch boundaries — but the mechanism is identical to user-submitted GPU jobs. Rank is the first app, not a special case.

---

## 12. Roadmap

### Phase 1 — Prove Consensus (weeks)
- Native `x/gpu` module skeleton
- Single hardcoded rank shader, epoch-triggered
- llvmpipe + one GPU backend
- Conformance test across validator set
- Verify integer determinism empirically across all backends

### Phase 2 — Shader Upload (months)
- `MsgUploadShader` with `naga` validation
- Static analysis → `ShaderProfile`
- Static gas model
- Shader registry in state

### Phase 3 — Full VM (months)
- `MsgGpuExecute` with dynamic dispatch
- Job queue + EndBlock execution
- Logical clock assignment and dependency resolution
- Output CID commitment to cybergraph
- Block GPU budget enforcement

### Phase 4 — Composability (months)
- CosmWasm ↔ GPU call interface
- Multi-binding input protocol
- Cross-block pipeline support

### Phase 5 — Hardening
- Full conformance test suite across all backends
- Governance parameters for all weights
- Slashing for non-deterministic results (after Phase 1 proves stability)
- Performance optimization, parallel job batching

---

## 13. Open Questions

1. **Fixed-point precision** — what scale factor is standard? 10^6 (microsats-style) or 10^9? Should the VM enforce a canonical scale factor or leave it to shader metadata?

2. **Shader upgrade mechanism** — CIDs are immutable. How do you version a shader? Governance vote to deprecate old CID and point canonical alias to new one?

3. **Output buffer size limits** — maximum output CID size? Needs to be bounded to prevent state bloat. Governance parameter.

4. **Incentive for GPU validators** — should GPU validators earn higher rewards? They bear higher hardware cost. Fee distribution mechanism TBD.

5. **Shader composability** — can a shader call another shader (like contract-to-contract calls in CosmWasm)? Complex to implement but powerful. Deferred to Phase 5+.

6. **ZK hybrid** — for shaders that are too slow for CPU fallback validators, can a ZK proof replace execution for non-GPU nodes? Optional upgrade path that doesn't require all validators to have GPUs.

---

*This document is a living spec. Implementation findings in Phase 1 will revise sections 4, 6, and 8.*