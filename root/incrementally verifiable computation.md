---
alias: IVC
tags: cyber, cryptographic proofs
crystal-type: process
crystal-domain: computer science
---
paradigm where a long computation is broken into steps, and each step produces a [[cryptographic proof]] that

- the previous step's proof was valid
- one more unit of computation was performed correctly

enables [[verification]] of an arbitrarily long computation by checking only the final proof

key insight: the [[verifier]] never needs to see intermediate states, only the succinct proof at the end

[[prover]] at step `i` takes

- the proof from step `i-1`
- current input
- produces a new proof that covers all steps `1..i`

foundational construction for recursive proof composition

closely related to [[proof-carrying data]] which generalizes IVC from chains to DAGs

relies on [[folding]] as the efficient mechanism to absorb a proof into an [[accumulator]] rather than fully verifying it at each step

constructions

- [[Nova]]: first practical folding scheme for R1CS, achieves IVC without SNARKs at each step
- [[SuperNova]]: extends Nova to support multiple instruction types (non-uniform IVC)
- [[HyperNova]]: generalizes folding to customizable constraint systems (CCS)
- [[Protostar]]: non-uniform IVC with support for high-degree gates and lookups

applications in [[cyber]]

- verifiable [[cybergraph]] state transitions: prove a chain of [[cyberlink]] insertions is valid
- incremental [[convergence vm]] updates: each rank recomputation proves correctness of the previous one
- light client protocols: a [[neuron]] can verify the full history of a shard by checking one proof
- scalable [[validator]] pipelines: validators fold block proofs instead of re-executing all transactions

properties

- succinctness: proof size is constant or logarithmic regardless of computation length
- incrementality: each step adds only marginal cost over a single proof
- composability: IVC proofs can be further composed with [[proof-carrying data]] for DAG-structured computations

related

- [[folding]]
- [[proof-carrying data]]
- [[hash path accumulator]]
- [[cryptographic proofs]]
- [[interactive proofs]]
- [[authenticated_graphs]]