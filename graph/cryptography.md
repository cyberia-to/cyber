---
alias: modern cryptography, crypto primitives
tags: computer science
crystal-type: entity
crystal-domain: computer science
stake: 7021323931679387
---
# cryptography

the science of protecting information and proving statements about it. four classical goals: confidentiality (only intended recipients read the data), integrity (data has not been altered), authentication (the sender is who they claim), non-repudiation (the sender cannot deny having sent it). modern cryptography extends these to zero-knowledge (prove a statement without revealing why it is true), homomorphic computation (compute on encrypted data), and verifiable computation (prove a program ran correctly).

## [[hashing]]

a hash function maps arbitrary input to a fixed-size digest. cryptographic hash functions satisfy three properties: preimage resistance (given H(x), hard to find x), second-preimage resistance (given x, hard to find x' ≠ x with H(x) = H(x')), collision resistance (hard to find any x ≠ x' with H(x) = H(x')).

| family | construction | digest | speed | STARK cost | status |
|---|---|---|---|---|---|
| [[SHA-2]] (SHA-256, SHA-512) | Merkle-Damgård | 256/512 bit | ~500 MB/s | ~25,000 constraints | standard since 2001, ubiquitous |
| [[SHA-3]] (Keccak) | sponge | 256/512 bit | ~400 MB/s | ~150,000 constraints | standard since 2015, backup family |
| [[BLAKE2]] / [[BLAKE3]] | Merkle tree + ChaCha | 256 bit | ~1 GB/s (BLAKE3) | ~10,000 constraints | fast software hash |
| [[Poseidon]] / Poseidon2 | algebraic sponge over prime field | field elements | ~300K hashes/s | ~250 constraints | ZK-native, 100× cheaper in circuits |

[[Poseidon]] and Poseidon2 are algebraic hashes designed for arithmetic circuits — they operate natively over prime fields, making them 100× cheaper inside [[STARK]] and [[SNARK]] proofs than binary hashes like SHA-256. the tradeoff: younger cryptanalysis, field-specific tuning required.

[[cyber]] uses [[Hemera]] (Poseidon2 over [[Goldilocks field]]) — see [[Hemera]], [[hemera/spec]]

## [[encryption]]

### symmetric encryption

one shared key for both encryption and decryption.

| cipher | type | key size | status |
|---|---|---|---|
| AES-128/256 | block cipher (SPN) | 128/256 bit | NIST standard, hardware-accelerated (AES-NI) |
| ChaCha20-Poly1305 | stream cipher + MAC | 256 bit | IETF standard, fast in software, used in TLS 1.3 and WireGuard |
| AES-256-GCM | authenticated encryption (AEAD) | 256 bit | most deployed AEAD mode |

AEAD (Authenticated Encryption with Associated Data) provides both confidentiality and integrity in a single operation. GCM and ChaCha20-Poly1305 are the two dominant AEAD modes.

### asymmetric encryption

a public key encrypts, the corresponding private key decrypts.

| scheme | assumption | key size | status |
|---|---|---|---|
| RSA-OAEP | integer factorization | 2048–4096 bit | legacy, being phased out |
| ECIES (over Curve25519, secp256k1) | elliptic curve discrete log | 256 bit | current standard for hybrid encryption |
| ML-KEM (CRYSTALS-Kyber) | Module-LWE | 800–1568 bytes | NIST PQC standard (FIPS 203), post-quantum |
| CSIDH / dCTIDH | supersingular isogeny class group | ~64 bytes | non-interactive key exchange, conjectured post-quantum |

hybrid encryption: encrypt a symmetric key with an asymmetric scheme, then encrypt the payload with the symmetric key. virtually all real-world systems use this pattern (TLS, Signal, age, GPG).

### [[homomorphic encryption]]

compute on ciphertext without decrypting. the result, when decrypted, equals the result of computing on the plaintext.

| scheme | operations | performance | use case |
|---|---|---|---|
| Paillier | addition only (partially homomorphic) | fast | voting, aggregation |
| BGV / BFV | addition + multiplication (somewhat homomorphic) | moderate | machine learning on encrypted data |
| [[TFHE]] | arbitrary boolean/arithmetic circuits (fully homomorphic) | ~10⁶× slower than plaintext | general-purpose encrypted computation |

[[TFHE]] (Fully Homomorphic Encryption over the Torus) enables arbitrary computation on encrypted data. the performance gap is shrinking — hardware accelerators and algorithmic improvements reduce overhead by 100–1000× compared to early FHE schemes.

## [[signature]]

a digital signature binds a message to a signer. anyone with the public key can verify, only the private key holder can sign.

| scheme | assumption | sig size | verify speed | status |
|---|---|---|---|---|
| RSA (PKCS#1 v1.5, PSS) | integer factorization | 256–512 bytes | fast | legacy, still widely deployed |
| ECDSA (secp256k1, P-256) | ECDLP | 64 bytes | moderate | [[Bitcoin]], [[Ethereum]], TLS |
| EdDSA (Ed25519, Ed448) | ECDLP (twisted Edwards) | 64 bytes | fast, deterministic | Signal, SSH, TLS 1.3 |
| Schnorr | discrete log | 64 bytes | fast, linearly aggregatable | [[Bitcoin]] Taproot (BIP 340) |
| BLS (BLS12-381) | bilinear pairings | 48 bytes | slow (pairing) | [[Ethereum]] 2.0 consensus, threshold sigs |
| SPHINCS+ / SLH-DSA | hash functions only | 7–49 KB | moderate | NIST PQC standard (FIPS 205), post-quantum |
| ML-DSA (CRYSTALS-Dilithium) | Module-LWE | 2.4–4.6 KB | fast | NIST PQC standard (FIPS 204), post-quantum |

Schnorr signatures enable native multi-signature aggregation: n signers produce one signature of the same size as a single signature. BLS signatures aggregate across different messages. both are foundations for scalable consensus.

an alternative: replace signatures with [[STARK]] proofs of hash preimage knowledge — no curves, no pairings, post-quantum from the hash alone. see [[cyber/identity]] for this approach.

## [[commitment scheme]]

bind to a value without revealing it, then open later with proof of what was committed.

| scheme | assumption | hiding | binding | use case |
|---|---|---|---|---|
| hash commitment | collision resistance | computational | computational | simple commit-reveal, [[Merkle trees]] |
| Pedersen commitment | discrete log | perfect (information-theoretic) | computational | confidential transactions ([[Monero]], Mimblewimble) |
| KZG (Kate-Zaverucha-Goldberg) | bilinear pairings + trusted setup | computational | computational | [[polynomial commitments]], Ethereum EIP-4844 |
| [[WHIR]] / [[FRI]] | hash collision resistance | computational | computational | transparent [[polynomial commitments]], no trusted setup |

[[polynomial commitments]] are a special case: commit to a polynomial, then prove evaluations at specific points without revealing the polynomial. they are the foundation of modern proof systems.

```
FRI (2018)  →  STIR (2024)  →  WHIR (2025)
baseline        fewer queries     richest queries (sumcheck + rate improvement)
306 KiB         160 KiB           157 KiB proofs
3.9 ms verify   3.8 ms verify     1.0 ms verify (290 μs at 100-bit)
```

all three are Reed-Solomon proximity tests by Arnon, Chiesa, Fenzi, Yogev. [[WHIR]] achieves faster verification than even trusted-setup schemes (KZG: 2.4 ms vs WHIR: 290 μs) while requiring no trusted setup and providing post-quantum security.

see [[FRI]], [[STIR]], [[WHIR]], [[polynomial commitment]]

## key exchange

two parties derive a shared secret over an insecure channel.

| protocol | assumption | interaction | status |
|---|---|---|---|
| Diffie-Hellman (DH) | discrete log | interactive | foundational (1976), broken by quantum |
| ECDH (X25519, X448) | ECDLP | interactive | current standard (TLS 1.3, Signal, WireGuard) |
| ML-KEM (CRYSTALS-Kyber) | Module-LWE | interactive (KEM) | NIST PQC standard, post-quantum |
| CSIDH / dCTIDH | supersingular isogeny | non-interactive | conjectured post-quantum, enables stealth addresses |

non-interactive key exchange (NIKE): both parties publish public keys, either can derive the shared secret without communication. classical DH and ECDH can be used this way (each publishes g^a, g^b). CSIDH provides NIKE with conjectured post-quantum security — valuable for asynchronous systems where parties may never be online simultaneously.

## [[zero knowledge proofs]]

prove a statement is true without revealing anything beyond the truth of the statement. a ZKP system has three properties: completeness (honest prover convinces verifier), soundness (cheating prover fails), zero-knowledge (verifier learns nothing beyond the statement's truth).

| system | setup | proof size | verify time | quantum safe | assumption |
|---|---|---|---|---|---|
| Groth16 | trusted (per-circuit) | 128 bytes | ~1 ms | no | bilinear pairings |
| PLONK / Halo2 | universal trusted | 400–800 bytes | ~3 ms | no | bilinear pairings |
| Bulletproofs | none | ~700 bytes | ~30 ms | no | discrete log |
| [[STARK]] | none (transparent) | 100–200 KB | 1–4 ms | yes | hash collision resistance |
| [[STARK]] + [[WHIR]] | none (transparent) | 60–157 KB | 0.3–1.0 ms | yes | hash collision resistance |

SNARKs (Succinct Non-interactive Arguments of Knowledge) achieve small proofs but typically require a trusted setup ceremony and rely on elliptic curve assumptions vulnerable to quantum computers. STARKs (Scalable Transparent Arguments of Knowledge) require no trusted setup and rely only on hash functions — post-quantum secure, larger proofs but faster verification (with WHIR).

### recursive composition

a proof system that can verify its own proofs enables recursive composition: prove that a verification of a proof was done correctly. the result is a constant-size proof regardless of the depth of recursion.

```
Level 0: Prove computation C → proof π₀
Level 1: Prove verify(π₀) → proof π₁ (same size)
Level N: Prove verify(π_{N-1}) → proof π_N (same size)

N proofs → one aggregated proof → O(1) verification
```

this is the foundation of rollups ([[Ethereum]] L2s), incrementally verifiable computation (IVC), and proof aggregation. systems: Nova (folding schemes), Halo2 (accumulation), STARKs (self-referential verification).

## multi-party computation

n parties jointly compute a function f(x₁, ..., x_n) where each party holds private input x_i. no party learns anything beyond the output. protocols: Yao's garbled circuits (2-party), SPDZ (n-party, malicious security), secret sharing (Shamir, additive).

the limitation: requires an honest majority assumption (typically n/2 or 2n/3 honest parties). the advantage: distributes trust — no single party controls the computation.

## the privacy trilateral

three complementary technologies cover each other's blind spots:

| technology | proves | hides | limitation |
|---|---|---|---|
| ZK ([[zero knowledge proofs]]) | computation correct | inputs and process | prover sees everything |
| FHE ([[homomorphic encryption]]) | nothing (computation only) | data from computer | heavy, no integrity proof |
| MPC (multi-party computation) | threshold agreement | individual shares | requires honest majority |

ZK proves without showing. FHE computes without seeing. MPC distributes without trusting. combined: trustless computation on private data with provable results. see [[privacy trilateral]]

## cryptographic data structures

data structures with built-in integrity guarantees via hashing or algebraic commitments.

### hash-based trees

| structure | property | used in |
|---|---|---|
| [[Merkle trees]] | membership proofs via hash paths, O(log n) | [[Bitcoin]], [[Ethereum]], certificate transparency, git |
| [[NMT]] (namespaced Merkle tree) | completeness proofs — prove ALL items in a namespace | [[Celestia]], [[cyber]] |
| [[MMR]] (Merkle mountain range) | append-only history, compact proofs, no rebalancing | Grin, [[neptune]], [[cyber]] |
| Patricia/MPT (Merkle Patricia trie) | key-value state with inclusion/exclusion proofs | [[Ethereum]] state tree |
| sparse Merkle tree | efficient non-membership proofs via default hashes | Cosmos, various L2s, Libra |
| Verkle tree | vector commitments replace hashes — O(log n) proof vs O(k log n) | [[Ethereum]] roadmap (replaces MPT) |

Verkle trees (Kuszmaul, 2019) replace hash-based branching with vector commitments (KZG or IPA). each internal node commits to its children as a vector rather than hashing them pairwise. the result: proof size is O(log n) regardless of branching factor k, vs O(k log n) for Merkle. this enables wide branching (k = 256) with small proofs — critical for stateless clients.

### accumulators

cryptographic accumulators represent arbitrarily large sets with a single constant-size value and prove membership in O(1).

| accumulator | assumption | membership | non-membership | dynamic | used in |
|---|---|---|---|---|---|
| RSA accumulator | strong RSA | O(1) proof | O(1) proof | yes (with trapdoor) | Zerocoin, stateless [[Bitcoin]] proposals |
| bilinear accumulator | bilinear pairings | O(1) proof | O(1) proof | yes | anonymous credentials |
| Merkle tree | collision resistance | O(log n) proof | O(log n) (sparse) | yes | everywhere (quasi-accumulator) |
| [[polynomial commitment]] | varies (KZG/WHIR) | O(1) amortized | O(1) amortized | yes | [[EdgeSet]], modern proof systems |

RSA and bilinear accumulators achieve constant-size proofs but require stronger assumptions (strong RSA, pairings). hash-based accumulators (Merkle trees) have logarithmic proofs but minimal assumptions. polynomial commitments achieve amortized O(1) via batching.

### probabilistic and append-only structures

| structure | property | used in |
|---|---|---|
| Bloom filter | probabilistic membership, false positives, compact | network protocols, caching, spam filters |
| cuckoo filter | probabilistic membership with deletion support | database lookups, deduplication |
| [[SWBF]] (sliding-window Bloom filter) | probabilistic membership with windowed removal | [[neptune]], [[cyber]] (nullifier tracking) |
| [[mutator set]] | UTXO privacy (AOCL + SWBF) | [[neptune]], [[cyber]] |
| append-only log (certificate transparency) | tamper-evident log via Merkle tree, public auditability | Google CT (2013), TLS certificate ecosystem |

### algebraic structures

| structure | property | used in |
|---|---|---|
| vector commitments (KZG, IPA) | commit to a vector, open at any index with O(1) proof | Verkle trees, [[Ethereum]] EIP-4844 |
| [[polynomial commitment]] | commit to polynomial, prove evaluations | [[STARK]], PLONK, [[cyber]] ([[WHIR]]-based) |
| [[EdgeSet]] | edge membership via polynomial commitment | [[cyber]] [[BBG]] |
| [[LogUp]] | cross-index consistency via algebraic lookup | Polygon, Scroll, [[cyber]] |
| authenticated skip list | O(log n) membership with authenticated pointers | early blockchain designs, distributed databases |

## quantum resistance

a sufficiently large quantum computer running Shor's algorithm breaks RSA, ECDSA, ECDH, and all discrete-log or factoring-based schemes. Grover's algorithm halves the effective security of symmetric ciphers and hash functions (AES-128 → 64-bit security, SHA-256 → 128-bit).

### NIST Post-Quantum Cryptography standards (2024)

| standard | scheme | type | basis |
|---|---|---|---|
| FIPS 203 (ML-KEM) | CRYSTALS-Kyber | key encapsulation | Module-LWE |
| FIPS 204 (ML-DSA) | CRYSTALS-Dilithium | digital signature | Module-LWE |
| FIPS 205 (SLH-DSA) | SPHINCS+ | digital signature | hash functions only |

lattice-based schemes (ML-KEM, ML-DSA) offer compact keys and fast operations. hash-based signatures (SLH-DSA) rely on the minimal assumption — hash collision resistance — but produce larger signatures (7–49 KB).

### what survives quantum computers

| primitive | quantum status | reason |
|---|---|---|
| AES-256 | safe (128-bit effective) | Grover halves security, 256 → 128 is sufficient |
| SHA-256, SHA-3-256 | safe (128-bit effective) | Grover halves, 256 → 128 is sufficient |
| [[STARK]] proofs | post-quantum | rely only on hash collision resistance |
| lattice KEM/signatures | post-quantum | no known quantum algorithm for Module-LWE |
| hash-based signatures | post-quantum | rely only on hash preimage/collision resistance |
| RSA, ECDSA, ECDH | broken | Shor's algorithm solves factoring and discrete log |
| BLS, KZG | broken | pairing-based, reduces to discrete log |

## storage and availability proofs

at planetary scale, content loss is the existential risk. cryptographic proofs can guarantee data survives:

| proof type | guarantees |
|---|---|
| storage proof (proof of space) | content bytes exist on specific storage |
| size proof | claimed content size matches actual byte count |
| replication proof | k independent copies exist |
| retrievability proof | content fetchable within bounded time |
| data availability (DAS) | block data was published and is accessible |
| encoding fraud proof | erasure coding was done correctly |

Filecoin uses proof-of-replication and proof-of-spacetime. [[Celestia]] pioneered namespace-aware DAS with [[NMT]]. see [[storage proofs]]

## cyber's cryptographic stack

[[cyber]] reduces the entire stack to one field, one hash, one VM, one proof system:

```
field:   Goldilocks (p = 2⁶⁴ − 2³² + 1)
hash:    Hemera (Poseidon2 over Goldilocks) — ~250 constraints
proofs:  STARK with WHIR low-degree testing — 290 μs verification
VM:      nox (register machine over Goldilocks)
```

authentication replaces [[signatures]] with STARK proofs of [[Hemera]] preimage knowledge. encryption uses lattice KEM (interactive) and CSIDH (non-interactive). the [[cybergraph]] state is authenticated via [[NMT]], [[MMR]], [[SWBF]], [[EdgeSet]], and [[LogUp]] — see [[BBG]] for the full architecture, [[cyber/proofs]] for the complete proof taxonomy, [[cyber/identity]] for the privacy layers.

```
H_edge(x)        = Hemera(0x01 ‖ x)    edge hashing
H_commit(x)      = Hemera(0x02 ‖ x)    record commitments
H_nullifier(x)   = Hemera(0x03 ‖ x)    SWBF index derivation
H_merkle(x)      = Hemera(0x04 ‖ x)    NMT and MMR nodes
H_fiat_shamir(x) = Hemera(0x05 ‖ x)    WHIR challenges
H_transcript(x)  = Hemera(0x06 ‖ x)    proof transcript binding
```

domain separation at the input, one function, six roles. post-quantum from genesis (hash-only authentication and anonymity). see [[data structure for superintelligence]] for the full specification.
