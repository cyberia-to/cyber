---
tags: cybics
crystal-type: pattern
crystal-domain: cybics
alias:: probabilistic logic
stake: 4474583683420153
diffusion: 0.0002617413451508586
springs: 0.0021100316198470986
heat: 0.0014855991843989757
focus: 0.0010609999954093593
gravity: 5
density: 7.48
---
replaces binary truth values with continuous degrees of truth in $[0, 1]$

introduced by Lotfi Zadeh (1965). conjunction is min, disjunction is max, negation is complement. generalizes classical [[logic]] — Boolean is the special case where truth is restricted to $\{0, 1\}$.

in the [[cybergraph]]: truth degree is [[focus]] weight $\pi_i \in [0, 1]$. a [[particle]] with high $\pi$ is strongly believed by the network; low $\pi$ is weakly attested. the [[tri-kernel]] computes these continuous confidence values by convergence, not by threshold. every statement in the graph has a naturally graded truth value — the collective assessment of all [[neurons]].