---
tags: cyber
crystal-type: entity
crystal-domain: cyber
---
[[CozoScript]] query syntax reference for [[datalog]] on the [[cybergraph]]

## rule types

every query is a script of one or more rules. each rule has a head (what it produces) and a body (how it computes)

### inline rules `:=`

logic defined in the rule body. the head declares output variables, the body declares how to compute them. multiple rules with the same head name form a disjunction (logical OR)

```datalog
relevant[particle] := *focus{particle, score}, score > 0.01
relevant[particle] := *karma{neuron, k}, k > 1000,
                      *cyberlinks{neuron, to: particle}
```

### fixed rules `<~`

built-in [[algorithms]] callable as first-class query operations: [[PageRank]], [[Dijkstra]], community detection, random walk. see [[datalog/algorithms]] for the full catalog

```datalog
ranked[particle, score] <~ PageRank(*cyberlinks[from, to])
```

### constant rules `<-`

literal data declared inline. useful for parameterizing queries or injecting lookup tables

```datalog
seeds[address] <- [["bostrom1abc..."], ["bostrom1def..."]]
```

### the entry rule `?`

the special head `?` marks the rule whose output the query returns. every script must have exactly one

```datalog
?[from, to, neuron] := *cyberlinks{from, to, neuron}
```

## atoms

atoms are the components of rule bodies, joined by commas (logical AND)

### rule application

reference another rule by name with positional bindings: `ranked[particle, score]`

### stored relation access

read persistent relations using `*` prefix. two binding styles

| style | syntax | when to use |
|---|---|---|
| positional | `*cyberlinks[from, to]` | column order known |
| named | `*cyberlinks{from: src, to: dst}` | clarity matters |

```datalog
?[src, dst] := *cyberlinks{from: src, to: dst, neuron: "bostrom1abc..."}
```

### expressions and unification

filter with comparisons (`k > 100`), compute with `=`

```datalog
?[neuron, boosted] := *karma{neuron, karma: k},
                       *focus{neuron, score: f},
                       boosted = k * f
```

### list unification

test membership with `in`

```datalog
?[neuron, k] := *karma{neuron, karma: k},
                 neuron in ["bostrom1abc...", "bostrom1def..."]
```

### negation

exclude with `not`. safety rule: every variable in a negated atom must also appear in a positive atom in the same rule. unsafe negation is rejected at compile time

```datalog
?[neuron] := *karma{neuron}, not *cyberlinks{neuron}
```

## recursion

rules can reference themselves for transitive closure, reachability, and arbitrary-depth traversal

```datalog
reachable[particle] := *cyberlinks{from: "Qm_seed_cid", to: particle}
reachable[particle] := reachable[mid],
                       *cyberlinks{from: mid, to: particle}

?[particle] := reachable[particle]
```

| constraint | reason |
|---|---|
| no recursion in negated position | `not reachable[x]` inside its own definition creates unstable fixpoints |
| stratified evaluation | negation and aggregation over recursive rules compute in layers bottom-up |
| semi-lattice aggregations allowed | `min`, `max`, `union`, `intersection` converge monotonically in self-recursive rules |

## aggregation

operators apply to head variables. variables without an operator become grouping keys

```datalog
?[neuron, count(particle)] := *cyberlinks{neuron, to: particle}
```

`neuron` is the grouping key, `count(particle)` aggregates over all matching values

| operator | description | semi-lattice |
|---|---|---|
| `count(x)` | number of values | no |
| `sum(x)` | total | no |
| `mean(x)` | arithmetic mean | no |
| `min(x)` | minimum value | yes |
| `max(x)` | maximum value | yes |
| `collect(x)` | gather into list | no |
| `unique(x)` | deduplicated list | yes |
| `union(x)` | set union | yes |
| `intersection(x)` | set intersection | yes |
| `choice(x)` | arbitrary pick | yes |

semi-lattice aggregations (marked yes) are safe in recursive rules. [[datalog]] uses bag (multiset) semantics by default — duplicates are preserved through computation, reduced by aggregation

## query options

options appear at the end of the script, prefixed with `:`

| option | syntax | description |
|---|---|---|
| `:limit` | `:limit 20` | return at most N rows, enables early stopping |
| `:offset` | `:offset 100` | skip first N rows, combine with `:limit` for pagination |
| `:sort` | `:sort -score` | sort output, `-` descending, `+` ascending (default) |
| `:order` | `:order field` | alias for `:sort` |
| `:timeout` | `:timeout 5` | abort if query exceeds N seconds |
| `:assert none` | `:assert none` | fail if query returns any rows (invariant check) |
| `:assert some` | `:assert some` | fail if query returns zero rows (existence check) |

```datalog
?[particle, score] := *focus{particle, score}
:sort -score
:limit 20
:offset 40
```

```datalog
// invariant: no neuron has negative karma
?[neuron, k] := *karma{neuron, karma: k}, k < 0
:assert none
```

## combining rule types

a single script can mix all rule types: constant rules provide parameters, inline rules define logic, fixed rules invoke [[algorithms]], the entry rule selects output

```datalog
seeds[addr] <- [["bostrom1abc..."], ["bostrom1def..."]]

nearby[particle] := seeds[addr],
                    *cyberlinks{neuron: addr, to: particle}
nearby[particle] := seeds[addr],
                    *cyberlinks{neuron: addr, to: mid},
                    *cyberlinks{from: mid, to: particle}

ranked[particle, score] <~ PageRank(*cyberlinks[from, to])

?[particle, score, k] := nearby[particle],
                          ranked[particle, score],
                          *karma{neuron, karma: k},
                          *cyberlinks{neuron, to: particle}
:sort -score
:limit 20
```

## cybergraph query patterns

### transitive link chains with depth

```datalog
chain[particle, 1] := *cyberlinks{from: "Qm_root", to: particle}
chain[particle, min(depth)] := chain[mid, d],
                                *cyberlinks{from: mid, to: particle},
                                depth = d + 1

?[particle, depth] := chain[particle, depth]
:sort +depth
:limit 100
```

## see also

- [[datalog]] — language overview and design principles
- [[datalog/stored relations]] — schema, mutations, transactions
- [[datalog/algorithms]] — fixed rule catalog: PageRank, Dijkstra, Louvain, BFS
- [[datalog/functions]] — built-in function reference
- [[datalog/cybergraph]] — cybergraph integration and [[rune]] interop
