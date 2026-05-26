---
tags: cyber
alias: trop, tropical semiring, tropical algebra, tropical optimization
crystal-type: entity
crystal-domain: cyber
---
the tropical semiring execution engine for [[cyber]]. (min, +) algebra for provable optimization. trop is the decision engine — where [[nebu]] proves truth and [[kuro]] proves bits, trop proves optimality.

`a ⊕ b = min(a, b)`. `a ⊗ b = a + b`. no additive inverse — this is a semiring, not a field. the missing inverse is what makes optimization irreducible to field arithmetic.

encoding min(a, b) over [[Goldilocks field|Goldilocks]] requires bit decomposition — ~10 constraints. native tropical: 1 operation. the 10× gap compounds over millions of optimization steps.

## workloads

| domain | tropical formulation |
|--------|---------------------|
| shortest path | tropical matrix power |
| dynamic programming | tropical recursion |
| Viterbi decoding | tropical matrix-vector multiply |
| belief propagation | tropical message passing |
| optimal transport | tropical linear program |
| attention (hardmax) | tropical softmax limit |
| scheduling | tropical eigenvalue |
| game theory (minimax) | tropical duality |

## nox integration

no new Layer 1 patterns. min(a, b) = branch(lt(a, b), a, b) — two existing [[nox]] patterns. tropical execution is a PROGRAM on nox.

Layer 3 jets: jet_trop_matmul, jet_trop_shortest, jet_trop_hungarian, jet_trop_viterbi, jet_trop_transport.

verification: tropical execution produces a witness. [[zheng]] verifies optimality via LP dual certificate over F_p.

## dependency graph

```
nebu (F_p) — verification backbone
  ↓
trop (min, +) ← this repo
  ↓
nox (jets)
  ↓
zheng (proof of optimality)
```

discover all [[concepts]]
