---
tags: cyber
alias: zheng, 証, proof system
crystal-type: entity
crystal-domain: cyber
subgraph: true
repo: ../zheng
exclude: ".claude/**, target/**"
---
the proof system for [[cyber]]. implements the Whirlaway architecture: [[SuperSpartan]] IOP + [[WHIR]] PCS + [[sumcheck]] protocol. zero trusted setup. post-quantum. sub-millisecond verification.

zheng (証 — proof/evidence in Japanese) provides the cryptographic machinery that turns [[nox]] execution traces into compact, verifiable proofs. one commitment, one opening, one proof.

## components

```
SuperSpartan    IOP for CCS — handles AIR constraints of any degree via sumcheck
WHIR            multilinear PCS — fastest verification of any polynomial commitment scheme
sumcheck        core interactive proof — reduces N-term sums to log(N) rounds
```

## dependency graph

```
aurum (field)
  ↓
hemera (hash)
  ↓
zheng (proofs) ← this repo
  ↓
bbg (state)
```

see [[cyber/stark]] for the concrete instantiation, [[WHIR]] for the PCS, [[SuperSpartan]] for the IOP, [[sumcheck]] for the core protocol
