---
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
alias: Merkle Mountain Range, Merkle mountain ranges, MMRs
---
# MMR

Merkle Mountain Range. an append-only authenticated data structure consisting of a forest of perfect binary Merkle trees. the heights of the trees correspond to the binary representation of the total leaf count. introduced by Peter Todd (2012). used in Grin, [[neptune]], and [[cyber]].

## structure

an MMR with N leaves has at most ⌈log₂(N)⌉ trees (peaks). new leaves are appended by creating a height-0 tree and merging adjacent equal-height trees:

```
MMR after 11 insertions (11 = 1011₂ → peaks at heights 3, 1, 0):

Peak 0 (height 3):           Peak 1 (height 1):   Peak 2 (height 0):
        h₇                         h₁₀                   h₁₁
       /   \                       /   \                    │
     h₃     h₆                  h₈    h₉                 leaf₁₁
    / \    / \                   │      │
  h₁  h₂ h₄  h₅               leaf₈  leaf₉
  │   │   │   │
 l₀  l₁  l₂  l₃  l₄ l₅ l₆ l₇

accumulator = H(peak₀ ‖ peak₁ ‖ peak₂) — O(log N) hash digests
```

## properties

| property | value |
|---|---|
| accumulator size | O(log N) peak hashes |
| append cost | O(1) amortized (merge adjacent equal-height peaks) |
| membership proof | Merkle path from leaf to peak, O(log N) hashes |
| modification | never — append-only by construction |
| deletion | not supported (append-only) |

## why MMR over Merkle tree

a standard Merkle tree requires a fixed capacity or rebalancing on growth. an MMR grows naturally by appending — no rebalancing, no pre-allocated depth. this makes it the right structure for any monotonically growing list:

- UTXO commitments ([[AOCL]] in the [[mutator set]])
- inactive [[SWBF]] chunks
- block header chains
- event logs

## use in cyber

the [[AOCL]] is an MMR storing addition records — one per UTXO ever created. the [[SWBF]] uses a separate MMR to compact inactive bloom filter chunks. both structures benefit from the same properties: O(log N) accumulator, O(1) append, O(log N) membership proof.

```
AOCL:  each leaf = H_commit(record ‖ ρ)           → tracks UTXO creation
SWBF:  each leaf = hash of compacted bloom chunk   → tracks historical spends
```

the MMR peaks feed into the [[BBG]] root hash, anchoring both structures in the global state commitment.

## membership proof

```
to prove leaf l is at position i in the MMR:

  1. provide the sibling hash at each level from leaf to peak
  2. verifier recomputes internal hashes up to the peak
  3. check peak matches the committed accumulator

cost: O(log N) hash computations
stark constraints: ~23,552 for N = 2³² (32 levels × 736 per Hemera hash)
```

see [[AOCL]] for UTXO commitment tracking, [[SWBF]] for bloom filter compaction, [[mutator set]] for the combined privacy primitive, [[BBG]] for the full graph state architecture