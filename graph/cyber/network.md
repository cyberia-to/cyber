---
tags: cyber, cip
crystal-type: pattern
crystal-domain: cyber
alias: network layer, p2p, peer-to-peer, cyber network
---
# network

how [[neurons]] find each other, propagate [[cyberlinks]], and maintain a shared view of the [[cybergraph]]. the network layer uses the [[cybergraph]] itself as its coordination substrate — peer discovery, topic routing, and reputation are subgraphs, not external systems.

## stack

```
┌─────────────────────────────────────┐
│  cyber/network                      │  gossip topology, peer discovery,
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

[[radio]] handles transport: QUIC connections, NAT hole-punching via [[radio/relay]], verified streaming via [[radio/bao]] (Hemera Merkle trees). [[cyber/communication]] handles privacy: onion routing, CSIDH key agreement, [[STARK]] proof of delivery. this page handles coordination: who connects to whom, how [[cyberlinks]] propagate, and how the network organizes itself.

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

the [[cybergraph]] resolution layer sits above Pkarr. Pkarr provides bootstrap — finding the first peers to connect to. once connected, the [[cybergraph]] provides the authoritative, stake-weighted peer directory. a [[neuron]]'s endpoint [[cyberlink]] is authenticated by their key, timestamped, and weighted by their stake. stale or fraudulent endpoint claims decay through standard [[forgetting]] mechanics.

## gossip: cyberlink propagation

when a [[neuron]] creates a [[cyberlink]], the link must reach all nodes that need it. [[radio/gossip]] provides the transport: topic-based publish/subscribe over epidemic broadcast trees (HyParView + PlumTree).

### topic structure

gossip topics partition the [[cybergraph]] into overlapping propagation domains. a topic is a 32-byte identifier. the topic structure mirrors the [[cybergraph]]'s own namespace organization:

| topic type | identifier | what propagates | who subscribes |
|---|---|---|---|
| global | `Hemera("cyberlinks/global")` | all new [[cyberlinks]] | full nodes, validators |
| namespace | `Hemera("cyberlinks/ns/" ∥ namespace)` | links within a namespace | nodes serving that namespace |
| neuron | `Hemera("cyberlinks/neuron/" ∥ pubkey)` | links by a specific neuron | followers, sync clients |
| particle | `Hemera("cyberlinks/particle/" ∥ cid)` | links involving a particle | watchers, query responders |

a full node subscribes to the global topic. a light client subscribes only to the namespaces and neurons it cares about. the topic structure enables selective sync without trusting the filtering node — the [[BBG]] completeness proof (§9.4 of the whitepaper) verifies that the filtered set is complete.

### propagation flow

```
Neuron creates cyberlink
        │
        ▼
signs link with neuron key
        │
        ▼
broadcasts to relevant gossip topics
        │
        ▼
epidemic broadcast tree propagates
(eager push on tree edges,
 lazy push on remaining links)
        │
        ▼
receiving nodes:
  1. verify signature
  2. verify neuron has sufficient focus for the link
  3. add to local cybergraph view
  4. update local tri-kernel state (incremental)
  5. forward to downstream subscribers
```

### propagation latency

the epidemic broadcast tree achieves O(log N) hop propagation for N subscribers. with QUIC transport over [[radio]], single-hop latency is dominated by network RTT. for a global network of ~10,000 full nodes:

- expected hops: ~13 (log₂ 10,000)
- per-hop latency: ~50-100ms (intercontinental QUIC)
- total propagation: ~0.4-1.3s to reach all full nodes

this is consistent with the [[foculus]] finality budget of 1-3s (§11.5 of the whitepaper).

## the cybergraph as its own routing table

the key insight: the [[cybergraph]] already encodes which [[neurons]] are interested in which [[particles]]. a [[neuron]] that has created many [[cyberlinks]] involving biology [[particles]] is likely interested in new biology links. the [[focus]] distribution $\pi^*$ over [[particles]] and [[neurons]] provides a natural routing metric.

### interest-based peering

nodes preferentially maintain connections to peers whose [[focus]] distributions overlap with their own. two nodes with high mutual information in their local $\pi^*$ views are likely to exchange useful [[cyberlinks]] — they attend to the same subgraph.

$$\text{peering\_affinity}(A, B) = \sum_{p \in P} \min(\pi^*_A(p), \pi^*_B(p))$$

this is the Bhattacharyya coefficient between the two nodes' focus distributions. high affinity means the nodes share attention on the same [[particles]]. the gossip layer uses this signal to maintain a partial view biased toward high-affinity peers, ensuring that relevant links arrive quickly.

### semantic routing

a query "what connects malaria to treatment?" does not flood the network. the querying node identifies high-[[focus]] [[particles]] in the malaria/treatment subgraph, finds [[neurons]] with high [[karma]] in that subgraph, and routes the query toward those neurons via the peering topology. the [[cybergraph]]'s own structure is the routing table.

```
query arrives
    │
    ▼
local node checks local cybergraph view
    │
    ├── sufficient data? → respond locally
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

## sybil resistance

the network layer inherits sybil resistance from the [[cybergraph]]'s stake-weighted structure:

- peer discovery via [[cybergraph]]: endpoint claims are stake-weighted. a sybil neuron with zero stake has zero weight in peer discovery
- gossip topic subscription: a node flooding a topic with invalid links burns its [[focus]] (link creation costs focus) and accumulates negative [[karma]] via [[Bayesian Truth Serum]] scoring
- relay economics: forwarding is reciprocal ([[BitTorrent]]-style tit-for-tat in the gossip layer). nodes that contribute nothing receive nothing. [[tokens]] handle the asynchronous cases

creating 1000 sybil [[neurons]] with zero stake produces zero influence on the network topology. the cost of disrupting the gossip layer is the cost of acquiring sufficient stake to create high-weight links — the same economic security bound as [[foculus]] consensus.

## consistency model

the network operates under partial synchrony: messages arrive within an unknown but finite bound $\Delta$.

### what is guaranteed

- safety: no conflicting finalized [[particles]] (from [[foculus]], §11.3)
- eventual consistency: every valid [[cyberlink]] eventually reaches all subscribed nodes
- completeness verification: a node can cryptographically verify that it has ALL links in a namespace via [[BBG]] completeness proofs

### what is not guaranteed

- real-time propagation: during network partitions, new links may be delayed
- ordered delivery: links may arrive out of creation order. the [[cybergraph]] uses timestamps and causal ordering, not delivery order

during asynchronous periods, no new [[particles]] finalize. existing finalized particles remain final. liveness resumes when connectivity restores. the gossip protocol's self-healing properties (HyParView maintains connectivity by replacing failed peers) ensure rapid recovery from partitions.

## bandwidth management

the gossip layer is the primary bandwidth consumer. bandwidth is managed through the relay primitive's pricing (see [[cyber/architecture]]):

- push (sender pays): broadcasting a new [[cyberlink]] costs relay fees proportional to message size
- pull (receiver pays): subscribing to a namespace topic costs the subscriber
- reciprocity: bilateral tit-for-tat handles most gossip exchange without on-chain settlement

[[focus]]-based prioritization: when bandwidth is scarce, nodes prioritize forwarding links from high-[[karma]] [[neurons]] targeting high-[[focus]] [[particles]]. low-priority links still propagate but with higher latency. the network's attention structure organizes its own traffic.

## connection to fractal architecture

as the network grows, the gossip topology naturally stratifies (see [[cyber/architecture]], fractal consensus):

- L0 (local): direct QUIC connections to nearby nodes. massive bandwidth, no consensus overhead
- L1 (neighborhood): gossip within geographic/semantic clusters. local BFT among ~10-100 nodes
- L2 (shard): cross-cluster gossip. shard-level state reconciliation
- L3 (global): header chain only. recursive STARK proofs. ~64 KB state

the gossip topology's emergent hub structure — driven by [[location proof]], relay economics, and [[focus]] dynamics — provides the empirical data for formalizing layer boundaries when the fractal architecture is deployed.

see [[radio]] for the transport layer. see [[radio/gossip]] for the epidemic broadcast tree protocol. see [[radio/discovery]] for bootstrap mechanisms. see [[cyber/communication]] for private messaging and proof of delivery. see [[cyber/architecture]] for relay pricing and emergent hierarchy. see [[foculus]] for consensus over the gossip layer
