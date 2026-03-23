---
tags: cyber, research, article
crystal-type: article
crystal-domain: cyber
date: 2026-03-23
---

# verified eventual consistency: formalization

## definition

Verified Eventual Consistency (VEC) is a consistency model for distributed systems where correctness is VERIFIABLE locally, not ASSUMED globally. it extends [[eventual consistency]] with three cryptographic guarantees.

a distributed system satisfies VEC if these four properties hold:

### P1: safety (convergence)

for any two correct nodes A and B with [[signal]] sets $S_A$ and $S_B$:

$$S_A = S_B \implies \text{state}(S_A) = \text{state}(S_B)$$

this follows from [[CRDT]] properties. the merge function is a [[join-semilattice]]: commutative ($a \sqcup b = b \sqcup a$), associative ($(a \sqcup b) \sqcup c = a \sqcup (b \sqcup c)$), idempotent ($a \sqcup a = a$). equal sets under a deterministic lattice merge produce equal states.

the algorithm is [[G-Set]] union for independent signals, topological sort by (causal order → [[VDF]] time → hash tiebreak) for dependent signals:

```
merge(S):
  sorted ← topological_sort(S, order = causal > vdf > hash)
  state ← initial_state
  for signal in sorted:
    state ← apply(state, signal)
  return state
```

deterministic: same set → same sort → same sequence → same state.

### P2: completeness (verifiable set equality)

a node can verify in O(log n) whether its signal set for a given source is complete:

$$\text{verify\_complete}(S_A^{(\nu)}, \text{steps}[a, b]) \to \{\text{true}, \text{false}\}$$

algorithm: [[NMT]] completeness proof. each source (device or [[neuron]]) commits its signal chain to a per-source NMT namespaced by step counter:

```
NMT_ν[step → H(signal)]

completeness_proof(ν, range [a,b]):
  path_left  ← merkle_path(leftmost leaf with step ≥ a)
  path_right ← merkle_path(rightmost leaf with step ≤ b)
  boundary_left  ← neighbor with step < a (or tree boundary)
  boundary_right ← neighbor with step > b (or tree boundary)
  return (paths, boundaries, leaves)

verify_complete(proof, root, range [a,b]):
  check path_left valid against root
  check path_right valid against root
  check boundary_left.step < a
  check boundary_right.step > b
  check sorting invariant: all leaves sorted by step
  → structurally impossible to omit a leaf in [a,b]
```

cost: O(log n) [[Hemera]] hashes for proof, O(log n) for verification. the guarantee is unconditional — it depends only on collision resistance of [[Hemera]], not on honest majorities or protocol execution.

### P3: availability (verifiable data existence)

a node can verify in O(√n) whether the data underlying its signal set physically exists across the network:

$$\text{verify\_available}(\text{root}, k) \to \{\text{available}, \text{unavailable}\}$$

algorithm: [[DAS]] with 2D Reed-Solomon [[erasure coding]] over [[Goldilocks field]]:

```
encode(data, rate=1/2):
  grid ← reshape(data, √n × √n)
  for each row:  extended_row ← reed_solomon_encode(row, rate)
  for each col:  extended_col ← reed_solomon_encode(col, rate)
  return extended_grid  # 2√n × 2√n

commit(extended_grid):
  for each row: row_root ← NMT(row cells, namespace = position)
  col_root ← NMT(row_roots)
  return col_root

sample(root, k=20):
  for i in 1..k:
    pos ← random_position()
    (cell, proof) ← request(pos)
    if not verify_nmt_proof(cell, proof, root): return unavailable
  return available  # confidence ≥ 1 - (1/2)^k

# k=20: 99.9999% confidence
# k=30: 99.99999999% confidence
```

the guarantee is probabilistic but information-theoretic — it depends on sampling randomness, not computational hardness. with rate 1/2 encoding, withholding >50% of data makes reconstruction fail, but random sampling detects it with probability approaching 1 exponentially in k.

### P4: liveness (eventual delivery)

if a signal is created by a correct node and the network is eventually connected, every correct node eventually receives it:

$$\forall s \in \text{created\_signals}, \exists t: \forall \text{correct } A, s \in S_A^{(t)}$$

this relies on gossip protocol + [[erasure coding]] reconstruction. if some chunks are lost, any k-of-n surviving chunks reconstruct the original. the liveness bound depends on network connectivity and gossip propagation — same as standard [[eventual consistency]], but strengthened by erasure reconstruction.

## relationship to existing models

| model | safety | completeness | availability | liveness |
|---|---|---|---|---|
| [[eventual consistency]] | assumed (not verified) | not provided | not provided | gossip-based |
| strong eventual consistency (SEC) | guaranteed if same set | not provided | not provided | assumed delivery |
| causal consistency | causal ordering guaranteed | not provided | not provided | causal delivery |
| linearizability | total ordering guaranteed | implied by ordering | implied by availability | immediate |
| VEC | guaranteed if same set (P1) | verifiable O(log n) (P2) | verifiable O(√n) (P3) | gossip + erasure (P4) |

VEC is strictly stronger than SEC: SEC assumes set equality, VEC verifies it. VEC is weaker than linearizability: no total ordering, no real-time guarantee.

the key distinction: SEC trusts that the message delivery layer is complete. VEC verifies completeness cryptographically. this is the difference between "we assume no messages were dropped" and "we prove no messages were dropped."

## adding validity and ordering: VEC+

the base VEC model (P1-P4) provides convergence, completeness, availability, and liveness. adding two more properties transforms it from "correct sync" to "sovereign computation":

### P5: validity (provable state transitions)

every signal carries a [[zheng]] proof σ covering all operations in the batch:

$$\text{verify\_signal}(s, \sigma) \to \{\text{valid}, \text{invalid}\}$$

algorithm: SuperSpartan IOP + WHIR PCS over [[Goldilocks field]]:

```
verify_signal(signal, proof):
  1. init transcript T from signal commitment
  2. for i in 1..k (sumcheck rounds):
     gᵢ ← proof.round_polynomials[i]
     assert gᵢ(0) + gᵢ(1) = previous_claim
     T.absorb(gᵢ)
     rᵢ ← T.squeeze()  # Fiat-Shamir via Hemera
  3. assert final_claim = constraint_eval(proof.value, r, statement)
  4. assert WHIR_verify(commitment, r, proof.value, proof.opening)
```

cost: O(log N) Hemera hashes + field ops. ~50 μs independent of computation size (recursive proof compresses all history). an invalid signal cannot be constructed — the constraint system (16 deterministic reduction patterns of [[nox]]) prevents it.

### P6: ordering (immutable causal structure)

every signal carries embedded ordering metadata:

```
signal.prev         = H(author's previous signal)     # hash chain
signal.merkle_clock = H(all known signal hashes)       # compact causal state
signal.vdf_proof    = VDF(prev, T_min)                 # physical time
signal.step         = monotonic counter                 # logical clock
```

properties:
- [[hash chain]]: immutable per-source history. inserting/removing/reordering signals breaks chain hashes. detectable by any peer in O(1) — compare prev fields
- [[VDF]]: physical rate limit. each signal costs T_min sequential wall-clock computation. equivocation (two signals with same prev) costs 2×T_min — detectable because total VDF time exceeds elapsed wall time
- deterministic total order from data alone: causal > VDF time > hash tiebreak. no coordination needed

## the full model: VEC+ = P1 + P2 + P3 + P4 + P5 + P6

| property | mechanism | algorithm | complexity | guarantee type |
|---|---|---|---|---|
| P1 convergence | [[CRDT]] merge | [[G-Set]] union + topo sort | O(signals) | algebraic (deterministic) |
| P2 completeness | [[NMT]] proof | namespace range proof | O(log n) | structural (unconditional) |
| P3 availability | [[DAS]] + [[erasure coding]] | 2D Reed-Solomon + sampling | O(√n) | probabilistic (information-theoretic) |
| P4 liveness | gossip + erasure | k-of-n reconstruction | network-dependent | eventual (gossip propagation) |
| P5 validity | [[zheng]] proof | SuperSpartan + WHIR | O(log N) verify | computational (collision resistance) |
| P6 ordering | [[hash chain]] + [[VDF]] | SHA/Hemera chain + sequential VDF | O(1) verify | physical (sequential time) |

each property has a different guarantee type. each comes from a different branch of mathematics. each is independently verifiable. this is compositional security — failure of one layer does not compromise others.

## lower bounds

conjecture: P1 + P2 + P3 is the minimum for verified convergence without coordination.

### argument by elimination

- remove P1 (convergence): equal sets produce different states. verification of completeness and availability is useless if the merge is non-deterministic
- remove P2 (completeness): convergence on incomplete data produces wrong state. availability proves data exists but not that you have all of it
- remove P3 (availability): convergence on complete data that no longer exists. NMT proves you had everything, but the data may be lost

each removal loses a property the other two cannot provide. the three core properties are orthogonal.

### potential unification: algebraic NMT

the [[algebraic-nmt]] proposal replaces NMT trees with polynomial commitments:

```
current (NMT + CRDT):
  completeness: NMT sorting invariant (structural)
  merge: G-Set union (algebraic)
  → two independent structures

algebraic NMT:
  completeness: polynomial evaluation proof (algebraic)
  merge: polynomial addition (algebraic)
  → one structure providing both

cost reduction:
  current: ~106K constraints per cyberlink (NMT paths)
  algebraic: ~3.2K constraints (PCS evaluation)
  → 33× improvement
```

if completeness and merge share a single polynomial commitment scheme, the minimum composition reduces from algebra + logic + probability to algebra + probability. two layers instead of three. the lower bound conjecture would need revision.

## composability

### vertical composability (the stack)

the cyber proof stack composes vertically:

```
application (cybergraph operations)
     ↓ expressed as
nox (16 reduction patterns, focus-metered execution)
     ↓ proven by
zheng (SuperSpartan IOP + WHIR PCS)
     ↓ committed via
hemera (Poseidon2 hashing, NMT/MMR construction)
     ↓ over
nebu (Goldilocks field arithmetic, p = 2⁶⁴ - 2³² + 1)
```

each layer provides guarantees consumed by the layer above:
- nebu: field operations are correct (algebraic closure)
- hemera: hashes are collision-resistant (2^128 security, margin 2^918)
- zheng: proofs are sound (sumcheck + WHIR, error ≤ 2^-512 at k=16)
- nox: execution is focus-metered (halt guaranteed)
- application: state transitions are valid

### horizontal composability (cross-graph)

if system A uses VEC and system B uses VEC, does A × B inherit VEC?

P1 (convergence): yes, if the merge functions are independent. CRDT products are CRDTs — the product lattice of two join-semilattices is a join-semilattice

P2 (completeness): yes, if namespaces are disjoint. NMT proofs for namespace A and namespace B are independent. the NMT structure naturally supports this — each system occupies a namespace range

P3 (availability): yes, if erasure coding is independent. DAS samples for A and B are separate. availability of A does not depend on availability of B

P4-P6: inherit straightforwardly from per-signal properties

cross-graph cyberlinks (A links to B) require: NMT inclusion proof in both namespaces. this is O(log n_A + log n_B) — additive, not multiplicative

### optimization directions

| layer | current | proposed optimization | improvement |
|---|---|---|---|
| P1 (merge) | [[G-Set]] union | algebraic accumulator | O(1) membership instead of O(n) scan |
| P2 (completeness) | NMT (23.5K constraints/path) | algebraic NMT (PCS evaluation) | 33× constraint reduction |
| P3 (availability) | 2D Reed-Solomon | 3D tensor codes | O(∛n) sampling instead of O(√n) |
| P5 (validity) | zheng proof (~70K constraints/verify) | jets (precompiled verification) | 8.5× constraint reduction |
| P5 (recursion) | fold per block | batched folding (IVC) | amortized O(1) per signal |
| cross-index | LogUp (~500 constraints/axon) | algebraic NMT (zero) | ∞× (eliminated) |

the complete optimization path: nebu provides fast field ops → hemera uses them for efficient hashing → zheng uses hemera for transparent proofs → nox uses zheng for provable execution → bbg uses everything for authenticated state. top-down: application demands drive optimization targets. bottom-up: field arithmetic improvements cascade through every layer.

this bidirectional optimization is why the stack exists as separate repos — each layer can be improved independently while the interfaces remain stable. a 2× improvement in [[Hemera]] permutation speed cascades to 2× faster NMT updates, 2× faster proof generation, 2× faster state transitions. a 33× reduction in NMT constraints (algebraic NMT) cascades to 33× cheaper cyberlinks, 33× more throughput, 33× lower per-signal cost.

see [[structural sync]] for the five-layer theory. see [[cyb/fs/sync]] for the unified patch+sync specification. see [[bostrom-to-onnx-pipeline]] for how graph structure compiles into transformer parameters. see [[algebraic-nmt]] for the polynomial NMT proposal. see [[cyber/research/spectral gap from convergence]] for the empirical observation that convergence is faster than predicted
