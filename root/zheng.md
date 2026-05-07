---
tags: cyber, cip
alias: zheng, 証, proof system, cyber stark, cyber starks
crystal-type: entity
crystal-domain: cyber
---
the [[proof system]] of the [[cyber]] protocol. [[SuperSpartan]] IOP + [[WHIR]] PCS + [[sumcheck]] protocol. zero trusted setup, post-quantum, sub-millisecond verification.

| component | role |
|-----------|------|
| IOP | [[SuperSpartan]] — one commitment, one opening per proof |
| PCS | [[WHIR]] — polynomial commitment over [[Goldilocks field]] |
| hash | [[hemera]] — algebraic hash for all commitments |
| field | [[nebu]] — Goldilocks arithmetic |
| VM | [[nox]] — execution trace IS the constraint system |

when a [[neuron]] submits a [[cyberlink]] or any other state-changing operation, zheng generates a succinct proof that the transition follows protocol rules. [[validators]] verify these proofs instead of re-executing every computation, dramatically reducing the cost of consensus.

zheng enables recursive proof composition, allowing multiple state transitions to be batched and verified in a single step. this capability scales the protocol toward handling planetary-level [[cybergraph]] operations.

as a transparent proof system, zheng requires no trusted setup ceremony. the security rests entirely on hash function collision resistance and the algebraic structure of the [[STARK]] protocol.

by separating proof generation from verification, zheng allows lightweight clients to trust the [[cybergraph]] state without running a full node, bringing verifiable knowledge to every device.

discover all [[concepts]]
