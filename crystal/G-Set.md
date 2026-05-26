---
tags: comp, distributed systems
crystal-type: entity
crystal-domain: comp
alias: G-Sets, grow-only set, grow-only sets
---
# G-Set

a G-Set (grow-only set) is the simplest [[CRDT]]. elements can be added and never removed. the merge function is set union

## definition

state: a set S

update: add(e) → S' = S ∪ {e}

merge: merge(S₁, S₂) = S₁ ∪ S₂

query: lookup(e) = e ∈ S

## algebraic structure

the G-Set forms a [[join-semilattice]] under set inclusion (⊆). the join is set union (∪). union is commutative, associative, and idempotent — the three properties that guarantee convergence regardless of update order

## role in cyber

[[BBG]] uses G-Sets for content sync. each [[particle]] is identified by a CID (content identifier). when a node encounters a new CID, it adds it to the local G-Set. merge with any other node is union — CIDs that appear on either side appear in the result. deduplication is automatic because set union ignores duplicates

the grow-only constraint is a natural fit for content-addressed data: a CID, once created, is immutable and permanent. there is no meaningful "remove" for content that is addressed by its own hash

---

see [[CRDT]] for the family of convergent data structures. see [[particle]] for the content objects identified by CID
