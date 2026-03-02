---
tags: cyber, core
crystal-type: pattern
crystal-domain: cyber
crystal-size: bridge
alias:: cyberpatch, patch system, patches
icon: "\U0001FA79"
---

content-addressed, identity-sovereign [[patch theory]] system for the [[cybergraph]]. treats changes as commutative morphisms instead of snapshots — independent patches apply in any order, [[conflicts]] are first-class data, merge is set union

every patch is a signed set of operations over [[particles]] and [[cyberlinks]], authored by a [[neuron]], weighted by [[focus]] contribution. patches earn rewards proportional to their impact on the [[knowledge]] graph (Δπ from the [[tri-kernel]])

three primitives map directly to [[cyber]] protocol:

- patch = [[cyberlink]] (signed, timestamped, weighted by Δπ)
- tracked content = [[particle]] (content-addressed node)
- channel = named view over the global patch DAG

[[conflicts]] between concurrent patches are algebraic objects with well-defined structure — they can be resolved by further patches, left in state, or arbitrated by [[consensus]]

designed for parallel [[neuron]] and agent workflows at planetary scale. post-quantum [[cryptography]] from genesis

see [[cyber/patch/spec]] for the full specification
