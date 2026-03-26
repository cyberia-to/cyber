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
