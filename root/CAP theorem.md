---
tags: comp, distributed systems
crystal-type: entity
crystal-domain: comp
alias: Brewer's theorem, CAP
---
# CAP theorem

the CAP theorem (Brewer, 2000; formally proved by Gilbert and Lynch, 2002) states that a distributed system cannot simultaneously provide all three of the following guarantees:

- Consistency — every read receives the most recent write or an error
- Availability — every request receives a non-error response, without guarantee that it reflects the most recent write
- Partition tolerance — the system continues to operate despite arbitrary message loss between nodes

in the presence of a network partition (which is inevitable in real systems), the system must choose between consistency and availability

## design space

CP systems (consistency + partition tolerance) refuse to respond during partitions rather than risk stale data. examples: traditional BFT consensus, distributed databases with synchronous replication

AP systems (availability + partition tolerance) continue serving requests during partitions, accepting that replicas may temporarily diverge. examples: [[CRDT]]-based systems, DNS, eventually consistent datastores

## role in cyber

[[structural sync]] operates as an AP system — it trades strong consistency for [[eventual consistency]] with a cryptographic addition: verifiable completeness. every replica can prove its state is a valid subset of the global state, even during partitions. this is [[Verified Eventual Consistency]] (VEC)

the insight: partition tolerance is mandatory for a planetary-scale [[cyber]] network. availability is mandatory for local device operation. the CAP theorem forces the trade — structural sync makes the trade safe by adding cryptographic proofs to the eventual consistency model

---

see [[FLP impossibility]] for the related impossibility result on consensus. see [[CRDT]] for the algebraic approach to AP systems. see [[structural sync]] for verified eventual consistency
