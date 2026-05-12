---
tags: cyber, physics
crystal-type: measure
crystal-domain: cybics
---
the energy available to do work — the portion of total [[energy]] not locked up in [[entropy]]

three formulations, one idea: systems spontaneously minimize free energy, and what remains at the minimum is [[equilibrium]]

## thermodynamic

Helmholtz: $F = E - TS$, where $E$ is internal [[energy]], $T$ is [[temperature]], $S$ is [[entropy]]

Gibbs: $G = H - TS$, where $H$ is enthalpy

a system at constant temperature spontaneously evolves toward the state that minimizes $F$. this is the second law of [[thermodynamics]] restated: the universe doesn't maximize disorder — it minimizes free energy

## variational (Friston)

the [[free energy principle]]: biological agents minimize variational free energy to persist

$$F = E_{q_\theta}[\log q_\theta(z) - \log p(s, z)]$$

where $q_\theta(z)$ is the agent's beliefs about hidden states, $p(s,z)$ is the generative model, $s$ is observations. minimizing $F$ simultaneously sharpens beliefs (perception) and selects actions (planning)

see [[active inference]] for the computational framework. see [[Karl Friston]] for the originator

## tri-kernel functional

the [[tri-kernel]] fixed point minimizes a unified free energy over the [[cybergraph]]:

$$\mathcal{F}(\phi) = \lambda_s\left[\frac{1}{2}\phi^\top L\phi + \frac{\mu}{2}\|\phi-x_0\|^2\right] + \lambda_h\left[\frac{1}{2}\|\phi-H_\tau\phi\|^2\right] + \lambda_d \cdot D_{KL}(\phi \| D\phi) - T \cdot S(\phi)$$

the spring term encodes structural coherence via the graph [[Laplacian]]. the heat term penalizes deviation from context-smoothed state. the [[diffusion]] term aligns with [[random walk]] distribution. the [[entropy]] term $S(\phi)$ encourages [[diversity]]

the weights $\lambda_s, \lambda_h, \lambda_d$ emerge as Lagrange multipliers — not tuned, but derived from the variational optimization

the solution: $\phi^*_i \propto \exp(-\beta[E_{\text{spring},i} + \lambda E_{\text{diffusion},i} + \gamma C_i])$ — a [[Boltzmann distribution]]

## the connection

all three formulations share the same structure: an energy term competing with an entropy term, balanced by temperature. the minimum is always a [[Boltzmann distribution]]. [[thermodynamics]] discovered it for gases. [[Karl Friston]] applied it to brains. [[cyber]] applies it to [[knowledge]]

Δφ* in [[learning incentives]] is the gradient of $\mathcal{F}$ — creating valuable structure in the [[cybergraph]] is literally reducing free energy

see [[cybics]] for the full unification. see [[negentropy vs entropy]] for the dual thermodynamics framework. see [[contextual free energy model]] for the context-dependent extension