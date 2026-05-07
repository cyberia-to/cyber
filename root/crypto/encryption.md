---
alias: encryption, crypto encryption
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
---
# crypto/encryption

transformation of plaintext into ciphertext using a key, rendering data unreadable without the corresponding decryption key.

## symmetric encryption

one shared key for both encryption and decryption.

| cipher | type | key size | status |
|---|---|---|---|
| AES-128/256 | block cipher (SPN) | 128/256 bit | NIST standard, hardware-accelerated (AES-NI) |
| ChaCha20-Poly1305 | stream cipher + MAC | 256 bit | IETF standard, fast in software, used in TLS 1.3 and WireGuard |
| AES-256-GCM | authenticated encryption (AEAD) | 256 bit | most deployed AEAD mode |

AEAD (Authenticated Encryption with Associated Data) provides both confidentiality and integrity in a single operation. GCM and ChaCha20-Poly1305 are the two dominant AEAD modes.

## asymmetric encryption

a public key encrypts, the corresponding private key decrypts.

| scheme | assumption | key size | status |
|---|---|---|---|
| RSA-OAEP | integer factorization | 2048-4096 bit | legacy, being phased out |
| ECIES (over Curve25519, secp256k1) | elliptic curve discrete log | 256 bit | current standard for hybrid encryption |
| [[ML-KEM]] (CRYSTALS-Kyber) | Module-LWE | 800-1568 bytes | NIST PQC standard (FIPS 203), post-quantum |
| CSIDH / [[dCTIDH]] | supersingular isogeny class group | ~64 bytes | non-interactive key exchange, conjectured post-quantum |

hybrid encryption: encrypt a symmetric key with an asymmetric scheme, then encrypt the payload with the symmetric key. virtually all real-world systems use this pattern (TLS, Signal, age, GPG).

## [[homomorphic encryption]]

compute on ciphertext without decrypting. the result, when decrypted, equals the result of computing on the plaintext.

| scheme | operations | performance | use case |
|---|---|---|---|
| Paillier | addition only (partially homomorphic) | fast | voting, aggregation |
| BGV / BFV | addition + multiplication (somewhat homomorphic) | moderate | machine learning on encrypted data |
| [[TFHE]] | arbitrary boolean/arithmetic circuits (fully homomorphic) | ~10^6x slower than plaintext | general-purpose encrypted computation |

[[TFHE]] (Fully Homomorphic Encryption over the Torus) enables arbitrary computation on encrypted data. the performance gap is shrinking — hardware accelerators and algorithmic improvements reduce overhead by 100-1000x compared to early FHE schemes.

see [[cryptography]], [[crypto/key-exchange]]