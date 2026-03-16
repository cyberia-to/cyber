---
tags: cyber, cip
alias: zheng, 証, proof system, cyber stark, cyber starks, Whirlaway implementation
crystal-type: entity
crystal-domain: cyber
subgraph: true
repo: ../zheng
exclude: ".claude/**, target/**, CLAUDE.md"
stake: 38544821775428340
---
the proof system for [[cyber]]. implements the Whirlaway architecture: [[SuperSpartan]] IOP + [[WHIR]] PCS + [[sumcheck]] protocol. zero trusted setup. post-quantum. sub-millisecond verification.

zheng (証 — proof/evidence in Japanese) provides the cryptographic machinery that turns [[nox]] execution traces into compact, verifiable proofs. one commitment, one opening, one proof.

```
COMPONENT         │ ROLE                          │ INSTANCE
──────────────────┼───────────────────────────────┼─────────────────────
hash              │ Fiat-Shamir, Merkle trees     │ Hemera (Poseidon2)
field             │ arithmetic substrate           │ Goldilocks (2⁶⁴ − 2³² + 1)
VM                │ execution trace generation     │ nox (16 patterns + hint + 5 jets)
IOP               │ constraint verification        │ SuperSpartan (CCS/AIR via sumcheck)
PCS               │ polynomial commitment          │ WHIR (multilinear)
```

## dependency graph

```
nebu (field)
  ↓
hemera (hash)
  ↓
zheng (proofs) ← this repo
  ↓
bbg (state)
```

see [[stark]] for the general theory, [[cyber/proofs]] for the full proof taxonomy
