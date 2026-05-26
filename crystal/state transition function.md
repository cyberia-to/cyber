---
alias: stf
tags: cyber
crystal-type: entity
crystal-domain: computer science
---

The state transition function is the deterministic rule that maps a current [[state]] and an input to a new [[state]]. Given identical inputs and starting conditions, the function always produces the same output, ensuring reproducibility across all [[validators]].

In distributed consensus systems, the state transition function is the core of the replicated state machine. Every node in the network applies the same function to the same ordered sequence of transactions, arriving at identical [[state]] independently.

In [[cyber]], the [[tru]] implements the state transition function. It processes incoming [[signals]] from [[neurons]], updates the [[cybergraph]], recalculates [[cyberank]], and advances the protocol to the next [[step]].

Each invocation of the state transition function takes the current graph topology, the set of new [[signals]] in the block, and protocol parameters as inputs. The output includes updated link weights, rank scores, and resource allocations.

Correctness of the state transition function is critical. Any divergence between [[validators]] in how they compute state transitions leads to consensus failure and chain forks. [[Zheng]] proofs in [[v6]] provide cryptographic verification that the function was executed faithfully.

The deterministic nature of the state transition function means the entire history of [[cyber]] can be replayed from genesis by any observer with access to the block sequence.

discover all [[concepts]]