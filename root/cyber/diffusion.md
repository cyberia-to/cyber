---
alias: random walk, markov, exploration, diffusion, freedom
tags: cyber
crystal-type: process
crystal-domain: cyber
---
first operator of the [[tri-kernel]]

transition matrix `P = AD⁻¹` governs probability flow across the [[cybergraph]]

`π^(t+1) = α P^T φ*^(t) + (1-α)u`

- α = teleport parameter
- u = prior (stake-weighted)

answers: "where does probability flow?"

the exploration component of the [[cyberank]]. the full cyberank is the fixed point of all three [[tri-kernel]] operators blended together

row-stochastic, preserves probability mass

powers remain local. converges to unique stationary distribution under ergodicity

locality: geometric decay via teleport parameter α

the exploration force — a gas wandering, sampling connections

universal pattern

- physics: gas wandering, sampling
- biology: synaptic chatter, neural noise
- ecology: species dispersal, seed rain
- economics: trade, migration, meme flow

together with [[springs]] and [[heat kernel]] forms the [[tri-kernel]]

see [[tri-kernel]] for completeness proof

discover all [[concepts]]