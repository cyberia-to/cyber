---
tags: cyber, research, article
crystal-type: article
crystal-domain: cyber
date: 2026-03-23
---

# algebraic state commitments: replacing hash trees with polynomials

## the problem

every state-modifying operation in a blockchain touches authenticated data structures. the standard: [[Merkle tree]] variants (Patricia, NMT, MMR). a single state update rehashes a path from leaf to root — O(log n) hash calls. at billions of entries across multiple indexes, this hash cost dominates everything else.

[[BBG]] maintains 9 public [[NMT]] indexes for the [[cybergraph]]. a single [[cyberlink]] touches 4-5 of them. at 4 billion entries per index (tree depth 32), one cyberlink costs ~106,000 constraints — almost entirely [[Hemera]] hash recomputation along tree paths.

the insight: the 9 trees contain the SAME data viewed from different angles. [[particles]] by content, axons by source, axons by target, [[neurons]] by identity. maintaining separate hash trees forces redundant computation proportional to the number of views, not the amount of change.

## the primitive: polynomial state

replace hash trees with a polynomial commitment. commit to a function $f: \text{index} \times \text{namespace} \times \text{position} \to \text{value}$ via a single [[lens]] (polynomial commitment scheme). querying any view is a polynomial opening — one proof that $f$ evaluates to the claimed value at the queried point.

```
NMT approach:    9 independent trees, each O(log n) to update
                 cross-index: LogUp argument (~500 constraints per lookup)
                 completeness: structural (sorting invariant)

polynomial:      1 commitment, O(1) to open at any point
                 cross-index: free (same polynomial, different evaluation)
                 completeness: algebraic (Lens binding)
```

## what changes

### 33× cheaper cyberlinks

| cost center | NMT (current) | algebraic | reduction |
|---|---|---|---|
| state update | ~106K constraints | ~3.2K constraints | 33× |
| cross-index consistency | ~1.5K (LogUp) | 0 (definitional) | ∞ |
| completeness proof | ~1 KiB per namespace | ~200 bytes | 5× |
| per-block (1000 links) | ~108M constraints | ~7.5M | 14× |

the constraint reduction cascades through the [[zheng]] proof pipeline: fewer constraints → smaller proofs → faster verification → cheaper participation → more [[neurons]] → denser [[cybergraph]].

### 5 TB storage eliminated

9 NMT trees at 4 billion entries each: (2n-1) × 64 bytes per index = ~550 GB per index × 9 = ~5 TB of internal tree nodes. the polynomial replaces ALL tree structure with 9 commitments × 32 bytes = 288 bytes.

leaf data stays. internal nodes vanish. a full node needs 5 TB less storage. a validator can run on hardware that costs 10× less.

### cross-index consistency for free

currently, consistency between `axons_out` and `axons_in` (same edge indexed from both endpoints) requires LogUp lookup arguments — ~500 constraints per lookup, ~2M per block.

with one polynomial, `BBG_poly(axons_out, P, _)` and `BBG_poly(axons_in, P, _)` are evaluations of the same committed object. they CANNOT disagree. consistency is not proven — it is structural within the algebraic framework. LogUp eliminated entirely.

### smaller proofs = lighter clients

| operation | NMT proof | polynomial proof |
|---|---|---|
| single inclusion | 32 × 32B = 1 KiB | ~200 bytes (Lens opening) |
| namespace range (100 entries) | ~100 KiB | ~5 KiB |
| light client sync per namespace | ~1 KiB | ~200 bytes |
| DAS sample | 1 KiB (NMT path) | 200 bytes (Lens opening) |

a light client on a phone downloading 100 namespace proofs: 100 KiB → 20 KiB. 5× less bandwidth. in regions with expensive mobile data, this is the difference between usable and unusable.

## what is traded

### structural → computational completeness

[[NMT]] completeness is structural: the sorting invariant is a property of the data structure itself. the tree CANNOT omit entries by construction. no computational assumption.

polynomial completeness is computational: the Lens (polynomial commitment scheme) must be sound. if the Lens breaks, the polynomial can produce false proofs. this is weaker.

the mitigation: Brakedown is a hash-based lens — its soundness relies on collision resistance of [[Hemera]], the same hash that NMT nodes use. the trust root is identical. the difference: NMTs use [[Hemera]] structurally (tree shape = completeness). polynomials use [[Hemera]] algebraically (polynomial binding = completeness). both trust the same primitive.

the hybrid migration: during transition, run BOTH systems. NMT and polynomial must agree on every query. divergence = bug or cryptographic break. years of agreement build confidence.

### familiar → novel

Merkle trees are understood since 1979. polynomial commitments over extension fields are 2019+. operational experience matters. bugs in novel cryptography can be catastrophic.

the migration path (prop in [[algebraic-nmt]]):
1. Verkle-tree hybrid: tree structure preserved, lens at nodes (33× cheaper, NMT as backup)
2. Flat polynomial: tree eliminated, Brakedown commitment, NMT removed after confidence
3. Unified polynomial: all 9 indexes → one commitment, cross-index structural

## the algorithm: Verkle-tree state commitment (phase 1)

### structure

replace [[Hemera]] hashes at NMT internal nodes with lens node commitments:

```
NMT node:      H(left_hash ‖ right_hash ‖ ns_min ‖ ns_max)
               → 1 Hemera permutation = 736 constraints

Verkle node:   Lens.commit([child_0, child_1, ..., child_{b-1}])
               → ~100 field operations (for arity b)

with binary tree (b=2):
  Verkle: ~100 constraints/node vs NMT: 736 constraints/node → 7.4×
  path of 32 nodes: 3,200 vs 23,552 → 7.4×

with arity-256 tree (b=256):
  depth: 32/8 = 4 levels
  per-level: ~100 constraints × 256 = ~25,600 (but only one path)
  total: 4 × ~800 = ~3,200 constraints
  vs NMT depth 32: 23,552 → 7.4×
```

the improvement ratio is the same regardless of arity because the hash-vs-field-op cost ratio dominates.

### update algorithm

```
update(tree, leaf_index, new_value):
  path ← find_path(tree.root, leaf_index)

  for level in path (leaf to root):
    children ← level.children
    children[child_index] ← updated_hash
    level.commitment ← lens.recommit(level.old_commitment, child_index, delta)
    # incremental recommit: O(1) field ops per child change

  tree.root ← path.root.commitment
  return updated_tree

# cost per update: O(depth) × O(1) recommits = O(log n) field ops
# vs NMT: O(depth) × O(1) Hemera = O(log n) hash ops
# field op ~100 constraints vs Hemera ~736 constraints → 7.4× cheaper
```

### batch update (deduplication)

```
batch_update(tree, updates: [(leaf_index, new_value)]):
  # group updates by affected subtree path
  dirty_nodes ← {}
  for (idx, val) in updates:
    path ← find_path(tree.root, idx)
    for node in path:
      dirty_nodes.insert(node.id)

  # bottom-up: recompute each dirty node exactly once
  for node in dirty_nodes.sorted_by_depth(deepest_first):
    node.commitment ← lens.recommit(node.children)

  # power-law: 1000 updates touching 200 unique leaves
  # dirty internal nodes: ~800 (not 4500)
  # savings: 4500/800 = 5.6× on top of per-node 7.4×
```

### opening (query proof)

```
open(tree, index, namespace, key):
  leaf ← find_leaf(tree, namespace, key)
  path ← path_from_root(tree.root, leaf)

  proof ← []
  for level in path:
    proof.append(Lens.open(level.commitment, child_index))

  return (leaf.value, proof)
  # proof size: depth × PCS_opening_size
  # binary Verkle, depth 32: 32 × ~200B = ~6.4 KiB (larger than NMT 1 KiB)
  # arity-256 Verkle, depth 4: 4 × ~200B = ~800B (smaller than NMT)

verify(root, namespace, key, value, proof):
  current ← root
  for (level_proof, child_index) in proof:
    assert Lens.verify(current, child_index, level_proof)
    current ← level_proof.child_commitment
  assert current == H(value)
  return true
```

## the algorithm: flat polynomial commitment (phase 2)

### structure

no tree at all. the state is a polynomial over the evaluation domain:

```
BBG_poly: F_p³ → F_p

BBG_poly(i, ns, pos) = value

where:
  i ∈ {0..8}:         index selector (particles, axons_out, ...)
  ns ∈ F_p:           namespace (CID hash, neuron_id, ...)
  pos ∈ [0, n_max):   position within namespace

committed: C = Brakedown.commit(BBG_poly)
opened:    (v, φ*) = Brakedown.open(BBG_poly, (i, ns, pos))
verified:  Brakedown.verify(C, (i, ns, pos), v, φ*)
```

### completeness via LogUp range check

```
completeness_proof(index i, namespace range [a, b]):
  # prover collects all entries in range
  entries ← {(ns_j, val_j) : a ≤ ns_j ≤ b, BBG_poly(i, ns_j, _) = val_j}

  # LogUp argument over F_{p²}:
  # random challenge β ∈ F_{p²} from Fiat-Shamir

  claimed_sum = Σ_{j ∈ entries} 1/(β - ns_j)
  committed_sum = Σ_{j ∈ all_in_index} 1/(β - ns_j)  (committed auxiliary poly)

  # boundary check: no ns between a and entries[0], no ns between entries[-1] and b
  # this is the algebraic analogue of NMT's sorting invariant

  # Schwartz-Zippel: Pr[false positive] ≤ degree / |F_{p²}| ≈ 2^{-96}

verify_completeness(C, range [a,b], entries, proof):
  assert LogUp_verify(C, entries, proof, β)
  assert boundary_check(entries, a, b)
  return true
```

### batch recommit

```
batch_recommit(C_old, changes: [(i, ns, pos, old_val, new_val)]):
  # Brakedown: linear-time recommit
  delta_poly ← zero_polynomial()
  for (i, ns, pos, old, new) in changes:
    delta_poly(i, ns, pos) += new - old

  C_new ← C_old + Brakedown.commit(delta_poly)

  # cost: O(|changes|) field operations for delta construction
  #        + O(N) for Brakedown recommit (where N = total entries)
  # amortized per block: O(N) dominates, but N = committed domain size
  # can be reduced to O(|changes| × log N) with sparse Brakedown
```

## the cascade: how this changes the stack

### [[zheng]] proof pipeline

```
current:
  signal → nox execution trace → SuperSpartan proof
  dominant cost: hemera hashes in NMT path verification

algebraic:
  signal → nox execution trace → SuperSpartan proof
  NMT verification replaced by Lens evaluation check
  ~33× fewer constraints in state transition circuits
  → smaller proofs → faster prover → faster verifier
```

### [[DAS]] (layer 4 of [[structural sync]])

```
current DAS:
  sample cell → NMT inclusion proof → hemera path verification
  cost per sample: O(log n) hemera hashes

algebraic DAS:
  sample cell → Lens opening → field op verification
  cost per sample: O(1) field ops

  20 samples: 20 KiB → 4 KiB bandwidth, hemera calls → field ops
```

### [[foculus]] convergence

```
current:
  φ* convergence requires reading neuron states from NMT
  each read: NMT proof verification → hemera path

algebraic:
  read neuron states via Lens opening → field ops
  convergence check becomes algebraically verifiable

  potential: prove φ* convergence inside zheng circuit
  → recursive proof of consensus itself
```

### light client

```
current:
  join: 1 zheng proof (50 μs) + NMT namespace sync (~1 KiB per namespace)

algebraic:
  join: 1 zheng proof (50 μs) + lens namespace sync (~200 bytes per namespace)

  full state verification: ONE polynomial commitment (32 bytes)
  every query: ONE Lens opening (~200 bytes)

  the lightest possible client: store 32 bytes, verify anything
```

## why this is game-changing

### quantitative: 33× cheaper computation

every operation in the [[cybergraph]] becomes 33× cheaper in constraints. at fixed hardware, this means 33× more throughput. or: the same throughput at 33× less hardware cost. or: mobile devices that currently cannot validate can now validate.

### qualitative: unified state

9 separate trees → 1 polynomial. cross-index consistency moves from "proven by LogUp" to "structural by construction." this is not an optimization — it is a simplification. the state model becomes one object instead of nine. debugging, auditing, reasoning about correctness all become simpler.

### architectural: tree-free blockchain

no Merkle tree for public state. no internal nodes. no tree rebalancing. no path recomputation. the entire concept of "tree traversal" disappears from the state layer. what replaces it: polynomial evaluation. a fundamentally different computational primitive.

every blockchain since Bitcoin uses hash trees for state authentication. algebraic state commitments are a departure from this 15-year architectural assumption. the question is not "is it faster?" (yes, 33×). the question is: "is the algebraic trust model sound enough to replace the structural trust model?" the [[Hemera]] collision resistance underlies both. the migration path with hybrid verification provides the empirical answer.

see [[algebraic-nmt]] for the BBG proposal with migration phases. see [[structural sync]] for how this changes layer 3 (completeness). see [[cyber/research/vec formalization]] for the formal VEC model. see [[unified-polynomial-state]] for the endgame (single polynomial for ALL state). see [[Hemera]] for the hash primitive that underlies both NMT and lens trust
