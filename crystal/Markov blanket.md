---
tags: cyber
crystal-type: pattern
crystal-domain: cybics
alias: Markov blankets
---
the statistical boundary between an agent and its environment — the set of states that separates internal dynamics from external dynamics

a [[neuron]]'s Markov blanket in the [[cybergraph]] consists of its sensory states (incoming [[cyberlinks]]) and active states (outgoing [[cyberlinks]]). given the blanket, internal states are conditionally independent of external states

## definition

for a node $i$ in a graph, the Markov blanket $B(i)$ consists of:
- parents: nodes with edges into $i$
- children: nodes with edges from $i$
- co-parents: other parents of $i$'s children

given $B(i)$, the internal state of $i$ is independent of all other nodes. the blanket carries all the information $i$ needs to infer the world and act on it

## in active inference

[[Karl Friston]] uses Markov blankets to define what an agent IS: any system with a Markov blanket that minimizes variational [[free energy]] across that boundary is an agent performing [[active inference]]

- sensory states: observations flowing in (link arrivals, traffic, [[token]] flows)
- active states: actions flowing out (create [[cyberlinks]], stake, sample)
- internal states: beliefs $q_\theta(z)$ about hidden causes

the blanket is not designed — it is discovered from the graph topology

## hierarchical blankets

the [[cybergraph]] decomposes into nested modules:

- a single [[neuron]] has a blanket (its direct connections)
- a cluster of [[neurons]] has a blanket (the boundary edges of the cluster)
- the entire network has a blanket (its interface with external systems)

each level runs [[active inference]] at its own timescale — fast updates within modules, slow message passing between them. this gives scalability without losing coherence

see [[active inference]] for the framework. see [[free energy principle]] for the theory. see [[Karl Friston]] for the person