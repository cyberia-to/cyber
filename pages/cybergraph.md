---
icon: 🕸
tags: cyber
alias: content oracle, cybergraphs
crystal-type: entity
crystal-domain: cyber
---
the data model of [[cyber]]

three primitives define everything that exists in the system:

- [[particle]] — content-addressed node. identity = [[hash]] of content. immutable
- [[neuron]] — agent with stake, identity, and [[focus]]. creates links
- [[cyberlink]] — signed, weighted, timestamped directed edge between two [[particles]]

cybergraph sits between [[CORE]] (the computation model) and [[cyber/bbg]] (the authenticated state). CORE defines how to compute. BBG defines how to prove. cybergraph defines what to compute and prove over: the graph of meaning created by [[neurons]]

## knowledge

cybergraph implements [[knowledge theory]]: [[neurons]] [[link]] [[particles]] in [[time]]

every [[cyberlink]] records [[three basic arguments]]: [[who]] linked, [[when]], and [[what]] [[particles]]

these arguments form [[explicit knowledge]] — directly stated, readily traversable

[[implicit knowledge]] emerges through inference over the accumulated graph: if A links to B and B links to C, then A relates to C

## structure

the graph is namespace-indexed from genesis. every edge belongs to [[namespaces]], enabling completeness proofs via [[cyber/bbg]]

[[neural language]] structures the graph with [[semantic conventions]], [[motifs]], and [[sentences]] — turning raw links into typed meaning

the [[relevance machine]] continuously computes [[focus]], [[karma]], and [[rank]] over the graph in [[consensus]]

## capabilities

- onchain [[dht]] with [[semantic core]] extending beyond words
- probabilistic content storage and retrieval, charged per file
- social information propagation and p2p retrieval
- private [[offline inference]] over fine-tuned [[llms]]

## architecture

together with [[relevance machine]] and [[neural language]], cybergraph forms the foundation of [[soft3]]

cybergraph of [[bostrom]] serves as [[semantic core]] for the [[bootloader]] of [[superintelligence]]

implemented as [[cyber/graph]] [[module]] of [[cyber-sdk]]