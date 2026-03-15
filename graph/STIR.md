---
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
alias: Shift To Improve Rate
---
# STIR

Shift To Improve Rate. a next-generation interactive oracle proof of proximity (IOPP) for Reed-Solomon codes that achieves fewer queries and smaller proofs than [[FRI]] by recursively improving the code rate at each round.

Arnon, Chiesa, Fenzi, Yogev (CRYPTO 2024, Best Paper Award). ePrint 2024/390.

## the key idea

[[FRI]] reduces polynomial degree at each round but keeps the code rate constant. STIR simultaneously reduces degree and improves rate — making each successive proximity test easier. the technique: after folding (degree reduction by factor k), apply a shift that maps the evaluation domain to a smaller one, increasing the fraction of the domain occupied by the codeword.

```
FRI:   degree ↓  rate =     →  many queries per round
STIR:  degree ↓  rate ↑     →  fewer queries per round

"decreasing rate makes proximity testing easier"
```

## protocol structure

each round combines two operations:

1. folding — reduce polynomial degree by factor k (same as [[FRI]])
2. quotienting — enforce consistency constraints between rounds via shift operations that improve the rate

the verifier sends random challenges, the prover folds and shifts, and proximity testing becomes progressively easier as the rate improves. the protocol terminates when the degree is small enough for direct checking.

## performance vs FRI

at d = 2²⁴, ρ = 1/2, 128-bit security:

```
                        FRI          STIR         improvement
─────────────────────────────────────────────────────────────
argument size           306 KiB      160 KiB      1.9× smaller
verifier hashes         5,600        2,600        2.2× fewer
verifier runtime        3.9 ms       3.8 ms       comparable
prover runtime          comparable   comparable   —
```

the improvement comes entirely from query reduction — fewer queries per round means smaller proofs. prover and verifier runtimes remain similar because the computational work per query is unchanged.

## query complexity

```
FRI:   O(λ · log d · max{1/(−log(1−δ)), 1/(−log√ρ)})
STIR:  O(λ/(−log(1−δ)) + log d + λ · log(log d/(−log√ρ)))

where:
  λ  = security parameter
  d  = polynomial degree
  ρ  = code rate
  δ  = proximity parameter
```

STIR decouples the security parameter λ from the degree d in the dominant term — queries scale as O(log d) instead of O(λ · log d). this is the theoretical advance that enables smaller proofs.

## use in cyber

[[cyber]] uses [[WHIR]] — the next generation after STIR. STIR is the intermediate step in the FRI evolution. the concepts introduced by STIR (rate improvement via shifts) are subsumed by WHIR's richer weight polynomial approach.

```
evolution:
  FRI  (2018)  — baseline
  STIR (2024)  — rate improvement via shifts
  WHIR (2025)  — rate + weight polynomials ← cyber uses this
```

see [[WHIR]] for what [[cyber]] uses, [[FRI]] for the foundational protocol, [[polynomial commitment]] for the commitment scheme, [[stark]] for the proof system
