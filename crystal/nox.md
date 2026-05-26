---
tags: cyber
alias: nox, nox vm, nox virtual machine, cyber/nox
crystal-type: entity
crystal-domain: cyber
---
the composition language and virtual machine of [[cyber]]. eighteen patterns total over the [[Goldilocks field]]: sixteen deterministic compute patterns, plus [[call]] (pattern 16, non-deterministic witness injection) and [[look]] (pattern 17, deterministic BBG read), plus five [[jets]]. every computation produces a [[stark]] proof of correct execution as a byproduct.

nox descends from [[Nock]] ([[Urbit]]), replacing natural numbers with [[Goldilocks field]] elements and decrement with field inverse. the execution trace IS the algebraic constraint system — there is no translation layer between the program and the proof.

five structural operations define how values compose regardless of what those values are:

| Op | Action | Analogy |
|---|---|---|
| `axis` | Navigate into a subtree by path | Array index |
| `quote` | Treat code as data | String literal |
| `compose` | Chain two computations | Function composition |
| `cons` | Build a pair | Struct constructor |
| `branch` | Conditional selection | If-then-else |

the critical difference from [[Nock]]: Nox's tree is a Merkle tree by construction. every `cons(a, b)` computes `hash(a, b)` and stores the digest at the parent node. `axis` produces a Merkle proof as a side effect. the authentication scheme is abstract — pluggable backends ([[Hemera]], SHA-256, Verkle, SMT).

nox is simultaneously the structural IR (the grammar all [[cyb/languages]] compile through), the node runtime (the production binary that runs the [[cyber]] blockchain), and the composition tier that orchestrates programs across all execution languages, manages [[proof]] aggregation, and defines the program structure of the whole system.

## three layers

```
Layer 1: 16 deterministic compute patterns (structural + field arithmetic + bitwise + hash)
Layer 2: call (16, non-deterministic witness injection, verified by Layer 1) + look (17, deterministic BBG read)
Layer 3: 5 jets (hash, poly_eval, merkle_verify, fri_fold, ntt)
```

Layer 1 defines truth. Layer 2 defines the prover-verifier boundary. Layer 3 defines performance. remove Layer 3: identical results, slower. remove Layer 2: no privacy, no ZK. remove Layer 1: nothing remains.

## dependency graph

```
nebu (field)
  ↓
hemera (hash)
  ↓
nox (VM) ← this repo
  ↓
zheng (proofs)
  ↓
bbg (state)
```

## computation as cyberlink

```
ask(ν, subject, formula, τ, a, v, t) → answer
```

the seven arguments of `ask` are the seven fields of a [[cyberlink]]. computation IS linking

1. compute `order_axon = H(formula, subject)`
2. lookup: does `axon(formula, subject)` have a verified result in the [[cybergraph]]?
   → yes: return cached result (zero computation — memoized)
   → no: `reduce(subject, formula)`, prove via [[zheng|STARK]]
3. link `order_axon → result` (with [[proof]])

the [[cybergraph]] is a universal, persistent, proven memo cache. every computation anyone ever did is reusable by everyone. the more the graph grows, the fewer computations actually execute

see [[cyber/nox]] for the full specification, [[zheng]] for the proof system, [[trident]] for the high-level language
