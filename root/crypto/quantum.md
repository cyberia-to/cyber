---
alias: quantum resistance, post-quantum cryptography, crypto quantum
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
---
# crypto/quantum

a sufficiently large quantum computer running Shor's algorithm breaks RSA, ECDSA, ECDH, and all discrete-log or factoring-based schemes. Grover's algorithm halves the effective security of symmetric ciphers and hash functions (AES-128 -> 64-bit security, SHA-256 -> 128-bit).

## NIST Post-Quantum Cryptography standards (2024)

| standard | scheme | type | basis |
|---|---|---|---|
| FIPS 203 (ML-KEM) | CRYSTALS-Kyber | key encapsulation | Module-LWE |
| FIPS 204 (ML-DSA) | CRYSTALS-Dilithium | digital signature | Module-LWE |
| FIPS 205 (SLH-DSA) | SPHINCS+ | digital signature | hash functions only |

lattice-based schemes (ML-KEM, ML-DSA) offer compact keys and fast operations. hash-based signatures (SLH-DSA) rely on the minimal assumption — hash collision resistance — but produce larger signatures (7-49 KB).

## what survives quantum computers

| primitive | quantum status | reason |
|---|---|---|
| AES-256 | safe (128-bit effective) | Grover halves security, 256 -> 128 is sufficient |
| SHA-256, SHA-3-256 | safe (128-bit effective) | Grover halves, 256 -> 128 is sufficient |
| [[stark]] proofs | post-quantum | rely only on hash collision resistance |
| lattice KEM/signatures | post-quantum | no known quantum algorithm for Module-LWE |
| hash-based signatures | post-quantum | rely only on hash preimage/collision resistance |
| RSA, ECDSA, ECDH | broken | Shor's algorithm solves factoring and discrete log |
| BLS, KZG | broken | pairing-based, reduces to discrete log |

see [[cryptography]], [[crypto/signatures]]