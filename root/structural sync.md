---
tags: cyber, core
crystal-type: pattern
alias: structural synchronization, sync layers
---

# structural sync

The five-layer verification framework for the [[cyber]] protocol ensuring verified [[eventual consistency]].

## layers

1. [[zheng]] validity — confirms each signal conforms to protocol grammar
2. [[hash chain]] + [[VDF]] ordering — establishes causal and temporal sequence per source
3. [[NMT]] completeness — guarantees every signal in a block is included in the namespace Merkle tree
4. [[DAS]] availability — verifies that block data can be reconstructed from erasure-coded samples
5. [[patch sync]] — propagates verified state deltas across nodes for convergence

Each layer builds on the one below, so a node can choose its trust depth depending on resource constraints.

discover all [[concepts]]
