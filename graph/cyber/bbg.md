---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: BBG, Big Badass Graph, privacy model, nox privacy, ZK privacy, cyber/privacy
stake: 43936669831471920
---
# BBG: Big Badass Graph

A naive graph database stores edges and answers queries. "I don't have any edges matching your query" is indistinguishable from "I'm hiding edges from you." Traditional systems require trust. Distributed systems require consensus on complete state. Neither scales.

The BBG solves this through unified polynomial commitments. One primitive handles everything: membership proofs, completeness proofs, indexes, state. One security analysis, one implementation, one mental model.

Edges are stored once but indexed by multiple dimensions—creator, source [[particle]], target [[particle]]. Each index is a sorted polynomial commitment enabling range proofs: "these are ALL edges in this namespace." When you sync your namespace, you receive cryptographic proof that nothing was withheld. The graph cannot exist without its indexes being consistent and complete—this is structural, not policy.

BBG uses polynomial commitments everywhere rather than mixing hash-based structures with polynomial structures. One primitive means one security analysis, one implementation, one mental model. The same [[WHIR]]-based machinery that makes UTXO proofs cheap (~1,000 constraints vs ~9,600 for Merkle) also handles graph completeness proofs.

This makes "sync only my namespace" a mathematical property, not a feature. A light client tracking one [[particle]] downloads only edges touching that [[particle]], with proof that the response is complete. A [[neuron]] syncing its own edges receives proof of its complete history. No trust in the data provider required.

## Structure

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                    BBG: BIG BADASS GRAPH STRUCTURE                         ║
╠═══════════════════════════════════════════════════════════════════════════╣
║                                                                            ║
║  LAYER 0: Edge Store (content-addressed, stored ONCE)                     ║
║  ──────────────────────────────────────────────────────────────           ║
║    edge_store : H(edge) → edge                                            ║
║    where edge = Cell(neuron, Cell(from, Cell(to, Cell(w, t))))           ║
║    No duplication. Identity = hash. Immutable.                            ║
║                                                                            ║
║  LAYER 1: Neuron Index (completeness by creator)                          ║
║  ───────────────────────────────────────────────────                      ║
║    by_neuron : PolynomialCommitment                                       ║
║    - Sorted by (neuron_id, edge_hash)                                     ║
║    - Range proof: "All edges where edge.neuron = n"                       ║
║    - Completeness via sorted range bounds                                 ║
║                                                                            ║
║  LAYER 2: Particle Index (completeness by endpoint)                       ║
║  ────────────────────────────────────────────────────                     ║
║    by_particle : PolynomialCommitment                                     ║
║    - Sorted by (particle_hash, edge_hash)                                 ║
║    - Range proof: "All edges where from=p OR to=p"                        ║
║    - Completeness via sorted range bounds                                 ║
║                                                                            ║
║  LAYER 3: Focus & Balance                                                 ║
║  ────────────────────────                                                 ║
║    focus   : PolynomialCommitment over (neuron_id, F_p) pairs            ║
║    balance : PolynomialCommitment over (neuron_id, F_p) pairs            ║
║                                                                            ║
║  LAYER 4: UTXO State (mutator set)                                       ║
║  ──────────────────────────────────────────────                           ║
║    aocl             : MMR                     (append-only commitment list)║
║    swbf             : SlidingWindowBloomFilter (double-spend prevention)  ║
║    particle_energy  : PolynomialCommitment     (public aggregates)        ║
║                                                                            ║
║  UNIFIED PRIMITIVE: All indexes use polynomial commitments                ║
║    - Membership proof: WHIR evaluation, O(log² n), ~1,000 constraints      ║
║    - Completeness proof: Sorted range bounds, O(log² n)                   ║
║    - One primitive, one security analysis, one implementation             ║
║                                                                            ║
║  GRAPH ROOT                                                               ║
║  ──────────                                                               ║
║    BBG_root = H(                                                            ║
║      by_neuron.commit  ‖                                                  ║
║      by_particle.commit ‖                                                 ║
║      focus.commit ‖                                                       ║
║      balance.commit ‖                                                     ║
║      aocl.peaks ‖                                                         ║
║      swbf.root                                                            ║
║    )                                                                       ║
║                                                                            ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

## Index Consistency Invariant

```
INVARIANT (enforced by stark on every state transition)
───────────────────────────────────────────────────────

For every edge e = (neuron, from, to, weight, time):

  1. H(e) ∈ by_neuron at position for namespace=neuron
  2. H(e) ∈ by_particle at position for namespace=from
  3. H(e) ∈ by_particle at position for namespace=to

  Multiplicity:
    If from ≠ to: H(e) appears in exactly 3 index positions
    If from = to: H(e) appears in exactly 2 index positions (self-link)

Cross-index consistency provable via polynomial identity testing:
  - Same edge hash appears in multiple sorted polynomials
  - WHIR proofs demonstrate membership in each
  - stark proves all memberships consistent

Proof size: O(log² n). Verification: O(log² n) field operations.
```

## Namespace Sync Protocol

```
NAMESPACE SYNC (Polynomial Range Proof)
───────────────────────────────────────

To sync namespace ns (neuron_id or particle_hash):

  1. REQUEST
     Client → Responder: "Give me namespace ns"

  2. RESPONSE
     Responder → Client:
       - Range bounds (i, j) in sorted polynomial
       - WHIR proofs for P(i-1), P(i), P(j), P(j+1)
       - Edge data { e | index i ≤ position ≤ j }

  3. VERIFY
     Client checks:
       a) P(i-1).namespace < ns (or i = 0)
       b) P(i).namespace = ns
       c) P(j).namespace = ns
       d) P(j+1).namespace > ns (or j = end)
       e) Received edges hash to claimed values
       f) All WHIR proofs valid against BBG_root

  4. GUARANTEE
     If verification passes:
       "I have ALL edges in namespace ns. Nothing hidden."

     Proof is mathematical, not trust-based.

Cost: O(|my_edges|) data + O(log² |G|) proof overhead
```

## Privacy Layer (Layer 4 Detail)

[[nox]] implements private ownership with public aggregates. Individual [[record]] ownership remains hidden — who owns what, who sent to whom — while aggregate properties remain publicly verifiable: total energy per [[particle]], conservation laws, [[focus]] distribution. The network knows that energy is conserved without knowing who holds it. This is the minimal privacy boundary for [[egregore]]: enough transparency for [[consensus]], enough privacy for participation.

The implementation uses a [[mutator set]] (AOCL + SWBF) for UTXO lifecycle tracking with [[Hemera]] commitments and ~50,000-constraint ZK circuits proving conservation. Addition and removal records share zero structural similarity visible to any observer — unlinkability is architectural.

### Privacy Boundary

```
┌────────────────────────────────────────────────────────────────────────────┐
│                   PRIVACY BOUNDARY SPECIFICATION                            │
├────────────────┬─────────────────────┬─────────────────────────────────────┤
│    LAYER       │       PUBLIC        │           PRIVATE                   │
├────────────────┼─────────────────────┼─────────────────────────────────────┤
│   PARTICLE     │ ✓ CID exists        │                                     │
│                │ ✓ Total energy      │                                     │
├────────────────┼─────────────────────┼─────────────────────────────────────┤
│   RECORD       │                     │ ✓ Individual value                  │
│                │                     │ ✓ Owner identity                    │
│                │                     │ ✓ Nonce                             │
├────────────────┼─────────────────────┼─────────────────────────────────────┤
│  TRANSACTION   │ ✓ SWBF bit indices  │ ✓ Which records spent               │
│                │ ✓ Addition records  │ ✓ Who spent them                    │
│                │ ✓ Δ per particle    │ ✓ New owners                        │
│                │ ✓ Proof validity    │                                     │
├────────────────┼─────────────────────┼─────────────────────────────────────┤
│    GRAPH       │ ✓ Edges exist (A→B) │ ✓ Who created edge                  │
│                │ ✓ Weight (aggregate)│ ✓ Individual stakes                 │
├────────────────┼─────────────────────┼─────────────────────────────────────┤
│    FOCUS       │ ✓ π distribution    │                                     │
│                │ ✓ Rankings          │                                     │
└────────────────┴─────────────────────┴─────────────────────────────────────┘
```

Invariant: The ZK circuit MUST enforce this boundary. Any violation breaks the economic integrity of collective attention.

### Mutator Set

The mutator set replaces both UTXO commitment polynomials and nullifier sets with two linked structures:

AOCL (Append-Only Commitment List) — an MMR storing addition records. Appended when a UTXO is created, never modified. Accumulator = O(log N) peak hashes. Membership proof = Merkle path from leaf to peak.

SWBF (Sliding-Window Bloom Filter) — tracks which UTXOs have been spent by setting pseudorandom bit positions derived from the record. Double-spend = all bits already set = verifier rejects. Active window (128 KB) handles recent removals directly; older chunks compact into an MMR.

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
│  Window slides forward periodically.                            │
│  Growth: O(log N) peaks regardless of chain age.                │
└─────────────────────────────────────────────────────────────────┘
```

Unlinkability: addition record = `H_commit(record ‖ ρ)`, removal record = SWBF bit positions derived from `H_nullifier(record ‖ aocl_index ‖ ρ)`. These share zero structural similarity. The ZK proof establishes validity without revealing which AOCL entry is being spent.

### Record Structure

```rust
struct Record {
    particle: [F_p; 4],  // Content identifier (which particle this value is bound to)
    value:    u64,        // Energy amount
    owner:    [F_p; 4],   // Owner public key hash
    nonce:    F_p,        // Random for uniqueness
}
```

### Commitment

```
commitment(r, ρ) = H_commit(r.particle ‖ r.value ‖ r.owner ‖ r.nonce ‖ ρ)

where ρ is hiding randomness contributed by the recipient
```

### Transaction Circuit

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
  input_records, input_secrets, input_randomness
  aocl_indices, aocl_paths, swbf_paths
  output_records, output_randomness
  input_enabled, output_enabled
```

```
SECTION 1: INPUT VALIDATION (~48,000 constraints for 4 inputs)
──────────────────────────────────────────────────────────────
For each active input:

  Commitment correctness (Hemera):            ~250 constraints
  AOCL membership (MMR Merkle path):        ~8,000 constraints
  SWBF index derivation:                      ~500 constraints
  SWBF bit verification:                    ~3,000 constraints
  Ownership proof:                            ~250 constraints

  Per input total:                          ~12,000 constraints
  4 inputs:                                 ~48,000 constraints


SECTION 2: OUTPUT VALIDATION                  ~1,250 constraints
SECTION 3: CONSERVATION                         ~100 constraints
SECTION 4: DELTA CONSISTENCY                    ~300 constraints
SECTION 5: UNIQUENESS                            ~50 constraints


TOTAL:                                        ~50,000 constraints
═══════════════════════════════════════════════════════════════════
Proof generation (Plonky2-class):             ~1.5-3.0 seconds
Proof size:                                   ~120-200 KB
Verification:                                 ~5-10 ms
```

### Transaction Types

```
TRANSFER   — Move energy between particles (1-4 in, 1-4 out)
MINT       — Create new energy (0 in, 1 out, special proof)
BURN       — Remove energy from circulation (1-4 in, 0-3 out)
SPLIT      — Divide one record into multiple (1 in, 2-4 out, same particle)
MERGE      — Combine multiple records (2-4 in, 1 out, same particle)
```

see [[data structure for superintelligence]] for full mutator set architecture, proof maintenance, and sliding window mechanics
