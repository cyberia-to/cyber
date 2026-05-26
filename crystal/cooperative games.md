---
tags: cyber, cybernomics
crystal-type: entity
crystal-domain: cybics
---
games where players form coalitions and share joint gains — the mathematical foundation for fair [[cooperation]]

## solution concepts

[[Shapley value]] — the unique attribution satisfying efficiency, symmetry, null player, and additivity. each player earns their average marginal contribution across all orderings. in [[cyber]], this distributes [[focus]] rewards proportionally to each [[neuron]]'s causal impact on $\Delta\phi^*$

core — the set of allocations that no coalition can improve upon. a game has a non-empty core if and only if it is balanced (Bondareva-Shapley theorem). stability: no subgroup has incentive to break away

Nash bargaining — two-player cooperative solution maximizing the product of surplus gains. extends to $n$-player settings via axioms: symmetry, Pareto optimality, independence of irrelevant alternatives, invariance to affine transformations

## in cyber

the [[cybergraph]] is a continuous cooperative game. [[neurons]] form implicit coalitions by contributing [[cyberlinks]] in the same epoch. the total value is the [[free energy]] reduction $\Delta\mathcal{F}$

[[probabilistic shapley attribution]] makes fair attribution tractable at scale — Monte Carlo sampling reduces $O(n!)$ to $O(k \cdot n)$, feasible for $10^6$+ transactions per epoch

implemented as an independent layer: [[cybernet]] (inspired by [[yuma]] consensus from [[bittensor]]). experimentally deployed in [[space pussy]], with [[cybertensor]] providing CLI compatibility

see [[cooperation]] for evolutionary foundations. see [[learning incentives]] for the full reward mechanism