---
tags: comp, algebra
crystal-type: entity
crystal-domain: comp
alias: join-semilattices
---
# join-semilattice

a join-semilattice is a partially ordered set (S, ≤) where every pair of elements a, b has a least upper bound, called the join: a ∨ b. the join is the smallest element c such that a ≤ c and b ≤ c

## properties

the join operation is:

- commutative: a ∨ b = b ∨ a
- associative: (a ∨ b) ∨ c = a ∨ (b ∨ c)
- idempotent: a ∨ a = a

these three properties are equivalent to the partial order definition — any binary operation that is commutative, associative, and idempotent defines a join-semilattice, and vice versa

## algebraic foundation of CRDT

the join-semilattice is the algebraic foundation of [[CRDT]]. every CRDT state space forms a join-semilattice. the merge operation between two replicas is the join. because the join is commutative, associative, and idempotent, the merge result is independent of the order in which updates are applied — this is what guarantees convergence

examples:

- [[G-Set]]: the partial order is set inclusion (⊆). the join is set union (∪)
- LWW-Register: the partial order is by timestamp. the join selects the value with the higher timestamp
- counters: the partial order is component-wise ≤ on vectors. the join is component-wise max

## monotonicity

information in a join-semilattice only grows — every join produces an element that is ≥ both inputs. this monotonicity is essential for distributed systems: updates can arrive in any order, duplicates are harmless, and the state only moves forward

---

see [[CRDT]] for the data structures built on this foundation. see [[lattice]] for the complete structure where meets also exist
