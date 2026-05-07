---
tags: cyber, computer science, cryptography
crystal-type: entity
crystal-domain: cyber
alias: sliding-window bloom filter, sliding window bloom filter
---
# SWBF

sliding-window bloom filter. the spending side of the [[mutator set]]. tracks which UTXOs have been spent by setting pseudorandom bit positions derived from the record. prevents double-spending without maintaining a monotonically growing nullifier set.

## the key insight

a traditional nullifier set grows forever — every spend adds a nullifier that can never be removed. the SWBF replaces this with a fixed-size active window plus compacted old chunks. growth is O(log N) regardless of chain age.

## structure

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

## spending a UTXO

```
removal record for UTXO u (with AOCL index l, randomness ρ):

  1. compute bit_indices = derive_indices(H_nullifier(u ‖ l ‖ ρ))
  2. for indices in active window: set the bit
  3. for indices in inactive chunks: provide MMR membership proof
  4. provide ZK proof that indices were correctly derived from a valid AOCL entry

double-spend detection:
  second spend → same bit_indices → all bits already set → verifier rejects
```

## unlinkability

```
addition record (AOCL):   H_commit(record ‖ ρ)        → a hash
removal record (SWBF):    bit positions in bloom filter → integers

these share ZERO structural similarity
no observer can link a creation event to a spending event
the ZK proof establishes the connection privately
```

## properties

| property | value |
|---|---|
| active window size | 2²⁰ bits (128 KB) |
| false positive rate | tunable via number of hash functions and window size |
| accumulator growth | O(log N) — inactive chunks compact into [[MMR]] |
| stark constraints for bit verification | ~3,000 per input |
| stark constraints for index derivation | ~500 per input |

## window sliding

periodically the oldest chunk of the active window slides into the inactive region:

1. the chunk is hashed and appended to the inactive chunks [[MMR]]
2. a new empty chunk extends the active window forward
3. proofs referencing bits in the newly inactive chunk switch from direct lookup to MMR Merkle path

this bounds the active window to a constant size while preserving the ability to verify historical spends via compact MMR proofs.

## comparison with nullifier sets

| property | nullifier set (Zcash model) | SWBF ([[neptune]] model) |
|---|---|---|
| storage growth | unbounded (monotonic) | O(log N) (MMR compaction) |
| non-membership proof | polynomial evaluation (~12,000 constraints) | bit check (~3,000 constraints) |
| structural linkability | nullifier links to commitment | zero structural similarity |
| false positives | impossible | theoretically possible (tunable) |
| total circuit cost | ~49,500 constraints | ~50,000 constraints |

the circuit costs are nearly identical. the SWBF wins on storage growth and structural unlinkability at the cost of a tunable (but negligible) false positive rate.

see [[mutator set]] for the combined architecture, [[AOCL]] for the creation side, [[MMR]] for the underlying compaction structure, [[BBG]] for the graph privacy layer