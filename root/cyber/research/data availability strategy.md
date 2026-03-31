---
tags: cyber, research, article
crystal-type: process
crystal-domain: cyber
date: 2026-03-23
---

# data availability: from sampling to algebraic proofs

how [[cyber]] guarantees that data physically exists without downloading it. layer 4 of [[structural sync]]. the mechanism that closes the gap between "the data is committed" and "the data is retrievable."

## the problem

a node commits to data (posts a hash, signs a root). the hash proves integrity — if you have the data, you can verify it matches. but the hash says nothing about whether the data is AVAILABLE. the committer could publish the root and discard the payload. every verifier sees a valid commitment to data nobody can retrieve.

this is the data availability problem. it is independent of consensus, ordering, and validity — a system can have perfect proofs of correct computation while the underlying data is lost.

## the solution: erasure coding + sampling

### 2D Reed-Solomon encoding

data is arranged in a $\sqrt{n} \times \sqrt{n}$ grid and extended with parity rows and columns via Reed-Solomon over [[Goldilocks field]]:

```
original: k × k data cells
extended: 2k × 2k (data + parity)
any k × k submatrix → full reconstruction
default rate 1/2: 2× storage overhead, distributed across N devices
```

withholding >50% makes reconstruction fail — but random sampling detects it exponentially fast. withholding <50% means the data is recoverable from parity anyway. binary outcome: available or detectable.

### data availability sampling (DAS)

a verifier samples O(√n) random cells, each with an [[NMT]] inclusion proof against the committed root:

```
for i in 1..k:
  pos ← random_cell()
  (cell, proof) ← request(pos)
  verify: NMT_proof(cell, proof, root)

k = 20: confidence 1 - (1/2)^20 = 99.9999%
k = 30: confidence 1 - (1/2)^30 = 99.99999999%
```

a phone verifies 1 TB of data with ~30 random samples. no full download. no trust in the data holder.

### fraud proofs for bad encoding

if a block producer encodes data incorrectly (wrong Reed-Solomon), any node that obtains k+1 cells from one row can detect it:

```
1. download k+1 cells from row
2. Reed-Solomon decode → expected polynomial
3. check: decoded polynomial matches committed row root?
4. if not → fraud proof (k+1 cells + proofs)
5. verification: O(k log n) — linear in row, logarithmic in block
```

## three scales

the same mechanism works at every scale. only parameters change:

| scale | participants | distribution | commitment | fraud handling |
|---|---|---|---|---|
| local (devices) | 1-20 devices of one [[neuron]] | capacity-weighted | per-device [[NMT]] | key revocation |
| global ([[neurons]]) | 10³-10⁶ neurons | [[stake]]-weighted | per-neuron NMT | stake slashing |
| query (light clients) | 10⁹ clients | pull-based | [[BBG]] root | proof rejection |

at local scale: a phone verifies its server has all files without downloading them. at global scale: light clients verify block data availability without running a full node.

## local scale: personal device erasure coding

a [[neuron]] with n devices and data of size D wants to survive f device failures without storing D on every device. erasure coding (k, n) where k = n − f gives the minimum: each device stores D/k.

| devices (n) | tolerated losses (f) | k = n−f | per device | total across devices | overhead vs full replication |
|---|---|---|---|---|---|
| 3 | 1 | 2 | D/2 | 1.5 × D | 2× less than 3-copy |
| 5 | 1 | 4 | D/4 | 1.25 × D | 4× less than 5-copy |
| 5 | 2 | 3 | D/3 | 1.67 × D | 3× less than 5-copy |
| 10 | 2 | 8 | D/8 | 1.25 × D | 8× less than 10-copy |

combined with tiering by criticality (same tiers as global DA, scaled to personal use):

```
tier 0 — keys, seeds, irreplaceable documents
  full replication on every device. small (<1 GB). loss = identity death.

tier 1 — active working files
  (k, n) erasure coding. each device holds D₁/k.
  frequent access, low latency required.

tier 2 — archive, media, old projects
  (k, n) erasure coding. cold storage acceptable.
  bulk of data lives here. retrieval latency in seconds is fine.
```

for a neuron with 3 devices and 100 GB:

```
tier 0:   5 GB × 3 copies  =  15 GB total, 5 GB per device
tier 1:  30 GB ÷ k=2       =  45 GB total, 15 GB per device
tier 2:  65 GB ÷ k=2       =  97 GB total, ~33 GB per device
─────────────────────────────────────────────────────────────
total:                        157 GB total, ~53 GB per device
vs full 3× replication:       300 GB total, 100 GB per device
```

### transparent fetch

a device that holds chunk i but needs chunk j (stored on a peer device) fetches it on demand. the flow:

```
1. local read for file F
2. check: do I hold the chunk(s) for F?
   → yes: serve from local storage
   → no:  query peer devices for missing chunk(s)
3. peer returns chunk + NMT inclusion proof
4. verify chunk against local commitment root
5. reconstruct file from k chunks (local + fetched)
6. optionally cache reconstructed file locally (LRU eviction)
```

the key property: the local commitment root is the same across all devices. a fetched chunk is verified cryptographically — no trust in the peer device, only in the math. a compromised device cannot serve bad chunks undetected.

cache policy matters: frequently accessed files that live on another device should migrate locally (hot promotion). rarely accessed local files can be evicted to parity-only (cold demotion). the device set collectively maintains the invariant that k of n chunks exist for every file — which chunks live where is a local optimization.

### rechunking on configuration change

when a device joins or leaves, the (k, n) parameters change and chunks must be redistributed. the cost depends on the change type:

```
DEVICE JOINS (n → n+1):
  option A — keep k, increase redundancy (f+1 tolerated failures)
    generate 1 new parity chunk per file, send to new device
    cost: O(D/k) encoding + O(D/k) network transfer
    no existing device re-encodes. only parity generation for the new member.

  option B — increase k (keep f, reduce per-device storage)
    full re-encode: split data into k+1 chunks instead of k
    cost: O(D) encoding + O(D) redistribution
    every device gets a smaller chunk set. storage per device drops.

DEVICE LEAVES (n → n-1):
  if f > 1: reduce f by 1. no rechunking needed. system still survives f-1 losses.
  if f = 1: critical — must rechunk to restore fault tolerance.
    reconstruct lost chunks from remaining k devices: O(D/k) decoding.
    re-encode with new (k', n-1) parameters.
    cost: O(D) encoding + O(D/(n-1)) transfer to each remaining device.

PRACTICAL NUMBERS (100 GB, 3 devices → 4 devices, option A):
  generate 1 parity chunk: ~50 GB encoding (RS over existing chunks)
  transfer to new device: ~50 GB over local network
  at 1 Gbps LAN: ~7 minutes
  existing devices: zero downtime, serve reads throughout

PRACTICAL NUMBERS (100 GB, 3 devices → 2 devices, emergency):
  reconstruct from 2 remaining: ~50 GB decoding
  re-encode as (1, 2) = full replication: 100 GB per device
  at 1 Gbps: ~14 minutes for full recovery
  during recovery: all data accessible from the 2 survivors (k=2 satisfied)
```

rechunking is incremental — files re-encode independently. the system remains available throughout: any k surviving chunks serve reads while background rechunking runs. priority queue: tier 0 (already fully replicated, zero work), tier 1 (active files first), tier 2 (archive last).

the commitment root updates atomically after rechunking completes for each file batch. devices running old and new chunk layouts simultaneously is safe — both layouts are valid against their respective roots. convergence to the new layout is [[eventual consistency]] at the device level.

### virtual disks

the local device set presents as a virtual filesystem with multiple disks. each disk has its own erasure parameters, cache policy, and rechunking priority. the [[neuron]] sees a unified namespace; the system maps files to erasure-coded chunks across devices transparently.

```
/vault/keys        → disk0: replication=full,  f=n-1
/vault/work        → disk1: erasure=(k,n),     f=1
/vault/archive     → disk2: erasure=(k,n),     f=1, cold fetch
/vault/media       → disk3: erasure=(k,n),     f=1, lazy rechunk
```

disk parameters adapt as devices join or leave. a [[neuron]] with 2 devices has k=1 everywhere (full replication, the only option). at 3 devices, disk1 can switch to (2,3) — each device drops from 100% to 50% for that volume. at 5 devices, media can run (4,5) — 25% per device.

```
devices   disk0 (keys)   disk1 (work)   disk2 (archive)   disk3 (media)
  2       full (1,2)     full (1,2)     full (1,2)        full (1,2)
  3       full (1,3)     (2,3) 50%      (2,3) 50%         (2,3) 50%
  5       full (1,5)     (4,5) 25%      (4,5) 25%         (4,5) 25%
 10       full (1,10)    (9,10) 11%     (9,10) 11%        (9,10) 11%
```

each disk is a separate commitment subtree under the [[neuron]]'s root. per-disk rechunking means adding a device only re-encodes volumes that change parameters — disk0 (full replication) just copies to the new member, disk1 re-encodes only if the neuron opts for tighter k.

### capacity-weighted chunk distribution

devices rarely have equal free space. the system adapts by using fine-grained chunks (per-file or fixed-size blocks, 1-4 MB) and distributing them proportional to each device's available capacity. the constraint: total available capacity across all devices ≥ D × n/k (the encoded data size).

```
100 GB data, rate 1/2 (k=2, n=3), encoded total = 150 GB:

device A (phone, 30 GB free):   gets  30 GB of chunks   (20%)
device B (laptop, 70 GB free):  gets  70 GB of chunks   (47%)
device C (server, 50 GB free):  gets  50 GB of chunks   (33%)
──────────────────────────────────────────────────────────────
total available: 150 GB = 150 GB needed                    ✓
```

any k chunks per file reconstruct it regardless of which device holds them. the placement is a bin-packing problem, not a rigid 1-chunk-per-device assignment. a file's chunks can all live on the largest device if needed — the redundancy guarantee is per-file (k of n chunks survive), not per-device (equal load).

rebalancing on capacity change:

```
device B frees 20 GB (user deletes local files):
  → B now has 90 GB free
  → system can migrate chunks from A (constrained) to B (spacious)
  → per-file k-of-n invariant maintained throughout
  → background process, zero downtime

device A fills up (user installs app):
  → A has 10 GB free, holds 30 GB of chunks → 20 GB must migrate out
  → system moves lowest-priority chunks (tier 2 archive) to B or C
  → if total capacity drops below D × n/k → alert: add device or reduce data
```

the virtual disk parameters (k, n, f) set the redundancy target. capacity weighting decides where chunks physically sit. these are orthogonal: the same (2,3) erasure code works whether devices are 50/50/50 GB or 10/40/100 GB. the only hard constraint is sum of capacities ≥ encoded size.

### two user-facing primitives

from the [[neuron]]'s perspective, the entire local storage system reduces to two operations:

```
create-disk <name> --redundancy f --tier <0|1|2> --cache-policy <hot|cold|lru>
attach <device> <disk> --capacity <size>
```

create-disk defines a logical volume: what redundancy, what access pattern. it has no physical storage until devices are attached. the system derives k = n − f automatically once enough devices exist (n ≥ f + 1).

attach allocates physical space from a specific device into a logical disk. one device can contribute to multiple disks. one disk spans multiple devices.

```
create-disk keys   --redundancy max  --tier 0  --cache-policy hot
create-disk work   --redundancy 1    --tier 1  --cache-policy lru
create-disk archive --redundancy 1   --tier 2  --cache-policy cold

attach phone  keys    --capacity 1GB
attach phone  work    --capacity 20GB
attach laptop keys    --capacity 1GB
attach laptop work    --capacity 50GB
attach laptop archive --capacity 100GB
attach server keys    --capacity 1GB
attach server work    --capacity 30GB
attach server archive --capacity 200GB
```

the system computes from attachments:

```
disk    devices   total capacity   k    per-file overhead   status
keys    3         3 GB             1    full replication     ✓ healthy (f=2)
work    3         100 GB           2    1.5×                 ✓ healthy (f=1)
archive 2         300 GB           1    full replication     ⚠ needs 3rd device for f=1
```

when a new device is attached to a disk, k increases (if the neuron opts in) and per-device storage drops. when a device detaches, the system rechunks in background to maintain the redundancy target. if total capacity falls below the encoded size, the system alerts before data loss becomes possible.

the separation is clean: create-disk is a policy decision (how safe). attach is a resource decision (how much space from where). chunk placement, rebalancing, transparent fetch, and rechunking are internal — the neuron never manages individual chunks.

this is the local-scale instance of [[cyb/fs]]: a content-addressed virtual filesystem over the [[cybergraph]]. at global scale the same abstraction applies — [[neurons]] instead of devices, [[stake]]-weighted distribution instead of capacity-weighted, slashing instead of key revocation. the FS interface is identical. only the trust model and parameters change.

## the cost (current NMT-based)

```
per sample:      NMT inclusion proof → O(log n) Hemera hashes
                 at n = 2³²: 32 × 736 = 23,552 constraints per sample
20 samples:      20 × 23,552 = 471,040 constraints
bandwidth:       20 × 1 KiB = 20 KiB per verification
```

this is layer 4's contribution to [[Verified Eventual Consistency]]. combined with layer 3 ([[NMT]] completeness) and layer 5 ([[CRDT]] merge), it provides the full VEC guarantee: convergence verified + completeness verified + availability verified.

## algebraic DAS: the evolution

### the shift

replace [[NMT]] inclusion proofs with [[lens]] (polynomial commitment scheme) openings. every sample becomes a polynomial evaluation instead of a Merkle path:

```
current (NMT):
  sample cell → NMT inclusion proof → O(log n) Hemera hashes
  proof size: ~1 KiB per sample
  verification: walk Merkle path, check sorting invariant

algebraic:
  sample cell → Lens opening → O(1) field operations
  proof size: ~200 bytes per sample
  verification: one polynomial evaluation check
```

### the numbers

| metric | NMT DAS | algebraic DAS | improvement |
|---|---|---|---|
| proof size per sample | ~1 KiB | ~200 bytes | 5× smaller |
| constraints per sample | 23,552 | ~100 | 235× fewer |
| 20 samples bandwidth | 20 KiB | 4 KiB | 5× less |
| 20 samples constraints | 471K | 2K | 235× fewer |
| in-circuit verification | prohibitive for bulk | feasible | enables provable DAS |

### why this matters

algebraic DAS is not a standalone optimization. it is a consequence of [[algebraic state commitments]]: when the state layer moves from hash trees to polynomials, the availability layer inherits the efficiency automatically. the NMT that commits erasure-coded chunks becomes a polynomial that commits them algebraically.

the cascade:
- [[BBG]] polynomial state (one commitment for all state) → state reads are O(1) field ops
- algebraic DAS (Lens openings for samples) → availability verification is O(1) field ops
- [[provable consensus]] (tri-kernel in [[zheng]] circuit) → the circuit can verify availability IN the proof

with NMT DAS: verifying 20 samples in-circuit costs 471K constraints. feasible but expensive.
with algebraic DAS: 2K constraints. negligible. the prover can verify data availability as part of proving consensus correctness — zero marginal cost.

## the complete availability stack

```
layer 4 in structural sync:

  DATA CREATION
    neuron creates signal → content chunks
         ↓
  ERASURE ENCODING
    2D Reed-Solomon over Goldilocks field
    k data chunks → 2k encoded chunks
         ↓
  COMMITMENT
    NMT (current): namespace Merkle tree over chunks
    polynomial (future): Lens commitment over chunk evaluations
         ↓
  DISTRIBUTION
    local: capacity-weighted across device set
    global: stake-weighted across neuron set
         ↓
  SAMPLING (DAS)
    verifier samples O(√n) random cells
    each cell: inclusion proof (NMT path or Lens opening)
    confidence: 1 - (1/2)^k with k samples
         ↓
  RECONSTRUCTION (on failure)
    any k-of-2k chunks → full reconstruction
    device dies → other devices hold parity → data survives
```

## what DAS does NOT solve

availability proves data EXISTS. it does not prove data is CORRECT (that is layer 1: [[zheng]] validity), ORDERED (layer 2: [[hash chain]] + [[VDF]]), or COMPLETE for a given source (layer 3: [[NMT]] completeness). each layer provides an independent guarantee. together: [[Verified Eventual Consistency]].

DAS also does not provide PERMANENT availability. erasure coding survives k failures out of 2k holders. if more than k holders disappear, data is lost. long-term persistence requires archival strategies — replication to additional holders as existing holders leave.

## relationship to other systems

| system | erasure coding | sampling | commitment | transparent |
|---|---|---|---|---|
| [[Celestia]] | 2D Reed-Solomon | DAS (O(√n)) | [[NMT]] | yes |
| Ethereum (EIP-4844) | 1D RS | no sampling | KZG | no (trusted setup) |
| EigenDA | 1D RS | no sampling | KZG | no |
| Avail | 2D RS (Kate) | DAS | KZG | no |
| cyber (current) | 2D RS over [[Goldilocks field]] | DAS (O(√n)) | [[NMT]] + [[Hemera]] | yes |
| cyber (algebraic) | 2D RS over Goldilocks | DAS (O(√n)) | lens ([[WHIR]]/Brakedown) | yes |

cyber's approach is closest to [[Celestia]] (both use NMT + 2D RS + transparent DAS). the key difference: cyber's algebraic evolution replaces NMT with lens, making availability verification algebraic — composable with the [[zheng]] proof system. Celestia keeps NMT permanently.

the other difference: cyber uses [[Goldilocks field]] for erasure coding (same field as proofs, consensus, FHE). no field mismatch. Celestia uses a separate field.

see [[DAS]] for the sampling algorithm. see [[erasure coding]] for Reed-Solomon construction. see [[NMT]] for namespace Merkle trees. see [[structural sync]] for the five-layer framework. see [[algebraic state commitments]] for the polynomial transition. see [[cyber/research/vec formalization]] for formal availability guarantees. see [[BBG]] for the authenticated state layer
