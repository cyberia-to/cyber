---
tags: cyber, core
alias: lenses, PCS, polynomial commitment scheme, polynomial commitment
crystal-type: entity
crystal-domain: crypto
---
the commitment layer for [[cyber]]. five polynomial commitment backends — one per [[five algebras|algebra]]. the layer between [[hemera]] (identity) and [[nox]] (execution).

a lens makes computation verifiable: commit to a polynomial, prove evaluations, verify without seeing the polynomial. each algebra sees through its own optic. same laws of verification ([[SuperSpartan]] + [[sumcheck]]). different lenses for different structures.

```
hemera → lens → nox → zheng → bbg
```

## five lenses

| full name | construction | impl | what it sees |
|-----------|-------------|------|-------------|
| scalar field | Brakedown | [[nebu]] | F_p scalar and extension polynomials |
| binary tower | Binius | [[kuro]] | F₂ tower polynomials |
| polynomial ring | Ikat | [[jali]] | R_q ring operations via NTT batching |
| tropical semiring | Assayer | [[trop]] | optimization witnesses via dual certificates |
| isogeny curves | Porphyry | [[genies]] | F_q curve polynomials |

## three operations

```
commit(polynomial) → 32 bytes          seal the computation
open(polynomial, point) → proof        reveal one evaluation
verify(commitment, point, value) → ok  check without seeing
```

## three roles

proof commitment — seal a [[nox]] execution trace for [[zheng]] verification
state commitment — seal [[bbg]] polynomial state for authenticated queries
noun identity — seal a [[nox]] noun for content addressing

## consumers

| consumer | uses lens for |
|----------|-------------|
| [[nox]] | noun identity: hemera(Lens.commit(noun_poly) ‖ tag) |
| [[zheng]] | proof commitment: SuperSpartan queries Lens |
| [[bbg]] | state root: BBG_root = hemera(Lens.commit(BBG_poly) ‖ ...) |

one trait. five lenses. three roles. three consumers.

discover all [[concepts]]
