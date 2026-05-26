---
alias: cryptography, modern cryptography, crypto primitives
tags: discipline, crypto, math, comp
crystal-type: entity
crystal-domain: crypto
---
# cryptography

the science of protecting information and proving statements about it. built on [[number theory]], [[algebra]], and computational complexity. four classical goals: confidentiality, integrity, authentication, non-repudiation. modern cryptography extends these to [[zero knowledge proofs]], [[homomorphic encryption]], and verifiable computation.

## [[crypto/hashing]]

a hash function maps arbitrary input to a fixed-size digest satisfying preimage resistance, second-preimage resistance, and collision resistance. prominent families: [[SHA-2]], [[Blake3]], [[Poseidon]]/Poseidon2 (algebraic, ZK-native). [[cyber]] uses [[Hemera]] (Poseidon2 over [[Goldilocks field]]).

## [[crypto/encryption]]

symmetric encryption (AES, ChaCha20) uses one shared key. asymmetric encryption (ECIES, ML-KEM, CSIDH) uses public/private key pairs. [[homomorphic encryption]] ([[TFHE]]) computes on ciphertext without decrypting. virtually all real-world systems use hybrid encryption.

## [[crypto/signatures]]

a digital signature binds a message to a signer. prominent schemes: EdDSA, Schnorr (aggregatable), BLS (cross-message aggregation), SPHINCS+ and ML-DSA (post-quantum). [[cyber]] replaces signatures with [[stark]] proofs of [[Hemera]] preimage knowledge.

## [[crypto/commitments]]

bind to a value without revealing it. hash commitments, Pedersen (information-theoretic hiding), KZG (trusted setup), [[WHIR]]/[[FRI]] (transparent, post-quantum). [[polynomial commitments]] — commit to a polynomial, prove evaluations — are the foundation of modern proof systems.

## [[crypto/key-exchange]]

two parties derive a shared secret over an insecure channel. ECDH (X25519) is the current standard. ML-KEM provides post-quantum security. CSIDH enables non-interactive key exchange for asynchronous systems.

## [[crypto/zero-knowledge]]

prove a statement without revealing anything beyond its truth. [[SNARKs]] (Groth16, PLONK) achieve small proofs with trusted setup. [[starks]] require no trusted setup and are post-quantum. recursive composition, [[folding]] (Nova, HyperNova), [[incrementally verifiable computation]], [[proof-carrying data]], and lookup arguments ([[LogUp]], Lasso) extend the paradigm to scalable verifiable computation.

## multi-party computation

n parties jointly compute a function on private inputs. no party learns anything beyond the output. protocols: Yao's garbled circuits (2-party), SPDZ (n-party, malicious security), secret sharing (Shamir, additive). requires an honest majority assumption. see [[privacy trilateral]]

## [[crypto/data-structures]]

data structures with built-in integrity: [[Merkle trees]], [[NMT]], [[MMR]], Verkle trees (vector commitments), [[hash path accumulators]], [[SWBF]], [[mutator set]], [[EdgeSet]], [[LogUp]], LtHash. erasure coding (Reed-Solomon) enables data availability sampling. see [[storage proofs]]

## [[crypto/quantum]]

Shor's algorithm breaks RSA, ECDSA, ECDH. Grover halves symmetric/hash security. NIST PQC standards (2024): ML-KEM (FIPS 203), ML-DSA (FIPS 204), SLH-DSA (FIPS 205). [[starks]], symmetric ciphers, and hash functions survive quantum.

## cyber's stack

[[cyber]] reduces the entire stack to one field, one hash, one VM, one proof system:

```
field:   Goldilocks (p = 2^64 - 2^32 + 1)
hash:    Hemera (Poseidon2 over Goldilocks) — ~736 constraints
IOP:     SuperSpartan (CCS/AIR via sumcheck) — linear-time prover
lens:    Brakedown (multilinear polynomial commitment) — 5 μs verification
VM:      nox (register machine over Goldilocks)
```

authentication via stark preimage proofs. encryption via lattice KEM (interactive) and CSIDH (non-interactive). graph state via [[NMT]], [[MMR]], [[SWBF]], [[EdgeSet]], [[LogUp]]. domain separation with one function, six roles:

```
H_edge(x)        = Hemera(0x01 | x)    edge hashing
H_commit(x)      = Hemera(0x02 | x)    record commitments
H_nullifier(x)   = Hemera(0x03 | x)    SWBF index derivation
H_merkle(x)      = Hemera(0x04 | x)    NMT and MMR nodes
H_fiat_shamir(x) = Hemera(0x05 | x)    Brakedown challenges
H_transcript(x)  = Hemera(0x06 | x)    proof transcript binding
```

see [[zheng]], [[cyber/proofs]], [[BBG]], [[cyber/identity]]