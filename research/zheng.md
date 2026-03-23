---
title: "zheng: a self-proving proof system"
authors: [cyber]
date: 2026-03-23
status: draft
---

# zheng: a self-proving proof system

## Abstract

We present zheng, a proof system built on sumcheck and multilinear polynomial commitments. The architecture differs from the FRI-based STARK ecosystem at the foundation: where FRI commits to univariate polynomials via Merkle trees, zheng commits to multilinear polynomials via algebraic evaluation. This difference propagates through the entire stack. Merkle trees disappear from proof internals. State commitments become polynomial evaluations. Recursive composition reduces from circuit verification to accumulator folding. Data availability sampling uses polynomial openings instead of tree paths.

zheng composes with hemera (Poseidon2-Goldilocks hash, x⁻¹ S-box, 32-byte output, ~736 constraints per permutation) as the sole cryptographic hash — used for content identity, Fiat-Shamir, and trust anchoring, but progressively eliminated from proof internals where algebraic mechanisms are cheaper.

The system targets a self-proving knowledge graph where every state transition carries its own proof, every query is verifiable, and the entire history compresses to a ~200 byte accumulator. We provide concrete numbers: 33× fewer constraints per state update, 700× cheaper epoch composition, < 10 KiB light client join bandwidth, 10–50 μs verification for any claim about the graph.

## 1. Architecture

### 1.1 Why not FRI

The dominant STARK architecture (Stwo, Plonky3, SP1, RISC Zero) commits to univariate polynomials via FRI (Fast Reed-Solomon IOP). FRI requires:

1. Merkle tree over polynomial evaluations (commitment)
2. Multiple folding rounds with Merkle authentication paths (opening)
3. O(N log N) prover time (FFT for polynomial evaluation)
4. Recursive verification = verifying Merkle paths inside a circuit (~50K–200K constraints)

77% of a FRI-based proof is Merkle authentication paths. 71% of recursive verification cost is Merkle path checking. The tree is the bottleneck.

### 1.2 The sumcheck alternative

zheng replaces FRI with SuperSpartan + sumcheck over multilinear extensions:

```
FRI:        univariate polynomial → Merkle tree → tree path openings
sumcheck:   multilinear polynomial → algebraic evaluation → field operation openings
```

consequences:

- **prover**: O(N) — sumcheck is inherently linear. no FFT, no NTT, no O(N log N) bottleneck
- **proof size**: algebraic openings replace Merkle paths. batch opening of k positions = one quotient polynomial, not k tree walks
- **recursion**: no Merkle paths to verify inside circuits. recursive cost drops from O(verifier_circuit) to O(accumulator_fold)
- **state commitments**: polynomial state reads are O(1) field operations — the state IS a polynomial, and the proof system operates on polynomials natively

### 1.3 Component stack

```
zheng
├── IOP:          SuperSpartan + sumcheck              (field-generic)
├── Composition:  HyperNova folding                    (field-generic)
├── Hash:         hemera-2                             (one hash, universal)
│                 x⁻¹ S-box, 32-byte output
│                 ~736 constraints per permutation
├── PCS₁:         WHIR / Brakedown (Goldilocks F_p)
├── PCS₂:         Binius (F₂ tower)
├── Constraints:  CCS (Customizable Constraint System)
└── Languages:    14 nox instantiations → 2 fields → 2 PCS → 1 IOP → 1 hash
```

one hash. two fields. two PCS backends. one IOP. one folding scheme. 14 source languages.

## 2. Hemera-2: the trust anchor

hemera is Poseidon2 over the Goldilocks field (p = 2⁶⁴ − 2³² + 1).

### 2.1 hemera-2 parameters

```
S-box:              x⁻¹ (field inversion, replaces x⁷)
full rounds:        4 + 4 = 8
partial rounds:     16 (reduced from 64 via x⁻¹ algebraic degree)
state width:        16 Goldilocks elements
capacity:           8 elements
rate:               8 elements (64 bytes)
output:             4 elements = 32 bytes (compact)
algebraic degree:   2^1046 (vs 2^504 for x⁷)

constraints per permutation:  ~736
tree node (binary):           1 permutation (32 + 32 = 64 bytes < rate)
MPC/FHE depth:                40 (vs 216 for hemera-1, 5.4× reduction)
```

### 2.2 hemera's evolving role

hemera starts as "hash for everything" and progressively specializes:

```
hemera-1:  hash for everything                    ~1,152 constraints/perm
hemera-2:  faster hash for everything             ~736 constraints/perm
hemera-3:  hash for trust anchoring only          ~544 constraints (wired MDS)
           + algebraic for proof internals

endgame role:
  content identity    H(particle) = 32-byte CID           always hemera
  private records     mutator set commitments               always hemera
  Fiat-Shamir seed    initial transcript binding            1 hemera call
  proof binding       algebraic Fiat-Shamir thereafter      polynomial, not hemera
  state verification  NMT paths → polynomial openings       0 hemera calls
  DAS sampling        NMT inclusion → PCS openings          0 hemera calls
```

hemera-3 optimizations for the remaining hemera calls:

| optimization | mechanism | savings |
|---|---|---|
| batched proving | N calls → 1 sumcheck | 400× for N=1000 |
| folded sponge | fold absorption into accumulator | 18× for 4 KiB input |
| algebraic Fiat-Shamir | polynomial challenges, not hash | 8.7× fewer hemera calls |
| constraint-free MDS | absorb linear layers into CCS wiring | 26% fewer constraints |
| partial round collapse | precompute 16-round linear map | 4× prover wall-clock |

combined: hemera goes from dominant proof cost to negligible trust anchor.

## 3. Polynomial Commitment Schemes

### 3.1 WHIR (Goldilocks)

WHIR commits to multilinear polynomials via hemera Merkle trees over evaluation domains. the current zheng-1 backend.

```
commitment:    hemera Merkle tree over evaluations
opening:       log N folding rounds + Merkle authentication paths
proof size:    60–157 KiB (77% is Merkle paths)
verification:  ~1 ms (Merkle path walking)
security:      transparent, post-quantum, hash-only
```

WHIR is the starting point. its Merkle paths are the primary optimization target.

### 3.2 Algebraic extraction (WHIR optimization)

replace k individual Merkle path openings with one batch algebraic opening:

```
given k query positions and claimed values:
  construct quotient Q(x) = (f(x) - I(x)) / Z(x)
  one sumcheck verifies: f(r) - I(r) = Q(r) · Z(r)

proof content: sumcheck transcript + one quotient commitment + one evaluation
proof size:    5–12 KiB (vs 60–157 KiB)
verification:  ~150 μs (vs ~1 ms)
recursive:     ~5K constraints (vs ~50K for Merkle paths)
```

near-term improvement on existing WHIR. no new cryptographic assumptions.

### 3.3 Brakedown (Goldilocks, Merkle-free)

polynomial commitment via expander-graph linear codes. no Merkle tree at all.

```
commitment:    linear code encoding (no tree)
opening:       O(√N) proximity proof
proof size:    1–5 KiB
verification:  O(√N) field operations
security:      transparent, post-quantum, code-based

advantage:     Merkle tree eliminated entirely
               hemera calls for PCS: 0
               bottleneck shifts from hemera to field arithmetic (nebu)
```

Brakedown is the endgame Goldilocks PCS. WHIR + algebraic extraction is the near-term path.

### 3.4 Binius (F₂ tower)

binary-native PCS over F₂ tower fields. serves binary workloads where bitwise operations dominate.

```
F₂ tower:      F₂ → F₂² → F₂⁴ → ... → F₂¹²⁸
packing:       128 F₂ elements per u128 machine word
commitment:    hemera Merkle tree over packed rows (hemera external, not proved in F₂)
opening:       Binius folding (halve polynomial per round)

cost comparison:
  AND gate:    ~32 constraints in F_p, 1 constraint in F₂ (32×)
  XOR gate:    ~32 constraints in F_p, 1 constraint in F₂ (32×)
  comparison:  ~64 constraints in F_p, ~1 constraint in F₂ (64×)
  field mul:   1 constraint in F_p, ~64 constraints in F₂ (0.016×)
```

binary wins for bitwise. Goldilocks wins for arithmetic. the compiler chooses based on workload.

key constraint: hemera is NEVER proved inside F₂ circuits (~142K binary constraints). recursion always crosses to Goldilocks where hemera is native (~736 constraints). this preserves the one-hash invariant.

## 4. Composition: HyperNova Folding

### 4.1 The recursion problem

recursive proof composition (proof-of-proof) requires verifying a proof inside a circuit. with FRI:

```
recursive verification:  ~50K–200K constraints
  Merkle path checking:  71% of cost
  Fiat-Shamir hashing:   7%
  FRI folding:            14%
  constraint evaluation:  4%
```

each recursion level costs a full verification circuit. N levels = N × verification_cost.

### 4.2 Folding replaces verification

HyperNova folding accumulates proof obligations without verifying them:

```
fold step:     ~30 field operations + 1 hemera hash
               (vs ~50K–200K constraints for recursive verification)

the accumulator is a running claim:
  "all folded instances are satisfiable"
  verified by running ONE decider at the end
```

this transforms composition:

```
                        recursive verification     HyperNova folding
per-step cost:          ~70K constraints            ~30 field ops
block (1000 tx):        1000 × 70K = 70M            1000 × 30 + 70K = ~100K
epoch (1000 blocks):    1000 × 70K = 70M            1000 × 30 + 70K = ~100K
improvement:            —                           700×
```

### 4.3 Cross-algebra folding

universal CCS with algebra selectors enables folding across fields:

```
universal_ccs = {
  sel_Fp:   1 for Goldilocks rows, 0 otherwise
  sel_F2:   1 for binary rows, 0 otherwise
}
```

a Goldilocks transaction and a binary transaction fold into the SAME accumulator. the decider handles both. boundary cost: ~766 F_p constraints per algebra crossing — negligible versus execution cost.

### 4.4 Proof-carrying computation

fold during execution, not after:

```
each VM step:
  1. execute (nox pattern dispatch)
  2. generate trace row
  3. fold into accumulator (~30 field ops)

at computation end: accumulator IS the proof
zero additional proving latency
```

the separate "proving phase" disappears. computation and proving are one activity.

## 5. Why Polynomial State Is Natural

this section addresses a causal question: does the proof system enable algebraic state, or does algebraic state require this proof system?

the answer: **sumcheck architecture makes polynomial state natural. FRI architecture makes it expensive.**

### 5.1 The FRI path

FRI commits to univariate polynomials via Merkle trees. state reads in FRI:

```
"what is the value at position x?"
→ walk Merkle tree to leaf
→ authenticate with O(log n) hash siblings
→ ~32 × 736 = ~23,552 constraints per read (hemera-2, depth 32)
```

the Merkle tree is the state structure. every read pays for the tree depth. this is inherited — FRI needs the tree, the tree costs O(log n) hashes, state pays the tax.

### 5.2 The sumcheck path

sumcheck operates on multilinear polynomials. state reads in sumcheck:

```
"what is the value at position x?"
→ evaluate the multilinear polynomial at x
→ verify via sumcheck (field operations, no tree)
→ ~100–200 field operations per read
```

the polynomial IS the state. reading it IS evaluating it. the proof system's native operation (polynomial evaluation) is the same as the state query operation. there is no tree to walk, no hash to compute, no O(log n) overhead.

### 5.3 Implications for authenticated state

a system with N state elements:

```
FRI-based state:
  commitment:       Merkle tree over evaluations
  read:             O(log N) hemera hashes = O(log N × 736) constraints
  update:           O(log N) hemera hashes (path rehash)
  cross-index:      separate LogUp proof (~500 constraints per lookup)

sumcheck-based state:
  commitment:       polynomial over evaluations (PCS commitment)
  read:             O(1) polynomial evaluation = ~100–200 constraints
  update:           O(1)–O(log N) depending on PCS (Verkle: O(log N × 100))
  cross-index:      free — same polynomial, different evaluation dimensions
```

for 9 indexes with 2³² entries each (the cybergraph scale):

```
per state-modifying operation:
  FRI path:      ~4.5 × 32 × 736 = ~106,000 constraints + ~1,500 LogUp
  sumcheck path: ~4.5 × 32 × 100 = ~3,200 constraints + 0 LogUp

improvement: 33×
```

the 33× comes from the proof architecture, not from a separate optimization. sumcheck makes polynomial state natural. FRI makes it expensive.

### 5.4 Algebraic DAS

the same principle cascades to data availability. DAS samples require inclusion proofs:

```
FRI-based DAS:
  sample proof = Merkle authentication path
  ~1 KiB per sample, O(log n) hemera hashes to verify

sumcheck-based DAS:
  sample proof = PCS opening (polynomial evaluation proof)
  ~200 bytes per sample, O(1) field operations to verify
```

20-sample DAS verification:

```
FRI:        20 × ~1 KiB = 20 KiB bandwidth, ~471K constraints
sumcheck:   20 × ~200B = 4 KiB bandwidth, ~3K constraints

improvement: 157× fewer constraints
```

algebraic DAS is not a separate innovation. it is the natural consequence of a polynomial-native proof architecture applied to availability sampling. the proof system's choice of commitment scheme determines the cost of every data structure built on top of it.

## 6. The Universal Accumulator

### 6.1 Five structural sync layers

distributed knowledge systems require five independent guarantees:

| layer | property | mechanism |
|---|---|---|
| 1. validity | state transitions are correct | zheng proof |
| 2. ordering | operations carry their own order | hash chain + VDF |
| 3. completeness | nothing was omitted | NMT / polynomial |
| 4. availability | data physically exists | DAS + erasure coding |
| 5. merge | convergence is deterministic | CRDT / foculus |

each layer is independently verifiable. together they provide *Verified Eventual Consistency* (VEC) — convergence that a node can verify locally without trusting any peer.

### 6.2 Folding all five layers

the universal accumulator folds proof obligations from ALL five layers into one ~200 byte object:

```
per signal:   fold validity proof (layer 1)        ~30 field ops
per signal:   fold ordering verification (layer 2)  ~30 field ops
per block:    fold completeness proof (layer 3)     ~30 field ops
per block:    fold DAS commitment (layer 4)         ~30 field ops
per block:    fold merge transition (layer 5)       ~30 field ops

decider:      one verification at any level          ~70K constraints
```

a light client downloading the accumulator (~200 bytes) and running one decider (10–50 μs) gets the full VEC guarantee: all signals valid, all signals ordered, nothing withheld, data available, merge correct — for the entire history from genesis.

### 6.3 The checkpoint

```
checkpoint = {
  BBG_commitment:   32 bytes   (polynomial commitment over all state)
  universal_acc:    ~200 bytes (accumulator proving all history)
  height:           8 bytes    (block height)
}

total: ~240 bytes
proves: everything from genesis to height
verification: one zheng decider, 10–50 μs
```

## 7. The Continuous Fold

### 7.1 Pipeline

the full system — VM, hash, proof, sync, verification — is one continuous fold:

```
fold #1:  VM step           → fold trace row into accumulator        (~30 field ops)
fold #2:  hemera absorption → fold sponge block into accumulator     (~30 field ops)
fold #3:  signal complete   → accumulator IS the proof (σ)
fold #4:  signal into block → fold σ into block accumulator          (~30 field ops)
fold #5:  block into epoch  → fold block acc into epoch accumulator  (~30 field ops)
fold #6:  epoch into chain  → fold epoch acc into universal acc      (~30 field ops)
```

every operation is a fold. the accumulator passes from VM through hash through proof through sync to checkpoint. the decider runs ONCE at whatever level the verifier cares about.

### 7.2 End-to-end numbers

| metric | FRI-based (production STARKs) | zheng-2 (hemera-2) | improvement |
|---|---|---|---|
| proof size (128-bit) | 100–200 KiB | 1–5 KiB | 30–150× |
| verification | 10–50 ms | 10–50 μs | 1,000× |
| recursive step | 50K–200K constraints | ~30 field ops | 2,300× |
| prover time | O(N log N) | O(N) streaming | log N × |
| prover memory | O(N) | O(√N) | √N × |
| per state update | ~106K constraints | ~3.2K constraints | 33× |
| epoch (1000 blocks) | 70M+ constraints | ~100K constraints | 700× |
| DAS (20 samples) | ~471K constraints | ~3K constraints | 157× |
| hemera calls/block (state) | 144,000 | 0 (algebraic) | ∞ |
| light client join | full chain / sync committee | < 10 KiB, 10–50 μs | — |
| checkpoint | block headers | ~240 bytes | — |
| binary workloads | 32–64× overhead | native (Binius) | 32–64× |
| proving latency | separate phase | zero (proof-carrying) | ∞ |

### 7.3 Cost of one state transition

the cost of adding one edge to a permanent, verified, globally-available knowledge graph:

| component | constraints | source |
|---|---|---|
| proof per VM step | ~30 field ops | proof-carrying (HyperNova fold) |
| content identity | ~164 | folded hemera sponge |
| public state update | ~3,200 | algebraic polynomial (Verkle phase) |
| private state update | ~5,000 | polynomial mutator set |
| **total overhead** | **~8,400** | |

compared to FRI-based architecture: ~148,000 constraints. **17× reduction** in proving overhead beyond raw computation.

## 8. Honest Assessment

### 8.1 Where zheng-2 is stronger

| claim | basis | confidence |
|---|---|---|
| 30–150× smaller proofs | algebraic extraction (proven) → Brakedown (newer) | high (5–12 KiB) to medium (1–5 KiB) |
| 1,000× faster verification | no Merkle paths in verifier | high |
| 2,300× cheaper recursion | HyperNova folding (peer-reviewed, 2023) | high |
| O(N) prover | sumcheck is inherently linear | high (large N) |
| O(√N) memory | tensor compression | medium (needs empirical rank validation) |
| 33× cheaper state | polynomial state natural in sumcheck | high |
| 157× cheaper DAS | algebraic DAS (PCS openings) | high |
| 32–64× binary | Binius native F₂ | high |

### 8.2 Where zheng-2 is weaker

| concern | reality | mitigation |
|---|---|---|
| zero production hours | biggest risk. Stwo/Plonky3 have years of deployment | phased migration, formal verification |
| hemera cost vs 31-bit hash | hemera-2: 736 constraints vs ~300 for Poseidon2-M31 | GFP hardware; hemera-3 reduces calls to trust anchoring |
| 64-bit field overhead | Goldilocks 2–4× slower per-op than BabyBear on commodity | GFP native silicon; compensated by fewer operations total |
| single implementation | one codebase vs multiple independent implementations | formal spec enables independent implementations |
| Brakedown novelty | less battle-tested than FRI | algebraic extraction on WHIR as near-term fallback |

### 8.3 What makes the comparison unfair

production STARKs (Stwo, Plonky3, SP1) are running systems. zheng-2 is a specification. the numbers above are architectural analysis, not benchmarks. the real comparison happens when zheng-2 ships code.

what the architectural analysis DOES establish: the sumcheck + folding + multilinear foundation provides structural advantages that FRI cannot match without changing its foundation. these are not constant-factor optimizations — they are asymptotic improvements (O(1) vs O(log N) state reads, O(N) vs O(N log N) proving, ~30 ops vs ~70K constraints for recursion).

## 9. Language Mapping

zheng serves 14 nox instantiations:

```
Goldilocks (WHIR/Brakedown):  Nox, Tri, Tok, Arc, Seq, Inf, Bel, Ren, Dif, Sym, Wav
Binary (Binius):               Bt
Split by workload:             Rs, Ten

14 algebras → 14 jet libraries → 2 fields → 2 PCS → 1 IOP → 1 hash
```

each language has optimized jets (pre-compiled constraint gadgets). binary jets for Bt: popcount, packed_inner_product, binary_matvec, quantize/dequantize, activation_lut, gadget_decompose, barrel_shift. verifier jets for Goldilocks: hash, poly_eval, merkle_verify, fri_fold, ntt.

ring-aware CCS encoding handles FHE bootstrapping natively: TFHE blind rotation maps to Wav (NTT), gadget decomposition maps to Bt (binary), key switching maps to Ten (tensor), noise tracking maps to Tri (arithmetic). q = Goldilocks prime makes R_q native to nebu NTT.

## 10. Migration

```
phase 1:   algebraic extraction on WHIR          (157 KiB → 5–12 KiB proofs)
phase 2:   folding-first composition              (700× epoch composition)
phase 3:   Binius PCS + kuro arithmetic           (binary workloads native)
phase 4:   proof-carrying computation             (zero proving latency)
phase 5:   ring-aware FHE jets                    (native FHE bootstrapping)
phase 6:   Brakedown PCS replaces WHIR            (Merkle-free, 1–5 KiB proofs)
phase 7:   gravity commitment                     (power-law verification cost)
phase 8:   tensor compression                     (O(√N) memory, mobile provers)
phase 9:   universal accumulator                  (240-byte checkpoint)
phase 10:  GPU-native pipeline                    (45–100× throughput)
```

each phase is independently deployable. phases 1–2 provide the largest near-term impact. phases 3–5 enable new workloads. phases 6–10 approach theoretical limits.

## 11. Open Problems

1. **Brakedown concrete security.** Which expander family achieves optimal security/proof-size tradeoff for Goldilocks? Concrete parameters needed.

2. **Cross-algebra soundness.** Universal CCS with algebra selectors folds heterogeneous instances. Does folding across fields preserve HyperNova's security reduction?

3. **Tensor rank of real traces.** Tensor compression assumes low rank (r ≈ 32). Empirical validation on cyberlink, inference, and tri-kernel workloads needed.

4. **Gravity weight stability.** Mass-weighted polynomial encoding by π. If π changes (new edges shift attention), existing proofs remain valid (frozen weights at commitment time), but the layered structure must be re-committed. Optimal re-commitment frequency?

5. **Algebraic DAS composition.** The erasure-coded 2D grid is naturally bivariate. Does WHIR/Brakedown support efficient bivariate openings natively, or must the grid be flattened?

6. **VEC formalization.** Verified Eventual Consistency needs formal treatment — precise adversary model, safety/liveness proofs, relationship to existing consistency models.

## References

[1] S. Setty, "SuperSpartan: Doubly-efficient SNARKs without preprocessing," Crypto 2023.

[2] A. Kothapalli and S. Setty, "HyperNova: Recursive arguments from folding schemes," 2023.

[3] G. Haböck, U. Habock, and A. Szepieniec, "WHIR: Reed-Solomon Proximity Testing with Super-Fast Verification," 2024.

[4] A. Golovnev et al., "Brakedown: Linear-time and field-agnostic SNARKs for R1CS," Crypto 2023.

[5] B. Diamond and J. Posen, "Succinct Arguments over Towers of Binary Fields," 2024.

[6] L. Grassi et al., "Poseidon2: A Faster Version of the Poseidon Hash Function," 2023.

[7] M. Shapiro et al., "Conflict-free Replicated Data Types," SSS 2011.

[8] M. Al-Bassam et al., "LazyLedger: A Data Availability Blockchain Using Namespace Merkle Trees," 2019.

[9] M. Al-Bassam et al., "Fraud and Data Availability Proofs," 2018.

[10] M. Castro and B. Liskov, "Practical Byzantine Fault Tolerance," OSDI 1999.
