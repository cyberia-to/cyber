---
tags: cyber, research, article, core
crystal-type: article
crystal-domain: cyber
date: 2026-03-24
---

# data structures for polynomial state

the design target is $10^{23}$ [[particles]]. at that scale, no machine, no datacenter, no company holds the full state. the architecture is DISTRIBUTED from line one. every participant stores a namespace shard and verifies the whole via 32 bytes.

this document derives storage architecture from [[Avogadro scale]] down to [[bostrom]], not the other way around.

## what we store: field elements

everything in [[BBG]] polynomial state is [[Goldilocks field]] elements ($\mathbb{F}_p$, 8 bytes each). record sizes in field elements:

| dimension | per-entry fields | field elements | bytes | what each field stores |
|---|---|---|---|---|
| particles (content) | CID + energy + φ* | 4 + 1 + 1 = 6 | 48 | identity (4 F_p hemera hash), aggregate inbound weight, tri-kernel focus |
| particles (axon) | CID + energy + φ* + weight + s_YES + s_NO + meta | 4 + 1 + 1 + 1 + 2 + 1 = 10 | 80 | above + conviction weight, ICBS market reserves, valence prediction |
| axons_out | source_CID + pointer | 4 + 1 = 5 | 40 | source particle identity, pointer to axon-particle |
| axons_in | target_CID + pointer | 4 + 1 = 5 | 40 | target particle identity, pointer to axon-particle |
| neurons | neuron_id + focus + karma + stake | 4 + 1 + 1 + 1 = 7 | 56 | identity, attention budget, BTS score, committed conviction |
| locations | neuron_id + coordinates + proof | 4 + 4 + 5 = 13 | 104 | identity, spatial position, location proof |
| coins | denom_hash + supply + params | 4 + 2 + 4 = 10 | 80 | denomination, total supply, mint curve parameters |
| cards | card_id + owner + metadata | 4 + 4 + 5 = 13 | 104 | NFT identity, ownership, properties |
| files | CID + availability + DAS_root | 4 + 2 + 4 = 10 | 80 | content identity, erasure coding params, DAS commitment |
| time | timestamp + BBG_snapshot | 1 + 4 = 5 | 40 | temporal coordinate, state snapshot reference |
| signals | signal_hash + height + proof_ref | 4 + 1 + 4 = 9 | 72 | finalized signal identity, block height, zheng proof |
| commitments | commitment_value | 4 | 32 | private record commitment (AOCL) |
| nullifiers | nullifier_value | 4 | 32 | spent record nullifier |

the dominant entries are particles and axons. at scale, ~60% of state is particles, ~30% is axon indexes, ~10% is everything else.

## scale table

| scale | particles | cyberlinks | neurons | state | where it lives |
|---|---|---|---|---|---|
| bostrom (now) | 3 × 10⁶ | 2.7 × 10⁶ | 1.2 × 10³ | ~500 MB | phone |
| village | 10⁷ | 10⁸ | 10⁴ | ~8 GB | laptop |
| city | 10⁹ | 10¹⁰ | 10⁵ | ~100 GB | server |
| nation | 10¹² | 10¹³ | 10⁷ | ~100 TB | cluster |
| planet | 10¹⁵ | 10¹⁶ | 10⁹ | ~100 PB | datacenter federation |
| solar system | 10¹⁸ | 10¹⁹ | 10¹² | ~100 EB | civilization infrastructure |
| Avogadro | 10²³ | 10²⁴ | 10¹⁵ | ~10 ZB | all participants |

at every scale: BBG_root = 32 bytes. verification = O(1) field ops. a phone verifies a claim about 10 ZB of state in 50 μs.

## the fundamental architecture: composable shards

the global polynomial is a COMPOSITION of shard polynomials:

$$\text{BBG\_root} = \text{compose}(C_1, C_2, \ldots, C_S)$$

each shard commits to a namespace range of the CID space $[0, 2^{256})$. composition is algebraic — O(S) field operations, not O(N) tree rebuilding. this is the property that [[hash tree|Merkle trees]] cannot provide: compose(root_A, root_B) requires rebuilding the tree from leaves. compose($C_A$, $C_B$) is one field operation.

### shard sizing

| scale | shards | state per shard | local data structure |
|---|---|---|---|
| bostrom | 1 | 500 MB | flat array in RAM |
| village | 1 | 8 GB | flat array in RAM |
| city | 10 | 10 GB | flat array in RAM (16 GB machine) |
| nation | 10³ | 100 GB | B+ tree on SSD |
| planet | 10⁶ | 100 GB | B+ tree on SSD per shard node |
| solar system | 10⁹ | 100 GB | B+ tree on SSD, replicated |
| Avogadro | 10¹² | 10 GB | flat array in RAM (each shard small enough!) |

the surprising result at Avogadro: 10²³ particles across 10¹² shards = 10¹¹ particles per shard = ~10 GB per shard. this FITS IN RAM on any modern machine. the problem at Avogadro scale is not storage per node — it is COORDINATION across 10¹² shards.

### per-node storage

a [[neuron]] does not store all shards. it stores:

```
my namespace:    particles I created, my neuron record         ~1 KB - 1 GB
my neighborhood: shards I query frequently (cached)            ~1 GB - 100 GB
proof cache:     Lens openings from remote shards (LRU)         ~100 MB
BBG_root:        global commitment                             32 bytes
```

a neuron on a phone: ~1 GB total. stores its own data, caches neighbors, verifies everything else via 32-byte root.

a validator: stores multiple shards. more shards = more responsibility = more reward (proof-of-storage incentive from [[DAS]] + φ*-weighted replication).

## within a shard: the local data structure

each shard is a self-contained polynomial sub-state. the choice of local data structure depends on ONE variable: does the shard fit in RAM?

### if shard fits in RAM: flat array + hash index

```
CID → compact_index:  HashMap<[u8;32], u32>    40 ns lookup
state[dimension][idx]: &[FieldElement]           10 ns access
dirty:                 BitVec                    O(1) mark

read:    50 ns
write:   60 ns + WAL append
commit:  O(|dirty|) field ops for polynomial delta
```

no tree. no B-tree. no LSM. direct memory access. the simplest possible structure because polynomial commitment handles authentication externally.

persistence: mmap the arrays. WAL for crash recovery. fsync once per block.

### if shard exceeds RAM: B+ tree on SSD

```
fanout:   4 KB page / 48 bytes per particle entry ≈ 83 children per node
depth:    log₈₃(10¹¹) = 5.7 → 6 levels at nation-scale shard
top 3-4 levels: pinned in RAM (~400 MB)
leaf reads: 2 SSD reads per lookup = 20 μs

random read:   20 μs (warm cache, 2 SSD reads)
batch write:   sort dirty entries → sequential merge → 100 μs for 1000 entries
range scan:    sequential leaf traversal → SSD bandwidth limited (~3 GB/s)
```

B+ tree is the right structure for "too big for RAM, random access needed" since 1970. nothing has improved on it for this access pattern.

### archival (full history, HDD or network): sorted log + NMT

here is where [[NMT]] returns — not for authentication but for DISK ACCESS OPTIMIZATION:

```
historical state: append-only log sorted by namespace
NMT index:        sorted namespace ranges → file offsets

advantage: namespace query = sequential scan of sorted region
           HDD sequential: 200 MB/s
           HDD random:     0.1 MB/s (8 ms seek)
           ratio: 2000×

NMT's sorted invariant = optimal disk layout for namespace queries
```

NMT on archival tier is a STORAGE INDEX, not a TRUST MECHANISM. the polynomial commitment authenticates. the NMT organizes bytes on slow disks for sequential access. two different jobs.

## the complete per-node architecture

```
┌──────────────────────────────────────────────┐
│ POLYNOMIAL LAYER (authentication)             │
│                                              │
│ BBG_root = compose(C₁, C₂, ..., C_S)        │
│ 32 bytes. O(1) verify. scale-invariant.      │
│                                              │
│ my_shard_commitment: 32 bytes                │
│ update: O(|dirty|) field ops per block       │
│ cross-shard verify: O(1) Lens opening         │
└──────────────────┬───────────────────────────┘
                   │
┌──────────────────▼───────────────────────────┐
│ HOT STATE (current, in RAM)                   │
│                                              │
│ flat array + HashMap index                   │
│ my shard's current entries                   │
│ read: 50 ns, write: 60 ns                   │
│ fits: up to 64 GB per shard                  │
│                                              │
│ persistence: mmap + WAL (fsync per block)    │
└──────────────────┬───────────────────────────┘
                   │
┌──────────────────▼───────────────────────────┐
│ WARM STATE (recent history, SSD)              │
│                                              │
│ B+ tree with RAM-cached top levels           │
│ historical queries: "state at time T"        │
│ read: 20 μs, range scan: SSD bandwidth      │
└──────────────────┬───────────────────────────┘
                   │
┌──────────────────▼───────────────────────────┐
│ COLD STATE (full history, HDD/network)        │
│                                              │
│ sorted log + NMT index for disk layout       │
│ NMT = storage optimization, not trust        │
│ namespace scan: sequential 200 MB/s          │
│ for: archival, research, deep replay         │
└──────────────────────────────────────────────┘
```

## the hardware evolution mapping

| storage tier | era | optimal local structure | authentication | notes |
|---|---|---|---|---|
| HDD | 1970s-2010s | B-tree / NMT (minimize seeks) | Merkle tree (tree serves dual role: index + auth) | tree tax paid once |
| SSD | 2010s-2020s | B+ tree with RAM cache | Merkle or polynomial | tree for index, polynomial for auth |
| RAM | 2020s | flat array | polynomial (32 bytes) | tree unnecessary — direct addressing |
| persistent memory (CXL) | 2030s | flat array, no WAL | polynomial (32 bytes) | even WAL unnecessary |
| [[Goldilocks field processor|GFP]] + RAM | target | flat array, commitment in hardware | polynomial (field ops in silicon) | data structure disappears |

the trend: as storage gets faster, data structures get simpler. trees are compensating mechanisms for slow storage. when access is O(1) (RAM), the tree adds cost without benefit. the polynomial commitment provides authentication without requiring any particular data layout.

GFP is the endgame: field arithmetic in silicon + flat array in RAM. the data structure literally disappears. what remains: memory addresses and polynomial evaluations. no nodes, no pointers, no pages, no seeks. bytes and math.

## what NMT actually is (reframed)

NMT was designed as an authentication structure (completeness proofs via sorted invariant). in polynomial state, authentication is handled by lens.

but NMT's sorted namespace property has a second life: optimal disk layout for cold storage. sorted data = sequential reads = fast on slow disks.

| role | polynomial state | NMT |
|---|---|---|
| authentication (trust) | primary: Lens opening, O(1) | unnecessary |
| hot storage index (RAM) | flat array | unnecessary |
| warm storage index (SSD) | B+ tree | unnecessary (B+ tree is better for random access) |
| cold storage layout (HDD) | not designed for disk | useful: sorted namespace = sequential scan |
| DAS chunk organization | lens-based algebraic DAS | useful: namespace-sorted chunks |

NMT survives in the architecture — not where it was designed to be (authentication), but where its sorted property happens to match the hardware (cold storage, DAS chunk layout).

## implications for BBG implementation

bbg/src/store.rs should implement three storage backends behind one trait:

```rust
trait ShardStore {
    fn get(&self, dimension: u8, key: &[u8; 32]) -> Option<&[FieldElement]>;
    fn put(&mut self, dimension: u8, key: &[u8; 32], value: &[FieldElement]);
    fn dirty_entries(&self) -> impl Iterator<Item = (u8, [u8; 32], &[FieldElement])>;
    fn commit(&mut self) -> [u8; 32];  // returns shard sub-commitment
}

// implementations:
struct FlatArrayStore { ... }    // RAM: mmap + HashMap + BitVec
struct BPlusTreeStore { ... }    // SSD: B+ tree with RAM cache
struct ArchivalStore { ... }     // HDD: sorted log + NMT layout index
```

the polynomial commitment logic is ABOVE the store — it consumes dirty entries and produces the 32-byte sub-commitment. the store doesn't know about polynomials. the polynomial doesn't know about storage. clean separation.

cross-shard composition happens at the network layer — above both store and polynomial. each node: store → polynomial → network → global BBG_root.

see [[algebraic state commitments]] for the polynomial commitment mechanism. see [[BBG]] for the state specification. see [[NMT]] for namespace Merkle trees (now reframed as cold storage optimization). see [[DAS]] for data availability sampling. see [[Goldilocks field processor]] for the hardware endgame. see [[cyber/research/provable consensus]] for why O(1) state reads matter
