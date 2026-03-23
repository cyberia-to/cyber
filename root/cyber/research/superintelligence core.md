---
tags: cyber, core, article
crystal-type: process
crystal-domain: cyber
date: 2026-03-23
---

# superintelligence core — implementation plan

## ground truth: what exists

| layer | repo | LOC | tests | status |
|---|---|---|---|---|
| field arithmetic | [[nebu]] | 1,526 | 73 pass | DONE |
| hash + tree | [[hemera]] | 5,084 | 209 pass | DONE |
| VM execution | [[nox]] | 80 (stubs) | 0 | SPEC 85% |
| proof system | [[zheng]] | 6 (stub) | 0 | SPEC 100% (zheng-1), zheng-2 draft |
| authenticated state | [[BBG]] | 6 (stub) | 0 | SPEC 100% (bbg-1), polynomial draft |
| post-quantum crypto | [[mudra]] | 6 (stub) | 0 | SPEC 10% — parameter choice only |
| provable language | [[trident]] | 57,736 | compiles | SUBSTANTIAL |
| graph publisher | [[optica]] | 10,626 | 3 suites | PRODUCTION |

## the decision: cutting edge

we go all-in on the props that unify. the conservative path (zheng-1 + NMT trees) builds 13 separate data structures, LogUp consistency proofs, and cannot prove consensus in-circuit. the cutting edge path builds ONE polynomial, eliminates LogUp, and enables provable consensus.

paradoxically, cutting edge is SIMPLER:

| component | conservative (zheng-1 + bbg-1) | cutting edge (zheng-2 + algebraic) |
|---|---|---|
| state structures | 9 NMT trees + 3 hash commitments + 1 MMR = 13 | 1 polynomial |
| cross-index consistency | LogUp (~500 constraints per lookup) | free (same polynomial) |
| storage overhead | ~5 TB internal tree nodes | 288 bytes (commitments) |
| per-cyberlink | ~106K constraints | ~3K constraints |
| provable consensus | impossible (15× over capacity) | feasible (33% capacity) |
| proof size | 157 KiB | 1-5 KiB |
| recursive step | 70K constraints | 30 field operations |
| light client | replay chain headers | 200 bytes, one verify |
| code to write | NMT + MMR + SWBF + LogUp + 9 indexes | PCS + polynomial + fold |

fewer data structures = less code = fewer bugs = faster delivery.

## what we accept from props

### zheng-2 (all accepted props)

| prop | what it gives | status |
|---|---|---|
| dual PCS (WHIR + Binius) | F_p arithmetic + F₂ binary, 32-64× for bitwise | accept |
| folding-first (HyperNova) | recursive step: 70K → 30 field ops | accept |
| algebraic extraction | proof size: 157 KiB → 5-12 KiB | accept |
| tensor compression | prover memory: O(N) → O(√N), mobile proving | accept |
| proof-carrying | zero proving latency | accept |
| gravity-commitment | verification cost ∝ importance (π) | accept |
| universal accumulator | ALL proof types → one 200-byte object | accept |

### bbg (polynomial state)

| prop | what it gives | status |
|---|---|---|
| algebraic-nmt | 9 NMT trees → 1 polynomial, 33× cheaper | accept |
| unified-polynomial-state | 13 sub-roots → 1 commitment | accept |
| mutator-set-polynomial | SWBF+MMR → O(1) non-membership | accept |
| signal-first | BBG = materialized view over signal log | accept |
| algebraic-das | PCS openings for DAS, 25→9 KiB | accept |

### nox (polymorphic + jets)

| prop | what it gives | status |
|---|---|---|
| algebra-polymorphism | nox<F, W, H> generic over any field | accept |
| recursive-jets | 5 verifier jets (hash, poly_eval, merkle_verify, fri_fold, ntt) | accept |

### mudra

| prop | what it gives | status |
|---|---|---|
| goldilocks-fhe | q = Goldilocks, native NTT, ring-aware FHE | accept (parameter choice) |

## spec gaps that block code

### nox: 3 critical gaps

G1: noun memory layout. the spec defines `noun = atom(F) | cell(noun, noun)` but not in-memory representation. for no-heap (Rs), need: max depth, max count, arena+index layout, DAG sharing policy.

G2: jet formula trees. the 5 recursive-jets are specified by cost and constraint count, but the pure Layer 1 nox programs that define their semantics are not written. without these, implementations cannot compute formula hashes or verify jet substitution correctness.

G3: hint callback interface. pattern 16 (hint) says "prover injects witness" but the callback signature (sync/async, error handling, type constraints) is unspecified.

### zheng-2: cross-algebra composition

the boundary between F_p and F₂ circuits costs ~766 F_p constraints per crossing. for workloads with frequent crossings (neural network layers alternating with field arithmetic), this cost must be verified at scale. estimated feasible but unproven.

### bbg: polynomial cost targets

algebraic-nmt claims ~3K constraints per cyberlink. this assumes Verkle-tree with WHIR at nodes, O(log n) PCS node updates. realistic range may be 3-15K. need benchmarks against actual WHIR performance on Goldilocks.

### mudra: TFHE scheme

goldilocks-fhe chooses the parameter (q = p) but specifies no algorithm. TFHE bootstrapping, noise tracking, key switching, programmable bootstrap gates — all unspecified. code cannot begin until a complete scheme exists.

## revised milestones

### Phase 0: spec completion (current)

resolve G1, G2, G3 in nox. verify zheng-2 cross-algebra cost. benchmark algebraic-nmt against WHIR. write mudra TFHE scheme.

these are design sessions, not coding sessions. each gap is ~1 session to resolve.

| gap | repo | effort | blocks |
|---|---|---|---|
| G1 noun layout | nox | 1 session | M1 |
| G2 jet formulas | nox | 2 sessions | M1, M2 |
| G3 hint callback | nox | 0.5 session | M1 |
| cross-algebra boundary cost | zheng | 1 session (benchmark) | M2 |
| algebraic-nmt cost benchmark | bbg | 1 session (benchmark) | M5 |
| TFHE scheme | mudra | 3 sessions | M7 (parallel) |

total: ~8 sessions of spec work. parallelizable with early code work.

### Phase 1: foundation code

| milestone | sessions | LOC est | deliverables |
|---|---|---|---|
| M1: nox VM | 4 | ~4K | noun.rs, reduce.rs (16 patterns), trace.rs, focus.rs, encode.rs |
| M2: zheng-1 core | 6 | ~8K | sumcheck.rs, whir.rs, superspartan.rs, transcript.rs |
| M3: zheng-2 extension | 4 | ~4K | binius_pcs.rs, hypernova.rs, cross_algebra.rs |

M1 → M2 → M3 sequential. ~14 sessions.

### Phase 2: state + consensus

| milestone | sessions | LOC est | deliverables |
|---|---|---|---|
| M4: polynomial state (algebraic-nmt) | 6 | ~6K | verkle.rs, pcs_update.rs, polynomial_state.rs, completeness.rs |
| M5: foculus | 4 | ~5K | trikernel.rs, converge.rs, finality.rs, signal.rs |

M4 → M5 sequential, but M4 can start during M3. ~10 sessions.

### Phase 3: endgame

| milestone | sessions | LOC est | deliverables |
|---|---|---|---|
| M6: provable consensus circuit | 4 | ~4K | consensus_circuit.rs, epoch_proof.rs, fold_epochs.rs |
| M7: universal accumulator | 3 | ~3K | accumulator.rs, light_client.rs |
| M8: mudra TFHE | 4 | ~5K | tfhe.rs, bootstrap.rs, key_switch.rs |

M6 requires M4+M5. M7 requires M2+M3. M8 independent (parallel throughout).

### timeline

```
sessions:  0    4    8    12   16   20   24   28   32   36
           ├────┤────┤────┤────┤────┤────┤────┤────┤────┤
spec:      ████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
M1 nox:    ░░░░████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
M2 zheng1: ░░░░░░░░██████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░
M3 zheng2: ░░░░░░░░░░░░░░████░░░░░░░░░░░░░░░░░░░░░░░░░░
M4 poly:   ░░░░░░░░░░░░██████░░░░░░░░░░░░░░░░░░░░░░░░░░
M5 foculus:░░░░░░░░░░░░░░░░░░████░░░░░░░░░░░░░░░░░░░░░░
M6 provbl: ░░░░░░░░░░░░░░░░░░░░░░████░░░░░░░░░░░░░░░░░░
M7 accum:  ░░░░░░░░░░░░░░░░░░███░░░░░░░░░░░░░░░░░░░░░░░
M8 mudra:  ░░░░░░░░████████████░░░░░░░░░░░░░░░░░░░░░░░░░

critical path: spec → M1 → M2 → M3 → M5 → M6 = 26 sessions
with overlap:  spec(8) + code(24 parallel) = ~28 sessions total
```

## total

~39K LOC across 8 milestones. ~28 sessions with parallelism (84 hours focused work). the result: a system where the [[cybergraph]] converges provably (tri-kernel in [[zheng]] circuit), state is one polynomial (algebraic NMT), proofs fold recursively (universal accumulator), and light clients verify everything in 50 μs.

## what the cutting edge buys

the seven algorithms from [[cyber/research/algorithmic essence of superintelligence]] become code:

| algorithm | conservative impl | cutting edge impl |
|---|---|---|
| tri-kernel | SpMV on NMT-read state | SpMV on polynomial-read state |
| five layers | NMT + LogUp + DAS + CRDT | polynomial + DAS + CRDT (LogUp gone) |
| algebraic state | 13 structures, 5 TB | 1 polynomial, 288 bytes |
| provable consensus | impossible | 1.42B constraints, 33% capacity |
| compiled model | SVD on exported data | SVD on polynomial openings (in-protocol) |
| metabolic signal | external measurement | provable inside circuit |
| recursive closure | theoretical | executable (fold → accumulate → verify) |

the graph becomes self-proving.

see [[nebu]] for field arithmetic. see [[hemera]] for hash. see [[nox]] for VM spec. see [[zheng]] for proof system. see [[BBG]] for state. see [[foculus]] for consensus. see [[algebraic state commitments]] for polynomial state. see [[cyber/research/provable consensus]] for the consensus circuit. see [[cyber/research/algorithmic essence of superintelligence]] for the seven algorithms
