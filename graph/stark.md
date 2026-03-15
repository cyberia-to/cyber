---
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
alias: starks, STARK, STARKs, Scalable Transparent Arguments of Knowledge, multilinear stark, multilinear starks
---
# stark

Scalable Transparent Argument of Knowledge. a proof system where a prover convinces a verifier that a computation was performed correctly — transparent setup, post-quantum security, hash-only assumption.

Ben-Sasson, Bentov, Horesh, Riabzev (2018). the foundation of verifiable computation in [[cyber]], [[Ethereum]] L2s (StarkNet, Polygon), and [[Celestia]].

## properties

| property | value |
|---|---|
| trusted setup | none (transparent) |
| post-quantum | yes (hash-only security) |
| proof size | 60–200 KB |
| prover | quasi-linear O(n log n) |
| verifier | polylogarithmic O(log² n) |
| security assumption | collision-resistant hash function |

## arithmetization

a stark proves that a computation satisfies algebraic constraints. the mapping from computation to constraints is called arithmetization. three major constraint systems:

### AIR — Algebraic Intermediate Representation

the native arithmetization for starks. represents computation as a matrix (the execution trace) plus polynomial constraints.

```
EXECUTION TRACE
  matrix: rows = time steps, columns = registers
  example: VM with 16 registers, 1024 steps → 1024 × 16 matrix

TRANSITION CONSTRAINTS
  polynomials relating consecutive rows:
  "column[3] at row t+1 = column[1] at row t × column[2] at row t"
  enforces: each instruction executed correctly

BOUNDARY CONSTRAINTS
  specific values at specific positions:
  "column[0] at row 0 = program_input"
  "column[0] at last row = program_output"
```

AIR constraints can have any degree (commonly 2–8). used by StarkWare/CAIRO, ethstark, Winterfell, Miden, [[cyber]]. the [[nox]] VM's sixteen reduction patterns map directly to AIR transition constraints — each pattern becomes a polynomial equation relating the register state before and after the reduction step.

### R1CS — Rank-1 Constraint System

the native arithmetization for [[SNARKs]]. each constraint has the form `(a · w) × (b · w) = (c · w)` where w is the witness vector and a, b, c are coefficient vectors. degree-2 only. used by Groth16, Spartan, Nova. natural for arithmetic circuits, less natural for sequential VM execution.

### CCS — Customizable Constraint Systems

generalizes R1CS, Plonkish (PLONK/Halo2), and AIR into one framework. Setty, Thaler, Wahby (2023).

```
CCS instance: (M₁, ..., M_t, S₁, ..., S_q, c₁, ..., c_q)

constraint:  Σⱼ cⱼ · ∏_{i ∈ Sⱼ} Mᵢ · z = 0

special cases:
  R1CS:     t=3, q=2, c₁=1, c₂=-1        → degree 2
  Plonkish: selector polynomials → M        → custom gates
  AIR:      shifted rows → M                → transition constraints
```

the unification matters because a proof system handling CCS handles all three — including AIR. [[SuperSpartan]] is this proof system.

## univariate vs multilinear

### univariate starks (classical, 2018)

the original construction. each trace column is interpolated into a univariate polynomial, constraints are checked via polynomial composition and division by a zerofier (vanishing polynomial), and [[FRI]] proves the quotient has bounded degree.

```
pipeline:
  1. execution → trace (N rows × M columns)
  2. interpolate each column → M univariate polynomials of degree N
  3. compose with constraint polynomials
  4. divide by zerofier Z(x) where Z vanishes on the trace domain
  5. FRI low-degree test: quotient Q(x) has degree ≤ d
  6. M commitments, M openings
```

the prover requires FFT/NTT for interpolation — O(N log N) per column. the verifier checks M separate polynomial openings. used by StarkWare/CAIRO, ethstark, early Polygon zkEVM.

### multilinear starks (modern, 2023–2025)

the entire execution trace becomes ONE multilinear polynomial. constraints are verified via the [[sumcheck]] protocol. [[WHIR]] (as a multilinear PCS) opens the commitment at the single point that sumcheck reduces to.

```
pipeline:
  1. execution → trace (2ⁿ rows × 2ᵐ columns)
  2. encode entire trace as ONE multilinear polynomial f(x₁, ..., x_{n+m})
     row index encoded in n boolean variables
     column index encoded in m boolean variables
     each variable has degree ≤ 1
  3. express constraints as CCS (AIR maps directly)
  4. sumcheck reduces ALL constraint checks to ONE evaluation at ONE random point r
  5. WHIR opens f(r) — one commitment, one opening
```

a multilinear polynomial in k variables:
```
f(x₁, ..., x_k) = Σ_{S ⊆ {1,...,k}} c_S · ∏_{i ∈ S} xᵢ

every variable appears with degree at most 1
example: f(x,y,z) = 3xy + 2xz + yz + x + 5
```

advantages over univariate:

| property | univariate | multilinear |
|---|---|---|
| commitments | M (one per column) | 1 (entire trace) |
| openings | M | 1 |
| constraint prover | O(N log N) per column (FFT) | O(N) total (field ops only) |
| constraint verifier | check M quotients | check sumcheck + 1 evaluation |
| trace representation | unnatural (polynomial interpolation) | natural (boolean hypercube) |

## SuperSpartan

SuperSpartan (Setty, Thaler, Wahby, 2023) is the IOP (Interactive Oracle Proof) that makes multilinear starks over CCS practical.

```
SuperSpartan for AIR:
  1. prover commits trace as multilinear polynomial → PCS commitment
  2. verifier sends random challenges
  3. prover and verifier run sumcheck
     reduces constraint satisfaction over ALL rows
     to evaluating the trace polynomial at ONE random point
  4. PCS opens the commitment at that point
  5. verifier checks: sumcheck transcript + PCS opening
```

key properties:
- linear-time prover for constraints (field operations only, no FFT/NTT)
- logarithmic-time verifier for AIR (O(log n) without preprocessing)
- high-degree constraints cost only field operations (no cryptographic cost increase)
- CCS-native: AIR, R1CS, and Plonkish all map directly

## Whirlaway

Whirlaway (LambdaClass, 2025) is the concrete instantiation: SuperSpartan IOP + [[WHIR]] as the multilinear polynomial commitment scheme.

```
Whirlaway = SuperSpartan (IOP for CCS) + WHIR (multilinear PCS)

prover:
  1. execute program → trace matrix (2ⁿ × 2ᵐ)
  2. commit trace as multilinear polynomial via WHIR_commit
  3. run sumcheck with verifier (constraint verification)
  4. open WHIR commitment at sumcheck output point via WHIR_open

verifier:
  1. check sumcheck transcript (field arithmetic only)
  2. check WHIR opening (one evaluation proof: WHIR_verify)
  3. all constraints verified
```

the combination achieves:
- transparent setup (WHIR requires no trusted setup)
- post-quantum security (WHIR is hash-only)
- sub-millisecond verification ([[WHIR]]: 290 μs at 100-bit, 1.0 ms at 128-bit)
- linear-time prover for the constraint phase
- proof size: ~60–157 KiB

this is what [[cyber]] uses. see [[WHIR]] for the PCS, [[nox]] for the VM.

## the sumcheck protocol

the engine inside multilinear starks. proves that the sum of a polynomial over a boolean hypercube equals a claimed value:

```
claim: Σ_{x ∈ {0,1}ᵏ} f(x) = T

round i (for i = 1, ..., k):
  prover sends univariate gᵢ(Xᵢ) = Σ_{rest} f(r₁, ..., r_{i-1}, Xᵢ, x_{i+1}, ..., x_k)
  verifier checks: gᵢ(0) + gᵢ(1) = previous round's claim
  verifier sends random challenge rᵢ
  new claim: gᵢ(rᵢ)

after k rounds:
  final claim = f(r₁, ..., r_k)
  verifier checks this single evaluation via WHIR opening

total verifier work: O(k) field operations + one WHIR_verify
```

the sumcheck converts a sum over 2^k terms into k rounds of interaction, each involving a low-degree univariate polynomial. the prover's work per round is linear in the remaining summation size — total prover work is O(2^k), which is linear in the trace size.

## heritage

```
2018  starks (Ben-Sasson et al.)       univariate, FRI, first transparent ZK at scale
2019  Spartan (Setty)                   R1CS via sumcheck, no FFT in prover
2023  SuperSpartan (Setty et al.)       CCS generalization, handles AIR natively
2024  STIR (Arnon et al.)               improved FRI: rate increases per round
2024  Circle starks (StarkWare)         starks over Mersenne31 field
2025  WHIR (Arnon et al.)               sub-millisecond verification, multilinear PCS
2025  Whirlaway (LambdaClass)           SuperSpartan + WHIR = multilinear stark
```

## use in cyber

[[cyber]] uses multilinear starks via the Whirlaway architecture:

```
┌─────────────────────────────────────────────────┐
│  nox program execution                           │
│  → execution trace (2ⁿ steps × registers)        │
├─────────────────────────────────────────────────┤
│  multilinear encoding                            │
│  → f(x₁, ..., x_{n+m}) over Goldilocks field    │
├─────────────────────────────────────────────────┤
│  SuperSpartan IOP                                │
│  → AIR constraints via CCS                       │
│  → sumcheck reduces to ONE evaluation point      │
├─────────────────────────────────────────────────┤
│  WHIR (multilinear PCS)                          │
│  → commit: WHIR_commit(f)                        │
│  → open: WHIR_open(f, r) at sumcheck point       │
│  → verify: WHIR_verify(C, r, v, π)              │
├─────────────────────────────────────────────────┤
│  Hemera (hash) + Goldilocks (field)              │
│  → one hash, one field, post-quantum             │
└─────────────────────────────────────────────────┘
```

one hash ([[Hemera]]). one field ([[Goldilocks field]]). one VM ([[nox]]). one PCS ([[WHIR]]). one IOP ([[SuperSpartan]]). one proof.

see [[cyber/stark]] for the concrete implementation in cyber (AIR from nox, constraint budget, recursive composition, BBG integration), [[WHIR]] for the polynomial commitment scheme, [[cyber/proofs]] for the full proof taxonomy, [[nox]] for the VM, [[Goldilocks field]] for the arithmetic, [[FRI]] for WHIR's heritage, [[cryptography]] for the broader field
