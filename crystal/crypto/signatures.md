---
alias: signature, signatures, digital signature, cryptographic signature, crypto signatures
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
---
# crypto/signatures

a digital signature binds a message to a signer. anyone with the [[public key]] can verify, only the [[private key]] holder can sign.

## schemes

| scheme | assumption | sig size | verify speed | status |
|---|---|---|---|---|
| RSA (PKCS#1 v1.5, PSS) | integer factorization | 256-512 bytes | fast | legacy, still widely deployed |
| ECDSA (secp256k1, P-256) | ECDLP | 64 bytes | moderate | [[Bitcoin]], [[Ethereum]], TLS |
| EdDSA (Ed25519, Ed448) | ECDLP (twisted Edwards) | 64 bytes | fast, deterministic | Signal, SSH, TLS 1.3 |
| Schnorr | discrete log | 64 bytes | fast, linearly aggregatable | [[Bitcoin]] Taproot (BIP 340) |
| BLS (BLS12-381) | bilinear pairings | 48 bytes | slow (pairing) | [[Ethereum]] 2.0 consensus, threshold sigs |
| SPHINCS+ / SLH-DSA | hash functions only | 7-49 KB | moderate | NIST PQC standard (FIPS 205), post-quantum |
| ML-DSA (CRYSTALS-Dilithium) | Module-LWE | 2.4-4.6 KB | fast | NIST PQC standard (FIPS 204), post-quantum |

Schnorr signatures enable native multi-signature aggregation: n signers produce one signature of the same size as a single signature. BLS signatures aggregate across different messages. both are foundations for scalable consensus.

an alternative: replace signatures with [[stark]] proofs of hash preimage knowledge — no curves, no pairings, post-quantum from the hash alone. see [[cyber/identity]] for this approach.

see [[cryptography]], [[crypto/quantum]]