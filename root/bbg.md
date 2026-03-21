---
tags: cyber
alias: bbg, Big Badass Graph, authenticated state
crystal-type: entity
crystal-domain: cyber
subgraph: true
repo: ../bbg
exclude: ".claude/**, target/**, CLAUDE.md"
focus: 0.0010230569493550432
gravity: 25
---
the authenticated state layer for [[cyber]]. stores the [[cybergraph]] — edges ([[cyberlinks]]), [[neuron]] state, [[particle]] energy, [[focus]], balances — with polynomial commitment indexes that provide cryptographic completeness proofs.

when you sync a namespace, you get mathematical proof that nothing was withheld. the graph cannot exist without its indexes being consistent and complete — this is structural, not policy.

## structure

```
Layer 0: Edge Store        content-addressed, immutable
Layer 1: Neuron Index      polynomial commitment, completeness by creator
Layer 2: Particle Index    polynomial commitment, completeness by endpoint
Layer 3: Focus & Balance   polynomial commitments over (neuron_id, value)
Layer 4: UTXO State        mutator set (AOCL + SWBF), privacy layer
```

## dependency graph

```
nebu (field)
  ↓
hemera (hash + trees)
  ↓
nox (VM)
  ↓
zheng (proofs)
  ↓
bbg (state) ← this repo
```

see [[cyber/bbg]] for the full specification, [[WHIR]] for polynomial commitments, [[LogUp]] for cross-index consistency, [[data structure for superintelligence]] for mutator set architecture