---
tags: cybics
crystal-type: pattern
crystal-domain: cybics
alias: probabilistic logic
---
replaces binary truth values with continuous degrees of truth in $[0, 1]$

introduced by Lotfi Zadeh (1965). conjunction is min, disjunction is max, negation is complement. generalizes classical [[logic]] — Boolean is the special case where truth is restricted to $\{0, 1\}$.

in the [[cybergraph]]: truth degree is [[focus]] weight $\phi^*_i \in [0, 1]$. a [[particle]] with high $\phi^*$ is strongly believed by the network; low $\phi^*$ is weakly attested. the [[tri-kernel]] computes these continuous confidence values by convergence, not by threshold. every statement in the graph has a naturally graded truth value — the collective assessment of all [[neurons]].