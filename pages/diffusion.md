alias:: random walk, markov, exploration
tags:: cyber
- first operator of the [[tri-kernel]]
- transition matrix `P = AD⁻¹` governs probability flow across the [[cybergraph]]
- `π^(t+1) = α P^T π^(t) + (1-α)u`
	- α = teleport parameter
	- u = prior (stake-weighted)
- answers: "where does probability flow?"
- this is the [[cyberank]] — probability of [[particle]] observation by random-walking [[neuron]], weighted by stake
- row-stochastic, preserves probability mass
- powers remain local. converges to unique stationary distribution under ergodicity
- locality: geometric decay via teleport parameter α
- the exploration force — a gas wandering, sampling connections
- universal pattern
	- physics: gas wandering, sampling
	- biology: synaptic chatter, neural noise
	- ecology: species dispersal, seed rain
	- economics: trade, migration, meme flow
- see [[tri-kernel]] for completeness proof
- discover all [[concepts]]
