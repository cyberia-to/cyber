---
tags: cyber, research, article, core
crystal-type: article
crystal-domain: cyber
date: 2026-03-25
---
# bootstrap plan

implementation plan for the [[cyber]] execution stack. principle: maximize [[nox]] code, minimize Rust. the nox kernel is the only irreducible Rust. everything else — proofs, state, crypto, compiler — executes as nox programs + jets.

## current state

| repo | status | LOC |
|------|--------|-----|
| [[nebu]] | DONE | ~1.5k Rust |
| [[hemera]] | DONE | ~5k Rust |
| [[genies]] | DONE | ~1.9k Rust |
| [[jali]] | DONE | ~1.4k Rust |
| [[kuro]] | DONE | working Rust |
| [[trop]] | STUBS | spec only |
| [[nox]] | STUBS | 8 modules, 80 LOC (all empty), full spec (9 reference files) |
| [[zheng]] | STUB | empty lib.rs, full spec (16 reference files) |
| [[bbg]] | STUB | empty lib.rs, full spec (10 files), BLOCKED on nox + zheng |
| [[mudra]] | STUB | empty lib.rs, full spec (9 files), deps satisfied |
| [[Trident]] | IMPLEMENTED | mature compiler, 24 VM targets, NO nox target |

## dependency graph

```
nebu (DONE) → hemera (DONE) → nox (STUB) → zheng (STUB) → bbg (STUB)
                                    ↓
                              trident (needs nox target)
                                    ↓
                              mudra (needs hemera + genies + jali)
```

## phase 0: nox Rust kernel (irreducible minimum)

the nox interpreter MUST be Rust — it cannot interpret itself.

| file | what | LOC |
|------|------|-----|
| noun.rs | arena, hash-consing, structural hash via [[hemera]] | ~250 |
| reduce.rs | 16 pattern dispatch + focus deduction | ~350 |
| focus.rs | focus metering, cost table | ~50 |
| hint.rs | Layer 2 non-deterministic witness, trait + null provider | ~80 |
| encode.rs | wire format (8/32/64 byte atoms/cells), content-addressed store trait | ~120 |
| memo.rs | computation cache: (H(object), H(formula)) → H(result) | ~60 |
| trace.rs | TraceRow recording during reduction | ~30 |
| jet.rs | jet registry: formula hash → native fn, empty initially | ~60 |
| lib.rs | top-level ask() = memo check + reduce + record | ~40 |

total: ~1,040 LOC Rust. this is the entire irreducible kernel.

exit criterion: all test vectors from patterns.md pass. ask() executes arbitrary formulas with correct focus metering.

## phase 1: wire the jets

connect existing Rust implementations as nox Layer 3 jets.

| jet | wraps | LOC |
|-----|-------|-----|
| jet_hash | hemera::hash() | ~30 |
| jet_ntt | nebu::ntt::ntt() | ~50 |
| jet_poly_eval | Horner's method via nebu | ~40 |
| jet_merkle_verify | hemera path walk | ~50 |
| jet_fri_fold | FRI folding via nebu | ~50 |
| equivalence test harness | jet vs Layer 1 on random inputs | ~100 |

total: ~320 LOC Rust. all jet wrappers, no new algorithms.

exit criterion: each jet produces identical output to Layer 1 pure formula on 1000 random inputs.

## phase 2: sumcheck as nox program

the [[sumcheck]] protocol from zheng/reference/sumcheck.md. pure field arithmetic — patterns 5 (add), 7 (mul), 9 (eq), 15 (hash).

the verifier is ~200 field ops. the prover is O(2^k) field ops accelerated by jet_ntt.

Rust in zheng/src/ = formula CONSTRUCTORS (build nox nouns). execution happens in nox.

```
pub fn sumcheck_verifier_formula(arena: &mut nox::Arena) -> nox::NounRef
pub fn sumcheck_prover_formula(arena: &mut nox::Arena) -> nox::NounRef
```

~300 LOC Rust (formula builders). the actual programs are nox nouns.

exit criterion: prover + verifier round-trip. valid proofs accepted, invalid rejected.

## phase 3: full [[zheng]] proof pipeline

[[Brakedown]] PCS + [[SuperSpartan]] IOP + Fiat-Shamir transcript. all as nox programs + jets.

| component | nox patterns used | jets used |
|-----------|------------------|-----------|
| transcript | hash (15) | jet_hash |
| Brakedown commit | mul (7), add (5) | jet_ntt |
| Brakedown open | mul (7), compose (2) | jet_poly_eval |
| Brakedown verify | mul (7), eq (9) | none (pure arithmetic) |
| SuperSpartan prover | mul (7), add (5), sumcheck | jet_ntt |
| SuperSpartan verifier | sumcheck verify, PCS verify | jet_poly_eval |
| constraint encoding | CCS matrices as constant nouns | none |

the verifier is ~825 constraints (per zheng/reference/verifier.md). the prover is dominated by matrix-vector products.

~600 LOC Rust (formula builders).

exit criterion: ask(object, formula, focus) → trace → prove(trace) → proof → verify(proof) → accept. end-to-end proven computation.

## phase 4: [[Trident]] nox target

add nox as compilation target. plan exists at trident/.claude/plans/cyber-stack-adoption.md.

| step | what | LOC |
|------|------|-----|
| 4a | replace trident's internal Goldilocks/Poseidon2 with nebu/hemera | ~200 |
| 4b | vm/nox/ target profile (target.toml) | ~50 |
| 4c | NounBuilder: AST → nox noun (direct, bypass TIR) | ~800 |
| 4d | os/cyber/ type definitions (Particle, Neuron, Cyberlink) | ~200 |
| 4e | pipeline integration: `trident build --target nox` | ~250 |

total: ~1,500 LOC Rust.

exit criterion: `trident build --target nox fibonacci.tri` produces valid nox noun.

## phase 5: trident self-hosting

write the trident compiler AS a nox program. trident compiles itself to nox.

| component | nox mechanism |
|-----------|--------------|
| tokenizer | pattern 9 (eq) + pattern 4 (branch) + pattern 3 (cons) |
| parser | recursive descent via compose (2), branch (4), cons (3) |
| type checker | pattern matching over AST noun, environment = subject |
| NounBuilder | tree rewriting = nox native strength |

verification: compile trident source with (a) Rust-hosted trident and (b) nox-hosted trident. outputs must be identical nouns (same structural hash).

~5,000-10,000 nox patterns (the compiler itself, as a noun). zero new Rust.

exit criterion: ask(trident_source_noun, self_hosted_compiler_noun, focus) = same output as `trident build --target nox`.

## phase 6: [[bbg]] authenticated state

BLOCKED on phase 0 + phase 3.

state operations as nox programs:
- READ = PCS.open (polynomial evaluation)
- WRITE = axis + cons
- ASSERT_EQ = pattern 9
- transactions: CYBERLINK (~3,200 constraints), PRIVATE TRANSFER, COMPUTATION, MINT/TRANSFER CARD, BRIDGE

~500 LOC Rust (formula builders) + nox programs.

## phase 7: [[mudra]] crypto protocols

7 protocols as nox programs + algebra jets:

| protocol | jet | source |
|----------|-----|--------|
| seal (ML-KEM) | jet_ntt | [[jali]] |
| stealth | jet_genies_action | [[genies]] |
| veil (TFHE) | jet_ntt | [[jali]] |
| quorum (threshold) | jet_genies_action | [[genies]] |
| delay (VDF) | jet_genies_action | [[genies]] |
| order (consensus) | jet_hash | [[hemera]] |
| place (location) | jet_hash | [[hemera]] |

~400 LOC Rust (jet wrappers) + nox programs.

## critical path

```
Phase 0 (nox kernel)     3 sessions ─── BLOCKS EVERYTHING
  │
  ├── Phase 1 (jets)           1.5 sessions
  │     │
  │     └── Phase 2 (sumcheck) 2 sessions
  │           │
  │           └── Phase 3 (zheng) 4 sessions
  │                 │
  │                 └── Phase 6 (bbg) 4 sessions
  │
  ├── Phase 4 (trident)  7.5 sessions ─── parallel with 2-3
  │     │
  │     └── Phase 5 (self-host) 6-8 sessions
  │
  └── Phase 7 (mudra)    3 sessions ─── parallel with 2-7
```

total critical path: ~14.5 sessions (0→1→2→3→6) + 13.5-15.5 sessions (4→5, parallel)

## Rust vs nox boundary

| component | Rust | nox | why |
|-----------|------|-----|-----|
| noun arena + hash-consing | yes | — | cannot interpret itself |
| 16-pattern dispatch | yes | — | IS the interpreter |
| focus metering | yes | — | inner loop |
| wire encoding | yes | — | byte-level |
| field arithmetic | nebu (done) | — | performance |
| hash function | hemera (done) | — | performance |
| NTT | nebu (done) | — | performance |
| sumcheck | formula builder | execution | pure field arithmetic |
| Brakedown PCS | formula builder | execution | matrix-vector multiply |
| SuperSpartan IOP | formula builder | execution | composes sumcheck + PCS |
| verifier | — | 825 constraints | field arithmetic |
| state operations | — | execution | patterns 5, 7, 9 |
| bbg transactions | — | execution | state ops + hash |
| crypto protocols | jet wrappers | execution | field arithmetic + jets |
| trident compiler | NounBuilder (Rust) | self-hosts as nox | phase 5 goal |

new Rust: ~4,660 LOC. nox programs: ~20,000-40,000 patterns. ratio 1:5 to 1:9 in favor of nox.

the end state: only the nox kernel (1,040 LOC) + jet wrappers (~1,220 LOC) + formula builders (~1,400 LOC) + trident NounBuilder (~1,500 LOC) are Rust. everything else is nox programs executing on nox. the compiler self-hosts. proofs self-verify. the machine bootstraps itself.

discover all [[concepts]]
