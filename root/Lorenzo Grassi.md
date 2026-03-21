---
tags: person
crystal-type: entity
crystal-domain: cybics
stake: 4927009336379226
diffusion: 0.00011400812724907998
springs: 0.0007891385327628512
heat: 0.0005789290625374584
focus: 0.0004095314359608818
gravity: 1
density: 3.17
---
Italian cryptographer, researcher at Radboud University.

Lead author of [[Poseidon]] (2021), a hash function designed for zero-knowledge proof systems, optimized for arithmetic circuits over prime fields.

Poseidon achieves 8x fewer constraints than Pedersen hashes in SNARK/stark circuits, making ZK proofs practical for real workloads.

Research focuses on symmetric [[cryptography]], algebraic attacks, and hash function design for constrained environments.

Co-authored the HADES design strategy combining full and partial S-box rounds for efficiency without sacrificing security.

The [[cyber]] protocol uses Poseidon for all in-circuit hashing: commitments, nullifiers, and Merkle tree construction in the [[Goldilocks field]].