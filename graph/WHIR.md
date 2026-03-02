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

this dual nature makes WHIR a direct building block for multilinear STARKs — no separate polynomial commitment layer needed.

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

WHIR represents the end of the FRI evolution path for [[cyber]]:

```
current:   FRI   — ~100-200 KB proofs, ~3.9 ms verification
upgrade 1: STIR  — ~60-120 KB proofs, ~3.8 ms verification
upgrade 2: WHIR  — ~60-120 KB proofs, ~1.0 ms verification

all upgrades are Layer 4 swaps — architecture unchanged
```

WHIR's sub-millisecond verification makes recursive proof composition cheaper: each recursive step (verify a proof inside a proof) runs the verifier as a [[nox]] program. faster verification = fewer constraints per recursive step = cheaper aggregation.

see [[FRI]] for the baseline protocol, [[STIR]] for the intermediate evolution, [[polynomial commitment]] for the commitment scheme, [[STARK]] for the proof system, [[Goldilocks field]] for the arithmetic foundation
