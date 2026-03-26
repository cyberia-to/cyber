---
tags: cyber, research, article, core
crystal-type: article
crystal-domain: cyber
date: 2026-03-25
---
# bootstrap plan

three-stage bootstrap for the [[cyber]] execution stack. like GCC: system compiler → self-host → proven self-host. each stage upgrades trust level. Rust = untrusted bootstrap + fast jets. Trident = proven canonical implementation. dual-language invariant at every level: `∀ f, x: rust_f(x) == trident_f(x)`.

## current state

| repo | status | LOC | tests | role |
|------|--------|-----|-------|------|
| [[nebu]] | DONE | 1,526 Rust | 73 | F_p scalar arithmetic |
| [[hemera]] | DONE | 5,084 Rust | 202 | Poseidon2 hash + Merkle tree |
| [[jali]] | DONE | 1,442 Rust | 70 | R_q polynomial ring |
| [[kuro]] | DONE | 1,157 Rust | 77 | F₂ binary tower |
| [[trop]] | DONE | 1,567 Rust | 77 | (min,+) tropical semiring |
| [[genies]] | DONE | 1,997 Rust | 55 | F_q isogeny group action |
| [[Trident]] | DONE | 57,736 Rust | 236 files, 24 VM targets | provable language compiler |
| [[nox]] | SPEC | 35 Rust (stubs) | — | VM (spec complete, 9 reference files) |
| [[zheng]] | SPEC | 6 Rust (stub) | — | proof system (spec complete, 18 reference files) |
| [[bbg]] | SPEC | 7 Rust (stub) | — | authenticated state (spec complete) |
| [[mudra]] | SPEC | 6 Rust (stub) | — | crypto protocols (spec complete) |

arithmetic layer: 12,773 LOC, 554 tests — production-ready. Trident compiler: 57K LOC, mature, has nock tree target (level 3) — nox target needed (hemera instead of Tip5, 4-element digest instead of 5).

## the Thompson trust problem

Ken Thompson (1984): a compiler can contain a self-replicating backdoor invisible in source code. the backdoor reproduces itself in every binary the compiler produces — including compilations of the compiler itself. inspecting source reveals nothing. the worm lives in the binary lineage, not in the source.

Rust inherits from LLVM. LLVM inherits from GCC. GCC inherits from the first C compiler. every link in this chain is a potential Thompson vector. if a worm entered at any point in 50 years of compiler history, it propagates through every subsequent compilation.

**does our Stage 3 (proven bootstrap) detect a Thompson worm?**

what the STARK proof covers:
- the nox interpreter correctly dispatched 16 patterns (field arithmetic verified)
- the Trident compiler correctly lowered AST to nox nouns (tree rewriting verified)
- hemera correctly hashed content (permutation verified)

what the STARK proof does NOT cover:
- the Rust nox VM that EXECUTED the nox program (the VM itself is unproven Rust)
- the hardware that ran the Rust VM

a Thompson worm in Rust/LLVM could make the Rust nox VM produce CORRECT-LOOKING proofs for INCORRECT executions. the proof says "this nox program produced this output" — but the Rust VM that ran the program might have been compromised.

**the escape:** nox.td (the proven interpreter) running on Rust nox VM creates a provable SECOND interpreter. if the Rust VM has a worm, the outputs of nox.td-on-Rust-VM and nox.td-on-independent-VM will DIFFER. multi-implementation verification:

```
implementation A: Rust nox VM (potentially compromised)
implementation B: independent nox VM (different compiler, different hardware)
implementation C: nox.td running on A
implementation D: nox.td running on B

if A(program) == B(program) == C(program) == D(program):
  no worm detected (or all four are compromised — requires conspiracy)
```

the STARK proof is the mathematical layer. multi-implementation is the physical layer. together they cover both source-level and binary-level attacks.

practical: implement nox VM in at least TWO independent toolchains (Rust + one other — Go, Zig, or hand-written assembly). cross-verify outputs. a worm that compromises ALL toolchains simultaneously is science fiction.

## Rs language

Rs (~/git/rs) is a restricted Rust edition that enforces determinism by construction:

| code | forbidden | replacement |
|------|-----------|-------------|
| RS501 | `Box::new()` | stack values or `Arena<T, N>` |
| RS502 | `Vec<T>` | `BoundedVec<T, N>` with compile-time capacity |
| RS503 | `String` | `&str` or `ArrayString<N>` |
| RS504 | `dyn Trait` | generics or enum dispatch |
| RS505 | `Arc<T>`, `Rc<T>` | cell-owned state or bounded channels |
| RS506 | `panic!()` | `Result` for recoverable, abort for unrecoverable |
| RS507 | `HashMap`, `HashSet` | `BTreeMap`, `BTreeSet`, or `BoundedMap<K,V,N>` |

Rs depends on the Rust compiler (which depends on LLVM). the restrictions eliminate non-determinism at the LANGUAGE level, but the COMPILER BINARY is still in the LLVM trust chain.

the path to full independence: Rs → Trident (self-hosted, proven) → no LLVM dependency. Trident compiled by proven Trident on proven nox = compiler with STARK-verified correctness. the LLVM lineage becomes irrelevant once the proven compiler exists.

## checkpoints (freeze points)

each checkpoint = "stop iterating, build on top." before mainnet: unfreezing = rebuild (days). after mainnet: unfreezing = catastrophic (hours across 10^6 nodes).

| checkpoint | what freezes | when | why |
|------------|-------------|------|-----|
| 0 | nox patterns | before Stage 1 | 16 patterns = instruction set. changing = new CPU |
| 1 | trident language | before Stage 2 Phase 3 | self-hosting requires stable syntax |
| 2 | hemera parameters | before Stage 2 Phase 4 | H(noun) = identity. changing = rehash everything |
| 3 | zheng protocol | before Stage 3 Phase 6 | proof format. changing = all proofs invalid |
| 4 | jet formulas | before Stage 3 Phase 9 | H(formula) = jet identity. protocol constants |

## the spine (seven repos, seven verbs)

```
hemera → lens → trident → cybergraph → nox → zheng → bbg
 hash    commit  compile      link      run    prove   store
```

[[cybergraph]] is the vertebra — the center that everything attaches to. jets, memos, types, dependencies, knowledge — all cyberlinks in one graph. see [[cyb/stack]] for full architecture.

## the self-hosting boundary

the spine is the MINIMAL set that cannot implement itself. once it exists, everything above is pure .td:

| below boundary (needs Rust bootstrap) | above boundary (pure .td) |
|---------------------------------------|--------------------------|
| hemera, lens, trident, nox, zheng, bbg | plumb, identity, social, geo, tru, foculus, mudra |
| dual existence: Rust + Trident | single existence: Trident only |

## genesis crystal

the [[cybergraph]] starts empty. core semcons need tokens to deploy. tokens need plumb semcon to exist. the crystal resolves this: a .td program that runs once with unlimited focus.

```
genesis.td:
  create_token(CYB, HYDROGEN, VOLT, AMPERE)
  register_semcon(plumb, identity, social, geo)
  distribute(initial_balances)
  // genesis focus expires. normal rules apply.
```

the crystal is the seed structure. without it — empty graph, no rules. with it — economics, types, constraints. even genesis is proven (compiled by trident, executed by nox, proof by zheng).

## dependency graph

```
DONE (arithmetic + identity):
  nebu → hemera → {jali, kuro, trop, genies}
  trident (57K LOC, nock target exists)

spine (seven repos):
  hemera → lens → trident → cybergraph → nox → zheng → bbg

STAGE 1 (Rust bootstrap):
  nox VM (Rs) → trident nox target

STAGE 2 (classical self-host):
  trident.td → nebu.td + hemera.td + lens.td → nox.td

STAGE 3 (proven bootstrap):
  zheng (Rust prover + Trident verifier) → proven re-self-host
    → remaining arithmetic.td → jet registry (in cybergraph)
      → genesis.td (crystal)
        → plumb.td, identity.td, tru.td (parallel)
```

## Stage 1: Rust bootstrap (untrusted foundation)

### Phase 1: nox VM in Rs                                    2-3 sessions

the irreducible Rust — nox cannot interpret itself (at this stage).

| file | what | LOC |
|------|------|-----|
| noun.rs | arena + hash-consing + structural hash via hemera | ~250 |
| reduce.rs | 16 pattern dispatch + focus deduction | ~350 |
| focus.rs | focus metering, cost table | ~50 |
| hint.rs | HintProvider trait, sync, NoHint → Halt | ~80 |
| encode.rs | wire format, content-addressed store trait | ~120 |
| memo.rs | (H(object), H(formula)) → H(result) cache | ~60 |
| trace.rs | TraceRow recording per reduce() | ~30 |
| jet.rs | jet registry: formula hash → native fn, empty initially | ~60 |
| lib.rs | ask() = memo check + reduce + record | ~40 |

~1,040 LOC Rs. Layer 1 only, no jets.

exit: all test vectors from patterns.md pass. CHECKPOINT 0 must hold.

### Phase 2: Trident nox target                              3-4 sessions

fork from nock target (tree architecture, level 3). key changes:

| what | nock target | nox target |
|------|------------|-----------|
| hash | Tip5 (5-element digest) | hemera (4-element digest, 32 bytes) |
| lowering | TreeLowering | NounLowering (direct AST → nox noun, bypass TIR) |
| cost model | reductions | focus (exact per-pattern costs) |
| output | .jam | .nox |

NounBuilder: direct AST → Noun path. every Trident construct maps to exactly one nox pattern. no intermediate representation (TIR destroyed parallelism info for tree targets). see trident/.claude/plans/cyber-stack-adoption.md for full spec.

~1,500 LOC Rs.

exit: `trident build --target nox fibonacci.td` → nox noun → `nox_eval(input, noun, focus)` → correct.

```
STATE AFTER STAGE 1:
  Rust nox VM runs nox programs (no proofs)
  Rust Trident compiler produces nox nouns
  trust: "I trust Rust" (LLVM lineage unverified)
```

## Stage 2: classical self-host (trust by equality)

### Phase 3: trident.td (self-hosting)                       3-4 sessions

write Trident compiler IN Trident. tokenizer, parser, type checker, NounBuilder — all in .td files.

```
trident_source ──Rust compiler──→ nox_noun_v1
trident_source ──nox_eval(source, nox_noun_v1)──→ nox_noun_v2

fixed point: nox_noun_v1 == nox_noun_v2 (structural equality)
```

hemera NOT required. structural noun equality. CHECKPOINT 1 must hold.

### Phase 4: arithmetic in Trident                           4-6 sessions

ALL six arithmetics + hemera as Trident programs:

| file | what | cross-verify against |
|------|------|---------------------|
| nebu.td | add, sub, mul, inv, eq, lt over F_p | nebu (Rust, 73 tests) |
| hemera.td | Poseidon2 permutation | hemera (Rust, 202 tests) |
| jali.td | R_q ring ops, NTT multiply, automorphisms | jali (Rust, 70 tests) |
| kuro.td | F₂ tower ops, XOR, AND, Karatsuba | kuro (Rust, 77 tests) |
| trop.td | tropical min, saturating add, matrix power | trop (Rust, 77 tests) |
| genies.td | F_q Montgomery, isogeny walk, class group | genies (Rust, 55 tests) |

compiled by self-hosted Trident. cross-verified: `∀ inputs: trident_f(x) == rust_f(x)` on all existing test vectors (554 total).

each .td file → nox noun → H(noun) = jet formula hash (used in Stage 3).

CHECKPOINT 2 must hold (hemera frozen before hemera.td).

### Phase 5: nox.td (proven interpreter)                     3-4 sessions

16 pattern dispatch written in Trident. uses nebu.td + hemera.td.

```
nox.td ──self-hosted trident──→ nox noun (meta-circular interpreter)

test: nox.td interpreting program P == Rust nox interpreting P
```

meta-circular: nox noun interprets nox nouns on Rust nox VM. correctness = cross-verification against Rust VM.

```
STATE AFTER STAGE 2:
  trident.td  self-hosted compiler (nox noun)
  nebu.td     field arithmetic (nox noun)
  hemera.td   hash function (nox noun)
  jali.td     ring arithmetic (nox noun)
  kuro.td     binary arithmetic (nox noun)
  trop.td     tropical arithmetic (nox noun)
  genies.td   isogeny arithmetic (nox noun)
  nox.td      interpreter (nox noun)
  ALL cross-verified against Rust references
  trust: "structural equality + cross-verification"
  NO proofs yet
```

## Stage 3: proven bootstrap (trust by STARK proof)

### Phase 6: zheng (proof infrastructure)                    5-8 sessions

PARALLEL TRACK A — Rust prover (permanent):
  sumcheck prover, Brakedown committer, HyperNova folder
  generates STARK proofs from nox traces

PARALLEL TRACK B — Trident verifier:
  sumcheck_verify.td, brakedown_verify.td, decider.td
  compiled by self-hosted Trident → nox nouns
  cross-verify: Trident verifier agrees with Rust reference

exit: Rust prover + Trident verifier: valid proofs accepted, invalid rejected.

CHECKPOINT 3 must hold. nox executions can now produce proofs.

### Phase 7: proven re-self-host                             1-2 sessions

RE-RUN Phase 3 with proof generation:

```
trident_source ──nox_eval(source, compiler_v1)──→ compiler_v2
                   ↓ (trace recorded)
                   Rust zheng prover → STARK proof
                   zheng.td verifier → ACCEPTS
```

the SAME compilation as Phase 3, but now with STARK proof.

### Phase 8: proven nox.td                                   1 session

RE-COMPILE nox.td with proven compiler + proof generation. the proven interpreter: programs running on it produce double proofs (program correct AND interpreter correct).

### Phase 9: jet registry                                    2-3 sessions

Trident arithmetic (.td files from Phase 4) = jet formulas. formula hashes computed from Phase 4 nox nouns. register Rust arithmetic as jet IMPLEMENTATIONS for Trident-derived formula hashes.

30 named jets across five algebras + decider:

| algebra | formulas from | Rust jet impl | jets |
|---------|--------------|---------------|------|
| nebu | nebu.td | nebu crate | 5 |
| kuro | kuro.td | kuro crate | 8 |
| jali | jali.td | jali crate | 5 |
| genies | genies.td | genies crate | 5 |
| trop | trop.td | trop crate | 6 |
| hemera | hemera.td | hemera crate | 1 (hash) |
| zheng | zheng.td | — | 1 (decider, 89 constraints) |

test: `∀ jets: rust_jet(x) == trident_formula(x)` on random inputs. CHECKPOINT 4 passes.

```
STATE AFTER STAGE 3:
  PROVEN compiler (STARK proof)
  PROVEN interpreter (STARK proof)
  PROVEN hash, arithmetic, verifier (STARK proofs)
  JET REGISTRY frozen (formula hashes = protocol constants)
  trust: mathematical certainty
```

## genesis and protocol layer (on proven foundation)

### Phase 10: genesis crystal                                1 session

the seed. runs once with unlimited focus. creates tokens, registers core semcons, distributes initial balances. proven: compiled by trident.td, executed by nox, proof by zheng.

### Phase 11: core semcons                                   8-12 sessions

four "heavy" semcons that reach deep into the spine:

| semcon | what | sessions |
|--------|------|----------|
| plumb.td | tokens, staking, delegation, conservation, UTXO | 3-4 |
| identity.td | neuron registration, key proof, ownership | 2-3 |
| social.td | following, reputation edges | 1-2 |
| geo.td | location proofs, physical attestation | 2-3 |

partially parallel (plumb blocks identity, social and geo independent).

### Phase 12: computed layer                                 6-10 sessions

| program | what | sessions |
|---------|------|----------|
| tru.td | tri-kernel, focus, cyberank, karma, decay | 4-6 |
| foculus.td | consensus, finality from topology | 2-4 |

### Phase 13: mudra.td                                       5-7 sessions

seven crypto protocols, each independent:
  seal, stealth, veil, quorum, delay, order, place

Phases 11-13 partially parallel (different file scopes).

## critical path

```
CHECKPOINT 0: nox patterns frozen
  ↓
STAGE 1 (Rust bootstrap)
  Phase 1: nox VM (Rs)                        2-3 sessions
  Phase 2: trident nox target                  3-4 sessions
  ↓
CHECKPOINT 1: trident stable
  ↓
STAGE 2 (classical self-host)
  Phase 3: trident.td self-hosts               3-4 sessions ── BOOTSTRAP MOMENT
  CHECKPOINT 2: hemera frozen
  Phase 4: arithmetic.td (6 repos + hemera)    4-6 sessions
  Phase 5: nox.td (proven interpreter)         3-4 sessions
  ↓
CHECKPOINT 3: zheng frozen
  ↓
STAGE 3 (proven bootstrap)
  Phase 6: zheng (prover ∥ verifier)           5-8 sessions ── PROOFS
  Phase 7: proven re-self-host                 1-2 sessions
  Phase 8: proven nox.td                       1 session
  Phase 9: jet registry (in cybergraph)        2-3 sessions
  CHECKPOINT 4: jets frozen
  ↓
═══════════════════════════════════════════════════════════
  THE BOUNDARY: above here = pure .td on proven spine
═══════════════════════════════════════════════════════════
  ↓
GENESIS
  Phase 10: genesis.td (crystal)               1 session
  ↓
PROTOCOL (core semcons)
  Phase 11: plumb.td + identity.td             5-7 sessions  ┐
            social.td + geo.td                 3-5 sessions  ├─ partial parallel
  Phase 12: tru.td + foculus.td                6-10 sessions ┘
  Phase 13: mudra.td                           5-7 sessions (parallel)
```

critical path to proven spine: ~25-34 sessions (Phase 1 → 9)
genesis + protocol: ~20-30 sessions (partially parallel)
total: ~40-55 sessions to full proven system

## the two columns (final state)

| component | Rust (fast, permanent) | Trident (proven, canonical) |
|-----------|----------------------|---------------------------|
| F_p arithmetic | nebu (jet impl) | nebu.td (jet formula) |
| F₂ arithmetic | kuro (jet impl) | kuro.td (jet formula) |
| R_q arithmetic | jali (jet impl) | jali.td (jet formula) |
| (min,+) arithmetic | trop (jet impl) | trop.td (jet formula) |
| F_q arithmetic | genies (jet impl) | genies.td (jet formula) |
| hash | hemera (jet impl) | hemera.td (jet formula) |
| VM interpreter | nox VM (bootstrap) | nox.td (proven interpreter) |
| proof generation | zheng prover | — |
| proof verification | — | zheng.td (proven verifier) |
| crypto protocols | — | mudra.td |
| authenticated state | — | bbg.td |
| ranking + consensus | — | tru.td |
| compiler | trident (bootstrap) | trident.td (self-hosted, proven) |

dual-language invariant at EVERY level, including arithmetic.

discover all [[concepts]]
