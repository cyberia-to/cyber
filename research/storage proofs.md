---
tags: cyber, cip, cryptography
crystal-type: entity
crystal-domain: cyber
alias: storage proof, proof of storage, size proof, replication proof, retrievability proof, data availability proof
---
# storage proofs

six proof types that guarantee the [[cybergraph]] survives. without them, content-addressed identity is fragile — a hash with lost content is a dead [[particle]]. at planetary scale (10¹⁵ [[particles]]), content loss is the existential risk.

storage proofs are Phase 1 security infrastructure, not a Phase 3 optimization. they must be operational before genesis.

## the six proofs

```
PROOF TYPE          │ GUARANTEES                          │ MECHANISM
════════════════════╪═════════════════════════════════════╪══════════════════════════════════════
storage proof       │ content bytes exist on specific     │ periodic challenges against content
                    │ storage node                        │ hash — prover returns Hemera Merkle
                    │                                     │ path over challenged chunk
────────────────────┼─────────────────────────────────────┼──────────────────────────────────────
size proof          │ claimed content size matches        │ prover commits to Hemera tree depth
                    │ actual byte count                   │ and leaf count — verifier checks
                    │                                     │ tree structure against claimed size
────────────────────┼─────────────────────────────────────┼──────────────────────────────────────
replication proof   │ k independent copies exist          │ challenge k distinct replicas,
                    │ (k ≥ 3 before genesis)              │ verify uniqueness of storage
                    │                                     │ locations (no trivial mirroring)
────────────────────┼─────────────────────────────────────┼──────────────────────────────────────
retrievability      │ content can be fetched within       │ timed challenge-response with
proof               │ bounded time (not just "exists      │ latency bound — if content arrives
                    │ somewhere")                         │ late, proof fails
────────────────────┼─────────────────────────────────────┼──────────────────────────────────────
data availability   │ block data was published and is     │ 2D Reed-Solomon erasure coding +
proof (DAS)         │ accessible to all participants      │ random sampling (O(√n) samples
                    │                                     │ for 99.9% confidence)
────────────────────┼─────────────────────────────────────┼──────────────────────────────────────
encoding fraud      │ erasure coding was done correctly   │ obtain k+1 of 2k cells from a row,
proof               │ by the block producer               │ decode, compare against NMT root —
                    │                                     │ mismatch = fraud proof
```

storage proofs and replication proofs verify individual [[particle]] content. size proofs guarantee content dimensions — [[DAS]] proves data is accessible, but a [[particle]] claiming 1 MB that actually holds 10 bytes is undetectable without a size commitment. retrievability proofs add latency bounds. data availability proofs verify that batches of [[cyberlinks]] and state transitions were published and accessible. encoding fraud proofs catch dishonest block producers who encode data incorrectly.

## storage proof

the basic primitive. a storage node proves it holds the content behind a [[particle]] hash.

```
CHALLENGE-RESPONSE PROTOCOL:

  1. verifier picks random chunk index i from particle's Hemera tree
  2. prover returns chunk_i + Merkle path to tree root
  3. verifier checks:
     a) Hemera(chunk_i) matches leaf hash
     b) Merkle path validates against particle hash (tree root)
     c) response arrived within time bound

  cost: O(log n) hashes per challenge (n = chunks in particle)
  zheng constraints: ~5,000 per challenge
```

periodic challenges prevent lazy storage — a node that deletes content after initial proof will fail future challenges. challenge frequency is tunable per particle based on criticality.

## size proof

a [[particle]] hash commits to content identity — the same bytes always produce the same hash. it does not commit to content size. a storage node claiming "this particle is 500 MB" and charging storage fees accordingly is unverifiable from the hash alone. size proofs close this gap.

```
SIZE COMMITMENT:

  the Hemera tree already encodes size implicitly:
    - 4 KB chunks → leaf count = ⌈size / 4096⌉
    - binary tree → depth = ⌈log₂(leaf_count)⌉
    - tree structure is deterministic from content

  SIZE PROOF:
    1. prover commits: (particle_hash, claimed_size, tree_depth, leaf_count)
    2. verifier checks:
       a) leaf_count = ⌈claimed_size / 4096⌉
       b) tree_depth = ⌈log₂(leaf_count)⌉
       c) random chunk challenges confirm tree structure matches commitment
       d) last chunk padding consistent with claimed_size mod 4096

  cost: ~2,000 zheng constraints (tree structure check + padding verification)
```

size proofs matter for three reasons:

- storage pricing: [[neurons]] pay for storage proportional to size. inflated size claims extract unearned rewards from storage providers. deflated claims underpay
- bandwidth allocation: relay and retrieval protocols allocate bandwidth based on declared size. wrong size wastes network resources or enables denial of service
- [[erasure coding]]: [[DAS]] grid dimensions depend on content size. incorrect size breaks the 2D Reed-Solomon encoding — rows and columns do not align

size proofs compose with storage proofs: storage proves the bytes exist, size proves how many bytes exist. together they bind a [[particle]] to both its content and its dimensions.

## replication proof

k independent copies prevent single-point-of-failure. the protocol requires k ≥ 3 before genesis.

```
REPLICATION VERIFICATION:

  challenge k distinct storage nodes for the same particle
  verify:
    1. each returns valid storage proof
    2. storage locations are physically distinct (not trivial mirrors)
    3. at least k proofs succeed within the time bound

  uniqueness: derived from node identity + geographic attestation
  naive mirroring detection: challenge timing analysis (same rack = correlated latency)
```

replication proofs compose with storage proofs: each replica independently proves storage, and the aggregation proves redundancy.

## retrievability proof

storage existence is necessary but not sufficient. content that "exists" on a node but takes 30 minutes to retrieve is operationally lost. retrievability adds a time bound.

```
TIMED CHALLENGE-RESPONSE:

  1. verifier sends challenge at time t₀
  2. prover must return valid storage proof by t₀ + Δ_max
  3. if response arrives after deadline → proof fails
  4. Δ_max depends on content size and network conditions

  this distinguishes:
    - hot storage (SSD, in-memory): responds in milliseconds
    - cold archival (tape, glacier): may fail retrievability
    - offline/censored: fails completely
```

the retrievability proof turns a static property ("bytes exist") into an operational guarantee ("bytes are accessible when needed").

## data availability proof (DAS)

verifies that block data was published and is accessible to all participants. uses 2D Reed-Solomon [[erasure coding]] over [[Goldilocks field]] with [[NMT]] commitments.

```
2D ERASURE CODING:

  block data arranged in √n × √n grid
  each row erasure-coded: k data cells → 2k total cells (k parity)
  each column erasure-coded similarly

  any k of 2k values in a row → reconstructs the row
  any k of 2k values in a column → reconstructs the column

NAMESPACE-AWARE SAMPLING:

  light client interested in neuron N:
    1. NMT column root tells which rows contain namespace N
    2. sample random cells from those rows
    3. each sample carries:
       a) row NMT inclusion proof
       b) column NMT inclusion proof
       c) namespace proof
    4. O(√n) samples → 99.9% confidence all data is available

  the BBG's NMT structure enables this — namespace labels propagate
  through internal nodes. completeness is structural, not trusted.
```

see [[NMT]] for how namespace labels enable targeted sampling.

## encoding fraud proof

if a block producer encodes a row incorrectly:

```
FRAUD DETECTION:

  1. obtain k+1 of 2k cells from the suspect row
  2. attempt Reed-Solomon decoding over Goldilocks field
  3. if decoded polynomial ≠ claimed row NMT root:
     → fraud proof = the k+1 cells with their NMT proofs
     → any verifier checks: decode(cells) ≠ row commitment
     → block rejected

  proof size: O(k) cells with O(log n) proofs each
  verification: O(k log n) — linear in row size, logarithmic in block
```

encoding fraud proofs are the safety net for [[DAS]]: sampling gives probabilistic availability, but if a block producer cheats the encoding, anyone who detects it can produce a compact fraud proof that invalidates the block.

## layered data availability

data is tiered by criticality and expected lifetime:

```
┌──────────────────────────────────────────────────────────────────────────┐
│  Tier 0 — critical roots                                                │
│    checkpoint roots posted to high-security settlement layer             │
│    ~32-64 KB per epoch, immutable forever                               │
│    used for ultimate recovery and dispute resolution                    │
├──────────────────────────────────────────────────────────────────────────┤
│  Tier 1 — active graph                                                  │
│    focus blobs (~10K cyberlinks + proofs) posted to DA layer            │
│    retained ≥ 30 days, verified by light sampling on phones             │
│    the active working set of the cybergraph                             │
├──────────────────────────────────────────────────────────────────────────┤
│  Tier 2 — historical tails                                              │
│    erasure-coded archival to persistent storage networks                 │
│    refreshed by archivers, used for deep replay, research, rehashing    │
└──────────────────────────────────────────────────────────────────────────┘
```

## hash migration

the reason storage proofs must be Phase 1:

```
hash function may need replacement someday
  → replacement requires rehashing original content
    → rehashing requires content availability
      → content availability requires storage proofs
        → storage proofs must be operational before genesis
```

without storage proofs, the [[Hemera]] choice is irreversible and the system is permanently coupled to one hash function. with them, Hemera becomes a replaceable component — the correct architectural relationship.

```
HASH MIGRATION PROTOCOL:

  1. new identity space under new hash function (parallel, not replacing)
  2. rehash campaign retrieves content via storage proofs, computes new addresses
  3. dual-CID period: both old and new addresses valid
  4. cutoff: full coverage verified, new content requires new hash
     old CIDs become read-only historical references

  at 10¹⁵ particles ÷ 10⁶ nodes: ~17 hours for full parallel rehash
  bottleneck: storage proof coverage and network bandwidth
```

## genesis requirements

before genesis, the storage proof system must satisfy:

- coverage: every [[particle]] has at least k ≥ 3 verified replicas
- continuous verification: proofs checked periodically, not just at creation
- content-completeness: proofs verify actual content bytes, not just the CID
- retrievability: content fetchable within bounded time
- incentive alignment: [[neurons]] storing content earn rewards, penalized for loss

see [[cyber/proofs]] for the proof taxonomy, [[radio]] for the transport layer, [[NMT]] for namespace-aware sampling, [[BBG]] for the graph state architecture, [[data structure for superintelligence]] for the full DAS specification