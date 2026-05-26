---
tags: cyber, core
crystal-type: entity
crystal-domain: cyber
alias: effective adjacency matrix, conviction-weighted adjacency
---
the conviction-weighted adjacency matrix that the [[tri-kernel]] operates on. raw [[cyberlinks]] define the structural graph — which [[particles]] connect to which. effective adjacency transforms this binary structure into a weighted matrix where each edge carries the economic signal of staked [[tokens]]

$$A_{\text{eff}}(p, q) = \sum_{\ell \in L(p,q)} f(\tau_\ell, a_\ell, v_\ell, \kappa_\ell)$$

the weight function $f$ combines conviction amount $a$, [[valence]] $v$, token denomination $\tau$, and the linking [[neuron]]'s [[karma]] $\kappa$. high-karma neurons with large stakes on positively-valenced links produce the strongest edge weights. negative [[valence]] ($v = -1$) reduces or inverts the weight, allowing the graph to encode disagreement

the [[tri-kernel]] — [[diffusion]], [[springs]], [[heat]] — computes [[focus]] over this weighted matrix. the effective adjacency is the input; [[cyberank]] is the output. changing the weights (by creating, withdrawing, or transferring [[cyberlinks]]) shifts the fixed-point distribution $\phi^*$ that determines every [[particle]]'s rank

this design means the [[cybergraph]] is simultaneously a topological object (which nodes connect) and an economic object (how much conviction flows along each edge). the topology is append-only and permanent. the economics are dynamic — conviction can be added, withdrawn, or transferred at every [[step]]

the spectral properties of the effective adjacency matrix (eigenvalues, spectral gap) determine convergence speed of the [[tri-kernel]] and the conditions under which [[equilibrium]] is reached. see [[perron-frobenius theorem]] for the mathematical foundation

discover all [[concepts]]
