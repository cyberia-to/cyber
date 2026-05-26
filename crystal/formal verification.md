---
tags: computer science
crystal-type: process
crystal-domain: computer science
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