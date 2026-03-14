---
alias: impulse, focus impulse, π_Δ, pi_delta, impulses
tags: cyber, core
crystal-type: process
crystal-domain: cyber
---
# impulse

the proven change in [[focus]] that a [[neuron]] delivers to the [[cybergraph]] via a [[signal]]. mathematically $\pi_\Delta$ — a sparse vector of (particle_id, $\Delta\pi$) pairs representing how the [[focus]] distribution $\pi^*$ shifts when the signal's [[cyberlinks]] are applied

in physics, impulse is force applied over time that changes momentum ($J = \Delta p$). in neuroscience, the nerve impulse is the action potential that propagates through a network and changes downstream potentials. in cyber, the impulse is the neuron's proven push on collective [[focus]] — discrete, has magnitude, delivered at a specific moment, and propagates through the [[cybergraph]]

## computation

the [[neuron]] computes the impulse by running the [[tri-kernel]] locally on their $O(\log(1/\varepsilon))$-hop neighborhood, adding their [[cyberlinks]], and measuring how $\pi$ shifts. the [[locality theorem]] guarantees effects beyond that radius are below $\varepsilon$ — most entries are zero, so the sparse representation is compact

the result is whatever the math says. there is no target, no threshold, no minimum. a link to a well-connected [[particle]] in a sparse region produces a larger impulse than a redundant link in a dense cluster. the neuron discovers their contribution by computing it

## proof

the impulse is accompanied by a [[STARK]] proof $\sigma$ that certifies correctness against the current [[BBG]] root. the proof covers the entire [[signal]] — all [[cyberlinks]] in the batch, all conviction UTXO movements, and the resulting $\pi_\Delta$ — in a single recursive verification. any node checks $\sigma$ in $O(\log n)$ without recomputing the [[tri-kernel]]

## reward

the impulse proof doubles as a reward claim. if $\|\pi_\Delta\| > 0$ and $\sigma$ is valid, the [[neuron]] self-mints [[$CYB]] proportional to the proven shift. no aggregator decides the reward — the proof IS the mining. see [[cyber/rewards]] for the full reward specification

## conservation

total minting per epoch is bounded by the actual global $\Delta\pi$, verifiable from consecutive headers. if the sum of individual impulses exceeds the actual shift (overlapping neighborhoods), all claims are scaled proportionally

see [[signal]], [[focus]], [[cyber/rewards]], [[cyber/network]]
