---
tags: cyber, computer science, cryptography
crystal-type: entity
crystal-domain: computer science
alias: LogUp lookup argument, LogUp lookup arguments, lookup arguments
---
# LogUp

a lookup argument protocol that proves "set {f₁, ..., f_m} is contained in table {t₁, ..., t_n}" via an algebraic identity. used in [[cyber]] for cross-index consistency in the [[BBG]] — proving that the same edge hash appears in all required [[EdgeSets]].

introduced by Haböck (2022). used in Polygon, Scroll, and [[cyber]].

## the problem

each [[cyberlink]] touches 3 [[EdgeSets]] (or 2 for self-links). the [[stark]] must prove that the edge hash inserted into by_neuron is the SAME hash inserted into by_particle[from] and by_particle[to]. without LogUp, each cross-index check requires independent [[WHIR]] proofs against each EdgeSet — ~7,500 constraints per edge.

## the protocol

LogUp proves containment via the algebraic identity:

```
Σᵢ 1/(β - fᵢ) = Σⱼ mⱼ/(β - tⱼ)

evaluated at a random challenge β via sumcheck
```

for [[cyber]]:

```
CROSS-INDEX CONSISTENCY via LogUp
─────────────────────────────────

transaction: create edge e = (neuron, from, to, weight, time)
edge hash: h = H_edge(e)

lookup statement:
  "h appears in by_neuron[neuron].EdgeSet
   AND h appears in by_particle[from].EdgeSet
   AND h appears in by_particle[to].EdgeSet"

LogUp proof:
  1. f = {h, h, h}  (the same hash, looked up 3 times)
  2. t₁ = by_neuron[neuron].EdgeSet evaluations
  3. t₂ = by_particle[from].EdgeSet evaluations
  4. t₃ = by_particle[to].EdgeSet evaluations

  prover constructs the sumcheck polynomial and commits
  verifier checks the identity at random β
```

## cost advantage

```
per edge:
  LogUp:              ~500 stark constraints (sumcheck + challenges)
  independent WHIR:    3 × 2,500 = 7,500 stark constraints
  savings:            15×

block with 10,000 edges:
  LogUp:              ~5,000,000 constraints
  independent WHIR:    ~75,000,000 constraints

prover work:  O(k log k) where k = edges in transaction
verifier work: O(log k) — dominated by sumcheck verification
```

the key property: prover cost is proportional to what it touches, not to the total graph size. this is bounded locality — the third design law of [[BBG]].

## batch verification

LogUp batches naturally. a block containing B transactions with E total edges:

```
all E edges → one LogUp proof for all 3E lookups

prover:  O(E log E) — linear in block size
verifier: O(log E) — logarithmic in block size
```

this is where LogUp becomes essential: at planetary scale (10¹⁵ nodes), verifying cross-index consistency edge-by-edge is physically impossible. LogUp makes it proportional to the block, not the graph.

## production heritage

LogUp lookup arguments have been deployed in production by Polygon and Scroll since 2023 for cross-table consistency in ZK rollups. [[cyber]] applies the same technique to graph index consistency rather than rollup state tables.

see [[BBG]] for the full graph architecture, [[EdgeSet]] for the polynomial commitments being looked up, [[WHIR]] for the underlying proof mechanism, [[NMT]] for the index structure containing EdgeSets
