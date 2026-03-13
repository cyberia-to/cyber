---
alias: dctidh
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
---
# dCTIDH

optimized constant-time implementation of [[CSIDH]] (Commutative Supersingular Isogeny Diffie-Hellman). a non-interactive key exchange primitive with conjectured post-quantum security.

## mechanism

a class group acts on supersingular elliptic curves over a prime field F_p. each party's secret key is a vector of integer exponents; the public key is the resulting curve.

```
Setup:
  E₀: supersingular elliptic curve over F_p
  Class group action: [a] · E₀ = E_a    (secret isogeny)

Alice:  secret a → public E_a = [a] · E₀
Bob:    secret b → public E_b = [b] · E₀

Shared secret:
  Alice computes: [a] · E_b = [a] · [b] · E₀
  Bob computes:   [b] · E_a = [b] · [a] · E₀

  [a]·[b]·E₀ = [b]·[a]·E₀               commutativity
```

commutativity is the key property: the class group action commutes, so both parties arrive at the same curve without exchanging messages. this is non-interactive key exchange (NIKE).

## why dCTIDH over CSIDH

the original CSIDH implementation leaks timing information through variable-time isogeny computation. dCTIDH uses dummy isogenies and constant-time arithmetic to resist side-channel attacks. "d" stands for dummy-free (using a division-based approach), "CT" for constant-time.

## parameters

| variant | classical security | public key size | status |
|---|---|---|---|
| dCTIDH-512 | ~64 bit | 64 bytes | research |
| dCTIDH-1024 | ~128 bit | 128 bytes | research |
| dCTIDH-2048 | ~256 bit | 256 bytes | research |

public keys are remarkably compact compared to lattice-based schemes (ML-KEM public keys are 800-1568 bytes).

## applications in [[cyber]]

- stealth addresses: sender creates a [[cyberlink]] detectable and decryptable only by the intended recipient, without prior communication
- non-interactive key exchange: two [[neurons]] derive a shared secret from public graph data, no message round-trip needed
- anonymous channels: the shared secret reveals nothing about which [[neurons]] communicate

## tradeoffs

- slower than lattice KEM (~5x for dCTIDH-2048 vs ML-KEM)
- the isogeny assumption is less studied than lattice assumptions — SIDH was broken in 2022, though CSIDH survived those attacks
- active research area, not yet standardized

see [[crypto/key-exchange]], [[crypto/encryption]], [[crypto/quantum]], [[cryptography]]
