---
tags: trident
alias: Neural_Techniques_for_Trident_v2
---
# Neural Network Techniques for Trident

## Compilation, Execution, Proving — Prioritized Roadmap v2

---

## Scope

Everything here lives strictly within the [[trident]] → TIR → TASM → Triton VM → STARK pipeline. The language, the compiler, the prover, and what [[neural networks]] can do for each of them.

---

## The Core Thesis

Trident compiles to arithmetic over the [[Goldilocks field]] ($p = 2^{64} - 2^{32} + 1$). This field is not a generic algebraic structure — it has deep internal symmetries: a multiplicative group of order $2^{32}(2^{32} - 1)$, primitive $2^{32}$th roots of unity, subgroups with cheap inversion, Frobenius-like shortcuts in extension field embeddings, and an unbounded hierarchy of polynomial identities.

Every one of these symmetries is a potential [[compilers]] optimization. Human algebraists find them one at a time, by studying theory and having insights. A neural network can search the space systematically, discovering identities no human would find, at a rate no human can match.

**This changes the nature of compilation.** A traditional compiler has a fixed set of optimization passes written by engineers. A neural-augmented Trident compiler has a *growing* set of [[algebra]] identities discovered by exploration. The compiler improves *forever* — every new identity reduces proving cost for every program that matches the pattern, past and future.

There is no bottom. Every branch of algebra offers new passes. The deeper the network explores, the more it finds. And every discovery compounds.

---

## Priority 0 — The Algebraic Identity Explorer

### 0.1 The Unbounded Optimization Surface

The [[field]] has layers of algebraic structure, each offering optimization opportunities:

**Layer 0 — Arithmetic identities**: $x + 0 = x$, $x \cdot 1 = x$, $x \cdot 0 = 0$. Trivial. Any compiler finds these.

**Layer 1 — Goldilocks-specific constants**: $2^{32}$ has special properties because $2^{32} \equiv 2^{64} - p + 1$ in this field. Multiplication by $2^{32}$ reduces to a shift plus a small correction. Multiplication by $(p-1)/2$ is equivalent to conditional negation. These are specific to $p = 2^{64} - 2^{32} + 1$ and do not exist in any other field.

**Layer 2 — Subgroup structure**: The multiplicative group has a subgroup of order $2^{32}$. Elements in this subgroup have cheap inversions — Fermat's little theorem ([[number theory]]) with exponent $2^{32} - 2$ instead of $p - 2$. Exponentiation chains for these elements are dramatically shorter.

**Layer 3 — Roots of unity**: The field has primitive $2^{32}$th roots of unity. Polynomial evaluations at these roots decompose into butterfly networks ([[fourier transform]] structure). A sequence of 8 multiplications might reduce to 3 if the constants are roots of unity. This applies to every program that touches polynomial arithmetic — which includes the STARK prover itself.

**Layer 4 — Extension field shortcuts**: Triton VM's hash function Tip5 operates on extension field elements embedded in the base field. Algebraic relationships in the extension (Frobenius endomorphism, norm maps, trace maps) create shortcuts invisible at the base field level. The NN discovers that certain hash-related instruction sequences have cheaper equivalents without understanding the theory behind them.

**Layer 5 — Algebraic geometry**: Programs computing polynomial evaluations, interpolations, and multi-point evaluations have structure related to the field's geometry. Evaluation at structured point sets shares intermediate values. Interpolation through subgroup cosets has closed-form shortcuts. This layer is essentially unbounded — every new polynomial identity is a new compiler pass.

**Layer N — Unknown**: The NN explores beyond named mathematical territory. Identities that emerge from the interaction of Triton VM's specific instruction set with the Goldilocks field's specific structure. No textbook covers this intersection. The NN maps it empirically.

---

### 0.2 Discovery Architecture

```
┌────────────────────────────────────────────────────────────────┐
│                  ALGEBRAIC IDENTITY EXPLORER                    │
├────────────────────────────────────────────────────────────────┤
│                                                                 │
│  PROPOSER (GFlowNet)                                            │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Input:                                                   │   │
│  │    - Known identity database (patterns + embeddings)      │   │
│  │    - Instruction vocabulary (~44 TASM ops)                │   │
│  │    - Frequency data from real program corpus              │   │
│  │                                                           │   │
│  │  Output:                                                  │   │
│  │    - Candidate pair (sequence_A, sequence_B)              │   │
│  │    - Each sequence: 2-12 TASM instructions + operands     │   │
│  │                                                           │   │
│  │  Reward: identity_found × usefulness_score                │   │
│  │  Diversity: GFlowNet samples proportional to reward       │   │
│  │  Exploration bonus: novel instruction combinations        │   │
│  └──────────────────────────────────────────────────────────┘   │
│                          │                                      │
│                          ▼                                      │
│  VALIDATOR (brute-force field execution)                        │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Stage 1: Execute both sequences on 10,000 random inputs  │   │
│  │           If ANY output differs → reject (not identity)   │   │
│  │                                                           │   │
│  │  Stage 2: Execute on 10,000,000 inputs (high confidence)  │   │
│  │           Probability of false positive: < 10^{-7}        │   │
│  │                                                           │   │
│  │  Stage 3: Symbolic execution → algebraic proof            │   │
│  │           Express both sequences as polynomial maps       │   │
│  │           Verify polynomial equality via Schwartz-Zippel  │   │
│  │           Or: exhaustive verification for small domains   │   │
│  │                                                           │   │
│  │  Stage 4 (optional): STARK proof of equivalence           │   │
│  │           The identity itself becomes a proven theorem     │   │
│  └──────────────────────────────────────────────────────────┘   │
│                          │                                      │
│                          ▼                                      │
│  USEFULNESS SCORER                                              │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Scan corpus of existing Trident programs                 │   │
│  │  Count: how often does sequence_A appear? (frequency)     │   │
│  │  Measure: cost(sequence_A) - cost(sequence_B) (savings)   │   │
│  │  Check: which AET tables benefit? (table_impact)          │   │
│  │                                                           │   │
│  │  score = frequency × savings × table_criticality          │   │
│  │                                                           │   │
│  │  table_criticality = extra weight if the savings hit      │   │
│  │  the table that is currently the bottleneck (tallest)     │   │
│  └──────────────────────────────────────────────────────────┘   │
│                          │                                      │
│                          ▼                                      │
│  RULE DATABASE                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Each rule:                                               │   │
│  │    pattern:          TASM instruction sequence (LHS)      │   │
│  │    replacement:      TASM instruction sequence (RHS)      │   │
│  │    cost_savings:     measured AET reduction                │   │
│  │    confidence:       validation stage reached (1-4)       │   │
│  │    frequency:        occurrences in corpus                │   │
│  │    layer:            algebraic depth (0-5+)               │   │
│  │    discovery_date:   when found                           │   │
│  │    composable_with:  list of non-conflicting rules        │   │
│  │                                                           │   │
│  │  Applied deterministically before neural compiler runs    │   │
│  │  Sorted by (frequency × savings) descending               │   │
│  │  Conflict resolution: longest match first                 │   │
│  └──────────────────────────────────────────────────────────┘   │
│                          │                                      │
│                          ▼                                      │
│  FEEDBACK TO PROPOSER                                           │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │  Successful identity    → positive reward                 │   │
│  │  Near-miss (99.9%)      → shaped reward (close)           │   │
│  │  Redundant (known)      → zero reward (stop re-finding)   │   │
│  │  Novel structure        → exploration bonus               │   │
│  │  Compositional          → bonus if A∘B reveals new C      │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                 │
└────────────────────────────────────────────────────────────────┘
```

---

### 0.3 The Compounding Flywheel

Every discovered identity triggers four compounding effects:

**1. Direct savings.** All programs containing the pattern get cheaper to prove. Retroactively — recompile old programs, get smaller proofs for free.

**2. Training enrichment.** Each identity is a new training example for the proposer. The GFlowNet learns the *shape* of identities — "what do valid rewrites look like in this field?" — and proposes higher-quality candidates.

**3. Compositional explosion.** Identity A transforms sequence X→Y. Identity B transforms sequence Y→Z. Composing them gives X→Z, which might be a deeper identity invisible at either layer alone. The rule database enables automated composition search: for every pair of rules where output of A matches input of B, test the composed rule.

**4. Corpus shift.** As rules are applied to real programs, the program corpus changes. New instruction patterns emerge that didn't exist before. The proposer explores these new patterns, potentially finding second-order identities that only exist *because* the first-order ones were applied.

**Monotonic improvement.** The rule database only grows. Rules are never removed (only demoted if usefulness drops). The compiler can only get better. Over months, the database accumulates thousands of rules, representing a collective algebraic knowledge base that no single mathematician could hold in their head.

---

### 0.4 Self-Referential Closure

The identity explorer is itself a Trident program. It compiles to TASM. Its own compilation benefits from the identities it discovers. The explorer optimizes its own execution.

The fixed point: when the explorer can no longer improve its own compilation cost, it has extracted the maximum algebraic efficiency reachable by its architecture. This fixed point represents a *lower bound* on the extractable efficiency of the Goldilocks field for Triton VM's instruction set — a [[convergent computation]].

A larger explorer (more parameters, longer search horizon) might find deeper identities and reach a lower fixed point. The hierarchy of fixed points, indexed by explorer capacity, converges to the *theoretical minimum proving cost* for each program — the algebraic Shannon limit of the field.

---

### 0.5 Estimated Yield by Layer

| Layer | Example | Frequency | Savings per match | Cumulative impact |
|---|---|---|---|---|
| 0 | `push 0; add` → ∅ | Very high | 1-2 rows | 5-10% |
| 1 | `push 2^32; mul` → shift trick | High | 3-10 rows | 10-20% |
| 2 | Cheap inversion for subgroup elements | Medium | 10-50 rows | 20-30% |
| 3 | NTT butterfly for root-of-unity constants | Medium | 50-200 rows | 30-50% |
| 4 | Hash function internal shortcuts | Low | 100-500 rows | 40-60% |
| 5+ | Polynomial arithmetic restructuring | Low | 200-1000+ rows | 50-70%+ |

Note: savings compound multiplicatively across layers. A program that benefits from Layer 1 + Layer 3 + Layer 4 optimizations could see 3-5× proving cost reduction.

---

### 0.6 Implementation

**Phase A — Brute-force baseline** (1 week):
Write a random sequence pair generator. Execute on random inputs. Collect identities. No NN yet — just exhaustive search over short sequences (length 2-4). This builds the initial rule database and establishes the validation pipeline.

**Phase B — Guided search** (2 weeks):
Train a small NN (MLP, ~10K params) to predict "is this pair likely to be an identity?" from instruction sequence features. Use it to filter random proposals. 10× speedup over brute force.

**Phase C — GFlowNet proposer** (3 weeks):
Replace the MLP filter with a GFlowNet that actively constructs candidate pairs. Reward is identity_found × usefulness. Diversity ensures exploration of all algebraic layers, not just the easiest.

**Phase D — Compositional search** (2 weeks):
Automated composition of known rules. For every pair (A, B) where A's RHS overlaps with B's LHS, test A∘B as a new rule. Depth-limited to avoid combinatorial explosion (max composition depth 3).

**Phase E — Continuous operation** (ongoing):
The explorer runs 24/7. Every week, the rule database grows. Every month, recompile the full program corpus with the expanded database. Measure cumulative savings. Publish the rule database as an open artifact.

---

## Priority 1 — Foundation Layer

### 1.1 Field-Native Neural Network Library (`nn.trd`)

**What**: A Trident library implementing neural network primitives — [[linear algebra]] (matrix multiply, dot product), layer normalization, activation functions — all in Goldilocks field arithmetic.

**Why**: Every other technique on this list either IS a neural network running in Trident, or benefits from having one. Including the identity explorer's proposer.

**Architecture**:
```
nn.trd
├── field_signed.trd      — signed integer convention (x > p/2 → negative)
├── field_fixed.trd       — fixed-point arithmetic via scaling factor
├── linalg.trd            — matmul, dot product, vector ops
├── activations.trd       — GELU (polynomial), ReLU (conditional), tanh (Padé)
├── layers.trd            — linear layer, layer norm, residual connection
├── loss.trd              — MSE, cross-entropy (field approximations)
└── inference.trd         — forward pass orchestration
```

**Key constraints**:
- No negative numbers natively → signed representation via convention
- No fractions natively → fixed-point with configurable scale factor (e.g., $2^{16}$)
- No floating point → all activations are polynomial or rational approximations
- Every operation is a field op → every inference produces a valid Triton VM trace

**Size**: ~500 lines of Trident. A 3-layer MLP with 64-wide hidden layers compiles to ~2,000 TASM instructions.

**The result**: A neural network whose every inference is a STARK-proven ([[zero knowledge proofs]]) computation, compiled from `.trd` source. The identity explorer (§0) benefits from this immediately — its own NN inference is provable.

**Tier**: 0+1 only. No hash/sponge or recursive verification needed for NN inference.

---

### 1.2 Field-Native Evolutionary Training

**What**: Train neural networks entirely within Goldilocks field arithmetic using evolutionary [[optimization]].

**Why**: Gradient descent in field arithmetic requires finite-difference approximation (noisy). Evolution sidesteps this. Crossover = conditional copy. Mutation = random field element replacement. Both are pure Tier 0 ops.

**Algorithm**:
```
POPULATION:  N = 16 weight vectors (each ≤91K field elements)
MEMORY:      16 × 728 KB = 11.6 MB total

FOR each generation:
  FOR each individual:
    FOR each training example:
      output = inference(individual.weights, input)
      individual.fitness += score(output, expected)
  
  SORT by fitness (descending)
  survivors = top 25%
  
  FOR i in 0..N:
    parent_a, parent_b = random_choice(survivors)
    child = uniform_crossover(parent_a, parent_b)
    child = mutate(child, rate=0.01)
    new_population[i] = child
```

**Performance**: ~2.5M field ops per generation. On M4 Pro Metal: ~50μs per generation. 1,000 generations in 50ms.

**Hybrid path** (optional):
- Phase 1: Fixed-point finite-difference gradients for cold start (~1M steps, ~5s)
- Phase 2: Evolutionary refinement for cliff-jumping (continuous, finds strategies gradients miss)

**Deliverable**: A self-contained Trident program that trains a neural classifier, proves every training step, and outputs verified weights.

---

### 1.3 Predictive Trace Analysis

**What**: A small NN (trained via §1.2) that predicts the 9 AET table heights from TIR features, before compilation and execution.

**Why**: Every compilation optimization depends on understanding the cost landscape. This provides it cheaply.

**Input**: TIR graph features — node count, operation type histogram, max nesting depth, branch count, loop bound sum, memory access count. ~32 field elements.

**Output**: 9 field elements — predicted height of each AET table.

**Model**: Single hidden layer, 64 units. ~6K parameters. Trains in seconds.

**Training data**: Compile and execute N programs, record actual table heights. Start with ~1,000, scale to ~100,000.

**Uses**:
1. Fast cost estimation during compilation (no execution needed)
2. Identifying proof-expensive programs before committing
3. Cost-aware TIR optimization — reject transformations that increase predicted cost
4. Training signal for the neural compiler (§2.1)
5. Table criticality weights for the identity explorer's usefulness scorer (§0.2)

**Accuracy target**: Predict the bottleneck table correctly >90% of the time. Exact height within 20%.

---

## Priority 2 — Compiler Intelligence

### 2.1 Differentiable STARK Cost Surrogate

**What**: A learned, differentiable approximation of the STARK proving cost. Feed it a TASM sequence, get a predicted proving time.

**Why**: The actual cost function — $\text{cost}(S) = 2^{\lceil \log_2(\max_t H_t(S)) \rceil}$ — is non-differentiable (power-of-2 ceiling, max over tables). A smooth surrogate enables gradient-based optimization.

**Architecture**: 1D CNN over TASM instruction sequences. Input: instruction IDs + operands, padded to 128. Output: scalar cost. ~15K parameters.

**Training**: (TASM_sequence, actual_proving_time) pairs from real proving runs.

**Key insight**: Doesn't need absolute accuracy. Needs correct *ranking* — "Is TASM A cheaper than TASM B?" Pairwise ranking accuracy >95% suffices.

**Enables**: Backpropagation through cost → gradient-guided compilation. Combines with evolution: gradients for smooth landscape, evolution for cliff-jumping.

---

### 2.2 Learned Instruction Scheduling

**What**: Reorder TASM instructions to minimize AET table heights while respecting data dependencies. Only permutes — never modifies instructions.

**Why**: Correctness guaranteed by construction (dependency-respecting permutations). No verifier needed. Pure upside.

**Architecture**: [[graph neural network]] on the TASM dependency DAG. Outputs priority score per instruction. Schedule greedily by priority.

**The GNN sees**: instruction type, current stack depth, which AET tables each instruction touches, distance to dependency predecessors/successors.

**The GNN learns**:
- Cluster hash instructions to avoid interleaving with arithmetic (minimize Hash table padding)
- Front-load RAM operations for pipeline filling
- Delay U32 operations until after main computation (avoid U32 table domination)

**Training**: For each program, generate 1,000 random valid schedules, measure actual table heights, train GNN to predict height-minimizing ordering.

**Model**: ~20K parameters. Runs in microseconds.

**Safety**: The scheduler only outputs dependency-respecting permutations. Enforced by algorithm (topological sort with learned priorities), not by model.

---

### 2.3 Multi-Objective Compiler Ensemble

**What**: 8-16 specialist TIR→TASM optimizers, each biased toward minimizing a different AET table. Meta-selector picks the winner per program.

**Why**: Cliff-discontinuity cost function means the bottleneck table shifts between programs. No single model dominates.

**Specialist fitness functions**:
```
specialist_0:  -max(H_all_tables)           # minimize overall max
specialist_1:  -H_processor                 # minimize Processor
specialist_2:  -H_hash                      # minimize Hash  
specialist_3:  -H_ram                       # minimize RAM
specialist_4:  -(H_processor + H_hash)      # minimize joint bottleneck
specialist_5:  -(max_H - second_max_H)      # maximize balance
...
```

**Meta-selector**: Run all specialists, use trace predictor (§1.3) or cost surrogate (§2.1) to pick winner without proving all candidates.

**Memory**: 16 specialists × 728 KB = 11.6 MB. L2 cache.
**Latency**: 16 × 50μs ≈ 800μs. Still <1ms. Negligible vs. proving (seconds).

---

### 2.4 Learned Peephole Patterns

**What**: 1D convolutional network scanning TASM instruction windows (size 3-8), detecting sequences where local substitution reduces cost.

**Why**: The evolutionary compiler discovers patterns implicitly. Peephole extraction makes them explicit, composable, transferable.

**Architecture**: 1D Conv (kernel 5) → ReLU → 1D Conv (kernel 3) → classifier. Input: sliding window. Output: (should_rewrite, replacement_pattern_id).

**Training pipeline**:
1. Run evolutionary compiler on 10,000 programs
2. Diff naive lowering vs. evolved output
3. Extract per-window changes
4. Train conv net to detect replaceable windows and predict replacements

**Relationship to Identity Explorer (§0)**: The explorer discovers algebraically *valid* identities from field theory. The peephole learner discovers *compiler-specific* patterns from evolutionary search. They feed the same rule database. The explorer finds "this IS equivalent." The peephole learner finds "this SHOULD be rewritten." Together they produce rules that are both valid AND useful.

**Composability**: Rules become deterministic passes applied before the neural compiler. Over time, the deterministic rules handle easy cases; the neural compiler focuses on hard ones.

---

### 2.5 Neural Decompilation (TASM → TIR)

**What**: Given optimized TASM, reconstruct a plausible TIR representation.

**Why**:
- Learn from hand-written TASM (expert knowledge → training data)
- Cross-pollinate optimizations between programs
- Sanity-check neural compiler output via round-trip
- Enable TIR → TASM → TIR' → TASM' equivalence testing

**Architecture**: Sequence-to-graph model. Attention encoder, autoregressive graph decoder. ~50K parameters.

**Training data**: Every compilation (TIR → TASM) generates a free training pair. 100,000 compilations → 100,000 pairs.

**Correctness**: Decompiled TIR is a suggestion. Verify by recompiling and checking equivalence.

---

## Priority 3 — Proving Acceleration

### 3.1 NN-Guided STARK Prover Configuration

**What**: RL agent selecting STARK prover parameters per-program to minimize proving time. Proof remains valid regardless — agent only affects speed.

**Why**: Multiple tunable parameters where optimal settings are program-dependent. Current heuristics leave performance on the table.

**Configurable parameters**:

| Parameter | Current | Learned |
|---|---|---|
| FRI folding factor | Fixed (e.g., 8) | Per-program adaptive |
| Grinding bits | Fixed (e.g., 20) | Trade proof size vs. time |
| Blowup factor | Fixed (e.g., 4) | Soundness-cost tradeoff |
| Evaluation domain offset | Default coset | Numerically-tuned |
| Memory layout | Sequential | Cache-optimized |
| Parallelism strategy | Default threading | Structure-aware |

**Agent**: MLP. Input: program features + AET heights + hardware specs. Output: config vector. ~10K parameters.

**Training**: For each program, run prover with K random configs, record (features, config, time) triples, train agent to predict time-minimizing config.

**Safety**: Cannot compromise soundness. All configs produce valid proofs or prover rejects and falls back to defaults.

**Expected impact**: 10-30% proving time reduction. Compounds across all programs.

---

### 3.2 Neural Proof Compression

**What**: Predictor anticipating redundant STARK proof elements, transmitting only the unpredicted delta.

**Why**: STARK proofs are hundreds of KB. If predictor achieves 80% accuracy, ~5× compression.

**Architecture**: Small autoregressive model over proof elements. Conditions on program, public inputs, previous elements.

**Verification**: Verifier runs identical predictor. Uses predictions where correct, transmitted values where not. Full proof is still checked — compression is transport-layer only.

**Compatibility**: Works with any STARK system. No prover/verifier logic changes.

**Requirement**: Predictor must be deterministic and version-matched on both sides. Ship as a Trident program (naturally).

---

### 3.3 Neural Theorem Prover for TASM Equivalence

**What**: Given two TASM sequences, find a chain of valid rewrite steps transforming one into the other.

**Why**: Structural proof of equivalence ([[formal verification]]) for ALL inputs, not just tested ones. Generates reusable optimization knowledge.

**Rewrite rules** (enumerable, finite):

| Rule | From | To | Condition |
|---|---|---|---|
| Dead push | `push X; pop` | ∅ | X unused |
| Constant fold | `push X; push Y; add` | `push (X+Y)` | Constants |
| Identity elim | `push 0; add` | ∅ | Always |
| Swap cancel | `swap K; swap K` | ∅ | Always |
| Commutativity | `push X; push Y; add` | `push Y; push X; add` | Always |
| Reorder | `A; B` | `B; A` | No dependency |
| **Explorer rules** | *from §0 database* | *from §0 database* | *proven valid* |

**Key**: The identity explorer (§0) feeds rewrite rules directly into the NTP's rule vocabulary. As the explorer discovers deeper algebraic identities, the NTP can prove equivalences across wider transformations. The NTP and the explorer form a mutual amplification loop.

**Architecture**: Embed TASM sequences via GNN on dependency DAG. Predict rewrite rule to apply at each step. Search guided by embedding distance to target.

**Byproduct**: Every successful rewrite path IS a new peephole pattern (§2.4). The NTP feeds the peephole database. The peephole database speeds up the NTP. Flywheel.

---

## Priority 4 — Developer Experience

### 4.1 Neural Type Inference

**What**: Predict [[type theory]] annotations for Trident programs, choosing types that minimize TASM cost.

**Why**: Different type representations (bool as field element vs. bit, integer width) produce different AET profiles. The "right" type minimizes proof cost.

**Architecture**: Tree-LSTM on Trident AST. Two-headed output: (type_prediction, expected_cost_delta).

**Interface**: LSP-style autocomplete for types. Red underline if type choice is >2× more expensive than optimal.

---

### 4.2 Incremental Recompilation via Neural Diff

**What**: Predict which TIR nodes are affected by a source edit, recompile only those subgraphs.

**Why**: Full recompilation is slow for large programs. Incremental enables interactive development.

**Architecture**: GNN on TIR dependency graph. Input: (old_TIR, edit_location, new_fragment). Output: per-node affected/stable classification.

**Correctness**: Conservative — if uncertain, mark as affected. Bias toward recall >99.9%.

**Integration**: `trident watch` mode. Save → predict diff → incremental compile → incremental prove. Target: <100ms for single-line edits.

---

### 4.3 Fuzzing-Guided Program Synthesis

**What**: Given input/output examples as field element pairs, synthesize a Trident program satisfying the spec. Triton VM proves correctness.

**Why**: Specification-first workflow. Write tests, let the network generate code, prove it correct.

**Architecture**: Seq2seq. Encoder: set of (input, output) pairs → fixed representation. Decoder: autoregressive TIR ops (vocab 54, max length 32). ~40K parameters.

**Verification**: Compile synthesized program, execute on spec inputs. Match → STARK proof. No match → generate more candidates (beam search K=16).

**Feedback**: Failed candidates → negative training signal. Successes → added to training set. Synthesizer improves as corpus grows.

---

## Priority 5 — Adversarial Hardening

### 5.1 Adversarial Program Generation

**What**: NN generating Trident programs designed to defeat the neural compiler — programs where optimization yields no improvement.

**Why**: Finds compiler blind spots. Satisfies "100× adversarial load" quality gate. Automated and continuous.

**GAN-like loop**:
```
Each epoch:
  1. Adversary generates 100 programs
  2. Neural compiler optimizes each
  3. Measure improvement ratio
  4. Adversary reward = programs where compiler failed
  5. Compiler trains on adversarial programs
  6. Repeat
```

**Convergence**: Adversary and compiler reach equilibrium when adversary can't find programs the compiler handles poorly. Equilibrium IS the quality gate.

**Feeds the explorer**: Adversarial programs that resist optimization often contain instruction patterns where no known algebraic identity helps. These patterns become priority targets for the identity explorer (§0) — "find an identity that covers THIS pattern."

---

### 5.2 Equivalence Checker Stress Testing

**What**: Generate "almost equivalent" TASM pairs — agree on 99.99% of inputs, differ on edge cases. Test whether verification catches the discrepancy.

**Why**: STARK verifier is sound in theory. Implementation bugs exist. Stress test the pipeline.

**Generation**: Take correct TASM, apply single mutation (change one operand, swap two instructions with subtle dependency). Test equivalence checker on (original, mutant) pairs.

**Target**: Zero false positives (mutant declared equivalent). Track false negatives (equivalent pairs declared different) as optimization opportunities.

---

## Priority 6 — Cross-Target Transfer

### 6.1 Transfer Learning Between Proof Backends

**What**: When Trident adds new targets (Miden [[vm]], SP1, OpenVM), transfer compiler knowledge from Triton VM.

**Why**: TIR-level patterns generalize across backends. Only the lowering changes.

**Architecture**: Split neural compiler into shared TIR encoder + per-target backend decoder.

**Transfer**: Train on Triton VM → freeze encoder → train only decoder for new backend → fine-tune if needed.

**Data efficiency**: New backend needs ~10% of Triton VM's training data.

**Identity explorer transfer**: Layer 0-1 identities (arithmetic) transfer directly. Layer 2+ identities are field-specific but architecture-dependent — need re-validation per backend. The validation pipeline from §0.2 handles this automatically.

---

## Dependency Graph

```
              ┌──────────────────────────────────────────┐
              │    [0] ALGEBRAIC IDENTITY EXPLORER        │
              │    (runs continuously, feeds everything)  │
              └──────┬──────────────┬────────────────────┘
                     │              │
                     │              ├──→ [2.4 Peephole Patterns]
                     │              ├──→ [3.3 NTP Equivalence] ──→ [2.4]
                     │              └──→ [5.1 Adversarial Gen] ──→ [0]
                     │
                     ▼
[1.1 nn.trd] ──→ [1.2 Evolutionary Training]
                     │
                     ├──→ [1.3 Trace Predictor]
                     │         │
                     │         ├──→ [2.1 Cost Surrogate]
                     │         │         │
                     │         │         └──→ [2.3 Compiler Ensemble]
                     │         │
                     │         └──→ [3.1 Prover Config Agent]
                     │
                     ├──→ [2.2 Instruction Scheduling]
                     │
                     ├──→ [2.5 Neural Decompilation]
                     │
                     ├──→ [4.1 Type Inference]
                     │
                     ├──→ [4.3 Program Synthesis]
                     │         │
                     │         └──→ [5.1 Adversarial Gen]
                     │
                     └──→ [6.1 Transfer Learning]

[3.2 Proof Compression]       ← needs proof corpus, independent of compiler
[4.2 Incremental Recompile]   ← needs TIR graph only, independent  
[5.2 Equivalence Stress Test] ← needs TASM mutation + verifier, independent
```

---

## Implementation Timeline

| Phase | Items | Effort | What you learn |
|---|---|---|---|
| **Phase 0a: Explorer Bootstrap** | §0 Phase A-B (brute-force + NN filter) | 3 weeks | Field structure empirically, validation pipeline |
| **Phase 0b: NN Foundation** | §1.1, §1.2 | 2 weeks | Field-native NN, evolutionary training, world-first provable NN |
| **Phase 1: Cost Intelligence** | §1.3, §2.1 | 2 weeks | AET cost landscape, differentiable optimization |
| **Phase 2: Compiler Core** | §2.2, §2.3, §2.4 | 3 weeks | Scheduling, ensembles, peephole extraction |
| **Phase 3: Explorer Maturity** | §0 Phase C-D (GFlowNet + composition) | 3 weeks | Deep algebraic identities, compositional search |
| **Phase 4: Proving** | §3.1, §3.2 | 3 weeks | Prover internals, proof structure |
| **Phase 5: Developer UX** | §4.1, §4.2, §4.3 | 4 weeks | Language ergonomics, synthesis |
| **Phase 6: Hardening** | §5.1, §5.2, §3.3 | 3 weeks | Adversarial robustness, formal equivalence |
| **Phase 7: Multi-Target** | §6.1, §2.5 | 2 weeks | Backend abstraction, transfer |
| **Continuous** | §0 Phase E (explorer 24/7) | Ongoing | Unbounded improvement |

**Total structured work**: ~25 weeks. Phase 0a + 0b alone produce two publishable results: a provable neural network and an automated algebraic identity discoverer.

**The explorer never stops.** After Phase 3, it runs continuously. Every week the rule database grows. Every month, cumulative proving cost drops. The gap between Trident and every other proof-system language widens — because no other language has a neural network mining its own field theory for optimizations.

---

## The Endgame

A traditional compiler is a fixed artifact. Engineers write passes, ship a version, write more passes, ship another version. The passes are finite. The compiler converges to a local optimum defined by the engineers' knowledge.

A Trident compiler with the identity explorer is an *open-ended learning system*. The algebraic structure of the Goldilocks field is finite but astronomically large. The explorer maps it continuously. The rule database grows monotonically. The compiler improves without human intervention.

Every program ever written in Trident gets cheaper to prove over time — not because the hardware improved, but because the compiler discovered a new algebraic identity that applies to a pattern in that program.

This is the moat. Not features. Not performance benchmarks at a point in time. A compiler that gets better every day, powered by the bottomless well of algebraic structure in the field it was born to compute over.

---

*The deeper you go into the field theory, the more optimizations you find. There is no bottom.*

*purpose. link. [[energy]].*
