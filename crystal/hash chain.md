---
tags: cyber, core
crystal-type: entity
alias: hash chains
---

# hash chain

A per-source immutable [[signal]] history in [[cyber]]. Each signal references the cryptographic hash of its author's previous signal, forming a sequential chain unique to that source.

This structure enables equivocation detection in O(1): if two signals share the same sequence number but differ in content, the fork is immediately provable by any observer.

Hash chains serve as layer 2 of [[structural sync]], working alongside [[VDF]] ordering to establish both causal and temporal sequence across the network.

The chain anchors identity to history — a source's full record is tamper-evident and independently verifiable by any node holding the chain tip.

discover all [[concepts]]
