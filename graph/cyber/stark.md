---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: cyber stark, cyber STARKs, Whirlaway implementation
stake: 38544821775428340
---
# cyber/stark

the concrete instantiation of [[STARK|multilinear STARKs]] inside [[cyber]]. five primitives, one architecture, zero trusted setup.

```
COMPONENT         │ ROLE                          │ INSTANCE
──────────────────┼───────────────────────────────┼─────────────────────
hash              │ Fiat-Shamir, Merkle trees     │ Hemera (Poseidon2)
field             │ arithmetic substrate           │ Goldilocks (2⁶⁴ − 2³² + 1)
VM                │ execution trace generation     │ nox (16 patterns + hint + 5 jets)
IOP               │ constraint verification        │ SuperSpartan (CCS/AIR via sumcheck)
PCS               │ polynomial commitment          │ WHIR (multilinear)
```

architecture: Whirlaway = [[SuperSpartan]] IOP + [[WHIR]] PCS. LambdaClass (2025).

## the pipeline

```
1. EXECUTE
   nox program runs on subject with bounded focus
   → execution trace: 2ⁿ rows × 16 register columns
   each row = one reduction step
   each column = one register value over Goldilocks

2. ARITHMETIZE
   nox's 16 patterns → AIR transition constraints
   pattern 5 (add):  reg[out]_{t+1} = reg[a]_t + reg[b]_t                (degree 1)
   pattern 7 (mul):  reg[out]_{t+1} = reg[a]_t × reg[b]_t                (degree 2)
   pattern 4 (branch): selector_t × (next_t − yes_t) + (1 − selector_t) × (next_t − no_t) = 0  (degree 2)
   pattern 15 (hash): Poseidon2 round constraints across consecutive rows  (degree 7)

   boundary constraints pin inputs and outputs:
     reg[0] at row 0 = program_input
     reg[0] at last row = program_output

3. ENCODE
   entire trace (2ⁿ × 2⁴) → ONE multilinear polynomial f(x₁, ..., x_{n+4})
   row index: n boolean variables
   column index: 4 boolean variables
   every variable has degree ≤ 1

4. COMMIT
   WHIR_commit(f) = C
   Hemera Merkle tree over evaluations on the boolean hypercube
   commitment size: one Hemera digest (64 bytes)

5. PROVE CONSTRAINTS
   SuperSpartan sumcheck:
     claim: Σ_{x ∈ {0,1}^n} constraint_polynomial(f, x) = 0
     n + 4 rounds of interaction (Fiat-Shamir in practice)
     reduces to: evaluate f at ONE random point r ∈ F^{n+4}

6. OPEN
   WHIR_open(f, r) = (v, π)
   prover demonstrates f(r) = v with proximity proof
   proof: ~60–157 KiB depending on security level

7. VERIFY
   verifier checks:
     a) sumcheck transcript is consistent (field arithmetic only)
     b) WHIR_verify(C, r, v, π) accepts (hash operations only)
   total: O(log² N) operations. sub-millisecond.
```

## why multilinear

classical univariate STARKs interpolate each trace column as a separate polynomial, check constraints via zerofier division, prove via [[FRI]]. M columns → M commitments → M openings.

multilinear STARKs encode the entire trace as one polynomial. constraints checked via [[sumcheck]]. one commitment. one opening. the advantage:

```
                           │ univariate         │ multilinear (cyber)
───────────────────────────┼────────────────────┼─────────────────────
commitments                │ 16 (one/column)    │ 1
openings                   │ 16                 │ 1
prover (constraints)       │ O(N log N) per col │ O(N) total
prover tool                │ FFT/NTT            │ field ops only
trace representation       │ interpolation      │ boolean hypercube (natural)
constraint system          │ AIR only           │ CCS (AIR + R1CS + Plonkish)
```

the [[sumcheck]] protocol is the mechanism. it converts a sum over 2^k terms (all trace rows) into k rounds, each a low-degree univariate. the verifier's work per round: check one polynomial evaluation. total verifier work: O(k) field ops + one [[WHIR]] opening.

## AIR from nox

each of [[nox]]'s sixteen reduction patterns becomes an AIR transition constraint — a polynomial equation that must hold between consecutive trace rows.

```
PATTERN → CONSTRAINT                                        │ DEGREE │ CONSTRAINTS
────────────────────────────────────────────────────────────┼────────┼────────────
0  axis     navigation through subject tree                │ 1      │ ~depth
1  quote    output = literal constant                       │ 1      │ 1
2  compose  chain: eval x, eval y, eval result on x's output│ 1      │ 2
3  cons     pair two sub-results                            │ 1      │ 2
4  branch   selector × (next − yes) + (1−sel) × (next − no)│ 2      │ 2
5  add      out = a + b mod p                               │ 1      │ 1
6  sub      out = a − b mod p                               │ 1      │ 1
7  mul      out = a × b mod p                               │ 2      │ 1
8  inv      out × input = 1 mod p (Fermat verification)    │ 2      │ 1
9  eq       (a − b) × inv = (a ≠ b), out = 1 − (a ≠ b)    │ 2      │ 1
10 lt       range decomposition into bits                   │ 1      │ ~64
11 xor      bit decomposition + XOR per bit                 │ 2      │ ~64
12 and      bit decomposition + AND per bit                 │ 2      │ ~64
13 not      bitwise complement                              │ 1      │ ~64
14 shl      shift via multiplication by 2^n                 │ 2      │ ~64
15 hash     Poseidon2 round function across rows            │ 7      │ ~300
16 hint     constraint check (Layer 1 verification)         │ varies │ varies
```

[[SuperSpartan]] handles AIR constraints of any degree via [[CCS]]. high-degree constraints (pattern 15: degree 7) cost only field operations in the prover — no cryptographic cost increase over degree-1 constraints. this is the CCS advantage: the Poseidon2 rounds inside the hash pattern are free in the IOP layer.

## constraint budget

```
PROOF TYPE                       │ TOTAL CONSTRAINTS │ DOMINANT COST
─────────────────────────────────┼───────────────────┼──────────────────────
identity (preimage)              │ ~300              │ one Hemera hash
anonymous cyberlink              │ ~13,000           │ WHIR membership + SWBF
delivery (per hop)               │ ~60,000           │ decryption + forwarding
private transfer (BBG)           │ ~50,000           │ AOCL/SWBF verification
STARK recursive verification     │ ~70,000 (jets)    │ Merkle + WHIR verify
STARK recursive (no jets)        │ ~600,000          │ Merkle verification
```

constraint count determines prover time. at ~10⁶ constraints/second on commodity hardware: a 13K-constraint anonymous [[cyberlink]] proves in ~13 ms. a 70K-constraint recursive step proves in ~70 ms. the [[Goldilocks field processor]] targets 10× acceleration.

## Hemera as the STARK hash

every hash operation inside a STARK — Fiat-Shamir challenges, Merkle trees in WHIR, commitment randomness — uses [[Hemera]]. the choice of hash is the single largest factor in STARK performance.

```
HASH                │ CONSTRAINTS PER CALL │ STARK OVERHEAD
────────────────────┼──────────────────────┼────────────────
SHA-256             │ ~25,000              │ baseline
Keccak-256          │ ~150,000             │ 6× worse
Poseidon (original) │ ~4,000               │ 6× cheaper
Hemera (Poseidon2)  │ ~1,200               │ 20× cheaper
```

Hemera's ~1,200 constraints per hash means Merkle verification at depth 32 costs ~38,400 constraints instead of ~800,000 with SHA-256. this 20× reduction is what makes recursive STARK composition practical at 70,000 total constraints.

the hash is also the field: Hemera operates natively on [[Goldilocks field]] elements. no bit-packing, no field conversion, no endianness gymnastics. eight elements in, eight elements out. the output is directly usable in polynomial commitments, constraint evaluations, and [[nox]] arithmetic.

## recursive composition

the STARK verifier is a [[nox]] program. it can be proven by the same STARK system.

```
Level 0: prove computation C → proof π₀
Level 1: prove verify(π₀)  → proof π₁   (~70K constraints with jets)
Level 2: prove verify(π₁)  → proof π₂   (same cost)
  ...
Level k: proof π_k — same size regardless of k
```

each recursion level costs ~70,000 constraints (with Layer 3 jets). the five jets — hash, poly_eval, merkle_verify, fri_fold, ntt — reduce the unoptimized 600,000-constraint verifier by 8.5×.

aggregation: N independent proofs → verify all N in one nox program → prove that verification → one proof. O(1) on-chain verification for O(N) transactions. this is how [[cyber]] scales: the chain verifies one proof per block, covering all cyberlinks, transfers, and state transitions within it.

## IVC and folding

[[incrementally verifiable computation]] chains proofs sequentially: each step absorbs the previous proof via [[folding]] into an [[accumulator]]. the accumulated proof at step i guarantees all steps 1..i are valid.

[[proof-carrying data]] generalizes IVC from linear chains to DAGs. in [[cyber]], the [[cybergraph]] is a DAG — PCD matches this topology. different [[validators]] prove different subgraphs, then merge proofs at shard boundaries.

[[HyperNova]] folding over [[CCS]] is the natural fit: CCS already powers [[SuperSpartan]], so the folding scheme and the STARK system share the same constraint language. fold a [[cyberlink]] insertion proof? same CCS instance type. fold a rank update? same CCS. one framework.

## integration with BBG

the [[BBG]] uses [[WHIR]]-based polynomial commitments for all indexes. the same WHIR instance that serves as the STARK PCS also handles:

```
OPERATION              │ MECHANISM                        │ CONSTRAINTS
───────────────────────┼──────────────────────────────────┼────────────
EdgeSet membership     │ WHIR evaluation proof            │ ~1,000
namespace completeness │ sorted range bounds + WHIR opens │ ~10,000
cross-index consistency│ LogUp via sumcheck               │ ~5,000
focus commitment       │ polynomial over (neuron, π)      │ ~1,000
balance commitment     │ polynomial over (neuron, balance)│ ~1,000
```

[[LogUp]] lookup arguments use the [[sumcheck]] protocol — the same sumcheck that powers [[SuperSpartan]]. cross-index consistency (every edge appearing in neuron index, source index, and target index) reduces to a sumcheck over logarithmic multiplicities. one protocol, two uses.

## security

```
ASSUMPTION              │ WHAT BREAKS IF VIOLATED
────────────────────────┼─────────────────────────────────────
Hemera collision resistance │ forge commitments, fake Merkle proofs
Goldilocks field hardness   │ extract witnesses from proofs
sumcheck soundness          │ false constraint satisfaction
WHIR proximity              │ evaluate committed polynomial incorrectly
Fiat-Shamir (random oracle) │ predictable challenges → forge proofs
```

all assumptions reduce to: collision resistance of [[Hemera]]. no discrete log, no pairings, no trusted setup. post-quantum: a quantum adversary with Grover's algorithm squares the brute-force effort on Hemera from 2^128 to 2^64 — but Hemera's 256-bit output provides 128-bit post-quantum security margin.

see [[STARK]] for the general theory, [[cyber/proofs]] for the full proof taxonomy, [[nox]] for the VM specification, [[WHIR]] for the PCS, [[SuperSpartan]] for the IOP, [[sumcheck]] for the core protocol, [[Hemera]] for the hash, [[Goldilocks field]] for the arithmetic, [[BBG]] for the graph structure
