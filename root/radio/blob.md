---
alias: blobs, iroh-blobs, radio blob
tags: cyber
crystal-type: entity
crystal-domain: cyber
diffusion: 0.0002874415858194279
springs: 0.0012741087586357093
heat: 0.0009551962440300113
focus: 0.0007169926693064325
gravity: 9
density: 6.76
---

the fundamental data unit of [[radio]]: content-addressed binary data of any size, from bytes to terabytes

identified by a 64-byte [[Hemera]] hash — the hash IS the address. same content always produces same hash, deduplication by default

## verified streaming

supports verified streaming via [[radio/bao]]: download any byte range and verify it against the root hash without downloading the whole blob

range requests let you specify chunks or byte ranges to download partial content with cryptographic proof of correctness

interrupted transfers resume from the last verified chunk

both provider and requester verify data integrity — dual validation at every step

## storage

pluggable store interface — in-memory (MemStore) or filesystem (FsStore with redb)

garbage collection cleans up unused blobs. temp tags protect blobs during active downloads

## formats

BlobFormat has two variants: Raw for direct data, and HashSeq for a sequence of hash pointers — see [[radio/hash-seq]]

## role in cyber

every [[particle]] in the [[cybergraph]] is a blob. the particle's address is its [[Hemera]] hash. [[radio/blob]] is how [[particles]] move between [[neurons]] across the physical network

crate: iroh-blobs