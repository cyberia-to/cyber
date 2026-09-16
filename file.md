---
tags: cyber, core
alias: files
icon: "📄"
crystal-type: entity
crystal-domain: cyber
crystal-size: atom
---
the thing a [[particle]] identifies. everything is a file — a document, an image, a model, a genome, a [[neuron]]'s key, an [[axon]]'s pair of endpoints. the [[cybergraph]] is files, linked by [[cyberlinks]] that [[neurons]] sign and [[tokens]] weigh

```
file = (particle, data, name, meta)
```

| component | role |
|-----------|------|
| [[particle]] | the [[hemera]] hash of the data — the file's identity, the 32 bytes a [[cyberlink]] holds |
| data | the bytes themselves — content, a key, a pair |
| [[name]] | the `~` label a neuron gives it — the moment a file becomes findable |
| meta | what the [[dialect]] declares: author neuron, [[height]], mime, related links |

the particle is intrinsic: two files with the same data share one particle. name and meta are the [[neuron]]-facing wrapper around it. the graph links by particle; a neuron reads, makes, and pays for files

a file's data may be absent on this machine — not yet fetched, or never published. the file still exists, its particle still links, [[focus]] still flows through it. [[cyb]] shows such a file as a black hole: mass without visible body, `cyb://particle/<hex>` until a [[cyb/reference/spark|spark]] can draw the bytes and it becomes `cyb://file/<hex>`

files are what flows through [[tape]] frames when an agent sends a thing rather than a raw atom. [[particles]] are the objects' identities; [[neurons]] are the subjects. a file's name is neither: it is a `~` label a neuron gives by a naming [[cyberlink]], mutable, resolved by supersession

discover all [[concepts]]
