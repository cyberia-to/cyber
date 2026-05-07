---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: focus dynamics, nox focus
---
# Focus Dynamics

[[focus]] is the collective [[attention]] distribution over all [[particles]] in the [[cybergraph]] — content-particles and [[axon]]-particles. it is not a fuel, not a token, not a per-[[neuron]] resource. it is what the [[tri-kernel]] computes from the aggregate of all [[attention]]

## How Focus Emerges

[[neurons]] lock [[balance]] to create [[will]]. [[will]] auto-distributes across [[cyberlinks]], producing [[attention]] at target [[particles]]. the [[tri-kernel]] aggregates all [[attention]] into a single [[probability]] distribution π over all [[particles]]. this distribution is focus

| Layer | What | Per-what |
|-------|------|----------|
| [[balance]] | tokens held | [[neuron]] |
| [[will]] | locked balance × time | [[neuron]] |
| [[attention]] | will allocated to targets | [[neuron]] × [[particle]] |
| focus | collective attention | [[particle]] |
| [[cyberank]] / [[prob]] | focus read at a point | [[particle]] |

## Conservation

```
Σᵢ focus(i) = 1   (always, enforced by normalization)

Focus sums to 1 because it is a probability distribution.
Emphasizing one particle defocuses all others.
This is not a separate conservation law — it is the
normalization step of the tri-kernel.
```

## Focus Flow Equation

the [[tri-kernel]] composite operator:

$$\phi^{(t+1)} = \text{norm}\big[\lambda_d \cdot D(\phi^t) + \lambda_s \cdot S(\phi^t) + \lambda_h \cdot H_\tau(\phi^t)\big]$$

where:
- $D$ — [[diffusion]] (random walk exploration)
- $S$ — [[springs]] (structural constraints via screened [[Laplacian]])
- $H_\tau$ — [[heat]] (multi-scale context smoothing)

the weights come from [[attention]]: each [[axon]]'s weight is the sum of all [[neurons]]' [[attention]] directed along that edge

## Convergence

the transition matrix P is stochastic, irreducible, aperiodic. by [[Perron-Frobenius theorem]], a unique π* exists:

$$\pi P = \pi, \quad \sum_i \pi_i = 1, \quad \pi_i > 0 \;\forall\, i$$

convergence rate determined by spectral gap: $\|\phi^{(t)} - \pi^*\| \leq C \cdot (1-\lambda)^t$

## Balance and Energy Conservation

```
BALANCE CONSERVATION
────────────────────
Σᵢ balance(i) = B_total   (for non-minting transactions)

Enforced by polynomial commitment structure.
Invalid conservation → invalid state transition → rejected.

ENERGY CONSERVATION (Privacy Layer)
───────────────────────────────────
Σ(record values) = initial + minted - burned

Enforced by ZK circuit constraints.
```

for the full probabilistic framework including axioms, proofs, and emergence theory, see [[collective focus theorem]]

see [[focus]] for the concept definition. see [[cyber/will]] for how [[will]] produces [[attention]]. see [[focus flow computation]] for the full protocol specification