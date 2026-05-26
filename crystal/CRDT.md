---
tags: comp, distributed systems
crystal-type: entity
crystal-domain: comp
alias: CRDTs, conflict-free replicated data type, conflict-free replicated data types
---
# CRDT

a conflict-free replicated data type is a data structure that can be replicated across multiple nodes, updated independently and concurrently, and merged automatically into a consistent state. the merge function is defined by a [[join-semilattice]] — commutative, associative, and idempotent — so the order of updates and merges is irrelevant

CRDTs guarantee strong eventual consistency (SEC): any two replicas that have received the same set of updates — in any order — hold identical state

## key types

[[G-Set]] — grow-only set. merge = union. the simplest CRDT

LWW-Register — last-writer-wins register. each update carries a timestamp. merge keeps the value with the highest timestamp. simple but requires a monotonic clock source

OR-Set — observed-remove set. elements can be added and removed. each add is tagged with a unique identifier. remove only affects tags that the remover has observed. concurrent add and remove resolve in favor of add

## algebraic structure

every CRDT state space forms a [[join-semilattice]]. the merge operation is the least upper bound (join). this is what makes convergence automatic — applying the same set of updates in any order always yields the same join

## role in cyber

[[cyber]] uses CRDTs for local device sync. a [[neuron]] running on multiple devices merges its local [[cybergraph]] views through CRDT semantics before broadcasting to the network. [[foculus]] consensus operates above this layer — CRDTs handle the pre-consensus local state

[[structural sync]] extends CRDT guarantees with cryptographic verification: every merge is accompanied by a proof that the resulting state is complete and correct

---

see [[join-semilattice]] for the algebraic foundation. see [[G-Set]] for the simplest instance. see [[foculus]] for consensus above the CRDT layer. see [[structural sync]] for verified sync
