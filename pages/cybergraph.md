---
icon: 🕸
tags: cyber, core
alias: content oracle, cybergraphs
crystal-type: observed
crystal-domain: cyber
crystal-size: article
---
the accumulated [[graph]] of all [[cyberlinks]] created by all [[neurons]]. the shared [[knowledge]] of [[cyber]]

five primitives define everything that exists in the system:

- [[particle]] — content-addressed node. identity = [[hash]] of content. immutable
- [[neuron]] — agent with stake and identity. creates links
- [[cyberlink]] — signed, weighted, timestamped directed edge between two [[particles]]
- [[token]] — unit of [[value]]: [[coins]], [[cards]], [[scores]], [[badges]]
- [[focus]] — attention distribution over the graph. conserved: Σ = 1. [[threshold]] filters spam

a [[particle]] cannot enter the cybergraph without at least one [[cyberlink]]. the first link — a `~` [[name]] — turns the particle into a [[file]]. further links between particles create [[knowledge]]

every [[cyberlink]] records [[three basic arguments]]: [[who]] linked, [[when]], and [[what]] [[particles]]

the cybergraph is the shared memory that sits between [[neurons]] and the [[tru]] in the observation loop:

```
neuron ──cyberlink──→ cybergraph ──tri-kernel──→ cyberank
  ↑                                                  │
  └──────────── observes, infers, links ←────────────┘
```

[[neurons]] write [[implicit knowledge]] into the cybergraph as [[cyberlinks]]. the [[tru]] reads the cybergraph and computes [[explicit knowledge]]. the cybergraph is where both directions meet

the [[tru]] continuously computes [[cyberank]], [[karma]], and [[syntropy]] over the graph in [[consensus]]

see [[cybergraph/architecture]] for capabilities, namespace structure, and implementation

discover all [[concepts]]
