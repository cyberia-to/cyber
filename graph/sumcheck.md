---
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
alias: sumcheck protocol
---
# sumcheck

an interactive proof protocol that reduces verifying a sum over exponentially many terms to checking a single evaluation. Lund, Fortnow, Karloff, Nisan (1992). the engine inside [[stark|multilinear starks]] and [[SuperSpartan]].

## the protocol

given a k-variate polynomial f, prove that the sum over the boolean hypercube equals a claimed value:

```
claim: Σ_{x ∈ {0,1}ᵏ} f(x₁, ..., x_k) = T

round i (for i = 1, ..., k):
  prover sends univariate polynomial gᵢ(Xᵢ)
  verifier checks: gᵢ(0) + gᵢ(1) = previous claim
  verifier sends random challenge rᵢ
  new claim: gᵢ(rᵢ)

after k rounds:
  final claim = f(r₁, ..., r_k)
  verifier checks this single evaluation (via PCS opening)
```

2^k terms reduced to k rounds. total verifier work: O(k) field operations + one polynomial evaluation check.

## cost

```
prover:   O(2^k) field operations — linear in the summation domain
verifier: O(k) field operations + one PCS opening check
rounds:   k (one per variable)
```

for a [[nox]] execution trace with 2ⁿ steps and 2ᵐ registers, k = n + m. the sumcheck prover is linear in the trace size. the verifier's work is logarithmic.

## role in cyber

in the [[stark|multilinear stark]] pipeline, sumcheck replaces the zerofier division step of classical univariate starks. [[SuperSpartan]] uses sumcheck to verify AIR constraints: the sum of constraint polynomials over all trace rows must be zero. sumcheck reduces this to evaluating the trace polynomial at one random point, which [[WHIR]] opens.

sumcheck also appears in [[LogUp]] lookup arguments for cross-index consistency in the [[BBG]].

see [[stark]] for the full proof pipeline, [[SuperSpartan]] for the IOP, [[WHIR]] for the polynomial commitment scheme
