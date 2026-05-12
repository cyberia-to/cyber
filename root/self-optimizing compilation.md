---
tags: trident
alias: Self_Optimizing_Compilation_for_Algebraic_Virtual_Machines
crystal-type: article
crystal-domain: cyber
---
# Self-Optimizing Compilation for Algebraic Virtual Machines

## Neural TIR→TASM Optimization with Provable Correctness and Evolutionary Self-Improvement

---

## Abstract

We propose a [[compilers]] architecture for [[trident]] — the deterministic programming language for the nox planetary intelligence substrate — in which a [[neural networks]] optimizer generates Triton VM assembly (TASM) from Trident's typed intermediate representation (TIR). The optimizer is small enough (80K parameters, 640 KB) to reside entirely in L2 cache and train continuously on a single workstation GPU. Correctness is guaranteed not by the model but by a post-hoc stark-based semantic equivalence verifier, making the neural path strictly speculative: it can improve compilation but never break it. The system is self-referential — the optimizer, the verifier, and the training loop are themselves Trident programs compiled by the system they improve — converging to a provable fixed point of [[convergent computation]] where the compiler can no longer improve its own code.

---

## 1. The Problem: Compilation for Proof Machines is Not Compilation for CPUs

Triton [[vm]] is a stack machine with Algebraic Execution Tables (AET). When a program executes, it produces a trace spread across 9 tables: Processor, Op Stack, RAM, Jump Stack, Hash, Cascade, Lookup, U32, and Degree Lowering. The [[zero knowledge proofs]] stark prover must commit to all tables, and all tables are padded to the same power-of-2 height — the height of the tallest table.

This means the cost function for TASM is not cycle count. It is:

$$\text{cost}(S) = 2^{\lceil \log_2(\max_{t \in \mathcal{T}} H_t(S))\rceil}$$

where $H_t(S)$ is the height of table $t$ induced by instruction sequence $S$, and $\mathcal{T}$ is the set of all 9 AETs.

This cost function has two properties that make it fundamentally unlike CPU optimization:

**1. Cliff discontinuity.** A program with maximum table height 1025 has the same proving cost as one with height 2048. Both pad to 2048. A program at height 1024 is 2× cheaper. This means small improvements can yield zero benefit, and tiny reductions can yield 2× speedup.

**2. Cross-table coupling.** Making one table taller affects the cost of all other tables (through padding). A program that trades 100 Processor rows for 50 Hash rows might be cheaper or more expensive depending on which table is currently the tallest. The optimizer must reason about all 9 tables simultaneously.

No existing compiler heuristic is designed for this cost landscape. Register allocators minimize spills. Instruction schedulers minimize pipeline stalls. Nothing minimizes padded-power-of-2-of-maximum-table-height.

---

## 2. Why TIR → TASM and Not Trident → TASM

The compilation pipeline has a natural factorization:

```
Trident source → [parse] → AST → [typecheck] → Typed AST → [normalize] → TIR → [lower] → TASM
```

The first three stages (parse, typecheck, normalize) have unique correct outputs. [[type theory]] inference is not an [[optimization]] problem. Bound propagation has one answer. Normalization is deterministic by construction (required by nox constraint C₂: identical semantics must produce identical hashes).

Only the final stage — lowering TIR to TASM — involves genuine optimization choices: instruction selection, stack scheduling, loop unrolling, table balancing.

Placing the neural optimizer at the TIR→TASM boundary yields a model that:

| Property | TIR → TASM | Trident → TASM |
|---|---|---|
| Parameters | ~80,000 | ~2,450,000 |
| Weight size | 640 KB | 19.6 MB |
| Fits in L2 cache (4 MB) | Yes | No |
| Inference latency (M4 GPU) | ~1–5 μs | ~100–200 μs |
| Training step latency | ~5 μs | ~230 μs |
| Must learn type checking | No | Yes |
| Must learn parsing | No | Yes |
| Failure mode | Suboptimal TASM | Suboptimal or ill-typed TASM |

The 31× size difference is not incidental. It reflects the information-theoretic cost of forcing a neural network to re-derive deterministic transformations that have exact algorithmic solutions. Every parameter spent learning that `Field + Field → Field` is a parameter not spent learning that rearranging stack operations near a power-of-2 boundary saves 50% proving cost.

The TIR representation preserves everything needed for optimization: the data-flow DAG, type annotations, liveness intervals, and loop bounds. It discards everything irrelevant: variable names, whitespace, comments, syntactic sugar. The model sees pure structure with zero noise.

---

## 3. The Optimization Surfaces

### 3.1 Stack Scheduling (Highest Impact)

Triton VM is a stack machine with 16 visible registers (`st0`–`st15`). Binary operations consume `st0` and `st1`. Every computation requires arranging operands at the top of the stack, which costs Processor Table rows.

The stack manipulation instructions are: `pick i`, `place i`, `dup i`, `swap i` (for $0 \leq i < 16$). The optimal scheduling of $n$ operations over $k$ live variables involves searching a space of size $O(16^n)$ — factorial in the effective stack depth, exponential in program length.

Human programmers manage stack scheduling intuitively for 2–3 live variables and fail systematically beyond 5. The tasm-lib repository's README implicitly acknowledges this: *"If you manage to lower any of the numbers by changing a TASM snippet, please make a pull request."* They are asking for exactly what a neural optimizer would provide.

### 3.2 Table Balancing (Unique to ZK Compilation)

Because all tables pad to the same power-of-2, the optimal program balances table heights to avoid one table dragging all others above a cliff boundary.

Consider a program with $H_{\text{proc}} = 900$, $H_{\text{hash}} = 600$, $H_{\text{u32}} = 200$. All tables pad to $2^{10} = 1024$. Now suppose an alternative instruction selection trades 200 Processor rows for 100 Hash rows (by using compound instructions like `sponge_absorb_mem` instead of manual field operations). The new profile is $H_{\text{proc}} = 700$, $H_{\text{hash}} = 700$, $H_{\text{u32}} = 200$. All tables still pad to 1024 — no improvement.

But if the original profile were $H_{\text{proc}} = 1100$, $H_{\text{hash}} = 600$, $H_{\text{u32}} = 200$, the same trade yields $H_{\text{proc}} = 900$, $H_{\text{hash}} = 700$ — dropping from pad-to-2048 to pad-to-1024. A 2× speedup from restructuring alone.

No classical compiler heuristic reasons about this. A neural network trained on the cliff-aware reward signal learns it immediately because the gradient (or evolutionary fitness signal) directly encodes the discontinuity.

### 3.3 Instruction Selection (Moderate Impact)

Triton VM provides compound instructions that perform multiple logical operations in a single cycle: `xx_dot_step` (extension field dot product accumulation), `sponge_absorb_mem` (absorb from RAM without stack loading), `merkle_step_mem` (Merkle traversal from RAM). Each reduces Processor Table rows but may increase other table rows (Hash Table for sponge operations, U32 Table for index arithmetic).

### 3.4 Loop Restructuring (Moderate Impact)

Trident loops have compile-time-known bounds. The compiler chooses between full unrolling (eliminates call/return overhead, increases Processor rows linearly), partial unrolling (reduces overhead, moderate size), and minimal looping via `call`/`recurse`/`recurse_or_return` (adds Jump Stack rows, minimizes Processor rows). The optimal choice depends on current table balance and cliff proximity.

### 3.5 [[hash]] Coprocessor Scheduling (High Impact for nox)

The Hash Table is driven by Tip5 permutation invocations: `hash`, `sponge_absorb`, `sponge_squeeze`, `sponge_absorb_mem`, `merkle_step`, `merkle_step_mem`. Each invocation adds rows to the Hash Table. For nox's dominant workloads — [[merklezation]] inclusion proofs, commitment construction, recursive stark verification — the Hash Table frequently becomes the tallest table and thus the sole determinant of proving cost.

Three optimization dimensions exist within hash scheduling:

**Absorb batching.** `sponge_absorb_mem` absorbs 10 field elements from RAM in a single Tip5 permutation (one Hash Table row). The manual alternative — loading elements to stack via `read_mem` then calling `sponge_absorb` — costs the same one Hash Table row but adds 10+ Processor Table rows for the loads. When Hash Table is not the bottleneck, the manual path may be preferable (it keeps Hash Table shorter at the cost of Processor rows). When Hash Table is the bottleneck, `sponge_absorb_mem` is strictly superior. The optimizer must reason about which table is currently constraining the padded height.

**Sponge lifetime minimization.** Each `sponge_init` resets the sponge state. If a program performs two independent hashing tasks (e.g., computing a commitment and verifying a Merkle proof), the order determines whether one or two `sponge_init` calls are needed. Interleaving the tasks requires two initializations (2 Hash Table rows of overhead plus potential state-saving cost). Sequential execution may allow sponge reuse if the output of the first task feeds into the second. The optimizer must discover task orderings that minimize total sponge initializations.

**Merkle traversal strategy.** `merkle_step` reads siblings from secret input; `merkle_step_mem` reads from RAM. The choice affects RAM Table height versus the cost of providing non-deterministic input. For authentication paths that are reused (e.g., verifiable Merkle tree updates where the same path confirms old leaf inclusion then computes new root), `merkle_step_mem` is required — but the optimizer controls the memory layout of the path, which affects RAM Table row ordering and thus the contiguity argument overhead.

For nox's transfer circuit (~7 Poseidon calls × 300 constraints = 2,100 Hash Table rows in the baseline), hash scheduling optimization directly attacks the dominant cost component. The polynomial commitment approach already described in nox v0.9 — replacing 32 sequential Merkle hashes with ~1,000 field multiplications — is itself a macro-level hash scheduling optimization. The neural optimizer operates at the micro level within each hash-using block, finding instruction orderings and batching strategies that minimize Hash Table growth while respecting data dependencies.

| Optimization | Hash Table Impact | Processor Table Impact | When Beneficial |
|---|---|---|---|
| `sponge_absorb_mem` over manual load + absorb | Same (1 row) | -10 rows | Always (for Processor) |
| Sponge reuse (avoid re-init) | -1 row per avoided init | Variable | When hash-bottlenecked |
| `merkle_step_mem` over `merkle_step` | Same | +RAM Table rows | When path is reused |
| Task reordering for sponge lifetime | -N rows (N = avoided inits) | Variable | Complex programs with multiple hash tasks |

---

## 4. Model Architecture

### 4.1 Input Encoding

A TIR basic block is encoded as a fixed-size tensor. Each node in the data-flow DAG is represented by 4 [[Goldilocks field]] elements:

```
node_encoding = (opcode : 6 bits,     // one of 54 TIR operations
                 type   : 3 bits,     // Field | Bool | U32 | Digest
                 input₀ : 5 bits,     // reference to input node 0
                 input₁ : 5 bits,     // reference to input node 1
                 live_start : 5 bits, // liveness interval start
                 live_end   : 5 bits) // liveness interval end
```

Maximum basic block size: 32 nodes. Input tensor: $32 \times 4 = 128$ field elements, zero-padded for shorter blocks.

For cross-block optimization, the model additionally receives a 16-element context vector encoding the incoming stack state (types and approximate liveness of `st0`–`st15` at block entry).

Total input dimension: 144 field elements.

### 4.2 Output Encoding

The model emits a sequence of TASM instructions, each encoded as 1 field element (7-bit opcode + 4-bit argument). Maximum output length: 64 instructions per basic block.

### 4.3 Network Structure

The model uses a compact encoder-decoder with graph-aware attention:

```
ENCODER (TIR → latent representation):
  DAG-aware self-attention: 2 layers, 2 heads, dim 64
    Q, K, V projections: 3 × 64 × 64 = 12,288 params per layer
    Output projection:   64 × 64      =  4,096 params per layer
    FFN: 64 → 128 → 64               =  16,512 params per layer
    LayerNorm: 2 × 64                =      128 params per layer
    Per layer total:                      33,024 params
    × 2 layers:                           66,048 params

DECODER (latent → TASM sequence):
  Autoregressive MLP:
    Input: 64 (latent) + 64 (prev instruction context) = 128
    Hidden: 128 → 128                 =  16,512 params
    Output: 128 → 64 (instruction)    =   8,256 params
    Total:                               24,768 params

TOTAL:                                   ~91,000 params
                                         ~728 KB
```

The attention mask in the encoder follows TIR DAG edges: node $i$ attends to node $j$ only if $j$ is an input to $i$ or shares a liveness interval. This restricts attention to structurally meaningful relationships rather than requiring the model to learn graph structure from position alone.

### 4.4 All Operations are Tier 0+1

The critical observation: everything the model computes — [[linear algebra]] matrix multiply, attention, layer normalization, activation — is pure [[field]] arithmetic ([[algebra]]).

```
Matrix multiply:  mul + add in bounded loops        → Tier 0+1
Attention scores: mul + add + invert (softmax ≈ 1/x) → Tier 0+1
LayerNorm:        mul + add + invert                  → Tier 0+1
Activation:       mul (polynomial GeLU approximation) → Tier 0+1
```

No hashing. No sponge operations. No extension field arithmetic. The entire model compiles to every Trident target, including GPU via KIR. Inference runs as a single Metal/CUDA kernel dispatch.

---

## 5. Training: Fixed-Point Arithmetic in Goldilocks Field

### 5.1 The Gradient Problem

Gradient descent requires continuous arithmetic. The [[Goldilocks field]] $\mathbb{F}_p$ where $p = 2^{64} - 2^{32} + 1$ is discrete. A learning rate of $0.001$ has no natural field interpretation.

We solve this with scaled fixed-point encoding:

```
ENCODING (scale factor S = 2^16 = 65536):
  Real value  0.375  →  Field element  24,576
  Real value -0.500  →  Field element  (p - 32,768)
  Real value  0.001  →  Field element  66  (≈ 65.536)

MULTIPLY with rescale:
  result = (a × b) × inv(S)    // 3 field operations instead of 1

EFFECTIVE PRECISION: 16 bits
  Sufficient for 91K-parameter models (empirically validated
  by quantization literature: models under 1M params tolerate
  8-bit weights with <1% accuracy loss)
```

Gradient computation follows standard backpropagation, with every operation replaced by its fixed-point field equivalent. A single training step costs approximately:

```
Forward pass:    ~50,000 field operations
Backward pass:   ~100,000 field operations  (2× forward)
Weight update:   ~91,000 field operations   (one mul + add per param)
Total:           ~241,000 field operations

On M4 Pro Metal: ~241K ops at ~50 GOPS = ~5 μs per training step
```

This enables **200,000 training steps per second** on a single Mac. No cluster required.

### 5.2 Training Signal

Each compilation produces a training tuple:

```
(TIR_block, TASM_candidate, score, verified)

where:
  score    = -max_table_height (lower is better)
  verified = semantic_equivalence(TIR_block, TASM_candidate) ∈ {0, 1}
```

The loss function combines two objectives:

$$\mathcal{L} = \mathcal{L}_{\text{valid}} + \lambda \cdot \mathcal{L}_{\text{score}}$$

where $\mathcal{L}_{\text{valid}}$ penalizes semantically incorrect outputs (verified = 0), and $\mathcal{L}_{\text{score}}$ rewards lower table heights for correct outputs. In practice $\lambda$ starts high (prioritize correctness) and anneals toward balanced (optimize once correctness is reliable).

---

## 6. Evolutionary Search: Gradient-Free Alternative

### 6.1 Motivation

Fixed-point gradients work but introduce approximation error that accumulates over many steps. An alternative — particularly elegant in field arithmetic — is evolutionary optimization, which requires no gradients at all.

### 6.2 Algorithm

```
POPULATION: N = 16 weight vectors (each 91K field elements)
            Total population memory: 16 × 728 KB = 11.6 MB

PROCEDURE evolve(population, tir_blocks):
  FOR each individual in population:
    FOR each tir_block in batch:
      candidate_tasm = inference(individual.weights, tir_block)
      individual.fitness += score(candidate_tasm, tir_block)
  
  SORT population by fitness (descending)
  survivors = population[0:4]          // top 25%
  
  new_population = []
  FOR i in 0..16:
    parent_a = random_choice(survivors)
    parent_b = random_choice(survivors)
    child = crossover(parent_a, parent_b)  // uniform crossover
    child = mutate(child, rate=0.01)       // per-weight mutation
    new_population.append(child)
  
  RETURN new_population
```

Every operation — fitness evaluation (inference + scoring), sorting, crossover (conditional copy), mutation (random field element replacement) — is pure Tier 0+1 field arithmetic.

### 6.3 Cost Analysis

```
Per generation:
  16 individuals × inference(50K ops)     = 800,000 field ops
  16 individuals × scoring(100K ops)      = 1,600,000 field ops
  Sort + crossover + mutation             = ~100,000 field ops
  Total:                                  ≈ 2,500,000 field ops

On M4 Pro Metal: ~2.5M ops at ~50 GOPS = ~50 μs per generation

Generations per compilation: 10–50 (background, non-blocking)
Total evolution cost per compilation: 0.5–2.5 ms
```

For comparison, stark proving for the same code takes **seconds**. The optimization cost is <0.1% of the proving cost.

### 6.4 Hybrid Strategy

The optimal architecture combines both approaches:

```
PHASE 1: Gradient training (cold start)
  Train from scratch using fixed-point backpropagation.
  ~1M steps to reach baseline quality.
  Wall time on Mac: ~5 seconds.

PHASE 2: Evolutionary refinement (continuous)
  Seed population with gradient-trained weights + random perturbations.
  Evolve continuously in background during normal compilation.
  No gradient approximation error. Pure field arithmetic.
  Discovers strategies that gradient descent misses (cliff-jumping).
```

Gradient descent is efficient for large smooth improvements (learning the basic mapping from TIR structure to good TASM). Evolution excels at discrete cliff-jumping (finding the exact instruction count that drops below a power-of-2 boundary). The hybrid leverages both.

---

## 7. Correctness: Verification, Not Trust

### 7.1 The Speculative Architecture

The neural optimizer is never trusted. Every candidate TASM sequence is verified via [[formal verification]] for semantic equivalence against the source TIR before acceptance:

```
COMPILE(tir_block):
  // Classical path (always runs, deterministic, proven correct)
  baseline_tasm = classical_lower(tir_block)
  baseline_score = max_table_height(baseline_tasm)
  
  // Neural path (speculative, may fail, may be worse)
  candidate_tasm = neural_inference(tir_block, frozen_weights)
  
  IF semantic_equivalent(tir_block, candidate_tasm):
    candidate_score = max_table_height(candidate_tasm)
    IF candidate_score < baseline_score:
      RETURN candidate_tasm    // Neural wins
  
  RETURN baseline_tasm          // Classical fallback

INVARIANT: Output is ALWAYS semantically correct.
INVARIANT: Output is ALWAYS ≤ classical baseline cost.
PROPERTY:  Monotonic improvement — neural path never makes things worse.
```

### 7.2 Semantic Equivalence Checking

Two TASM sequences are semantically equivalent if, for all valid inputs, they produce identical outputs. For bounded programs (Trident guarantees bounded execution), this can be checked through:

**Symbolic execution:** Run both sequences on symbolic stack/RAM states. Compare output states symbolically. This is exact for straight-line code and bounded loops.

**Cost:** Symbolic execution of a 64-instruction TASM block: ~10K field operations ≈ 0.2 μs.

### 7.3 stark Proof of Compilation

For the highest assurance level, the entire compilation — including neural inference, [[verification]], and selection — can be wrapped in a stark proof ([[cryptographic proofs]]):

```
PROVABLE COMPILATION:

  Input:  TIR block (public)
  Output: TASM sequence (public)
  
  Witness: neural weights (private), intermediate computations (private)
  
  Circuit proves:
    1. TASM was produced by inference with specific weights
    2. TASM is semantically equivalent to TIR
    3. TASM has lower-or-equal table height vs classical baseline
  
  Proof size: O(log n) field elements
  Verification: O(log n) field operations (constant-time in practice)
```

Anyone can verify the compilation was faithful without trusting the compiler, the optimizer, or the hardware it ran on. This is nox constraint C₉ (self-verifying) applied to compilation itself.

---

## 8. The Self-Referential Fixed Point

### 8.1 The Loop

The neural optimizer, the verifier, the scorer, and the evolutionary training loop are all Trident programs. They are compiled by the same compiler they optimize. This creates a self-improvement loop:

```
ITERATION 0:
  Compiler₀ (classical, no neural path) compiles:
    - Neural inference engine    → TASM₀(inference)
    - Semantic verifier          → TASM₀(verifier)
    - Table height scorer        → TASM₀(scorer)
    - Evolutionary trainer       → TASM₀(trainer)
  
  Train neural weights W₁ using TASM₀(trainer)
  
ITERATION 1:
  Compiler₁ = Compiler₀ + neural path with weights W₁
  Compiler₁ compiles:
    - Neural inference engine    → TASM₁(inference)   // potentially better
    - Semantic verifier          → TASM₁(verifier)
    - Table height scorer        → TASM₁(scorer)
    - Evolutionary trainer       → TASM₁(trainer)
  
  Train neural weights W₂ using TASM₁(trainer)        // faster training

ITERATION k:
  Compilerₖ = Compiler₀ + neural path with weights Wₖ
  Score_k = Σ table_height(TASMₖ(program)) for all nox programs
  
CONVERGENCE:
  |Score_{k+1} - Score_k| < ε
  
  The compiler has reached a fixed point: it can no longer
  improve its own output. This is verifiable — compute Score_k
  and Score_{k+1} and check the difference.
```

### 8.2 Convergence Guarantee

**Theorem (Monotonic Convergence).** The sequence $\{\text{Score}_k\}$ is non-increasing and bounded below by the information-theoretic minimum TASM cost. Therefore it converges.

*Proof sketch.* At each iteration, the speculative architecture guarantees $\text{Score}_{k+1} \leq \text{Score}_k$ (the neural path is only accepted if it improves on the classical baseline, and the classical baseline at iteration $k+1$ uses the best compilation from iteration $k$). The score is bounded below by the minimum number of Triton VM cycles required to compute the program's semantics (which is positive for any non-trivial program). A non-increasing sequence bounded below converges. $\square$

### 8.3 What the Fixed Point Means

At [[convergent computation]] convergence, the compiler has found instruction sequences for its own components that it cannot improve. This is the computational analogue of a Nash equilibrium — no unilateral deviation (change to any single component's TASM) can improve the system's total score.

The fixed point is content-addressed (nox constraint C₂): the converged compiler has a unique hash that identifies the version of itself that achieved self-optimality. Any node in the nox network can verify that a claimed fixed-point compiler actually is one — recompile the compiler with itself and check that the output matches.

---

## 9. Implementation Roadmap

### Phase 1: Foundation (Weeks 1–4)

**Deliverable:** TIR specification with explicit optimization-relevant annotations.

- Define TIR node encoding (4 field elements per node, as specified in §4.1)
- Implement TIR→TASM classical lowering baseline
- Build table height profiler for all 9 AETs
- Benchmark all nox hot paths: transfer circuit, cyberlink creation, focus update, recursive verifier
- Establish baseline scores for each

### Phase 2: TASM-Gym Environment (Weeks 5–8)

**Deliverable:** Reinforcement learning environment for TASM optimization.

- State: partial TASM sequence + remaining TIR nodes + current table heights
- Action: next TASM instruction to emit
- Reward: cliff-aware score $-2^{\lceil \log_2(\max H_t) \rceil}$ at block completion; small shaping reward per step
- Constraint checker: symbolic stack type/depth tracker, semantic equivalence verifier
- Benchmark suite: 1000 TIR blocks extracted from nox programs

### Phase 3: Neural Model in Trident (Weeks 9–14)

**Deliverable:** The 91K-parameter model implemented as a Trident Tier 0+1 program.

- Fixed-point matrix multiply, attention, layer normalization in Goldilocks field
- Compile to Metal via KIR for GPU inference
- Compile to TASM for provable inference
- Validate numerical equivalence between GPU and TASM execution
- Measure inference latency on M4 Pro (target: <10 μs per block)

### Phase 4: Training Pipeline (Weeks 15–18)

**Deliverable:** Continuous learning system running on a single Mac.

- Fixed-point backpropagation in Trident
- Evolutionary search (population 16, as specified in §6.2)
- Hybrid training: gradient cold start → evolutionary refinement
- Background training integrated into `trident build` workflow
- Weight versioning with content-addressed identity

### Phase 5: Verification & Proof (Weeks 19–22)

**Deliverable:** stark-proven compilation.

- Symbolic equivalence checker for TIR↔TASM
- Speculative compilation architecture (§7.1)
- Optional stark wrapping of compilation decisions
- Recursive verification of compilation proof within nox block proofs

### Phase 6: Self-Referential Convergence (Weeks 23–26)

**Deliverable:** The compiler that optimizes itself to a fixed point.

- Compile all system components (inference, verifier, scorer, trainer) with the neural compiler
- Iterate until score convergence
- Verify fixed point: recompile and confirm hash identity
- Publish converged compiler hash as nox genesis artifact

---

## 10. Expected Impact

### 10.1 Quantitative Targets

Based on analysis of nox's hot paths and the five optimization surfaces described in §3:

| Program | Baseline (classical) | Target (neural) | Improvement | Dominant Surface |
|---|---|---|---|---|
| Transfer circuit | ~44,000 focus | ~33,000 focus | ~25% | Hash scheduling + table balance |
| Cyberlink creation | ~10,600 focus | ~8,500 focus | ~20% | Stack scheduling |
| Focus update (single node) | ~2,000 focus | ~1,500 focus | ~25% | Stack scheduling + loop restructuring |
| Recursive stark verifier | ~10⁶ focus | ~7.5×10⁵ focus | ~25% | Hash scheduling + table balance |

The 20–25% improvement estimates are conservative. They assume the neural optimizer captures stack scheduling improvements (where human-written tasm-lib is known to be suboptimal for complex functions), table balancing near cliff boundaries (which no current tool attempts), and hash coprocessor scheduling (which is critical for hash-heavy programs like the transfer circuit and stark verifier where Hash Table height dominates proving cost).

A single cliff-boundary crossing — reducing the maximum table height from just above $2^k$ to just below $2^k$ — yields a 2× improvement for that program. nox's transfer circuit, executed for every transaction on the network, is the highest-leverage target.

### 10.2 At Planetary Scale

nox targets $10^{15}$ [[cybergraph]] nodes processing $10^{12}$ transactions per second. Every percentage point of proving cost reduction, applied to every [[cyberlink]] transaction, compounds to enormous [[energy]] and latency savings. A 20% reduction in transfer circuit proving cost at full scale is the difference between feasibility and infeasibility for certain classes of hardware.

### 10.3 The Meta-Result

Beyond the quantitative improvements, the system demonstrates a principle: **a programming language designed for provable computation can prove the correctness of its own optimization**. The compiler's intelligence is not trusted — it is verified. The compiler's improvement is not hoped for — it is measured. The compiler's convergence is not assumed — it is proven.

This closes nox's trust loop at the compiler level. You don't trust the developer (you verify the [[neural proofs]] of execution). You don't trust the compiler (you verify the proof of compilation). You don't trust the optimizer (you verify the proof of optimization). Mathematics, all the way down.

---

## Appendix A: Triton VM Table Structure

For reference, the 9 Algebraic Execution Tables and their primary row-growth drivers:

| Table | Primary Growth Driver | Instructions |
|---|---|---|
| Processor | 1 row per clock cycle | All |
| Op Stack | Stack depth changes | push, pop, pick, place, dup, swap |
| RAM | Memory read/write operations | read_mem, write_mem, sponge_absorb_mem |
| Jump Stack | Function call/return pairs | call, return, recurse, recurse_or_return |
| Hash | Tip5 permutation invocations | hash, sponge_init/absorb/squeeze, merkle_step |
| Cascade | Lookup argument support | (internal to hash verification) |
| Lookup | S-box evaluation support | (internal to hash verification) |
| U32 | Bitwise/comparison operations | split, lt, and, xor, log_2_floor, pow, div_mod, pop_count |
| Degree Lowering | Helper columns for AIR degree | (automatic, proportional to trace) |

**All tables pad to** $2^{\lceil \log_2(H_{\max}) \rceil}$ **where** $H_{\max} = \max_t H_t$.

## Appendix B: Trident Tier 0+1 Neural Primitives

The complete set of field operations needed for neural inference, all available in Trident Tier 0+1:

```
MATRIX MULTIPLY:     mul, add         (bounded nested loops)
DOT PRODUCT:         mul, add         (single bounded loop)
SOFTMAX APPROX:      mul, add, invert (1/x approximation)
LAYER NORMALIZATION: mul, add, invert (mean and variance via field ops)
GELU APPROXIMATION:  mul              (polynomial: 0.5x(1 + tanh(√(2/φ*)(x + 0.044715x³))))
                                       (tanh via Pade approximant: rational function)
ARGMAX:              eq, lt           (comparison chain)
RANDOM (mutation):   divine           (non-deterministic input for evolution)
```

No Tier 2 (hash/sponge) or Tier 3 (recursive verification) operations required for model inference or training. The neural compiler deploys on every Trident target.

## Appendix C: The 640 KB Argument

For context on why 91K parameters suffice:

The input space is 144 field elements (≈32 TIR nodes × 4 features + 16 context). The output space is 64 instructions from a vocabulary of ~44. This is comparable in complexity to machine translation between two languages with 50-word vocabularies and 64-word sentences.

State-of-the-art character-level language models achieve near-optimal performance at 100K–500K parameters for constrained vocabularies. The TIR→TASM mapping is more structured than natural language (types constrain valid outputs, DAG structure constrains ordering), suggesting that less model capacity is needed, not more.

The model does not need to generalize to unseen TIR operations or unseen TASM instructions. The set of 54 TIR operations and ~44 TASM instructions is closed and known at compile time. This is a fixed-vocabulary, fixed-grammar translation task — the simplest class of sequence transduction problems.

---

*The compiler that proves its own optimization. The optimizer that improves its own compilation. The system that verifies its own convergence. Not by trust, but by proof.*

*purpose. link. [[energy]].*