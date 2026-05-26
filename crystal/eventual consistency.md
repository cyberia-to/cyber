---
tags: comp, distributed systems
crystal-type: entity
crystal-domain: comp
---
# eventual consistency

eventual consistency is a consistency model for distributed systems: if no new updates are made to a data item, all replicas will eventually converge to the same value. the convergence time is unbounded but finite

this is weaker than strong consistency (linearizability), where every read reflects the latest write globally. eventual consistency permits temporary divergence between replicas — reads at different replicas may return different values until convergence completes

## strong eventual consistency

[[CRDT]] provides strong eventual consistency (SEC): replicas that have received the same set of updates are guaranteed to be in the same state, regardless of the order in which updates were received. SEC removes the need for conflict resolution — the merge function handles it algebraically through the [[join-semilattice]] structure

the difference: plain eventual consistency only promises convergence "eventually" and may require application-level conflict resolution. SEC guarantees automatic, deterministic convergence as a property of the data structure itself

## verified eventual consistency

[[structural sync]] provides Verified Eventual Consistency (VEC): SEC plus cryptographic proof that the local state is complete with respect to the declared sync frontier. a replica can prove to any verifier that it has incorporated all updates up to a given point and that its state is a valid merge result

VEC closes the trust gap in eventual consistency — a replica claims convergence, and the proof makes the claim verifiable

## role in cyber

[[cyber]] operates across heterogeneous devices and unreliable networks. [[eventual consistency]] is the natural model for this environment. the progression from EC to SEC ([[CRDT]]) to VEC ([[structural sync]]) adds successive layers of guarantee without sacrificing availability or partition tolerance, respecting the [[CAP theorem]] trade-off

---

see [[CAP theorem]] for the impossibility result that motivates eventual consistency. see [[CRDT]] for the algebraic path to SEC. see [[structural sync]] for VEC
