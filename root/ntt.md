---
tags: cyber, mathematics, cryptography
crystal-type: entity
crystal-domain: cyber
alias: NTT, number theoretic transform
---

Number Theoretic Transform: a discrete Fourier transform over a finite field, enabling fast polynomial multiplication in [[stark]] proof systems over the [[goldilocks field]]

NTT replaces complex roots of unity with primitive roots in a prime field, achieving O(n log n) polynomial multiplication with exact integer arithmetic

in [[cyber]], NTT is a core operation in [[STARK]] proof generation, where large polynomial evaluations and interpolations dominate computation cost

efficient NTT implementations exploit the structure of [[Mersenne prime]]s and [[goldilocks field]] primes to maximize hardware parallelism
