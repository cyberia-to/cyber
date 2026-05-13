---
alias: path hash accumulator, hash path accumulators
tags: cyber, cryptographic proofs
crystal-type: entity
crystal-domain: computer science
---
authenticated data structure that represents a path in a graph as a balanced or biased binary tree of [[hash]] digests

internal nodes store hashes of concatenated sub-paths

enables logarithmic-size [[cryptographic proofs]] for graph properties: connectivity, distance, type queries

core building block of [[authenticated_graphs]]

how it works

- given a path `v₀ → v₁ → ... → vₖ` in a graph
- build a binary tree over the path edges
- each leaf is the [[hash]] of an edge label or vertex attribute
- each internal node is `H(left_child || right_child)`
- the root digest commits to the entire path
- to prove a sub-path or property, reveal the sibling hashes along the tree (logarithmic in path length)

comparison with Merkle trees

- Merkle trees authenticate sets or sequences of data
- hash path accumulators authenticate paths in graphs specifically
- both use binary tree structure with [[hash]] nodes
- hash path accumulators are optimized for path queries (connectivity, reachability, shortest path)

role in [[folding]] and [[incrementally verifiable computation]]

- accumulators serve as the "running digest" in folding schemes
- in [[Nova]] and related schemes, the [[accumulator]] absorbs each new proof instance without fully verifying it
- the final accumulated value is then checked once via a single decider proof
- this is what makes IVC efficient: fold instead of verify at each step

dynamic variants

- dynamic authenticated forests support link/cut operations with `O(log n)` proofs and `O(n)` space
- paths are partitioned into solid and dashed segments whose accumulators are linked
- enables real-time updates as the [[cybergraph]] evolves

applications in [[cyber]]

- [[cybergraph]] path verification: prove that two [[particles]] are connected through a specific chain of [[cyberlinks]] without transmitting the full path
- [[authenticated_graphs]] with fractional cascading: hash path accumulators form the per-shard layer, with fractional cascading overlay for cross-shard queries
- focus proof infrastructure: every random-walk step in the [[convergence vm]] publishes its proof against the attention root, enabling anyone to recompute focus
- light client verification: [[neurons]] verify shard integrity with logarithmic bandwidth using path proofs
- negative proofs: prove that a forbidden relationship is absent via authenticated complement paths

zero-knowledge friendly variants

- when using [[hash]] functions like Poseidon, the accumulator tree can be verified inside a ZK circuit efficiently
- each hash costs ~736 constraints vs ~1 constraint per field operation
- this is why [[hash]] function choice (see ADR-001) is critical for accumulator performance in proof systems

related

- [[accumulator]]
- [[hash]]
- [[authenticated_graphs]]
- [[incrementally verifiable computation]]
- [[proof-carrying data]]
- [[folding]]
- [[cryptographic proofs]]
- [[cybergraph]]