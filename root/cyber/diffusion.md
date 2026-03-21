---
alias: random walk, markov, exploration, diffusion
tags: cyber
crystal-type: process
crystal-domain: cyber
stake: 18413858326369884
diffusion: 0.006659191981963559
springs: 0.0005263965087819268
heat: 0.0024379164149360426
focus: 0.003975098226603514
gravity: 73
density: 4.12
---
first operator of the [[tri-kernel]]

transition matrix `P = AD⁻¹` governs probability flow across the [[cybergraph]]

`π^(t+1) = α P^T π^(t) + (1-α)u`

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