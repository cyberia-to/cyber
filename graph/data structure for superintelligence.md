---
tags: article, cyber, cip
crystal-type: entity
crystal-domain: computer science
status: draft
stake: 10577346440909906
---

## Authenticated State Architecture for [[nox]]

Version 1.0 — February 2026

*"The network doesn't simulate thinking. The network IS thinking."*

---

## Abstract

This document specifies the complete authenticated state architecture for [[nox]] — a planetary-scale collective intelligence system targeting 10¹⁵ nodes with cryptographic proofs, privacy by construction, and bounded-locality updates.

The architecture emerges from a single constraint: every operation must be provable, every proof must be verifiable, and verification cost must remain within a constant factor of computation — at any scale.

Five ontological primitives ([[particle]], [[cyberlink]], [[neuron]], [[token]], [[focus]]) authenticated by five cryptographic data structures, each proven in production:

| Primitive | Role | Production Heritage |
|-----------|------|-------------------|
| Namespaced Merkle Tree (NMT) | Graph completeness proofs | Celestia (2023—) |
| Merkle Mountain Range (MMR) | Append-only UTXO history | Grin, Neptune (2019—) |
| Sliding-Window Bloom Filter (SWBF) | Private double-spend prevention | Neptune (2024—) |
| WHIR Polynomial Commitments | Edge membership & batch proofs | FRI/Plonky2 (2022—), WHIR (2025) |
| LogUp Lookup Arguments | Cross-index consistency | Polygon, Scroll (2023—) |

Unified by a single hash function ([[Hemera]]), a single field ([[Goldilocks field]], p = 2⁶⁴ − 2³² + 1), and a single proof system ([[stark]] with [[WHIR]]).

---

# Part I: Foundations

## 1. Design Principles

### 1.1 The Three Laws

Law 1: Bounded Locality. No global recompute for local change. Every operation's cost must be proportional to what it touches, not to the total graph size. At 10¹⁵ nodes, global operations are physically impossible — light-speed delays across Earth exceed any acceptable latency bound.

Law 2: Compute-Verify Symmetry. Verification cost ≤ c · computation cost, where c is a small constant. A system where verification is cheaper than computation can scale through delegation. A system where verification is MORE expensive than computation cannot scale at all.

Law 3: Structural Security. Security guarantees must emerge from data structure invariants, not from protocol correctness. A protocol can have bugs. A tree whose internal nodes carry min/max namespace labels cannot lie about completeness — the structure itself prevents it.

### 1.2 The Ontology

Five irreducible primitives. Everything in the system is composed from these. See [[concepts]] for the full reference.

| Primitive | Role | Identity |
|-----------|------|----------|
| [[particle]] ⭕️ | content-addressed node, atom of knowledge | hash of content |
| [[cyberlink]] 🔗 | authenticated directed edge, unit of meaning | hash of (neuron, from, to, weight, time) |
| [[neuron]] 🤪 | agent with stake, identity, and focus | hash of public key |
| [[token]] 🪙 | protocol-native value: [[coin]], [[card]], [[score]], [[badge]] | denomination hash / content hash |
| [[focus]] 🎯 | emergent attention distribution (π), computed by [[tri-kernel]] | [[diffusion]], [[springs]], [[heat kernel]] |

Focus is computed by three local operators — the [[tri-kernel]]: [[diffusion]] (where probability flows), [[springs]] (what satisfies structural constraints), [[heat kernel]] (what the graph looks like at scale τ). These are the only operator families that survive the locality constraint. See [[tri-kernel]] for the completeness proof.

### 1.3 Naming Convention

Three layers, three names:

- [[nox]] — the computation model (16 reduction patterns, deterministic costs)
- [[cybergraph]] — the data model ([[particles]], [[cyberlinks]], [[neurons]], [[tokens]], [[focus]])
- [[cyber/bbg]] — the authenticated state structure (this document)

BBG = Big Badass Graph. The name is earned.

---

## 2. Field and Hash

### 2.1 The [[Goldilocks field]]

All arithmetic operates in the [[Goldilocks field]]:

$$p = 2^{64} - 2^{32} + 1 = 18446744069414584321$$

Properties that make this field optimal:

- Fast reduction: a mod p = a_lo − a_hi × (2³² − 1) + correction. Two 64-bit operations.
- Large 2-adic subgroup: 2³² divides p−1, enabling FFTs up to length 2³² without extension fields.
- Native word size: Fits in a single 64-bit register on all modern hardware.
- stark-optimal: WHIR folding operates directly on F_p without embedding overhead.

Every value in the system — balances, weights, hashes, commitments, proofs — is one or more elements of F_p.

### 2.2 [[Hemera]] (Poseidon2-Goldilocks)

One hash function everywhere:

$$H : F_p^* \rightarrow F_p^4$$

Poseidon2 (second-generation algebraic hash, 2023) with parameters tuned for the Goldilocks field.

| Property | Value |
|----------|-------|
| Input | Variable-length F_p elements |
| Output | 4 F_p elements (256 bits) |
| stark constraints | ~250 per invocation |
| Native throughput | ~70 MB/s on modern CPU |
| Security | 128-bit (collision, preimage, second preimage) |
| Quantum resistance | 85-bit post-Grover (sufficient with doubled output) |

Why Poseidon2 over alternatives:

- vs SHA-256: ~100× cheaper in stark circuits (250 vs 25,000 constraints)
- vs Poseidon (original): ~20% fewer constraints, improved diffusion layer
- vs Tip5: Broader ecosystem support, more extensive cryptanalysis
- vs Rescue Prime: Lower constraint count for same security level
- vs Blake3: Not algebraic — prohibitively expensive in ZK circuits

The decision to use one hash everywhere is architectural, not just convenient. One hash = one security analysis, one implementation to audit, one hardware target for acceleration, one set of test vectors. At planetary scale, heterogeneity is an attack surface.

### 2.3 Domain Separation

All hash invocations use domain-separated prefixes to prevent cross-protocol attacks:

```
H_edge(x)       = H(0x01 ‖ x)    Edge hashing
H_commit(x)     = H(0x02 ‖ x)    Record commitments
H_nullifier(x)  = H(0x03 ‖ x)    SWBF index derivation (mutator set)
H_merkle(x)     = H(0x04 ‖ x)    NMT and MMR internal nodes
H_fiat_shamir(x)= H(0x05 ‖ x)    WHIR challenge derivation
H_transcript(x) = H(0x06 ‖ x)    Proof transcript binding
```

---

# Part II: The Cybergraph Layer

## 3. Namespaced Merkle Trees

### 3.1 Structure

An NMT is a binary Merkle tree over sorted leaves where each internal node carries the minimum and maximum namespace of its subtree:

```
Internal node:
  NMT_node = (min_ns, max_ns, H_merkle(left_child ‖ right_child))

Leaf node:
  NMT_leaf = (namespace, H_merkle(payload))

Invariant:
  For every internal node N with children L, R:
    N.min_ns = L.min_ns
    N.max_ns = R.max_ns
    L.max_ns ≤ R.min_ns        ← sorting invariant
```

The sorting invariant is structural — it's enforced by construction, not by protocol. Any valid NMT path that violates sorting produces an invalid Merkle root, detectable by any verifier with just the root hash.

### 3.2 Completeness Proofs

The NMT's defining capability: prove "these are ALL items in namespace N."

```
COMPLETENESS PROOF for namespace N:
  1. Path to leftmost leaf with namespace N
  2. Path to rightmost leaf with namespace N
  3. Left boundary: left neighbor has namespace < N (or is tree boundary)
  4. Right boundary: right neighbor has namespace > N (or is tree boundary)

VERIFICATION (by any client with only the root):
  a) Both Merkle paths verify against root           — O(log n) hashes
  b) Leftmost leaf has namespace = N                  — 1 comparison
  c) Rightmost leaf has namespace = N                 — 1 comparison
  d) Left neighbor namespace < N                      — 1 comparison
  e) Right neighbor namespace > N                     — 1 comparison

GUARANTEE:
  If verification passes, the client has provably received every leaf
  in namespace N. No leaf can be hidden — the tree structure prevents it.

ABSENCE PROOF for namespace N:
  Show two adjacent leaves with namespaces that bracket N:
    leaf_i.namespace < N < leaf_{i+1}.namespace
  This proves no leaf with namespace N exists in the tree.

Cost:
  Proof size: O(log n) hash digests
  Verification: O(log n) hash computations = O(log n × 250) stark constraints
  For n = 2³²: ~32 × 250 = 8,000 constraints
```

### 3.3 Why NMTs Cannot Be Replaced

Sorted polynomial commitments (as proposed in v0.9) can approximate completeness but lack structural guarantees:

- Polynomial completeness requires a protocol: Prove boundary entries, prove contiguity, prove sorting was maintained. Each step requires a separate argument. Any step can have bugs.
- NMT completeness is a structural invariant: The tree physically cannot represent a valid root over misordered leaves. There is no protocol to debug because there is no protocol — just a tree.
- DAS requires NMTs: Namespace-aware Data Availability Sampling (Celestia model) needs namespace labels propagated through internal nodes. Polynomials don't have internal nodes.
- Sync requires NMTs: "Give me everything for neuron N with proof nothing is hidden" is the cybergraph's fundamental operation. NMT completeness proofs make this trustless. Polynomial approaches require trusting that sorting was maintained by consensus.

Production evidence: Celestia has processed millions of blocks with NMT-based DAS since October 2023. No production system uses sorted polynomial completeness proofs.

---

## 4. The Six Indexes

The [[cybergraph]] maintains six NMT indexes over the same edge and [[token]] data. Edges and [[tokens]] are stored once; indexes contain only hashes and commitments.

### 4.1 Architecture

```
╔═══════════════════════════════════════════════════════════════════════╗
║                    AUTHENTICATED CYBERGRAPH STATE                     ║
╠═══════════════════════════════════════════════════════════════════════╣
║                                                                       ║
║  INDEX 1: by_neuron                                                  ║
║  ─────────────────                                                   ║
║    Structure: NMT[ neuron_id → EdgeSet ]                             ║
║    Namespace: neuron_id (creator of the edge)                        ║
║    Leaf payload: EdgeSet polynomial commitment                       ║
║    Proves: "These are ALL edges created by neuron N"                 ║
║                                                                       ║
║  INDEX 2: by_particle                                                ║
║  ───────────────────                                                 ║
║    Structure: NMT[ particle_hash → EdgeSet ]                         ║
║    Namespace: particle_hash (endpoint of the edge)                   ║
║    Leaf payload: EdgeSet polynomial commitment                       ║
║    Proves: "These are ALL edges touching particle P"                 ║
║                                                                       ║
║  INDEX 3: focus                                                      ║
║  ──────────────                                                      ║
║    Structure: NMT[ neuron_id → F_p ]                                 ║
║    Namespace: neuron_id                                              ║
║    Leaf payload: current focus value π_i                              ║
║    Proves: "Neuron N has focus π_i, and this is the complete set"    ║
║                                                                       ║
║  INDEX 4: balance                                                    ║
║  ────────────────                                                    ║
║    Structure: NMT[ neuron_id → F_p ]                                 ║
║    Namespace: neuron_id                                              ║
║    Leaf payload: current balance                                     ║
║    Proves: "Neuron N has balance B, and this is the complete set"    ║
║                                                                       ║
║  INDEX 5: coins                                                      ║
║  ──────────────                                                      ║
║    Structure: NMT[ denom_hash → supply_record ]                      ║
║    Namespace: denomination hash (e.g. H("CYB"), H("BOOT"))          ║
║    Leaf payload: total supply, mint authority, transfer rules         ║
║    Proves: "Denomination D has supply S, and this is complete"       ║
║    Per-neuron coin balances tracked in INDEX 4 (balance).            ║
║                                                                       ║
║  INDEX 6: cards                                                      ║
║  ──────────────                                                      ║
║    Structure: NMT[ card_id → card_record ]                           ║
║    Namespace: card_id (content-addressed: H(particle ‖ creator))     ║
║    Leaf payload: bound particle, current owner, provenance chain     ║
║    Proves: "Card C exists, is bound to particle P, owned by N"      ║
║    Enables: knowledge assets, authorship proofs, citation rights,    ║
║             dataset ownership, model lineage certificates.           ║
║    Every card is itself a particle — content-addressed, immutable.   ║
║                                                                       ║
╚═══════════════════════════════════════════════════════════════════════╝
```

### 4.2 EdgeSet: Polynomial Commitment per Namespace

Within each NMT leaf, the EdgeSet is a WHIR polynomial commitment to the set of edge hashes belonging to that namespace:

```
EdgeSet for neuron N:
  edges = { e | e.neuron = N }
  edge_hashes = { H_edge(e) | e ∈ edges }
  
  Construct polynomial P_N(x) such that:
    P_N(0) = edge_hashes[0]
    P_N(1) = edge_hashes[1]
    ...
    P_N(k-1) = edge_hashes[k-1]
  
  EdgeSet commitment: C_N = WHIR_commit(P_N)
```

This is where polynomial commitments earn their place: membership proofs within an EdgeSet.

```
Membership proof: "Edge e belongs to neuron N's EdgeSet"
  1. Compute h = H_edge(e)
  2. WHIR evaluation proof: P_N(i) = h for some index i
  3. Verification: ~2,500 stark constraints
  
vs. Merkle membership within EdgeSet:
  Depth log(k) where k = edges per neuron
  Cost: log(k) × 250 constraints
  For k = 1,000: 10 × 250 = 2,500 constraints (comparable)
  For k = 1,000,000: 20 × 250 = 5,000 constraints (Merkle wins)
```

The polynomial advantage manifests in batch proofs: proving multiple edges belong to the same EdgeSet costs sublinearly in the number of edges, via batched WHIR openings. This matters for transaction verification (a cyberlink touches 3 EdgeSets) and for cross-index consistency proofs.

### 4.3 NMT Leaf Structure

```
by_neuron leaf:
  ┌────────────────────────────────────────┐
  │ namespace: neuron_id (32 bytes)        │
  │ payload:                               │
  │   edge_count: u64                      │
  │   edgeset_commitment: F_p⁴ (WHIR)      │
  │   total_weight: F_p                    │
  │   latest_timestamp: u64               │
  └────────────────────────────────────────┘

by_particle leaf:
  ┌────────────────────────────────────────┐
  │ namespace: particle_hash (32 bytes)    │
  │ payload:                               │
  │   edge_count: u64                      │
  │   edgeset_commitment: F_p⁴ (WHIR)      │
  │   inbound_weight: F_p                  │
  │   outbound_weight: F_p                 │
  └────────────────────────────────────────┘

focus leaf:
  ┌────────────────────────────────────────┐
  │ namespace: neuron_id (32 bytes)        │
  │ payload:                               │
  │   focus_value: F_p                     │
  │   last_regeneration: u64              │
  └────────────────────────────────────────┘

balance leaf:
  ┌────────────────────────────────────────┐
  │ namespace: neuron_id (32 bytes)        │
  │ payload:                               │
  │   balance_value: F_p                   │
  │   nonce: u64                          │
  └────────────────────────────────────────┘

coins leaf:
  ┌────────────────────────────────────────┐
  │ namespace: denom_hash (32 bytes)       │
  │ payload:                               │
  │   total_supply: F_p                    │
  │   mint_authority: F_p⁴ (neuron hash)  │
  │   max_supply: F_p (0 = unlimited)     │
  │   transfer_rules: u8 (bitfield)       │
  └────────────────────────────────────────┘

cards leaf:
  ┌────────────────────────────────────────┐
  │ namespace: card_id (32 bytes)          │
  │ payload:                               │
  │   bound_particle: F_p⁴                │
  │   current_owner: F_p⁴ (neuron hash)  │
  │   creator: F_p⁴ (neuron hash)        │
  │   creation_time: u64                  │
  └────────────────────────────────────────┘
```

### 4.4 Edge Structure

```
edge = Cell(neuron, Cell(from, Cell(to, Cell(weight, time))))

where:
  neuron : F_p⁴     — hash of creator's public key
  from   : F_p⁴     — particle hash (source)
  to     : F_p⁴     — particle hash (target)
  weight : F_p       — unsigned weight (economic commitment)
  time   : u64       — block height of creation

Identity: H_edge(edge) — content-addressed, immutable

Storage: edge_store : H_edge(e) → e
         Content-addressed. Stored once. Referenced by index.
```

### 4.5 Consistency Invariant

```
INVARIANT (enforced by stark on every state transition)
───────────────────────────────────────────────────────

For every edge e = (neuron, from, to, weight, time):

  1. H_edge(e) ∈ by_neuron[neuron].EdgeSet
  2. H_edge(e) ∈ by_particle[from].EdgeSet
  3. H_edge(e) ∈ by_particle[to].EdgeSet

Multiplicity:
  If from ≠ to: H_edge(e) appears in exactly 3 EdgeSets
  If from = to: H_edge(e) appears in exactly 2 EdgeSets (self-link)

Cross-index consistency enforced via LogUp lookup arguments (§7).
```

---

## 5. Edge Store

The edge store is a content-addressed key-value map, separate from the index layer: `edge_store : H_edge(e) → e`

- identity = hash. Two identical edges produce the same key
- immutable. Once stored, never modified
- off-graph. The live graph (NMTs + EdgeSets) contains only hashes and commitments. Full edge data lives in the edge store
- retrievable. Any node with the hash can request the edge from any peer
- forgettable. Old edges can be dropped from local storage. The hash in the index is permanent; the data is not

Failure mode: if edge data is requested but unavailable → ⊥_unavailable. The system degrades gracefully — missing data is a content retrieval failure, not a [[consensus]] failure. The graph root remains valid.

### 5.1 CID-Verified Blob Storage

Edge payloads (the actual content that particles reference) are stored off-graph via content-addressed blobs:

```
Tiered availability:
  L1: Hot graph — NMT roots, EdgeSet commitments, focus/balance values
      Size: O(neurons + particles) — megabytes to gigabytes
      Latency: sub-millisecond (in-memory)
      
  L2: Edge store — full edge data, indexed by H_edge(e)
      Size: O(edges) — gigabytes to terabytes
      Latency: milliseconds (SSD)
      
  L3: Blob store — particle content, indexed by CID
      Size: unbounded — petabytes across network
      Latency: seconds (network retrieval)
      
  L4: Archival — historical state snapshots, proofs
      Size: unbounded
      Latency: minutes to hours
```

---

# Part III: The Privacy Layer

## 6. The Mutator Set

### 6.1 Motivation

The [[cybergraph]] is public — edges, [[particles]], [[focus]] distribution are all visible. This is necessary for collective intelligence: the network must see the graph to compute [[focus]].

But economic transactions (who owns how much, who sent to whom) must be private. Without privacy, participation reveals wealth, and surveillance kills the freedom to link.

The [[mutator set]] provides: private ownership, unlinkable transactions, no trusted parties, and logarithmic verification — simultaneously.

### 6.2 Why Not Polynomial + Nullifier

The standard approach (Zcash, v0.9) uses polynomial commitments for the UTXO set and a sorted nullifier set for double-spend prevention. This has a fatal flaw at planetary scale:

The nullifier set grows monotonically with every spend, forever.

```
Year 1:   10⁸ transactions  → 10⁸ nullifiers → prover stores 800 MB
Year 10:  10¹¹ cumulative   → 10¹¹ nullifiers → prover stores 800 GB  
Year 50:  10¹³ cumulative   → 10¹³ nullifiers → prover stores 80 TB

Non-membership proof cost grows logarithmically but never stops:
  WHIR eval against degree-10¹³ polynomial: ~47 folding layers
  Inside stark: ~12,000 constraints per nullifier check

And the total circuit cost (including nullifier non-membership proofs)
is ~49,500 constraints — NEARLY IDENTICAL to the mutator set approach
when honestly accounted.
```

The claimed 4× circuit advantage of polynomial-over-Merkle evaporates when you include nullifier non-membership proofs. The polynomial approach saves on inclusion but spends those savings on exclusion.

### 6.3 Mutator Set Architecture

The mutator set replaces both the UTXO commitment polynomial AND the nullifier set with two linked structures:

#### AOCL — Append-Only Commitment List

An MMR (Merkle Mountain Range) storing addition records:

```
Addition record for UTXO u:
  ar = H_commit(u.particle ‖ u.value ‖ u.owner ‖ u.nonce ‖ r)
  
  where r is hiding randomness contributed by the recipient

Properties:
  - Appended when a UTXO is created. Never modified.
  - MMR structure: forest of perfect binary trees
  - Peaks = O(log N) hash digests
  - Membership proof: Merkle path from leaf to peak, O(log N) hashes
  - Append cost: O(1) amortized (merge adjacent equal-height peaks)
```

MMR structure after 11 insertions:

```
Peak 0 (height 3):           Peak 1 (height 1):   Peak 2 (height 0):
        h₇                         h₁₀                   h₁₁
       /   \                       /   \                    │
     h₃     h₆                  h₈    h₉                 ar₁₁
    / \    / \                   │      │
  h₁  h₂ h₄  h₅               ar₈   ar₉
  │   │   │   │
 ar₀ ar₁ ar₂ ar₃  ar₄ ar₅ ar₆ ar₇

Accumulator = (peak₀.root, peak₁.root, peak₂.root) — 3 hash digests
```

#### SWBF — Sliding-Window Bloom Filter

Tracks which UTXOs have been spent by setting pseudorandom bit positions:

```
Removal record for UTXO u (with AOCL index l, randomness r):
  bit_indices = derive_indices(H_nullifier(u ‖ l ‖ r))
  
  Spending u:
    1. Compute bit_indices from (u, l, r)
    2. For each index in active window: set the bit
    3. For each index in inactive window: provide MMR membership proof
    4. Provide ZK proof that indices were correctly derived from a valid AOCL entry

Double-spend prevention:
  Second spend attempt → all bits already set → verifier rejects

Unlinkability:
  Addition record: H_commit(u ‖ r) — hash commitment
  Removal record: bit positions in Bloom filter
  These share ZERO structural similarity visible to any observer
  The ZK proof establishes validity without revealing which AOCL entry
```

Sliding window mechanism:

```
┌─────────────────────────────────────────────────────────────────┐
│                        SWBF Timeline                            │
│                                                                 │
│  ◄──── Inactive (compacted in MMR) ────►◄── Active Window ──►  │
│                                                                 │
│  ┌──────┬──────┬──────┬──────┐  ┌──────────────────────────┐   │
│  │chunk₀│chunk₁│chunk₂│chunk₃│  │   2²⁰ bits (128 KB)     │   │
│  │(MMR) │(MMR) │(MMR) │(MMR) │  │   directly accessible    │   │
│  └──────┴──────┴──────┴──────┘  └──────────────────────────┘   │
│                                                                 │
│  Old removals: bits in MMR,     New removals: bits in window   │
│  proofs via Merkle paths        proofs via direct lookup        │
│                                                                 │
│  Window slides forward periodically.                            │
│  Oldest active chunk → compacted into MMR.                      │
│  Growth: O(log N) peaks regardless of chain age.                │
└─────────────────────────────────────────────────────────────────┘
```

### 6.4 Record Model

```rust
struct Record {
    particle: [F_p; 4],  // Content identifier (which particle this value is bound to)
    value:    u64,        // Energy amount
    owner:    [F_p; 4],   // Owner public key hash
    nonce:    F_p,        // Random for uniqueness
}
```

Commitment:

$$\text{commit}(r, \rho) = H_{\text{commit}}(r.\text{particle} \| r.\text{value} \| r.\text{owner} \| r.\text{nonce} \| \rho)$$

where $\rho$ is hiding randomness. The commitment is a Pedersen-like hiding commitment using Poseidon2.

### 6.5 Transaction Circuit

A private transfer proves: "I own these inputs, they exist in the [[mutator set]], I'm spending them correctly, and energy is conserved."

```
PRIVATE TRANSFER CIRCUIT
════════════════════════

PUBLIC INPUTS:
  aocl_peaks:    [F_p⁴; log(N)]     AOCL MMR peak hashes
  swbf_root:     F_p⁴               SWBF inactive chunks MMR root
  swbf_window:   F_p⁴               Hash of active SWBF window
  removal_data:  [BitIndices; 4]     SWBF bit positions for each input
  additions:     [F_p⁴; 4]          New addition records
  deltas:        [(F_p⁴, i64); 8]   Per-particle value changes
  fee:           u64                 Transaction fee

PRIVATE WITNESS:
  input_records:   [Record; 4]       Records being spent
  input_secrets:   [F_p; 4]          Ownership secrets
  input_randomness:[F_p; 4]          Hiding randomness (ρ)
  aocl_indices:    [u64; 4]          Insertion positions in AOCL
  aocl_paths:      [MerklePath; 4]   AOCL membership proofs
  swbf_paths:      [MerklePath; 4]   SWBF inactive chunk proofs
  output_records:  [Record; 4]       New records being created
  output_randomness:[F_p; 4]         New hiding randomness
  input_enabled:   [bool; 4]         Which inputs are active
  output_enabled:  [bool; 4]         Which outputs are active

CONSTRAINTS:
─────────────────────────────────────────────────────────

SECTION 1: INPUT VALIDATION  (~48,000 constraints for 4 inputs)

  For each active input i:

    1.1 Commitment correctness                        ~250 constraints
        commit_i = H_commit(record_i ‖ ρ_i)
        
    1.2 AOCL membership (MMR Merkle path)           ~8,000 constraints
        Verify commit_i at aocl_indices[i] against aocl_peaks
        Path depth: ~32 levels × 250 constraints/hash
        
    1.3 SWBF index derivation                         ~500 constraints
        indices_i = derive_indices(H_nullifier(record_i ‖ aocl_indices[i] ‖ ρ_i))
        Poseidon2 + index arithmetic
        
    1.4 SWBF bit verification                       ~3,000 constraints
        For indices in inactive window:
          MMR membership proofs against swbf_root
        For indices in active window:
          Direct lookup against swbf_window hash
        Verify NO bits already set (prevents double-spend)
        
    1.5 Ownership proof                               ~250 constraints
        H(secret_i) = record_i.owner
        
    Per input total:                               ~12,000 constraints
    4 inputs:                                      ~48,000 constraints


SECTION 2: OUTPUT VALIDATION  (~1,250 constraints)

  For each active output j:

    2.1 Commitment correctness                        ~250 constraints
        addition_j = H_commit(output_record_j ‖ output_ρ_j)
        Must match public addition record
        
    2.2 Range check (64-bit value)                     ~64 constraints
    
    4 outputs:                                      ~1,256 constraints


SECTION 3: CONSERVATION  (~100 constraints)

    Σ input_values = Σ output_values + fee            ~8 constraints
    Fee non-negative                                  ~64 constraints
    Boolean flags valid                               ~16 constraints


SECTION 4: DELTA CONSISTENCY  (~300 constraints)

    Per-particle energy changes match records          ~300 constraints
    (public aggregates for graph-level accounting)


SECTION 5: UNIQUENESS  (~50 constraints)

    All removal records distinct                       ~24 constraints
    All addition records distinct                      ~24 constraints


════════════════════════════════════════════════════════════
TOTAL:                                           ~50,000 constraints
Proof generation (Plonky2-class):                ~1.5-3.0 seconds
Proof size:                                      ~120-200 KB
Verification:                                    ~5-10 ms
════════════════════════════════════════════════════════════
```

### 6.6 Proof Maintenance

Every UTXO holder must keep their proofs synchronized as the [[mutator set]] evolves. This is the cost of privacy.

```
EVENT: New UTXO created (addition to AOCL)
  Impact: New leaf appended to AOCL MMR
  Your proof: AOCL Merkle path may need update IF your subtree gains a new peak
  Frequency: O(log L) times over L total appends
  Cost per update: O(log N) hash computations
  Expected per block: O(1) — most blocks don't merge your subtree

EVENT: Old UTXO spent (removal from SWBF)
  Impact: Bits set in SWBF, possibly in inactive chunks you reference
  Your proof: SWBF MMR proofs may need update for affected inactive chunks
  Worst case: O(U · log N) over UTXO lifetime (U = UTXO set size)
  Average case: O(log L · log N) — most removals don't affect your proofs

EVENT: Window slides
  Impact: Active chunk → compacted into inactive MMR
  Your proof: If your bits were in the chunk being compacted, new MMR path needed
  Frequency: Periodic (every W blocks where W = window period)
  Cost: O(log N) per compaction that affects your proof

TOTAL USER COST:
  Average: O(log L · log N) per UTXO lifetime
  Worst:   O((U + log L) · log N)
  
  For practical parameters (10⁹ users, 10-year UTXO):
    Average: ~50 hash operations per block
    Worst:   ~10,000 hash operations per block (rare)
    
  This is ~50 × 250 = 12,500 stark constraints per block for maintenance.
  Trivial on modern hardware.
```

### 6.7 Privacy Boundary

```
┌────────────────────────────────────────────────────────────────────────┐
│                     PRIVACY BOUNDARY SPECIFICATION                     │
├────────────────┬─────────────────────┬─────────────────────────────────┤
│    LAYER       │       PUBLIC        │           PRIVATE               │
├────────────────┼─────────────────────┼─────────────────────────────────┤
│   PARTICLE     │ ✓ CID exists        │                                 │
│                │ ✓ Total energy      │                                 │
├────────────────┼─────────────────────┼─────────────────────────────────┤
│   RECORD       │                     │ ✓ Individual value              │
│                │                     │ ✓ Owner identity                │
│                │                     │ ✓ Nonce and randomness          │
├────────────────┼─────────────────────┼─────────────────────────────────┤
│  TRANSACTION   │ ✓ SWBF bit indices  │ ✓ Which records spent           │
│                │ ✓ Addition records  │ ✓ Who spent them                │
│                │ ✓ Δ per particle    │ ✓ New owners                    │
│                │ ✓ Proof validity    │ ✓ Link between add & remove     │
├────────────────┼─────────────────────┼─────────────────────────────────┤
│  CYBERGRAPH    │ ✓ Edges exist (A→B) │ (optional: who created edge     │
│                │ ✓ Weight (aggregate)│  via Tier 2 privacy extension)  │
├────────────────┼─────────────────────┼─────────────────────────────────┤
│    FOCUS       │ ✓ π distribution    │                                 │
│                │ ✓ Rankings          │                                 │
└────────────────┴─────────────────────┴─────────────────────────────────┘

Minimal privacy for collective intelligence:
  Enough transparency for consensus. Enough privacy for participation.
```

---

# Part IV: Cross-Index Integrity

## 7. LogUp Lookup Arguments

### 7.1 The Problem

Each [[cyberlink]] touches 3 EdgeSets (or 2 for self-links). The [[stark]] must prove that the edge hash inserted into by_neuron is the SAME hash inserted into by_particle[from] and by_particle[to].

Without a mechanism for this, each cross-index check requires independent WHIR proofs against each EdgeSet — expensive and not naturally batched.

### 7.2 LogUp Protocol

LogUp proves "set {f₁, ..., f_m} is contained in table {t₁, ..., t_n}" via the algebraic identity:

$$\sum_{i=1}^{m} \frac{1}{\beta - f_i} = \sum_{j=1}^{n} \frac{m_j}{\beta - t_j}$$

evaluated at a random challenge $\beta$ via sumcheck.

For nox:

```
CROSS-INDEX CONSISTENCY via LogUp
─────────────────────────────────

Transaction: Create edge e = (neuron, from, to, weight, time)
Edge hash: h = H_edge(e)

Lookup statement:
  "h appears in by_neuron[neuron].EdgeSet
   AND h appears in by_particle[from].EdgeSet
   AND h appears in by_particle[to].EdgeSet"

LogUp proof:
  1. f = {h, h, h}  (the same hash, looked up 3 times)
  2. t₁ = by_neuron[neuron].EdgeSet evaluations
  3. t₂ = by_particle[from].EdgeSet evaluations
  4. t₃ = by_particle[to].EdgeSet evaluations
  
  Prover constructs the sumcheck polynomial and commits.
  Verifier checks the identity at random β.
  
Prover work: O(k log k) where k = number of edges in transaction
             Proportional to what you TOUCH, not to |G|
             THIS IS BOUNDED LOCALITY

Verifier work: O(log k) — dominated by sumcheck verification

stark constraints: ~500 per edge (sumcheck + challenges)
vs. 3 × 2,500 = 7,500 for independent WHIR proofs per edge
Savings: 15× for cross-index consistency
```

### 7.3 Batch Transaction Verification

LogUp batches naturally. A block containing B transactions with a total of E edges:

```
Block verification:
  All E edges → one LogUp proof for all 3E lookups
  
  Prover: O(E log E) — linear in block size
  Verifier: O(log E) — logarithmic in block size
  
  stark constraints for cross-index consistency of entire block:
    ~500 × E + O(log E) amortization
    
  For a block with 10,000 edges:
    ~5,000,000 constraints for cross-index (vs. 75,000,000 without LogUp)
```

---

# Part V: Incremental Verification

## 8. Nova/HyperNova Folding

### 8.1 The Problem

A light client joining the network must verify the entire chain history. Without folding:

```
Naive verification:
  N blocks × stark_verify(block) per block
  Each stark verification: ~10⁵ operations
  If recursing (proving verification of a proof): ~10⁵ stark constraints per level
  
  For N = 10⁶ blocks: impractical for mobile clients
```

### 8.2 Folding-Based IVC (Incremental Verifiable Computation)

Nova-style folding maintains a constant-size accumulator that absorbs each block's proof:

```
FOLDING PROTOCOL
────────────────

Initialize:
  acc₀ = ∅ (empty accumulator)

Per block b:
  proof_b = stark_prove(state_transition_b)
  acc_b = fold(acc_{b-1}, proof_b)
  
  fold() cost: O(1) group operations — CONSTANT regardless of history
  acc size: O(1) — constant regardless of chain length

Finalize (when needed):
  final_proof = compress(acc_n)
  compress() cost: O(n) but done ONCE, on demand
  
  final_proof verifies: "the entire chain from genesis to block n
                         is valid"
  
  Verification of final_proof: O(1) — single stark verification

LIGHT CLIENT PROTOCOL:
  1. Receive current acc_n from any peer
  2. Receive current state root
  3. Verify compress(acc_n) against state root
  4. Sync forward: fold each new block into local accumulator
  
  Join cost: ONE stark verification (regardless of chain length)
  Ongoing cost: ONE fold per block (constant)
```

### 8.3 Integration with BBG

The folding accumulator becomes part of the checkpointed state:

```
CHECKPOINT = (
  G_root,           ← NMT roots for cybergraph
  mutator_set_acc,   ← AOCL peaks + SWBF root + window hash
  folding_acc,       ← Nova accumulator (constant size)
  block_height       ← current height
)

Checkpoint size: O(1) — a few hundred bytes
Contains: proof that ALL history from genesis is valid
Updated: O(1) per block via folding
```

### 8.4 HyperNova Extension

HyperNova extends Nova to support multiple instances and more efficient folding for complex circuits. For nox, this enables:

- Parallel folding: Multiple block proofs folded simultaneously across shard boundaries
- CCS compatibility: Direct folding of nox's constraint system without R1CS conversion
- Sub-linear verification: Verification of folded proof is sub-linear in circuit size

Migration from Nova to HyperNova is internal to the folding layer — no changes to the BBG structure or proof interfaces.

---

# Part VI: World State

## 9. Unified State Root

### 9.1 BBG Root

All authenticated state is committed under a single root:

```
BBG_root = H_merkle(
  by_neuron.root       ‖    NMT root (cybergraph by creator)
  by_particle.root     ‖    NMT root (cybergraph by endpoint)
  focus.root           ‖    NMT root (focus distribution)
  balance.root         ‖    NMT root (public balances)
  coins.root           ‖    NMT root (fungible token denominations)
  cards.root           ‖    NMT root (non-fungible knowledge assets)
  aocl.peaks_hash      ‖    H(peak₀ ‖ peak₁ ‖ ... ‖ peak_k)
  swbf.inactive_root   ‖    MMR root (inactive SWBF chunks)
  swbf.window_hash     ‖    H(active window bits)
  particle_energy.root      NMT root (public aggregate energy per particle)
)
```

### 9.2 State Diagram

```
                              ┌──────────────┐
                              │   BBG_root   │
                              └──────┬───────┘
                                     │
     ┌──────────┬────────────┬───────┼───────┬──────────┬───────────┐
     │          │            │       │       │          │           │
┌────┴─────┐ ┌─┴────────┐ ┌─┴────┐ ┌┴─────┐ ┌┴──────┐ ┌┴─────┐ ┌──┴──────────┐
│by_neuron │ │by_particle│ │focus │ │balance│ │coins │ │cards │ │ mutator_set │
│  (NMT)   │ │  (NMT)    │ │(NMT)│ │(NMT) │ │(NMT) │ │(NMT) │ │(AOCL+SWBF)  │
└────┬─────┘ └──┬───────┘ └──────┘ └──────┘ └──────┘ └──────┘ └──┬──────────┘
     │          │                                                   │
┌────┴─────┐ ┌──┴───────┐                                    ┌───┴────────┐
│ EdgeSets │ │ EdgeSets │                                    │    AOCL    │
│  (WHIR)  │ │  (WHIR)  │                                    │   (MMR)    │
└──────────┘ └──────────┘                                    ├────────────┤
     │              │                                         │    SWBF    │
     └──────┬───────┘                                         │(MMR+Window)│
            │                                                 └────────────┘
     ┌──────┴───────┐
     │  edge_store   │
     │(content-addr) │
     │  H(e) → e     │
     └──────────────┘
```

### 9.3 State Transition

```
TRANSITION: W × Transaction → W' | ⊥

TRANSACTION TYPES:
──────────────────

1. CYBERLINK — Add edge to graph
   Input:  (neuron, from_particle, to_particle, weight, signature)
   Effect: Insert into by_neuron[neuron], by_particle[from], by_particle[to]
   Cost:   focus proportional to weight
   Proof:  stark proving 3 EdgeSet updates + LogUp consistency + focus deduction

2. PUBLIC TRANSFER — Move balance between neurons
   Input:  (from_neuron, to_neuron, amount, signature)
   Effect: Update balance NMT leaves
   Cost:   fixed fee
   Proof:  stark proving balance conservation + signature validity

3. PRIVATE TRANSFER — Move energy between records
   Input:  (removal_records, addition_records, deltas, fee, zk_proof)
   Effect: Update mutator set (AOCL append + SWBF bit set)
   Cost:   fee (publicly committed)
   Proof:  ZK proof of spend validity + conservation (§6.5)

4. COMPUTATION — Execute nox reduction
   Input:  (neuron, subject, formula, budget, signature)
   Effect: Consumes focus, produces result
   Cost:   focus proportional to computation steps
   Proof:  stark proving reduction trace + focus deduction

5. MINT CARD — Create a non-fungible knowledge asset
   Input:  (neuron, bound_particle, signature)
   Effect: Insert into cards NMT, creator = owner = neuron
   Cost:   focus fee (prevents spam minting)
   Proof:  stark proving particle exists in by_particle + card_id uniqueness
   The card is itself content-addressed: card_id = H(particle ‖ creator)

6. TRANSFER CARD — Transfer knowledge asset ownership
   Input:  (from_neuron, to_neuron, card_id, signature)
   Effect: Update owner field in cards NMT leaf
   Cost:   fixed fee
   Proof:  stark proving current ownership + signature validity

VALIDITY CONDITIONS:
────────────────────
  1. Authorization: valid signature OR valid ZK proof
  2. Balance sufficiency: balance ≥ transfer amount
  3. Focus sufficiency: focus ≥ computation cost
  4. Conservation: inputs = outputs + fee (for transfers)
  5. Consistency: cross-index LogUp proof valid (for cyberlinks)
  6. Non-duplication: no double-spend (SWBF check for private transfers)
  7. Temporal: timestamp within acceptable range
```

---

# Part VII: Data Availability

## 10. DAS with Namespace-Aware Sampling

### 10.1 2D Reed-Solomon Erasure Coding

Block data is arranged in a √n × √n grid and erasure-coded in both dimensions:

```
ORIGINAL DATA (k × k):          EXTENDED DATA (2k × 2k):

┌─────┬─────┬─────┐            ┌─────┬─────┬─────┬─────┐
│ d₀₀ │ d₀₁ │ d₀₂ │            │ d₀₀ │ d₀₁ │ d₀₂ │ p₀₃ │
├─────┼─────┼─────┤            ├─────┼─────┼─────┼─────┤
│ d₁₀ │ d₁₁ │ d₁₂ │  ──RS──►  │ d₁₀ │ d₁₁ │ d₁₂ │ p₁₃ │
├─────┼─────┼─────┤            ├─────┼─────┼─────┼─────┤
│ d₂₀ │ d₂₁ │ d₂₂ │            │ d₂₀ │ d₂₁ │ d₂₂ │ p₂₃ │
└─────┴─────┴─────┘            ├─────┼─────┼─────┼─────┤
                                │ p₃₀ │ p₃₁ │ p₃₂ │ p₃₃ │
                                └─────┴─────┴─────┴─────┘
                                
RS encoding over F_p (Goldilocks field).
Any k of 2k values in a row → reconstructs the row.
Any k of 2k values in a column → reconstructs the column.
```

### 10.2 NMT Commitment Structure

```
For each row i:
  row_nmt_root_i = NMT_commit(row_i_cells, sorted by namespace)

Column NMT:
  col_nmt_root = NMT_commit([row_nmt_root_0, ..., row_nmt_root_{2k-1}])

Block data commitment:
  data_root = col_nmt_root

NAMESPACE-AWARE SAMPLING:
  Light client interested in neuron N:
    1. col_nmt tells which rows contain namespace N
    2. Sample random cells from THOSE rows
    3. Each cell comes with:
       a) Row NMT inclusion proof (proves cell belongs to row)
       b) Column NMT inclusion proof (proves row belongs to block)
       c) Namespace proof (proves cell is in correct namespace)
    4. If enough cells are available → data is available with high probability

  Sampling complexity: O(√n) cells for 99.9% confidence
  Each sample: O(log n) proof size
```

### 10.3 Fraud Proofs for Bad Encoding

If a block producer encodes a row incorrectly:

```
ENCODING FRAUD PROOF:
  1. Obtain enough cells from the row (k+1 out of 2k)
  2. Attempt Reed-Solomon decoding
  3. If decoded polynomial doesn't match claimed row NMT root:
     → Fraud proof = the k+1 cells with their NMT proofs
     → Any verifier can check: decode(cells) ≠ row commitment
     → Block rejected

Size of fraud proof: O(k) cells with O(log n) proofs each
Verification: O(k log n) — linear in row size, logarithmic in block size
```

---

# Part VIII: Temporal Dynamics

## 11. [[focus]] Computation via [[tri-kernel]]

### 11.1 The Locality Constraint

At 10¹⁵ nodes, any algorithm requiring global recomputation for a local change is physically impossible (Law 1). Systematic elimination of all graph ranking algorithms under this constraint leaves exactly three operator families. No others survive. See [[tri-kernel]] for the completeness proof.

These three operators — [[diffusion]], [[springs]], [[heat kernel]] — are design necessities. They are what remains after impossibility eliminates everything else.

### 11.2 The Three Operators

#### [[diffusion]] (Markov/Random Walk)

$$\pi^{(t+1)} = \alpha P^T \pi^{(t)} + (1-\alpha)u$$

Transition matrix P = AD⁻¹, teleport α, prior u (stake-weighted). Powers remain local. Converges to unique stationary distribution under ergodicity.

Answers: "Where does probability flow?" The exploration component of the [[cyberank]].

- row-stochastic, preserves probability mass
- locality: geometric decay via teleport parameter α
- [[stark]] cost: O(deg(v)) field operations per node update

#### [[springs]] (Screened Laplacian)

$$(L + \mu I)x^* = \mu x_0$$

Graph Laplacian L = D − A, screening μ > 0, reference x₀. The screened Green's function (L+μI)⁻¹ has exponential decay.

Answers: "What satisfies structural constraints?" Encodes hierarchy — keeps connected nodes at consistent levels. Deviation from structural equilibrium is detectable via residual.

- positive semi-definite, null space = constant vectors
- locality: exponential decay via screening parameter μ
- [[stark]] cost: O(deg(v)) field operations per node update

#### [[heat kernel]] (Multi-scale Smoothing)

$$\partial H / \partial \tau = -LH, \quad H_0 = I, \quad H_\tau = \exp(-\tau L)$$

Temperature τ controls scale. Chebyshev polynomial approximation gives h-local computation with bounded error.

Answers: "What does the graph look like at scale τ?" High τ explores (annealing), low τ commits (crystallization). The thermostat of collective attention.

- positivity-preserving, semigroup ($H_{\tau_1} H_{\tau_2} = H_{\tau_1 + \tau_2}$)
- locality: Gaussian tail decay, h = O(log(1/ε)) hops
- [[stark]] cost: O(K · deg(v)) for K-term Chebyshev approximation

#### Composite Update

$$\varphi^{(t+1)} = \text{norm}[\lambda_d \cdot D(\varphi^t) + \lambda_s \cdot S(\varphi^t) + \lambda_h \cdot H_\tau(\varphi^t)]$$

where λ_d + λ_s + λ_h = 1. Fixed point minimizes the free-energy functional:

$$F(\varphi) = \lambda_s \left[\tfrac{1}{2}\varphi^T L\varphi + \tfrac{\mu}{2}\|\varphi - x_0\|^2\right] + \lambda_h \left[\tfrac{1}{2}\|\varphi - H_\tau\varphi\|^2\right] + \lambda_d \cdot D_{KL}(\varphi \| D\varphi)$$

Structure ([[springs]]) + context ([[heat kernel]]) + exploration ([[diffusion]]). The fixed point φ* is the [[cyberank]] — the per-[[particle]] score of the [[cybergraph]]. The [[tri-kernel]] is complete: no other local operators exist.

### 11.3 Convergence

The composite operator ℛ = norm[λ_d·D + λ_s·S + λ_h·H_τ] is a contraction under standard conditions:

- Diffusion: [[Perron-Frobenius theorem]] guarantees unique stationary distribution under ergodicity (strong connectivity + aperiodicity). Geometric convergence via teleport.
- Springs: Screening μ > 0 ensures (L+μI) is strictly positive definite. Exponential decay of Green's function.
- Heat: Bounded τ ensures H_τ is contractive. Gaussian tail decay.

Composite contraction: Under ergodicity of P, screening μ > 0, and bounded τ, the composite operator ℛ is a contraction with coefficient κ < 1. Hence φ^t → φ* linearly.

Convergence rate: $\|\phi^{(t)} - \phi^*\| \leq C \cdot \kappa^t$ where κ < 1 depends on the spectral gap, screening parameter, and temperature.

### 11.4 Bounded-Locality Focus Update

Focus updates must be local — recomputing the global π for every edge change is O(n), violating Law 1.

When edge (i → j, weight w) is created:

1. DETECT affected neighborhood N_h around edit batch. h = O(log(1/ε)) hops. All three operators have decay guarantees: [[diffusion]] geometric (teleport α), [[springs]] exponential (screening μ), [[heat kernel]] Gaussian tail (temperature τ). Beyond h hops, perturbation < ε. Everything outside N_h is unchanged.
2. PULL boundary conditions from cached φ, boundary flows, Laplacian blocks
3. APPLY local [[diffusion]] on N_h — recompute P columns for affected [[neurons]], fixed-point iteration with boundary injection. Cost: O(deg(i) + deg(j)) per iteration
4. APPLY local [[springs]] on N_h — solve (L + μI)x* = μx₀ on affected subgraph via local CG. Cost: O(deg(v)) per affected node
5. APPLY local [[heat kernel]] on N_h — K-term Chebyshev polynomial filter on affected neighborhood. Cost: O(K · deg(v)) per affected node
6. BLEND: φ_new = norm[λ_d · D(φ) + λ_s · S(φ) + λ_h · H_τ(φ)] — normalize and splice back into global φ

Total cost: O(|N_h| · c) per kernel for average degree c — bounded by what you touch, not by graph size.

[[focus]] NMT update: only leaves for affected [[neurons]] need NMT path recomputation. O(k · log n) hash operations where k = |affected neurons|.

Telemetry per epoch: entropy H(π), negentropy J(π), spectral gap estimate, ℓ₁ drift ‖π^t − π^(t-1)‖, locality radius h, nodes touched.

## 12. Temporal Decay (Forgetting Law)

Edges are not permanent. Without a forgetting mechanism, the [[cybergraph]] grows without bound.

### 12.1 Exponential Weight Decay

$$w_{\text{eff}}(e, t) = e.\text{weight} \cdot \alpha^{(t - e.\text{time})}$$

where $\alpha \in (0, 1)$ is the global decay constant.

- each edge decays independently (bounded locality)
- decayed weight returns to the global [[focus]] pool (conservation)
- edge with w_eff < ε is eligible for pruning
- to keep an edge alive: renew it (pay [[focus]] cost again)

[[stark]]-provable: α^n approximated via Taylor series in F_p — 4 terms gives ~10⁻⁶ precision, ~20 field operations = ~20 [[stark]] constraints.

Conservation invariant with decay: Σ π_i + Σ decayed_weight_pool = 1. The decayed weight pool is a single F_p value in the balance NMT, updated each block as edges age.

### 12.2 Pruning Protocol

Condition: w_eff(e, current_block) < ε

1. Prove w_eff < ε ([[stark]]: ~20 constraints for decay calculation)
2. Remove H_edge(e) from by_neuron[e.neuron].EdgeSet
3. Remove H_edge(e) from by_particle[e.from].EdgeSet
4. Remove H_edge(e) from by_particle[e.to].EdgeSet
5. Return e.weight × α^(age) to decay pool
6. LogUp proof of consistent removal from all 3 EdgeSets

Cost: O(log n) NMT updates + 3 EdgeSet updates + LogUp proof. Pruners earn a fraction of recycled [[focus]].

---

# Part IX: Proof System

## 13. [[stark]] Architecture

### 13.1 Proof Pipeline

```
                    COMPUTATION
                        │
                        ▼
              ┌─────────────────┐
              │  Execution Trace │   Sequence of (state, pattern, result) tuples
              │  over F_p        │   Deterministic. Reproducible.
              └────────┬────────┘
                       │
                       ▼
              ┌─────────────────┐
              │   Trace Table    │   Algebraic Intermediate Representation
              │   Polynomials    │   Columns = registers. Rows = steps.
              └────────┬────────┘
                       │
                       ▼
              ┌─────────────────┐
              │  Constraint      │   Transition constraints (row to row)
              │  Polynomials     │   Boundary constraints (first/last row)
              └────────┬────────┘
                       │
                       ▼
              ┌─────────────────┐
              │      WHIR        │   Low-degree testing
              │   Commitment     │   Hash-based. Post-quantum.
              └────────┬────────┘
                       │
                       ▼
              ┌─────────────────┐
              │   stark Proof    │   Succinct. Non-interactive.
              │   (~100-200 KB)  │   Verifiable by anyone with the root.
              └─────────────────┘
```

### 13.2 Self-Verification

[[nox]] can verify its own [[stark]] proofs using its own reduction patterns. This is the self-referential closure:

1. Computation C produces result R
2. [[stark]] prover generates proof π of "C → R"
3. [[nox]] verification program V checks π: V = reduce(subject=(π, public_inputs), formula=stark_verifier)
4. V produces [[stark]] trace
5. Another [[stark]] proof π' proves "V accepted π"
6. This can recurse, or be folded via Nova

The verifier V is a [[nox]] program: [[Hemera]] = pattern 15 (hash), field arithmetic = patterns 5-8, comparisons = patterns 9-10, control flow = patterns 2, 4. The system closes on itself.

### 13.3 Constraint Costs Reference

```
OPERATION                              stark CONSTRAINTS
─────────────────────────────────────────────────────────

Poseidon2 hash (one invocation)               ~250
NMT Merkle path (depth d)                     ~250 × d
NMT completeness proof (two paths)            ~500 × d
MMR membership proof (depth d)                ~250 × d
WHIR evaluation proof (realistic)             ~2,500-3,500
EdgeSet membership (WHIR within NMT leaf)     ~2,500
LogUp per lookup                              ~50
LogUp sumcheck overhead                       ~500 per batch
SWBF index derivation                         ~500
SWBF bit verification                         ~3,000
Conservation check                            ~100
Signature verification                        ~1,000
Decay computation (4-term Taylor)             ~20

COMPOSITE OPERATIONS:
  Cyberlink transaction (1 edge):             ~12,000
  Private transfer (4 in, 4 out):             ~50,000
  Block verification (10K edges):             ~5,000,000
  Tri-kernel update (local, k neighbors):      ~1,000 × k (diffusion + springs + heat)
```

### 13.4 WHIR as the Low-Degree Test

[[cyber]] uses [[WHIR]] (Weights Help Improving Rate, EUROCRYPT 2025) — the third generation of the FRI family. WHIR introduces weight polynomials from sumcheck to achieve sub-millisecond verification and the smallest proofs of any hash-based polynomial commitment scheme.

```
evolution:
  FRI  (2018)  — baseline Reed-Solomon proximity testing
  STIR (2024)  — rate improvement via shifts (CRYPTO 2024 Best Paper)
  WHIR (2025)  — rate + weight polynomials ← cyber uses this

WHIR achieves:
  proof size:     ~60-120 KB (vs FRI ~100-200 KB)
  verification:   ~1.0 ms (vs FRI ~3.9 ms), 290 μs at 100-bit security
  trusted setup:  none
  post-quantum:   yes

Impact on architecture: NONE. WHIR is a Layer 4 (proof system) swap.
Layers 1-3 (NMT, MMR, SWBF) and Layers 5-7 (consensus, sync, API) unchanged.
```

---

# Part X: Sync Protocol

## 14. Trustless Namespace Synchronization

### 14.1 Full Namespace Sync

```
NAMESPACE SYNC PROTOCOL
───────────────────────

Client wants: "All edges created by neuron N"

  1. REQUEST
     Client → Peer: (by_neuron, namespace=N, state_root=BBG_root)

  2. RESPONSE
     Peer → Client:
       - NMT completeness proof π for namespace N in by_neuron
       - EdgeSet commitment C_N
       - Edge data { e | H_edge(e) ∈ EdgeSet_N }

  3. VERIFY
     Client checks:
       a) π valid against by_neuron.root (extracted from BBG_root)
       b) Received edges: { H_edge(e_i) } matches EdgeSet
       c) WHIR_verify(C_N, { (i, H_edge(e_i)) }) — all edges in committed set

  4. GUARANTEE
     "I have ALL edges created by neuron N. Nothing hidden."

Trust requirement: Only BBG_root (from consensus). Peer is COMPLETELY untrusted.
Data transferred: O(|my_edges|) + O(log |G|) proof overhead
Not transferred: O(|all_edges|) — bounded locality in bandwidth
```

### 14.2 Incremental Sync

```
INCREMENTAL SYNC (after initial sync)
─────────────────────────────────────

Client has: EdgeSet_N at block height h₁
Client wants: Updates through block height h₂

  1. REQUEST
     Client → Peer: (by_neuron, namespace=N, since=h₁, current=h₂)

  2. RESPONSE  
     Peer → Client:
       - New edges: { e | e.time > h₁ AND e.neuron = N }
       - Removed edges: { H_edge(e) | pruned since h₁ }
       - Updated NMT proof for namespace N at height h₂
       - Updated EdgeSet commitment

  3. VERIFY
     Same verification as full sync, but against updated BBG_root at h₂.

Data transferred: O(|changes since h₁|) — NOT O(|all edges|)
```

### 14.3 Light Client Protocol

```
LIGHT CLIENT JOIN
─────────────────

New client joins with no history:

  1. Obtain latest CHECKPOINT = (BBG_root, mutator_set_acc, folding_acc, height)
     from any peer
     
  2. Verify folding_acc:
     final_proof = compress(folding_acc)
     stark_verify(final_proof, BBG_root)
     → This single verification proves ALL history from genesis is valid
     
  3. Sync namespaces of interest (§14.1)
  
  4. Maintain:
     - Fold each new block into local folding_acc (O(1) per block)
     - Update mutator set proofs for owned UTXOs (O(log N) per block)
     - Update NMT proofs for monitored namespaces (O(log n) per block)

Join cost: ONE stark verification + namespace sync
Ongoing cost: O(log N) per block
Storage: O(|my_edges| + |my_UTXOs| × log N)
```

---

# Part XI: Scaling

## 15. Hierarchical Sharding

### 15.1 Shard Model

At 10¹⁵ nodes, no single machine can hold the full state. The [[cybergraph]] must shard.

```
SHARD STRUCTURE
───────────────

Shard S_k = subtree of the cybergraph defined by a namespace range:

  S_k contains all neurons with id ∈ [ns_start_k, ns_end_k)
  
  Each shard maintains:
    - Local by_neuron NMT (leaves in namespace range)
    - Local by_particle NMT (particles referenced by local edges)
    - Local focus/balance NMTs
    - Local mutator set (for UTXOs held by local neurons)
    - Local edge store
  
  Cross-shard edges:
    Edge (neuron_A, from_P, to_Q) where A ∈ S_1, P ∈ S_2, Q ∈ S_3:
      - Stored in S_1 (by_neuron index, since creator is in S_1)
      - Referenced in S_2 and S_3 (by_particle indexes)
      - Cross-shard consistency proved via LogUp across shard boundaries
```

### 15.2 Shard Roots

```
GLOBAL ROOT = H_merkle(shard_root_0 ‖ shard_root_1 ‖ ... ‖ shard_root_{n-1})

Each shard_root_k = BBG_root for shard k

Cross-shard proofs:
  "Edge h exists in S_1's by_neuron AND S_2's by_particle"
  = LogUp proof spanning two shard roots
  = Verified at the global root level
  
Shard count: Dynamic. Split when shard exceeds capacity threshold.
NMT enables natural splitting: just bisect the namespace range.
```

---

# Part XII: Summary

## 16. The Complete Stack

```
LAYER 7:  API / Sync Protocol
          Namespace sync, light client protocol, DAS sampling
          ─────────────────────────────────────────────────

LAYER 6:  Consensus (pluggable)
          Focus-weighted BFT (reference), folding accumulator
          ─────────────────────────────────────────────────

LAYER 5:  Data Availability
          2D Reed-Solomon, NMT over shards, fraud proofs
          ─────────────────────────────────────────────────

LAYER 4:  Proof System
          stark (WHIR), Nova/HyperNova folding
          ─────────────────────────────────────────────────

LAYER 3:  Cross-Index Integrity
          LogUp lookup arguments, conservation proofs
          ─────────────────────────────────────────────────

LAYER 2:  Privacy
          Mutator Set (AOCL + SWBF), ZK transaction circuits
          ─────────────────────────────────────────────────

LAYER 1:  Authenticated State
          NMT (cybergraph), MMR (UTXO history), EdgeSet (WHIR)
          ─────────────────────────────────────────────────

LAYER 0:  Field and Hash
          Goldilocks field, Poseidon2, domain separation
          ─────────────────────────────────────────────────
```

## 17. Primitive Inventory

```
FIVE PRIMITIVES
═══════════════

1. NMT (Namespaced Merkle Tree)
   Uses:     by_neuron, by_particle, focus, balance, particle_energy, DAS
   Provides: Completeness proofs, absence proofs, namespace sync
   Hash:     Poseidon2-Goldilocks
   Heritage: Celestia (production since 2023)

2. MMR (Merkle Mountain Range)  
   Uses:     AOCL (mutator set), SWBF inactive chunks
   Provides: Append-only history, O(log N) membership proofs
   Hash:     Poseidon2-Goldilocks
   Heritage: Grin, Neptune (production since 2019)

3. SWBF (Sliding-Window Bloom Filter)
   Uses:     Double-spend prevention (mutator set)
   Provides: Unlinkable removal, bounded-growth tracking
   Hash:     Poseidon2-Goldilocks (index derivation)
   Heritage: Neptune (production since 2024)

4. WHIR Polynomial Commitments
   Uses:     EdgeSet membership, stark proofs, batch openings
   Provides: O(log² n) membership proofs, algebraic batching
   Field:    Goldilocks
   Heritage: Plonky2, Stwo (production since 2022)

5. LogUp Lookup Arguments
   Uses:     Cross-index consistency, batch edge verification
   Provides: O(k log k) multi-table lookups, bounded-locality proofs
   Field:    Goldilocks (sumcheck over F_p)
   Heritage: Polygon, Scroll (production since 2023)

UNIFIED BY:
  Hash:    Poseidon2-Goldilocks (one function, one analysis, one target)
  Field:   F_p where p = 2⁶⁴ - 2³² + 1 (one arithmetic, one FFT)
  Proofs:  stark (one proof system, post-quantum, hash-based)
```

## 18. Security Properties

```
PROPERTY               MECHANISM                    ASSUMPTION
──────────────────────────────────────────────────────────────────
Collision resistance    Poseidon2 (128-bit)          Algebraic hash security
Completeness            NMT structural invariant     Hash binding
Double-spend prevention SWBF bit collision           Bloom filter parameters
Unlinkability           Mutator set construction     Poseidon2 preimage resistance
Conservation            stark circuit constraint     Soundness of proof system
Post-quantum            Hash-based (no pairings)     Grover bounded by output size
Data availability       NMT + 2D erasure coding      Honest minority sampling
Fork resistance         Focus-weighted BFT           2/3 focus-weighted honest
Verification closure    Self-verifying stark         nox expressiveness
Bounded locality        LogUp + local tri-kernel     Spectral gap + screening μ + temperature τ
```

## 19. Complexity Summary

```
OPERATION                           TIME          SPACE         CONSTRAINTS
──────────────────────────────────────────────────────────────────────────────
Create cyberlink                    O(log n)      O(log n)      ~12,000
Sync namespace (neuron)             O(k + log n)  O(k + log n)  N/A (verifier)
Private transfer (4 in, 4 out)      O(log N)      O(log N)      ~50,000
Focus update (tri-kernel, local)     O(deg(v))     O(deg(v))     ~1,000 × k
  Diffusion (random walk)           O(deg(v))     O(1)          ~500 × deg(v)
  Springs (screened Laplacian)      O(deg(v))     O(1)          ~500 × deg(v)
  Heat kernel (Chebyshev)           O(K·deg(v))   O(K)          ~500 × K × deg(v)
Light client join                   O(1)          O(1)          ~100,000 (one-time)
Light client per-block              O(log N)      O(1)          ~1,000
UTXO proof maintenance              O(log N)/blk  O(log N)      ~12,500
DAS sampling                        O(√n)         O(√n log n)   N/A (verifier)
Edge pruning                        O(log n)      O(log n)      ~8,000
Mint card                           O(log n)      O(log n)      ~4,000
Transfer card                       O(log n)      O(log n)      ~3,000
Block verification (B tx, E edges)  O(E log E)    O(E)          ~5,000 × E
```

Where:
- n = graph size (edges)
- N = total historical transactions
- k = edges in queried namespace
- E = edges in block
- deg(v) = degree of affected vertex

Every operation is bounded by what it touches.
Nothing scales with the total graph size.
This is how you build data structures for a planet.

---

*purpose. link. energy.*
