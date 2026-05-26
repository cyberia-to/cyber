---
tags: cyber, cryptography
alias: Poseidon2 hash, poseidon hash
crystal-type: entity
crystal-domain: cyber
---

an algebraic hash function optimized for [[stark]] proof circuits over the [[goldilocks field]]. the specific instantiation used by [[hemera]]

## design

poseidon2 operates over a state vector of field elements with a substitution-permutation network:

$$\text{Poseidon2}: \mathbb{F}_p^t \;\to\; \mathbb{F}_p^c$$

where $p = 2^{64} - 2^{32} + 1$ (the [[goldilocks field]]), $t$ is the state width, and $c$ is the output capacity. the standard configuration outputs 8 field elements

## structure

each round applies three operations in sequence:

1. round constant addition — breaks symmetry
2. S-box $x \mapsto x^7$ — the nonlinear layer, chosen for minimal constraint degree over $\mathbb{F}_p$
3. linear mixing — an MDS matrix diffuses the state

full rounds apply the S-box to every element; partial rounds apply it to one element only. this split minimizes the total constraint count in arithmetic circuits

## performance

the [[goldilocks field]] enables $x^7$ with just 3 multiplications. combined with the partial round optimization, poseidon2 achieves the lowest constraint count per hash among algebraic hash families. this directly translates to faster [[stark]] proof generation in [[hemera]]

## usage in cyber

[[hemera]] uses poseidon2 as the internal hash for merkle commitments, [[cyberlink]] authentication, and [[mutator set]] accumulators. every proof in the system bottlenecks on hashing — the choice of poseidon2 determines the throughput ceiling

see [[hemera]], [[stark]], [[goldilocks field]], [[mutator set]], [[bbg]]
