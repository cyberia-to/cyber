---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: BBG, Big Badass Graph, privacy model, nox privacy, ZK privacy, cyber/privacy
---
# BBG: Big Badass Graph

A naive graph database stores edges and answers queries. "I don't have any edges matching your query" is indistinguishable from "I'm hiding edges from you." Traditional systems require trust. Distributed systems require consensus on complete state. Neither scales.

The BBG solves this through unified polynomial commitments. One primitive handles everything: membership proofs, completeness proofs, indexes, state. One security analysis, one implementation, one mental model.

Edges are stored once but indexed by multiple dimensions—creator, source [[particle]], target [[particle]]. Each index is a sorted polynomial commitment enabling range proofs: "these are ALL edges in this namespace." When you sync your namespace, you receive cryptographic proof that nothing was withheld. The graph cannot exist without its indexes being consistent and complete—this is structural, not policy.

BBG uses polynomial commitments everywhere rather than mixing hash-based structures with polynomial structures. One primitive means one security analysis, one implementation, one mental model. The same FRI-based machinery that makes UTXO proofs cheap (~1,000 constraints vs ~9,600 for Merkle) also handles graph completeness proofs.

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
║  LAYER 4: UTXO State (for privacy layer)                                  ║
║  ──────────────────────────────────────────────                           ║
║    commitment_poly  : PolynomialCommitment  (UTXO set as polynomial)      ║
║    nullifier_set    : PolynomialCommitment  (spent records, sorted)       ║
║    particle_energy  : PolynomialCommitment  (public aggregates)           ║
║                                                                            ║
║  UNIFIED PRIMITIVE: All indexes use polynomial commitments                ║
║    - Membership proof: FRI evaluation, O(log² n), ~1,000 constraints      ║
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
║      commitment_poly.commit ‖                                             ║
║      nullifier_set.commit                                                 ║
║    )                                                                       ║
║                                                                            ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

## Index Consistency Invariant

```
INVARIANT (enforced by STARK on every state transition)
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
  - FRI proofs demonstrate membership in each
  - STARK proves all memberships consistent

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
       - FRI proofs for P(i-1), P(i), P(j), P(j+1)
       - Edge data { e | index i ≤ position ≤ j }

  3. VERIFY
     Client checks:
       a) P(i-1).namespace < ns (or i = 0)
       b) P(i).namespace = ns
       c) P(j).namespace = ns
       d) P(j+1).namespace > ns (or j = end)
       e) Received edges hash to claimed values
       f) All FRI proofs valid against BBG_root

  4. GUARANTEE
     If verification passes:
       "I have ALL edges in namespace ns. Nothing hidden."

     Proof is mathematical, not trust-based.

Cost: O(|my_edges|) data + O(log² |G|) proof overhead
```

## Privacy Layer (Layer 4 Detail)

[[nox]] implements private ownership with public aggregates. Individual record ownership remains hidden — who owns what, who sent to whom — while aggregate properties remain publicly verifiable: total energy per [[particle]], conservation laws, [[focus]] distribution. The network knows that energy is conserved without knowing who holds it. This is the minimal privacy boundary for [[egregore]]: enough transparency for [[consensus]], enough privacy for participation.

The implementation uses a UTXO model with [[Hemera]] commitments, nullifiers for double-spend prevention, and ~10,000-constraint ZK circuits proving conservation. This represents a 4x improvement over naive Merkle-based designs, achieved through polynomial inclusion proofs.

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
│  TRANSACTION   │ ✓ Nullifiers        │ ✓ Which records spent               │
│                │ ✓ Commitments       │ ✓ Who spent them                    │
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

### Record Structure

```rust
struct Record {
    particle: Field,    // Content identifier (CID → field)
    value: u64,         // Energy amount
    owner: Field,       // Owner public key hash
    nonce: Field,       // Random for uniqueness
}
```

### Commitment

```
commitment(r) = Hemera(
    COMMITMENT_DOMAIN,
    r.particle,
    r.value,
    r.owner,
    r.nonce
)
```

### Nullifier

```
nullifier(r, secret) = Hemera(
    NULLIFIER_DOMAIN,
    r.nonce,
    secret
)

Properties:
  - Cannot derive from commitment (needs secret)
  - Cannot derive commitment from nullifier (one-way)
  - Unique per record
  - Deterministic (same record → same nullifier)
```

### Authenticated Graph Structures in ZK

Inside a ZK circuit, we must prove "this record exists in the UTXO set" while keeping the record private. The naive approach uses Merkle trees:

```
MERKLE TREE APPROACH (what most systems use)
────────────────────────────────────────────
Structure: Binary tree of hashes, depth 32
Proof: 32 sibling hashes forming path to root
Verification: Hash leaf, then hash with siblings up to root

Cost inside ZK circuit:
  - Each hash: ~300 constraints (Hemera S-box + MDS)
  - 32 levels: 32 × 300 = 9,600 constraints
  - Per input: 9,600 constraints just for inclusion!

For 4 inputs: 38,400 constraints for inclusion alone
```

Field operations are cheap in comparison:

```
OPERATION COSTS IN ZK CIRCUIT
─────────────────────────────
Field addition:       1 constraint
Field multiplication: 1 constraint
Field comparison:     ~64 constraints
Hemera hash:          ~300 constraints

Ratio: Hash is 300x more expensive than multiply!
```

### Polynomial Commitment Solution

Following Goodrich & Tamassia's [[authenticated graphs]] principles, we represent the UTXO set as a polynomial:

```
POLYNOMIAL REPRESENTATION
─────────────────────────
Given n commitments {c₀, c₁, ..., c_{n-1}}

Construct polynomial P(x) such that:
  P(0) = c₀
  P(1) = c₁
  ...
  P(n-1) = c_{n-1}

State commitment: C = FRI_commit(P)

Inclusion proof for cᵢ:
  - Prove P(i) = cᵢ using FRI evaluation proof
  - Verification: ~log²(n) field operations
  - Cost: ~1,000 constraints (vs 9,600 for Merkle)
```

```
                     │ Merkle Tree    │ Polynomial (FRI)
─────────────────────┼────────────────┼──────────────────
Primary operation    │ Hash           │ Field multiply
Operation cost       │ ~300           │ 1
Operations needed    │ 32             │ ~1,000
Total per proof      │ 9,600          │ ~1,000
4 inputs             │ 38,400         │ ~4,000
─────────────────────┼────────────────┼──────────────────
Improvement          │ baseline       │ ~10x fewer constraints
```

### Transaction Circuit

```
MAX_INPUTS  = 4      // Maximum input records per tx
MAX_OUTPUTS = 4      // Maximum output records per tx
MAX_UTXOS   = 2^32   // Maximum UTXO set size (polynomial degree bound)
```

```
SECTION 1: INPUT VALIDATION (~7,600 constraints)
────────────────────────────────────────────────
For each of 4 possible inputs:

  Commitment correctness (Hemera):           ~300 constraints
  Polynomial inclusion proof (FRI):          ~1,000 constraints
  Ownership verification (Hemera):           ~300 constraints
  Nullifier derivation (Hemera):             ~300 constraints

  Per input total:                           ~1,900 constraints
  4 inputs maximum:                          ~7,600 constraints


SECTION 2: OUTPUT VALIDATION (~1,500 constraints)
SECTION 3: CONSERVATION (~100 constraints)
SECTION 4: DELTA CONSISTENCY (~300 constraints)
SECTION 5: UNIQUENESS (~50 constraints)


TOTAL: ~10,000 constraints
════════════════════════════════════════════════
With Plonky2/STARK optimizations:   ~7,000 gates
Proof generation time:              ~0.3-0.8 seconds
Proof size:                         ~50-80 KB
Verification time:                  ~1-3 ms
```

### Transaction Types

```
TRANSFER   — Move energy between particles (1-4 in, 1-4 out)
MINT       — Create new energy (0 in, 1 out, special proof)
BURN       — Remove energy from circulation (1-4 in, 0-3 out)
SPLIT      — Divide one record into multiple (1 in, 2-4 out, same particle)
MERGE      — Combine multiple records (2-4 in, 1 out, same particle)
```
