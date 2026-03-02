---
tags: cyber
crystal-type: entity
crystal-domain: cyber
stake: 23574464290686432
---
how [[datalog]] maps to the [[cybergraph]] — schema, query patterns, and integration with the [[soft3]] stack

## cybergraph schema

the [[cybergraph]] maps naturally to stored relations. [[particles]] are nodes, [[cyberlinks]] are edges, [[neurons]] are agents, [[focus]] is the ranking output

```datalog
// core graph structure
:create particles {
    cid: String
    =>
    content_type: String,
    size: Int,
    created: Validity
}

:create cyberlinks {
    neuron: String,
    from_cid: String,
    to_cid: String
    =>
    weight: Float,
    timestamp: Validity
}

:create neurons {
    address: String
    =>
    stake: Int,
    karma: Float,
    link_count: Int
}

// tri-kernel output
:create focus {
    cid: String
    =>
    score: Float
}

:create karma {
    neuron: String
    =>
    score: Float
}
```

the key structure reflects query access patterns: [[cyberlinks]] are keyed by (neuron, from, to) for uniqueness — one [[neuron]] can create at most one link between any pair of [[particles]]. [[focus]] and [[karma]] are keyed by their subject for direct lookup

## common query patterns

### search — find relevant particles for a query

```datalog
// probabilistic resolution: what does "photosynthesis" link to?
results[to_cid, focus_score] := *cyberlinks{from_cid: "Qm_photosynthesis", to_cid},
                                 *focus{cid: to_cid, score: focus_score}
?[to_cid, focus_score] := results[to_cid, focus_score]
:sort -focus_score
:limit 20
```

### linkchain traversal — find transitive connections

```datalog
// recursive linkchain: what can be reached from a particle in N hops?
reachable[cid, 0] := cid = "Qm_start_particle"
reachable[to, depth + 1] := reachable[from, depth],
                             *cyberlinks{from_cid: from, to_cid: to},
                             depth < 5

?[cid, min_depth] := reachable[cid, depth], min_depth = min(depth)
:sort min_depth
```

### motif detection — find recurring subgraph patterns

```datalog
// triadic closure: A→B, B→C, A→C
triangles[a, b, c] := *cyberlinks{from_cid: a, to_cid: b},
                       *cyberlinks{from_cid: b, to_cid: c},
                       *cyberlinks{from_cid: a, to_cid: c},
                       a != c, b != c

?[a, b, c] := triangles[a, b, c]
:limit 100
```

```datalog
// co-citation: multiple neurons linking the same pair
cocitation[from_cid, to_cid, count(neuron)] := *cyberlinks{neuron, from_cid, to_cid}

?[from_cid, to_cid, n_citations] := cocitation[from_cid, to_cid, n_citations],
                                     n_citations > 3
:sort -n_citations
```

### semcon discovery — find emergent semantic conventions

```datalog
// particles that appear as middle nodes in many A→X→B patterns
// (candidate semcons — structural bridges)
bridge_count[middle, count(pair)] := *cyberlinks{from_cid: a, to_cid: middle},
                                      *cyberlinks{from_cid: middle, to_cid: b},
                                      pair = list(a, b)

?[middle, n_pairs, focus_score] := bridge_count[middle, n_pairs],
                                    *focus{cid: middle, score: focus_score},
                                    n_pairs > 10
:sort -n_pairs
```

### namespace traversal — explore a neuron's file system

```datalog
// list all particles in a neuron's namespace
?[path_particle, target, focus_score] := *cyberlinks{neuron: "bostrom1master...",
                                                       from_cid: path_particle,
                                                       to_cid: target},
                                          *focus{cid: target, score: focus_score}
:sort -focus_score
```

### neuron analysis — karma and contribution patterns

```datalog
// top neurons by total focus contribution
neuron_focus[neuron, sum(focus_score)] := *cyberlinks{neuron, to_cid},
                                           *focus{cid: to_cid, score: focus_score}

?[neuron, total_focus, karma_score] := neuron_focus[neuron, total_focus],
                                        *karma{neuron, score: karma_score}
:sort -total_focus
:limit 50
```

## graph algorithms on the cybergraph

fixed rules operate directly on cyberlink relations. see [[datalog/algorithms]] for the full reference

```datalog
// PageRank over cyberlinks (compare with tri-kernel diffusion)
edges[from_cid, to_cid] := *cyberlinks{from_cid, to_cid}
?[cid, rank] <~ PageRank(edges[], damping: 0.85)
:sort -rank
:limit 20
```

```datalog
// find communities of particles via Louvain
edges[from_cid, to_cid, weight] := *cyberlinks{from_cid, to_cid, weight}
?[cid, community] <~ CommunityDetectionLouvain(edges[])
```

```datalog
// shortest path between two particles (weighted by inverse focus)
edges[from_cid, to_cid, 1.0 / weight] := *cyberlinks{from_cid, to_cid, weight},
                                            weight > 0
start[] <- [["Qm_source"]]
goal[] <- [["Qm_target"]]
?[cid] <~ ShortestPathDijkstra(edges[], start[], goal[])
```

## integration with rune

[[rune]] scripts invoke datalog queries through the `ctx` API in the [[cyb]] runtime

```rune
// rune calling datalog
async fn find_related(particle: Particle, limit: int) -> Vec<Particle> {
    let results = ctx.query(f"""
        ?[to_cid, score] := *cyberlinks{{from_cid: "{particle.cid}", to_cid}},
                             *focus{{cid: to_cid, score}}
        :sort -score
        :limit {limit}
    """);

    results.map(|row| resolve(row.to_cid))
}
```

## integration with neural language

[[neural language]] patterns map to datalog queries:

| neural language concept | datalog query pattern |
|---|---|
| [[semcon]] discovery | bridge particle detection (high betweenness) |
| [[sentence]] parsing | ordered cyberlink batch within transaction |
| [[motif]] detection | subgraph pattern matching via recursive rules |
| [[name]] resolution | deterministic lookup: latest link by neuron + path |
| [[linkchain]] traversal | recursive reachability with depth tracking |
| [[semantic core]] | top-k particles by focus score |

## time-travel

CozoDB supports querying past states of any relation. for the [[cybergraph]], this means: how did [[focus]] distribution look yesterday? which [[cyberlinks]] existed at block N? how has a [[particle]]'s [[cyberank]] evolved?

```datalog
// focus score of a particle at a past point
?[score] := *focus{cid: "Qm_target", score} @ "2025-01-01T00:00:00Z"
```

see [[datalog/stored relations]] for transaction and time-travel mechanics
