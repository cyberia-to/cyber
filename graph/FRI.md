---
tags: cyber, computer science, cryptography
crystal-type: entity
crystal-domain: computer science
alias: Fast Reed-Solomon IOP, FRI protocol, FRI proofs
---
# FRI

Fast Reed-Solomon Interactive Oracle Proof. a protocol for proving that a committed function is close to a low-degree polynomial. the core building block of [[stark]] proof systems and [[polynomial commitments]] in [[cyber]].

introduced by Ben-Sasson, Bentov, Horesh, Riabzev (2018). used in StarkWare, Plonky2, Stwo, and [[cyber]].

## what it proves

given a committed evaluation table (values of a function at many points), FRI proves: "this function is a polynomial of degree at most d." this is the low-degree test — the foundation of all stark-based proof systems.

```
CLAIM: committed function f has degree ≤ d

PROOF (interactive, made non-interactive via Fiat-Shamir):
  1. verifier sends random challenge α
  2. prover folds: f'(x) = f_even(x) + α · f_odd(x)
     degree of f' ≤ d/2
  3. repeat until degree is constant
  4. verifier checks final constant against random queries

FOLDING LAYERS:
  f  (degree d)    → query ~log(d) positions
  f' (degree d/2)  → query ~log(d) positions
  f''(degree d/4)  → query ~log(d) positions
  ...
  f* (degree 1)    → check directly

SECURITY: soundness error ≈ (max_degree/|F|)^{num_queries}
  over Goldilocks field: negligible with ~30 queries per layer
```

## use in cyber

[[cyber]] uses [[WHIR]] — the third generation of the FRI family — for three purposes:

| purpose | what is committed | where |
|---|---|---|
| [[stark]] verification | execution trace polynomials | every [[nox]] computation |
| [[EdgeSet]] membership | edge hash polynomial per namespace | [[BBG]] indexes |
| [[polynomial commitment]] proofs | arbitrary polynomial evaluation | graph state queries |

## FRI over Goldilocks

the [[Goldilocks field]] (p = 2⁶⁴ − 2³² + 1) has a multiplicative subgroup of order 2³², enabling FFTs up to length 2³² without extension fields. FRI folding operates directly on F_p — native 64-bit arithmetic, no embedding overhead.

```
FRI parameters for cyber:
  field:           Goldilocks (p = 2⁶⁴ − 2³² + 1)
  folding factor:  2 (halve degree each round)
  blowup factor:   8 (8× redundancy for soundness)
  queries per round: ~30
  hash:            Hemera (Poseidon2-Goldilocks)
```

## constraint costs

```
FRI verification inside stark:
  without jets:  ~50,000 constraints (fri_fold + NTT + Merkle)
  with jets:     ~10,000 constraints (Layer 3 jets accelerate fri_fold + NTT)

EdgeSet membership via FRI:
  evaluation proof:  ~1,000 constraints (vs ~9,600 for Merkle)
  batch proof (N elements): ~1,000 amortized (sublinear via batched openings)
```

the batch proof property is the polynomial advantage: proving N edges belong to the same [[EdgeSet]] costs sublinearly in N, via batched FRI openings. this matters for transaction verification (a [[cyberlink]] touches 3 EdgeSets) and for cross-index [[LogUp]] consistency proofs.

## the evolution: FRI → STIR → WHIR

```
FRI (2018)    →   STIR (2024)     →   WHIR (2024/2025)
baseline           fewer queries        richest queries
306 KiB proofs     160 KiB proofs       157 KiB proofs
3.9 ms verify      3.8 ms verify        1.0 ms verify
CRYPTO 2018        CRYPTO 2024 (BP)     EUROCRYPT 2025
```

all three generations maintain the same interface: Reed-Solomon proximity testing. [[cyber]] uses [[WHIR]] — the final generation. all layers above the proof system remain unchanged from the FRI interface.

see [[WHIR]] for what [[cyber]] uses, [[STIR]] for the intermediate evolution, [[polynomial commitment]] for the commitment scheme, [[EdgeSet]] for edge membership proofs, [[stark]] for the proof system, [[BBG]] for the graph architecture, [[Goldilocks field]] for the arithmetic foundation
