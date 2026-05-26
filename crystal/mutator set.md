---
tags: cyber, cryptography
alias: mutator sets
crystal-type: entity
crystal-domain: cyber
---

a cryptographic accumulator that enables private [[cyberlinks]]. tracks spent nullifiers without revealing which specific records were consumed

## mechanism

a mutator set $\mathcal{M}$ maintains two components:

$$\mathcal{M} = (\mathcal{A},\; \mathcal{N})$$

| component | role |
|---|---|
| $\mathcal{A}$ | accumulator — a compact commitment to the set of active items |
| $\mathcal{N}$ | nullifier set — records of consumed items, preventing double-spend |

a [[neuron]] proves membership of an item in $\mathcal{A}$ and publishes a nullifier to $\mathcal{N}$ without linking the two. the accumulator updates in constant time regardless of set size

## privacy guarantees

the observer sees that some valid item was consumed. the observer cannot determine which item. this unlocks anonymous graph operations: a [[neuron]] can create [[cyberlinks]] from previously accumulated [[particles]] without revealing its identity or history

## integration

mutator sets live inside [[bbg]], anchored to the authenticated state. every namespace in the [[cybergraph]] can optionally enable private linking through its own mutator set instance. [[hemera]] provides the hash primitives that keep the accumulator compact

see [[bbg]], [[cyberlinks]], [[neuron]], [[hemera]], [[cybergraph]]
