icon:: 🎯
alias:: attention distribution, π
tags:: cyber
- emergent attention distribution over the [[cybergraph]]
- stationary vector of the [[token]]-weighted random walk
- not assigned — computed. not voted — converged
- three ranks are computed from focus
	- [[cyberank]]: probability of [[particle]] observation by random-walking [[neuron]], weighted by stake. determines content relevance
	- [[karma]]: aggregate focus earned by a [[neuron]] across all [[particles]] it has linked. determines reputation
	- topic rank: focus concentration within a namespace or semantic cluster. determines which domains of [[knowledge]] the network is currently attending to
- focus regenerates proportionally to stake
- focus is consumed by [[cyberlinks]] and computation
- conservation: the sum of all focus equals 1. emphasizing one thing defocuses others
- focus flow equation
	- `π_i = Σ_j P_ij · π_j + r_i - c_i`
	- where `P_ij = (w_ij · b_j) / (Σ_k w_kj · b_k)`
	- w_ij = edge weight, b_j = balance of [[neuron]] j
	- r_i = regeneration (proportional to balance)
	- c_i = consumption ([[cyberlinks]] + computation)
- convergence guaranteed by Perron-Frobenius theorem for irreducible aperiodic transition matrices
- bounded locality: updates cost O(degree) not O(graph size)
- see [[focus_flow_whitepaper]] for full protocol specification
- see [[focusflow blueprint]] for implementation architecture
- discover all [[concepts]]
