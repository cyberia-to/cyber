---
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
alias: Customizable Constraint Systems
---
# CCS

Customizable Constraint Systems. a unified constraint framework that generalizes [[R1CS]], Plonkish ([[PLONK]]/Halo2), and [[AIR]] into one representation. Setty, Thaler, Wahby (2023).

```
CCS instance: (M₁, ..., M_t, S₁, ..., S_q, c₁, ..., c_q)

constraint:  Σⱼ cⱼ · ∏_{i ∈ Sⱼ} Mᵢ · z = 0

special cases:
  R1CS:     t=3, q=2, c₁=1, c₂=-1        → degree 2
  Plonkish: selector polynomials → M        → custom gates
  AIR:      shifted rows → M                → transition constraints
```

the unification matters because a proof system handling CCS handles all three — including AIR constraints of any degree. [[SuperSpartan]] is this proof system.

## why CCS matters for zheng

in [[zheng]], [[nox]]'s sixteen reduction patterns produce AIR transition constraints with degrees ranging from 1 (add, sub) to 7 (Poseidon2 hash rounds). classical R1CS can only express degree-2 constraints, requiring high-degree operations to be decomposed into many degree-2 gates — inflating constraint count.

CCS represents high-degree constraints natively. pattern 15 (hash, degree 7) costs only field operations in the [[SuperSpartan]] prover — no cryptographic cost increase over degree-1 constraints. the Poseidon2 rounds inside the hash pattern are free in the IOP layer.

## CCS and folding

[[HyperNova]] folding operates over CCS instances. since CCS already powers [[SuperSpartan]], the folding scheme and the [[stark]] system share the same constraint language. fold a [[cyberlink]] insertion proof? same CCS instance type. fold a rank update? same CCS. fold a cross-shard merge? same CCS. one framework for every proof in the [[zheng]] taxonomy.

see [[zheng]] for the proof system, [[SuperSpartan]] for the IOP, [[stark]] for the general theory
