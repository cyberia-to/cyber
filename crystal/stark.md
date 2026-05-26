---
tags: cyber, core, cryptography
alias: starks, STARK, STARKs, Scalable Transparent Argument of Knowledge
crystal-type: entity
crystal-domain: cyber
crystal-size: bridge
---
# stark

Scalable Transparent Argument of Knowledge. a proof system where a prover convinces a verifier that a computation was correct — transparent setup, hash-only security, post-quantum.

the theoretical family on which [[zheng]] is built. [[zheng]] is the concrete proof system of [[cyber]]; stark is the science behind it.

## key properties

| property | value |
|---|---|
| trusted setup | none (transparent) |
| post-quantum | yes (hash-only security) |
| proof size | 60–200 KB |
| prover | quasi-linear O(N log N) |
| verifier | polylogarithmic O(log² N) |
| security assumption | collision-resistant hash function |

## stark vs snark

| property | SNARK | stark |
|---|---|---|
| trusted setup | required | transparent |
| quantum resistance | vulnerable | resistant |
| proof size | ~200 bytes | ~60-200 KB |
| security basis | discrete log / pairing | hash collision resistance |
| prover complexity | $O(N \log N)$ | $O(N)$ to $O(N \log N)$ |
| verifier complexity | $O(1)$ pairing | $O(\log^2 N)$ hash |

stark proofs are larger than SNARKs but the security model is strictly stronger: hash-only, no algebraic assumptions, no ceremony, no quantum vulnerability.

## zheng goes further

[[zheng]] builds on STARK theory but extends it with HyperNova folding — replacing batch proving with incremental accumulation. the protocol proof object (σ in a [[signal]]) is a HyperNova accumulator (~200 bytes), not a classical STARK proof. validators run `decide()` to verify. see [[zheng]] for the full architecture.

see [[zheng/docs/explanation/stark]] for the complete STARK theory, arithmetization, and heritage timeline.

discover all [[concepts]]
