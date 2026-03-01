---
tags: cyber, cip
crystal-type: property
crystal-domain: cyber
alias: privacy model, CORE privacy, ZK privacy
---
# CORE Privacy Layer

Traditional systems force a choice: transparency (everyone sees everything) or privacy (no one can verify anything). Zero-knowledge proofs dissolve this false dichotomy: prove properties without revealing data.

[[CORE]] implements private ownership with public aggregates. Individual record ownership remains hidden—who owns what, who sent to whom—while aggregate properties remain publicly verifiable: total energy per [[particle]], conservation laws, [[focus]] distribution. The network knows that energy is conserved without knowing who holds it. This is the minimal privacy boundary for [[egregore]]: enough transparency for [[consensus]], enough privacy for participation.

The implementation uses a UTXO model with Poseidon commitments, nullifiers for double-spend prevention, and ~10,000-constraint ZK circuits proving conservation. This represents a 4× improvement over naive Merkle-based designs, achieved through polynomial inclusion proofs.

## Privacy Boundary

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

## Record Model

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
commitment(r) = Poseidon(
    COMMITMENT_DOMAIN,
    r.particle,
    r.value,
    r.owner,
    r.nonce
)
```

### Nullifier

```
nullifier(r, secret) = Poseidon(
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

## Authenticated Graph Structures in ZK

### The Core Problem

Inside a ZK circuit, we must prove "this record exists in the UTXO set" while keeping the record private. The naive approach uses Merkle trees:

```
MERKLE TREE APPROACH (what most systems use)
────────────────────────────────────────────
Structure: Binary tree of hashes, depth 32
Proof: 32 sibling hashes forming path to root
Verification: Hash leaf, then hash with siblings up to root

Cost inside ZK circuit:
  - Each hash: ~300 constraints (Poseidon S-box + MDS)
  - 32 levels: 32 × 300 = 9,600 constraints
  - Per input: 9,600 constraints just for inclusion!

For 4 inputs: 38,400 constraints for inclusion alone
```

### The Insight: Field Ops are Cheap

```
OPERATION COSTS IN ZK CIRCUIT
─────────────────────────────
Field addition:       1 constraint
Field multiplication: 1 constraint
Field comparison:     ~64 constraints
Poseidon hash:        ~300 constraints

Ratio: Hash is 300× more expensive than multiply!
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

### Comparison

```
                     │ Merkle Tree    │ Polynomial (FRI)
─────────────────────┼────────────────┼──────────────────
Primary operation    │ Hash           │ Field multiply
Operation cost       │ ~300           │ 1
Operations needed    │ 32             │ ~1,000
Total per proof      │ 9,600          │ ~1,000
4 inputs             │ 38,400         │ ~4,000
─────────────────────┼────────────────┼──────────────────
Improvement          │ baseline       │ ~10× fewer constraints
```

## Transaction Circuit

### Constants

```
MAX_INPUTS  = 4      // Maximum input records per tx
MAX_OUTPUTS = 4      // Maximum output records per tx
MAX_UTXOS   = 2^32   // Maximum UTXO set size (polynomial degree bound)
```

### Circuit Constraints

```
SECTION 1: INPUT VALIDATION (~7,600 constraints)
────────────────────────────────────────────────
For each of 4 possible inputs:

  Commitment correctness (Poseidon):           ~300 constraints
  Polynomial inclusion proof (FRI):          ~1,000 constraints
  Ownership verification (Poseidon):           ~300 constraints
  Nullifier derivation (Poseidon):             ~300 constraints

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
