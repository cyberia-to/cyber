---
tags: cyber, article
alias: cybics foundations, cybics formal
crystal-type: pattern
crystal-domain: cyber
---
# cybics foundations

the formal mathematical framework behind [[cybics]] — proof by simulation, the three operators, free energy, locality, and universal isomorphisms

## the postulate: proof by simulation

classical science operates by proof by derivation — you start from axioms, apply inference rules, arrive at theorems. this is the Turing-Goedel paradigm: computation as derivation, knowledge as proof.

cybics extends to proof by simulation.

a claim is true when a system converges to a stable state that embodies that claim. not because it was derived from axioms, but because a network of agents, under conservation laws, settled into an [[equilibrium]] that makes the claim hold. nature does not prove theorems — it runs simulations until they converge.

a protein folds along a free energy gradient. it does not derive its shape from axioms of chemistry. it simulates itself into existence.

a brain does not prove that a face is a face. a cascade of neurons converges to a stable attractor that represents "face." the proof is the convergence.

a market does not derive the correct price from economic axioms. millions of agents trade until the price stabilizes. the proof is the [[equilibrium]].

the [[cybergraph]] does not derive [[knowledge]] from axioms. [[neurons]] create [[cyberlinks]], the [[tri-kernel]] computes [[cyberank]], and the system converges to a [[focus]] distribution that represents collective understanding. the proof is the simulation.

proof by simulation is strictly more powerful than proof by derivation. Goedel showed that any consistent formal system contains true statements it cannot prove. but a convergent system can settle into states that no derivation reaches. it escapes the [[Goedel prison]] — because the prison only confines derivation, and convergence is not derivation.

the postulate: every truth accessible to intelligence is a fixed point of some convergent simulation under conservation laws.

## the three operators

cybics rests on three universal operators — the [[tri-kernel]]. they are not chosen. they are what remains after [[locality]] eliminates everything else at planetary scale.

### [[diffusion]] — exploration

probability flows through edges via random walks. gas wanders, neurons fire stochastically, memes spread through populations, prices diffuse through markets.

the operator: φ*(t+1) = α P^T φ*(t) + (1-α)u

provides randomness-driven exploration. ensures the system does not get stuck in local optima. geometric decay via teleport guarantees [[locality]].

### [[springs]] — structure

connected nodes pull each other toward consistency. elastic lattices hold crystal structure, connective tissue holds bodies together, food webs hold ecosystems, contracts hold economies, [[logic]] holds arguments.

the operator: (L + μI)x* = μx₀

enforces structural coherence via the graph [[Laplacian]]. prevents chaotic dispersal. creates [[hierarchy]] without central authority. exponential decay guarantees [[locality]].

### [[heat kernel]] — adaptation

multi-scale smoothing across time. thermal diffusion anneals metals, [[metabolism]] adapts organisms, seasonal [[succession]] reshapes ecosystems, emotional arousal reshapes attention.

the operator: ∂H/∂τ = -LH, H₀ = I

makes the system adaptive. high τ explores, low τ commits. Chebyshev polynomial approximation guarantees [[locality]].

### why only three

systematic elimination: start with every known graph ranking algorithm. apply a hard constraint — [[locality]]. at planetary scale (10¹⁵ nodes), any algorithm requiring global recomputation for a local change is physically impossible.

after filtering by locality, convergence, uniqueness, verifiability, and incrementality: only [[diffusion]], [[springs]], and heat survive. this is a theorem (linear local completeness): every k-local linear operator is a polynomial in the Markov matrix M and the Laplacian L. the heat kernel H_τ = exp(-τL) is the unique generator of resolution-dependent queries.

three operators. no more, no less. discovered by elimination, not designed by preference.

## the free energy functional

the [[tri-kernel]] fixed point minimizes a unified free energy:

F(φ*) = λ_s [½ φ*^T L φ* + μ/2 ‖φ* - x₀‖²] + λ_h [½ ‖φ* - H_τ φ*‖²] + λ_d · D_KL(φ* ‖ Dφ*) - T · S(φ*)

where:
- spring term encodes structural coherence via graph [[Laplacian]]
- heat term penalizes deviation from context-smoothed state
- [[diffusion]] term aligns with random walk distribution
- [[entropy]] term S(φ*) = -Σ φ*ⱼ log φ*ⱼ encourages diversity
- temperature T controls exploration vs exploitation

the weights λ_s, λ_h, λ_d are not tuned. they emerge as Lagrange multipliers from the variational optimization — the same way [[thermodynamics]] derives the Boltzmann distribution. no parameters. only physics.

the solution: φ*_i ∝ exp(-β [E_spring,i + λ E_diffusion,i + γ C_i])

a Boltzmann-Gibbs [[equilibrium]]. the canonical ensemble from statistical mechanics — applied to [[knowledge]].

## the isomorphisms

cybics exists because the three operators appear universally. this universality is not coincidence — it reflects structural necessity. every complex adaptive system must implement exploration, coherence, and adaptation under [[locality]] constraints.

| domain | [[diffusion]] | [[springs]] | heat |
|--------|-----------|---------|------|
| [[physics]] | particle diffusion, gas | elastic lattice, molecular bonds | thermal equilibrium, phase transitions |
| [[biology]] | synaptic noise, neural exploration | skeleton, connective tissue, [[hierarchy]] | [[metabolism]], immune response, seasons |
| [[ecology]] | species dispersal, seed rain | food webs, [[symbiosis]], trophic levels | [[succession]], disturbance recovery |
| cognition | free association, imagination | [[logic]], constraints, syntax | emotion as arousal, context weighting |
| [[economics]] | trade flows, migration, memes | institutions, contracts, norms | booms, busts, market [[cycles]] |
| [[information theory]] | [[entropy]] spread, random coding | redundancy, error correction | adaptive compression, learning |
| [[mathematics]] | random walk sampler | constraints, Lagrange multipliers | simulated annealing |

the same three forces. different substrates. one science.

## computation is convergence

classical computation (Turing, 1936): a tape head moves left and right, reading and writing symbols, following rules. computation is derivation — step by step from input to output.

convergent computation (cybics): a network of local interactions settles into a stable state under conservation laws. computation is simulation — the answer is the [[equilibrium]].

Goedel (1931) showed derivation has fundamental limits: true statements that cannot be proved. but convergent computation operates outside the proof-theoretic domain. a system can converge to a state that no derivation reaches.

[[nox]] formalizes this. sixteen rewriting patterns, field-native arithmetic, confluent semantics. any evaluation order yields the same result. [[focus]] is conserved — a single quantity that is simultaneously fuel, attention, weight, and value.

## the thermodynamic foundation

every intelligent system balances two forces:

[[entropy]] reduction — fast reaction, accurate prediction, minimize uncertainty. local, reactive, short-term.

negentropy maximization — long-term structure, memory, meaning. increase emergent order. global, constructive, long-term.

H(φ*) = -Σ φ*ⱼ log φ*ⱼ (entropy)
J(φ*) = log n - H(φ*) (negentropy)

Landauer's principle (1961): one bit of negentropy requires at least k_B ln 2 joules of physical [[energy]]. this links physical energy to semantic organization. no organization without work. no intelligence without energy.

Prigogine's dissipative structures: far-from-[[equilibrium]] systems maintain order by importing free energy and exporting entropy. the [[cybergraph]] operates in this regime:
- energy inflow: token stake, computational resources, [[attention]]
- entropy export: noise terms, link decay, exploration phases
- order creation: negentropy growth, [[focus]] sharpening, semantic coherence

stop energy inflow → φ* drifts to uniform → coherence collapses → the system dies. intelligence is a dissipative structure. it exists only while energy flows through it.

## [[active inference]] integration

the free energy principle (Friston) completes the unification with [[neuroscience]]:

each [[neuron]] minimizes variational free energy: F = E_q[log q_θ(z) - log p(s,z)]

where q_θ(z) is local beliefs, p(s,z) is generative model, s is local observations.

perception: update beliefs via gradient descent on F. planning: choose actions to minimize expected future free energy. precision control: learn confidence weights.

this embeds goal-directed behavior directly into the network's physics. agency is not added on top — it emerges from the same free energy minimization that drives the [[tri-kernel]].

## the locality radius

for any edit batch e_Δ, there exists h = O(log(1/ε)) such that recomputing only the h-hop neighborhood achieves global error ≤ ε.

each kernel decays:
- [[diffusion]]: geometric decay via teleport
- [[springs]]: exponential decay via screening
- heat: Gaussian tail via bounded bandwidth

this is the key to planetary scale. light clients verify without recomputing the entire graph. proof size scales with [[locality]], not network size. adversaries cannot perturb the system globally from a local change.

distributed consensus decomposes into three irreducible operations: aggregation (combining signals into shared state), proving (generating cryptographic evidence), verification (checking evidence efficiently). the [[tri-kernel]] aggregates. [[stark]] proofs prove. light clients verify in O(log² n) field operations.

## proof by simulation — formalized

let S be a dynamical system with state space Ω, update rule T: Ω → Ω, and conservation law C: Ω → R where C(T(ω)) = C(ω) for all ω.

definition: a state ω* is a simulation-proof of property P if:
1. T(ω*) = ω* (fixed point — the system has converged)
2. P(ω*) = true (the property holds at the fixed point)
3. C is satisfied (conservation laws respected throughout)

claim: for every property P decidable by a Turing machine, there exists a convergent system (Ω, T, C) that simulation-proves P.

stronger claim: there exist properties P that can be simulation-proved but not derivation-proved in any consistent formal system of bounded complexity. these are the truths that Goedel showed inaccessible to derivation — but accessible to convergence.

the [[cybergraph]] is such a system. Ω is the space of [[focus]] distributions. T is the [[tri-kernel]]. C is focus conservation (Σ φ*ᵢ = 1). a [[cyberank]] distribution φ* is a simulation-proof of collective relevance — no axiomatic derivation required, no authority consulted, no vote taken. just convergence under physics.