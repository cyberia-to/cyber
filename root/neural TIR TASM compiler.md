---
tags: cyber, trident, neural
alias: neural-tir-tasm-compiler
crystal-type: entity
crystal-domain: cyber
type: research engineering task
domain: neural program synthesis / compiler optimization
stack: Rust, burn (wgpu backend), rayon, Triton VM
priority: high — critical path for cyb agent runtime proof performance
version: 2.0 (incorporates architecture review)
---

# Neural Compiler: TIR → TASM (Triton VM Assembly)

---

## Why This Works

The model is small because the problem is narrow. The problem is narrow because the execution oracle is fast. The execution oracle being fast is what makes learned compilation viable here.

Specifically: TASM stack semantics are fully decidable — every candidate can be validated in milliseconds, not seconds. This turns program synthesis from an open-ended [[search]] problem into a generate-and-filter loop where the filter is cheap, authoritative, and binary. A small specialized model with strong inductive bias outperforms a large general model because the search space is algebraically constrained, not linguistically open.

---

## 1. Problem Statement

The [[trident]] compiler produces valid but unoptimized TASM from TIR (Trident Intermediate Representation). Proof time in Triton VM scales directly with clock cycle count. Formal [[optimization]] of TASM is NP-hard in the general case — the compiler is necessarily conservative.

Train a small [[neural]] model that generates TASM from TIR with fewer proof cycles than the Trident compiler output, while remaining fully valid under Triton VM execution and proof [[verification]]. The model must be trainable and runnable entirely on local hardware (CPU + Apple Silicon / discrete GPU via wgpu, no external APIs).

This is a program synthesis task with a verifiable execution oracle — not code generation in the LLM sense. Correctness is binary and fast to check. The search space is highly constrained by TASM stack semantics. These properties make a small, specialized architecture vastly more appropriate than a fine-tuned general-purpose LLM.

The model targets function patterns present in the [[cyb]] agent runtime — kernel functions, Poseidon2/[[Goldilocks field]] arithmetic, reduction operations, [[hash]] computations. Out-of-distribution generalization is not a goal for v1.

---

## 2. Inputs, Outputs, and Data Strategy

### 2.1 Training Data (Starting Point)

| Source | Description | Count |
|--------|-------------|-------|
| Hand-compiled | Manually written TASM, passes validation | 30–50 functions |
| Compiler output | Trident-generated TASM, passes validation | 30–50 functions |

Both sources provide `(TIR, TASM)` pairs where TASM is known-valid. Compiler pairs provide a correctness baseline; hand-compiled pairs demonstrate more optimal patterns.

### 2.2 Data Strategy and Training Phases

The 50 seed pairs are not sufficient for optimization. They are sufficient for correctness. The system operates in two distinct phases with different objectives:

Phase A — Correctness Mode (seed data only, Stages 1–2 in §4)

The model learns to generate *valid* TASM. Optimization against the compiler baseline is a secondary signal, not the primary goal. With 5 holdout examples from 50 seeds, numeric metrics are indicative only — do not treat them as gates. The gate for Phase A completion is: validity rate ≥ 80% on holdout, measured with bootstrap CI over 1000 resamples.

Phase B — Optimization Mode (triggered after ≥ 100 online build results accumulated)

With enough real production data in the replay buffer, the model has seen diverse function patterns and has a distribution of `(valid, proof_cycles)` outcomes to learn from. Optimization metrics become meaningful. The targets in §8 apply to Phase B.

Every design decision about replay buffer, online learning, and reward shaping serves one goal — reaching Phase B as quickly as possible. The transition from Phase A to Phase B is the critical milestone for this project, not initial training loss.

### 2.3 TIR Representation

TIR is the existing Trident IR type. The [[graph neural network]] encoder expects it as:

```rust
pub struct TirGraph {
    pub nodes: Vec<TirNode>,          // indexed 0..N
    pub edges: Vec<(usize, usize, EdgeKind)>,  // (src, dst, kind)
}

pub struct TirNode {
    pub op: OpKind,                   // operation type — maps to node feature
    pub field_type: FieldType,        // BFE | XFE | UNKNOWN
    pub immediate: Option<u64>,       // constant value if any
}

pub enum EdgeKind {
    DataDep,     // value flows from src to dst
    ControlFlow, // control dependency
    MemOrder,    // memory ordering constraint
}
```

Node feature vector (input to GNN): `[op_onehot (|OpKind| dims) ‖ field_type_onehot (3 dims) ‖ has_immediate (1 dim) ‖ immediate_normalized (1 dim)]`. Total: `|OpKind| + 5` dimensions. Edge type is encoded as a learned embedding (3 types × d_edge dims) passed to GATv2 [[attention]].

If `TirGraph` does not yet exist as a standalone type in Trident, define it in this crate and provide a conversion function `fn from_trident_ast(ast: &TridentAst) -> TirGraph`.

### 2.4 Model Output

TASM — a sequence of Triton VM instructions operating on a typed stack. Valid output must satisfy:

- Stack depth invariants at every instruction (depth ≥ pop arity, depth + push arity ≤ MAX\_STACK)
- Type compatibility (BFE vs XFE) at every pop/push
- Correct function calling convention
- Termination with expected stack state

### 2.5 Acceptance Criterion

```rust
/// Returns true if generated TASM is acceptable as a replacement for baseline.
/// `baseline` is the Trident compiler output for the same TIR.
fn accept(tasm: &[Instruction], baseline: &[Instruction]) -> bool {
    let valid = triton_vm::execute(tasm).is_ok(); // simulation, not full proof
    if !valid { return false; }
    let cycles = triton_vm::clock_cycles(tasm);   // from simulation
    let baseline_cycles = triton_vm::clock_cycles(baseline);
    cycles <= baseline_cycles  // <=, not <: equal is acceptable, not a regression
}
```

Important distinction: `triton_vm::execute` is a simulation/dry-run (milliseconds). `triton_vm::prove` is a full stark proof (seconds). Acceptance checking uses simulation only. Full proof is never called during model training or beam search ranking. It is called by the downstream consumer of the compiled output, not by this system.

---

## 3. Architecture

### 3.1 Overview

```
TIR Graph (TirGraph struct)
    │
    ▼
GNN Encoder — see §3.2 for CPU/GPU split
    │  node embeddings (N × d) + global context (d)
    ▼
Stack-Aware Transformer Decoder (6 layers, 8 heads, d=256)
    │  + stack_depth_emb + stack_type_emb injected at each step
    ▼
Logit projection (d → vocab_size)
    │
    ▼
Grammar Mask (WGSL compute shader — GPU-side, no CPU sync)
    │  -inf on invalid instructions, 0.0 on valid
    ▼
Beam Search (K=32, GPU-resident)
    │
    ▼  [first CPU↔GPU sync after full decode]
Parallel Triton VM simulation (K candidates, CPU/rayon)
    │
    ▼
Rank by clock_cycles → return best
If all K invalid → fallback to compiler output (see §3.5)
```

Parameter budget: ~10–15M total (encoder ~3M, decoder ~10M). ~40MB fp32, ~10MB Q8.

### 3.2 GNN Encoder

Build adjacency representation from `TirGraph`. Each node's feature vector is defined in §2.3. Run 3–4 rounds of GATv2 message passing — GATv2 is preferred over GraphSAGE because edge types (DataDep vs ControlFlow vs MemOrder) carry semantic meaning that attention should weight differently.

Pool to global context via concatenation of mean-pool and max-pool over all node embeddings: `global = [mean(nodes) ‖ max(nodes)]`, projected to `d`.

GNN implementation scope in `burn`: This is a 2–4 week sub-task. `burn` does not have native GNN support. Required custom ops:

- `scatter_add` over variable-degree neighbours (for mean aggregation)
- Edge-conditioned attention logits (GATv2: `a^T · LeakyReLU(W·[h_i ‖ h_j ‖ e_ij])`)
- Softmax over irregular neighbourhood sizes

Implement as a standalone `gnn` module with `burn` tensor primitives. If this scope is prohibitive for v1, fall back to GraphSAGE with mean aggregation — simpler scatter_add, no edge attention, ~1 week. GraphSAGE will be weaker on heterogeneous edge types but functional.

CPU vs GPU split for GNN:

| Context | Hardware | Reason |
|---------|----------|--------|
| Training, batch ≥ 16 | GPU | Pack all graphs into one large disconnected graph; scatter ops amortized over batch |
| Inference, single function | CPU/NEON | GPU command submission overhead (~5–20µs) exceeds matmul time for 50–200 node graphs |
| Crossover point | ~8 graphs | Measure empirically on target hardware — see Open Questions §9.2 |

For batched training: construct a single large graph by offsetting node indices. `edges_batched[i] = (src + offset[b], dst + offset[b], kind)`. This is the standard PyG/DGL batching trick; implement identically in `burn`.

### 3.3 Stack-Aware Decoder

Standard autoregressive Transformer decoder with two modifications at the input embedding layer:

Stack depth embedding: Integer `d ∈ [0, MAX_STACK]` encoded as a learned vector of size `d_stack=32`. Concatenated with token embedding: `input_t = [token_emb ‖ depth_emb(stack_depth_t)]`. Gives the model direct access to current stack depth without inferring it from sequence history.

Stack type embedding: Fixed-width encoding of the top-`W` type slots (W=8 sufficient for most TASM patterns). Each slot: BFE=0, XFE=1, EMPTY=2 → one-hot (3 dims). Flattened to `3W=24` dims, passed through a small linear projection to `d_type=32`, added to input embedding. Tracks the type state the model should respect.

Both embeddings are updated by the grammar state machine (§3.4) after each token is sampled.

The decoder attends to GNN node embeddings via cross-attention — each decoder step can attend to relevant TIR nodes, not just the global context vector. This is the primary mechanism by which the decoder "reads" the input program.

### 3.4 Grammar Masking — Critical Path, GPU-Resident

TASM stack semantics are fully decidable at each step in O(1): given `(depth, types[MAX_STACK])`, valid next instructions form a computable set. This mask must run on GPU to avoid per-token CPU↔GPU synchronization.

For a 200-instruction function with K=32 beams: 200 sequential mask applications. On PCIe GPU, CPU↔GPU sync costs ~10µs × 200 = 2ms overhead minimum, before any compute. The WGSL shader eliminates this entirely — state update and masking happen in the same GPU pass as logit computation.

WGSL shader (`shaders/grammar_mask.wgsl`):

```wgsl
// Constants baked at shader compile time
const VOCAB_SIZE: u32 = 120u;    // actual TASM instruction count
const MAX_STACK: u32 = 64u;      // Triton VM stack limit — verify against spec
const NEG_INF: f32 = -1e9;

struct StackState {
    depth: i32,
    types: array<u32, 64>,  // 0=BFE, 1=XFE, 2=EMPTY; MAX_STACK entries
}

// Per-instruction static tables (baked from TASM spec at init time)
@group(0) @binding(0) var<storage, read> pop_arity:  array<i32,  120>;
@group(0) @binding(1) var<storage, read> push_arity: array<i32,  120>;
@group(0) @binding(2) var<storage, read> input_types: array<u32, 240>; // [instr][slot]
@group(0) @binding(3) var<storage, read> output_types: array<u32, 240>;

// Per-step mutable state
@group(1) @binding(0) var<storage, read_write> stack_states: array<StackState>;
@group(1) @binding(1) var<storage, read>       sampled_tokens: array<u32>;
@group(1) @binding(2) var<storage, read_write> valid_masks: array<f32>; // [beam × vocab]

@compute @workgroup_size(32)  // one thread per beam; K=32 fits one workgroup
fn update_and_mask(@builtin(global_invocation_id) gid: vec3<u32>) {
    let beam = gid.x;
    if beam >= K { return; }

    var state = stack_states[beam];
    let tok = sampled_tokens[beam];

    // --- 1. Update stack state from previous token ---
    let n_pop  = pop_arity[tok];
    let n_push = push_arity[tok];

    // Shift stack down by n_pop, then push n_push new types
    for (var i = 0i; i < MAX_STACK - n_push; i++) {
        state.types[i] = state.types[i + u32(n_pop)];
    }
    for (var i = 0u; i < u32(n_push); i++) {
        state.types[u32(MAX_STACK) - 1u - i] = output_types[tok * 2u + i];
    }
    state.depth = state.depth - n_pop + n_push;
    stack_states[beam] = state;

    // --- 2. Compute valid mask for next step ---
    let base = beam * VOCAB_SIZE;
    for (var instr = 0u; instr < VOCAB_SIZE; instr++) {
        let req_pop  = pop_arity[instr];
        let req_push = push_arity[instr];

        let depth_ok = (state.depth >= req_pop)
                    && (state.depth - req_pop + req_push <= i32(MAX_STACK));

        var type_ok = true;
        for (var s = 0u; s < u32(req_pop); s++) {
            let expected = input_types[instr * 2u + s];
            let actual   = state.types[u32(state.depth) - 1u - s];
            if expected != 2u && expected != actual { // 2=ANY
                type_ok = false;
            }
        }

        valid_masks[base + instr] = select(NEG_INF, 0.0, depth_ok && type_ok);
    }
}
```

Implementation note: Verify `MAX_STACK` against the actual Triton VM specification before baking into the shader. Memory per workgroup: 32 beams × 64 stack slots × 4 bytes = 8KB — within wgpu limits.

During training (teacher forcing): Ground truth tokens are fed directly, no sequential state machine execution. Apply grammar mask as a logit penalty over the full sequence in a single forward pass: `masked_logits = logits + precomputed_mask_sequence`, where mask sequence is computed CPU-side from ground truth tokens before the forward pass. No GPU-side state machine needed during training.

### 3.5 Fallback Policy

If all K=32 beam candidates fail `triton_vm::execute`:

1. Log the failure with the TIR hash for offline analysis
2. Return the Trident compiler output unchanged — the system is a transparent optimizer, never a blocker
3. Record as a `BuildResult` with `valid=false` and `proof_cycles=None` — enters replay buffer with zero reward, informs future training

This fallback must be unconditional. The model is never in the critical path for correctness — only for performance.

---

## 4. Training Pipeline

### 4.1 Data Augmentation — Real Augmentations Only

With 50 seed examples, augmentation is structural, not cosmetic. SSA variable renaming does not augment the GNN — the encoder operates on operation types and [[graph]] structure, not variable names. All renaming produces identical node features and adjacency matrices.

Effective augmentations (apply to TIR graph level):

Structural augmentations (change graph topology):
- Reorder independent operations within basic blocks — topological sort has many valid linearizations; each is a distinct training sample
- Inline a leaf call site (replace call node with callee's subgraph) — preserves semantics, increases graph size
- Dead code insertion: add a computation that contributes nothing to the output — model must learn to ignore it; the augmented TASM must similarly discard it

Output-space augmentations (change TASM without changing TIR):
- For each compiler-output TASM: apply local random walk — randomly swap adjacent independent instructions, or substitute equivalent instruction sequences (e.g. `push 0; add` → `nop` where applicable). If result passes `triton_vm::execute`, it is a new valid training target for the same TIR.
- This is the most valuable augmentation: it directly expands the model's knowledge of the TASM output space.

Coverage measurement: After augmentation, cluster TASM sequences by edit distance (or instruction n-gram overlap). Track cluster [[entropy]]. If augmented dataset clusters tightly (low entropy), the augmentations are not diverse enough — invest more in local random walk.

Target: 50 seeds → ~5,000–10,000 pairs for Stage 1. The 50,000 figure from v1 was aspirational; start with 5,000 and measure training loss saturation.

### 4.2 Stage 1: Supervised Pre-training (Correctness)

Objective: Learn to generate valid TASM. Do not optimize for cycle count yet — the model has seen too few patterns to generalize optimization.

Standard cross-entropy with teacher forcing. Loss masked to exclude padding. Grammar mask applied as logit penalty before softmax — keeps training and inference behaviour consistent.

```rust
// Training step pseudo-code
let logits = model.forward(tir_graph, target_tasm[..t]);      // (T, vocab)
let masks  = precompute_grammar_masks(target_tasm);            // CPU, before loop
let masked = logits + masks;                                    // broadcast add
let loss   = cross_entropy(masked, target_tasm[1..]);
```

Optimizer: AdamW, lr=3e-4, cosine decay to 1e-5, weight decay=0.01, batch size=32. Gradient clip at 1.0. Train until validation loss on holdout set plateaus for 3 consecutive epochs.

Phase A completion gate: Validity rate ≥ 80% on holdout, measured with 1000-resample bootstrap to get confidence interval. With 5 holdout examples, single-point estimates are meaningless — the CI is what matters (e.g. "75%–95% at 90% CI" is a passing result; "40%–90% at 90% CI" is not).

### 4.3 Stage 2: GFlowNets — Diversity-Preserving Optimization

Why not PPO or REINFORCE: Policy gradient methods for discrete sequence generation collapse to the first valid solution found. For TASM, many valid sequences exist per TIR with varying cycle counts. GFlowNets train a policy proportional to reward — they maintain diversity and continue discovering lower-cycle variants.

Reward definition:

```
R(tasm) = ε                                                    if !valid
         = 1 + max(0, (compiler_cycles - model_cycles) / compiler_cycles)  if valid
```

where `ε = 1e-3`. This ε is not optional. TB loss computes `log R(x)`. If R=0, the loss is undefined (log(0) = -∞), gradients become NaN, and training crashes. Always clip reward from below.

Trajectory Balance (TB) loss:

```
L_TB = ( log Z + log P_F(τ) - log P_B(τ) - log R(x) )²
```

where:
- `Z` — learned scalar (log-partition function), initialized to 0, trained jointly
- `P_F(τ)` — forward policy probability of the generated sequence (sum of log-probs from decoder)
- `P_B(τ)` — backward policy, uniform over valid completions (set to a fixed small constant; can be learned later)
- `R(x)` — clipped reward as above

Reward sparsity problem and mitigation: In early Stage 2, the model from Stage 1 may produce 50–80% valid sequences — manageable. But if validity drops (e.g. fine-tuning degraded Stage 1 behaviour), most rollouts get R=ε and gradients provide no learning signal about *what makes sequences valid*.

Mitigation — partial credit reward shaping:

```
R_shaped(tasm, k) = ε + (k / total_length) × validity_bonus
```

where `k` is the step at which the first stack violation occurs (or `total_length` if fully valid). This provides gradients even for invalid sequences, guiding the model toward sequences that fail later (i.e., are more nearly valid).

Apply reward shaping only in early Stage 2 (first 1000 steps). Transition to `R(tasm)` pure reward once validity rate reaches 70%.

Implementation: TB loss is ~30 lines of tensor arithmetic on top of `burn`. No external GFlowNet library needed. The core computation:

```rust
fn tb_loss(
    log_pf: Tensor<B, 1>,   // sum of log P(token_t | history) over trajectory
    log_pb: f32,             // log P_B, uniform constant
    log_r:  f32,             // log R(x), clipped
    log_z:  Tensor<B, 0>,   // learned scalar
) -> Tensor<B, 0> {
    let residual = log_z + log_pf - log_pb - log_r;
    residual.powf_scalar(2.0).mean()
}
```

[[temperature]] annealing: Start with temperature τ=2.0 for diverse [[sampling]]. Decay to τ=0.5 over Stage 2. High temperature early → model explores many valid sequences. Low temperature late → model concentrates on good ones.

### 4.4 Stage 3: Online Learning — The Path to Phase B

Every invocation of the model in production generates a `BuildResult`:

```rust
pub struct BuildResult {
    pub tir_hash: [u8; 32],                 // Poseidon2 CID of the TIR input
    pub generated_tasm: Vec<Instruction>,
    pub valid: bool,
    pub clock_cycles: Option<u64>,          // from triton_vm::clock_cycles (simulation)
    pub compiler_cycles: u64,               // always computed; baseline reference
    pub fallback_used: bool,                // true if all K beams were invalid
    pub timestamp: u64,
    pub model_version: u32,                 // checkpoint ID that generated this result
}
```

Prioritized Experience Replay buffer: Priority = reward magnitude. High-reward results (big cycle reduction) are sampled more frequently. Maintains a sliding window of the last 10,000 results; older results expire.

Micro-finetune trigger: When buffer contains ≥ 50 new results since last update (or 24 hours elapsed, whichever comes first), run a GFlowNet micro-update: 200 gradient steps on the new batch + a 10% sample from historical buffer (prevents forgetting). Update takes ~2 minutes on M-series GPU.

Regression guard (see §8): Before committing the updated checkpoint, run the full evaluation set. If validity rate drops > 2pp from previous checkpoint, discard the update and log the anomaly. Keep previous checkpoint active.

Phase B activation: When the replay buffer contains ≥ 100 results with `valid=true && !fallback_used`, Phase B begins. From this point, evaluation metrics in §8 are binding (not indicative).

---

## 5. Compute Allocation: CPU vs GPU

This split is non-obvious. Do not deviate without measurement.

### Assignment Table

| Component | Hardware | Reason |
|-----------|----------|--------|
| TIR parsing, TirGraph construction | CPU | Symbolic, sequential, no vectorization |
| GNN encoder — inference (single function) | CPU/NEON | GPU submission overhead > matmul for ≤200 nodes; crossover at ~8 graphs |
| GNN encoder — training (batched) | GPU | Batched sparse matmul amortizes overhead; profitable at batch ≥ 16 |
| Transformer decoder forward | GPU | Dense matmul at d=256; profitable at any K |
| Grammar mask shader | GPU | Must co-locate with decoder to eliminate sync |
| Top-K beam selection | GPU | argsort over K×vocab tensor |
| GFlowNet TB loss + backward | GPU | Standard autodiff, no special requirements |
| AdamW optimizer state | GPU | Keep in GPU memory; avoid transfer |
| Triton VM simulation (K beams) | CPU/rayon | Dynamic control flow, not vectorizable; parallelism is across independent beams |
| Replay buffer management | CPU/RAM | Random access, priority queue operations |
| Build result logging | CPU | I/O bound |

### Synchronization Points (Inference)

```
CPU: parse TIR → TirGraph
CPU: GNN encoder → latent (N×d, global d) tensor
  │
  └─── [transfer to GPU — zero-copy on Apple Silicon; explicit on discrete] ───►
                                                                            GPU: decoder loop
                                                                            GPU: K TASM sequences
  ◄─── [transfer to CPU — copy K×T token sequences] ──────────────────────
CPU: triton_vm::execute × K (rayon::par_iter)
CPU: rank by clock_cycles
CPU: return best (or fallback)
```

Two synchronization points total per inference call. No per-token sync.

### Latency Estimate (Apple M-series, K=32, 200-instruction function)

These are estimates based on known M-series throughput. Validate with `benches/end_to_end.rs` before treating as ground truth.

| Component | Estimated time | Location | Notes |
|-----------|---------------|----------|-------|
| TIR parsing + graph build | ~0.2 ms | CPU | Depends on TirGraph conversion complexity |
| GNN encoder (100-node graph) | ~0.5 ms | CPU/NEON | GraphSAGE; GATv2 ~1.5ms |
| Decoder (200 steps, K=32) | ~40–80 ms | GPU | 200 sequential dispatch; bulk in matmul |
| Grammar shader (per step) | ~0.05 ms | GPU | Included in decoder estimate above |
| GPU→CPU transfer (K sequences) | ~0.1 ms | PCIe / unified | Negligible on Apple Silicon |
| Triton VM × 32 candidates | ~10–40 ms | CPU/rayon | Dominant variable; depends on function complexity |
| Ranking | ~0.1 ms | CPU | |
| Total | ~50–120 ms | mixed | Triton VM is dominant bottleneck |

Decoder latency note: 200 sequential GPU dispatches is the binding constraint, not FLOPs. Each dispatch submits one grammar shader + one decoder step. On Metal, command buffer submit latency is ~5µs → 200 × 5µs = 1ms overhead, acceptable. On Vulkan/DX12, measure independently — may differ.

---

## 6. Rust Crate Dependencies

```toml
[dependencies]
# Neural network — training and inference
burn = { version = "0.15", features = ["wgpu", "autodiff"] }

# Parallel CPU workloads (Triton VM beam execution)
rayon = "1.10"

# Triton VM — simulation, clock_cycles, validation
# Use simulation API only; full prove() is not called by this crate
triton-vm = { path = "../triton-vm" }

# Graph data structures for TirGraph construction
petgraph = "0.6"

# Serialization: replay buffer, checkpoints, TirGraph on disk
serde = { version = "1", features = ["derive"] }
bincode = "2"

# Statistics for bootstrap CI in evaluation (Phase A gate)
statrs = "0.17"

[dev-dependencies]
criterion = { version = "0.5", features = ["async_tokio"] }
```

No Python. No PyTorch. No C FFI. Training, inference, validation, and online learning run in the same Rust binary as the cyb runtime. The compiled model is callable from [[rune]] executable [[particles]] via the standard `ctx` API.

Quantization for deployment (future): Once the model is stable, quantize to Q8 with `burn`'s built-in quantization. At ~10MB, the model fits in L2 cache on M-series; inference latency improves ~2× with Q8 on CPU path. Do not implement in v1.

---

## 7. Repository Layout

```
neural-compiler/
├── src/
│   ├── lib.rs                      # public API: compile(tir) -> Result<Tasm>
│   ├── model/
│   │   ├── encoder.rs              # GNN over TirGraph; CPU and batched-GPU paths
│   │   ├── decoder.rs              # Stack-aware Transformer; cross-attn to encoder
│   │   ├── grammar.rs              # Stack state machine + WGSL shader loader
│   │   └── vocab.rs                # TASM instruction set ↔ token index mapping
│   ├── training/
│   │   ├── augment.rs              # Structural TIR augmentations (see §4.1)
│   │   ├── supervised.rs           # Stage 1: CE loss + teacher forcing
│   │   ├── gflownet.rs             # Stage 2: TB loss, temperature annealing
│   │   └── online.rs               # Stage 3: PER buffer, micro-finetune, regression guard
│   ├── inference/
│   │   ├── beam.rs                 # Beam search coordinator; fallback logic
│   │   └── execute.rs              # triton_vm::execute × K via rayon
│   └── data/
│       ├── tir_graph.rs            # TirGraph type + conversion from Trident AST
│       ├── pairs.rs                # (TirGraph, TASM) pair loading + validation
│       └── replay.rs               # Prioritized experience replay buffer
├── shaders/
│   └── grammar_mask.wgsl           # Stack state machine + mask (see §3.4)
├── data/
│   ├── seed/                       # 50 seed pairs as bincode; committed to repo
│   └── augmented/                  # Generated by augment.rs; gitignored
├── checkpoints/                    # Model weights + optimizer state; gitignored
│   ├── stage1_best.bin
│   ├── stage2_latest.bin
│   └── production.bin              # symlink to current production checkpoint
└── benches/
    └── end_to_end.rs               # Criterion: model latency vs compiler; validates §5 estimates
```

---

## 8. Validation and Acceptance Criteria

### Per-function Acceptance (Production)

```rust
fn accept(tasm: &[Instruction], baseline: &[Instruction]) -> bool {
    triton_vm::execute(tasm).is_ok()
        && triton_vm::clock_cycles(tasm) <= triton_vm::clock_cycles(baseline)
}
```

Note `<=` (not `<`): equal cycle count is not a regression. Only improvement or neutral outcomes are accepted. Strictly worse output always falls back to compiler.

### Phase A Metrics (Correctness Mode)

Measured on 5-example holdout with 1000-resample bootstrap:

| Metric | Gate | Notes |
|--------|------|-------|
| Validity rate (90% CI lower bound) | ≥ 70% | Single-point estimate is noise at N=5 |
| Training loss plateau | < 0.5 nats | Indicates model has learned basic TASM grammar |

### Phase B Metrics (Optimization Mode — applies after ≥ 100 online results)

Measured on evaluation set (holdout + online valid results not used in training):

| Metric | Target | Notes |
|--------|--------|-------|
| Validity rate | ≥ 95% | Bootstrapped; report CI |
| Improvement rate | ≥ 60% of valid outputs beat compiler | On cycle count |
| Median cycle reduction | ≥ 10% vs compiler baseline | |
| P90 inference latency | ≤ 200 ms per function | End-to-end including Triton VM |
| Fallback rate | ≤ 5% | Fraction of calls where all K beams invalid |

### Regression Guard (Online Learning)

Before activating any micro-finetuned checkpoint:
1. Run full evaluation set (holdout + labeled online results)
2. Compute validity rate delta vs current production checkpoint
3. If delta < −2pp: discard update, log anomaly, continue with existing checkpoint
4. If delta ≥ −2pp: activate new checkpoint, log metrics

---

## 9. Open Questions (Requiring Measurement Before Finalizing)

These are not design gaps — they are known unknowns with a clear resolution path. Each should be resolved in order before the dependent component is implemented.

Q1 — WGSL shader at TASM vocab size *(resolve before §3.4 implementation)*
The shader iterates over all VOCAB\_SIZE entries per beam per step. Profile: compile and run the shader with K=32, VOCAB\_SIZE=120, 200 steps. Measure total GPU time vs equivalent CPU masking + sync. If GPU is not faster, use chunked CPU masking (every N=8 tokens) as fallback.

Q2 — GNN inference crossover point *(resolve before §3.2 implementation)*
Measure actual GPU command submission latency on target hardware. Run GNN forward pass for graphs of size 10, 50, 100, 200 nodes both on CPU/NEON and GPU. Find crossover. The document assumes ~8 graphs; this may be lower on discrete GPU with PCIe vs Apple Silicon unified memory. Parameterize the threshold as a config value, not a compile-time constant.

Q3 — Triton VM simulation cost distribution *(resolve before latency targets are committed)*
What is the distribution of `triton_vm::clock_cycles` across the 50 seed functions? The latency estimate in §5 assumes 10–40ms for 32 beams. If some functions are cheap (1ms) and others expensive (200ms), the P90 latency target needs to be function-category-specific, not a single number.

Q4 — GATv2 vs GraphSAGE on this task *(resolve before committing to encoder architecture)*
The heterogeneous edge types (DataDep / ControlFlow / MemOrder) suggest GATv2. But with 50 seed functions, the encoder may not have enough data to learn meaningful edge attention weights. Test: train Stage 1 with both architectures, compare validation cross-entropy. If the difference is < 5%, use GraphSAGE (simpler, faster to implement).

Q5 — [[Goldilocks field]] element encoding in node features *(resolve before TirGraph spec is finalized)*
TIR operates over BFE (Goldilocks base field, 64-bit) and XFE (extension field, 3×64-bit). Immediate values in TIR nodes are field elements. The node feature vector includes a `has_immediate` flag and `immediate_normalized` scalar. But XFE immediates are 3 field elements — how to encode them in a single scalar? Options: (a) encode only BFE immediates, mark XFE as `has_immediate=0`; (b) use 3-dimensional immediate feature. Decide before implementing `TirGraph::node_features()`.

---

## 10. What This Is Not

- It is a small specialized compiler optimizer, not a general-purpose code LLM. Do not add Transformer layers to improve "general reasoning." Narrowness is the design.
- It is a candidate generator; `triton_vm::execute` is the verifier. The model proposes candidates; the [[vm]] accepts or rejects. [[formal verification]] lives in the VM.
- It is a transparent post-hoc optimizer for [[trident]]. Trident always runs first; the model tries to improve its output.
- It targets function patterns in the cyb agent runtime. OOD generalization is not a v1 goal.
- It uses simulation only. Full stark [[cryptographic proofs]] are the downstream consumer's responsibility — `triton_vm::prove()` is never called by this system.

---

## 11. Rationale Summary

| Decision | Alternative | Reason |
|----------|-------------|--------|
| Small custom model (~10M params) | Fine-tune CodeLlama/StarCoder | 50 seeds → catastrophic forgetting; no inductive bias for stack machines; general LLMs can't exploit TASM grammar structure |
| GATv2 GNN encoder | MLP on flattened adjacency | Graph structure is the primary inductive bias; MLP loses it; GATv2 handles heterogeneous edge types |
| GFlowNets for RL stage | PPO / REINFORCE | Policy gradient → mode collapse to first valid solution; GFlowNet maintains diversity, finds lower-cycle variants |
| TB loss with ε-clipped reward | Raw binary reward | log(0) = -∞ → NaN gradients; ε=1e-3 is numerically necessary, not optional |
| Grammar masking via WGSL shader | CPU masking with per-token sync | PCIe: 200 syncs × 10µs = 2ms overhead; shader eliminates CPU involvement until decode complete |
| CPU for single-function GNN | GPU for all GNN | Submission overhead > matmul for small graphs; crossover empirically ~8 graphs |
| CPU/rayon for Triton VM | GPU simulation | Dynamic control flow, symbolic state, not SIMD-vectorizable; parallelism is across K independent executions |
| `<=` in acceptance criterion | `<` strict improvement | Strictly optimal model rejects equal solutions; equal cycle count is neutral, not negative |
| Phase A / Phase B split | Single training regime | 50 seed holdout (5 examples) makes optimization metrics statistically meaningless; phases make this explicit |
| `burn` + wgpu | PyTorch / tch-rs | Pure Rust, no C FFI; same binary as cyb runtime; Rune-callable; works on Metal/Vulkan/DX12 without platform SDK |
| Fallback to compiler output | Hard failure on all-invalid | Model is an optimizer, not a gatekeeper; must never block compilation |
| Bootstrap CI for Phase A evaluation | Point estimate | N=5 holdout makes point estimates ±20pp noise; CI is the only meaningful measurement |