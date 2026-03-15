---
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
alias: CCS, Customizable Constraint Systems
---
# SuperSpartan

an Interactive Oracle Proof for Customizable Constraint Systems (CCS) — a generalization that simultaneously captures R1CS, Plonkish, and AIR without overhead. Setty, Thaler, Wahby (2023). ePrint 2023/552.

CCS unifies the three major arithmetization approaches into one framework. a proof system handling CCS handles all of them:

| arithmetization | used by | degree | CCS encoding |
|---|---|---|---|
| R1CS | Groth16, Spartan, Nova | 2 | 3 matrices, 2 terms |
| Plonkish | PLONK, Halo2 | custom gates | selector matrices |
| AIR | [[stark]], StarkWare/CAIRO | any | shifted row matrices |

SuperSpartan uses the [[sumcheck]] protocol as its core mechanism. for AIR instances (the case relevant to [[cyber]]):

```
1. prover commits execution trace as multilinear polynomial → PCS commitment
2. verifier sends random challenges
3. sumcheck reduces constraint satisfaction over ALL trace rows
   to evaluating the trace polynomial at ONE random point
4. PCS opens the commitment at that point
5. verifier checks: sumcheck transcript + PCS opening
```

key properties:
- linear-time prover: constraint verification requires field operations only, no FFT/NTT
- logarithmic-time verifier: for AIR, O(log n) without preprocessing
- high-degree constraints: prover cost scales with degree only in field operations (no cryptographic cost increase)
- PCS-agnostic: works with any polynomial commitment scheme

when the PCS is [[WHIR]], the result is Whirlaway (LambdaClass, 2025) — a multilinear [[stark]] with transparent setup, post-quantum security, and sub-millisecond verification. this is the architecture [[cyber]] uses.

see [[stark]] for the full proof system, [[WHIR]] for the polynomial commitment scheme, [[sumcheck]] for the core protocol
