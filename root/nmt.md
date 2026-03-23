---
tags: cyber, cryptography
alias: namespaced merkle tree, NMTs, namespaced merkle trees
crystal-type: entity
crystal-domain: cyber
---

namespaced merkle tree. an authenticated data structure that indexes [[particles]] by namespace, doubling as a data availability layer

## structure

an NMT extends the standard merkle tree by ordering leaves under namespace identifiers:

$$\text{NMT} = \text{Merkle}\bigl(\{(ns_i, d_i)\}_{i=1}^{n}\bigr) \quad \text{sorted by } ns_i$$

each leaf carries a namespace tag $ns_i$ and data $d_i$. internal nodes store the minimum and maximum namespace of their subtrees, enabling efficient range proofs over a single namespace without downloading the rest

## properties

- completeness: a namespace proof covers all leaves in the requested namespace — the verifier knows nothing was withheld
- ordering: leaves are sorted by namespace, so all entries for a given namespace form a contiguous range
- each namespace corresponds to a content type or domain partition in the [[cybergraph]]

## role in cyber

the NMT provides the data availability guarantee for [[bbg]]. when a [[neuron]] syncs a namespace, the tree proves that every [[particle]] and [[cyberlink]] in that namespace was included. light clients verify availability by sampling random shares and checking namespace proofs against the root

## relation to consensus

during block production, the NMT root commits to the full ordered data of the block. validators reconstruct and verify the tree; any omission breaks the namespace range property and is detectable. this makes censorship of specific namespaces provably observable

see [[bbg]], [[particles]], [[cyberlinks]], [[cybergraph]], [[hemera]]
