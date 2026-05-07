---
alias: commitment scheme, commitment schemes, crypto commitments
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
---
# crypto/commitments

bind to a value without revealing it, then open later with proof of what was committed. two phases: commit (produce binding token), reveal (open the commitment with proof).

## schemes

| scheme | assumption | hiding | binding | use case |
|---|---|---|---|---|
| hash commitment | collision resistance | computational | computational | simple commit-reveal, [[Merkle trees]] |
| Pedersen commitment | discrete log | perfect (information-theoretic) | computational | confidential transactions ([[Monero]], Mimblewimble) |
| KZG (Kate-Zaverucha-Goldberg) | bilinear pairings + trusted setup | computational | computational | [[polynomial commitments]], Ethereum EIP-4844 |
| [[WHIR]] / [[FRI]] | hash collision resistance | computational | computational | transparent [[polynomial commitments]], no trusted setup |

## [[polynomial commitments]]

a special case: commit to a polynomial, then prove evaluations at specific points without revealing the polynomial. the foundation of modern proof systems.

```
FRI (2018)  →  STIR (2024)  →  WHIR (2025)
baseline        fewer queries     richest queries (sumcheck + rate improvement)
306 KiB         160 KiB           157 KiB proofs
3.9 ms verify   3.8 ms verify     1.0 ms verify (290 us at 100-bit)
```

all three are Reed-Solomon proximity tests by Arnon, Chiesa, Fenzi, Yogev. [[WHIR]] achieves faster verification than even trusted-setup schemes (KZG: 2.4 ms vs WHIR: 290 us) while requiring no trusted setup and providing post-quantum security.

see [[FRI]], [[STIR]], [[WHIR]], [[polynomial commitment]], [[cryptography]]