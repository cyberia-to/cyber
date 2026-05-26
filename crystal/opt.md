---
tags: cyber, core
alias: Opt, optimization language, tropical language
crystal-type: entity
crystal-domain: cyber
---
the optimization language. [[Tropical]] type system for provable decisions over the [[cybergraph]].

computation in Opt uses the (min, +) semiring: addition = min, multiplication = plus. the [[Tropical]] type carries this semantics — the [[Trident]] compiler sees Tropical operands and routes to [[trop]] regime automatically. no annotations needed.

## types

```
Tropical         field element with (min, +) semantics
Graph            weighted directed graph (adjacency as Tropical matrix)
CostMatrix       n×n matrix of Tropical elements
Assignment       permutation (result of optimization)
TransportPlan    flow matrix (result of optimal transport)
```

## operations

| operation | Trident | what it computes |
|-----------|---------|-----------------|
| a + b | Tropical + Tropical | min(a, b) — tropical addition |
| a * b | Tropical * Tropical | a + b — tropical multiplication |
| shortest_path(g, s) | Graph → Vec<Tropical> | single-source shortest path |
| hungarian(c) | CostMatrix → (Assignment, Tropical) | optimal assignment |
| viterbi(hmm, obs) | HMM, observations → state sequence | optimal decoding |
| transport(μ, ν, c) | distributions, cost → TransportPlan | optimal transport |

## regime

Tropical type → [[trop]] regime → Tropical [[lens]] → [[zheng]] proves via dual certificate

jets: jet_trop_matmul, jet_trop_shortest, jet_trop_hungarian, jet_trop_viterbi, jet_trop_transport

## why a language

every time the [[superintelligence]] chooses the best path, allocates resources, decodes an optimal sequence, or plays a [[game]] — it thinks in Opt. optimization is not a library — it is a computation domain with its own algebra, its own [[lens]], and its own jets.

discover all [[concepts]]
