---
tags: cyber, core
alias: files
icon: "📄"
crystal-type: entity
crystal-domain: cyber
crystal-size: atom
---
the thing a [[particle]] identifies: data, and its identity. everything is a file — a document, an image, a model, a genome, a [[neuron]]'s key, an [[axon]]'s pair of endpoints. the [[cybergraph]] is files, linked by [[cyberlinks]] that [[neurons]] sign and [[tokens]] weigh

```
file = (particle, data)        particle = hemera(data)
```

the particle is the file's identity — 32 bytes, computed from the data, immutable; what a [[cyberlink]] holds. the data is the bytes — content, a key, a pair. nothing else is inside a file. two files with the same data are one file

everything else said about a file lives in the graph, as cyberlinks that point at it:

| about a file | how it is said |
|---|---|
| its [[name]] | a `~` label a neuron gives by a naming cyberlink — mutable, resolved by supersession: the latest link under a path wins |
| author, [[height]], mime, whatever a [[dialect]] declares | cyberlinks, and the [[signal]] that carried them |
| what it relates to | cyberlinks, weighted into [[focus]] |

a file has exactly one identity and any number of names, or none

a file's data may be absent on this machine — not yet fetched, or never published. the file still exists, its particle still links, focus still flows through it. [[cyb]] shows such a file as a black hole: `cyb://particle/<hex>` until a [[cyb/reference/spark|spark]] can draw the bytes and it becomes `cyb://file/<hex>`

on the wire a file is a [[tade]] frame: `marker type size data` — the type byte says what kind of thing the bytes are, the data is the file's data. [[particles]] are the objects' identities; [[neurons]] are the subjects

discover all [[concepts]]
