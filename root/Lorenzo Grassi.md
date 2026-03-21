---
tags: person
crystal-type: entity
crystal-domain: cybics
stake: 4927009336379226
diffusion: 0.00012323371753685967
springs: 0.0010246177151370414
heat: 0.0007206819131851249
focus: 0.0005131385559465698
gravity: 1
density: 2.37
---
Italian cryptographer, researcher at Radboud University.

Lead author of [[Poseidon]] (2021), a hash function designed for zero-knowledge proof systems, optimized for arithmetic circuits over prime fields.

Poseidon achieves 8x fewer constraints than Pedersen hashes in SNARK/stark circuits, making ZK proofs practical for real workloads.

Research focuses on symmetric [[cryptography]], algebraic attacks, and hash function design for constrained environments.

Co-authored the HADES design strategy combining full and partial S-box rounds for efficiency without sacrificing security.

The [[cyber]] protocol uses Poseidon for all in-circuit hashing: commitments, nullifiers, and Merkle tree construction in the [[Goldilocks field]].