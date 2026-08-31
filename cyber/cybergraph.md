---
title: cybergraph contract
tags: cyber, core
alias: cybergraph contract, cybergraph axioms, formal cybergraph
crystal-type: pattern
crystal-domain: cyber
---
# the cybergraph contract

what [[cyber]] demands of its one data structure — the commitments every implementation must honor, stated at protocol altitude. the idea lives at [[cybergraph]]; the full mathematics at [[cybergraph spec]]; the reference machine at [[soft3/cybergraph]]; the query surface at [[inf/cybergraph]]. this page is the interface between them

## the object

a cybergraph is a triple $\mathbb{G} = (P, N, L)$: content-addressed [[particles]], authenticated [[neurons]], and a multiset of staked [[cyberlinks]] — with [[tokens]] and [[karma]] derived from $L$, never primitive. everything else the protocol does is a reading of this triple

## six commitments

the axioms are the protocol's promises. break one and the thing running is not a cybergraph:

| | commitment | what it buys |
|---|---|---|
| A1 | **content-addressing** — identity equals content, $H$ collision-resistant | no one can quietly edit the record; the same knowledge is the same particle everywhere |
| A2 | **authentication** — every [[signal]] carries its neuron's valid signature | every claim has an author; there is no anonymous write path to bypass staking |
| A3 | **append-only** — $L$ grows monotonically; only economic weight decays | history cannot be rewritten, only outweighed — [[forgetting]] is economic, not destructive |
| A4 | **entry** — a particle exists iff it is linked | no orphan data: the graph contains exactly what someone paid to connect |
| A5 | **conservation** — $\sum_p \phi^*_p = 1$ at every block | [[focus]] is a budget, not a score: attention gained here is attention lost elsewhere |
| A6 | **homoiconicity** — every edge hashes into a particle ([[axon]]) | the graph can rank, link, and reason about its own structure |

A5 is the bridge clause: the graph promises a conserved $\phi^*$ exists, but computing it is not the graph's job — that boundary is next

## sharp boundaries

the contract works because each organ owns exactly one thing and the seams are explicit:

| organ | owns | must not |
|---|---|---|
| cybergraph | the triple, adjacency, the [[signal]] lifecycle | compute focus, store bytes, decide anything |
| [[tru]] | the dynamics — [[tri-kernel]] to $\phi^*$, [[syntropy]], [[impulse]] | define the graph it reads |
| [[bbg]] | authenticated storage and commitments | interpret what it stores |
| [[foculus]] | finality — $\phi^*_i > \tau$ | rank or vote |
| [[zheng]] | proofs gating every commit | trust anyone |
| [[soma]] | deciding and executing — the smart half | write unproven results |

adjacency is where economics enters the mathematics: raw weight from stake, effective weight $A^{\text{eff}}$ after [[karma]] and [[market inhibition]] multiply it. the graph hands tru a matrix that already encodes who is trusted and what the market believes

## one structure, the whole protocol

identity, key exchange, consensus, fork choice, finality, privacy, incentives, version control, file system, type system, computation, data availability, sybil resistance — fifteen protocol functions run through these five primitives with no second data structure anywhere. the full table with per-function mechanisms is in [[cybergraph spec]] §the graph is the protocol

this is the deepest design bet in [[cyber]]: a protocol with one structure cannot disagree with itself. there is no state that is not graph, so there is nothing to reconcile

see the [[whitepaper]] §3 for the narrative form · [[cybergraph spec]] for axioms with full mathematics · [[soft3/cybergraph]] for the machine that enforces them · [[inf/cybergraph]] for reading the result

discover all [[concepts]]
