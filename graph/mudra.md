---
tags: cyber
alias: mudra, मुद्रा, crypto primitives
crystal-type: entity
crystal-domain: cyber
subgraph: true
repo: ../mudra
exclude: ".claude/**, target/**, CLAUDE.md"
---
post-quantum cryptographic primitives for [[cyber]]. four modules, four security assumptions, one repo. mudra (मुद्रा — seal/gesture in Sanskrit) provides the cryptographic gestures that [[cyber]] needs beyond the hash-based stack.

## modules

| module | primitive | security assumption |
|--------|----------|-------------------|
| kem | lattice KEM (Module-RLWE) | NIST FIPS 203 |
| ctidh | dCTIDH (isogeny NIKE) | CSIDH (conjectured post-quantum) |
| tfhe | fully homomorphic encryption | LWE |
| threshold | Shamir SSS, VSS, DKG | information-theoretic + hash |

each module has its own security boundary. they share no cryptographic code with each other. [[hemera]] provides commitments for verifiable secret sharing in the threshold module. [[aurum]] provides field arithmetic for lattice KEM and TFHE polynomial rings.

## dependency graph

```
aurum (field)
  ↓
hemera (hash)
  ↓
mudra (crypto) ← this repo
```

mudra is consumed at the protocol/node level — not part of the core proof pipeline (aurum → hemera → nox → zheng → bbg).

see [[lattice KEM]] for interactive key exchange, [[dCTIDH]] for non-interactive key exchange, [[TFHE]] for homomorphic encryption
