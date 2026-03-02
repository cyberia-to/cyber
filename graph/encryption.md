---
tags: computer science, cryptography
crystal-type: process
crystal-domain: computer science
stake: 4710011005933789
---
Transformation of plaintext into ciphertext using a key, rendering data unreadable to anyone without the corresponding decryption key. The guardian of secrets in digital systems.

## symmetric encryption

One key for both encryption and decryption. Fast, used for bulk data.
- AES (Advanced Encryption Standard): block cipher, 128/192/256-bit keys, dominant standard
- ChaCha20: stream cipher, used in TLS, fast in software

## asymmetric encryption

Public key encrypts, private key decrypts. Enables secure communication without shared secrets.
- RSA: based on difficulty of factoring large primes
- elliptic curve cryptography (ECC): shorter keys, equivalent security, used in Bitcoin and [[cyber]]
- Diffie-Hellman: key exchange protocol, establishing shared secrets over public channels

## hybrid approach

Asymmetric encryption exchanges a symmetric session key; symmetric encryption handles the data. TLS/HTTPS works this way.

## applications

- secure communication: HTTPS, Signal protocol, end-to-end messaging
- digital signatures: proving authorship and integrity, authentication in [[cyber]] and [[bostrom]]
- disk encryption: protecting data at rest
- [[zero knowledge proofs]]: advanced cryptographic protocols built on encryption primitives

## connections

Security relies on [[complexity theory]] -- the hardness of factoring and discrete logarithm. [[compilers]] implement cryptographic libraries. [[consensus algorithms]] use encryption for message authentication.
