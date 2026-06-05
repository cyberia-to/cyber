---
tags: cyber, research, consensus, foculus, cft
crystal-type: entity
crystal-domain: cyber
alias: foculus architecture, foculus consensus, CFT consensus
---
# foculus

graph-native, block-free consensus protocol implementing the [[collective focus theorem]].

## exponential optimality

the exponential allocation principle explains why base-*e* distributions appear in least-action physics, maximum-entropy thermodynamics, and attention economics.

CFT is a special case: group attention over competing items decays exponentially with rank.

optimal cognitive and computational systems should structure resource allocation to approximate base-*e* efficiency.

## cft in consensus

CFT applied to consensus enables probabilistic attention finality instead of block ordering.

nodes model the network as a token-weighted directed graph; repeated random walks converge to a stable stationary distribution (φ*).

transactions finalize when their φ*-value exceeds a threshold τ, guaranteeing safety under honest-majority focus.

millions of TPS, sub-3s finality, low communication overhead.

## protocol

- GPU-accelerated sparse matrix × vector updates every ~100ms
- particles = transactions/data; cyberlinks = weighted endorsements
- finality = φ*ᵢ > τ; conflicts below τ are discarded
- safety: ≥50% honest φ*-mass prevents double finality
- liveness: ergodicity ensures all honest transactions finalize

## economic model

minting rewards tied to Δφ*(p) — measurable shift in collective focus caused by a proof particle.

only focus updates mint; all other useful proofs rewarded from transaction fees.

fee split: 50% burned, 50% funds auxiliary proofs.

stake delegation = attention delegation; long-term reputation from accumulated φ*-weight.

eternal weight via burn anchors critical [[knowledge]] permanently.

## state model

graph-native state: particles (nodes) and cyberlinks (edges) as first-class citizens.

token-weighted attention determines significance.

hybrid architecture: integrates account-based deterministic execution with probabilistic, resource-based cognition.

suitable for AGI substrate — supports semantic emergence, modular knowledge, and scalable parallelism.

## data availability

tiered DA stack:
- tier 0: ethereum calldata checkpoints (immutable, minimal bandwidth)
- tier 1: active graph focus blobs on celestia, mirrored to IPFS/Filecoin
- tier 2: archival erasure-coded storage

phone-class light clients verify via DAS sampling; future-proofed for recursive brakedown proofs.

## authenticated graph

fully-authenticated focus cascading (FFC):
- merkelized subgraphs with path-hash accumulators
- fractional cascading overlays for O(log n) cross-shard lookups
- sharding: id-hash, neuron-centric, topic, community, geo/ownership, temporal, hybrid

every edge and weight shaping φ* is cryptographically verifiable.

## confidentiality

- 64-byte Blake3-XOF digests for quantum-resilient content addressing
- Pedersen commitments for weights: perfectly hiding, homomorphic
- Poseidon2 tags for zk efficiency
- default anonymization of node IDs; selective disclosure possible
- range proofs ensure bounded, valid weights

## sharding (v2)

two-tier commit:
- K committee shards each sign micro-roots
- beacon committee aggregates to a vector root

GPU-per-shard parallel focus computation.

throughput: up to 10⁷ links/s with K=50 while keeping per-node load constant.

safety: attack requires corrupting >1/3 committees and beacon in same slot.

## roadmap

1. prototype consensus and economic layers
2. integrate lattice-based quantum-resilient checkpoints
3. incentivize public-good computation
4. enhance DA and bundling
5. mainnet beta with adaptive RL optimization

---

CFT + exponential optimality + authenticated/confidential graph structures + GPU-native proof-weighted consensus = substrate for earth-scale decentralized [[superintelligence]].

scalable: 10⁶–10⁷ TPS class. secure: honest-majority φ*-mass safety. economically aligned: rewards tied to measurable contribution to collective cognition. trust-minimized: cryptographic proofs for all state. privacy-preserving: confidential by default.
