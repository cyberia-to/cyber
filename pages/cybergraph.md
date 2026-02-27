---
icon: 🕸
tags: cyber, core
alias: content oracle, cybergraphs
crystal-type: entity
crystal-domain: cyber
---
the data model of [[cyber]]

five primitives define everything that exists in the system:

- [[particle]] — content-addressed node. identity = [[hash]] of content. immutable
- [[neuron]] — agent with stake and identity. creates links
- [[cyberlink]] — signed, weighted, timestamped directed edge between two [[particles]]
- [[token]] — unit of [[value]]: [[coins]], [[cards]], [[scores]], [[badges]]
- [[focus]] — attention distribution over the graph. conserved: Σ = 1. [[threshold]] filters spam

[[cyber/bbg]] defines how to store. [[cyber/core]] defines how to compute. cybergraph defines what to compute and store: the graph of meaning created by [[neurons]]

## knowledge

cybergraph implements [[knowledge theory]]: [[neurons]] [[link]] [[particles]] in [[time]]

a [[particle]] cannot enter the cybergraph without at least one [[cyberlink]]. the first link — a `~` [[name]] — turns the particle into a [[file]]. further links between particles create [[knowledge]]

every [[cyberlink]] records [[three basic arguments]]: [[who]] linked, [[when]], and [[what]] [[particles]]

the cybergraph is the shared memory that sits between [[neurons]] and the [[truth machine]] in the observation loop:

```
neuron ──cyberlink──→ cybergraph ──tri-kernel──→ cyberank
  ↑                                                  │
  └──────────── observes, infers, links ←────────────┘
```

[[neurons]] write [[implicit knowledge]] into the cybergraph as [[cyberlinks]]. the [[truth machine]] reads the cybergraph and computes [[explicit knowledge]]. the cybergraph is where both directions meet

## structure

the graph is namespace-indexed from genesis. every edge belongs to [[namespaces]], enabling completeness proofs via [[cyber/bbg]]

[[neural language]] structures the graph with [[semantic conventions]], [[motifs]], and [[sentences]] — turning raw links into typed meaning

the [[truth machine]] continuously computes [[cyberank]], [[karma]], and [[syntropy]] over the graph in [[consensus]]

## capabilities

- onchain [[dht]] with [[semantic core]] extending beyond words
- probabilistic content storage and retrieval, charged per file
- social information propagation and p2p retrieval
- private [[offline inference]] over fine-tuned [[llms]]

## architecture

together with [[truth machine]] and [[neural language]], cybergraph forms the foundation of [[soft3]]

cybergraph of [[bostrom]] serves as [[semantic core]] for the [[bootloader]] of [[superintelligence]]

implemented as [[bostrom/graph]] [[module]] of [[cyber-sdk]]
