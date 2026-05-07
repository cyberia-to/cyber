---
alias: hashing, crypto hashing, hash families
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
---
# crypto/hashing

a [[hash]] function maps arbitrary input to a fixed-size digest. cryptographic hash functions satisfy three properties: preimage resistance (given H(x), hard to find x), second-preimage resistance (given x, hard to find x' with H(x) = H(x')), collision resistance (hard to find any x, x' with H(x) = H(x')).

## families

| family | construction | digest | speed | stark cost | status |
|---|---|---|---|---|---|
| [[SHA-2]] (SHA-256, SHA-512) | Merkle-Damgard | 256/512 bit | ~500 MB/s | ~25,000 constraints | standard since 2001, ubiquitous |
| [[SHA-3]] (Keccak) | sponge | 256/512 bit | ~400 MB/s | ~150,000 constraints | standard since 2015, backup family |
| [[Blake2]] / [[Blake3]] | Merkle tree + ChaCha | 256 bit | ~1 GB/s (Blake3) | ~50,000-100,000 constraints | fast software hash |
| [[Poseidon]] / Poseidon2 | algebraic sponge over prime field | field elements | ~300K hashes/s | ~736 constraints | ZK-native, 100x cheaper in circuits |

## algebraic hashes

[[Poseidon]] and Poseidon2 are algebraic hashes designed for arithmetic circuits — they operate natively over prime fields, making them 100x cheaper inside [[stark]] and [[SNARK]] proofs than binary hashes like SHA-256. the tradeoff: younger cryptanalysis, field-specific tuning required.

[[cyber]] uses [[Hemera]] (Poseidon2 over [[Goldilocks field]]) — see [[Hemera]], [[hemera/spec]], [[hash function selection]] for the full decision record. see [[crypto/hash/features]] for the complete feature taxonomy.

see [[cryptography]]