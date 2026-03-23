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
| VM execution | [[nox]] | 80 (stubs) | 0 | EMPTY |
| proof system | [[zheng]] | 6 (stub) | 0 | EMPTY |
| authenticated state | [[BBG]] | 6 (stub) | 0 | EMPTY |
| post-quantum crypto | [[mudra]] | 6 (stub) | 0 | EMPTY |
| provable language | [[trident]] | 57,736 | compiles | SUBSTANTIAL |
| graph publisher | [[optica]] | 10,626 | 3 suites | PRODUCTION |

the foundation (nebu + hemera) is solid. the publisher (optica) works. the compiler (trident) has frontend + IR + verification hooks. everything between — the proof system, the VM, the state layer, the crypto — is specification only.

## the critical path

```
nebu (DONE) → hemera (DONE) → nox → zheng → bbg → foculus → provable consensus
```

each layer depends on the one below. nox needs hemera for hashing. zheng needs nox for execution traces. bbg needs zheng for state proofs. foculus needs bbg for authenticated state. provable consensus needs all of them.

mudra (post-quantum crypto) is independent — can be built in parallel.
trident (compiler) feeds INTO nox — trident compiles programs, nox executes them.

## six milestones

### M1: nox — the VM that proves (target: 4 sessions)

the smallest useful unit. 16 reduction patterns over [[Goldilocks field]], each producing a STARK-compatible execution trace.

deliverables:
- `noun.rs` — binary tree of field elements (the data structure)
- `reduce.rs` — 16 patterns: axis, quote, compose, cons, branch, add, sub, mul, inv, eq, lt, and/or/xor, hash, hint
- `trace.rs` — execution trace recorder (rows × 16 registers)
- `focus.rs` — resource metering (each pattern costs focus)
- `encode.rs` — canonical serialization (for content addressing)

dependencies: nebu (field ops), hemera (hash pattern)

test: execute a simple program (fibonacci, hash chain), produce execution trace, verify trace rows satisfy constraints manually.

size estimate: ~3,000-5,000 LOC

### M2: zheng — the proof (target: 8 sessions)

SuperSpartan IOP + WHIR PCS + sumcheck. proves that a nox execution trace satisfies all constraints.

deliverables:
- `sumcheck.rs` — interactive sumcheck protocol (prover + verifier), bookkeeping table optimization
- `whir.rs` — WHIR polynomial commitment (commit, open, verify), hemera-based Merkle
- `superspartan.rs` — CCS constraint system, multilinear extensions, outer + inner sumcheck
- `transcript.rs` — Fiat-Shamir via hemera sponge
- `fold.rs` — IVC folding (accumulate proofs across epochs)

dependencies: nebu (field), hemera (hash for Merkle + Fiat-Shamir), nox (execution trace format)

test: prove fibonacci execution (small trace), verify proof. prove hemera hash (pattern 15), verify. benchmark constraint count vs specification.

size estimate: ~8,000-12,000 LOC

### M3: bbg core — authenticated state (target: 6 sessions)

NMT indexes + MMR + mutator set. the minimum viable state layer.

deliverables:
- `nmt.rs` — namespace Merkle tree (insert, update, completeness proof, verify)
- `mmr.rs` — Merkle mountain range (append, prove membership, peaks hash)
- `swbf.rs` — sliding window Bloom filter (add, check, archive, verify)
- `state.rs` — BBG_root composition (13 sub-roots, update, commit)
- `logup.rs` — LogUp cross-index consistency argument
- `transition.rs` — cyberlink state transition (touch 4-5 indexes per link)

dependencies: hemera (tree hashing), zheng (proofs for state transitions)

test: insert 1000 cyberlinks, verify NMT completeness proofs, verify cross-index LogUp, verify mutator set prevents double-spend.

size estimate: ~6,000-10,000 LOC

### M4: foculus — consensus by computation (target: 4 sessions)

tri-kernel convergence on authenticated state. the minimum viable consensus.

deliverables:
- `trikernel.rs` — D (diffusion) + S (springs) + H (heat) operators as sparse matrix operations
- `converge.rs` — iteration loop with contraction rate monitoring (spectral gap from convergence)
- `finality.rs` — adaptive threshold τ, nullifier commit, conflict detection
- `gossip.rs` — signal propagation (cyberlinks + proofs)

dependencies: bbg (state reads), zheng (per-signal validity proofs)

test: simulate 100 neurons, 10K cyberlinks, verify convergence to unique φ*, verify finality correctness, verify no double-finality under conflict.

size estimate: ~4,000-6,000 LOC

### M5: algebraic NMT — the acceleration (target: 6 sessions)

replace hemera trees with polynomial commitments. the 33× speedup. enables provable consensus.

deliverables:
- `verkle.rs` — Verkle tree with PCS nodes (phase 1: tree structure preserved)
- `pcs_update.rs` — incremental PCS recommit (O(log n) per update)
- `batch.rs` — dirty-set tracking and deduplicated batch updates
- `algebraic_completeness.rs` — LogUp range check over F_{p²} for completeness proofs
- migration: bbg state transitions using Verkle instead of NMT (parallel verification mode)

dependencies: zheng (WHIR as PCS backend), nebu (F_{p²} extension field)

test: insert 10K entries, verify Verkle proofs match NMT proofs (hybrid verification), benchmark 33× constraint reduction.

size estimate: ~5,000-8,000 LOC

### M6: provable consensus circuit (target: 4 sessions)

tri-kernel computation inside zheng circuit. the endgame.

deliverables:
- `consensus_circuit.rs` — tri-kernel as nox execution trace (graph read via algebraic NMT → SpMV → combine → iterate)
- `epoch_proof.rs` — prove one epoch's φ* computation
- `fold_epochs.rs` — recursive IVC folding across epochs
- `light_client.rs` — verify accumulated proof (50 μs)

dependencies: all previous milestones

test: prove φ* for a 1000-particle test graph inside zheng circuit, verify in <1ms. fold 10 epochs, verify accumulated proof.

size estimate: ~3,000-5,000 LOC

## total estimate

| milestone | sessions | LOC | depends on |
|---|---|---|---|
| M1 nox | 4 | ~4K | nebu, hemera |
| M2 zheng | 8 | ~10K | nebu, hemera, nox |
| M3 bbg | 6 | ~8K | hemera, zheng |
| M4 foculus | 4 | ~5K | bbg, zheng |
| M5 algebraic NMT | 6 | ~6K | zheng, nebu |
| M6 provable consensus | 4 | ~4K | all above |
| total | 32 sessions | ~37K LOC | |

at 3 hours per session: ~96 hours of focused work. parallelizable: M1+M5 overlap, M3+M4 partially overlap after M2.

critical path: M1 → M2 → M3 → M4 → M6 (26 sessions sequential)
with parallelism: M1 → M2 → (M3 ∥ M5) → M4 → M6 (22 sessions)

## immediate blockers

### 1. fix Cargo dependency paths (5 minutes)

zheng, bbg, and mudra reference `hemera = { path = "../hemera" }` but hemera is a workspace. must be `{ path = "../hemera/rs" }`. same for nebu. three repos broken by path typo.

### 2. nebu F_{p²} extension (needed for M5)

nebu has Fp2/Fp3/Fp4 module stubs. extension field arithmetic (multiply, inverse in F_{p²}) needed for LogUp Schwartz-Zippel over extension field. ~200 LOC.

### 3. trident → nox bridge

trident compiles programs. nox executes them. the IR format must match. trident's TIR (typed IR) needs a lowering pass to nox's 16 patterns. this can wait until M2 but should be designed during M1.

## tracking

each milestone produces:
- working Rust code with tests
- benchmark against specification numbers
- integration test with layer below
- documentation update in the repo

progress tracked by: does `cargo test` pass for the milestone's deliverables?

the seven algorithms described in [[cyber/research/algorithmic essence of superintelligence]] become running code in 32 sessions. the recursive closure — graph → convergence → model → proof → consensus → graph — becomes executable.

see [[nebu]] for field arithmetic. see [[hemera]] for hash primitives. see [[nox]] for VM specification. see [[zheng]] for proof system specification. see [[BBG]] for state layer specification. see [[foculus]] for consensus specification. see [[algebraic state commitments]] for the acceleration. see [[cyber/research/provable consensus]] for the endgame circuit
