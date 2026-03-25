---
tags: cyber, research, article, core
crystal-type: article
crystal-domain: cyber
date: 2026-03-25
---
# bootstrap plan

implementation plan for the [[cyber]] execution stack. two tracks: Rust (reference + prover, forever) and [[Trident]] (provable, compiles to [[nox]]). dual-language invariant: both produce identical output on all inputs. divergence = bug.

## current state

| repo | status | LOC | tests |
|------|--------|-----|-------|
| [[nebu]] | DONE | 1,526 Rust | 73 |
| [[hemera]] | DONE | 5,084 Rust | 202 |
| [[jali]] | DONE | 1,442 Rust | 70 |
| [[kuro]] | DONE | 1,157 Rust | 77 |
| [[trop]] | DONE | 1,567 Rust | 77 |
| [[genies]] | DONE | 1,997 Rust | 55 |
| [[nox]] | SPEC COMPLETE | 35 Rust (stubs) | — |
| [[zheng]] | SPEC COMPLETE | 6 Rust (stub) | — |
| [[bbg]] | SPEC COMPLETE | 7 Rust (stub) | — |
| [[mudra]] | SPEC COMPLETE | 6 Rust (stub) | — |
| [[Trident]] | 57K Rust | 236 source files, 24 VM targets | working compiler |

arithmetic layer: 12,773 LOC, 554 tests. all six repos compile and pass tests.

Trident has a nock target (tree architecture, Goldilocks field, level 3). nox target needed: same tree architecture, hemera hash (not Tip5), 4-element digest (not 5).

## dependency graph

```
DONE (arithmetic):
  nebu (F_p) → hemera (hash)
  jali (R_q), kuro (F₂), trop (min,+), genies (F_q)

TO BUILD:
  nox (VM)  → trident (nox target) → trident self-hosts
                                         ↓
                                    hemera.td (provable hash)
                                         ↓
                              zheng verifier.td + Rust prover
                                         ↓
                              jet formulas via trident
                                         ↓
                    ┌────────────────┬────────────────┐
                 mudra.td         bbg.td          tru.td
                    └────────────────┴────────────────┘
```

## checkpoints (freeze points)

each checkpoint = "stop iterating, build on top." before mainnet: unfreezing = rebuild everything above (days). after mainnet: unfreezing = catastrophic rehash (hours across 10^6 nodes).

```
CHECKPOINT 0: nox patterns             ← before Phase 1
  16 pattern semantics, tag numbering, noun model (atom|cell),
  focus cost model, hint interface, arena bounds (depth 64, count 2^24)

CHECKPOINT 1: trident language          ← before Phase 3
  syntax, type system, compilation strategy
  (stable, not frozen — can evolve with re-self-host)

CHECKPOINT 2: hemera parameters         ← before Phase 4
  permutation (t=16, Rp=16, Rf=8, d=x⁻¹), constants, sponge mode,
  output size (32 bytes), capacity layout
  (additive extensions safe: erasure coding, capacity typing)

CHECKPOINT 3: zheng protocol            ← before Phase 5
  CCS format, SuperSpartan IOP, Brakedown PCS, HyperNova folding,
  Fiat-Shamir transcript, proof format

CHECKPOINT 4: jet formulas              ← before Phase 6
  H(formula) for each jet — protocol constants
  (depends on checkpoints 1 + 2)
```

## two tracks

```
Rust track (reference + prover):        Trident track (provable):
  permanent, fast, NOT proven              compiles to nox, produces proofs

  nebu       ✓ done                        —
  hemera     ✓ done                        hemera.td
  jali       ✓ done                        —
  kuro       ✓ done                        —
  trop       ✓ done                        —
  genies     ✓ done                        —
  nox VM     → Phase 1                     —
  trident    → Phase 2 (nox target)        trident.td (Phase 3, self-hosting)
  zheng      → prover (Rust, forever)      verifier.td (Phase 5)
  mudra      → reference tests             mudra.td (Phase 7)
  bbg        → reference tests             bbg.td (Phase 8)
  tru        → reference simulation        tru.td (Phase 9)
```

Rust arithmetic (nebu, jali, kuro, trop, genies) stays forever — hardware-close, optimized. zheng PROVER stays Rust — generates witnesses, not proven. nox VM stays Rust — IS the interpreter. everything else moves to Trident.

## Phase 1: nox VM (Rust kernel)

the irreducible Rust — nox cannot interpret itself.

| file | what | est. LOC |
|------|------|----------|
| noun.rs | arena + hash-consing + structural hash via hemera | ~250 |
| reduce.rs | 16 pattern dispatch + focus deduction | ~350 |
| focus.rs | focus metering, cost table | ~50 |
| hint.rs | HintProvider trait, sync callback, NoHint → Halt | ~80 |
| encode.rs | wire format, content-addressed store trait | ~120 |
| memo.rs | (H(object), H(formula)) → H(result) cache | ~60 |
| trace.rs | TraceRow recording per reduce() call | ~30 |
| jet.rs | jet registry: formula hash → native fn, empty initially | ~60 |
| lib.rs | ask() = memo check + reduce + record | ~40 |

~1,040 LOC Rust. Layer 1 only, no jets.

exit criterion: all test vectors from patterns.md pass.

**CHECKPOINT 0 must pass before starting.**

est: 2-3 sessions

## Phase 2: Trident nox target

add nox as compilation target to Trident. the nock target (tree architecture, level 3) is the starting point — change hash to hemera, digest to 4 elements.

| step | what | est. LOC |
|------|------|----------|
| 2a | vm/nox/target.toml (hemera, 4-element digest) | ~50 |
| 2b | replace Tip5 references with hemera in tree lowering | ~200 |
| 2c | NounBuilder: AST → nox noun (extend tree lowering) | ~800 |
| 2d | os/cyber/ type definitions (Particle, Neuron, Cyberlink) | ~200 |
| 2e | pipeline integration: `trident build --target nox` | ~250 |

~1,500 LOC Rust.

exit criterion: `trident build --target nox fibonacci.td` → valid nox noun → `nox_eval(input, noun, focus)` → correct result.

est: 3-4 sessions

## Phase 3: Trident self-hosting

write the Trident compiler IN Trident. the compiler itself becomes a nox program.

```
trident_source ──Rust compiler──→ nox_noun_v1
trident_source ──nox_eval(source, nox_noun_v1)──→ nox_noun_v2

fixed point check: nox_noun_v1 == nox_noun_v2 (structural equality)
```

| component | nox patterns used |
|-----------|------------------|
| tokenizer | eq (9), branch (4), cons (3) |
| parser | recursive descent via compose (2), branch (4), cons (3) |
| type checker | pattern matching over AST noun |
| NounBuilder | tree rewriting = nox native strength |

exit criterion: compiler compiles itself, produces identical output. hemera NOT required for this check — structural noun equality.

**CHECKPOINT 1 must pass before starting.**

est: 3-4 sessions

**── BOOTSTRAP COMPLETE ──**

## Phase 4: hemera in Trident

write Poseidon2 permutation as hemera.td. compile → nox noun. this noun IS jet 0 (hash) formula. H(noun) = jet 0 formula hash.

```
hemera.td ──trident compile──→ nox noun (hemera program)
  ↓
H(nox_noun) = formula_hash for jet 0 (hash)
  ↓
register in jet registry
  ↓
test: ∀ inputs: trident_hemera(x) == rust_hemera(x)
      (cross-verify on all 202 existing test vectors)
```

exit criterion: bit-exact match on all 202 hemera test vectors.

**CHECKPOINT 2 must pass before starting.**

est: 2 sessions

## Phase 5: zheng (split across tracks)

PARALLEL TRACK A — Rust prover (stays forever):
- sumcheck prover (generates round polynomials)
- Brakedown committer (encode + commit)
- HyperNova folder (accumulate instances)
- output: Proof struct

PARALLEL TRACK B — Trident verifier (provable):
- sumcheck_verify.td (replay rounds, check consistency)
- brakedown_verify.td (spot-check openings)
- hypernova_decide.td (final decider)
- compile → nox nouns → formula hashes for verifier jets

```
Rust prover generates proof
  ↓
Trident verifier (running on nox) accepts/rejects
  ↓
test: valid proofs accepted, invalid proofs rejected
```

exit criterion: end-to-end proven computation. ask(object, formula, focus) → trace → prove(trace) → proof → verify(proof) → accept.

**CHECKPOINT 3 must pass before starting.**

est: 5-8 sessions (parallel tracks)

**── SELF-VERIFYING PROOFS ──**

## Phase 6: jet formulas via Trident

write ALL jet formulas in Trident. compile → nox nouns → formula hashes → jet registry.

five algebras, 30 named jets:

| algebra | jets | source |
|---------|------|--------|
| nebu (F_p) | hash, poly_eval, merkle_verify, fri_fold, ntt | nebu arithmetic |
| kuro (F₂) | popcount, packed_inner_product, binary_matvec, quantize, dequantize, activation_lut, gadget_decompose, barrel_shift | kuro arithmetic |
| jali (R_q) | ntt_batch, key_switch, gadget_decomp, noise_track, blind_rotate | jali arithmetic |
| genies (F_q) | group_action, isogeny_walk, vrf_eval, vdf_step, secret_hash | genies arithmetic |
| trop (min,+) | trop_matmul, trop_shortest, trop_hungarian, trop_viterbi, trop_transport, witness_commit | trop arithmetic |
| decider | decider (89 constraints) | zheng verifier |

each jet: write in Trident → compile to nox noun → H(noun) = formula hash → register in Rust nox VM jet registry → test: jet(x) == pure_formula(x).

**CHECKPOINT 4 passes when all formula hashes are computed.**

exit criterion: all jets produce identical output to pure Trident formulas. nox VM 8× faster with jets.

est: 3-4 sessions

## Phase 7: mudra in Trident

seven crypto protocols, each independent (parallelizable):

| protocol | algebra | est. sessions |
|----------|---------|---------------|
| seal.td | jali (R_q) | 2 |
| stealth.td | genies (F_q) | 1 |
| veil.td | jali (R_q) | 2 |
| quorum.td | nebu (F_p) | 1 |
| delay.td | genies (F_q) | 1 |
| order.td | hemera (hash) | 0.5 |
| place.td | hemera + delay | 0.5 |

cross-verify: ∀ modules: trident_module(x) == rust_arithmetic(x)

est: 5-7 sessions (parallelizable)

## Phase 8: bbg in Trident

authenticated state as Trident programs:

| component | what |
|-----------|------|
| bbg_poly.td | 10-dimension polynomial state |
| mutator_set.td | A(x) commitment + N(x) nullifier |
| signal_validate.td | signal structure verification |
| state_transition.td | 6 transaction types |
| sync_verify.td | 5 verification layers |
| das_verify.td | algebraic DAS sampling |

est: 5-8 sessions

## Phase 9: tru + foculus in Trident

provable ranking and consensus:

| component | what |
|-----------|------|
| tri_kernel.td | diffusion + springs + heat |
| focus_update.td | local π_Δ computation |
| foculus.td | π-convergence finality |
| karma.td | BTS scoring |
| decay.td | temporal weight decay |

test: tri-kernel converges to same fixed point as Rust simulation. Σπ = 1.

est: 4-6 sessions

## critical path

```
CHECKPOINT 0: nox patterns frozen
  ↓
Phase 1: nox VM (Rust)                2-3 sessions
  ↓
Phase 2: trident nox target           3-4 sessions
  ↓
CHECKPOINT 1: trident stable
  ↓
Phase 3: trident self-hosts            3-4 sessions
  ↓                                    ── BOOTSTRAP COMPLETE ──
CHECKPOINT 2: hemera frozen
  ↓
Phase 4: hemera.td                     2 sessions
  ↓
CHECKPOINT 3: zheng frozen
  ↓
Phase 5: zheng (Rust prover ∥          5-8 sessions
         Trident verifier)             ── SELF-VERIFYING PROOFS ──
  ↓
CHECKPOINT 4: jet formulas frozen
  ↓
Phase 6: jet formulas                  3-4 sessions
  ↓
  ├── Phase 7: mudra.td               5-7 sessions  ┐
  ├── Phase 8: bbg.td                 5-8 sessions  ├─ parallel
  └── Phase 9: tru.td                 4-6 sessions  ┘
                                       ── FULL STACK PROVEN ──
```

critical path to self-verifying proofs: ~16-21 sessions (Phase 1 → 5)
parallel phase (7+8+9): ~14-21 sessions concurrent
total: ~30-40 sessions to full proven stack

## Rust vs nox boundary (final state)

| component | Rust (permanent) | Trident/nox (provable) |
|-----------|-----------------|----------------------|
| field arithmetic | nebu, jali, kuro, trop, genies | — |
| hash function | hemera (reference) | hemera.td (provable) |
| VM interpreter | nox kernel (~1,040 LOC) | — |
| jet dispatch | jet registry + wrappers | jet formulas (Trident) |
| proof generation | zheng prover (Rust) | — |
| proof verification | — | zheng verifier.td |
| crypto protocols | — | mudra.td |
| authenticated state | — | bbg.td |
| ranking + consensus | — | tru.td |
| compiler | trident (Rust, bootstrap) | trident.td (self-hosted) |

Rust: ~4,500 LOC permanent (kernel + prover + arithmetic wrappers)
Trident/nox: all application logic, provable end-to-end

dual-language invariant: `∀ f, x: rust_f(x) == trident_f(x)`. Rust reference exists for every Trident module. both tested, divergence = bug.

discover all [[concepts]]
