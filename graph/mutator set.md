---
tags: cyber, computer science, cryptography
crystal-type: entity
crystal-domain: cyber
alias: mutator sets
---
# mutator set

a privacy primitive that replaces both UTXO commitment sets and nullifier sets with two linked structures: [[AOCL]] and [[SWBF]]. invented by the [[neptune]] team. used in [[cyber]] for private ownership tracking within the [[cybergraph]].

## the problem it solves

the standard approach (Zcash model) uses polynomial commitments for the UTXO set and a sorted nullifier set for double-spend prevention. the nullifier set grows monotonically with every spend, forever:

```
Year 1:   10⁸ transactions  → 10⁸ nullifiers → 800 MB
Year 10:  10¹¹ cumulative   → 10¹¹ nullifiers → 800 GB
Year 50:  10¹³ cumulative   → 10¹³ nullifiers → 80 TB
```

the mutator set eliminates unbounded nullifier growth by replacing the nullifier set with a [[SWBF]] — a sliding-window [[bloom filter]] that compacts old data into an [[MMR]].

## architecture

```
┌─────────────────────────────────────────────────────┐
│                   MUTATOR SET                        │
├──────────────────────┬──────────────────────────────┤
│        AOCL          │           SWBF               │
│  (append-only        │  (sliding-window             │
│   commitment list)   │   bloom filter)              │
│                      │                              │
│  records creation    │  records spending            │
│  MMR structure       │  active window + inactive    │
│  O(log N) peaks      │  MMR for old chunks          │
│  append-only         │  O(log N) growth             │
└──────────────────────┴──────────────────────────────┘
```

creating a UTXO appends a commitment to the [[AOCL]]. spending a UTXO sets bits in the [[SWBF]]. the two structures share zero structural similarity visible to any observer — unlinkability is architectural.

## unlinkability

```
addition record:  H_commit(record ‖ ρ)     → hash commitment
removal record:   bit positions in SWBF    → derived from H_nullifier(record ‖ aocl_index ‖ ρ)

these share ZERO structural similarity
the ZK proof establishes validity without revealing which AOCL entry is being spent
```

an observer sees a hash being appended (creation) and bits being set (spending). there is no way to correlate the two without the private witness.

## transaction circuit

```
PRIVATE TRANSFER CIRCUIT (~50,000 constraints)
═══════════════════════════════════════════════

Per input (4 max):
  commitment correctness (Hemera):       ~250 constraints
  AOCL membership (MMR Merkle path):   ~8,000 constraints
  SWBF index derivation:                 ~500 constraints
  SWBF bit verification:              ~3,000 constraints
  ownership proof:                       ~250 constraints
  ─────────────────────────────────────────────
  per input total:                    ~12,000 constraints

Output validation:                     ~1,250 constraints
Conservation (inputs = outputs + fee):   ~100 constraints
Delta consistency:                       ~300 constraints
Uniqueness:                               ~50 constraints
═══════════════════════════════════════════════
TOTAL:                                ~50,000 constraints
Proof size:                           ~120-200 KB
Verification:                         ~5-10 ms
```

## proof maintenance

every UTXO holder keeps proofs synchronized as the mutator set evolves. this is the cost of privacy.

| event | impact | cost |
|---|---|---|
| new UTXO created | AOCL MMR may gain new peak | O(log N) hashes, O(1) expected per block |
| old UTXO spent | SWBF bits set, inactive chunks affected | O(log L × log N) average |
| window slides | active chunk compacted into inactive MMR | O(log N) per affected proof |

for practical parameters (10⁹ users, 10-year UTXO): ~50 hash operations per block average. ~12,500 [[stark]] constraints per block for maintenance.

## the [[neptune]] precedent

[[neptune]] (Alan Szepieniec, COSIC/KU Leuven) invented the mutator set and launched mainnet February 2025. [[cyber]] inherits the primitive with its own hash ([[Hemera]] instead of Tip5) and its own VM ([[nox]] instead of Triton VM). same field ([[Goldilocks field]]). same architecture. different instantiation.

see [[AOCL]] for the append-only commitment list, [[SWBF]] for the sliding-window bloom filter, [[BBG]] for how the mutator set fits into the graph privacy architecture, [[record]] for the UTXO data model
