---
tags: cyber, cip
crystal-type: pattern
crystal-domain: cyber
alias: network layer, p2p, peer-to-peer, cyber network
---
# network

how [[neurons]] find each other, propagate [[cyberlinks]], and maintain a shared view of the [[cybergraph]]. the network is lean: you pay for what you consume, epidemic broadcast is reserved for headers only, and most [[cyberlinks]] never touch most nodes.

## the principle: narrowcast everything, broadcast nothing

a [[cyberlink]] about Balinese rice terraces does not concern a node aggregating DeFi price feeds in Frankfurt. epidemic broadcast — sending every link to every node — treats the network as a stadium PA system. the [[cybergraph]] is a conversation, not an announcement.

the only artifact that every node needs is the block header (~232 bytes). headers commit to the full [[BBG]] root, enabling any claim to be verified. everything else is narrowcast: sent only to those who will aggregate it, subscribe to it, or pay for it.

```
what propagates how:

  headers (~232 bytes)     epidemic      every node
  cyberlinks               narrowcast    aggregators + namespace subscribers
  block data (DA blobs)    sampling      DAS verifiers (random sparse checks)
  query responses          point-to-point   the requester only
```

## stack

```
┌─────────────────────────────────────┐
│  cyber/network                      │  narrowcast routing, paid headers,
│  (this page)                        │  cybergraph-native coordination
├─────────────────────────────────────┤
│  cyber/communication                │  onion routing, proof of delivery,
│                                     │  CSIDH key agreement
├─────────────────────────────────────┤
│  radio                              │  QUIC, hole-punching, relay,
│  (iroh fork with Hemera)            │  verified streaming, blob transfer
├─────────────────────────────────────┤
│  UDP/IP                             │  physical transport
└─────────────────────────────────────┘
```

[[radio]] handles transport: QUIC connections, NAT hole-punching via [[radio/relay]], verified streaming via [[radio/bao]] (Hemera Merkle trees). [[cyber/communication]] handles privacy: onion routing, CSIDH key agreement, [[stark]] proof of delivery. this page handles coordination: who connects to whom, how data flows, and who pays for what.

## peer discovery via cybergraph

traditional p2p networks use external mechanisms for peer discovery: DHTs (Kademlia), DNS seeds, hardcoded bootstrap nodes. cyber uses the [[cybergraph]] itself.

every [[neuron]] publishes its endpoint information as a [[cyberlink]]:

```
~neuron/endpoint → particle(addr: relay_url, direct: [socket_addrs])
```

this is a standard [[name]] resolution: the `~` prefix signals deterministic resolution. any [[neuron]] that knows another neuron's public key can resolve their current network address by traversing the [[cybergraph]].

three discovery mechanisms work together (inherited from [[radio/discovery]]):

| mechanism | scope | how it works |
|---|---|---|
| [[cybergraph]] resolution | global | resolve `~neuron/endpoint` via graph traversal |
| [[Pkarr]] (DHT) | global | PublicKey → EndpointAddr via distributed hash table |
| mDNS | local network | multicast discovery for nearby [[neurons]] without internet |

Pkarr provides bootstrap — finding the first peers to connect to. once connected, the [[cybergraph]] provides the authoritative, stake-weighted peer directory. a [[neuron]]'s endpoint [[cyberlink]] is authenticated by their key, timestamped, and weighted by their stake. stale or fraudulent endpoint claims decay through standard [[forgetting]] mechanics.

## paid headers: the lean protocol

the block header is the trust anchor — it commits to the full [[BBG]] root and lets any light client verify any claim about the [[cybergraph]]. distributing headers for free means light clients extract full verification value at zero cost. cyber does not do this.

headers are a pull resource. the receiver extracts value (verification capability), so the receiver pays.

### bootstrap economics

a new [[neuron]] entering the network must acquire some [[$CYB]] before downloading even the first header. this is skin in the game from the first byte. acquisition paths:

- receive from another [[neuron]] (gift, payment, grant)
- earn through relay services (tit-for-tat reciprocity does not require tokens)
- buy on an external market via [[cyber/ibc]] bridge

once the neuron holds tokens, it buys headers from peers. neighbors can offer headers cheaper — lower relay cost due to proximity, reciprocity credits from prior interactions. this creates geographic price differentiation naturally, without protocol-level sharding.

### header pricing

```
header price = base_fee(relay) × header_size × 1/peer_latency
```

- `base_fee(relay)` is the EIP-1559 exponential fee for the relay primitive (see [[cyber/architecture]])
- `header_size` is ~232 bytes (constant)
- `1/peer_latency` rewards geographic proximity: closer peers deliver faster and cheaper

a neighbor on the local network (mDNS-discovered) offers headers at near-zero cost. a peer across the planet charges more. the header market creates the same geographic hierarchy that [[location proof]] formalizes — without requiring location proof infrastructure to be operational first.

### recursive stark headers

with recursive [[stark]] composition, a new node does not need the full header chain. it needs one recursive proof (~100-200 KB) covering the entire chain from genesis, plus the latest header. the cost of syncing from genesis is the cost of purchasing and verifying one proof — seconds of compute, kilobytes of data.

this proof is itself a saleable artifact. a node that maintains the recursive chain proof can sell "instant sync" to new participants at a premium over raw header-by-header sync.

## cyberlink propagation: narrowcast to aggregators

a [[neuron]] creates a [[cyberlink]]. who needs it?

| consumer | why | delivery |
|---|---|---|
| aggregator serving this namespace | will include it in the next block | direct send (push) |
| namespace subscribers | explicitly requested this subgraph | topic delivery (pull) |
| the neuron's followers | personal interest | topic delivery (pull) |
| everyone else | they don't need it | never delivered |

### the flow

```
neuron creates cyberlink
        │
        ▼
signs link with neuron key
        │
        ▼
sends directly to aggregator(s) serving this namespace
        │
        ▼
aggregator:
  1. verifies signature
  2. verifies neuron has sufficient focus
  3. includes in block
  4. produces stark proof of correct inclusion
  5. publishes block header (epidemic — 232 bytes)
  6. publishes erasure-coded block data to DA layer
        │
        ▼
namespace subscribers pull their slice + completeness proof
        │
        ▼
DAS verifiers sample random chunks (sparse, probabilistic)
```

the [[cyberlink]] itself travels one hop: neuron → aggregator. the header travels epidemically (but it is 232 bytes). the block data is erasure-coded and sampled, not downloaded in full by anyone except the aggregator.

### aggregator discovery

aggregators are [[neurons]] that serve specific namespaces. they advertise their role via [[cyberlinks]]:

```
~aggregator/serves → particle(namespace: "biology")
~aggregator/serves → particle(namespace: "defi")
```

a [[neuron]] creating a biology [[cyberlink]] resolves `~*/serves/biology` to find active aggregators for that namespace. multiple aggregators may serve the same namespace — redundancy without epidemic broadcast.

aggregators earn fees for inclusion (sender pays — the neuron creating the link). competition between aggregators for the same namespace keeps fees low and inclusion fast.

## focus propagation: signals as π updates

the network has no central node that computes the [[focus]] distribution π*. instead, π* emerges from [[cyber/signals]]. every [[cyber/signal]] carries a $\pi_\Delta$ — the neuron's locally computed focus shift for a batch of [[cyberlinks]] — proven by a single [[stark]] proof.

### signal structure

```
signal {
    neuron:     pubkey
    links:      [cyberlink]                one or more 7-tuple assertions
    pi_delta:   [(particle_id, Δπ)]        sparse focus update for the batch
    proof:      stark                       proof of correct local computation
    timestamp:  u64
}
```

the `pi_delta` covers particles within the neuron's O(log(1/ε))-hop neighborhood. the [[locality]] theorem guarantees effects beyond that radius are below ε. the proof references a specific `bbg_root` from a header the neuron has verified. a single proof covers the entire batch of links — proving $n$ links together costs less than $n$ separate proofs because shared neighborhood state is proved once.

### how π converges without central computation

```
neuron queries neighborhood π + edges (with proofs from any peer)
    │
    ▼
creates cyberlinks, runs local tri-kernel step for the batch
    │
    ▼
produces stark proof: "this pi_delta follows from
  applying my links to the graph at bbg_root_t"
    │
    ▼
bundles into signal, sends to aggregator
    │
    ▼
aggregator applies pi_delta to local π view
    │
    ▼
namespace subscribers receive signal, apply pi_delta
    │
    ▼
their future signals carry updated pi_deltas
    │
    ▼
π* emerges from convergence of all local proven updates
```

this is gossip-based distributed belief propagation. the [[tri-kernel]] contraction theorem (§5.6 of the whitepaper) guarantees convergence: any order of applying proven pi_deltas reaches the same π*. the global fixed point crystallizes from local proofs without any node computing it centrally.

### self-minting

the $\pi_\Delta$ proof doubles as a reward claim. if the proven $\Delta\pi > 0$, the neuron mints [[$CYB]] proportional to the shift. no aggregator decides the reward — the proof IS the mining. see §14.2 of the whitepaper for the conservation constraint and attribution mechanism.

a [[neuron]] on a phone: buy a header, query neighborhood state, create [[cyberlinks]], prove Δπ, bundle into a [[cyber/signal]], mint tokens. the device that creates knowledge is the device that earns from it.

## data availability: sampling without global knowledge

the full network does not store or download block data. data availability is verified probabilistically through DAS (Data Availability Sampling).

the aggregator erasure-codes each block's [[cyberlinks]] and publishes the coded chunks. DAS verifiers — any node, including light clients — sample random chunks and verify them against the block header's DA commitment. if enough random samples succeed, the data is available with high probability.

```
block data (N cyberlinks)
        │
        ▼
erasure coding (2N coded chunks)
        │
        ▼
chunks distributed to nearby peers
        │
        ▼
DAS verifiers sample k random chunks
        │
        ▼
if k/k pass → data available with probability 1 - (1/2)^k
```

with k = 30 samples, the probability of falsely confirming availability is $< 10^{-9}$. each sample is a single chunk (~256 bytes) plus a Merkle proof (~1 KB). total DAS cost per block per verifier: ~30 KB.

the [[BBG]]'s namespace structure enables namespace-aware DAS: a subscriber sampling "give me everything for namespace N" receives data plus a completeness proof — cryptographic certainty that nothing was withheld.

## gossip topology

[[radio/gossip]] (HyParView + PlumTree) provides the transport for both epidemic header broadcast and narrowcast topic delivery.

### topic structure

| topic | what propagates | propagation mode | who subscribes |
|---|---|---|---|
| `Hemera("headers")` | block headers (~232 bytes) | epidemic | every node |
| `Hemera("ns/" ∥ namespace)` | cyberlinks within namespace | narrowcast | namespace aggregators + subscribers |
| `Hemera("neuron/" ∥ pubkey)` | links by a specific neuron | narrowcast | followers |
| `Hemera("da/" ∥ block_hash)` | erasure-coded block chunks | pull | DAS verifiers |

the critical distinction: only the headers topic uses epidemic broadcast. all other topics are narrowcast — delivery to subscribers only, no flooding.

### header propagation latency

headers are the only epidemic artifact. for a global network:

- header size: 232 bytes
- expected hops: O(log N) via broadcast tree
- per-hop latency: ~50-100ms (intercontinental QUIC)
- for 10,000 nodes: ~13 hops, ~0.4-1.3s total

this is the [[foculus]] finality budget. the header is the finality signal. everything else arrives later, on demand.

## the cybergraph as its own routing table

the [[cybergraph]] encodes which [[neurons]] are interested in which [[particles]]. a [[neuron]] that has created many [[cyberlinks]] involving biology [[particles]] is interested in biology links. the [[focus]] distribution $\pi^*$ provides a natural routing metric.

### interest-based peering

nodes maintain connections to peers whose [[focus]] distributions overlap with their own:

$$\text{peering\_affinity}(A, B) = \sum_{p \in P} \min(\pi^*_A(p), \pi^*_B(p))$$

the Bhattacharyya coefficient between two nodes' focus distributions. high affinity means shared attention on the same [[particles]]. the gossip layer maintains a partial view biased toward high-affinity peers — relevant cyberlinks arrive from peers who care about the same subgraph.

### semantic routing

a query "what connects malaria to treatment?" does not flood the network. the querying node identifies high-[[focus]] [[particles]] in the relevant subgraph, finds [[neurons]] with high [[karma]] there, and routes the query toward those neurons.

```
query arrives
    │
    ▼
local node checks local cybergraph view
    │
    ├── sufficient data? → respond locally (with proof)
    │
    └── insufficient? → route to high-affinity peers
                              │
                              ▼
                        peers with high π* on
                        query-relevant particles
                              │
                              ▼
                        response + proof flows back
```

the response includes a proof against the [[BBG]] root. the querying node verifies without trusting the responder.

## sybil resistance

the network layer inherits sybil resistance from the [[cybergraph]]'s stake-weighted structure:

- peer discovery via [[cybergraph]]: endpoint claims are stake-weighted. a sybil neuron with zero stake has zero weight in peer discovery
- paid headers: a node with no tokens cannot sync the chain, let alone flood it
- aggregator economics: submitting invalid cyberlinks to an aggregator costs [[focus]] and accumulates negative [[karma]] via [[Bayesian Truth Serum]] scoring. the aggregator drops invalid links before inclusion
- relay reciprocity: [[BitTorrent]]-style tit-for-tat in the gossip layer. nodes that contribute nothing receive nothing

creating 1000 sybil [[neurons]] with zero stake produces zero influence on the network. the cost of disrupting aggregation is the cost of acquiring sufficient stake to create high-weight links — the same economic security bound as [[foculus]] consensus.

## consistency model

the network operates under partial synchrony: messages arrive within an unknown but finite bound $\Delta$.

### what is guaranteed

- safety: no conflicting finalized [[particles]] (from [[foculus]])
- completeness verification: a node can cryptographically verify that it has ALL links in a namespace via [[BBG]] completeness proofs
- DA guarantee: if DAS passes, the block data is available with overwhelming probability

### what is not guaranteed

- real-time propagation of cyberlinks: during partitions, links may be delayed to aggregators
- ordered delivery: links may arrive at the aggregator out of creation order. the aggregator determines inclusion order

during asynchronous periods, no new [[particles]] finalize. existing finalized particles remain final. liveness resumes when connectivity restores.

## bandwidth budget

the narrowcast model radically reduces bandwidth compared to epidemic broadcast:

| artifact | size | frequency | delivery | bandwidth per node |
|---|---|---|---|---|
| headers | 232 bytes | every block (~1/s) | epidemic | ~232 bytes/s |
| cyberlinks (as creator) | ~100-500 bytes | per link created | one hop to aggregator | negligible |
| cyberlinks (as subscriber) | varies | per subscribed namespace | pull | proportional to subscriptions |
| DAS samples | ~30 KB | per block | random pull | ~30 KB/s |

a minimal node (headers + DAS only): ~30 KB/s. a namespace aggregator: proportional to namespace activity. no node downloads the full block data unless it chooses to.

[[focus]]-based prioritization: when an aggregator is overloaded, it prioritizes links from high-[[karma]] [[neurons]] targeting high-[[focus]] [[particles]]. low-priority links queue. the network's attention structure organizes its own traffic.

## connection to fractal architecture

the narrowcast model maps naturally onto the fractal [[consensus]] layers (see [[cyber/architecture]]):

- L0 (local): direct QUIC connections. aggregators receive cyberlinks from local neurons. massive bandwidth, no consensus overhead
- L1 (neighborhood): aggregators within geographic/semantic clusters coordinate. local BFT among ~10-100 nodes
- L2 (shard): cross-cluster aggregator reconciliation. shard-level state roots
- L3 (global): header chain only. recursive stark proofs. ~232 bytes per block. the 64 KB blockchain

the header market's geographic price differentiation — neighbors are cheaper — creates the same clustering that [[location proof]] formalizes. the network self-organizes into layers before anyone designs the layers.

see [[radio]] for the transport layer. see [[radio/gossip]] for the broadcast tree protocol. see [[radio/discovery]] for bootstrap mechanisms. see [[cyber/communication]] for private messaging and proof of delivery. see [[cyber/architecture]] for relay pricing and emergent hierarchy. see [[foculus]] for consensus over the header chain. see [[cyber/light]] for the light client that consumes this protocol