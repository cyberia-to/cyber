---
tags: person
crystal-type: entity
crystal-domain: cybics
---
Italian cryptographer, researcher at Radboud University.

Lead author of [[Poseidon]] (2021), a hash function designed for zero-knowledge proof systems, optimized for arithmetic circuits over prime fields.

Poseidon achieves 8x fewer constraints than Pedersen hashes in SNARK/stark circuits, making ZK proofs practical for real workloads.

Research focuses on symmetric [[cryptography]], algebraic attacks, and hash function design for constrained environments.

Co-authored the HADES design strategy combining full and partial S-box rounds for efficiency without sacrificing security.

The [[cyber]] protocol uses Poseidon for all in-circuit hashing: commitments, nullifiers, and Merkle tree construction in the [[Goldilocks field]].