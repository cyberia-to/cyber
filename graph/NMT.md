---
tags: cyber, computer science, cryptography
crystal-type: entity
crystal-domain: computer science
alias: namespaced Merkle tree, namespaced Merkle trees, NMTs
---
# NMT

namespaced Merkle tree. a binary Merkle tree over sorted leaves where each internal node carries the minimum and maximum namespace of its subtree. the defining property: structural completeness proofs — the tree physically cannot represent a valid root over misordered leaves.

invented for [[Celestia]] (2023). used in [[cyber]] as the primary index structure for the [[cybergraph]] ([[BBG]]).

## structure

```
internal node:
  NMT_node = (min_ns, max_ns, H_merkle(left_child ‖ right_child))

leaf node:
  NMT_leaf = (namespace, H_merkle(payload))

invariant:
  for every internal node N with children L, R:
    N.min_ns = L.min_ns
    N.max_ns = R.max_ns
    L.max_ns ≤ R.min_ns        ← sorting invariant
```

the sorting invariant is structural — enforced by construction. any valid NMT path that violates sorting produces an invalid Merkle root, detectable by any verifier with only the root hash.

## completeness proofs

the defining capability: prove "these are ALL items in namespace N."

```
COMPLETENESS PROOF for namespace N:

  1. path to leftmost leaf with namespace N
  2. path to rightmost leaf with namespace N
  3. left boundary: left neighbor has namespace < N (or is tree boundary)
  4. right boundary: right neighbor has namespace > N (or is tree boundary)

VERIFICATION (by any client with only the root):
  a) both Merkle paths verify against root           — O(log n) hashes
  b) leftmost leaf has namespace = N                  — 1 comparison
  c) rightmost leaf has namespace = N                 — 1 comparison
  d) left neighbor namespace < N                      — 1 comparison
  e) right neighbor namespace > N                     — 1 comparison

GUARANTEE:
  the client has provably received every leaf in namespace N
  no leaf can be hidden — the tree structure prevents it

ABSENCE PROOF for namespace N:
  show two adjacent leaves with namespaces that bracket N:
    leaf_i.namespace < N < leaf_{i+1}.namespace
  this proves no leaf with namespace N exists

COST:
  proof size: O(log n) hash digests
  verification: O(log n) hash computations
  for n = 2³²: ~32 × 250 = 8,000 stark constraints
```

## why NMTs cannot be replaced

sorted [[polynomial commitments]] can approximate completeness but lack structural guarantees:

- polynomial completeness requires a protocol: prove boundary entries, prove contiguity, prove sorting was maintained. each step requires a separate argument. any step can have bugs.
- NMT completeness is a structural invariant: the tree physically cannot represent a valid root over misordered leaves. there is no protocol to debug because there is no protocol — a tree.
- DAS requires NMTs: namespace-aware Data Availability Sampling (Celestia model) needs namespace labels propagated through internal nodes. polynomials have no internal nodes.
- sync requires NMTs: "give me everything for neuron N with proof nothing is hidden" is the [[cybergraph]]'s fundamental operation.

## use in cyber

the [[BBG]] maintains six NMT indexes over the same edge and [[token]] data:

| index | namespace | proves |
|---|---|---|
| by_neuron | neuron_id | all edges created by a [[neuron]] |
| by_particle | particle_hash | all edges touching a [[particle]] |
| focus | neuron_id | current [[focus]] value per [[neuron]] |
| balance | neuron_id | current balance per [[neuron]] |
| coins | denom_hash | fungible [[token]] supply records |
| cards | card_id | non-fungible knowledge assets |

within each NMT leaf, an [[EdgeSet]] ([[polynomial commitment]] via [[WHIR]]) stores the actual edge hashes. the NMT provides completeness ("all edges for this namespace"). the EdgeSet provides efficient membership ("this edge belongs to this set").

## data availability sampling

block data is arranged in a √n × √n grid, erasure-coded in both dimensions (Reed-Solomon over [[Goldilocks field]]), and committed via NMT per row. light clients sample random cells with namespace-aware proofs — O(√n) samples for 99.9% confidence that all data is available.

see [[BBG]] for the full graph architecture, [[EdgeSet]] for polynomial commitments within NMT leaves, [[data structure for superintelligence]] for the complete specification, [[Celestia]] for production heritage
