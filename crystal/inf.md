---
tags: cyber, language
crystal-type: entity
crystal-domain: cyber
alias: Inf, infer, inference language, datalog, CozoScript, cozodb
---
declarative graph query [[language]] for the [[cybergraph]], implemented via [CozoDB](https://github.com/cozodb/cozo)

part of the [[soft3]] stack, running inside [[cyb]] alongside [[rune]]. where [[rune]] constructs and mutates the [[cybergraph]], datalog queries and reasons over it. where [[trident]] compiles to proofs, datalog compiles to query plans

```datalog
// find all particles linked by a neuron, ranked by focus
?[particle, focus_score] := *cyberlinks{neuron: "bostrom1abc...", to: particle},
                            *focus{particle, score: focus_score}
:sort -focus_score
:limit 20
```

## why datalog for the cybergraph

the [[cybergraph]] is a directed, weighted, authenticated graph. querying it requires: recursive traversal ([[linkchains]]), pattern matching ([[motifs]]), aggregation ([[cyberank]], [[karma]]), and built-in graph algorithms. SQL handles tables. SPARQL handles triples. datalog handles all of this natively

| requirement | SQL | SPARQL | datalog |
|---|---|---|---|
| recursive queries | CTEs (verbose) | property paths (limited) | native recursion |
| graph algorithms | external | external | built-in fixed rules |
| pattern matching | JOINs (manual) | triple patterns | rule composition |
| aggregation | GROUP BY | GROUP BY | inline aggregation |
| set semantics | explicit DISTINCT | implicit | native |
| vector search | extension | external | built-in HNSW |

CozoDB adds what standard datalog lacks: ACID transactions, stored relations, time-travel, vector indices, and a library of graph algorithms callable as fixed rules

## the CozoDB implementation

CozoDB is a hybrid relational-graph-vector database. queries are written in CozoScript — a datalog dialect with extensions for mutations, transactions, and graph algorithms

key capabilities:

- semi-naive evaluation — avoids redundant computation in recursive queries
- magic set rewriting — optimizes queries by restricting computation to relevant subsets
- stratification — handles negation and aggregation in recursive contexts
- fixed rules — built-in graph algorithms (PageRank, Dijkstra, Louvain, BFS, random walk) callable directly in queries
- HNSW indices — vector proximity search for embedding-based queries
- time-travel — query the state of any relation at any past transaction
- MinHash-LSH — near-duplicate detection for content deduplication

## in the stack

```
cyb runtime
  ├── rune    — construct, mutate, script (imperative)
  └── datalog — query, reason, analyze (declarative)
         │
         ├── stored relations  — persistent cybergraph state
         ├── inline rules      — recursive graph traversal
         ├── fixed rules       — PageRank, Dijkstra, Louvain...
         └── HNSW indices      — vector similarity search
```

[[rune]] calls datalog for queries. datalog reads the graph that [[rune]] writes. both run in the [[cyb]] runtime. [[trident]] operates at a different level — it compiles to the proof VM for [[stark]] verification. datalog operates at the application level for interactive queries

## design principles

1. set semantics everywhere. relations are sets of tuples. duplicates are eliminated structurally. this matches the [[cybergraph]] where each [[cyberlink]] is unique
2. recursion as primitive. [[linkchains]], transitive closure, reachability — all require recursion. datalog makes this declarative rather than procedural
3. algorithms as rules. PageRank, shortest path, community detection are fixed rules — first-class query operations, not external libraries
4. schema flexibility. keys and values separated by `=>`. types optional. the same relation can be queried with positional or named bindings
5. transactions as boundaries. every query runs in a transaction. multi-query scripts chain atomically. this aligns with [[sentences]] in [[neural language]] — transaction-atomic semantics
6. graph-native. edges and nodes are the natural data model. no impedance mismatch between the [[cybergraph]] and the query language

## deep dives

- [[inf/queries]] — CozoScript syntax: rules, atoms, recursion, aggregation
- [[inf/stored relations]] — data model: schema, mutations, transactions
- [[inf/algorithms]] — graph algorithms: PageRank, pathfinding, community detection
- [[inf/functions]] — built-in function reference: math, string, vector, JSON
- [[inf/cybergraph]] — integration: cybergraph schema, query patterns, rune interop