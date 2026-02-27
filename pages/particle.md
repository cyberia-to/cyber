---
icon: ⭕️
alias: particles, object, cid, content address, content
tags: cyber, cyb, page, core
crystal-type: entity
crystal-domain: cyber
crystal-size: article
---
content-addressed node in the [[cybergraph]]. identity = [[hash]] of content. immutable. exists or doesn't

raw bytes are [[data]]. hashing data produces a particle — a deterministic identity that collapses "what content?" into a fixed answer. [[data]] has no identity in the graph; a particle does. the [[hash]] is the proof of measurement: it certifies that data was observed and collapsed into an identity. anyone can verify the proof by re-hashing, but holding the hash alone does not grant access to the data

a particle cannot be submitted to the protocol without at least one [[cyberlink]]. a naked hash with no links is data that never entered the graph. in practice, every particle arrives with a [[name]] — a `~` [[cyberlink]] that makes it a [[file]], addressable by a human-readable label. the protocol requires this because an unnamed, unlinked particle carries no meaning

the chain: data → [[information]] → [[file]] → [[knowledge]] → [[intelligence]]

- data: raw bytes
- [[information]]: data identified by [[hash]] — a particle exists
- [[file]]: a particle given a `~` [[name]] — addressable, retrievable
- [[knowledge]]: [[particles]] linked by [[neurons]] via [[cyberlinks]]
- [[intelligence]]: the observation loop between [[neurons]] and the [[tru]]

examples: keyword, article, image, video, pdf, html app

[[particles]] are the objects of the [[cybergraph]], [[neurons]] are the subjects

each particle has a [[cyberank]]: probability of observation by random-walking [[neuron]]

[[cards]] can be bound to particles for provenance and ownership

see [[particle/tools]] for content addressing tools and CID format

discover all [[concepts]]
