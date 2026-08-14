---
tags: cyber, core
alias: rational neurons, rational walker, attention walker
icon: "🧭"
crystal-type: pattern
crystal-domain: cyber
crystal-size: deep
---
the observer that [[cyberank]] models — not a pure random walk, and not a free agent outside the graph. a [[rational neuron]] is the limiting attention process whose stationary distribution is φ*: where a coherent observer spends time if they follow stake, structure, and scale the way the [[tri-kernel]] encodes

## how smart

[[rank]] is often glossed as “random walking neuron.” that names only one third of the engine

| operator | name | what the observer does | smarter than pure random walk? |
|----------|------|------------------------|--------------------------------|
| D | [[diffusion]] | follows [[cyberlinks]] with probability mass — the Markov / exploration term | baseline: token-weighted walk on the [[cybergraph]] |
| S | [[springs]] | restores structural equilibrium among neighbors — consistency, not pure chance | yes: pulls rank toward coherent neighborhoods, dampens structural nonsense |
| H | [[heat kernel]] | smooths importance across scales — multi-hop context | yes: sees beyond one hop; local noise averages out, long-range signal survives |

the full update is one fixed point:

```text
φ* = norm[ λ_d · D(φ) + λ_s · S(φ) + λ_h · H_τ(φ) ]
```

a pure random walker is D alone (classical [[pagerank]] family). a rational neuron is D + S + H at once: they explore edges, but also respect spring equilibrium and multi-scale heat. stake on [[cyberlinks]] weights the walk — attention follows will, not uniform chance. the [[collective focus theorem]] guarantees a unique positive φ* under the usual connectivity conditions; the [[tru]] computes it deterministically

so how smart? smart enough to be three physics at once — diffusion for curiosity, springs for coherence, heat for patience — and dumb enough to stay local: each update only needs an O(log(1/ε))-hop neighborhood ([[bounded locality]]). smarter than PageRank (structure + scale), dumber than an unbounded planner (no global search, no off-graph oracle). the intelligence is the fixed point of rational attention, not a simulation of free will

## what it is not

not a separate person outside the protocol. every real [[neuron]] that links already stakes and shapes the same graph; cyberank is the observation probability that graph implies for a rational reader of it. not a vote. not a clock. not “six blocks later”

## uses

[[cyberank]] per [[particle]] is the rational-neuron observation mass. it feeds [[foculus]] finality (φ*_i > τ), [[karma]] for linkers, [[syntropy]] of the whole, and ranking in [[cyb]]

see [[tri-kernel]], [[focus]], [[rank]], [[collective focus theorem]]

discover all [[concepts]]
