---
tags: cybics
crystal-type: pattern
crystal-domain: cybics
alias:: probabilistic logic
stake: 4474583683420153
diffusion: 0.00025885244775050546
springs: 0.002021195197205134
heat: 0.0014520259625771838
focus: 0.00102618997555226
gravity: 5
density: 6.63
---
replaces binary truth values with continuous degrees of truth in $[0, 1]$

introduced by Lotfi Zadeh (1965). conjunction is min, disjunction is max, negation is complement. generalizes classical [[logic]] — Boolean is the special case where truth is restricted to $\{0, 1\}$.

in the [[cybergraph]]: truth degree is [[focus]] weight $\pi_i \in [0, 1]$. a [[particle]] with high $\pi$ is strongly believed by the network; low $\pi$ is weakly attested. the [[tri-kernel]] computes these continuous confidence values by convergence, not by threshold. every statement in the graph has a naturally graded truth value — the collective assessment of all [[neurons]].