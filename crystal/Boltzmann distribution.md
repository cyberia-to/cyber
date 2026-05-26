---
tags: cyber, physics
crystal-type: pattern
crystal-domain: cybics
alias: Gibbs distribution, canonical ensemble
---
the probability distribution that maximizes [[entropy]] subject to a fixed average [[energy]] — the unique [[equilibrium]] of any system minimizing [[free energy]]

$$p_i \propto \exp(-\beta E_i)$$

where $E_i$ is the energy of state $i$ and $\beta = 1/T$ is inverse [[temperature]]. low-energy states are more probable. higher temperature flattens the distribution (more exploration). lower temperature sharpens it (more exploitation)

## derivation

start from the [[entropy]] maximization problem: maximize $S = -\sum_i p_i \log p_i$ subject to $\sum_i p_i = 1$ and $\sum_i p_i E_i = \langle E \rangle$

the Lagrange multiplier for the energy constraint is $\beta$, which turns out to be inverse temperature. the solution is the Boltzmann distribution. no other distribution satisfies both constraints simultaneously

discovered by [[Ludwig Boltzmann]] (1868) for gases. the same math appears everywhere a system balances [[energy]] and [[entropy]]

## in cyber

the [[tri-kernel]] fixed point is a Boltzmann distribution over [[particles]]:

$$\phi^*_i \propto \exp\big(-\beta[E_{\text{spring},i} + \lambda E_{\text{diffusion},i} + \gamma C_i]\big)$$

where the three energy terms come from the three operators: [[springs]] (structural coherence), [[diffusion]] (random walk alignment), and [[heat kernel]] context $C_i$

this is not a design choice — it is a mathematical consequence of minimizing the [[free energy]] functional $\mathcal{F}(\phi)$. the [[cybergraph]] settles into the distribution that balances structural constraints ([[energy]]) against exploratory diversity ([[entropy]])

temperature $T$ controls the tradeoff: high $T$ = dispersed [[focus]] across many [[particles]] (exploration). low $T$ = concentrated [[focus]] on high-value [[particles]] (exploitation). see [[heat]] for how $\tau$ parameter implements this

## where it appears

- [[statistical mechanics]]: energy distribution of gas molecules
- [[machine learning]]: softmax is a Boltzmann distribution with logits as energies
- [[focus flow computation]]: the local update rule converges to Boltzmann [[equilibrium]]
- [[cybics]]: the canonical ensemble applied to [[knowledge]]
- simulated annealing: optimization by cooling a Boltzmann system

see [[free energy]] for the functional being minimized. see [[entropy]] for the quantity being maximized. see [[Ludwig Boltzmann]] for the person