icon:: 🔒
tags:: cyber
- private value bound to a [[particle]] and an owner ([[neuron]])
- hidden behind commitments. spent via ZK proofs
- the economic substrate of the system
- records enable private transfers without revealing sender, receiver, or amount — while proving conservation
- the mutator set (AOCL + SWBF) tracks record lifecycle without ever exposing which record was spent
- structure
	- particle: content identifier (which [[particle]] this value is bound to)
	- value: energy amount
	- owner: owner [[neuron]] hash
	- nonce: random for uniqueness
- commitment: `H_commit(particle ‖ value ‖ owner ‖ nonce ‖ ρ)` where ρ is hiding randomness
- spending a record
	- prove ownership via ZK proof
	- prove record exists in AOCL (append-only commitment list)
	- set bits in SWBF (sliding-window bloom filter) to prevent double-spend
	- the link between addition and removal is invisible to any observer
- see [[data_structure_for_superintelligence]] for full mutator set architecture
- discover all [[concepts]]
