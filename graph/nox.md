---
tags: cyber
alias: nox, nox vm, nox virtual machine
crystal-type: entity
crystal-domain: cyber
subgraph: true
repo: ../nox
exclude: ".claude/**, target/**, CLAUDE.md"
---
the virtual machine of [[cyber]]. sixteen deterministic reduction patterns over the [[Goldilocks field]], plus one non-deterministic [[hint]] pattern and five [[jets]]. every computation produces a [[stark]] proof of correct execution as a byproduct.

nox descends from [[Nock]] ([[Urbit]]), replacing natural numbers with [[Goldilocks field]] elements and decrement with field inverse. the execution trace IS the algebraic constraint system — there is no translation layer between the program and the proof.

## three layers

```
Layer 1: 16 deterministic patterns (structural + field arithmetic + bitwise + hash)
Layer 2: hint (non-deterministic witness injection, verified by Layer 1)
Layer 3: 5 jets (hash, poly_eval, merkle_verify, fri_fold, ntt)
```

Layer 1 defines truth. Layer 2 defines the prover-verifier boundary. Layer 3 defines performance. remove Layer 3: identical results, slower. remove Layer 2: no privacy, no ZK. remove Layer 1: nothing remains.

## dependency graph

```
aurum (field)
  ↓
hemera (hash)
  ↓
nox (VM) ← this repo
  ↓
zheng (proofs)
  ↓
bbg (state)
```

see [[cyber/nox]] for the full specification, [[cyber/stark]] for the proof pipeline, [[trident]] for the high-level language, [[zheng]] for the proof system
