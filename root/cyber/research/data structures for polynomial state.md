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

| index | entries | record size | total |
|---|---|---|---|
| particles | 3M | 96 bytes | 288 MB |
| axons_out | 2.7M | 64 bytes | 173 MB |
| axons_in | 2.7M | 64 bytes | 173 MB |
| neurons | 70K | 56 bytes | 4 MB |
| locations | 70K | 104 bytes | 7 MB |
| coins | ~100 | 81 bytes | 8 KB |
| cards | ~10K | 104 bytes | 1 MB |
| files | ~100K | 76 bytes | 8 MB |
| time | ~1M snapshots | 72 bytes | 72 MB |
| total | ~12M entries | | ~726 MB |

at 10× growth (30M particles, 27M cyberlinks): ~7.3 GB
at 100× growth (300M particles): ~73 GB
at Avogadro scale: impossible on one machine — sharding required

## the access pattern

every block (~5 seconds):
- read: ~5000 random entries (validate signals, check nullifiers, read neuron states)
- write: ~1000 entries (update particles, axons, neurons for new cyberlinks)
- scan: 0-10 range queries (namespace completeness proofs, light client sync)
- commit: compute polynomial delta from ~1000 dirty entries

the key observation: reads dominate (5:1 ratio). writes are batched per block. scans are rare. commitment is pure computation (field arithmetic, not I/O).

## tier 1: in-memory (the state fits in RAM)

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
HOT STATE (current block, in RAM):
  flat array + Swiss table HashMap
  ~726 MB at current scale
  50 ns reads, 60 ns writes
  polynomial commitment: 32 bytes

WARM STATE (recent history, SSD):
  B+ tree with top-3-level RAM cache
  20-50 μs reads
  for: light client historical queries, replay

COLD STATE (full history, HDD/network):
  LSM with bloom filters in RAM
  8 ms per random read (avoided by bloom)
  for: archival, research analytics, deep replay

POLYNOMIAL COMMITMENT:
  always in RAM (32 bytes)
  delta update per block: O(|dirty|) field ops
  full recompute: O(N) field ops (for snapshot verification)
```

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

### near-term (bostrom scale, 726 MB)

```
store.rs:
  hot:        flat mmap'd arrays + Swiss table CID→idx
  persist:    mmap + WAL (fsync per block)
  commitment: polynomial delta, 32 bytes

  total: ~822 MB RAM, ~726 MB disk
  block time: ~100 μs (reads + writes + commitment)
```

### medium-term (10× growth, 7.3 GB)

```
same architecture, just more RAM
  total: ~8 GB RAM, ~7.3 GB disk
  still fits on any modern machine (16 GB minimum)
  block time: ~200 μs (more dirty entries per block)
```

### long-term (100× growth, 73 GB)

```
split:
  hot partition (current epoch, most-accessed):  flat array in RAM (~8 GB)
  warm partition (older state):                  B+ tree on SSD (~65 GB)

  reads: 95% hit hot partition (50 ns), 5% go to SSD (20 μs)
  weighted average: ~1 μs per read
  block time: ~5 ms
```

### planetary scale (10⁹ particles, 730 GB+)

```
sharding by namespace:
  each shard: one polynomial commitment for a namespace range
  cross-shard: batch PCS openings

  per-shard size: ~73 GB → B+ tree on SSD per shard
  total shards: ~10
  each shard: independent machine or CXL-attached memory pool
```

## the surprising conclusion

the optimal data structure for polynomial state is the SIMPLEST one that fits the hardware tier:

| state size | structure | why |
|---|---|---|
| < 8 GB | flat array + mmap | everything in RAM, no overhead needed |
| 8-64 GB | flat array (hot) + B+ tree (warm) | hot/warm split, SSD for overflow |
| 64 GB+ | B+ tree on SSD, sharded | B-tree minimizes SSD random I/O |
| archival | LSM on HDD | sequential access, bloom filters |

no novel data structure is needed. the polynomial commitment eliminates the TREE OVERHEAD (5 TB of Merkle nodes). what remains is the RAW DATA — and raw data is best stored in the simplest structure that matches the hardware.

the innovation is not in the data structure. it is in REMOVING the data structure. NMT added 5 TB of tree nodes. polynomial state removes them. what's left is a flat key-value store — the simplest possible thing.

trees are a tax we pay for authentication. polynomial commitments are a different authentication mechanism that doesn't require trees. the tax disappears.

see [[algebraic state commitments]] for the polynomial commitment mechanism. see [[BBG]] for the state layer specification. see [[Goldilocks field processor]] for how hardware acceleration of field arithmetic removes the remaining compute bottleneck. see [[cyber/research/provable consensus]] for why fast state access matters (tri-kernel in-circuit needs O(1) reads)
