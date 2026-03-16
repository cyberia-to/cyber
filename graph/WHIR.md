---
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
alias: Weights Help Improving Rate
---
# WHIR

Weights Help Improving Rate. an interactive oracle proof of proximity for constrained Reed-Solomon codes that achieves the fastest verification of any polynomial commitment scheme — including schemes with trusted setup. simultaneously serves as a multilinear polynomial commitment scheme.

Arnon, Chiesa, Fenzi, Yogev (EUROCRYPT 2025). ePrint 2024/1586.

## the key idea

[[STIR]] improved [[FRI]] by increasing code rate at each round. WHIR goes further: it introduces weight polynomials from the sumcheck protocol to achieve richer queries per round. each round extracts more information from the prover, allowing the verifier to reach conviction faster with fewer rounds and fewer queries.

```
FRI:   degree ↓  rate =  queries = many     →  3.9 ms verification
STIR:  degree ↓  rate ↑  queries = fewer    →  3.8 ms verification
WHIR:  degree ↓  rate ↑  queries = richest  →  1.0 ms verification
```

the technique: combine sumcheck rounds (from BaseFold) with rate improvements (from STIR) via a new soundness property called "mutual correlated agreement." this lets each round simultaneously test proximity and evaluate multilinear extensions.

## dual nature

WHIR is both an IOPP (proximity test) and a multilinear polynomial commitment scheme (PCS):

- as IOPP: proves a committed function is close to a low-degree polynomial (like [[FRI]] and [[STIR]])
- as PCS: commits to a multilinear polynomial and proves evaluations at specific points

this dual nature makes WHIR a direct building block for multilinear starks — no separate polynomial commitment layer needed.

## performance

at d = 2²⁴, 128-bit security:

```
                        FRI          STIR         WHIR
──────────────────────────────────────────────────────────────
argument size           306 KiB      160 KiB      157 KiB
verifier hashes         5,600        2,600        2,700
verification time       3.9 ms       3.8 ms       1.0 ms
```

at d = 2²², 100-bit security:

```
                        BaseFold     WHIR         improvement
──────────────────────────────────────────────────────────────
argument size           7.95 MiB     101 KiB      74× smaller
verification time       24 ms        610 μs       39× faster
```

comparison with trusted-setup schemes (100-bit security):

```
                        KZG          PST          WHIR
──────────────────────────────────────────────────────────────
trusted setup           required     required     none
post-quantum            no           no           yes
verification time       2.4 ms       3.6 ms       290 μs
argument size           48 bytes     ~200 bytes   101 KiB
```

WHIR verification is faster than KZG despite requiring no trusted setup and providing post-quantum security. the tradeoff: larger argument size (101 KiB vs 48 bytes).

## properties

| property | value |
|---|---|
| trusted setup | none (transparent) |
| post-quantum security | yes |
| verification time | 290 μs – 1.0 ms (fastest of any PCS) |
| argument size | 101-157 KiB (state-of-the-art for hash-based) |
| field compatibility | any field with large multiplicative subgroup |
| prover | comparable to FRI/STIR |

## the evolution

```
FRI (2018)   →  STIR (2024)    →  WHIR (2024/2025)
baseline         fewer queries      richest queries
                 rate improvement    rate + weight polynomials
                 1.9× smaller       1.9× smaller + 3.8× faster verification
                 CRYPTO 2024        EUROCRYPT 2025
```

all three are by the same team: Arnon, Chiesa, Fenzi, Yogev. each generation keeps the same interface (Reed-Solomon proximity testing) while improving concrete efficiency. [[cyber]] can upgrade FRI → STIR → WHIR without changing any layer above the proof system.

## use in cyber

[[cyber]] uses WHIR as the polynomial commitment scheme inside a [[stark|multilinear stark]] (Whirlaway architecture: [[SuperSpartan]] IOP + WHIR PCS).

WHIR's dual nature is the reason this works. as a multilinear PCS, WHIR commits to the entire [[nox]] execution trace encoded as a single multilinear polynomial. the [[SuperSpartan]] IOP verifies AIR constraints via [[sumcheck]], reducing all constraint checks to one evaluation at one random point. WHIR opens the commitment at that point. one commitment, one opening, one proof.

```
WHIR in cyber:
  role:                    multilinear PCS for stark proofs
  API:                     WHIR_commit / WHIR_open / WHIR_verify
  trace commitment:        entire nox execution trace → one multilinear polynomial
  constraint verification: SuperSpartan sumcheck → reduces to one WHIR evaluation
  EdgeSet membership:      WHIR evaluation proofs (polynomial commitments in BBG)
  proof size:              ~60-157 KB
  verification:            ~1.0 ms (sub-millisecond at 100-bit: 290 μs)
```

sub-millisecond verification makes recursive proof composition practical: each recursive step (verify a proof inside a proof) runs the WHIR verifier as a [[nox]] program. at ~70,000 constraints per recursion level (with jets), deep recursion trees become feasible — O(1) on-chain verification for O(N) transactions.

see [[FRI]] for the baseline protocol, [[STIR]] for the intermediate evolution, [[polynomial commitment]] for the commitment scheme, [[stark]] for the proof system and Whirlaway architecture, [[Goldilocks field]] for the arithmetic foundation

## design notes: open problems

### WHIR-compressed Merkle proofs

[[hemera]] trees currently use binary Merkle proofs: d sibling hashes at 64 bytes each. for a 2^20-leaf tree, one proof is 1,280 bytes; k proofs (even batched) still carry O(k × d) hashes in the worst case.

the alternative: commit to each tree level as a multilinear polynomial and use WHIR to open at queried positions. a node at level ℓ has 2^ℓ children. encode them as a multilinear polynomial f_ℓ over ⌈log₂(2^ℓ)⌉ = ℓ variables. a WHIR opening at one point costs ~101 KiB but is independent of the branching factor or the number of opened positions (batch openings amortize). for k leaves the proof becomes: one WHIR opening per level traversed, with batch openings combining multiple queries at the same level.

open questions:

- evaluation table layout: how do 2^ℓ node hashes at level ℓ map to a multilinear polynomial? natural encoding: f_ℓ(b₁, ..., b_ℓ) = hash of the node reached by path bits b₁...b_ℓ. the evaluation table is the full set of node hashes at that level.
- root finalization: does the WHIR commitment root use [[hemera]]'s ROOT flag? the commitment itself (a Merkle root over evaluations) needs domain separation from content tree roots.
- crossover point: WHIR argument size is ~101 KiB. a standard Merkle proof for depth 20 is 1,280 bytes. WHIR-compressed proofs only win when proving many leaves at once (batch opening amortizes the 101 KiB across k queries). the crossover is roughly k > 80 leaves for depth 20. below that threshold, standard batch proofs (see [[hemera]] tree spec) are smaller.
- hybrid approach: use standard Merkle proofs for small batches (k < threshold) and WHIR-compressed proofs for large batches. the prover selects the mode; the verifier accepts both. this needs a mode flag in the proof structure.

### incremental WHIR commitment updates

[[EdgeSet]] polynomial commitments use WHIR_commit(P_N) where P_N interpolates all edges in a namespace. when a new edge arrives, the entire polynomial changes and re-commitment from scratch costs O(N log N) for an N-edge EdgeSet.

incremental update path: if the polynomial is represented as f(x) = f_old(x) + Δ(x), where Δ is a sparse update (one new evaluation point), the commitment can be updated by committing to Δ separately and combining. this requires:

- additive homomorphism: WHIR_commit(f + g) = WHIR_commit(f) ⊕ WHIR_commit(g). hash-based commitments (Merkle over evaluations) are not additively homomorphic in general. however, if the evaluation domain is fixed and the commitment is over the evaluation table, updating one entry means rehashing one Merkle path — O(log N) hashes, not O(N log N).
- amortized cost: for a fixed evaluation domain of size D ≥ N_max, inserting edge k+1 means: evaluate P at the new point, update one leaf in the evaluation-table Merkle tree, rehash the path. cost: 1 polynomial evaluation + log₂(D) × 2 permutations. for D = 2^16: 32 permutations per edge insertion vs rebuilding the full NTT + Merkle tree.
- domain sizing: the evaluation domain must be large enough to accommodate future edges without re-commitment. over-provisioning wastes proof size (WHIR argument scales with domain size). under-provisioning forces full re-commitment when the domain is exhausted. a doubling strategy (start at 2^10, double when 75% full, re-commit on doubling) amortizes re-commitment cost to O(1) per insertion.

this matters for block production: a validator adding 1,000 edges per block should pay ~1,000 × 32 = 32,000 permutations for EdgeSet updates, not ~1,000 × full re-commit.

### WHIR verification circuit decomposition

the [[nox]] spec defines two jets relevant to WHIR verification:

- `merkle_verify(root, leaf, path, index)` — d × 300 constraints per depth-d path
- `fri_fold(poly_layer, challenge)` — N/2 constraints per folding round

a full WHIR_verify decomposes into:

```
WHIR_verify(commitment, point, value, proof):
    1. Fiat-Shamir: derive challenges from transcript
       → Poseidon2 hashes (p2r jet, ~22 cycles per challenge)
    2. For each folding round (log(N) rounds):
       a. merkle_verify: check evaluation query paths
          → merkle_verify jet, d × 300 constraints per path
       b. fri_fold: fold polynomial with round challenge
          → fri_fold jet, N_i/2 constraints (N_i halves each round)
       c. sumcheck round: weight polynomial combination
          → field arithmetic, ~10 constraints per round
    3. Final check: verify folded polynomial is constant
       → 1 field comparison
```

estimated total for N = 2^16 evaluations, 100-bit security:

| component | calls | constraints |
|---|---|---|
| Fiat-Shamir (Poseidon2) | ~16 hashes | ~19,200 |
| merkle_verify | ~50 paths × depth 16 | ~240,000 |
| fri_fold | 16 rounds, geometric sum | ~32,768 |
| sumcheck rounds | 16 rounds | ~160 |
| total | | ~292,000 |

without jets (pure Layer 1): ~2,900,000 constraints (10× more). the jets reduce WHIR verification to a cost comparable to ~120 `hash_node` calls — making recursive proof composition practical at ~300K constraints per recursion level.

a composed `WHIR_VERIFY` jet (combining merkle_verify + fri_fold + Fiat-Shamir into one operation) could reduce this further to ~100K constraints, but adds ISA complexity. the current two-jet decomposition is sufficient for recursive composition depths up to ~10 levels within a single proof.
