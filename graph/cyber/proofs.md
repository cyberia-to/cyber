---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: STARK verification, nox STARKs, cyber/stark, STARK, STARK proofs
stake: 29173948768097356
---
# STARK Verification

[[nox]] uses STARKs (Scalable Transparent Arguments of Knowledge) as its proof system. The choice is driven by alignment with nox's design principles: no trusted setup, [[Hemera]]-only security (post-quantum), and native compatibility with [[Goldilocks field]] arithmetic.

## Why STARKs

```
Property          │ SNARK         │ STARK
──────────────────┼───────────────┼─────────────────
Trusted setup     │ Required      │ NOT REQUIRED
Quantum resistant │ No            │ Yes
Proof size        │ ~200 bytes    │ ~100-200 KB
Security basis    │ Discrete log  │ Hash only
Field compatible  │ Specific      │ Any (Goldilocks)
```

## Self-Verification Property

```
THEOREM: The STARK verifier for nox is expressible as a nox program.

STARK verification requires:
  1. Field arithmetic (patterns 5, 7, 8)
  2. Hash computation (pattern 15)
  3. Polynomial evaluation (patterns + recursion)
  4. Merkle verification (pattern 15 + conditionals)

All are nox-native. QED.

CONSEQUENCE:
  verify(proof) can itself be proven
  This enables recursive proof composition
  O(1) verification regardless of computation size
```

This is the key property: the system closes on itself. No trusted external verifier remains.

## Verifier Complexity

```
STARK VERIFIER COMPONENTS       │ Layer 1 only │ With Layer 3 jets
────────────────────────────────┼──────────────┼──────────────────
1. Parse proof                  │     ~1,000   │    ~1,000
2. Fiat-Shamir challenges       │    ~30,000   │    ~5,000  (hash jet)
3. Merkle verification          │   ~500,000   │   ~50,000  (merkle_verify jet)
4. Constraint evaluation        │    ~10,000   │    ~3,000  (poly_eval jet)
5. FRI verification             │    ~50,000   │   ~10,000  (fri_fold + ntt jets)
────────────────────────────────┼──────────────┼──────────────────
TOTAL                           │   ~600,000   │   ~70,000

~8.5× reduction. This cost is CONSTANT regardless of what was proven.
Layer 3 jets make recursive composition practical.
```

## Recursive Composition

```
Level 0: Prove computation C → proof π₀
Level 1: Prove verify(π₀) → proof π₁ (~100-200 KB)
Level 2: Prove verify(π₁) → proof π₂ (same size)

AGGREGATION:
  N transactions → N proofs
  Verify all N in one nox program
  Prove that verification → single proof

  Result: O(1) on-chain verification for O(N) transactions
```
