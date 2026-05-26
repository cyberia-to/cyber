---
alias: vimputer architecture, cyber architecture, five primitives
tags: cyber, article, cip
crystal-type: pattern
crystal-domain: cyber
crystal-size: deep
status: draft
---
# the five primitives of a [[vimputer]]

## a resource-complete architecture for earth-scale distributed computation

## the problem

every [[vimputer]] is an incomplete computer. they meter compute (gas, cycles, [[stark]] proofs) but treat the network itself — messaging, bandwidth, storage, and sequencing — as invisible infrastructure. nodes gossip for free. storage is outsourced to separate networks. ordering is bundled invisibly into block production. the result: [[vimputers]] cannot reason about their own metabolism, cannot price the resources they actually consume, and cannot incentivize efficient operation of the infrastructure they depend on.

a [[vimputer]] that operates at planetary scale must price every resource it consumes. this document defines the minimal complete architecture.

## five irreducible primitives

a [[vimputer]] consumes exactly five fundamental resources. each is irreducible — remove any one and the system ceases to function as a distributed computer.

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│    SEQUENCE ──── the causal backbone                    │
│        │                                                │
│        ├── COMPUTE ──── transform state                 │
│        │                                                │
│        ├── STORAGE ──── hold state                      │
│        │                                                │
│        ├── RELAY ────── move state                      │
│        │                                                │
│        └── CONSENSUS ── agree on state                  │
│                                                         │
│              φ* (focus) = exchange rate between all five  │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

why five and not fewer:

- without sequence: no causality. compute is incoherent, storage has no versioning, relay has no ordering, [[consensus]] cannot resolve conflicts
- without compute: no state transformation. the system can store and move data but cannot derive new [[knowledge]]
- without storage: no persistence. every result vanishes after computation
- without relay: no communication. every node is an island — a laptop, not a network
- without [[consensus]]: no shared truth. compute is local calculation, storage is a hard drive, relay is TCP

## primitive 1: sequence

verifiable ordering of events. not clock time — causal structure.

Lamport (1978) proved that distributed systems cannot rely on physical time. you need logical ordering. [[consensus]] tells you what we agree on. sequence tells you in what order things happened. you need ordering to even formulate the question that [[consensus]] answers.

producing verifiable ordering is the most expensive thing most [[vimputers]] do. PoW burns energy to establish ticks. PoS allocates validator slots as clock positions. VDFs prove irreducible sequential computation. all of these are, at their core, clock mechanisms.

| ordering strength | cost | mechanism | use case |
|---|---|---|---|
| none (commutative) | free | CRDTs | counters, sets, grow-only structures |
| causal (partial) | cheap | vector clocks, DAGs | message dependencies, local causality |
| total within shard | moderate | local sequencer, BFT | intra-shard state transitions |
| total global | expensive | cross-shard BFT, recursive proofs | double-spend prevention, global finality |

most operations need only causal ordering. global total ordering is the scarcest and most expensive resource in any distributed system — yet [[vimputers]] give it away for free, hidden inside block production. a [[vimputer]] should price ordering explicitly: cheap for causal, expensive for global.

## primitive 2: compute

state transformation. taking inputs and producing outputs.

all distributed computation decomposes into exactly three operations:

```
Cost(Aggregation) ≥ Cost(Proving) ≥ Cost(Verification)
```

- aggregation — combining distributed signals into shared state. this is the irreducible purpose of having a network. a market price, a [[focus]] distribution, a [[governance]] decision — all require information from many parties. remove aggregation and you remove the reason for the network's existence

- proving — generating cryptographic evidence that the aggregation was performed correctly. the bridge between untrusted computation and verifiable results. see [[cyber/proofs]]

- verification — checking proofs efficiently. this is where decentralization lives — where phones, sensors, and embedded devices participate. verification cost must be polylogarithmic in computation complexity

aggregation should happen once (by a specialized engine). proving should happen competitively (by market participants). verification should happen everywhere (by every node). the separated model scales linearly with verifiers (cheap) while holding aggregation constant:

$$\text{Total cost} = 1 \times \text{Cost}(A) + M \times \text{Cost}(P) + N \times \text{Cost}(V)$$

where $M \ll N$ and $\text{Cost}(V) \ll \text{Cost}(A)$.

economic polarity: write operations (state changes) are sender-pays — the sender modifies shared state, imposing externalities on all future readers. read operations (queries, ranking) are receiver-pays — the reader extracts value from existing state.

## primitive 3: storage

holding state across time.

three independent axes define storage cost:

$$\text{cost}(d) = f(\text{duration}, \text{privacy}, \text{structure})$$

### axis 1: duration

| duration | mechanism | economic model | example |
|---|---|---|---|
| ephemeral (seconds-hours) | DA layer, mempool | per-byte-per-slot, auto-expires | [[Celestia]] blobs |
| medium (days-years) | contract storage + PoST | buyer-defined duration, ongoing proofs, collateral | [[Filecoin]] deals |
| permanent | endowment + demand-driven replication | one-time payment, declining real cost (Kryder's Law) | [[Arweave]] |

these are not three competing products — they are one spectrum with a continuous decay function. a [[vimputer]] should expose duration as a single parameter that smoothly interpolates the economic model.

### axis 2: privacy/popularity gradient

$$\text{storage\_cost} \propto \frac{\text{privacy}}{\text{popularity}}$$

- public popular content: cost approaches zero. [[BitTorrent]] economics — more demand = more replicas = self-sustaining. the crowd stores it for free because serving it earns relay credit
- private rare content: cost is strictly positive, borne by the owner. nobody else has incentive to replicate what they can't read

dynamic transition mechanism: content enters explicit storage markets when first stored, transitions to demand-driven replication as popularity grows, falls back to explicit storage if demand wanes.

### axis 3: data structure

storage is tightly coupled with the logical structure of data. different structures imply different hardware, different proofs, different access patterns, and different cost profiles:

| structure | hardware affinity | proof type | access pattern | use for |
|---|---|---|---|---|
| KV store | SSD/RAM | external (bolt-on Merkle) | random read | [[consensus]] state, balances |
| Merkle trie | SSD (poorly) | native inclusion/exclusion | tree traversal | authenticated state |
| append-only log | HDD/sequential | position = proof | sequential | sequencing, event history |
| content-addressed DAG | any | hash chain | content lookup | immutable [[particles]], blobs |
| dense vector | GPU VRAM | none standard | batch parallel | [[focus]] computation, embeddings |
| adjacency / CSR | RAM/SSD | graph commitments | random walk | [[cybergraph]], edge traversal |

different structure types should have different fee schedules within the storage resource dimension. writing to authenticated KV ([[consensus]] state) is expensive — it affects proof sizes for everyone. appending to the sequencing log is cheap — it is sequential. storing a CID blob off-graph is cheapest — no proof overhead in live state.

verification: [[storage proofs]] are the most mature non-compute verification mechanism. [[Filecoin]] PoRep (slow sequential encoding to zk-SNARK) and WindowPoST (24-hour rolling audit) provide cryptographic certainty. [[Arweave]] SPoRA incentivizes full-dataset storage through mining probability.

## primitive 4: relay

moving state between nodes. the circulatory system of the [[vimputer]].

bandwidth is not a separate primitive. bandwidth is the derived throughput of relay operations. you do not prove "I CAN relay 100 Mbps" — you prove "I DID relay this message from A to B." the market discovers your effective bandwidth through how much relay work you win and complete.

verification — signature chains (NKN model): each relay node adds a cryptographic signature to a growing chain. the final chain is publicly verifiable. probabilistic on-chain recording: only packets whose final signature hash falls below a difficulty threshold are eligible for rewards. statistical fairness without full on-chain overhead.

| direction | who pays | example | routing optimization |
|---|---|---|---|
| push (sender pays) | sender has intent, bears cost | transactions, announcements, [[cyberlink]] proposals | minimize latency to finality |
| pull (receiver pays) | receiver has demand, bears cost | queries, subscriptions, sync requests | maximize relevance filtering |

push messages carry sender-signed relay requests. pull messages carry receiver-signed relay requests. the verification mechanism (signature chains) is identical — only the payment direction differs.

reciprocity at the protocol level: [[BitTorrent]]-style tit-for-tat embedded in the gossip layer creates immediate, local incentive alignment without waiting for on-chain settlement. nodes that do not relay do not receive relay. [[tokens]] handle the asynchronous and asymmetric cases where bilateral reciprocity breaks down.

location-aware routing: relay efficiency depends on physical geography. a node in Singapore relaying traffic between Tokyo and Sydney is useful. the same node relaying traffic between London and New York is wasteful. [[location proof]] enables the relay layer to route by physics, not by topology — replacing BGP's institutional path selection with RTT-optimal geographic routing. relay fees weighted by inverse latency (`fee proportional to 1/latency`) make geographic honesty a dominant strategy: nodes that honestly report location earn more because they get routed traffic that physically passes near them.

## primitive 5: consensus

converting individual subjective signals into shared objective state. the membrane through which private resources become collective truth.

you can have compute without [[consensus]] (a laptop). you can have storage without [[consensus]] (a USB stick). you can have relay without [[consensus]] (a router). but you cannot have a [[vimputer]] without [[consensus]]. [[consensus]] is the resource that transforms private computation into shared reality.

| finality | cost | mechanism | scope |
|---|---|---|---|
| probabilistic (minutes) | cheap but reversible | Nakamoto/longest chain | global, weak guarantee |
| deterministic (seconds) | moderate, bounded validators | BFT/Tendermint | shard or zone |
| instant (sub-second) | cheap but local scope | DAG-based, local BFT | neighborhood |
| irreversible (checkpointed) | expensive, global | recursive proof + L1 settlement | global, strong guarantee |

cheap [[consensus]] is fast but fragile. expensive [[consensus]] is slow but permanent. the same economic tradeoff as storage duration — and the same design principle: expose it as a continuous parameter, not discrete tiers.

## [[location proof]]: the missing layer

the existing internet addressing stack conflates two orthogonal concepts: identity (who you are) and location (where you are physically). an IP address encodes both simultaneously. this architectural error, present since 1973, produces a fragile hierarchy: IANA to RIR to AS to ISP to user. every layer is a point of control, censorship, and failure.

the correct separation:

```
pubkey    →  WHO   (permanent, cryptographic, self-sovereign)
geohash   →  WHERE (dynamic, physical, verifiable)
```

without verifiable location, decentralized routing remains dependent on the same institutional hierarchies it seeks to replace. [[location proof]] is not a sixth primitive — it is cross-cutting infrastructure that makes relay efficient, sequence verifiable, and [[consensus]] geographically aware.

### four axioms, zero trusted institutions

```
A1. I exist.
    (Cogito ergo sum — the irreducible act of observation.)

A2. Signal propagation speed is bounded by a canonical constant
    c_medium, known per medium and publicly verifiable.
    (Physics. Not negotiable.)

A3. Earth is a sphere of known circumference.
    (~40,075 km. Observable. Not an institutional claim.)

A4. At least one honest observer exists in the mesh.
    (Sybil bound. Weaker than any existing PKI assumption.)
```

no GPS. no IANA. no certificate authorities. no trusted anchors.

### core construction

RTT as distance bound: Round Trip Time between two nodes A and B, over a medium with canonical speed `c_medium`, establishes a hard physical upper bound on distance:

$$\text{dist}(A, B) \leq \frac{\text{RTT}(A, B) \times c_{\text{medium}}}{2}$$

a node can only prove itself farther from another node than it actually is, never closer. faking proximity is physically impossible. this asymmetry is the foundational security property.

VDF prevents pre-computation: a verifiable delay function ensures challenge-response timing cannot be gamed. the responding node cannot pre-compute responses before receiving the challenge.

Merkle causal clock: all RTT measurements are committed simultaneously via Merkle tree, preventing selective presentation of favorable measurements.

### anchor-free coordinate system

given N nodes measuring pairwise RTTs, construct a distance matrix normalized by declared medium speed:

$$D[i][j] = \frac{\text{RTT}(i, j) \times c_{\text{medium}(i,j)}}{2}$$

classical Multidimensional Scaling (MDS) recovers a 3D coordinate embedding from D alone — no known positions needed. the solution is unique up to rotation, reflection, and translation.

the planet's circumference encodes directly into the maximum observable RTT. as the mesh grows globally, the spherical geometry forces a unique embedding. the coordinate system self-calibrates to Earth's scale from canonical propagation speeds and physical reality — no external reference.

Sybil resistance: a Sybil node claiming position G must present RTTs consistent with G to all existing nodes simultaneously. faking consistency with a dense global mesh is physically impossible.

```
Honest node in Bali:          Sybil claiming Bali from Moscow:
  RTT(Singapore) = 20ms  OK     RTT(Singapore) = 160ms  FAIL
  RTT(Tokyo)     = 70ms  OK     RTT(Tokyo)     = 140ms  FAIL
  RTT(London)    = 180ms OK     RTT(London)    = 60ms   FAIL
  → consistent with Bali       → inconsistent, rejected by MDS
```

### economic enforcement: honesty as dominant strategy

if relay fees are proportional to inverse latency:

$$\text{fee} = \frac{k}{\text{latency}}$$

then honest location reporting maximizes relay income regardless of what other nodes do:

```
for all strategies S of other nodes:
  U(honest | S) >= U(lie | S)
```

this is stronger than Nash equilibrium — it is a dominant strategy [[equilibrium]]. geographic honesty is enforced not by [[cryptography]] but by economic gravity. lying about your location makes you a worse router and earns you less. the market verifies location continuously without explicit proof verification in steady state.

### what this replaces

| current stack | [[vimputer]] with PoL |
|---|---|
| IANA to RIR to AS to ISP | physics + mesh [[consensus]] |
| IP address (identity + location conflated) | pubkey (identity) + geohash (location) |
| BGP (political routing) | RTT-optimal routing |
| GPS (US military infrastructure) | canonical c_medium + MDS |
| trusted anchors (FOAM, Helium) | Earth's geometry |

### how PoL strengthens the five primitives

relay: location-aware routing beats topology-only routing on latency. nodes earn relay fees proportional to their geographic utility, not just their connectivity. this is what creates emergent hubs.

sequence: RTT bounds combined with VDFs provide verifiable timing constraints. you cannot compress time. this strengthens ordering guarantees beyond what logical clocks alone provide.

[[consensus]]: validator geographic distribution becomes verifiable rather than assumed. the network can ensure [[consensus]] participants are physically distributed, hardening against correlated failures.

storage: geographic proximity to data consumers reduces retrieval latency. nodes can be compensated for storing data close to where it is demanded.

compute: location-aware task routing enables placing computation near the data it needs, reducing relay overhead for compute-intensive operations.

## emergent hierarchy: [[focus]] + relay economics

a single-chain [[vimputer]] with full replication does not need sharding to develop structure. hierarchy emerges naturally from the economics of the five primitives combined with [[location proof]].

### how hubs form without permission

in the current internet, Tier 1 providers are hubs because they institutionally agreed on settlement-free peering. their centrality is a result of contracts, not physics. a new player cannot become Tier 1 without permission from existing Tier 1 providers. this is a cartel.

in a [[vimputer]], centrality is a computed quantity:

$$\text{centrality}(n) = f(\phi^*_n,\ \text{relay\_throughput}_n,\ \text{location\_utility}_n)$$

a node with high [[focus]] has lots of [[attention]] flowing through it. a node with high relay throughput moves lots of data. a node with high location utility sits where routing is physically efficient. these three signals reinforce each other:

```
good location → attracts relay traffic
  → generates relay fees
    → enables more stake
      → higher weight on cyberlinks
        → higher φ* for content through this node
          → more demand for storage/compute nearby
            → more economic activity
              → even more relay traffic
```

positive feedback loop. hubs emerge not by permission, but by physics and economics.

### liquid hierarchy

unlike the fixed hierarchy of the internet (which changes only through multi-year business negotiations), [[vimputer]] hierarchy is reversible in real time. a node that stops relaying, loses stake, or degrades its bandwidth loses centrality immediately. [[focus]] recomputes continuously. there is no lock-in.

you do not need sharding to have structure. on a single chain with full replication, relay economics + [[location proof]] + [[focus]] dynamics already produce a differentiated network [[topology]] where some nodes naturally serve as hubs. sharding can be introduced later, informed by observed emergent structure — not designed in advance.

## the fractal [[consensus]] architecture (scaling vision)

the single-chain design comes first. but the architecture must contain the seeds of what comes next. nature shows the pattern: computation organizes in layers, with massive local activity and tiny upward state commitments. the brain does not track every molecule — it receives radically compressed signals from each subsystem.

the fundamental law:

$$\text{computation volume} \propto \frac{1}{\text{layer height}}$$
$$\text{state committed upward} \propto \frac{1}{\text{layer height}}$$
$$\text{trust requirement} \propto \text{layer height}$$

### layer structure

| layer | scope | computation | state upward | trust model |
|---|---|---|---|---|
| L0: local | single node | massive, free | hash of result | self (no [[consensus]]) |
| L1: neighborhood | ~10-100 nodes | moderate, cheap [[consensus]] | aggregate + proof | local BFT |
| L2: shard | ~10^3-10^6 nodes | significant, BFT | state root + proof | shard [[consensus]] |
| L3: global | all shards | minimal — verification only | single small commitment | recursive proof |

### how the five primitives map across layers

sequence: each layer has its own clock speed. L0 ticks in milliseconds. L1 in seconds. L2 in tens of seconds. L3 — the global singleton — ticks slowest and most expensively. ordering precision is a resource priced per layer.

compute: 99.9% happens at L0. each upper layer only aggregates proofs from below. L0 does aggregation, L0 to L1 does proving, L2 to L3 does verification only. the irreducible triad maps perfectly onto the layer hierarchy.

storage: L0 stores full data (cheap, local, ephemeral). L1 stores aggregated state + proofs. L2 stores state roots. L3 stores only the global commitment — O(1) size regardless of network scale. the duration spectrum maps directly: L0 is ephemeral, L3 is permanent.

relay: mostly horizontal within layers (peer gossip within neighborhoods), with narrow vertical channels between layers (proof submission upward, finality confirmation downward). bandwidth demand is massive within L0, minimal at L3. [[location proof]] determines which neighborhoods form — geographic proximity creates natural L1 clusters.

[[consensus]]: L0 needs none (self-trust). L1 uses lightweight local BFT. L2 uses shard-level [[consensus]]. L3 needs only to verify recursive proofs — near-zero computation, maximum trust. the global singleton state can be constant-size (Mina-like ~22kb) because recursive [[stark]] composition produces fixed-size proofs regardless of the computation being proved.

### the 22kb global state

recursive proof composition enables:

$$\text{global proof size} = O(1) \text{ regardless of network scale}$$

L1 nodes prove "these 100 local computations were correct." L2 aggregates those proofs into "these 1000 L1 proofs were valid." L3 compresses everything into one proof: "the entire network state transition was valid." proof size does not grow with the computation being proved.

### from single chain to fractal

the single-chain phase is where emergent hierarchy is observed. the fractal phase is where it is formalized. the transition happens when the single chain's observed hub structure — which nodes relay most, which geographic clusters communicate densely, where [[focus]] concentrates — provides empirical data for layer boundary decisions. sharding follows physics, not theory.

## two verification categories

the five primitives collapse into two verification types:

### state proofs (what you HOLD)

- storage: PoRep / PoST (prove you hold this data right now). see [[storage proofs]]
- compute: [[stark]] (prove you computed this correctly). see [[cyber/proofs]]
- [[consensus]]: recursive proof composition (prove the network agreed)

### flow proofs (what you DID)

- relay: signature chains (prove you forwarded this message)
- sequence: VDFs / position in append-only log (prove this ordering is valid)
- location: RTT mesh + MDS consistency (prove where you are physically). see [[location proof]]
- bandwidth: derived from relay throughput over time. not a separate verification problem

## [[focus]] as universal resource pricing oracle

the [[focus]] vector $\phi^*$ — the stationary distribution of the [[token]]-weighted random walk on the [[cybergraph]] — is not just an [[attention]] metric. it is the exchange rate between all five resource types.

how [[focus]] prices each resource:

| resource | high-[[focus]] content | low-[[focus]] content |
|---|---|---|
| storage | cheap (demand-driven replication, self-sustaining) | expensive (requires explicit storage market) |
| relay | cheap (cached at edges, many replicas) | expensive (must be fetched from source) |
| compute | cheap (results memoized, widely cached) | expensive (must compute from scratch) |
| [[consensus]] | deserves tight ordering (expensive, global finality) | tolerates loose ordering (cheap, local) |
| sequence | fast ticks, high precision | slow ticks, eventual consistency sufficient |

[[focus]] is not set by [[governance]]. it emerges from the same [[focus]] dynamics that drive ranking. the market does not need to discover resource prices separately — the [[attention]] signal that already organizes the [[knowledge]] graph also organizes the resource economy.

conservation: $\sum_i \phi^*_i = 1$ always. [[focus]] is a zero-sum resource. [[attention]] given to one [[particle]] is [[attention]] taken from another. this creates natural scarcity without artificial supply caps.

## economic design principles

### 1. multi-dimensional fee markets

each of the five resource primitives gets its own base fee, updated independently via the provably near-optimal EIP-1559 exponential rule:

$$\text{basefee}_r[t+1] = \text{basefee}_r[t] \times \exp\left(\alpha_r \times \frac{\text{usage}_r[t] - \text{target}_r}{\text{target}_r}\right)$$

per-dimension block limits enforce safety. per-dimension base fees enable independent price discovery. a single user-facing fee preserves UX — the protocol allocates the budget across dimensions.

### 2. polarity-aware pricing

every resource operation declares its direction (push or pull). the payer is determined by who extracts more value:

| | push (sender pays) | pull (receiver pays) |
|---|---|---|
| relay | transactions, broadcasts | queries, subscriptions |
| storage | private persistence | public retrieval |
| compute | state changes (writes) | rank queries (reads) |
| [[consensus]] | proposal submission | finality confirmation |
| sequence | ordering claim | ordering verification |

### 3. dominant resource fairness for node compensation

for each node, identify the resource where it contributes the highest fraction of needed network capacity (its "dominant resource"). compensate based on this bottleneck contribution. DRF is strategy-proof, envy-free, and Pareto efficient — nodes cannot inflate rewards by oversupplying abundant bandwidth while under-providing scarce storage.

### 4. relay fees create structure

relay fees are the only revenue component that is not shared equally among validators. they flow to specific nodes proportional to relay contribution weighted by inverse latency. this differentiation — combined with [[location proof]] — is what creates emergent hierarchy on a flat single chain. nodes in better physical locations with better bandwidth earn more relay fees, stake more, create more weighted [[cyberlinks]], and accumulate higher [[focus]].

### 5. reciprocity before [[tokens]]

bilateral reciprocity (tit-for-tat) handles most resource exchange without on-chain settlement. [[tokens]] handle the asymmetric and asynchronous cases. this minimizes [[consensus]] overhead for the vast majority of resource exchanges.

## the complete ontology

```
VIMPUTER PRIMITIVES
═══════════════════

SEQUENCE    verifiable ordering of events.
            spectrum: commutative → causal → total
            priced by: ordering precision

COMPUTE     state transformation via aggregation → proving → verification.
            polarity: write (sender pays) / read (receiver pays)
            priced by: operation complexity × proof generation cost

STORAGE     holding state across time.
            three axes: duration × privacy/popularity × data structure
            priced by: f(duration, privacy/popularity, structure type)

RELAY       moving state between nodes. signature chain verified.
            polarity: push (sender pays) / pull (receiver pays)
            location-aware routing via proof of location.
            priced by: message size × route length × 1/latency

CONSENSUS   converting private signals into shared truth.
            spectrum: probabilistic → deterministic → irreversible
            priced by: finality strength × scope

φ* (FOCUS)   the universal exchange rate between all five resources.
            emergent from token-weighted random walks on the cybergraph.
            not governance-set — computed. not voted — converged.
            conservation: Σ φ*ᵢ = 1 (always)

PROOF OF LOCATION
═════════════════

            cross-cutting infrastructure. not a sixth primitive —
            the physical substrate that makes relay efficient,
            sequence verifiable, and consensus geographically honest.

            construction: RTT mesh + MDS + Earth calibration.
            four axioms: existence, bounded signal speed,
            spherical Earth, one honest observer.
            economic enforcement: fee proportional to 1/latency →
            geographic honesty is dominant strategy.

EMERGENT HIERARCHY
══════════════════

            φ* + relay economics + proof of location →
            hubs form without permission.
            liquid hierarchy: reversible in real time.
            no sharding needed for structure to emerge.
            institutional hierarchy replaced by physical/economic hierarchy.

FRACTAL ARCHITECTURE (scaling vision)
═════════════════════════════════════

            L0 (local)         massive compute, no consensus, free.
            L1 (neighborhood)  local BFT, small proofs upward.
            L2 (shard)         shard BFT, state root upward.
            L3 (global)        verification only. O(1) state. the singleton.

            LAW: computation compresses as it rises.
                 trust requirements increase.
                 global state is constant-size (~22kb).
                 layer boundaries emerge from observed hub structure,
                 then are formalized — not designed in advance.
```

## what to build first

the starting point is a single chain with full replication — every node stores everything, executes everything, relays everything. the simplest possible case. no sharding, no layers, no role separation. the goal: an ideal [[vimputer]] that prices and verifies every resource it consumes.

priority 1 — five-dimensional fee market. each transaction pays for sequence, compute, storage, relay, and [[consensus]] as separate metered resources. EIP-1559 exponential base fee per dimension. single user-facing fee with protocol-side decomposition.

priority 2 — relay signature chains. integrate NKN-style relay accounting into the networking layer. every message hop is signed. probabilistic on-chain settlement. relay fees flow to relayers, not to block producers — this is the seed of emergent hierarchy.

priority 3 — [[location proof]]. RTT mesh between all nodes. MDS coordinate embedding. geohash claims verified by mesh consistency. `fee proportional to 1/latency` for relay pricing. no GPS, no trusted anchors.

priority 4 — duration-parameterized storage. unify ephemeral, medium-term, and permanent storage under a single primitive with continuous duration economics. storage fee = f(size, duration, privacy, structure type). see [[storage proofs]].

priority 5 — [[stark]] proof of block execution. every block produces a proof that all state transitions were valid. this is the foundation that enables future scaling — but on single chain it already provides trustless light clients and instant sync. see [[cyber/proofs]].

what comes later (the fractal [[consensus]] architecture):

- layer separation — formalize the emergent hub structure into L0/L1/L2/L3 hierarchy
- sharding — only after observing which geographic/economic clusters communicate densely
- recursive proof composition — compress planetary activity into O(1) global state (~22kb)
- cross-shard [[consensus]] — only after shard boundaries are understood empirically

the principle: build the simplest complete system first. observe what structure emerges. then formalize that structure into architecture.

the [[vimputer]] does not simulate a computer — it IS a computer. one that prices every resource it consumes, routes by physics instead of politics, lets hubs emerge from economics instead of contracts, and uses [[attention]] as the universal exchange rate between computation, storage, communication, ordering, and truth.