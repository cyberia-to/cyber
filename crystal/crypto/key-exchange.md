---
alias: key exchange, key exchange protocols, crypto key exchange
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
---
# crypto/key-exchange

two parties derive a shared secret over an insecure channel.

## protocols

| protocol | assumption | interaction | status |
|---|---|---|---|
| Diffie-Hellman (DH) | discrete log | interactive | foundational (1976), broken by quantum |
| ECDH (X25519, X448) | ECDLP | interactive | current standard (TLS 1.3, Signal, WireGuard) |
| [[ML-KEM]] (CRYSTALS-Kyber) | Module-LWE | interactive (KEM) | NIST PQC standard, post-quantum |
| CSIDH / [[dCTIDH]] | supersingular isogeny | non-interactive | conjectured post-quantum, enables stealth addresses |

## non-interactive key exchange

non-interactive key exchange (NIKE): both parties publish public keys, either can derive the shared secret without communication. classical DH and ECDH can be used this way (each publishes g^a, g^b). CSIDH provides NIKE with conjectured post-quantum security — valuable for asynchronous systems where parties may never be online simultaneously.

see [[cryptography]], [[crypto/encryption]], [[crypto/quantum]]