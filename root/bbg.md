---
tags: cyber
alias: bbg, Big Badass Graph, authenticated state, cyber/bbg
crystal-type: entity
crystal-domain: cyber
subgraph: true
repo: ../bbg
exclude: ".claude/**, target/**, CLAUDE.md"
diffusion: 0.0009830749016173444
springs: 0.0008907866604507944
heat: 0.000942261310131483
focus: 0.0009472257109702348
gravity: 26
density: 4.19
---
the authenticated state layer for [[cyber]]. stores the [[cybergraph]] — edges ([[cyberlinks]]), [[neuron]] state, [[particle]] energy, [[focus]], balances — with polynomial commitment indexes that provide cryptographic completeness proofs.

when you sync a namespace, you get mathematical proof that nothing was withheld. the graph cannot exist without its indexes being consistent and complete — this is structural, not policy.

## three laws

1. bounded locality — every query touches O(log n) nodes
2. constant-cost verification — proof size ~2 KiB, verification ~5 μs
3. structural security — the graph cannot exist with inconsistent indexes

## structure

13 sub-roots under one state commitment:

9 public [[NMT]] indexes:

| index | content |
|-------|---------|
| particles | content-addressed particle store |
| axons_out | outgoing cyberlink index by creator |
| axons_in | incoming cyberlink index by target |
| neurons | neuron identity and metadata |
| locations | namespace location index |
| coins | token denomination registry |
| cards | delegation and staking cards |
| files | binary blob references |
| time | temporal ordering index |

3 private indexes (mutator set: AOCL + SWBF):

| index | content |
|-------|---------|
| cyberlinks | private edge store |
| spent | spent nullifier set |
| balance | encrypted balance commitments |

1 finalization index:

| index | content |
|-------|---------|
| signals | pending signal queue |

[[LogUp]] ensures cross-index consistency across all 13 sub-roots.

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