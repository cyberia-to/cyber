---
tags: cyber, core
alias: linkchain
crystal-type: entity
crystal-domain: cyber
---

ordered sequences of [[cyberlinks]] forming directed paths through the [[cybergraph]]. chains of semantic connections that emerge from linking patterns

## structure

a linkchain is an ordered tuple of [[cyberlinks]]:

$$\mathcal{C} = (\ell_1, \ell_2, \ldots, \ell_n) \quad \text{where } q_i = p_{i+1}$$

the target [[particle]] of each link becomes the source particle of the next, producing a directed walk through the graph

## properties

- length $n$: the number of [[cyberlinks]] in the chain
- each link carries its own [[neuron]] signature, stake, and timestamp — a chain may involve one or many authors
- chains compose: concatenating two chains $\mathcal{C}_1 \circ \mathcal{C}_2$ yields a longer chain when the final particle of the first matches the initial particle of the second
- cycles emerge when $q_n = p_1$, forming closed loops in the [[cybergraph]]

## semantics

linkchains capture multi-hop reasoning. a single [[cyberlink]] states one relation; a chain traces an argument, a derivation, or a narrative across the graph. the [[tri-kernel]] propagates [[attention]] along these paths, amplifying chains with consistent stake backing

## discovery

the [[tru]] indexes linkchains during [[consensus]], enabling queries like "all directed paths of length $k$ from [[particle]] $p$." this makes the graph traversable as a reasoning substrate

see [[cyberlinks]], [[cybergraph]], [[particle]], [[neuron]], [[tri-kernel]]

discover all [[concepts]]
