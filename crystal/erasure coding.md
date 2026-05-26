---
tags: comp, coding theory
crystal-type: entity
crystal-domain: comp
alias: erasure codes
---
# erasure coding

erasure coding splits data into k original chunks and generates n-k parity chunks such that any k of the n total chunks are sufficient to reconstruct the original data. the redundancy ratio is n/k, and the code tolerates up to n-k chunk losses

## Reed-Solomon

the standard construction for blockchain data availability is Reed-Solomon coding. the data chunks are treated as evaluations of a polynomial of degree k-1. the parity chunks are additional evaluations of the same polynomial at distinct points. polynomial interpolation from any k points recovers the original polynomial and thus the original data

Reed-Solomon codes over the [[Goldilocks field]] (p = 2^64 - 2^32 + 1) are efficient for NTT-based encoding and are used in [[BBG]] for data availability

## properties

- MDS (maximum distance separable): achieves the theoretical optimum — any k of n chunks suffice
- systematic: the first k chunks are the original data, unchanged
- encoding complexity: O(n log n) using NTT (number-theoretic transform)
- decoding complexity: O(k log^2 k) using fast polynomial interpolation

## role in cyber

[[BBG]] encodes block data with 2D Reed-Solomon erasure coding to enable [[DAS]]. validators store and serve coded chunks. light nodes sample random chunks and verify them against commitments. the erasure coding guarantee ensures that if enough samples succeed, the full data is reconstructable by anyone

---

see [[DAS]] for the sampling protocol built on erasure coding. see [[data availability]] for the broader design. see [[Goldilocks field]] for the arithmetic field
