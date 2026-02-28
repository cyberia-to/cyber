---
tags: computer science
crystal-type: entity
crystal-domain: computer science
---

Classification of computational problems by the resources (time, space, randomness) required to solve them. The science of what is feasible to compute.

## core classes

P: problems solvable in polynomial time by a deterministic [[automata|Turing machine]]

NP: problems whose solutions are verifiable in polynomial time

NP-complete: the hardest problems in NP, every NP problem reduces to them (Cook-Levin theorem)

NP-hard: at least as hard as NP-complete, possibly harder

PSPACE: problems solvable with polynomial space

BPP: problems solvable in polynomial time with bounded error probability

BQP: problems solvable by a quantum computer in polynomial time

## key concepts

reductions: proving one problem is at least as hard as another by transformation

P vs NP: the central open question -- does efficient verification imply efficient solution?

Big-O notation: asymptotic resource bounds, the language of [[algorithms]] analysis

circuit complexity: measuring computation by Boolean circuit size and depth

## applications

[[cryptography]] relies on the assumption that certain problems (factoring, discrete log) are hard. [[zero knowledge proofs]] exploit the gap between proving and revealing. [[consensus algorithms]] face impossibility results (FLP theorem) rooted in complexity.