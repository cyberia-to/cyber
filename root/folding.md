---
tags: cyber, cryptographic proofs
crystal-type: process
crystal-domain: computer science
---
technique where instead of fully verifying a [[cryptographic proof]], you absorb it into an [[accumulator]]

the accumulated instance can be checked once at the end via a single decider verification

key enabler of efficient [[incrementally verifiable computation]] and [[proof-carrying data]]

intuition

- traditional approach: at each step, verify the previous proof (expensive, recursive SNARK)
- folding approach: at each step, combine the new instance with the accumulated instance using a cheap algebraic operation
- defer the expensive check to the very end

how it works

- given two instances of a relation (e.g. R1CS), a folding scheme produces a single instance that is satisfiable if and only if both originals were
- the [[prover]] sends a small cross-term or error vector
- the [[verifier]] computes a random linear combination to merge them
- cost per fold: a few field operations and one hash, instead of a full proof verification

schemes

- [[Nova]]: folding for R1CS (rank-1 constraint systems), the first practical folding scheme
- [[SuperNova]]: non-uniform IVC via folding with multiple circuit types
- [[HyperNova]]: folding for CCS (customizable constraint systems), generalizes R1CS, Plonkish, AIR
- [[Protostar]]: supports high-degree gates, lookups, and non-uniform computation
- ProtoGalaxy: multi-instance folding with logarithmic verifier cost

relation to accumulators

- folding is the mechanism; the [[accumulator]] is the state
- each fold updates the accumulator with a new instance
- the [[hash path accumulator]] is one concrete data structure that can serve as the running state

applications in [[cyber]]

- fold [[cyberlink]] insertion proofs across blocks instead of re-verifying the full chain
- fold [[convergence vm]] rank updates incrementally as new links arrive
- fold cross-shard proofs when merging [[authenticated_graphs]] digests

related

- [[incrementally verifiable computation]]
- [[proof-carrying data]]
- [[hash path accumulator]]
- [[accumulator]]
- [[cryptographic proofs]]