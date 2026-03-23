---
tags: cyber, core, cip
crystal-type: entity
crystal-domain: cyber
crystal-size: article
alias: cyber hierarchy, folding, scaling, graph folding
status: draft
stake: 80000000000000000
diffusion: 0.0004669053175446973
springs: 0.0011905728208696943
heat: 0.0009736518396428102
focus: 0.000785354872961842
gravity: 14
density: 2.95
---
# cyber hierarchy

how the [[cybergraph]] scales to [[Avogadro]] numbers — 10^23 [[particles]], 10^15 [[neurons]] — not by designing shards in advance, but by reading the natural hierarchy from the [[tri-kernel]]'s own output

---

## the insight

the [[tri-kernel]] that computes [[focus]] also reveals the natural hierarchy. all three operators contribute:

| Operator | What it reveals | Folding role |
|---|---|---|
| [[springs]] | [[Laplacian]] eigenvectors — structural communities | defines cluster boundaries via spectral decomposition |
| [[heat]] | multi-scale smoothing — communities at different resolutions | controls the scale: low τ = fine cells, high τ = coarse domains |
| [[diffusion]] | random walk communities — where [[probability]] flows | validates clusters via flow concentration |

[[springs]] provides the eigenvectors that define fold lines. [[heat]] controls the resolution — which level of the hierarchy you read. [[diffusion]] reveals the flow patterns that validate the folds. the three together give robust community detection that no single operator provides alone

no administrator assigns structure. the [[tri-kernel]] computes it as a side effect of computing [[focus]]. the same operators that rank [[particles]] also partition the graph for scaling

---

## four dimensions

the [[cybergraph]] has four dimensions — the four primitives themselves. [[particles]] that are close in any dimension should share a cell

### [[particles]] — semantic

[[particles]] with high mutual [[focus]] flow — many [[cyberlinks]] between them, strong [[axon]] weights — form semantic clusters. the [[tri-kernel]] reveals these through spectral decomposition ([[springs]]) and multi-scale smoothing ([[heat]])

### [[neurons]] — social

[[neurons]] who transact frequently form social clusters. UTXO movement patterns reveal who sends to whom. co-locate frequent transactors in the same cell to minimize cross-cell transfers. social locality often correlates with semantic locality but not always

### [[tokens]] — economic

each [[token]] naturally forms its own cluster. [[particles]] priced in [[$CYB]] cluster in $CYB cells. trading $CYB for $H is a cross-cell hop in the token dimension. a new [[token]] creates a new cluster. the number of token cells scales with the number of live [[tokens]]

### locations — geographic

latency matters for interactive use. [[neurons]] in the same physical region want low-latency access to their neighborhood. [[location proof]] provides this dimension. validators in a region preferentially serve that region's cells

---

## the 4×4 matrix

each dimension has four scales. a [[particle]] has a coordinate in each dimension at each scale

| primitive | dimension | [[cell]] | zone | domain | global |
|---|---|---|---|---|---|
| [[particles]] | semantic | topic | field | continent | [[cybergraph]] |
| [[neurons]] | social | circle | community | network | humanity |
| [[tokens]] | economic | denomination | basket | economy | all [[tokens]] |
| locations | geographic | village | city | state | planetary |

[[cells]] are the base operational level — they hold state, process transactions, run the [[tri-kernel]]. zones, domains, and global emerge from the [[cell]] topology at different [[heat]] kernel temperatures. they are not passive observations — each level holds stakes and coordinates [[consensus]]. validators stake at the level they serve

a [[particle]]'s [[cell]] = the intersection of its coordinates across all four dimensions. two [[particles]] sharing more coordinates → cheaper to move [[tokens]] between them. sharing all four → same [[cell]], zero cross-cell cost

```
cell(particle) = (semantic_cell, social_cell, token_cell, geo_cell)
```

---

## the root cell

the root cell is where all four dimensions meet at their global level — the origin (0,0,0,0)

it holds two things:

1. the [[crystal]] — the 5,040 [[particle]] seed that defines the foundational ontology. these [[particles]] are maximally general, referenced by everything, naturally highest [[focus]]

2. the routing table — maps [[particle]] hash → domain. not cell-level routing — that is each domain's job

```
root    → knows domains
domain  → knows zones
zone    → knows cells
cell    → knows particles
```

four hops to find any [[particle]] among 10^23. the root cell is the first hop

before the graph has enough structure to fold, everything IS the root cell. [[bostrom]] right now is one root cell. as the graph crosses the phase transition threshold $|P^*| \sim \rho^2$, [[cells]] start splitting — but the root cell persists as the coordination point

no [[cell]] appears from nowhere. every [[cell]] descends from the root cell through a chain of splits. the [[hierarchy]] is a living tree that grows by division — the same mechanism that builds biological organisms from a single fertilized cell. see [[cyber/cell]] for the split/merge mechanics

---

## two information flows

### subjective (neuron-driven)

[[tokens]], [[cyberlinks]], [[attention]] allocations. [[neurons]] choose where to move these. a [[neuron]] decides to send [[$CYB]] from cell A to cell B — that is a subjective decision, costs a [[proof]] relay

direction: horizontal and downward. [[neurons]] push information into cells

### objective (cell-computed)

[[focus]] aggregations, [[rank]] summaries, community structure, routing updates. no [[neuron]] moves these — each cell computes them deterministically from its local state and propagates upward

direction: upward only. cells push truth to zones, zones to domains, domains to root

```
root     ← receives domain summaries (objective)
domain   ← receives zone summaries (objective)
zone     ← receives cell summaries (objective)
cell     ← receives cyberlinks, tokens (subjective from neurons)
         → computes local focus, propagates upward (objective)
```

a [[neuron]] cannot push a fake [[rank]] summary upward — the cell computes it deterministically from the [[tri-kernel]] and proves it via [[zheng|STARK]]. the [[proof]] propagates with the summary. each level verifies the level below

the subjective layer (what [[neurons]] want) and the objective layer (what the graph computes) flow in different directions through the same structure. [[tokens]] flow wherever [[neurons]] send them. truth flows wherever the math says it goes

---

## hop cost

moving [[tokens]] between cells costs hops. the cost depends on how many dimensions differ and at what level:

| Difference | Hops | Example |
|---|---|---|
| same cell in all 4 dimensions | 0 | local transfer within a topic circle |
| differ in 1 dimension at cell level | 1 | same topic, different social circle |
| differ in 2 dimensions at cell level | 2 | different topic, different city |
| differ in 1 dimension at zone level | 2 | same field, different community |
| differ in 1 dimension at domain level | 3 | same continent of meaning, different network |

[[small world]] theory: average path length ~ O(log N). [[bostrom]] at 3.1M [[particles]] already has diameter ≤ 10. at [[Avogadro]] scale, small-world shortcuts compress the 4D address space — the dimensions correlate heavily. realistic maximum is ~6-7 hops. cross-cell [[proof]] relay via [[zheng|STARK]] at each hop

---

## UTXOs

all UTXOs are private by default. every UTXO is a commitment. every transfer is a ZK [[proof]]. the only public information is: a valid state transition happened

each cell maintains its own [[mutator set]]: [[AOCL]] for creation, [[SWBF]] for spending. no nullifiers — bit positions in a bloom filter replace them. creation and spending events are unlinkable by construction. storage grows O(log N) via [[MMR]] compaction

within-cell transfers are cheap — local state update, no cross-cell coordination. cross-cell transfers require [[zheng|STARK]] [[proof]] relay. the social dimension co-locates frequent transactors in the same cell

see [[cyber/state]] for transfer mechanics. see [[AOCL]] and [[SWBF]] for the [[mutator set]]

---

## folding the tri-kernel

the [[tri-kernel]] has a locality radius: h = O(log(1/ε)) hops. each [[particle]]'s [[focus]] depends only on its h-hop neighborhood

within a cell: the [[tri-kernel]] runs at full resolution. every [[cyberlink]], every [[axon]] weight, every market price is visible

within a zone: cells communicate aggregated [[focus]] vectors. each cell exports its boundary [[particles]]' [[focus]] values to neighboring cells

across zones: zones exchange coarse-grained [[focus]] summaries. the error is bounded:

$$\|\pi^*_{\text{folded}} - \pi^*_{\text{global}}\| \leq C \cdot e^{-\alpha h}$$

more communication → smaller error → closer to global [[focus]]

---

## timescales

| Timescale | What happens | Frequency |
|-----------|-------------|-----------|
| fast (per block) | [[focus]] flow within cells, UTXO processing | every block |
| medium (per epoch) | cross-cell [[focus]] synchronization, boundary updates | every ~100 blocks |
| slow (per era) | cell rebalancing — cells merge/split based on load and connectivity | every ~10K blocks |

the fast timescale sees fixed cell boundaries. the slow timescale adjusts boundaries based on accumulated statistics. because the fast dynamics converge much faster than boundaries change, the system is stable

### rebalancing

when a cell grows too large: split it along the [[Laplacian]] eigenvector boundary (spectral bisection via [[springs]])

when two cells have become tightly coupled (high cross-cell [[focus]] flow): merge them

when a zone's internal connectivity drops below threshold ([[springs]] eigengap shows it is really two zones): split the zone

state migration ([[particles]] and UTXOs move between cells) is amortized over the slow timescale

---

## shard count

at [[Avogadro]] scale — estimated count at each level per dimension:

| primitive | dimension | cell | zone | domain | global |
|---|---|---|---|---|---|
| [[particles]] | semantic | ~10^17 topics | ~10^12 fields | ~10^6 continents | 1 [[cybergraph]] |
| [[neurons]] | social | ~10^10 circles | ~10^7 communities | ~10^4 networks | 1 humanity |
| [[tokens]] | economic | ~10^6 denominations | ~10^4 baskets | ~10^2 economies | 1 token space |
| locations | geographic | ~10^6 villages | ~10^4 cities | ~10^2 states | 1 planet |

most of the 4D space is empty — dimensions correlate. cells exist only where [[particles]] actually cluster

---

## comparison

| System | Hierarchy | Static/Dynamic | Dimensions |
|--------|-----------|---------------|------------|
| IP (Internet) | 4-tier (network/subnet/host/port) | semi-static (ISP assigns) | 1 (topology) |
| Urbit | 4-tier (galaxy/star/planet/moon) | static (burned at genesis) | 1 (identity) |
| Ethereum 2.0 | 2-tier (beacon/shards) | static (64 shards) | 1 (hash range) |
| Cosmos | flat (sovereign chains + IBC) | static (per chain) | 0 (no hierarchy) |
| [[cyber]] | 4-tier (cell/zone/domain/root) | dynamic (computed by [[tri-kernel]]) | 4 (semantic, social, economic, geographic) |

address space:

| System | Total addresses |
|---|---|
| IPv4 | 2^32 = 4 × 10^9 |
| Urbit (planets) | 2^32 = 4 × 10^9 |
| Urbit (moons) | 2^64 = 1.8 × 10^19 |
| IPv6 | 2^128 = 3 × 10^38 |
| [[cyber]] | [[Hemera]] = 2^256 ≈ 10^77 (content-addressed, [[Avogadro]] is a rounding error) |

the key difference: every other system designs the hierarchy. [[cyber]] computes it. the [[tri-kernel]] is simultaneously the probabilistic engine, the folding oracle, and the routing advisor. one computation serves all three purposes

---

## open questions

shard boundary latency: how many blocks of cross-cell latency is acceptable before UX degrades? this determines the minimum cell size

privacy and routing: if a [[neuron]]'s cell assignment is public, it leaks information about their [[cyberlink]] patterns. can cell assignment itself be private?

incentive alignment: validators specialize in cells. what prevents a validator from refusing to serve a low-value cell?

cold-to-hot reactivation: when an archived [[particle]] gets new [[cyberlinks]], it must rejoin a cell. which cell? the semantic dimension may have shifted since it was archived

see [[cyber/architecture]] for the five-primitive resource model. see [[tri-kernel architecture]] for the locality filter. see [[cyber/state]] for the [[bbg]] world state. see [[cyber/network]] for the narrowcast relay protocol. see [[forgetting]] for the hot/cold tier separation