---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
status: draft
stake: 21118892632327500
---
# nox security

security properties and formal guarantees of [[nox]]

## security bounds

```
SOUNDNESS:       Invalid transactions rejected with probability ≥ 1 - 2^(-128)
PRIVACY:         Cannot distinguish transactions with same public structure
CONSERVATION:    Σ(energy) = initial + minted - burned (mathematically enforced)
QUANTUM RESIST:  Hash-based security only, ~128-bit post-quantum (Grover limit)
```

## attack surface

```
Attack          │ Defense
────────────────┼─────────────────────────────────────────────
Double Spend    │ Nullifier set prevents reuse
Inflation       │ Circuit enforces conservation
Front-Running   │ Privacy hides transaction contents
Sybil           │ Focus proportional to stake
DoS             │ Focus-based metering limits computation
Eclipse         │ Namespace completeness proofs
Replay          │ Nonces and nullifiers ensure uniqueness
Forgery         │ ZK proofs unforgeable without witness
```

## formal properties

### Turing completeness
Theorem: nox is Turing-complete.
Proof: Construct encoding of arbitrary Turing machine M via patterns 0-4, 9. ∎

### confluence
Theorem: nox is confluent.
Proof: Orthogonal rewrite system by Huet-Levy (1980). ∎

### cost determinism
Theorem: Cost is identical across all reduction orders and implementations.
Proof: By structural induction on formula. ∎

### focus conservation
Theorem: $\sum_i \text{focus}(i) = 1$ for all valid states.
Proof: All operations preserve sum; invalid transitions rejected by verification. ∎

### privacy soundness
Theorem: A valid ZK proof implies all circuit constraints are satisfied with probability $\geq 1 - 2^{-128}$.
Proof: By stark/Plonky2 soundness. ∎

### double-spend prevention
Theorem: Same record cannot be spent twice.
Proof:
1. Each record has unique (nonce, owner_secret) pair
2. Nullifier = H(nonce, owner_secret) is deterministic
3. Same record → same nullifier
4. Nullifier set is append-only
5. Transaction rejected if nullifier already in set ∎

## complexity comparison

```
┌─────────────────────┬───────────────┬───────────────┬───────────────┬───────────────┐
│     Operation       │  Traditional  │  Blockchain   │   Database    │     nox       │
│                     │  (RAM model)  │  (Ethereum)   │  (SQL/NoSQL)  │               │
├─────────────────────┼───────────────┼───────────────┼───────────────┼───────────────┤
│ Equality check      │ O(n) compare  │ O(n) compare  │ O(n) compare  │ O(1) hash     │
│ Membership proof    │ O(n) scan     │ O(log n) MPT  │ O(log n) index│ O(log² n) poly│
│ Completeness proof  │ impossible    │ impossible    │ impossible    │ O(log² n) poly│
│ Computation verify  │ O(n) re-exec  │ O(n) re-exec  │ N/A           │ O(log n) stark│
│ Recursive verify    │ O(n) re-exec  │ O(n) re-exec  │ N/A           │ O(1) composed │
│ Privacy + verify    │ incompatible  │ incompatible  │ incompatible  │ O(1) ZK proof │
├─────────────────────┼───────────────┼───────────────┼───────────────┼───────────────┤
│ Cost determinism    │ ✗ cache-dep   │ ~ gas approx  │ ✗ query-dep   │ ✓ structural  │
│ Parallel safety     │ ✗ explicit    │ ✗ sequential  │ ✗ locks       │ ✓ confluent   │
└─────────────────────┴───────────────┴───────────────┴───────────────┴───────────────┘
```

see [[cyber/nox]] for the execution model, [[cyber/bbg]] for the ZK privacy architecture, [[zheng]] for proof verification
