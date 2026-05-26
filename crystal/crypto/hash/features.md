---
tags: cyber, article, compound
---

# Complete Feature Taxonomy of [[hash]] Functions

A compact reference to every capability a [[hash]] function can provide.

See also: [[hash function selection]], [[hashing]], [[hashing and confidentiality]]

## 1. Classical Security Properties

| Property | Definition |
|---|---|
| Preimage resistance | Given h, finding m such that H(m) = h requires brute force |
| Second preimage resistance | Given m₁, finding m₂ ≠ m₁ with H(m₁) = H(m₂) requires brute force |
| Collision resistance | Finding any m₁ ≠ m₂ with H(m₁) = H(m₂) requires brute force |
| Random oracle behavior | Output indistinguishable from random for any input |
| Length extension resistance | Given H(m), computing H(m‖suffix) requires knowing m |
| Near-collision resistance | Finding inputs whose hashes differ in few bits requires brute force |
| Multi-target resistance | Security holds constant regardless of number of targets |

These properties form the foundation of [[cryptography]]. See [[cryptographic proofs]] for how they compose into larger arguments.

## 2. Performance Properties

| Property | What it means | Examples |
|---|---|---|
| Hardware throughput | Raw GB/s on commodity CPUs | Blake3 ~2 GB/s, SHA-256 ~500 MB/s |
| SIMD acceleration | Exploits vector instructions (AVX2/512, NEON) | Blake3, SHA-NI |
| Parallelizability | Splits work across cores for single input | Blake3 (tree mode), ParallelHash |
| Latency | Time-to-first-hash for small inputs | Matters for interactive apps |
| Constant-time | Free of timing side channels (no secret-dependent branches) | Critical for key derivation |
| Small message efficiency | Fast for inputs < 1KB | Some sponge constructions have high initialization cost |

## 3. Structural / Compositional Properties

| Property | Definition | Who has it |
|---|---|---|
| Tree hashing | Built-in Merkle tree mode for parallel [[hashing]] of large inputs | Blake3, KangarooTwelve, ParallelHash |
| Incremental hashing | Update hash when input changes without full rehash | Homomorphic hashes (LtHash, AdHash) |
| Streaming | Process input in chunks without buffering entire input | All sponge/Merkle-Damgård constructions |
| Verified streaming | Verify data integrity chunk-by-chunk as it arrives (Bao) | Blake3 (native via Bao), must be built for others |
| Slice proofs | Prove integrity of arbitrary byte range without full content | Blake3/Bao, any tree-hash with proof extraction |
| Extendable output (XOF) | Produce arbitrary-length output | SHAKE128/256, Blake3, KangarooTwelve |
| Sponge construction | Absorb-then-squeeze paradigm, yields both hash and XOF | SHA-3/Keccak, Poseidon, Rescue, Tip5 |
| Compression function | Fixed-input-length primitive, used inside Merkle-Damgård or standalone | Poseidon2, Anemoi/Jive |
| Domain separation | Provably different outputs for different use cases from same primitive | Blake3 (key derivation, MAC, hash all from same core) |
| Duplex construction | Interleave absorb/squeeze for online authenticated [[encryption]] | Keccak duplex, Xoodyak |

See [[hash path accumulator]] for how these compositional properties enable accumulator constructions.

## 4. Algebraic / Proof-System Properties

The new frontier — properties that matter for [[zero knowledge proofs]], MPC, and FHE.

| Property | Definition | Who has it |
|---|---|---|
| Arithmetization-friendly | Low multiplicative complexity when expressed as arithmetic circuit over 𝔽ₚ | Poseidon/2, Rescue, Griffin, Anemoi, Tip5, Monolith, MiMC |
| R1CS-efficient | Few constraints in Rank-1 Constraint Systems (Groth16) | Poseidon, Anemoi (~2× better than Poseidon) |
| Plonk-efficient | Few constraints in Plonkish arithmetization | Poseidon2, Anemoi (21-35% better than Poseidon) |
| AIR/stark-efficient | Low trace width and degree in Algebraic Intermediate Representation | Tip5, Monolith, RPO (Rescue Prime Optimized) |
| Lookup-compatible | Uses lookup tables for nonlinearity (requires lookup arguments in prover) | Tip5, Monolith, Reinforced Concrete |
| Field-native | Operates natively over a specific prime field | All AO hashes; field choice matters ([[Goldilocks field]], BN254, BLS12-381) |
| Low multiplicative depth | Few sequential multiplications (critical for MPC and FHE) | Poseidon2, MiMC |
| MPC-friendly | Efficient in multi-party computation protocols | Poseidon2, LowMC, PASTA |
| FHE-friendly | Efficient under [[Goldilocks homomorphic encryption]] and other FHE schemes | LowMC, PRINCE, SIMON (low AND-depth) |
| Recursive-proof friendly | Cheap enough to verify inside itself for [[proof-carrying data]] composition | Tip5 (designed specifically for this), Poseidon2 |

See [[zheng]] for stark verification in the cyber protocol. [[incrementally verifiable computation]] relies on recursive-proof friendly hashes.

### S-box Design Strategies (determines security/efficiency tradeoff)

| Strategy | How it works | Algebraic degree | Examples |
|---|---|---|---|
| Low-degree power map | x → x^d (d=3,5,7) | Low per round, accumulates | Poseidon, Rescue, Griffin |
| Split-and-lookup | Decompose field element → small S-box per chunk → reassemble | High (~p) from lookup | Tip5, Monolith, Reinforced Concrete |
| Feistel/Lai-Massey | Structured permutation over field pairs | Depends on round function | Neptune (hash), Anemoi/Flystel |
| Legendre symbol | x → Legendre(x) ∈ {-1,0,1} | High-degree map, cheap to evaluate | Grendel |

### Security Tradeoffs in AO Hashes

| Attack class | What it exploits | Strong against it | Weak against it |
|---|---|---|---|
| Groebner basis | Low-degree polynomial representation | Lookup-based (Tip5, Monolith) | Low-degree power maps (Poseidon) |
| Resultant attacks | Structured polynomial systems | — | Poseidon, Griffin, Neptune (CRYPTO 2025 improvements) |
| CICO problem | Constrained-Input-Constrained-Output on sponge | High-degree S-boxes | Recent breaks on some low-degree constructions |
| Statistical/differential | Probability propagation through rounds | Power maps | Lookup-based (need careful design) |
| Subspace trails | Linear subspace propagation through partial rounds | Full-round S-box layers | Partial-round designs (Hades strategy) |

## 5. Content Addressing Properties

| Property | Definition | Who has it |
|---|---|---|
| Deterministic identity | Same bytes → same hash, always | All raw-byte hashes (distinct from CIDv0/UnixFS) |
| Deduplication | Identify identical content by hash equality | Universal property |
| Self-certifying names | Hash IS the unforgeable name of content | [[ipfs]] CIDs, Blake3 content addresses |
| Multihash/multicodec | Self-describing hash format (includes algorithm ID + length) | CIDv1 ecosystem, extensible |
| Content integrity | Verify content matches its hash | Universal |
| Content availability proofs | Prove you store content without revealing it | Algebraic hashes + stark (Filecoin) |
| Provable replication | Prove unique copy exists on specific storage | Filecoin PoRep (slow encoding via VDF-like construction) |

In [[cyber]], every [[particle]] is content-addressed. See [[hash function selection]] for how the protocol chose its hash. [[data availability strategy]] covers availability in the cyber context.

## 6. Homomorphic Properties

| Property | Definition | Examples |
|---|---|---|
| Additive homomorphism | H(A∪B) = H(A) + H(B) for set hashing | LtHash, AdHash, ECMH, MuHash, HexaMorphHash |
| Incremental updates | H(S ∪ {x}) = H(S) + H(x), H(S \ {x}) = H(S) - H(x) | LtHash (Facebook), HexaMorphHash |
| Lattice-based | Security from Short Integer Solution (SIS) problem, post-quantum | HexaMorphHash |
| Elliptic-curve-based | Security from ECDLP, compact digests | ECMH, MuHash |
| Multiset hashing | Order-independent hash of collections | All homomorphic hashes above |

Use case: [[databases]] integrity where elements are frequently added/removed without rehashing everything.

## 7. Similarity / Fuzzy Hashing Properties

| Property | Definition | Examples |
|---|---|---|
| Locality-sensitive (LSH) | Similar inputs → same bucket with high probability | MinHash, SimHash, p-stable LSH |
| Locality-preserving | Preserves neighborhood structure in reduced dimension | LPH, data-dependent methods |
| Perceptual hashing | Visually similar images → similar hashes | pHash, dHash, DinoHash (adversarially robust, DINOV2-based) |
| Fuzzy hashing | Similar files → similar hashes (edit-distance aware) | ssdeep, TLSH, sdhash |
| Minhash | Estimate Jaccard similarity between sets | Used in deduplication, plagiarism detection |

Critical difference: [[cryptography]] hashes maximize avalanche (1-[[bit]] change → 50% bits flip). Similarity hashes minimize it for similar inputs.

## 8. Time-Based Properties

| Property | Definition | Examples |
|---|---|---|
| Verifiable Delay Function (VDF) | Requires sequential time T to compute, O(log T) to verify | Wesolowski, Pietrzak (RSA groups) |
| Time-lock puzzle | Encrypt message recoverable only after T sequential steps | RSA time-lock (Rivest-Shamir-Wagner) |
| Proof of Sequential Work | Prove T sequential hash iterations were performed | Cohen-Pietrzak (Chia blockchain) |
| Iterated hashing | H(H(H(...H(x)...))) as proof of elapsed time | SHA-256 chains (Bitcoin-adjacent) |
| Slow encoding | Transform data such that encoding takes time T, decoding is fast | Filecoin PoRep |

VDFs and [[proof of work]] both rely on sequential computation. See [[consensus algorithms]] for how these properties compose into consensus mechanisms.

## 9. Quantum Resistance Properties

| Property | Status |
|---|---|
| Grover's algorithm | Halves effective security bits: 256-[[bit]] hash → 128-bit quantum security |
| Hash-based [[signature]] | Post-quantum secure (SPHINCS+, XMSS, LMS) — rely only on hash preimage resistance |
| Lattice-based hash | Post-quantum from SIS/LWE hardness assumptions |
| VDF quantum vulnerability | RSA/DLP-based VDFs broken by Shor's algorithm; hash-based VDFs survive |
| AO hash quantum status | Algebraic hashes over large fields: Grover applies to preimage, algebraic structure may introduce additional quantum attack surfaces (under-studied) |

## 10. Protocol-Level Properties

Properties that emerge when hashes are composed into larger [[data structures]]:

| Property | Definition | Relevant hash feature |
|---|---|---|
| Merkle tree | O(log n) proof of membership in set | Any hash; algebraic hashes give cheap in-circuit verification |
| Merkle Mountain Range (MMR) | Append-only accumulator with O(log n) proofs | Any hash; on-chain MMR needs algebraic hash |
| Sparse Merkle tree | Prove membership AND non-membership | Any hash; expensive in ZK without algebraic hash |
| Vector commitment | Commit to vector, open at any position | Poseidon-based, or polynomial commitments |
| Accumulator | Compact representation of set with membership proofs | RSA accumulator, hash-based (Merkle) |
| Hash chain | Sequential composition for authentication/[[signature]] | MPC-friendly chains need AO hashes |
| Key derivation (KDF) | Derive keys from shared secret | HKDF (HMAC-based), Argon2 |
| Password hashing | Intentionally slow, memory-hard | Argon2, bcrypt, scrypt |
| Commitment scheme | H(m‖r) commits to m without revealing it | Any collision-resistant hash |
| Random oracle | Idealized hash modeling for proof techniques | Instantiated by SHA-256, BLAKE, etc. |

See [[authenticated_graphs]] for how hash-based structures enable verifiable graph [[data structures]]. The [[cybergraph]] uses these properties to maintain provable state.

## 11. The Landscape: Every Named Hash Function Family

### Classical (bit-oriented, hardware-optimized)
SHA-256, SHA-3/Keccak, BLAKE2/3, MD5†, RIPEMD-160

### AO: Pure Power Map (no lookups)
MiMC, GMiMC, Poseidon, Poseidon2, Neptune, Rescue/RPO, Griffin, Anemoi, Ciminion

### AO: Lookup-Based (require lookup arguments)
Reinforced Concrete, Tip5, Monolith, Skyscraper

### AO: Legendre/Other
Grendel (Legendre symbol), RAIN

### FHE-Friendly
LowMC, PRINCE, SIMON/SPECK, PASTA, Kreyvium

### Homomorphic Set Hashing
LtHash, AdHash, MuHash, ECMH, HexaMorphHash

### Similarity/Fuzzy
MinHash, SimHash, ssdeep, TLSH, pHash, DinoHash

### Password/KDF
Argon2, bcrypt, scrypt, PBKDF2, HKDF, Balloon

### VDF-Adjacent
Iterated SHA-256, Sloth, MinRoot, MiMC-based VDFs

See [[universal hash]] for a democratic [[proof of work]] [[algorithms]] approach.

## 12. Decision Matrix: Which Features Matter For What

| Building... | Must-have features | Nice-to-have | Irrelevant |
|---|---|---|---|
| Content-addressed storage | Deterministic identity, fast throughput, verified streaming | Slice proofs, tree hashing | Algebraic, homomorphic |
| ZK proof system | Arithmetization-friendly, field-native, low constraint count | Lookup compatibility, recursive-proof friendly | Throughput, verified streaming |
| Provable storage network | Algebraic + content addressing, Merkle proofs in-circuit | Slow encoding (PoRep), availability proofs | Similarity, password hardness |
| Blockchain [[consensus]] | Collision resistance, VDF/PoSW capability | Post-quantum | Algebraic, FHE |
| MPC protocol | Low multiplicative depth, MPC-friendly | Post-quantum | Throughput, content addressing |
| Encrypted computation (FHE) | Low AND-depth, FHE-friendly | — | Most other properties |
| [[databases]] integrity | Incremental/homomorphic, fast updates | Post-quantum (HexaMorphHash) | Algebraic, ZK |
| Document deduplication | Fast throughput, similarity hashing | Fuzzy hashing | Everything else |
| [[knowledge graph]] with proofs | ALL OF: deterministic identity + algebraic + tree hashing + in-circuit Merkle + content addressing | Homomorphic, recursive proofs, VDF | Similarity, password |

## 13. The Impossibility Constraints

Every hash function optimizes for a subset of axes. The fundamental tensions:

1. Speed vs. Algebraic Efficiency — Bit-oriented operations (rotations, XOR) are fast on CPUs but catastrophically expensive in arithmetic circuits. Algebraic operations (field multiplication, power maps) are cheap in circuits but 10-100× slower on CPUs.

2. Portability vs. Optimization — Lookup-based hashes (Tip5, Monolith) are faster in proof systems that support lookups, but incompatible with systems that lack lookup arguments. Power-map hashes (Poseidon2) work everywhere but are slower.

3. Cryptanalysis Maturity vs. Innovation — Newer designs (Tip5, Monolith) may have better theoretical properties but less real-world cryptanalysis. Older designs (SHA-256, even Poseidon) have more known attacks — which paradoxically means better-understood security margins.

4. Field Choice Locks Ecosystem — An algebraic hash over [[Goldilocks field]] is cheap in Plonky2/Miden/Triton VM but expensive in BN254-based SNARKs (Groth16/Ethereum). And vice versa. The field IS the ecosystem choice. See [[Goldilocks field processor]] for dedicated hardware.

5. Similarity vs. Security — Locality-sensitive [[hashing]] requires collisions by design. Cryptographic [[hashing]] requires collision resistance. These are mathematically opposed goals.

Every hash function is a point in this multidimensional feature space. The art is knowing which dimensions matter for your system. For how [[cyber]] navigates this space, see [[data structure for superintelligence]].