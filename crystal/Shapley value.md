---
tags: cybernomics, cyber
crystal-type: pattern
crystal-domain: cybics
alias: Shapley, Shapley values, shapley value
---
a solution concept from cooperative game theory that assigns each player their exact fair share of the total value created by a coalition

invented by [[Lloyd Shapley]] (1953). the only attribution method satisfying all four fairness axioms simultaneously: efficiency (total value is fully distributed), symmetry (equal contributors get equal reward), null player (zero-contribution agents get nothing), additivity (attributions compose linearly across games).

for a coalition $N$ with value function $v$, the Shapley value of player $i$ is:

$$\phi_i(v) = \sum_{S \subseteq N \setminus \{i\}} \frac{|S|!\,(|N|-|S|-1)!}{|N|!} \left[ v(S \cup \{i\}) - v(S) \right]$$

the average marginal contribution of $i$ across all possible orderings in which the coalition forms.

exact computation is $O(n!)$ — intractable at scale. [[probabilistic shapley attribution]] approximates via Monte Carlo sampling: compute each transaction's individual $\Delta\mathcal{F}$, sample $k$ random orderings, cluster by affected neighborhood. complexity drops to $O(k \cdot n)$ with $k \ll n$.

in [[cyber]], the coalition is all [[neurons]] contributing [[cyberlinks]] in an epoch. the value function is the total [[focus]] shift $\Delta\phi^*$. the Shapley value distributes rewards so each [[neuron]] earns proportionally to their causal impact on the [[equilibrium]] — the only mathematically fair attribution under the four axioms.

[[Lloyd Shapley]] won the Nobel Memorial Prize in Economics (2012) for this and matching theory. the value has since become foundational in [[machine learning]] (SHAP explanations), mechanism design, and decentralized reward systems.