---
tags: cyber, cip
alias: zheng, 証, proof system, zheng proof, zheng proofs
crystal-type: entity
crystal-domain: cyber
---
the proof system of [[cyber]]. builds on [[stark]] theory, then goes further: HyperNova folding replaces batch proving with incremental accumulation. zero trusted setup. post-quantum. sub-millisecond verification.

## architecture

| component | role |
|-----------|------|
| IOP | [[SuperSpartan]] — constraint verification via sumcheck over CCS |
| PCS | [[Brakedown]] — expander-graph codes, no Merkle trees, O(N) field ops |
| folding | [[HyperNova]] — folds proof instances into a running accumulator |
| hash | [[hemera]] — algebraic hash, binding commitment |
| field | [[nebu]] — Goldilocks arithmetic |
| VM | [[nox]] — execution trace IS the constraint system |

## the proof object

a zheng proof is a HyperNova accumulator (~200 bytes). it is not a classical STARK proof. the prover folds each computation step into the accumulator at ~30 field operations per step. validators run `decide()` to verify — one SuperSpartan + sumcheck + Brakedown check on the folded instance (10–50 μs).

the accumulator stays open so it can be folded further: signal accumulators fold into block accumulators, those fold into epoch accumulators. one `decide()` at the top verifies everything.

## how it differs from stark

| | classical stark | zheng |
|---|---|---|
| PCS | hash-based (FRI/WHIR) | Brakedown (expander codes) |
| proof object | self-contained, large (~60-157 KB) | accumulator, tiny (~200 bytes) |
| verification | immediate | requires `decide()` |
| proving | batch, O(N) | incremental, ~30 field ops/step |
| aggregation | recursive composition | folding hierarchy |

see [[stark]] for the theoretical foundation. see [[zheng/specs/proof-types]] for every proof type the protocol generates.

discover all [[concepts]]
