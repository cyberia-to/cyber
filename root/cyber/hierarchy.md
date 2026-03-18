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

the [[tri-kernel]] that computes [[focus]] also reveals the natural hierarchy. [[springs]] (graph [[Laplacian]] eigenvectors) define cluster boundaries via spectral decomposition. [[heat]] (exp(-τL) at different temperatures) controls the resolution — which level of the hierarchy you read:

| Scale | Temperature | What it reveals | Shard level |
|-------|-------------|----------------|-------------|
| fine | τ₁ (small) | local neighborhoods — tightly linked [[particles]] | cell |
| medium | τ₂ | semantic regions — topic areas with high mutual [[focus]] flow | zone |
| coarse | τ₃ (large) | continents of meaning — the broadest clusters | domain |

no administrator assigns shards. the [[tri-kernel]] computes them as a side effect of computing [[focus]]. the same operators that rank [[particles]] also partition the graph for scaling

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

at [[Avogadro]] scale — estimated count at each level per dimension:

| primitive | dimension | cell | zone | domain | global |
|---|---|---|---|---|---|
| [[particles]] | semantic | ~10^17 topics | ~10^12 fields | ~10^6 continents | 1 [[cybergraph]] |
| [[neurons]] | social | ~10^10 circles | ~10^7 communities | ~10^4 networks | 1 humanity |
| [[tokens]] | economic | ~10^6 denominations | ~10^4 baskets | ~10^2 economies | 1 token space |
| locations | geographic | ~10^6 villages | ~10^4 cities | ~10^2 states | 1 planet |

a [[particle]]'s shard = the intersection of its coordinates. most of the 4D space is empty — dimensions correlate (semantically close [[particles]] are usually socially close and geographically close). cells exist only where [[particles]] actually cluster

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

all UTXOs are private by default. every UTXO is a commitment. every transfer is a ZK [[proof]]. the only public information is: a valid state transition happened

each cell maintains its own [[mutator set]]: [[AOCL]] for creation, [[SWBF]] for spending. no nullifiers — bit positions in a bloom filter replace them. creation and spending events are unlinkable by construction. storage grows O(log N) via [[MMR]] compaction

within-cell transfers are cheap — local state update, no cross-shard coordination. cross-cell transfers require [[zheng|STARK]] [[proof]] relay between cells. cross-zone transfers relay across zone boundaries — higher latency, higher cost. the social dimension co-locates frequent transactors in the same cell

see [[cyber/state]] for the transfer mechanics. see [[AOCL]] and [[SWBF]] for the [[mutator set]]. see [[cyber/proofs]] for the ZK [[proof]] taxonomy

---

## the self-referential property

the [[tri-kernel]]'s output informs the sharding. the sharding constrains the [[tri-kernel]]'s input (each shard sees its local graph). this is another fixed-point problem

all three operators contribute to community detection:

| Operator | What it reveals | Sharding role |
|---|---|---|
| [[diffusion]] | random walk communities — where probability flows | identifies semantic clusters via flow concentration |
| [[springs]] | Laplacian eigenvectors — structural communities | spectral clustering on the graph [[Laplacian]] — the standard method |
| [[heat]] | multi-scale smoothing — communities at different resolutions | controls the scale: low τ = fine cells, high τ = coarse domains |

[[springs]] provides the eigenvectors that define cluster boundaries. [[heat]] controls the resolution — which level of the hierarchy you're reading. [[diffusion]] reveals the flow patterns that validate the clusters. the three together give robust community detection that no single operator provides alone

convergence is guaranteed by two-timescale separation:

| Timescale | What happens | Frequency |
|-----------|-------------|-----------|
| fast (per block) | [[focus]] flow within shards, UTXO processing | every block |
| medium (per epoch) | cross-shard focus synchronization, boundary updates | every ~100 blocks |
| slow (per era) | shard rebalancing — cells merge/split based on load and connectivity | every ~10K blocks |

the fast timescale sees fixed shard boundaries. the slow timescale adjusts boundaries based on accumulated statistics. because the fast dynamics converge much faster than boundaries change, the system is stable

### shard rebalancing

when a cell grows too large (too many [[particles]], too much UTXO traffic): split it along the [[Laplacian]] eigenvector boundary (spectral bisection via [[springs]])

when two cells have become tightly coupled (high cross-cell [[focus]] flow, many cross-cell transfers): merge them

when a zone's internal connectivity drops below threshold ([[springs]] eigengap shows it is really two zones): split the zone

these operations require state migration — [[particles]] and UTXOs move between cells. the cost is amortized over the slow timescale

---

## comparison

| System | Hierarchy | Static/Dynamic | Dimensions |
|--------|-----------|---------------|------------|
| IP (Internet) | 4-tier (network/subnet/host/port) | semi-static (ISP assigns) | 1 (topology) |
| Urbit | 4-tier (galaxy/star/planet/moon) | static (burned at genesis) | 1 (identity) |
| Ethereum 2.0 | 2-tier (beacon/shards) | static (64 shards) | 1 (hash range) |
| Cosmos | flat (sovereign chains + IBC) | static (per chain) | 0 (no hierarchy) |
| [[cyber]] | 4-tier (cell/zone/domain/global) | dynamic (computed by [[tri-kernel]]) | 4 (semantic, social, economic, geographic) |

address space:

| System | Total addresses |
|---|---|
| IPv4 | 2^32 = 4 × 10^9 |
| Urbit (planets) | 2^32 = 4 × 10^9 |
| Urbit (moons) | 2^64 = 1.8 × 10^19 |
| IPv6 | 2^128 = 3 × 10^38 |
| [[cyber]] | [[Hemera]] = 2^256 ≈ 10^77 (content-addressed, [[Avogadro]] is a rounding error) |

the key difference: every other system designs the hierarchy. [[cyber]] computes it. the [[tri-kernel]] is simultaneously the ranking engine, the sharding oracle, and the routing advisor. one computation serves all three purposes

---

## open questions

shard boundary latency: how many blocks of cross-shard latency is acceptable before UX degrades? this determines the minimum cell size (smaller cells → more cross-shard traffic)

privacy and routing: if a [[neuron]]'s shard assignment is public, it leaks information about their [[cyberlink]] patterns. can shard assignment itself be private?

incentive alignment: validators specialize in cells. what prevents a validator from refusing to serve a low-value cell? the social dimension must ensure that serving any cell is profitable

cold-to-hot reactivation: when an archived [[particle]] gets new [[cyberlinks]], it must rejoin a hot cell. which cell? the semantic dimension may have shifted since it was archived

initial bootstrapping: before the graph has enough structure for the [[heat]] kernel to reveal clusters, what sharding strategy applies? likely: single shard (like bostrom now) until the graph crosses the phase transition threshold $|P^*| \sim \rho^2$

see [[cyber/architecture]] for the five-primitive resource model. see [[tri-kernel architecture]] for why the [[heat]] kernel survives the locality filter. see [[cyber/state]] for the BBG world state structure. see [[cyber/network]] for the narrowcast relay protocol. see [[forgetting]] for the hot/cold tier separation
