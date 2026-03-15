---
alias: lattice KEM, ML-KEM, CRYSTALS-Kyber, lattice key encapsulation
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
---
# crypto/lattice-KEM

key encapsulation mechanism based on the hardness of Module-LWE (Learning With Errors) over structured lattices. the sender encapsulates a shared secret under the receiver's public key; the receiver decapsulates with the secret key. interactive — the receiver must publish a public key first.

## ML-KEM (FIPS 203)

NIST standardized CRYSTALS-Kyber as ML-KEM in 2024. three parameter sets:

| parameter set | classical security | public key | ciphertext | shared secret |
|---|---|---|---|---|
| ML-KEM-512 | 128 bit | 800 bytes | 768 bytes | 32 bytes |
| ML-KEM-768 | 192 bit | 1184 bytes | 1088 bytes | 32 bytes |
| ML-KEM-1024 | 256 bit | 1568 bytes | 1568 bytes | 32 bytes |

post-quantum secure: no known quantum algorithm solves Module-LWE efficiently.

## Module-RLWE variant over Goldilocks

in [[cyber]], the lattice KEM operates over Module-RLWE (Ring Learning With Errors) with [[Goldilocks field]] arithmetic (p = 2^64 - 2^32 + 1). the same field used by [[Hemera]], [[nox]], and [[stark]] verification — native arithmetic, no field conversion.

```
LATTICE KEM PROTOCOL
════════════════════

Setup:
  Ring R = Z_p[x] / (x^64 + 1)        cyclotomic polynomial, degree 64
  Module dimension: 4×4 over R
  Field: p = 2^64 - 2^32 + 1           Goldilocks

keygen():
  secret s ← small_distribution(R^4)
  public A ← uniform(R^{4×4})
  public b = A·s + e                    e ← error_distribution
  return (sk=s, pk=(A, b))

enc(pk, message):
  r ← small_distribution(R^4)
  ciphertext_1 = A^T · r + e'
  ciphertext_2 = b^T · r + e'' + encode(message)
  return (c1, c2)

dec(sk, c1, c2):
  message = decode(c2 - s^T · c1)
  return message
```

the security assumption: given A and b = A·s + e, recovering s is computationally hard. the error term e masks the secret — without it, the system would be solvable by linear algebra.

## comparison with [[dCTIDH]]

| property | lattice KEM | [[dCTIDH]] |
|---|---|---|
| interaction | interactive (receiver publishes key first) | non-interactive (NIKE) |
| quantum security | post-quantum (Module-LWE) | conjectured post-quantum (isogeny class group) |
| public key size | 800-1568 bytes | 64-256 bytes |
| performance | fast | ~5x slower |
| standardization | NIST FIPS 203 (2024) | research |

lattice KEM handles the interactive case ([[neptune]] approach). [[dCTIDH]] covers non-interactive scenarios like stealth addresses and anonymous channels where no prior communication is possible.

## applications in [[cyber]]

- private [[neuron]]-to-[[neuron]] data: receiver publishes a lattice public key as a [[particle]], sender encrypts [[cyberlink]] metadata
- encrypted [[spell]] parameters
- private [[particle]] delivery

see [[crypto/key-exchange]], [[crypto/encryption]], [[crypto/quantum]], [[cryptography]]
