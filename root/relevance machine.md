---
tags: cyber, core
crystal-type: entity
crystal-domain: cyber
---

a system that computes what matters from a graph of assertions

## definition

the [[tru]] is a relevance machine: it takes the full [[cybergraph]] as input and outputs the [[focus]] distribution $\phi^*$ — a [[probability]] measure over all [[particles]] reflecting collective attention

$$\text{tru}: \mathbb{G} \;\longrightarrow\; \phi^* \in \Delta(P)$$

where $\mathbb{G}$ is the [[cybergraph]] and $\Delta(P)$ is the probability simplex over the particle set $P$

## computation

the relevance machine applies a diffusion process over the [[cybergraph]] topology. [[cyberlinks]] weighted by stake $a$ and valence $v$ define transition probabilities. the stationary distribution of this process is $\phi^*$ — the canonical ranking of all content in the system

## properties

the machine is deterministic: given the same [[cybergraph]] state, it always produces the same [[focus]] distribution. it is also permissionless — any [[neuron]] can shift $\phi^*$ by adding [[cyberlinks]], and the cost of doing so is governed by the [[costly signal]] mechanism

the relevance machine replaces centralized ranking algorithms with a cryptoeconomic consensus on what deserves attention

see [[tru]], [[cybergraph]], [[focus]], [[particle]], [[cyberlink]], [[diffusion]]

discover all [[concepts]]
