---
tags: cyber, computer science, cryptography
crystal-type: entity
crystal-domain: cyber
alias: append-only commitment list, AOCL
---
# AOCL

append-only commitment list. the creation side of the [[mutator set]]. an [[MMR]] storing addition records — one per UTXO ever created. appended when a UTXO is created, never modified.

## structure

the AOCL is a [[Merkle Mountain Range]]: a forest of perfect binary trees whose heights correspond to the binary representation of the total leaf count.

```
MMR after 11 insertions:

Peak 0 (height 3):           Peak 1 (height 1):   Peak 2 (height 0):
        h₇                         h₁₀                   h₁₁
       /   \                       /   \                    │
     h₃     h₆                  h₈    h₉                 ar₁₁
    / \    / \                   │      │
  h₁  h₂ h₄  h₅               ar₈   ar₉
  │   │   │   │
 ar₀ ar₁ ar₂ ar₃  ar₄ ar₅ ar₆ ar₇

Accumulator = (peak₀.root, peak₁.root, peak₂.root)
```

## addition record

when a [[record]] is created, its commitment is appended:

```
addition_record = H_commit(record.particle ‖ record.value ‖ record.owner ‖ record.nonce ‖ ρ)

where ρ is hiding randomness contributed by the recipient
```

the commitment hides all fields. an observer sees a hash being appended. nothing about the particle, value, owner, or nonce is revealed.

## properties

| property | value |
|---|---|
| accumulator size | O(log N) peak hashes |
| membership proof | Merkle path from leaf to peak, O(log N) hashes |
| append cost | O(1) amortized (merge adjacent equal-height peaks) |
| modification | never — append-only by construction |
| stark constraints for membership | ~23,552 (32 levels × 736 constraints per [[Hemera]] hash) |

## relationship to SWBF

the AOCL stores commitments. the [[SWBF]] tracks which commitments have been spent. together they form the [[mutator set]].

spending a UTXO requires proving two things:
1. the commitment exists in the AOCL (membership proof via Merkle path)
2. the derived bit positions have not been set in the [[SWBF]] (non-double-spend proof)

the AOCL index of a commitment feeds into the [[SWBF]] index derivation: `bit_indices = derive_indices(H_nullifier(record ‖ aocl_index ‖ ρ))`. this links the two structures cryptographically while keeping them structurally unlinkable.

## proof maintenance

as new commitments are appended, the MMR grows. existing membership proofs may need updates when peaks merge:

```
appending ar₁₂ to the 11-element MMR above:
  peak₂ (height 0) merges with ar₁₂ → new peak (height 1)
  new peak merges with peak₁ (height 1) → new peak (height 2)

  proof paths for ar₈, ar₉, ar₁₀, ar₁₁ need update
  proof paths for ar₀..ar₇ remain valid (peak₀ unchanged)
```

expected update frequency: O(log L) times over L total appends. cost per update: O(log N) hash computations.

see [[mutator set]] for the combined architecture, [[SWBF]] for the spending side, [[MMR]] for the underlying data structure, [[BBG]] for the graph privacy layer