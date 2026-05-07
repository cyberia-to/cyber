---
alias: cyber-bao, BAO, verified streaming
tags: cyber
crystal-type: entity
crystal-domain: cyber
---

cyber-bao: [[Hemera]] Merkle trees for verified streaming. a binary tree of hashes over fixed-size chunks that enables partial downloads with proof

## architecture

three layers

- tree: pure geometry — node indexing (in-order binary tree), chunk counting, block sizes
- hash: pluggable hash backend with Poseidon2Backend implementation
- io: encoding, decoding, outboard creation, slice extraction

## parameters

chunk size: 4 KiB (4096 bytes). pair size: 128 bytes (two 64-byte [[Hemera]] hashes concatenated)

## tree structure

TreeNode uses in-order binary tree indexing. leaves sit at even positions (0, 2, 4...), parents at odd positions. level equals trailing ones count

## operations

- encode: transform data into BAO-encoded format with root hash
- decode: decompress and verify against root hash
- outboard: generate standalone Merkle proof tree without payload
- verify: validate file against expected root hash

## why BAO matters

you can download byte range [1000..2000] of a 10 GB blob and verify it against the root hash. the proof is logarithmic in blob size — a few KB covers any range of any blob

## role in cyber

every [[particle]] transfer uses BAO verification. [[radio]] never trusts — it verifies every chunk cryptographically during streaming. this is what makes [[Hemera]] hashes self-certifying addresses

crate: cyber-bao