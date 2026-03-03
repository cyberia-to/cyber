---
alias: iroh-docs, replica, document sync, radio docs
tags: cyber
crystal-type: entity
crystal-domain: cyber
---

# docs

eventually-consistent multi-dimensional key-value documents

## replica

a document instance identified by a NamespaceId (public key). contains unlimited entries. the namespace private key grants write authority over the entire document

## entries and authors

an entry is identified by the tuple (namespace, author, key). its value is a [[Hemera]] hash pointing to content stored as a [[radio/blob]], plus size and timestamp. an author is an Ed25519 signing key proving authorship — AuthorId is the corresponding public key

## synchronization

range-based set reconciliation — recursive partitioning with fingerprint comparison. only changed entries sync, not the whole document. efficient even for large replicas with millions of entries

## meta-protocol

depends on [[radio/blob]] for content storage and [[radio/gossip]] for change notification. docs combines both into a live-syncing document layer. download policies give fine-grained control over which content to fetch: complete, incomplete, or missing

## events

LocalInsert fires when an entry is added locally. RemoteInsert fires when an entry arrives from a peer, carrying content availability status so the application knows whether the blob is already downloaded

## role in cyber

docs enables collaborative knowledge construction. multiple [[neurons]] write to a shared replica — each authoring entries under their own key. the replica syncs automatically via [[radio/gossip]], content transfers via [[radio/blob]]. this is the substrate for shared [[cybergraph]] partitions

crate: iroh-docs
