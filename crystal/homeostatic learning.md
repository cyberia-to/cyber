---
alias: homeostatic plasticity, synaptic scaling, homeostatic regulation
tags: neuro, learning
crystal-type: process
crystal-domain: biology
---
# homeostatic learning

the regulator that keeps neural activity within functional bounds. neither excitatory nor inhibitory in the Hebbian sense — homeostatic plasticity adjusts all synapses of a neuron proportionally to maintain a target firing rate.

$$w_{ij}(t+1) = w_{ij}(t) \cdot \frac{r_{\text{target}}}{r_i(t)}$$

where $r_i(t)$ is the current firing rate and $r_{\text{target}}$ is the setpoint. if a neuron fires too much, all its incoming weights scale down. if too little, they scale up. the mechanism is global to the neuron but local to the network — each neuron self-regulates independently.

homeostatic plasticity operates on a slower timescale than [[Hebbian learning]] and [[anti-Hebbian learning]] (hours to days vs milliseconds to minutes). it prevents runaway excitation from Hebbian reinforcement and prevents complete silencing from anti-Hebbian suppression. the system stays in a dynamic regime where learning can continue.

## in [[cyber]]

[[focus]] conservation ($\sum \phi^*_i = 1$) is the homeostatic constraint on the [[cybergraph]]. total [[attention]] is fixed — if one [[particle]] gains [[focus]], others lose it. this is synaptic scaling at the graph level: the system cannot run away because the total resource is conserved.

the exploration-exploitation balance in [[collective learning]] serves the same function:

$$\varepsilon = \beta \cdot (1 - C_{\text{local}}) \cdot S_{\text{global}}$$

weak local [[consensus]] drives exploration (scale up weak connections). strong local [[consensus]] drives exploitation (maintain current weights). the system self-regulates its learning rate.

[[forgetting]] is the temporal dimension of homeostasis — [[stake dynamics]] decay old [[cyberlinks]], preventing the graph from saturating with stale structure.

## the ternary triad

homeostatic learning is the modulatory (0) member of the three irreducible learning types: [[Hebbian learning]], [[anti-Hebbian learning]], [[homeostatic learning]]. excitation, inhibition, modulation — the ternary architecture of [[intelligence]]. see [[two three paradox]].

see [[learning]], [[synaptic plasticity]]