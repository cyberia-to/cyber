---
alias: HashSeq, collection, hash sequence
tags: cyber
crystal-type: entity
crystal-domain: cyber
diffusion: 0.00014237595671140588
springs: 0.0016424286226267763
heat: 0.0011733365862070652
focus: 0.0007985838823851387
gravity: 2
density: 5.98
---

a [[radio/blob]] whose content is a sequence of 64-byte [[Hemera]] hashes — pointers to other [[blobs]]

the content must be a multiple of 64 bytes, each entry is one hash

## collections

named groupings built on hash sequences. a collection is a metadata blob (names + hashes) plus the child blobs it references

## use cases

- group related [[particles]]: a directory of files, a set of images, a [[knowledge]] graph snapshot
- recursive structure: a hash-seq can point to other hash-seqs, building DAGs (directed acyclic graphs)
- bulk transfer: request a hash-seq and [[radio]] automatically fetches all referenced blobs

## partial downloads

ChunkRangesSeq lets you specify which children to download and which byte ranges within each child when requesting a hash-seq

infinite repeating ranges: request "all chunks" and it applies to every child in the sequence

## role in cyber

a [[neuron]]'s published [[knowledge]] — a set of [[particles]] linked by [[cyberlinks]] — can be serialized as a hash-seq

one hash addresses the entire collection. sync a neuron's full knowledge set by fetching one hash-seq