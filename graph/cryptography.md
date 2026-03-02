---
alias: modern cryptography, crypto primitives
tags: cyber, computer science
crystal-type: entity
crystal-domain: computer science
stake: 7021323931679387
---
# cryptography

the science of proving things without revealing them. [[cyber]] reduces its entire cryptographic stack to one field, one hash, one VM, one proof system — then builds identity, privacy, communication, consensus, and storage from that foundation.

## the stack

```
┌──────────────────────────────────────────────────────────────────┐
│  APPLICATION        identity, communication, privacy, consensus   │
├──────────────────────────────────────────────────────────────────┤
│  PROOF SYSTEM       STARK (FRI → STIR → WHIR)                    │
├──────────────────────────────────────────────────────────────────┤
│  DATA STRUCTURES    NMT, MMR, SWBF, EdgeSet, LogUp               │
├──────────────────────────────────────────────────────────────────┤
│  COMMITMENTS        polynomial commitments (FRI-based)            │
├──────────────────────────────────────────────────────────────────┤
│  HASH               Hemera (Poseidon2-Goldilocks)                 │
├──────────────────────────────────────────────────────────────────┤
│  FIELD              Goldilocks (p = 2⁶⁴ − 2³² + 1)              │
└──────────────────────────────────────────────────────────────────┘
```

## primitives

### [[hashing]]

one-way functions. the foundation of content addressing, commitment, authentication, and proof construction. [[cyber]] uses [[Hemera]] (Poseidon2 over [[Goldilocks field]]) — ~250 STARK constraints per invocation vs ~25,000 for SHA-256.

see [[crypto/hash/features]] for the complete feature taxonomy, [[Hemera]] for the specific construction, [[hemera/spec]] for the full decision record

### [[encryption]]

transforming plaintext into ciphertext. [[cyber]] uses two schemes for different scenarios:

| scheme | type | assumption | use case |
|---|---|---|---|
| lattice KEM (Module-RLWE) | interactive | Module-RLWE hardness | private [[neuron]]-to-[[neuron]] data, encrypted [[cyberlink]] metadata |
| CSIDH / dCTIDH | non-interactive | isogeny class group action | stealth addresses, anonymous channels, [[key exchange medium]] |
| AES-256-GCM | symmetric | standard model | message payload after key agreement |
| [[TFHE]] | fully homomorphic | LWE hardness | compute on encrypted [[cybergraph]] data |

all operate natively over [[Goldilocks field]] arithmetic. see [[cyber/identity]] for the encryption layers, [[cyber/communication]] for onion-encrypted messaging, [[privacy trilateral]] for how ZK + FHE + MPC combine

### [[signature]]

classical cryptography binds a public key to a private key and produces a verifiable tag on each message. [[cyber]] eliminates signatures entirely — authentication is a [[STARK]] proof of [[Hemera]] preimage knowledge.

```
traditional:  secret_key → sign(message) → signature → verify(public_key, signature)
cyber:        secret → Hemera(secret) = address → STARK_proof(∃ x : Hemera(x) = address)
```

the only assumption: collision resistance of [[Hemera]]. no elliptic curves, no lattices, no pairings. see [[cyber/identity]] for the full specification, [[signer]] for the complexity of traditional multi-chain signing

### [[commitment scheme]]

bind to a value without revealing it, then reveal later with proof. [[cyber]] uses two forms:

- hash commitment: `H_commit(value ‖ randomness)` — the basis of [[record]] privacy in the [[mutator set]]
- [[polynomial commitment]]: FRI-based, enables membership proofs over committed sets ([[EdgeSet]])

see [[polynomial commitment]] for the FRI-based scheme, [[BBG]] for how commitments compose into the graph state

## proof systems

### [[zero knowledge proofs]]

prove a statement is true without revealing anything beyond the truth of the statement. [[cyber]] uses STARKs exclusively — no trusted setup, post-quantum secure, hash-only assumption.

see [[cyber/proofs]] for the complete proof taxonomy (25+ proof types), [[STARK]] for the proof system

### [[FRI]] → [[STIR]] → [[WHIR]]

the evolution of Reed-Solomon proximity testing — the core building block of STARKs:

```
FRI (2018)    →   STIR (2024)     →   WHIR (2024/2025)
baseline           fewer queries        richest queries
306 KiB proofs     160 KiB proofs       157 KiB proofs
3.9 ms verify      3.8 ms verify        1.0 ms verify
```

### recursive composition

the system verifies its own proofs: a STARK proof of a STARK verification is itself a valid STARK proof. this enables O(1) verification for O(N) computations — the foundation of scalability.

see [[cyber/proofs]] for recursive composition details

## data structures

cryptographic structures that authenticate the [[cybergraph]] state:

| structure | purpose | heritage |
|---|---|---|
| [[NMT]] | namespace completeness proofs | Celestia (2023) |
| [[MMR]] | append-only UTXO history | Grin, [[neptune]] (2019) |
| [[SWBF]] | private double-spend prevention | [[neptune]] (2024) |
| [[EdgeSet]] | edge membership via polynomial commitment | [[FRI]]/Plonky2 (2022) |
| [[LogUp]] | cross-index consistency | Polygon, Scroll (2023) |
| [[mutator set]] | combined AOCL + SWBF for UTXO privacy | [[neptune]] (2024) |

see [[BBG]] for how they compose, [[data structure for superintelligence]] for the full specification

## privacy

### the trilateral

three complementary technologies cover each other's blind spots:

| technology | proves | hides | limitation |
|---|---|---|---|
| ZK ([[zero knowledge proofs]]) | computation correct | inputs and process | prover sees everything |
| FHE ([[TFHE]]) | nothing (computation only) | data from computer | heavy, no integrity proof |
| MPC (multi-party computation) | threshold agreement | individual shares | requires honest majority |

ZK proves without showing. FHE computes without seeing. MPC distributes without trusting. combined: trustless computation on private data with provable results.

see [[privacy trilateral]] for the full analysis, [[BBG]] for the privacy boundary, [[cyber/identity]] for anonymous [[cyberlinks]]

### key agreement

two [[neurons]] derive a shared secret without communication:

| protocol | interaction | assumption | use in cyber |
|---|---|---|---|
| CSIDH / dCTIDH | non-interactive | isogeny class group | stealth addresses, graph-based key exchange |
| lattice KEM (Module-RLWE) | interactive | Module-RLWE | encrypted particle delivery |

the [[cybergraph]] itself is the key exchange medium — public curves published as [[particles]], shared secrets derived by reading the graph. see [[cyber/communication]]

## storage and availability

six proof types guarantee content survival at planetary scale:

| proof | guarantees |
|---|---|
| storage proof | content bytes exist on specific node |
| size proof | claimed content size matches actual byte count |
| replication proof | k independent copies exist |
| retrievability proof | content fetchable within bounded time |
| data availability (DAS) | block data was published, is accessible |
| encoding fraud proof | erasure coding was done correctly |

see [[storage proofs]] for the full specification

## the one-hash principle

[[cyber]] uses [[Hemera]] for everything: content addressing, authentication, commitment, Merkle trees, polynomial commitments, Fiat-Shamir challenges, proof transcripts. one hash = one security analysis, one implementation to audit, one hardware target.

```
H_edge(x)        = Hemera(0x01 ‖ x)    edge hashing
H_commit(x)      = Hemera(0x02 ‖ x)    record commitments
H_nullifier(x)   = Hemera(0x03 ‖ x)    SWBF index derivation
H_merkle(x)      = Hemera(0x04 ‖ x)    NMT and MMR nodes
H_fiat_shamir(x) = Hemera(0x05 ‖ x)    FRI/STIR challenges
H_transcript(x)  = Hemera(0x06 ‖ x)    proof transcript binding
```

domain separation at the input, not the output. one function, six roles.

## quantum resistance

| layer | primitive | quantum status |
|---|---|---|
| authentication | STARK proof of Hemera preimage | post-quantum (hash-only) |
| anonymity | ZK set membership + nullifiers | post-quantum (hash-only) |
| encryption (interactive) | lattice KEM (Module-RLWE) | post-quantum |
| encryption (non-interactive) | CSIDH / dCTIDH | conjectured post-quantum |
| computation privacy | [[TFHE]] over Goldilocks | post-quantum (LWE) |
| consensus | [[proof of stake]] + STARK execution | post-quantum |

authentication and anonymity require only hashes — post-quantum from genesis. encryption introduces additional assumptions (lattice, isogeny) — each chosen to operate natively over [[Goldilocks field]].

see [[cyber/identity]] for the six privacy layers, [[cyber/security]] for formal guarantees
