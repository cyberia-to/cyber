---
tags: cyber, core
alias: lenses, PCS, polynomial commitment scheme, polynomial commitment
crystal-type: entity
crystal-domain: crypto
---
a lens is how an algebra presents its computation for verification. each of the [[five algebras]] computes in its own structure — scalars, binary, rings, semirings, isogenies. the lens makes that computation verifiable without revealing the polynomial.

same laws of verification ([[SuperSpartan]] + [[sumcheck]]). different lenses for different structures.

## five lenses

| lens | algebra | what it sees |
|------|---------|-------------|
| Brakedown | [[nebu]] (F_p, F_p², F_p³, F_p⁴) | scalar and extension field polynomials |
| Binius | [[kuro]] (F₂) | binary tower polynomials |
| Ring-aware | [[jali]] (R_q) | polynomial ring operations via NTT batching |
| Isogeny | [[genies]] (F_q) | supersingular curve polynomials |
| Tropical | [[trop]] (min,+) | optimization witnesses via dual certificates |

## three operations

```
commit(polynomial) → 32 bytes          seal the computation
open(polynomial, point) → proof        reveal one evaluation
verify(commitment, point, value) → ok  check without seeing
```

## three roles

proof commitment — seal a [[nox]] execution trace for verification
state commitment — seal [[bbg]] polynomial state for authenticated queries
noun identity — seal a [[nox]] noun for content addressing

one trait. five lenses. three roles. see [[zheng]] for the proof system that uses them.

discover all [[concepts]]
