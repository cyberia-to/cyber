---
tags: cybernomics
crystal-type: entity
crystal-domain: economics
---
An accumulator is a [[cryptographic]] data structure that compactly represents a set of elements while supporting efficient membership proofs. A single fixed-size digest can attest to the inclusion of any element in the accumulated set.

In the [[cyber]] protocol, accumulators provide state commitments for the [[cybergraph]]. Rather than transmitting the entire state, a [[neuron]] can verify that a specific [[particle]] or [[cyberlink]] belongs to the current state by checking a short proof against the accumulator value.

Accumulators pair naturally with [[hemera]] and [[zheng]]. The hash function produces the digests that feed into the accumulator, while the proof system verifies membership claims inside arithmetic circuits without revealing the full dataset.

This structure enables light clients to participate in the network with minimal storage. A device holding only the current accumulator value can authenticate any piece of knowledge returned by a full node.

Dynamic accumulators allow both additions and deletions, tracking the evolving state of the [[cybergraph]] as [[neurons]] create and retire links. Each update produces a new digest without reprocessing the entire history.

By compressing set membership into constant-size proofs, accumulators make planetary-scale [[knowledge graph]] verification practical on resource-constrained devices.

discover all [[concepts]]