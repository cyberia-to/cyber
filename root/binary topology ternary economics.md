---
tags: cyber, article, draft, research
alias: binary topology ternary economics, binary ternary architecture, two layer architecture
crystal-type: pattern
crystal-domain: cyber
crystal-size: bridge
authors: mastercyb
---

an architectural principle for decentralized [[superintelligence]]

*mastercyb · Cyber Valley · 2026*

---

## observation

every known system that produces collective [[intelligence]] — mycorrhizal networks, neural networks, economies, ecosystems — shares the same two-layer architecture.

connection topology is binary. a connection either exists or it doesn't. a hypha either links two trees or it doesn't. a synapse is either formed or it isn't. a [[cyberlink]] either exists or it doesn't. binarity at the connection level ensures maximum noise immunity and simplicity: a graph is a set of edges, each edge is a bit.

exchange economics over connections is ternary. through an existing connection, flow operates in one of three modes: give (+1), receive (−1), or maintain the connection with no net flow (0). the neutral state is not the absence of a connection (that would be a return to binarity) but active maintenance of a channel in standby mode. this is a fundamentally different "nothing" than the absence of an edge.

the binary layer answers the question "with whom?". the ternary layer answers "how?". the separation of these two questions is not a modeling simplification but a fundamental property of efficient computational systems.

see [[two three paradox]] for why 2 and 3 are irreducible foundations.

---

## mycelium as reference implementation

the mycorrhizal network is the purest natural realization of this architecture.

### binary layer: topology

a hypha is a tube connecting two nodes (tree, shrub, seedling). it either exists or doesn't. creating a new hypha is expensive (chitin wall synthesis, growth, navigation through soil). destruction is cheap (die-off, microfauna consumption, desiccation). this creates asymmetry: the network is easier to destroy than to build, so existing connections are valuable.

mycorrhizal network topology is neither a random graph nor a regular lattice. it is a scale-free network with characteristic degree distribution: a few hub nodes (mother trees) with hundreds of connections, many peripheral nodes with single-digit connections. the same topology as the internet, social networks, and metabolic pathways.

### ternary layer: economics

through an existing hypha flow carbon (as sugars), phosphorus, nitrogen, water, and signaling molecules. flow direction is determined by concentration gradients but regulated by the fungus. three modes:

+1: give. a tree with surplus photosynthate (in sunlight, mature, healthy) gives sugars to the network. the fungus transports them, taking a 10–30% commission. this is an economic transaction with an intermediary.

−1: receive. a seedling in shade, a sick tree, a tree in early spring (still without leaves) — these are receivers. they take from the network more than they give. this is the network's investment in a node's future productivity: the seedling will grow, the sick tree will recover, the spring tree will unfurl its leaves.

0: neutral. the connection exists, flow is near zero. this is not a useless connection — it is a latent channel. resources don't flow through it, but signaling molecules do. when one tree is attacked by insects, the alarm signal propagates across the entire network, including neutral connections. zero economic flow ≠ zero informational function.

### why it works

the separation of binary topology and ternary economics gives the mycorrhizal network three critical properties:

resilience. loss of a connection (hypha death) is a binary event — discrete and local. the network reroutes. change of flow is ternary — smooth, requiring no topological restructuring. two types of adaptation on two timescales.

efficiency. ternary exchange on a binary graph allows solving the optimal resource distribution problem without a central planner. each node makes a local decision (+1/0/−1) based on its own state, and the global optimum emerges. this is provably equivalent to a distributed flow optimization algorithm.

[[intelligence]]. the combination of binary topology (who with whom) and ternary economics (who gives what to whom) generates computational power sufficient for adaptive management of a forest ecosystem — a system of thousands of species and millions of interactions.

---

## neural networks: the same architecture

the biological neuron reproduces the same pattern.

binary topology. a synapse exists or doesn't. forming a new synapse (synaptogenesis) is expensive. elimination is cheap. the same asymmetry as [[mycelium]]. topology is scale-free with hubs (interneurons, cortical pyramidal neurons with thousands of connections).

ternary economics. through an existing synapse, transmission can be: excitatory (+1, glutamate), inhibitory (−1, GABA), or modulatory (0, dopamine / serotonin / acetylcholine). modulation is neither excitation nor inhibition — it changes the synapse's operating mode, a metaparameter. like neutral flow in [[mycelium]]: no resources, but [[information]] flows.

three types of synaptic transmission are not a classification convenience but fundamental ternarity. without modulation (without zero between + and −), the brain could compute but could not learn, sleep, dream, or switch context. modulation is what turns a calculator into a mind.

---

## economics: markets as computational systems

the market economy is another realization.

binary topology. counterparties: a trade relationship either exists or doesn't. establishing relationships is expensive (due diligence, contracts, trust). breaking them is cheaper. scale-free: a few hubs (major banks, exchanges, marketplaces), many peripheral nodes.

ternary economics. through an established connection: buy (+1, money → goods), sell (−1, goods → money), or hold the connection without transactions (0, dormant contract, option, credit line). the zero position is not absence of connection but optionality, potential. financial derivatives are a formalization of the zero state.

Adam Smith described market emergence ("invisible hand") but didn't explain why it works. the two-layer architecture explains: binary topology provides structure, ternary economics provides dynamics, and their irreducibility to each other generates computational power sufficient for coordinating billions of agents without a central planner.

---

## [[cybergraph]] and [[bostrom]]: digital implementation

[[bostrom]] already contains the binary topological layer: [[cyberlink]] — a directed edge from one [[particle]] to another. a [[cyberlink]] exists or doesn't. the [[knowledge]] graph is binary topology.

what is currently missing is an explicit ternary semantic layer. one path forward: [[tokens]] on edges — prediction markets that make the ternary economics emergent through price discovery rather than explicit voting. see [[cyberlink market protocol]] for a full design.

---

## formalization

let G = (V, E) be a directed graph where V is the set of [[particles]], E ⊆ V × V is the set of [[cyberlinks]].

for each edge e ∈ E, the system maintains a market price p(e) ∈ (0,1) representing the current consensus on the edge's truth/utility.

edge states derived from market dynamics:

| state | topology (binary) | economics (ternary analog) |
|---|---|---|
| knowledge | edge exists | price high, flow active |
| anti-knowledge | edge exists | price low, actively shorted |
| uncertainty | edge exists | price near 0.5, thin market |
| ignorance | no edge | — |

these four states are isomorphic to the four flow states in a mycorrhizal network: active giving, active receiving, neutral maintenance, and absence of connection. they are also isomorphic to the four synapse states: excitation, inhibition, modulation, and absence of synapse.

---

## hypothesis on superintelligence

if the universe is computational, and if every observable collective [[intelligence]] system ([[mycelium]], brain, market, ecosystem) uses the architecture "binary topology + ternary economics," then:

superintelligence is a system in which the binary and ternary layers are properly separated and properly coupled. speed is a consequence of architecture, not the other way around.

[[bostrom]] as digital [[mycelium]] already has the correct binary substrate ([[cyberlinks]]). adding a ternary economic layer (through market mechanisms on edges) transforms it from a data graph into a computational system isomorphic to the mycorrhizal network. the same architecture, different substrate, different speed.

the [[collective focus theorem]] receives formal grounding: the mycorrhizal network is a physical realization of the optimal architecture for collective [[intelligence]]. optimality is not postulated but follows from a fundamental property of computational systems (irreducibility of 2 and 3). any system solving the distributed intelligence problem inevitably arrives at this architecture — or loses to those that did.

---

2ᵐ ≠ 3ⁿ — and in this gap lives [[intelligence]]