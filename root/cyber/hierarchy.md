---
tags: cyber, core, cip
crystal-type: entity
crystal-domain: cyber
crystal-size: article
alias: cyber hierarchy, sharding, scaling, n-dimensional sharding
status: draft
stake: 80000000000000000
---
# cyber hierarchy

how the [[cybergraph]] scales to 10^15 [[particles]] and 10^10 [[neurons]] — not by designing shards in advance, but by reading the natural hierarchy from the [[tri-kernel]]'s own output

---

## the insight

the [[heat]] kernel H_τ = exp(-τL) is a multi-scale clustering operator. at different temperatures τ it reveals community structure at different resolutions:

| Scale | Temperature | What it reveals | Shard level |
|-------|-------------|----------------|-------------|
| fine | τ₁ (small) | local neighborhoods — tightly linked [[particles]] | cell |
| medium | τ₂ | semantic regions — topic areas with high mutual [[focus]] flow | zone |
| coarse | τ₃ (large) | continents of meaning — the broadest clusters | domain |

the hierarchy IS the [[heat]] kernel spectrum. no administrator assigns shards. the [[tri-kernel]] computes them as a side effect of computing [[focus]]

this is the same operator that provides the third force in the [[tri-kernel]] — adaptation via multi-scale smoothing. the force that adapts [[focus]] to context also reveals where the natural boundaries are

---

## four dimensions of locality

sharding along a single dimension (hash range, chain ID) creates arbitrary boundaries that cut through natural clusters. the [[cybergraph]] has four dimensions of locality — [[particles]] that are close in any dimension should share a shard

### semantic locality

[[particles]] with high mutual [[focus]] flow — many [[cyberlinks]] between them, strong [[axon]] weights — form semantic clusters. the [[tri-kernel]] reveals these through the [[heat]] kernel's community detection

mechanism: periodically compute H_τ₂ and extract connected components above a threshold. these are the semantic zones. [[particles]] within a zone have high internal connectivity and low external connectivity — the natural shard boundary

### economic locality

[[neurons]] who transact frequently form economic clusters. UTXO movement patterns reveal who sends to whom. co-locate frequent transactors in the same shard to minimize cross-shard transfers

mechanism: track UTXO flow graph (sender → receiver) over an epoch. apply spectral clustering to the flow graph. [[neurons]] in the same economic cluster share a shard. economic locality often correlates with semantic locality (you transact with people interested in similar things) but not always

### geographic locality

latency matters for interactive use. [[neurons]] in the same physical region want low-latency access to their neighborhood. [[location proof]] provides this dimension

mechanism: [[neurons]] with [[location proof]] are grouped by geographic proximity. validators in a region preferentially serve that region's shard. cross-region communication uses the relay layer with higher latency budget

### temporal locality

hot state (recent, active) vs cold state (archival). the [[forgetting]] mechanism already separates hot/cold tiers. shards can specialize by time horizon

mechanism: [[particles]] with recent [[cyberlinks]] and active UTXO traffic live in the hot tier. [[particles]] that pass the archival sweep (low stake, low price, no traffic for N epochs) move to cold tier. cold tier is read-only for the [[tri-kernel]] — excluded from active computation but queryable

---

## the hierarchy levels

```
Level 0: particles          — individual content-addressed nodes
Level 1: cells              — local neighborhoods (τ₁, ~100-1000 particles)
Level 2: zones              — semantic regions (τ₂, ~10K-100K particles)
Level 3: domains            — continents of meaning (τ₃, ~1M-10M particles)
Level 4: the cybergraph     — global state (all particles)
```

each level is a shard boundary. computation at level N sees full state within its shard and aggregated state from neighboring shards at level N+1

### routing

a [[particle]]'s address is its [[Hemera]] hash — this never changes. but the shard it lives on changes as the graph evolves. a lightweight routing index maps hash → current shard:

```
routing(particle_hash) → (domain, zone, cell)
```

this index is itself a knowledge graph problem — maintained by the [[tri-kernel]] as part of the slow-timescale shard rebalancing

---

## sharding the tri-kernel

the [[tri-kernel]] has a locality radius: h = O(log(1/ε)) hops. this means each [[particle]]'s [[focus]] depends only on its h-hop neighborhood. sharding exploits this:

within a cell (level 1): the [[tri-kernel]] runs at full resolution. every [[cyberlink]], every [[axon]] weight, every market price is visible. convergence is fast because the graph is small

within a zone (level 2): cells communicate aggregated focus vectors. each cell exports its boundary [[particles]]' focus values to neighboring cells. the [[tri-kernel]] treats cross-cell focus as boundary conditions

across zones (level 3): zones exchange coarse-grained focus summaries. the global [[focus]] distribution is approximated by composing zone-level summaries. the error is bounded by the locality theorem:

$$\|\pi^*_{\text{sharded}} - \pi^*_{\text{global}}\| \leq C \cdot e^{-\alpha h}$$

where h is the communication horizon and α depends on the spectral gap. more communication → smaller error → closer to global [[focus]]

---

## sharding UTXOs

the hardest scaling challenge. per-[[particle]] UTXOs will have massive transaction volume. private UTXOs make it harder — you cannot see which shard a UTXO belongs to

### public UTXOs

a UTXO is committed to a [[neuron]]. the [[neuron]]'s shard (determined by their primary cell) owns the UTXO. transfers within a cell are cheap — local state update, local proof

cross-cell transfers: the source cell produces a proof that the UTXO was validly spent (nullifier published, balance sufficient). the destination cell verifies the proof and creates the new UTXO. the proof is O(log |state|) via [[zheng|STARK]]

cross-zone transfers: same mechanism but the proof relay crosses zone boundaries. higher latency, higher proof cost. the economic dimension helps — if Alice and Bob transact frequently, they end up in the same economic cluster → same cell → cheap transfers

### private UTXOs

each cell maintains its own nullifier set (part of the [[mutator set]]: AOCL + SWBF). spending a private UTXO publishes a nullifier to the cell's set

within-cell private transfer: the [[neuron]] proves in zero knowledge that a valid UTXO exists in this cell's accumulator, publishes a nullifier, and creates a new commitment in the same cell. standard ZK transfer — no cross-shard coordination

cross-cell private transfer: two proofs required:
1. source cell: ZK proof that a valid UTXO was spent (nullifier published to source cell's set)
2. destination cell: the commitment is inserted into the destination cell's accumulator

the privacy is maintained — neither cell learns which UTXO was spent or what the new commitment contains. the cross-cell relay carries only the proofs, not the values

### the nullifier scaling problem

nullifier sets grow monotonically — every spent UTXO adds a nullifier forever. at 10^10 [[neurons]] with multiple UTXOs each, the global nullifier set would be enormous

sharded nullifier sets solve this: each cell's nullifier set contains only nullifiers for UTXOs that were in that cell. the total size is the same, but no single node holds it all. double-spend prevention is local within a cell, and cross-cell transfers use proof relay

---

## the self-referential property

the [[tri-kernel]]'s output informs the sharding (clusters come from the [[heat]] kernel). the sharding constrains the [[tri-kernel]]'s input (each shard sees its local graph). this is another fixed-point problem

convergence is guaranteed by two-timescale separation:

| Timescale | What happens | Frequency |
|-----------|-------------|-----------|
| fast (per block) | [[focus]] flow within shards, UTXO processing | every block |
| medium (per epoch) | cross-shard focus synchronization, boundary updates | every ~100 blocks |
| slow (per era) | shard rebalancing — cells merge/split based on load and connectivity | every ~10K blocks |

the fast timescale sees fixed shard boundaries. the slow timescale adjusts boundaries based on accumulated statistics. because the fast dynamics converge much faster than boundaries change, the system is stable

### shard rebalancing

when a cell grows too large (too many [[particles]], too much UTXO traffic): split it along the [[heat]] kernel's finest-scale cluster boundary

when two cells have become tightly coupled (high cross-cell [[focus]] flow, many cross-cell transfers): merge them

when a zone's internal connectivity drops below threshold (the [[heat]] kernel shows it's really two zones): split the zone

these operations require state migration — [[particles]] and UTXOs move between cells. the cost is amortized over the slow timescale

---

## comparison

| System | Hierarchy | Static/Dynamic | Dimensions |
|--------|-----------|---------------|------------|
| IP (Internet) | 4-tier (network/subnet/host/port) | semi-static (ISP assigns) | 1 (topology) |
| Urbit | 4-tier (galaxy/star/planet/moon) | static (burned at genesis) | 1 (identity) |
| Ethereum 2.0 | 2-tier (beacon/shards) | static (64 shards) | 1 (hash range) |
| Cosmos | flat (sovereign chains + IBC) | static (per chain) | 0 (no hierarchy) |
| [[cyber]] | N-tier (cell/zone/domain) | dynamic (computed by [[tri-kernel]]) | 4 (semantic, economic, geographic, temporal) |

the key difference: every other system designs the hierarchy. [[cyber]] computes it. the [[tri-kernel]] is simultaneously the ranking engine, the sharding oracle, and the routing advisor. one computation serves all three purposes

---

## open questions

shard boundary latency: how many blocks of cross-shard latency is acceptable before UX degrades? this determines the minimum cell size (smaller cells → more cross-shard traffic)

privacy and routing: if a [[neuron]]'s shard assignment is public, it leaks information about their [[cyberlink]] patterns. can shard assignment itself be private?

incentive alignment: validators specialize in cells. what prevents a validator from refusing to serve a low-value cell? the economic dimension must ensure that serving any cell is profitable

cold-to-hot reactivation: when an archived [[particle]] gets new [[cyberlinks]], it must rejoin a hot cell. which cell? the semantic dimension may have shifted since it was archived

initial bootstrapping: before the graph has enough structure for the [[heat]] kernel to reveal clusters, what sharding strategy applies? likely: single shard (like bostrom now) until the graph crosses the phase transition threshold $|P^*| \sim \rho^2$

see [[cyber/architecture]] for the five-primitive resource model. see [[tri-kernel architecture]] for why the [[heat]] kernel survives the locality filter. see [[cyber/state]] for the BBG world state structure. see [[cyber/network]] for the narrowcast relay protocol. see [[forgetting]] for the hot/cold tier separation
