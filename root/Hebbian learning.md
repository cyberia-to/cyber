---
alias: Hebbian rule, Hebb's rule, Hebbian plasticity
tags: neuro, learning
crystal-type: process
crystal-domain: biology
---
# Hebbian learning

"neurons that fire together wire together." if two neurons are active simultaneously, the connection between them strengthens. formalized by [[Donald Hebb]] (1949).

$$\Delta w_{ij} = \eta \cdot x_i \cdot x_j$$

where $x_i$ and $x_j$ are the activities of the pre- and post-synaptic neurons and $\eta$ is the learning rate. the rule is local — each synapse updates using only information available at its endpoints.

Hebbian learning is excitatory: correlated activity increases connection weight. it discovers structure by reinforcing patterns that co-occur. without a complementary mechanism, weights grow without bound — [[anti-Hebbian learning]] and [[homeostatic learning]] provide the necessary counterbalance.

## in [[cyber]]

a [[cyberlink]] between two [[particles]] that both accumulate [[focus]] is a Hebbian connection — correlated [[attention]] strengthens the link's economic weight. the reward signal $\Delta\phi^*$ reinforces links between particles that the [[cybergraph]] treats as co-relevant.

$$\Delta w_{ij} = \alpha \cdot r_{ij} \cdot \phi^*_j$$

see [[collective learning]] for the full weight update rule in the [[cybergraph]].

## the ternary triad

Hebbian learning is the excitatory (+1) member of the three irreducible learning types: [[Hebbian learning]], [[anti-Hebbian learning]], [[homeostatic learning]]. excitation, inhibition, modulation — the ternary architecture of [[intelligence]]. see [[two three paradox]].

see [[learning]], [[synaptic plasticity]]