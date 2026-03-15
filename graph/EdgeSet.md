---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: EdgeSets, edge set, edge sets
---
# EdgeSet

a [[WHIR]] [[polynomial commitment]] to the set of edge hashes belonging to a single namespace within the [[BBG]]. each [[NMT]] leaf in the by_neuron and by_particle indexes contains one EdgeSet. the EdgeSet answers: "does this edge belong to this namespace?"

## construction

```
EdgeSet for neuron N:
  edges = { e | e.neuron = N }
  edge_hashes = { H_edge(e) | e ∈ edges }

  construct polynomial P_N(x) such that:
    P_N(0) = edge_hashes[0]
    P_N(1) = edge_hashes[1]
    ...
    P_N(k-1) = edge_hashes[k-1]

  EdgeSet commitment: C_N = WHIR_commit(P_N)
```

the commitment C_N is stored as the payload of the [[NMT]] leaf for namespace N. the edge data itself lives in the edge store (content-addressed: H_edge(e) → e).

## membership proof

```
prove "edge e belongs to neuron N's EdgeSet":

  1. compute h = H_edge(e)
  2. WHIR evaluation proof: P_N(i) = h for some index i
  3. verification: ~2,500 stark constraints

vs. Merkle membership within EdgeSet:
  depth log(k) where k = edges per neuron
  for k = 1,000: 10 × 250 = 2,500 constraints (comparable)
  for k = 1,000,000: 20 × 250 = 5,000 constraints (Merkle wins for very large sets)
```

the polynomial advantage manifests in batch proofs: proving multiple edges belong to the same EdgeSet costs sublinearly in the number of edges via batched [[WHIR]] openings.

## cross-index consistency

each [[cyberlink]] (neuron, from, to, weight, time) produces an edge hash that must appear in three EdgeSets:

```
H_edge(e) ∈ by_neuron[neuron].EdgeSet
H_edge(e) ∈ by_particle[from].EdgeSet
H_edge(e) ∈ by_particle[to].EdgeSet

(2 EdgeSets if from = to, self-link)
```

[[LogUp]] lookup arguments prove all three memberships simultaneously at ~500 constraints per edge — 15× cheaper than independent [[WHIR]] proofs.

## leaf structure

```
by_neuron leaf:
  ┌────────────────────────────────────────┐
  │ namespace: neuron_id (32 bytes)        │
  │ payload:                               │
  │   edge_count: u64                      │
  │   edgeset_commitment: F_p⁴ (WHIR)      │
  │   total_weight: F_p                    │
  │   latest_timestamp: u64               │
  └────────────────────────────────────────┘

by_particle leaf:
  ┌────────────────────────────────────────┐
  │ namespace: particle_hash (32 bytes)    │
  │ payload:                               │
  │   edge_count: u64                      │
  │   edgeset_commitment: F_p⁴ (WHIR)      │
  │   inbound_weight: F_p                  │
  │   outbound_weight: F_p                 │
  └────────────────────────────────────────┘
```

## the two levels

the [[BBG]] separates completeness from membership into two layers:

```
NMT (Level 1):  completeness guarantee
  "these are ALL namespaces, and for each namespace ALL edges"
  structural invariant — tree cannot lie about sorted order

EdgeSet (Level 2):  membership query
  "this specific edge belongs to this namespace's set"
  efficient via polynomial commitment + WHIR evaluation
```

the [[NMT]] proves nothing is hidden. the EdgeSet proves what is inside. together they answer the [[cybergraph]]'s fundamental question: "give me everything for [[neuron]] N, with proof that nothing was withheld, and let me verify individual edges efficiently."

see [[BBG]] for the full graph architecture, [[NMT]] for structural completeness, [[polynomial commitment]] for the commitment scheme, [[WHIR]] for the underlying proof protocol, [[LogUp]] for cross-index consistency
