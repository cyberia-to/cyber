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
├── PCS₁:         Brakedown (Goldilocks F_p, Merkle-free)
├── PCS₂:         Binius (F₂ tower)
├── Constraints:  CCS (Customizable Constraint System)
└── Languages:    14 nox instantiations → 2 fields → 2 PCS → 1 IOP → 1 hash
```

one hash. two fields. two PCS backends (both Merkle-free). one IOP. one folding scheme. 14 source languages.

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

### 3.1 Brakedown (Goldilocks, primary)

Brakedown commits to multilinear polynomials via expander-graph linear codes. no Merkle tree. no hemera in the PCS at all. the bottleneck shifts from hashing to field arithmetic — exactly where Goldilocks and nebu excel.

```
commitment:    linear code encoding via expander graph (no tree)
opening:       O(√N) proximity proof
proof size:    1–5 KiB
verification:  O(√N) field operations
prover:        O(N) linear time (code encoding is linear)
security:      transparent, post-quantum, code-based

key property:  Merkle tree eliminated entirely
               hemera calls for PCS: 0
               proof cost dominated by field arithmetic (nebu)
               not by hash computation (hemera)
```

Brakedown's linear-time commitment is native to the sumcheck architecture: both are O(N). the prover never leaves the field — no hash tree construction, no NTT for evaluation domain, no O(N log N) bottleneck anywhere in the pipeline.

### 3.2 WHIR (bootstrap path)

WHIR commits via hemera Merkle trees over evaluation domains. it is the zheng-1 backend and serves as the bootstrap path before Brakedown is production-ready.

```
commitment:    hemera Merkle tree over evaluations
proof size:    60–157 KiB (77% is Merkle paths)
verification:  ~1 ms
```

WHIR's Merkle paths are its weakness. algebraic extraction (batch algebraic opening via quotient polynomial) reduces proof size to 5–12 KiB without changing the PCS:

```
algebraic extraction:
  replace k Merkle openings with one quotient Q(x) = (f(x) - I(x)) / Z(x)
  proof: sumcheck transcript + quotient commitment + one evaluation
  size: 5–12 KiB (vs 60–157 KiB)
  verification: ~150 μs (vs ~1 ms)
```

the migration path: WHIR → WHIR + algebraic extraction → Brakedown. each step is independently deployable. Brakedown is the target architecture.

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

## 8. The Landscape (Early 2026)

| system | field | PCS | proof size | verification | prover | status |
|---|---|---|---|---|---|---|
| Stwo (StarkWare) | M31 | FRI | ~200 KiB | 10–50 ms | O(N log N) | production (Starknet) |
| Plonky3 (Polygon) | Goldilocks | FRI | 100–200 KiB | 10–50 ms | O(N log N) | production (zkEVM) |
| SP1 (Succinct) | BabyBear | FRI | ~150 KiB | ~30 ms | O(N log N) | production (zkVM) |
| RISC Zero | BabyBear | FRI | ~200 KiB | ~50 ms | O(N log N) | production (Bonsai) |
| Binius (Irreducible) | F₂ tower | sumcheck | research | research | O(N) | early research |
| **zheng** | **Goldilocks + F₂** | **Brakedown + Binius** | **1–5 KiB** | **10–50 μs** | **O(N)** | **spec draft** |

every production system shares: FRI-based univariate polynomial commitment with Merkle trees.

zheng is structurally different:
- **multilinear** (not univariate) — SuperSpartan with multilinear extensions
- **sumcheck** (not FRI) — inherently O(N), no FFT
- **Brakedown** (not Merkle) — expander-graph codes, no tree
- **folding** (not recursive verification) — HyperNova, ~30 ops vs ~70K constraints
- **dual PCS** — Brakedown for Goldilocks, Binius for F₂
- **one hash** — hemera for trust anchoring, not for proof internals

## 9. Honest Assessment

### 9.1 Where zheng is stronger

| claim | basis | real? | confidence |
|---|---|---|---|
| 30–150× smaller proofs | Brakedown: expander codes, no Merkle paths | yes — O(√N) proof, no tree overhead | medium-high (Brakedown newer than FRI) |
| 1,000× faster verification | no Merkle paths in verifier, O(√N) field ops | yes — field ops only, no hashing | high |
| 2,300× cheaper recursion | HyperNova folding (peer-reviewed, 2023) | yes — accumulate, don't verify | high |
| O(N) prover | sumcheck + Brakedown both linear | yes — mathematically inherent | high (at large N) |
| O(√N) memory | tensor compression of structured traces | plausible — needs empirical rank | medium |
| 33× cheaper state | polynomial state natural in sumcheck | yes — architecture, not optimization | high |
| 157× cheaper DAS | PCS openings replace NMT paths | yes — follows from Brakedown | high |
| 32–64× binary | Binius native F₂, 128 elements/u128 | yes — algebraic distance F_p vs F₂ | high |

### 9.2 Where zheng is weaker

| concern | severity | reality | mitigation |
|---|---|---|---|
| zero production hours | critical | Stwo/Plonky3: years of deployment, hundreds of bugs found and fixed. zheng: zero code, zero audits, zero bugs found because zero code | phased migration (WHIR → Brakedown), formal verification, spec-first development |
| hemera cost | medium | hemera-2: ~736 constraints/perm. Poseidon2 over M31: ~300. 2.4× more expensive per hash. this is the cost of 64-bit field + 256-bit security designed for century-scale content addressing | GFP hardware makes Goldilocks native. hemera-3 reduces remaining calls to trust anchoring only. Brakedown eliminates hemera from PCS entirely |
| 64-bit field overhead | medium | Goldilocks: 2–4× slower per field operation than BabyBear/M31 on commodity x86. constant factor, not asymptotic | GFP: custom silicon where 64-bit is native. zheng compensates by doing FEWER total operations (sumcheck O(N) vs FRI O(N log N), Brakedown O(N) vs FRI Merkle O(N log N)) |
| Brakedown maturity | medium | less battle-tested than FRI. fewer implementations, fewer audits, fewer years of cryptanalysis | WHIR bootstrap path as fallback. algebraic extraction gives 5–12 KiB proofs on WHIR if Brakedown issues arise |
| single implementation | low-medium | one planned codebase vs 4+ independent FRI implementations | formal spec enables independent implementations. Rs deterministic language enforces correctness by construction |

### 9.3 What makes the comparison unfair

production STARKs are running systems. zheng is a specification. the numbers above are architectural analysis, not benchmarks. the real comparison happens when zheng ships code.

what the architectural analysis DOES establish: sumcheck + folding + Brakedown provides structural advantages that FRI cannot match without changing its foundation. these are asymptotic improvements, not constant factors:

```
state read:     O(1) field ops    vs  O(log N) hemera hashes      (asymptotic)
prover time:    O(N) linear       vs  O(N log N) FFT               (asymptotic)
recursion:      ~30 field ops     vs  ~70K constraints              (2,300×)
proof internal: 0 hemera calls    vs  O(N log N) hemera calls       (architectural)
```

a FRI-based system optimizing constants will always lose to a sumcheck-based system at sufficient scale. the question is whether that scale arrives before zheng achieves production maturity.

### 9.4 What zheng enables that FRI cannot

1. **provable consensus.** tri-kernel π computation = 1.42B constraint circuit. zheng at 33% capacity handles this. FRI-based systems need 4× larger proofs for the same circuit. validators compute π, not vote — consensus becomes a mathematical fixed point.

2. **polynomial state.** state reads are O(1) field operations in sumcheck. state IS a polynomial, and the proof system operates on polynomials natively. FRI-based systems pay O(log N) hemera hashes per state read because FRI commits via Merkle trees. section 5 details why this is architectural, not optimizable.

3. **algebraic data availability.** DAS samples are PCS openings (~200 bytes) instead of Merkle paths (~1 KiB). 157× fewer constraints for 20-sample verification. again: architecture, not optimization.

4. **mobile proving.** O(√N) prover memory via tensor compression. a phone with 4 GB RAM proves computations that require 64 GB in FRI-based provers.

5. **zero-latency recursion.** HyperNova folds during execution. no separate proving step. proof-carrying computation: the proof is ready when the computation finishes.

6. **unified mining.** one primitive set (nebu field arithmetic) serves both signal proof and consensus computation. FRI provers are NTT-biased — different hardware for proving vs execution. zheng's sumcheck + Brakedown are both linear — same hardware profile throughout.

## 10. Language Mapping

zheng serves 14 nox instantiations:

```
Goldilocks (WHIR/Brakedown):  Nox, Tri, Tok, Arc, Seq, Inf, Bel, Ren, Dif, Sym, Wav
Binary (Binius):               Bt
Split by workload:             Rs, Ten

14 algebras → 14 jet libraries → 2 fields → 2 PCS → 1 IOP → 1 hash
```

each language has optimized jets (pre-compiled constraint gadgets). binary jets for Bt: popcount, packed_inner_product, binary_matvec, quantize/dequantize, activation_lut, gadget_decompose, barrel_shift. verifier jets for Goldilocks: hash, poly_eval, merkle_verify, fri_fold, ntt.

ring-aware CCS encoding handles FHE bootstrapping natively: TFHE blind rotation maps to Wav (NTT), gadget decomposition maps to Bt (binary), key switching maps to Ten (tensor), noise tracking maps to Tri (arithmetic). q = Goldilocks prime makes R_q native to nebu NTT.

## 11. Migration

```
phase 1:   WHIR + algebraic extraction            bootstrap (157 KiB → 5–12 KiB)
phase 2:   Brakedown replaces WHIR                target PCS (Merkle-free, 1–5 KiB)
phase 3:   folding-first composition              700× epoch composition
phase 4:   Binius PCS + kuro arithmetic           binary workloads native
phase 5:   proof-carrying computation             zero proving latency
phase 6:   ring-aware FHE jets                    native FHE bootstrapping
phase 7:   gravity commitment                     power-law verification cost
phase 8:   tensor compression                     O(√N) memory, mobile provers
phase 9:   universal accumulator                  240-byte checkpoint
phase 10:  GPU-native pipeline                    45–100× throughput
```

Brakedown moves to phase 2 — it is the target architecture, not a late optimization. WHIR + algebraic extraction is the bootstrap path only. once Brakedown is validated, hemera exits the PCS entirely. each subsequent phase builds on Brakedown's Merkle-free foundation.

## 12. Open Problems

1. **Brakedown expander construction.** Brakedown's proof size and security depend on the expander graph family. which family achieves optimal security/proof-size tradeoff for the Goldilocks field? concrete parameters for 128-bit security needed. this is the most critical near-term research question — Brakedown is the target PCS.

2. **Brakedown bivariate openings.** algebraic DAS requires efficient openings on bivariate erasure-coded grids. does Brakedown support bivariate polynomial evaluation natively via its linear code structure, or must the 2D grid be flattened to univariate? flattening works but may lose the 2D structure that makes DAS fraud proofs efficient.

3. **Cross-algebra soundness.** universal CCS with algebra selectors folds heterogeneous instances (F_p and F₂). does folding across fields preserve HyperNova's security reduction? formal proof needed.

4. **Tensor rank of real traces.** tensor compression assumes low rank (r ≈ 32) for nox execution traces. empirical validation on cyberlink insertion, quantized inference, and tri-kernel SpMV workloads needed. if r >> 32, tensor compression may not help.

5. **Gravity weight stability.** mass-weighted polynomial encoding by π. if π changes (new edges shift attention), existing proofs remain valid (frozen weights at commitment time), but the layered structure must be re-committed. optimal re-commitment frequency?

6. **VEC formalization.** Verified Eventual Consistency needs formal treatment — precise adversary model, safety/liveness proofs, relationship to existing consistency models (SEC, causal+, linearizability).

7. **Brakedown + Binius unification.** both Brakedown and Binius are linear-code-based PCS. can the two backends share infrastructure (code construction, proximity testing) despite operating over different fields? a unified linear-code PCS framework could simplify the dual-algebra architecture.

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
