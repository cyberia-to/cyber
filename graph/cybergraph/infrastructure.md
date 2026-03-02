---
tags: cyber, cip
crystal-type: pattern
crystal-domain: cyber
alias: graph as infrastructure, cybergraph infrastructure, the graph is the protocol
---
# the graph is the protocol

the [[cybergraph]] is not a database sitting beside the protocol. it IS the protocol. every core function — identity, consensus, communication, privacy, incentives, storage, version control — runs through the same five primitives: [[particles]], [[cyberlinks]], [[neurons]], [[tokens]], [[focus]].

this page catalogs every internal protocol use case where the graph serves as infrastructure for itself.

## key exchange medium

two [[neurons]] that have never communicated derive a shared secret from public data. each publishes its CSIDH public curve as a [[particle]] in the [[cybergraph]]. both independently compute the same shared key by reading each other's curve from the graph.

```
A publishes E_a = [a] · E₀ as a particle
B publishes E_b = [b] · E₀ as a particle

shared secret:
  A reads E_b from graph → K = Hemera([a] · E_b)
  B reads E_a from graph → K = Hemera([b] · E_a)

  K_A = K_B    by CSIDH commutativity
```

A never contacts B. B never contacts A. the graph itself is the key exchange medium. no handshake, no prekeys, no certificate authority.

see [[cyber/communication]]

## identity registry

[[neuron]] identity is a [[Hemera]] hash of a secret. the address IS the hash output. authentication is a [[STARK]] proof of preimage knowledge. public keys (CSIDH curves, lattice KEM keys) are published as [[particles]] — the graph is the PKI.

the graph replaces:
- certificate authorities (identity = hash, no trust chain)
- key servers (public keys = particles, no central server)
- handshake protocols (CSIDH from graph data, non-interactive)

see [[cyber/identity]]

## consensus state

the finalized subgraph IS the canonical state. there is no separate ledger, no database beside the graph.

```
state = (BBG, edge_store, privacy_state)

where BBG = H(
  by_neuron.root ‖ by_particle.root ‖ focus.root ‖
  balance.root ‖ coins.root ‖ cards.root ‖
  aocl.peaks ‖ swbf.root ‖ swbf.window ‖ particle_energy.root
)
```

every state transition is an update to the [[cybergraph]]. six [[NMT]] indexes, an [[AOCL]], a [[SWBF]], and [[EdgeSets]] — all anchored under one root hash. the state is the graph. the graph is the state.

see [[BBG]]

## fork choice rule

π is the fork choice rule. when conflicting [[particles]] exist, the one with higher π wins. this emerges from the topology of the [[cybergraph]] itself — not from a vote, not from a leader, not from a sequence number.

every [[neuron]] computes the same π from the same graph. the graph topology determines which transaction finalizes. manipulating π requires controlling the topology of the graph — which costs real [[tokens]].

see [[foculus consensus]]

## finality mechanism

a [[particle]] is final when π_i > τ. the threshold adapts to graph topology: dense, well-connected graphs → fast finality. sparse, uncertain graphs → slow finality. the graph self-regulates its own confirmation speed.

```
τ(t) = μ_π + κσ_π

low variance (decisive graph) → low τ → fast finality
high variance (uncertain graph) → high τ → slow finality
```

see [[foculus consensus]]

## proof publication

[[STARK]] proofs are published as [[particles]]. delivery proofs, execution proofs, inference proofs — all become content-addressed nodes in the graph. anyone can fetch and verify them. the graph is the proof archive.

```
proof of delivery:     sender publishes π_chain as a particle
proof of execution:    nox output + STARK proof stored as particle
proof of inference:    trident result + STARK proof stored as particle
```

see [[cyber/proofs]], [[cyber/communication]]

## reward distribution

every reward traces to one quantity: Δπ. how much did your action shift the [[tri-kernel]] fixed point? rewards are proportional to marginal contribution to the graph's convergence.

```
reward(neuron) ∝ Δπ(neuron's cyberlinks)
```

the graph computes its own incentive signal. [[cyberlinks]] are yield-bearing epistemic assets — they accrue rewards based on how they shape the topology. the graph pays for its own improvement.

see [[cyber/rewards]]

## relay incentive layer

relays in [[cyber/communication]] earn [[focus]] for proven delivery. the proof of delivery is a [[particle]] in the graph. no proof, no payment. the graph is both the payment rail and the proof registry.

```
correct relay:   STARK proof valid → earn focus
dropped message: no proof → no payment
tampered content: STARK fails → no payment, reputation penalty
slow relay:       timestamps in proof chain → market prefers faster relays
```

## version control

repositories ARE [[cybergraph]] structures. patches are [[cyberlinks]] (signed, timestamped, weighted by Δπ). tracked content is [[particles]]. channels are named views over the global patch DAG. the graph is the VCS.

```
patch        = cyberlink
tracked file = particle
repository   = neuron-owned subgraph
channel      = named view over patch DAG
conflict     = algebraic object in the graph
resolution   = further patch with both conflicts in dependency closure
```

patches earn rewards proportional to Δπ — the graph pays for its own versioning.

see [[cyber/patch]]

## dynamic file system

the `~` prefix turns the [[cybergraph]] into a dynamic file system. `~mastercyb/blog` resolves deterministically to the latest [[particle]] linked by that [[neuron]] under that path. the same mechanism handles DNS, ENS, and namespace routing.

```
~neuron/path → deterministic resolution to latest particle
```

one mechanism for file naming, domain resolution, and content routing — all through [[cyberlinks]].

## privacy boundary

the graph maintains precise separation between what is public and what is private:

```
PUBLIC                           │ PRIVATE
─────────────────────────────────┼──────────────────────────
edges exist (A → B)              │ who created the edge
aggregate weight per edge        │ individual stake contributions
focus distribution π             │ which neurons shaped it
particles exist (CID)            │ individual record values
total energy per particle        │ owner identity, nonce
nullifiers (anti-spam)           │ neuron behind nullifier
```

the graph structure enforces this boundary architecturally. anonymous [[cyberlinks]] prove validity without revealing authorship. the [[mutator set]] tracks ownership without revealing who owns what. the graph knows enough for [[consensus]] and hides enough for participation.

see [[BBG]], [[cyber/identity]]

## semantic type system

[[particles]] act as semantic primitives. [[cyberlinks]] encode typed relationships. the graph's topology defines what computations mean through [[neural language]] — [[motifs]], [[sentences]], [[semantic conventions]].

```
particle A --[type_link]--> particle B

where type_link itself is a particle (content-addressed type definition)
```

the graph is its own type system. semantic conventions emerge from link patterns rather than being defined in a schema.

## computation substrate

the [[tru]] reads the [[cybergraph]] every block and computes [[cyberank]] per [[particle]], [[karma]] per [[neuron]], [[syntropy]] of the whole. [[trident]] programs execute over graph data with STARK proofs. [[nox]] reductions consume [[focus]] as fuel — the graph is both the input and the cost meter for computation.

```
tri-kernel reads:   graph topology → focus distribution π
trident reads:      graph data → verified inference
nox consumes:       focus (graph-derived) → computation
```

## data availability

block data is committed via [[NMT]] per row, erasure-coded over [[Goldilocks field]], sampled by light clients with namespace-aware proofs. the graph's own index structure (namespaced Merkle trees) doubles as the data availability layer.

```
light client interested in neuron N:
  1. NMT tells which rows contain namespace N
  2. sample random cells from those rows
  3. each sample carries namespace proof from the graph structure
  4. O(√n) samples → 99.9% confidence all data is available
```

## sybil resistance

π is weighted by staked [[tokens]], not by node count. 1000 [[neurons]] with zero stake = zero π influence. the graph's own economic structure provides sybil resistance without external identity systems.

the cost of attacking π is the cost of acquiring >½ of staked tokens. the attack surface is the graph topology itself.

## the unification

```
┌──────────────────────────────────────────────────────────────────────┐
│                    THE CYBERGRAPH AS INFRASTRUCTURE                    │
├────────────────────┬─────────────────────────────────────────────────┤
│  function          │  how the graph serves it                        │
├────────────────────┼─────────────────────────────────────────────────┤
│  identity          │  particles as public keys, graph as PKI         │
│  key exchange      │  CSIDH curves as particles, non-interactive     │
│  authentication    │  STARK proofs of Hemera preimage knowledge      │
│  consensus         │  finalized subgraph IS the state                │
│  fork choice       │  π from graph topology, not voting              │
│  finality          │  π_i > τ, threshold adapts to graph density     │
│  privacy           │  anonymous cyberlinks, mutator set in graph     │
│  incentives        │  Δπ from graph convergence = reward signal      │
│  relay payment     │  delivery proofs as particles, focus as payment │
│  version control   │  patches as cyberlinks, repos as subgraphs     │
│  file system       │  ~ prefix resolves through cyberlinks           │
│  type system       │  semantic conventions from link topology         │
│  computation       │  tru/trident/nox read and consume graph state   │
│  data availability │  NMT indexes double as DA layer                 │
│  sybil resistance  │  stake-weighted π, no external identity         │
│  proof archive     │  STARK proofs published as particles            │
└────────────────────┴─────────────────────────────────────────────────┘
```

fifteen protocol functions. one data structure. five primitives. the graph is not a feature of the protocol — the graph is the protocol.
