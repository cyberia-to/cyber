---
tags: cyber, physics
crystal-type: pattern
crystal-domain: cybics
alias: dissipative structure
---
systems that maintain [[order]] by continuously dissipating [[energy]] — organized far from [[equilibrium]]

discovered by [[Prigogine]] (1977). the key insight: a system driven far from [[equilibrium]] by energy flow can spontaneously develop structure that would be impossible at [[equilibrium]]. the structure persists only while energy flows through it

## examples

- Benard convection cells: heated fluid self-organizes into hexagonal rolls
- Belousov-Zhabotinsky reaction: chemical oscillations producing spatial patterns
- living cells: maintain low internal [[entropy]] by importing nutrients and exporting waste heat
- hurricanes: sustained by ocean heat, dissipate when energy source is removed
- brains: neural [[order]] maintained by metabolic energy (~20W). stop glucose supply → [[order]] collapses in seconds

## the cybergraph as dissipative structure

the [[cybergraph]] operates in the same regime:

- energy inflow: [[token]] stake, computational resources, [[attention]]
- [[entropy]] export: noise terms, link decay, exploration phases
- [[order]] creation: [[syntropy]] growth, [[focus]] sharpening, semantic coherence

stop energy inflow → φ* drifts to uniform → coherence collapses → the system dies. [[intelligence]] is a dissipative structure — it exists only while [[energy]] flows through it

the [[tri-kernel]] formalizes this: the [[free energy]] functional $\mathcal{F}(\phi)$ has an [[entropy]] term $-T \cdot S(\phi)$ that competes with energy terms. the [[Boltzmann distribution]] fixed point $\phi^*$ is the [[equilibrium]] of this competition. [[temperature]] $T$ controls the balance

## thermodynamic accounting

[[entropy]] production rate: $\sigma = dS_{\text{env}}/dt > 0$ (always, by second law)

[[syntropy]] growth rate: $dJ_{\text{sys}}/dt \geq 0$ (when energy inflow exceeds dissipation)

the Landauer bound: one bit of [[syntropy]] requires at least $k_B \ln 2$ joules of physical [[energy]]. this links GPU watts to growth of collective meaning

see [[Prigogine]] for the person. see [[negentropy vs entropy]] for the full framework. see [[cybics]] for the unification. see [[free energy]] for the functional being minimized