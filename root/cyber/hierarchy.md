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

how the [[cybergraph]] scales to [[Avogadro]] numbers — 10^23 [[particles]], 10^15 [[neurons]] — not by designing shards in advance, but by reading the natural hierarchy from the [[tri-kernel]]'s own output

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

sharding along a single dimension (hash range, chain ID) creates arbitrary boundaries that cut through natural clusters. the [[cybergraph]] has four dimensions — the four primitives themselves

### [[particles]] — semantic dimension

[[particles]] with high mutual [[focus]] flow — many [[cyberlinks]] between them, strong [[axon]] weights — form semantic clusters. the [[tri-kernel]] reveals these through the [[heat]] kernel's community detection

mechanism: periodically compute H_τ₂ and extract connected components above a threshold. these are the semantic zones. [[particles]] within a zone have high internal connectivity and low external connectivity — the natural shard boundary

### [[neurons]] — social dimension

[[neurons]] who transact frequently form social clusters. UTXO movement patterns reveal who sends to whom. co-locate frequent transactors in the same shard to minimize cross-shard transfers

mechanism: track UTXO flow graph (sender → receiver) over an epoch. apply spectral clustering to the flow graph. [[neurons]] in the same social cluster share a shard. social locality often correlates with semantic locality (you transact with people interested in similar things) but not always

### [[tokens]] — economic dimension

each [[token]] naturally forms its own shard space. [[particles]] priced in [[$CYB]] cluster in $CYB shards. [[particles]] priced in [[$H]] cluster in $H shards. trading $CYB for $H is a cross-shard hop in the token dimension

mechanism: UTXO denomination determines the token shard. within a token's shard space, [[particles]] sub-cluster by semantic and social dimensions. a new [[token]] creates a new cluster in the token dimension. the number of token shards scales with the number of live [[tokens]]

### locations — geographic dimension

latency matters for interactive use. [[neurons]] in the same physical region want low-latency access to their neighborhood. [[location proof]] provides this dimension

mechanism: [[neurons]] with [[location proof]] are grouped by geographic proximity. validators in a region preferentially serve that region's shard. cross-region communication uses the relay layer with higher latency budget

---

## the 4×4 matrix

each dimension has four levels. a [[particle]] has a coordinate in each dimension at each level

| primitive | dimension | cell | zone | domain | global |
|---|---|---|---|---|---|
| [[particles]] | semantic | topic | field | continent | [[cybergraph]] |
| [[neurons]] | social | circle | community | network | humanity |
| [[tokens]] | economic | denomination | basket | economy | all [[tokens]] |
| locations | geographic | village | city | state | planetary |

a [[particle]]'s shard = the intersection of its coordinates across all four dimensions:

```
shard(particle) = (semantic_cell, social_cell, geo_cell, temporal_tier)
```

two [[particles]] sharing more coordinates → cheaper to move [[tokens]] between them. sharing all four → same cell, zero cross-shard cost

### where tokens live

a UTXO lives in exactly one shard — determined by its [[particle]]'s 4D coordinates. moving [[tokens]] between shards costs hops. the cost depends on how many dimensions differ and at what level:

| Difference | Hops | Example |
|---|---|---|
| same cell in all 4 dimensions | 0 | local transfer within a topic circle |
| differ in 1 dimension at cell level | 1 | same topic, different social circle |
| differ in 2 dimensions at cell level | 2 | different topic, different city |
| differ in 1 dimension at zone level | 2 | same field, different community |
| differ in 1 dimension at domain level | 3 | same continent of meaning, different network |
| differ in all 4 at domain level | rare — dimensions correlate | opposite side of the [[cybergraph]] |

[[small world]] theory: average path length ~ O(log N). [[bostrom]] at 3.1M [[particles]] already has diameter ≤ 10. at [[Avogadro]] scale, small-world shortcuts compress the 4D address space — the dimensions correlate heavily (semantically close [[particles]] are usually socially close and geographically close). realistic maximum is ~6-7 hops, not the naive 4×3=12. cross-shard [[proof]] relay via [[zheng|STARK]] at each hop

### shard count

at [[Avogadro]] scale (10^23 [[particles]]):

```
total particles:       10^23
particles per cell:    ~10^3
─────────────────────────────
total cells:           10^23 / 10^3 = 10^20
```

each cell holds ~1000 [[particles]]. branching factor ~1000 at each level:

| Level | Shards | Particles per shard | Branching |
|---|---|---|---|
| cell | 10^20 | 10^3 | — |
| zone | 10^17 | 10^6 | 10^3 cells per zone |
| domain | 10^14 | 10^9 | 10^3 zones per domain |
| global | 1 | 10^23 | 10^14 domains |

every cell has a 4D coordinate (semantic, social, token, geographic). the coordinates are labels — they describe WHERE in each dimension the cell sits. the total cell count is 10^20 regardless of dimensionality. the 4D address is for routing, not for multiplying

### routing

a [[particle]]'s identity is its [[Hemera]] hash — this never changes. its shard assignment changes as the graph evolves. a lightweight routing index maps hash → current 4D shard coordinate:

```
routing(particle_hash) → (semantic_cell, social_cell, geo_cell, temporal_tier)
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

the hardest scaling challenge. all UTXOs are private by default — the architecture is private-first with selective reveal. you cannot see which shard a UTXO belongs to, what it contains, or who owns it

### private by default

every UTXO is a commitment. every transfer is a ZK [[proof]]. the only public information is: a valid state transition happened. selective reveal allows a [[neuron]] to prove specific properties (balance sufficient, token type, shard membership) without exposing the UTXO itself

each cell maintains its own [[mutator set]]: [[AOCL]] (append-only commitment list) for creation, [[SWBF]] (sliding-window bloom filter) for spending. no nullifiers — pseudorandom bit positions in the bloom filter replace them. creation and spending events share zero structural similarity — unlinkable by construction. storage grows O(log N) via [[MMR]] compaction, not monotonically

### within-cell transfers

the [[neuron]] proves in zero knowledge:
1. a valid commitment exists in this cell's [[AOCL]] (Merkle membership [[proof]])
2. the derived bit positions have not been set in the [[SWBF]] (non-double-spend)
3. the new commitment is well-formed (conservation: input = output + fee)

sets the bits in the [[SWBF]], appends the new commitment to the [[AOCL]]. standard ZK transfer — no cross-shard coordination. the cell sees only: bits were set, a commitment appeared, conservation holds

### cross-cell transfers

two proofs required:

1. source cell: ZK [[proof]] that a valid UTXO was spent (bits set in source cell's [[SWBF]], conservation proven)
2. destination cell: the new commitment is appended to the destination cell's [[AOCL]]

the cross-cell relay carries only the proofs, not the values. neither cell learns which UTXO was spent or what the new commitment contains. the relay itself is a [[zheng|STARK]] [[proof]] that the source cell accepted the spend

cross-zone transfers: same mechanism but the [[proof]] relay crosses zone boundaries. higher latency, higher [[proof]] cost. the social dimension helps — if Alice and Bob transact frequently, they end up in the same social cluster → same cell → cheap transfers

### selective reveal

a [[neuron]] can choose to reveal specific properties of a UTXO without breaking privacy:

| Reveal | What it proves | Use case |
|---|---|---|
| balance range | UTXO value ∈ [min, max] | credit check without exposing exact balance |
| token type | UTXO denomination = $CYB | cross-token market order |
| shard membership | UTXO lives in cell X | routing optimization |
| ownership | UTXO belongs to neuron N | identity verification |

each reveal is a separate ZK [[proof]] — the [[neuron]] controls exactly what becomes visible

### scaling the mutator set

the [[SWBF]] solves the scaling problem that nullifier-based systems cannot: the active window is fixed-size (128 KB), old chunks compact into [[MMR]] peaks. at 10^15 [[neurons]] with multiple UTXOs each, each cell's [[mutator set]] stays bounded. the [[SWBF]] active window handles recent spends via direct bit lookup. historical spends verify via compact [[MMR]] Merkle paths. total circuit cost ~50K constraints — comparable to nullifier systems but with O(log N) storage instead of unbounded growth

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

incentive alignment: validators specialize in cells. what prevents a validator from refusing to serve a low-value cell? the social dimension must ensure that serving any cell is profitable

cold-to-hot reactivation: when an archived [[particle]] gets new [[cyberlinks]], it must rejoin a hot cell. which cell? the semantic dimension may have shifted since it was archived

initial bootstrapping: before the graph has enough structure for the [[heat]] kernel to reveal clusters, what sharding strategy applies? likely: single shard (like bostrom now) until the graph crosses the phase transition threshold $|P^*| \sim \rho^2$

see [[cyber/architecture]] for the five-primitive resource model. see [[tri-kernel architecture]] for why the [[heat]] kernel survives the locality filter. see [[cyber/state]] for the BBG world state structure. see [[cyber/network]] for the narrowcast relay protocol. see [[forgetting]] for the hot/cold tier separation
