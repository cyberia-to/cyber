---
alias: anti-Hebbian rule, anti-Hebbian plasticity, decorrelation learning
tags: neuro, learning
crystal-type: process
crystal-domain: biology
---
# anti-Hebbian learning

the inverse of [[Hebbian learning]]: correlated activity weakens the connection. neurons that fire together lose their shared weight.

$$\Delta w_{ij} = -\eta \cdot x_i \cdot x_j$$

anti-Hebbian plasticity serves as an inhibitory signal. where Hebbian learning concentrates representation (amplifying co-occurring patterns), anti-Hebbian learning decorrelates representation (suppressing redundancy). the result: sparse, efficient codes where each neuron carries independent information.

found in the cerebellum (parallel fiber to Purkinje cell synapses), the hippocampus (feedforward inhibition), and in independent component analysis (ICA) — a computational model that recovers statistically independent sources from mixed signals.

## in [[cyber]]

[[market inhibition]] is the anti-Hebbian mechanism in the [[cybergraph]]. the [[inversely coupled bonding surface]] (ICBS) suppresses [[cyberlinks]] the collective disbelieves — edges with low market-implied probability are weighted toward zero. [[karma]] penalizes [[neurons]] whose links consistently lose market confidence.

$$A^{\text{eff}}_{pq} = \sum a(\ell)\cdot \kappa(\nu(\ell))\cdot f(m(\ell))$$

when $m(\ell) \to 0$ (market rejects the link), $f(m(\ell)) \to 0$ — the connection is suppressed. this is anti-Hebbian: correlated rejection weakens the edge.

## the ternary triad

anti-Hebbian learning is the inhibitory (-1) member of the three irreducible learning types: [[Hebbian learning]], [[anti-Hebbian learning]], [[homeostatic learning]]. excitation, inhibition, modulation — the ternary architecture of [[intelligence]]. see [[two three paradox]].

see [[learning]], [[synaptic plasticity]]