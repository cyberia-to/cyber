---
tags: computer science
crystal-type: process
crystal-domain: computer science
stake: 4213689365244941
diffusion: 0.0005257407432848711
springs: 0.0002264624552415453
heat: 0.00033943550130739226
focus: 0.0003986962084763893
gravity: 10
density: 3.89
---
Mathematical proof that a system (software, hardware, protocol) meets its specification. Certainty beyond testing.

## approaches

- model checking: exhaustive exploration of all reachable states, temporal logic (CTL, LTL), counterexample generation
- theorem proving: interactive or automated construction of proofs. Coq, Isabelle, Lean, Agda
- abstract interpretation: sound approximation of program behavior, static analysis
- SMT solvers: satisfiability modulo theories, automated decision procedures (Z3)

## connection to type theory

The [[Curry-Howard correspondence]] in [[type theory]] equates proofs with programs and propositions with types. Dependently typed languages (Idris, Agda) merge programming and proving into one activity.

## applications

- verified [[compilers]]: CompCert (verified C compiler)
- verified [[operating systems]]: seL4 (formally proven microkernel)
- verified [[consensus algorithms]]: proofs of safety and liveness for BFT protocols
- smart contract verification: proving correctness of on-chain logic in [[cyber]]
- hardware verification: proving chip designs match specification

## relation to complexity

Verification itself can be computationally expensive. Many verification problems are undecidable in general ([[complexity theory]]), but tractable for restricted domains.