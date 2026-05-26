---
alias: zero knowledge proofs, zero-knowledge proofs, ZKP, crypto zero-knowledge
tags: computer science, cryptography
crystal-type: entity
crystal-domain: computer science
---
# crypto/zero-knowledge

prove a statement is true without revealing anything beyond the truth of the statement. a ZKP system has three properties: completeness (honest prover convinces verifier), soundness (cheating prover fails), zero-knowledge (verifier learns nothing beyond the statement's truth).

## systems

| system | setup | proof size | verify time | quantum safe | assumption |
|---|---|---|---|---|---|
| Groth16 | trusted (per-circuit) | 128 bytes | ~1 ms | no | bilinear pairings |
| PLONK / Halo2 | universal trusted | 400-800 bytes | ~3 ms | no | bilinear pairings |
| Bulletproofs | none | ~700 bytes | ~30 ms | no | discrete log |
| [[stark]] | none (transparent) | 100-200 KB | 1-4 ms | yes | hash collision resistance |
| [[stark]] + [[WHIR]] | none (transparent) | 60-157 KB | 0.3-1.0 ms | yes | hash collision resistance |

[[SNARKs]] (Succinct Non-interactive Arguments of Knowledge) achieve small proofs but typically require a trusted setup ceremony and rely on elliptic curve assumptions vulnerable to quantum computers. [[starks]] (Scalable Transparent Arguments of Knowledge) require no trusted setup and rely only on hash functions — post-quantum secure, larger proofs but faster verification (with [[WHIR]]).

## recursive composition

a proof system that can verify its own proofs enables recursive composition: prove that a verification of a proof was done correctly. the result is a constant-size proof regardless of the depth of recursion.

```
Level 0: Prove computation C → proof p0
Level 1: Prove verify(p0) → proof p1 (same size)
Level N: Prove verify(p_{N-1}) → proof pN (same size)

N proofs → one aggregated proof → O(1) verification
```

the foundation of rollups ([[Ethereum]] L2s), [[incrementally verifiable computation]] (IVC), and proof aggregation. systems: [[Nova]] ([[folding]] schemes), Halo2 (accumulation), [[starks]] (self-referential verification).

## folding and IVC

[[folding]] makes recursive composition practical. instead of fully verifying a proof at each step (expensive), absorb it into an [[accumulator]] via a cheap algebraic operation, then verify once at the end.

```
traditional recursion:  verify(p_{i-1}) at each step → full SNARK per step
folding:                fold(accumulator, pi) → one field operation per step
                        verify(accumulator) once at the end → one decider proof
```

| scheme | constraint system | year | key property |
|---|---|---|---|
| [[Nova]] | R1CS | 2022 | first practical folding, IVC without per-step SNARKs |
| SuperNova | R1CS (non-uniform) | 2022 | multiple circuit types in one IVC chain |
| [[HyperNova]] | CCS | 2023 | generalizes folding to CCS — handles R1CS, Plonkish, AIR |
| [[Protostar]] | CCS + lookups | 2023 | high-degree gates, lookup arguments in folding |
| ProtoGalaxy | CCS (multi-instance) | 2023 | logarithmic verifier cost for multi-instance folding |

[[incrementally verifiable computation]] (IVC) breaks a long computation into steps, each step produces a proof covering all previous steps. the verifier checks only the final proof. [[proof-carrying data]] (PCD) generalizes IVC from sequential chains to arbitrary DAGs — multiple independent computations merge proofs via folding. see [[folding]], [[incrementally verifiable computation]], [[proof-carrying data]]

## lookup arguments

prove that a set of values appears in a pre-defined table without revealing which entries. the key optimization in modern proof systems — replaces expensive range checks and cross-table consistency proofs.

| protocol | technique | prover | verifier | used in |
|---|---|---|---|---|
| Plookup | sorted polynomial identity | O(n log n) | O(log n) | Plonkish circuits |
| [[LogUp]] | sumcheck over rational identity | O(n log n) | O(log n) | Polygon, Scroll, [[cyber]] |
| Lasso | sparse lookups via sumcheck | O(n) | O(log n) | Jolt VM (a]Priori) |
| cq (cached quotients) | precomputed quotient | O(n) | O(1) | general lookup |

[[LogUp]] proves that the same edge hash appears in all required [[EdgeSets]] in the [[BBG]] — 15x cheaper than independent polynomial proofs. see [[LogUp]]

see [[cryptography]], [[stark]], [[zheng]], [[cyber/proofs]]