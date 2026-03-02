---
tags: cyber, cyb
alias: iroh
crystal-type: entity
crystal-domain: cyber
---
# radio

connectivity for [[superintelligence]]. a fork of [[iroh]] where every hash — content identifiers, verified streaming trees, relay handshakes — runs through [[Hemera]] ([[Poseidon2]] over the [[Goldilocks field]]) instead of BLAKE3

[github](https://github.com/cyberia-to/radio)

## why

BLAKE3 hashes at 2 GB/s. [[Hemera]] reaches ~50–100 MB/s on CPU. the tradeoff: proving a single BLAKE3 hash inside a STARK costs 50,000–100,000 constraints. [[Hemera]] costs ~300. this enables:

- [[storage proofs]] without downloading content
- verified streaming with [[Hemera]] Merkle trees
- private computation over encrypted [[knowledge graph]]
- post-quantum security via STARKs

## architecture

radio preserves [[iroh]] networking (QUIC, hole-punching, relay servers) and replaces the cryptographic foundation across four strata:

| stratum | layer | crate |
|---|---|---|
| protocols | blob transfer, docs, gossip, willow | iroh-blobs, iroh-docs, iroh-gossip, iroh-willow |
| verified streaming | [[Hemera]] Merkle tree operations | cyber-bao |
| content identity | sponge, compression, KDF in [[Goldilocks field]] | cyber-poseidon2 |
| networking | QUIC transport, relay, hole-punching | iroh, iroh-relay |

## crates

- cyber-poseidon2 — [[Hemera]] hash implementation (CPU + GPU scaffolding)
- cyber-bao — verified streaming protocol (Hemera Merkle trees)
- cyber-hash — CLI hashing tool
- iroh-blobs — content-addressed blob transfer
- iroh-relay — relay servers with Hemera handshakes
- iroh-docs — document synchronization
- iroh-gossip — gossip protocol
- iroh-willow — [[willow]] protocol implementation

## status

zero BLAKE3 dependencies remain. 395 tests pass across all crates

## in the stack

radio is the data transport layer of [[cyb]]. where [[ipfs]] uses CIDv1 with multicodec headers, radio uses raw 64-byte [[Hemera]] outputs as [[particle]] addresses. one hash function, one address space, zero self-describing overhead

see [[Hemera]] for the hash primitive, [[hemera/spec]] for the full decision record
