---
tags: cyber, cyb
alias: iroh
crystal-type: entity
crystal-domain: cyber
---
# radio

connectivity for [[superintelligence]]. a fork of [[iroh]] where every hash — content identifiers, verified streaming trees, relay handshakes — runs through [[Hemera]] instead of [[Blake3]]

[github](https://github.com/cyberia-to/radio)

## why

Blake3 hashes at 2 GB/s. [[Hemera]] reaches ~50–100 MB/s on CPU. the tradeoff: proving a single Blake3 hash inside a stark costs 50,000–100,000 constraints. [[Hemera]] costs ~736. this enables:

- [[storage proofs]] without downloading content
- verified streaming with [[Hemera]] Merkle trees
- private computation over encrypted [[knowledge graph]]
- post-quantum security via starks

## architecture

radio preserves [[iroh]] networking (QUIC, [[radio/hole-punching]], [[radio/relay]]) and replaces the cryptographic foundation across four strata:

| stratum | layer | crate |
|---|---|---|
| protocols | [[radio/blob]], [[radio/docs]], [[radio/gossip]], [[radio/willow]] | iroh-blobs, iroh-docs, iroh-gossip, iroh-willow |
| verified streaming | [[radio/bao]] ([[Hemera]] Merkle trees) | cyber-bao |
| content identity | sponge, compression, KDF in [[Goldilocks field]] | cyber-poseidon2 |
| networking | [[radio/endpoint]], [[radio/relay]], [[radio/hole-punching]] | iroh, iroh-relay |

## crates

- cyber-poseidon2 — [[Hemera]] hash implementation (CPU + GPU scaffolding)
- cyber-bao — [[radio/bao]] protocol (Hemera Merkle trees)
- cyber-hash — CLI hashing tool
- iroh-blobs — [[radio/blob]] transfer
- iroh-relay — [[radio/relay]] servers with Hemera handshakes
- iroh-docs — [[radio/docs]] synchronization
- iroh-gossip — [[radio/gossip]] protocol
- iroh-willow — [[radio/willow]] protocol implementation

## status

zero Blake3 dependencies remain. 395 tests pass across all crates

## in the stack

radio is the data transport layer of [[cyb]]. where [[ipfs]] uses CIDv1 with multicodec headers, radio uses raw 64-byte [[Hemera]] outputs as [[particle]] addresses. one hash function, one address space, zero self-describing overhead

connections route through the [[radio/router]] (ALPN multiplexer). content is shared via [[radio/ticket]]. [[radio/endpoint]] [[radio/discovery]] resolves public keys to addresses

## migration status

hemera (Poseidon2) migration is complete: zero blake3 dependencies remain, 395 tests pass. content addressing, BAO trees, gossip message IDs all use hemera.

remaining work: replace Ed25519 with [[stark]] proofs for peer authentication. ~800 lines of direct crypto across three areas:

- identity types (iroh-base/src/key.rs) — `NodeId = PublicKey` wraps curve25519-dalek. replace with `NodeId = Poseidon2(secret)`, sign → STARK proof, verify → STARK verify
- TLS handshake (iroh/src/tls/) — rustls expects classical signatures. options: (a) fork rustls for STARK verification, (b) replace TLS with custom Noise-like protocol over raw QUIC, (c) keep TLS as dumb encryption pipe, authenticate with STARK proofs at application layer after channel is established. option (c) is least invasive
- relay handshake (iroh-relay/src/protos/handshake.rs) — replace Ed25519 challenge-response with STARK proof of identity

gossip, blobs, bao, docs — already clean, no signatures involved.

the key exchange (QUIC/TLS) can use [[mudra]] primitives (kem/ctidh) once the handshake is redesigned. encrypted channels after handshake use aead.

see [[Hemera]] for the hash primitive, [[hemera/spec]] for the full decision record