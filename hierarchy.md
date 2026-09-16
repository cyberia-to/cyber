---
tags: cyber, core, cip
crystal-type: entity
crystal-domain: cyber
crystal-size: article
alias: cyber hierarchy, folding, scaling, graph folding
status: draft
---
# cyber hierarchy

This proposal studies how the [[cybergraph]] could scale toward [[Avogadro]]
numbers — 10^23 [[particles]], 10^15 [[neurons]] — by deriving graph partitions
from the [[tri-kernel]]'s output. Counts, hop costs, proof costs and timescales
below are design estimates whose assumptions must be tested.

[[shards]] hold regions of graph state. [[neurons]] remain protocol subjects,
and progs retain their work under those subjects. Shard routing IDs, network
destinations, token books and neuron IDs have separate meanings under the
[domain model](specs/domain-ladder.md). Partitioning state preserves its
validation, availability, spending and finality obligations.

---

## the insight

the [[tri-kernel]] that computes [[focus]] also reveals the natural hierarchy. all three operators contribute:

| Operator | What it reveals | Folding role |
|---|---|---|
| [[springs]] | [[Laplacian]] eigenvectors — structural communities | defines cluster boundaries via spectral decomposition |
| [[heat]] | multi-scale smoothing — communities at different resolutions | controls the scale: low τ = fine shards, high τ = coarse domains |
| [[diffusion]] | random walk communities — where [[probability]] flows | validates clusters via flow concentration |

[[springs]] provides the eigenvectors that define fold lines. [[heat]] controls the resolution — which level of the hierarchy you read. [[diffusion]] reveals the flow patterns that validate the folds. the three together give robust community detection that no single operator provides alone

no administrator assigns structure. the [[tri-kernel]] computes it as a side effect of computing [[focus]]. the same operators that rank [[particles]] also partition the graph for scaling

---

## four dimensions

the [[cybergraph]] has four dimensions — the four primitives themselves. [[particles]] that are close in any dimension should share a shard

### [[particles]] — semantic

[[particles]] with high mutual [[focus]] flow — many [[cyberlinks]] between them, strong [[axon]] weights — form semantic clusters. the [[tri-kernel]] reveals these through spectral decomposition ([[springs]]) and multi-scale smoothing ([[heat]])

### [[neurons]] — social

[[neurons]] that frequently interact can supply social-locality signals. Public
links and explicitly disclosed coordination may support placement decisions.
Private UTXO ownership and spending relationships retain the selected protocol's
privacy policy. A neuron may act across several shards; co-location is a routing
optimization subject to disclosure and capacity constraints.

### [[tokens]] — economic

each [[token]] supplies an economic grouping. [[particles]] priced in [[$CYB]]
can cluster in $CYB regions; trading $CYB for $H crosses books in the token
dimension. A new token creates a book under [[research/oikos|oikos]]. The
assignment of one or more books to physical shards is a separate partitioning
decision, preserving each book's complete obligations and issuer rules.

The economic dimension supplies explicit labels alongside computed clustering.
A [[signal]] names its target `network` under the selected protocol profile.
That destination does not determine a shard, token issuer or signing subject.
An application may display networks as [[cards]] and may offer a private network
convention such as $H(\texttt{"network:"} \| \nu)$; using that convention requires
an explicit network profile. A neuron can address several compatible networks,
and a network can contain many neurons and shards. Joining or creating a network
requires its own admission and initialization rules. See [[network]].

### locations — geographic

latency matters for interactive use. [[neurons]] in the same physical region want low-latency access to their neighborhood. [[location proof]] provides this dimension. validators in a region preferentially serve that region's shards

---

## the 4×4 matrix

each dimension has four scales. a [[particle]] has a coordinate in each dimension at each scale

| primitive | dimension | [[shard]] | zone | domain | global |
|---|---|---|---|---|---|
| [[particles]] | semantic | topic | field | continent | [[cybergraph]] |
| [[neurons]] | social | circle | community | network | humanity |
| [[tokens]] | economic | denomination | basket | economy | all [[tokens]] |
| locations | geographic | village | city | state | planetary |

[[shards]] are the base operational level in this proposal — they hold state,
process transactions and run the [[tri-kernel]]. Zones, domains and the root
aggregate shard topology at different [[heat]] kernel temperatures. The selected
[[consensus]] profile must define the stake, validator assignment, authenticated
summaries and validation duties at every level. Graph clustering alone grants
no authority to finalize another region's state.

a [[particle]]'s [[shard]] = the intersection of its coordinates across all four dimensions. two [[particles]] sharing more coordinates → cheaper to move [[tokens]] between them. sharing all four → same [[shard]], zero cross-shard cost

```
shard(particle) = (semantic_shard, social_shard, token_shard, geo_shard)
```

---

## the root shard

the root shard is where all four dimensions meet at their global level — the origin (0,0,0,0)

it holds two things:

1. the [[crystal]] — the 5,040 [[particle]] seed that defines the foundational ontology. these [[particles]] are maximally general, referenced by everything, naturally highest [[focus]]

2. the routing table — maps [[particle]] hash → domain. not shard-level routing — that is each domain's job

```
root    → knows domains
domain  → knows zones
zone    → knows shards
shard   → knows particles
```

four hops to find any [[particle]] among 10^23. the root shard is the first hop

Before the graph has enough structure to fold, this model places its state in
one root region. An unpartitioned bootloader such as [[bostrom]] is the reference
case. The proposed phase transition threshold $|P^*| \sim \rho^2$ is a candidate
signal for considering division; the protocol must also authorize and verify
the split. The root continues to coordinate domain routing.

Within one hierarchy, new shards derive from an authenticated split or merge
of existing coverage. The biological metaphor is growth from a single fertilized
cell; its protocol counterpart requires explicit state handoff. See
[[spectral cell division]] for the proposed split/merge mechanism and its open
questions. Registration of a foreign domain establishes a separate route and
verification profile rather than inventing ancestry in this tree.

---

## two information flows

### subjective (neuron-driven)

[[tokens]], [[cyberlinks]], [[attention]] allocations. [[neurons]] choose where to move these. a [[neuron]] decides to send [[$CYB]] from shard A to shard B — that is a subjective decision, costs a [[proof]] relay

direction: horizontal and downward. [[neurons]] push information into shards

### objective (shard-computed)

[[focus]] aggregations, [[rank]] summaries, community structure, routing updates. no [[neuron]] moves these — each shard computes them deterministically from its local state and propagates upward

direction: upward only. shards push truth to zones, zones to domains, domains to root

```
root     ← receives domain summaries (objective)
domain   ← receives zone summaries (objective)
zone     ← receives shard summaries (objective)
shard    ← receives cyberlinks, tokens (subjective from neurons)
         → computes local focus, propagates upward (objective)
```

a [[neuron]] cannot push a fake [[rank]] summary upward — the shard computes it deterministically from the [[tri-kernel]] and proves it via [[zheng]]. the [[proof]] propagates with the summary. each level verifies the level below

the subjective layer (what [[neurons]] want) and the objective layer (what the graph computes) flow in different directions through the same structure. [[tokens]] flow wherever [[neurons]] send them. truth flows wherever the math says it goes

---

## hop cost

moving [[tokens]] between shards costs hops. the cost depends on how many dimensions differ and at what level:

| Difference | Hops | Example |
|---|---|---|
| same shard in all 4 dimensions | 0 | local transfer within a topic circle |
| differ in 1 dimension at shard level | 1 | same topic, different social circle |
| differ in 2 dimensions at shard level | 2 | different topic, different city |
| differ in 1 dimension at zone level | 2 | same field, different community |
| differ in 1 dimension at domain level | 3 | same continent of meaning, different network |

[[small world]] models motivate an average path length of order O(log N) under
their topology assumptions. Correlation between dimensions may shorten routes,
but graph diameter and authenticated routing cost require separate measurements.
The proposed six-to-seven-hop target at [[Avogadro]] scale remains a hypothesis.
Each cross-shard hop must carry the [[zheng]] evidence required by its profile.

---

## UTXOs and spending-state profiles

The private-UTXO profile represents notes by commitments and requires a ZK
[[proof]] for spending. Its disclosure contract specifies which public transition
metadata remains visible. Public balance projections and explicit public-credit
profiles retain their separate semantics.

The mutator-set research variant assigns each shard an [[AOCL]] for creation
and [[SWBF]] for spending. That variant proposes bloom-filter positions as the
spending representation and [[MMR]] compaction for retained witnesses. Its privacy,
witness-update and storage bounds need their own proof and measurements.
Profiles using a nullifier set retain their nullifier uniqueness and finality
rules. A split or merge must preserve spending uniqueness across old and new
boundaries for either representation; renaming the partition changes no ledger
encoding or monetary rule.

within-shard transfers are cheap — local state update, no cross-shard coordination. cross-shard transfers require [[zheng]] [[proof]] relay. the social dimension co-locates frequent transactors in the same shard

see [[cyber/state]] for transfer mechanics. see [[AOCL]] and [[SWBF]] for the [[mutator set]]

---

## folding the tri-kernel

the [[tri-kernel]] has a locality radius: h = O(log(1/ε)) hops. each [[particle]]'s [[focus]] depends only on its h-hop neighborhood

within a shard: the [[tri-kernel]] runs at full resolution. every [[cyberlink]], every [[axon]] weight, every market price is visible

within a zone: shards communicate aggregated [[focus]] vectors. each shard exports its boundary [[particles]]' [[focus]] values to neighboring shards

across zones: zones exchange coarse-grained [[focus]] summaries. the error is bounded:

$$\|\phi^*_{\text{folded}} - \phi^*_{\text{global}}\| \leq C \cdot e^{-\alpha h}$$

more communication → smaller error → closer to global [[focus]]

---

## timescales

| Timescale | What happens | Frequency |
|-----------|-------------|-----------|
| fast (per block) | [[focus]] flow within shards, UTXO processing | every block |
| medium (per epoch) | cross-shard [[focus]] synchronization, boundary updates | every ~100 blocks |
| slow (per era) | shard rebalancing — shards merge/split based on load and connectivity | every ~10K blocks |

The fast timescale assumes fixed shard boundaries. The slow timescale adjusts
them using accumulated statistics. Stability analysis must bound convergence
relative to boundary changes, delayed messages and adversarial load.

### rebalancing

when a shard grows too large: split it along the [[Laplacian]] eigenvector boundary (spectral bisection via [[springs]])

when two shards have become tightly coupled (high cross-shard [[focus]] flow): merge them

when a zone's internal connectivity drops below threshold ([[springs]] eigengap shows it is really two zones): split the zone

State migration ([[particles]] and UTXOs change serving partitions) is amortized
over the slow timescale. Token obligations remain in their home books. A handoff
must authenticate old/new coverage and checkpoints, preserve available history
and duplicate-spend protection, and fence writers before new partitions admit
effects. Open cross-shard operations keep their original identities, conditions
and unknown outcomes through migration.

---

## shard count

at [[Avogadro]] scale — estimated count at each level per dimension:

| primitive | dimension | shard | zone | domain | global |
|---|---|---|---|---|---|
| [[particles]] | semantic | ~10^17 topics | ~10^12 fields | ~10^6 continents | 1 [[cybergraph]] |
| [[neurons]] | social | ~10^10 circles | ~10^7 communities | ~10^4 networks | 1 humanity |
| [[tokens]] | economic | ~10^6 denominations | ~10^4 baskets | ~10^2 economies | 1 token space |
| locations | geographic | ~10^6 villages | ~10^4 cities | ~10^2 states | 1 planet |

most of the 4D space is empty — dimensions correlate. shards exist only where [[particles]] actually cluster

---

## comparison

| System | Hierarchy | Static/Dynamic | Dimensions |
|--------|-----------|---------------|------------|
| IP (Internet) | 4-tier (network/subnet/host/port) | semi-static (ISP assigns) | 1 (topology) |
| Urbit | 4-tier (galaxy/star/planet/moon) | static (burned at genesis) | 1 (identity) |
| Historical Ethereum 2.0 sharding design | 2-tier (beacon/shards) | configured shard count | 1 (hash range) |
| Cosmos | flat (sovereign chains + IBC) | static (per chain) | 0 (no hierarchy) |
| [[cyber]] | 4-tier (shard/zone/domain/root) | dynamic (computed by [[tri-kernel]]) | 4 (semantic, social, economic, geographic) |

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

shard boundary latency: how many blocks of cross-shard latency is acceptable before UX degrades? this determines the minimum shard size

privacy and routing: publishing which shards a [[neuron]] uses can disclose
interaction patterns. Which placement and subscription records can remain private
while coverage and routing remain verifiable?

incentive alignment: validators specialize in shards. what prevents a validator from refusing to serve a low-value shard?

cold-to-hot reactivation: when an archived [[particle]] gets new [[cyberlinks]], it must rejoin a shard. which shard? the semantic dimension may have shifted since it was archived

see [[cyber/architecture]] for the five-primitive resource model. see [[tri-kernel architecture]] for the locality filter. see [[cyber/state]] for the [[bbg]] world state. see [[cyber/network]] for the narrowcast relay protocol. see [[forgetting]] for the hot/cold tier separation

discover all [[concepts]]
