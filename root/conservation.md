---
tags: cyber, core
alias: conservation law, conservation laws, conserved quantity
crystal-type: pattern
crystal-domain: cybics
crystal-size: bridge
---
a quantity that remains constant through every transformation. the constraint that shapes where [[convergence]] can go

without conservation, a system can collapse to zero, explode to infinity, or drift without limit. conservation forces the dynamics onto a bounded surface where the [[banach fixed-point theorem]] can find [[equilibrium]]

---

## in physics

three conservation laws hold across all known physics:

energy — the total energy of an isolated system never changes. it transforms between kinetic, potential, thermal, electromagnetic — but the sum is constant. discovered empirically, later understood as a consequence of time-translation symmetry ([[Noether]]'s theorem, 1918)

momentum — total momentum is conserved in the absence of external forces. consequence of space-translation symmetry

charge — electric charge is neither created nor destroyed. consequence of gauge symmetry

every conservation law corresponds to a symmetry of the system ([[Noether]]'s theorem). conservation is not a rule imposed from outside — it is structure that the dynamics cannot violate

---

## in cyber

the [[cybergraph]] has three conservation laws enforced at every state transition:

### focus conservation

$$\sum_i \text{focus}(i) = 1 \quad \text{always}$$

[[focus]] can flow between [[neurons]], be consumed by computation, and regenerate proportionally to [[stake]]. it cannot be created from nothing, destroyed, or exceed 1 in total

this single constraint does the work that other systems split across gas models, fee markets, and priority auctions. it forces the [[tri-kernel]] onto the probability simplex $\Delta^{|P|-1}$, where [[convergence]] produces a unique [[Boltzmann distribution]] as [[equilibrium]]

enforced in [[nox]] by stark circuit constraints — an invalid conservation proof means an invalid state transition, rejected by every verifier

### balance conservation

$$\sum_i \text{balance}(i) = B_{\text{total}} \quad \text{for non-minting transactions}$$

[[tokens]] move between [[neurons]] but the total supply is fixed outside minting events. enforced by polynomial commitment structure

### energy conservation (privacy layer)

$$\sum(\text{record values}) = \text{initial} + \text{minted} - \text{burned}$$

enforced by ZK circuit constraints. the network verifies conservation without seeing individual values — private ownership with public aggregates

---

## why conservation shapes convergence

conservation is not a side constraint. it is the reason convergence produces something meaningful

without $\sum \phi_i = 1$: the [[tri-kernel]] could push all focus to zero (everything becomes irrelevant) or to infinity (everything becomes infinitely important). both are meaningless. conservation eliminates these degenerate outcomes and forces the system to make choices — emphasizing one [[particle]] necessarily defocuses others

this is why [[focus]] works as both attention and fuel simultaneously. a conserved quantity that represents attention is automatically scarce. scarcity forces prioritization. prioritization creates structure. structure is [[syntropy]]

in [[thermodynamics]]: energy conservation forces the system to find the [[Boltzmann distribution]] — the unique distribution that maximizes [[entropy]] subject to fixed total energy. in [[cyber]]: focus conservation forces the system to find $\phi^*$ — the unique distribution that minimizes [[free energy]] subject to fixed total focus. same mathematics, same principle

---

## conservation and costly signals

conservation is what makes [[cyberlinks]] meaningful. because [[focus]] is conserved, spending it on a link is a real sacrifice — the [[neuron]] cannot spend the same focus elsewhere. this is the [[costly signal]] property

without conservation, signaling is free. free signals carry no information (cheap talk). conservation transforms every [[cyberlink]] into an economic commitment — a statement backed by finite resources. this is the bridge between physics and [[game theory]]: conservation laws create the scarcity that makes [[incentives]] work

---

## conservation and proof by simulation

the [[cybics]] postulate: every truth accessible to [[intelligence]] is a fixed point of convergent simulation under conservation laws

the last three words are load-bearing. convergence without conservation is unconstrained optimization — it can find any fixed point, including trivial ones. conservation constrains the space of admissible states, ensuring the fixed point is physically meaningful

in the formal definition: a simulation-proof of property $P$ requires a dynamical system $(Ω, T, C)$ where $C(T(ω)) = C(ω)$ for all $ω$. the conservation law $C$ is part of the proof. remove it and the proof loses its anchor

---

## the symmetry beneath

[[Noether]]'s theorem: every continuous symmetry of a system implies a conserved quantity

in the [[cybergraph]], focus conservation corresponds to a symmetry: the [[tri-kernel]] is invariant under relabeling of time steps. it does not matter when a [[cyberlink]] is created — the same graph structure produces the same $\phi^*$. this time-invariance is the symmetry; focus conservation is the consequence

see [[convergence]] for why conservation shapes the destination. see [[focus]] for the conserved quantity. see [[costly signal]] for the economic consequence. see [[cybics]] for the philosophical role

discover all [[concepts]]
