icon:: 🎯
alias:: attention distribution, π
tags:: cyber
- emergent attention distribution over the [[cybergraph]]
- stationary vector of the [[token]]-weighted random walk
- assigned by computation. converged from collective dynamics
- computed by the [[tri-kernel]]: three local operators that are the only ones surviving the locality constraint
	- [[diffusion]]: where probability flows. exploration
	- [[springs]]: what satisfies structural constraints. hierarchy
	- [[heat kernel]]: what the graph looks like at scale τ. adaptation
- composite update: `φ^(t+1) = norm[λ_d · D(φ^t) + λ_s · S(φ^t) + λ_h · H_τ(φ^t)]`
- fixed point minimizes the free-energy functional
- focus regenerates proportionally to stake
- focus is consumed by [[cyberlinks]] and computation
- conservation: the sum of all focus equals 1. emphasizing one thing defocuses others
- [[threshold]]: minimum focus required for a [[cyberlink]] to affect ranking
- bounded locality: updates cost O(degree) not O(graph size)
- derived metrics
	- [[cyberank]]: per-[[particle]] score from the full [[tri-kernel]]
	- [[karma]]: per-[[neuron]] aggregate focus across linked [[particles]]
- see [[tri-kernel]] for the completeness proof
- see [[focus flow whitepaper]] for full protocol specification
- see [[focusflow blueprint]] for implementation architecture
- discover all [[concepts]]