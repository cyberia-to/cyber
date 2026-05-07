---
tags: cyber
crystal-type: pattern
crystal-domain: cybics
alias: FEP
---
any system that persists must minimize variational [[free energy]] — or equivalently, maximize the evidence for its own generative model of the world

originated by [[Karl Friston]] (2006). the principle unifies [[thermodynamics]], [[information theory]], and [[biology]] under a single variational bound

## the claim

a self-organizing system at [[equilibrium]] with its environment occupies states that minimize surprise (the negative log-probability of observations). since surprise is intractable, the system minimizes an upper bound: variational [[free energy]]

$$F = D_{KL}(q_\theta(z) \| p(z|s)) - \log p(s) \geq -\log p(s)$$

minimizing $F$ simultaneously:
- improves perception (sharpen $q_\theta$ toward the true posterior)
- reduces surprise (select actions that make observations expected)
- builds structure (learn generative models that compress regularities)

## implications

- perception, action, and [[learning]] are aspects of one optimization process
- agency emerges from [[free energy]] minimization — goal-directed behavior is a consequence, not an assumption
- [[Markov blankets]] define the boundary between agent and environment: states that separate internal from external dynamics
- [[precision]] (inverse variance) weights prediction errors — attention as confidence-weighted error

## in cyber

each [[neuron]] in the [[cybergraph]] can be modeled as an [[active inference]] agent minimizing variational [[free energy]]:

- observations: local traffic, link arrivals, [[token]] flows
- beliefs: variational posterior $q_\theta(z)$ over latent graph states
- actions: create [[cyberlinks]], stake, sample [[particles]]
- [[precision]]: adaptive [[token]] staking that amplifies trusted signals

the [[tri-kernel]] [[free energy]] functional $\mathcal{F}(\phi)$ is a collective analog — the entire [[cybergraph]] minimizes free energy through distributed local updates

see [[active inference]] for the computational framework. see [[Karl Friston]] for the person. see [[free energy]] for the three formulations. see [[cybics]] for the integration with [[cyber]]