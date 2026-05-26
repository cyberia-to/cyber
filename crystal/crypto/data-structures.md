---
alias: cryptographic data structures, crypto data structures
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
---
# crypto/data-structures

data structures with built-in integrity guarantees via hashing or algebraic commitments.

## hash-based trees

| structure | property | used in |
|---|---|---|
| [[Merkle trees]] | membership proofs via hash paths, O(log n) | [[Bitcoin]], [[Ethereum]], certificate transparency, git |
| [[NMT]] (namespaced Merkle tree) | completeness proofs — prove ALL items in a namespace | [[Celestia]], [[cyber]] |
| [[MMR]] (Merkle mountain range) | append-only history, compact proofs, no rebalancing | Grin, [[neptune]], [[cyber]] |
| Patricia/MPT (Merkle Patricia trie) | key-value state with inclusion/exclusion proofs | [[Ethereum]] state tree |
| sparse Merkle tree | efficient non-membership proofs via default hashes | Cosmos, various L2s, Libra |
| Verkle tree | vector commitments replace hashes — O(log n) proof vs O(k log n) | [[Ethereum]] roadmap (replaces MPT) |

Verkle trees (Kuszmaul, 2019) replace hash-based branching with vector commitments (KZG or IPA). each internal node commits to its children as a vector rather than hashing them pairwise. the result: proof size is O(log n) regardless of branching factor k, vs O(k log n) for Merkle. this enables wide branching (k = 256) with small proofs — critical for stateless clients.

## accumulators

cryptographic accumulators represent arbitrarily large sets with a single constant-size value and prove membership in O(1).

| accumulator | assumption | membership | non-membership | dynamic | used in |
|---|---|---|---|---|---|
| RSA accumulator | strong RSA | O(1) proof | O(1) proof | yes (with trapdoor) | Zerocoin, stateless [[Bitcoin]] proposals |
| bilinear accumulator | bilinear pairings | O(1) proof | O(1) proof | yes | anonymous credentials |
| Merkle tree | collision resistance | O(log n) proof | O(log n) (sparse) | yes | everywhere (quasi-accumulator) |
| [[polynomial commitment]] | varies (KZG/WHIR) | O(1) amortized | O(1) amortized | yes | [[EdgeSet]], modern proof systems |
| [[hash path accumulator]] | collision resistance | O(log k) path proof | O(log k) absence proof | yes (dynamic forests) | [[cyber]] [[cybergraph]], authenticated graphs |

RSA and bilinear accumulators achieve constant-size proofs but require stronger assumptions (strong RSA, pairings). hash-based accumulators (Merkle trees) have logarithmic proofs but minimal assumptions. polynomial commitments achieve amortized O(1) via batching.

[[hash path accumulators]] extend the accumulator concept from sets to graphs — they authenticate paths rather than elements. a path v0 -> v1 -> ... -> vk is represented as a binary tree of hash digests, enabling O(log k) proofs for connectivity, distance, and reachability. dynamic variants support link/cut with O(log n) updates as the graph evolves. ZK-friendly when built with algebraic hashes ([[Hemera]]). used in [[cyber]] for [[cybergraph]] path verification, light client proofs, and as the running digest in [[folding]] schemes for [[incrementally verifiable computation]].

## probabilistic and append-only structures

| structure | property | used in |
|---|---|---|
| Bloom filter | probabilistic membership, false positives, compact | network protocols, caching, spam filters |
| cuckoo filter | probabilistic membership with deletion support | database lookups, deduplication |
| [[SWBF]] (sliding-window Bloom filter) | probabilistic membership with windowed removal | [[neptune]], [[cyber]] (nullifier tracking) |
| [[mutator set]] | UTXO privacy (AOCL + SWBF) | [[neptune]], [[cyber]] |
| append-only log (certificate transparency) | tamper-evident log via Merkle tree, public auditability | Google CT (2013), TLS certificate ecosystem |

## algebraic structures

| structure | property | used in |
|---|---|---|
| vector commitments (KZG, IPA) | commit to a vector, open at any index with O(1) proof | Verkle trees, [[Ethereum]] EIP-4844 |
| [[polynomial commitment]] | commit to polynomial, prove evaluations | [[stark]], PLONK, [[cyber]] (Brakedown-based) |
| [[EdgeSet]] | edge membership via polynomial commitment | [[cyber]] [[BBG]] |
| [[LogUp]] | cross-index consistency via algebraic lookup | Polygon, Scroll, [[cyber]] |
| LtHash (lattice hash) | homomorphic set commitment — add/remove elements in O(1) | [[cyber]] collection state |
| authenticated skip list | O(log n) membership with authenticated pointers | early blockchain designs, distributed databases |

LtHash is an additive homomorphic commitment over a finite field: `add(state, H(x))` and `remove(state, H(x))` are single field operations. merging two sets is field addition. stark-provable at zero cost (addition is linear in arithmetization). used in [[cyber]] for O(1) shard and neuron state updates.

## erasure coding

transform data so it can be recovered from any k-of-n fragments. the cryptographic application: data availability — prove that block data was published without downloading all of it.

| code | rate | recovery | used in |
|---|---|---|---|
| Reed-Solomon | k/n | any k of n fragments | [[Celestia]], [[cyber]] DAS, RAID-6 |
| LDPC (Low-Density Parity Check) | variable | iterative decoding | 5G, Wi-Fi, deep space |
| fountain codes (LT, Raptor) | rateless | any k+e fragments | content distribution, streaming |

[[Celestia]] and [[cyber]] arrange block data in a sqrt(n) x sqrt(n) grid, erasure-code rows and columns with Reed-Solomon over [[Goldilocks field]], then commit each row via [[NMT]]. light clients sample random cells — O(sqrt(n)) samples for 99.9% confidence that all data is available. incorrect encoding is caught by encoding fraud proofs. see [[storage proofs]], [[NMT]]

see [[cryptography]]