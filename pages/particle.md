---
icon: ⭕️
alias: particles, object, cid, content address, content
tags: cyber, cyb, page
crystal-type: entity
crystal-domain: cyber
---
content-addressed node in the [[cybergraph]]. identity = [[hash]] of content. immutable. exists or doesn't

raw bytes are [[data]]. hashing data produces a particle — a deterministic identity that collapses "what content?" into a fixed answer. [[data]] has no identity in the graph; a particle does. the [[hash]] is the proof of measurement: it certifies that data was observed and collapsed into an identity. anyone can verify the proof by re-hashing, but holding the hash alone does not grant access to the data

a particle cannot be submitted to the protocol without at least one [[cyberlink]]. a naked hash with no links is data that never entered the graph. in practice, every particle arrives with a [[name]] — a `~` [[cyberlink]] that makes it a [[file]], addressable by a human-readable label. the protocol requires this because an unnamed, unlinked particle carries no meaning

the chain: data → [[information]] → [[file]] → [[knowledge]] → [[intelligence]]

- data: raw bytes
- [[information]]: data identified by [[hash]] — a particle exists
- [[file]]: a particle given a `~` [[name]] — addressable, retrievable
- [[knowledge]]: [[particles]] linked by [[neurons]] via [[cyberlinks]]
- [[intelligence]]: the observation loop between [[neurons]] and the [[truth machine]]

CID format: (version, algorithm, parameters, field, digest)

instead of using the location on a server:
```
https://bitcoin.org/bitcoin.pdf
```

we use the object itself:
```
QmRA3NWM82ZGynMbYzAgYTSXCVM14Wx1RZ8fKP42G6gjgj
```

properties of content addressing

- mesh-network future-proof
- interplanetary accessibility
- censorship resistance
- technological independence
- deduplication

examples

- keyword
- article
- image
- video
- pdf
- html app

[[particles]] are the objects of the [[cybergraph]], [[neurons]] are the subjects

each particle has a [[cyberank]]: probability of observation by random-walking [[neuron]]

[[uniqs]] can be bound to particles for provenance and ownership

[[bostrom]] uses [[cidv0]] standard of content addressing — SHA-256 [[hash]] with rich software and hardware infrastructure

to compute particle from data: [[cyb/oracle]], [[cy]], or any [[ipfs]] tool

discover all [[concepts]]
