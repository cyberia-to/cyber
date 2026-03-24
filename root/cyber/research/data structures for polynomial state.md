---
tags: cyber, research, article, core
crystal-type: article
crystal-domain: comp
date: 2026-03-24
---

# data structures for polynomial state

the [[cybergraph]] must be stored, indexed, and queried. the polynomial commitment ([[algebraic state commitments]]) authenticates the data — but the DATA must live somewhere. this document analyzes every known data structure family for storing polynomial-committed state across three tiers: in-memory (nanoseconds), SSD (microseconds), and HDD (milliseconds).

## what we are storing

the [[BBG]] state at [[bostrom]] scale:

| index | entries | record size | total | notes |
|---|---|---|---|---|
| particles | 3M | 128 bytes (4 × F_p CID + energy + π* + flags) | 384 MB | CID = 4 field elements |
| axons_out | 2.7M | 64 bytes | 173 MB | |
| axons_in | 2.7M | 64 bytes | 173 MB | |
| neurons | 70K | 56 bytes | 4 MB | |
| locations | 70K | 104 bytes | 7 MB | |
| coins | ~100 | 81 bytes | 8 KB | |
| cards | ~10K | 104 bytes | 1 MB | |
| files | ~100K | 76 bytes | 8 MB | |
| time | ~1M snapshots | 72 bytes | 72 MB | |
| total | ~12M entries | | ~822 MB | |

these are [[bostrom]] numbers — the bootloader. the design target is [[Avogadro scale]]:

| scale | particles | cyberlinks | state size | fits on |
|---|---|---|---|---|
| bostrom (now) | 3 × 10⁶ | 2.7 × 10⁶ | ~822 MB | phone |
| city (10³ neurons, dense) | 10⁹ | 10¹⁰ | ~128 GB | server |
| planet (10⁶ neurons) | 10¹⁵ | 10¹⁶ | ~128 PB | datacenter cluster |
| solar system | 10¹⁸ | 10¹⁹ | ~128 EB | civilization-scale |
| Avogadro | 10²³ | 10²⁴ | ~12.8 ZB | distributed across all participants |

no single machine holds the full state past city scale. the data structure must be DISTRIBUTED from the design, not "sharded later." every participant stores a namespace shard and verifies the whole via polynomial commitment.

## the design constraint: Avogadro from day one

the architecture cannot assume "fits on one machine." at 10²³ particles, no machine, no datacenter, no company holds the full state. the state is INHERENTLY distributed across all participants.

this changes everything about data structure choice:

- there is no "hot state in RAM" for the full graph
- every node stores a SHARD (its namespace neighborhood)
- the polynomial commitment authenticates the WHOLE (32 bytes regardless of scale)
- proof = PCS opening: "this value at this position in this shard is consistent with the global commitment"
- a phone storing 1 MB of its neuron's state can verify claims about 12.8 ZB of global state

the access pattern PER NODE:

| operation | what | frequency | latency budget |
|---|---|---|---|
| local read | entries in my namespace shard | ~5000/block | nanoseconds (RAM) |
| local write | update my particles, my axons | ~1000/block | nanoseconds (RAM) |
| remote verify | PCS opening proof from another shard | ~100/block | microseconds (network + verify) |
| commitment delta | update my shard's sub-commitment | 1/block | microseconds (field arithmetic) |
| global commitment | compose all shard sub-commitments | 1/epoch | milliseconds (aggregate) |

the key insight: reads and writes are LOCAL (to my shard, in my RAM). verification is GLOBAL (any shard, via PCS opening). the data structure optimizes local access. the polynomial optimizes global verification.

## the distributed polynomial: shard commitments compose

the global BBG_poly is not one giant polynomial. it is a composition of shard polynomials:

$$\text{BBG\_root} = \text{compose}(C_1, C_2, \ldots, C_S)$$

each shard $C_i$ commits to a namespace range. the composition is a polynomial operation (not a hash tree merge). properties:

- update shard $C_i$: O(|dirty_i|) field ops (local)
- compose all shards: O(S) field ops where S = number of shards
- cross-shard proof: open $C_i$ at point + compose proof against BBG_root
- verify: O(1) field ops (same as single-shard)

sharding by namespace is natural: particles are CID-addressed (uniformly distributed hash space). split hash space into ranges. each range = one shard. each shard = one node (or redundant set of nodes).

```
shard assignment:
  CID space: [0, 2²⁵⁶)
  S shards: each covers [i × 2²⁵⁶/S, (i+1) × 2²⁵⁶/S)
  neuron stores: shards containing its particles + shards it queries frequently

at Avogadro (10²³ particles, S = 10⁶ shards):
  particles per shard: 10¹⁷
  state per shard: ~12.8 TB
  → each shard is a serious machine (or cluster)

at planet (10¹⁵ particles, S = 10³ shards):
  particles per shard: 10¹²
  state per shard: ~128 GB
  → one server per shard

at city (10⁹ particles, S = 10 shards):
  particles per shard: 10⁸
  state per shard: ~12.8 GB
  → one machine holds several shards
```

## within a shard: the local data structure

each shard is a self-contained polynomial state. the data structure question is: how does a shard store its entries locally?

## tier 1: in-memory (shard fits in RAM)

when total state < available RAM (currently 726 MB, fits on any machine with 2+ GB):

### flat array + hash index

```
CID → compact_index:  HashMap<[u8;32], u32>     O(1) lookup, ~96 MB at 3M entries
state[index][idx]:    &[Record]                  O(1) access, ~726 MB
dirty:                BitVec                      O(1) mark, 375 KB at 3M entries

read:    hash(CID) → idx → state[index][idx]     ~50 ns
write:   same + mark dirty                        ~60 ns
commit:  iterate dirty bits → field arithmetic    ~21 μs for 1000 entries
```

this is the fastest possible. no tree traversal. no serialization. no page management. direct memory access.

persistence: mmap the arrays to files. OS handles page-out transparently. WAL for crash recovery. fsync once per block.

| metric | value |
|---|---|
| random read | 50 ns |
| random write | 60 ns |
| block commit (1000 entries) | 100 μs |
| memory footprint | ~822 MB (state + index) |
| disk footprint | ~726 MB (state files) + WAL |

### when this breaks

at 73 GB (100× growth): needs 73 GB RAM. expensive but available on servers.
at 730 GB (1000× growth): exceeds most machine RAM. need tier 2.

### hash index alternatives

| structure | lookup | insert | memory overhead | when to use |
|---|---|---|---|---|
| HashMap (open addressing) | 50 ns avg | 60 ns avg | 2× key storage | default: fast, simple |
| BTreeMap | 200 ns (cache-friendly) | 300 ns | 1.3× key storage | if sorted iteration needed |
| perfect hash (CHD/MPHF) | 30 ns | rebuild O(n) | 2-3 bits/key | if keys are static (snapshot) |
| cuckoo hash | 40 ns worst case | amortized 60 ns | 1.05× | if worst-case latency matters |
| Swiss table (hashbrown) | 40 ns avg | 50 ns avg | 1.5× | Rust default, SIMD-optimized |

for polynomial state: Swiss table (Rust's default HashMap) is near-optimal. perfect hash would be 20% faster for reads but requires rebuild on every new particle — not worth it for a growing set.

## tier 2: SSD (state exceeds RAM)

when state > RAM, we need a structure that minimizes SSD random I/O. SSD characteristics:

```
sequential read:    3-7 GB/s (NVMe)
random read (4K):   ~10 μs per page (~500K IOPS)
sequential write:   2-5 GB/s
random write:       ~20 μs per page
page size:          4 KB (OS) or 512 bytes (NVMe native)
```

the challenge: a random read costs 10 μs on SSD vs 50 ns in RAM — 200× slower. minimizing random I/O is everything.

### B+ tree (the classical answer)

```
internal nodes: keys + pointers to children
leaf nodes:     keys + values (records)
fanout:         4 KB page / 40 bytes per entry = ~100 children per node
depth at 300M:  log_100(300M) = 4.2 → 5 levels

random read:    5 page reads × 10 μs = 50 μs
                BUT: top 2-3 levels fit in RAM → 2 page reads → 20 μs
batch write:    sort + merge into leaf pages → sequential I/O
range scan:     follow leaf-level linked list → sequential
```

| metric | value |
|---|---|
| random read (warm cache) | 20-30 μs (2 SSD reads) |
| random read (cold) | 50 μs (5 SSD reads) |
| batch write (1000/block) | ~100 μs (sequential, WAL + merge) |
| space amplification | ~1.5× (internal nodes) |
| range scan | sequential I/O — fast |

B+ tree is the RIGHT structure for SSD when state doesn't fit in RAM. every database uses it for a reason. the top levels stay in RAM; only leaf reads hit SSD.

### LSM tree (write-optimized)

```
writes:  all go to in-memory memtable → flush to L0 when full
reads:   check memtable → L0 → L1 → ... (multiple SSD reads)
compaction: merge sorted runs → write amplification

random read:    20-100 μs (check 3-5 levels)
batch write:    ~1 μs (memtable insert, amortized)
compaction:     10-30× write amplification
space:          1.1-1.5× with bloom filters
```

LSM is optimized for write-heavy workloads (>100K writes/sec). our workload is 1000 writes per 5-second block = 200 writes/sec. LSM's write optimization is wasted; its read penalty is paid. B+ tree wins for our access pattern.

### fractal tree (TokuDB/Beringei)

```
hybrid B-tree + message buffers
random read:    similar to B+ tree (slightly worse)
batch write:    better than B+ tree (message batching)
space:          similar
```

marginal improvement over B+ tree for our workload. not worth the complexity.

### radix/trie (Patricia, ART)

```
depth:          key length (32 bytes for CID = 32 levels in naive trie)
read:           32 pointer chases worst case → terrible cache behavior
ART (adaptive radix tree): reduces depth via node type adaptation
                4-8 pointer chases → 80-160 ns in RAM, 40-80 μs on SSD
```

worse than B+ tree for CID keys (long, pseudo-random). tries are good for prefix-shared keys (IP addresses, strings). CID hashes have no shared prefixes.

### sorted arrays + binary search

```
read:    binary search over sorted entries → log₂(300M) = 28 comparisons
         each comparison = 1 random read at cold → 28 × 10 μs = 280 μs (terrible)
         with page-aligned blocks → ~5 reads → 50 μs (same as B+ tree)
write:   requires re-sorting or auxiliary structure
```

effectively the same as B+ tree but with worse write performance. no advantage.

### the answer for SSD

B+ tree with hot node caching. top 3 levels (~100K nodes, ~400 MB) pinned in RAM. leaf reads hit SSD. 2 SSD reads per random lookup = 20 μs.

## tier 3: HDD (archival, historical state)

HDD characteristics:

```
sequential read:    150-250 MB/s
random read (4K):   ~8 ms per seek (seek + rotation)
sequential write:   150-250 MB/s
random write:       ~10 ms
```

random reads are 1000× slower than SSD. the ONLY viable strategy: make everything sequential.

### log-structured storage

```
state as append-only log segments
reads: index in RAM points to (segment_file, offset)
       one HDD seek per read → 8 ms
       BUT: batch reads by sorting requests → sequential scan → 150 MB/s

for 5000 reads per block:
  random:     5000 × 8 ms = 40 seconds (impossible in 5-second block)
  batched:    sort by file position → ~3 sequential scans → ~50 ms (feasible)
```

### LSM on HDD

actually makes sense here — writes are sequential (append to WAL + memtable). reads go through bloom filters (in RAM) to avoid HDD seeks. compaction is sequential merge.

| metric | value |
|---|---|
| random read (bloom filter hit) | 0 μs (answer from RAM: absent) |
| random read (bloom filter miss) | 8 ms (1 HDD seek) |
| batch read (sorted) | 50 ms for 5000 entries |
| write | ~1 μs (memtable, amortized) |

### the answer for HDD

LSM with large bloom filters in RAM. bloom filters (2M entries × 10 bits = 2.5 MB per level) stay in RAM. most "is this key present?" queries answered without touching disk. actual data reads batched and sorted for sequential access.

but honestly: HDD is for archival/historical state, not hot path. active consensus should never run on HDD.

## the complete storage architecture

```
PER-NODE (each participant):

  MY SHARD (hot, in RAM or SSD depending on shard size):
    flat array + HashMap    if shard < RAM
    B+ tree on SSD          if shard > RAM
    polynomial sub-commitment: 32 bytes

  NEIGHBOR CACHE (warm, for frequent cross-shard queries):
    LRU cache of PCS openings from other shards
    no trust: every cached value has a proof

  HISTORY (cold, SSD or HDD):
    append-only log of my shard's state changes
    B+ tree for historical queries by time
    for: replay, analytics, archival

GLOBAL (network-wide):

  BBG_root = compose(C₁, C₂, ..., C_S)
  32 bytes regardless of total state size
  every node knows BBG_root (from consensus)
  cross-shard verification: O(1) field ops per opening
```

the critical property: a node holding 1 MB of its own namespace can verify claims about the ENTIRE graph via BBG_root. the polynomial commitment is scale-invariant — verification cost is O(1) whether the graph has 3M or 10²³ particles.

## the relationship to hardware evolution

the [[Goldilocks field processor|GFP]] optimizes COMPUTATION: fma, ntt, p2r, lut. these are jet suite operations — fixed function silicon for field arithmetic.

data structures optimize STORAGE: how bytes are laid out in memory, on SSD, on spinning disk. these evolve with storage technology:

| era | storage | optimal structure | bottleneck |
|---|---|---|---|
| 1970s | HDD only | B-tree (minimize seeks) | seek latency |
| 2000s | HDD + RAM | LSM (buffer writes in RAM) | write amplification |
| 2010s | SSD | B+ tree with RAM cache | random I/O amplification |
| 2020s | large RAM | flat array (everything in memory) | memory bandwidth |
| 2030s | persistent memory (CXL, PMEM) | flat array (no persistence distinction) | compute |

the trend: as storage becomes faster, data structures become SIMPLER. B-trees exist because disks are slow. when everything is in memory, a flat array wins. when persistent memory eliminates the RAM/disk distinction, even the WAL becomes unnecessary.

GFP + flat array polynomial state is the endgame architecture:
- GFP: field arithmetic at hardware speed (jets)
- flat array: data access at memory speed (no tree overhead)
- polynomial commitment: authentication at field-op speed (no hash overhead)

the data structure disappears. what remains: memory and math.

## implications for BBG

### bostrom (3M particles, 822 MB) — phone

```
one node holds everything: flat array + mmap
100 μs per block. polynomial delta: 21 μs.
even a phone can be a full validator.
```

### city (10⁹ particles, 128 GB) — server

```
10 shards, each ~12.8 GB
per-shard: flat array in RAM (if 16+ GB machine) or B+ tree on SSD
cross-shard: PCS openings over network (~1 ms RTT + verify)
block time: ~5 ms (dominated by cross-shard queries)
```

### planet (10¹⁵ particles, 128 PB) — datacenter cluster

```
1000 shards, each ~128 GB → one server per shard
per-shard: B+ tree on SSD with RAM cache
cross-shard: dedicated network fabric, batched PCS openings
block time: ~50 ms (network-dominated)
each neuron: stores its own namespace (~1-100 GB) + caches neighbors
```

### Avogadro (10²³ particles, 12.8 ZB) — all participants

```
10⁶ shards, each ~12.8 TB
per-shard: B+ tree on SSD cluster (or distributed across shard participants)
cross-shard: internet-scale, locality-aware routing
block time: ~1 second (speed of light limited for global consensus)
BBG_root: still 32 bytes. verification: still O(1) field ops.
a phone verifying a claim about 10²³ particles: 50 μs.
```

## the conclusion

the data structure problem for polynomial state has two parts:

### within a shard: match the hardware

| shard size | structure | why |
|---|---|---|
| < RAM | flat array + mmap | direct memory access, 50 ns reads |
| < SSD | B+ tree with RAM cache | minimize SSD random reads, 20 μs |
| < disk cluster | B+ tree distributed | partition across disks |

this is not novel. it is the simplest structure that fits the storage tier.

### across shards: polynomial composition

this IS novel. the polynomial commitment composes across shards:

$$\text{BBG\_root} = \text{compose}(C_1, \ldots, C_S)$$

each shard updates independently. composition is O(S) field ops. verification is O(1) regardless of total state size. no Merkle tree aggregation. no root recomputation from all leaves. the global state is authenticated by 32 bytes that compose algebraically.

the innovation is not in the local data structure (B+ tree, flat array — these are 50-year-old solutions). the innovation is in the AUTHENTICATION that makes local data globally verifiable. polynomial commitments compose. hash trees do not (merging two Merkle roots requires rebuilding the tree). this is why polynomial state scales to Avogadro and Merkle state does not.

trees are a tax we pay for hash-based authentication. polynomial commitments are a different authentication that composes algebraically. the tax disappears. the local data structure becomes whatever is fastest for the hardware. the global data structure is 32 bytes.

see [[algebraic state commitments]] for the polynomial commitment mechanism. see [[BBG]] for the state layer specification. see [[Goldilocks field processor]] for how hardware acceleration of field arithmetic removes the remaining compute bottleneck. see [[cyber/research/provable consensus]] for why fast state access matters (tri-kernel in-circuit needs O(1) reads)
