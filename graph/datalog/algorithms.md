---
tags: cyber
crystal-type: entity
crystal-domain: cyber
alias: graph algorithms, fixed rules
stake: 36872019461486496
---
built-in graph algorithms available as fixed rules (`<~`) in [[datalog]]. these run native implementations inside the CozoDB query engine — no external libraries, no data export

fixed rules use a distinct arrow: `?[] <~ AlgorithmName(input[], params...)`. the input is a relation representing edges. the output binds with algorithm-specific columns

## centrality

### PageRank
stationary distribution of a random walk — probability a random walker lands on each node. directly related to [[cyberank]] and [[diffusion]]
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
?[node, rank] <~ PageRank(edges[], theta: 0.85)
:order -rank
:limit 50
```
output: `[node, rank]`. theta is damping (default 0.85). optional `epsilon`, `iterations`

### DegreeCentrality
in-degree, out-degree, and total degree per node
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
?[node, degree, in_deg, out_deg] <~ DegreeCentrality(edges[])
:order -degree
```

### BetweennessCentrality
how often a node lies on shortest paths between all pairs. identifies bridge [[particles]] whose removal fragments the [[cybergraph]]
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
?[node, centrality] <~ BetweennessCentrality(edges[])
:order -centrality
```

### ClosenessCentrality
average shortest-path distance from a node to all reachable nodes
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
?[node, centrality] <~ ClosenessCentrality(edges[])
:order -centrality
```

## pathfinding

### ShortestPathBFS
unweighted shortest path. output: `[path]`
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
?[path] <~ ShortestPathBFS(edges[], "QmParticleA", "QmParticleB")
```

### ShortestPathDijkstra
weighted shortest path for optimal [[linkchains]]. output: `[path, cost]`
```datalog
edges[from, to, weight] := *cyberlinks{from_particle: from, to_particle: to, weight}
?[path, cost] <~ ShortestPathDijkstra(edges[], "QmParticleA", "QmParticleB")
```

### KShortestPathYen
k alternative shortest paths — multiple [[linkchains]] between two [[particles]]
```datalog
edges[from, to, weight] := *cyberlinks{from_particle: from, to_particle: to, weight}
?[path, cost] <~ KShortestPathYen(edges[], "QmParticleA", "QmParticleB", k: 5)
```

### ShortestPathAStar
heuristic-guided search. faster than Dijkstra when a distance estimate is available
```datalog
edges[from, to, weight] := *cyberlinks{from_particle: from, to_particle: to, weight}
heuristic[node, est] := *particle_embeddings{particle: node, distance_estimate: est}
?[path, cost] <~ ShortestPathAStar(edges[], "QmParticleA", "QmParticleB", heuristic[])
```

### BreadthFirstSearch
traversal from a source with depth tracking. optional `limit` caps depth
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
?[node, depth] <~ BreadthFirstSearch(edges[], "QmStartParticle")
```

### DepthFirstSearch
depth-first traversal — explores deep chains before branching
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
?[node, depth] <~ DepthFirstSearch(edges[], "QmStartParticle")
```

## community detection

### CommunityDetectionLouvain
maximizes modularity to find topic clusters in the [[cybergraph]] — groups of [[particles]] densely interlinked but sparsely connected outside. optional `resolution` controls granularity
```datalog
edges[from, to, weight] := *cyberlinks{from_particle: from, to_particle: to, weight}
?[node, community_id] <~ CommunityDetectionLouvain(edges[])
```

### LabelPropagation
faster community detection. each node adopts the majority label of its neighbors
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
?[node, label] <~ LabelPropagation(edges[])
```

### ClusteringCoefficients
fraction of a node's neighbors also connected to each other
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
?[node, coeff] <~ ClusteringCoefficients(edges[])
:order -coeff
```

## connectedness

### ConnectedComponents
connected subgraphs. reveals isolated knowledge islands
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
?[node, component_id] <~ ConnectedComponents(edges[])
```

### StronglyConnectedComponent
subsets where every node reaches every other. required for [[tri-kernel]] convergence: [[cyberank]] stationary distribution exists when the graph is strongly connected or has teleport structure
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
?[node, scc_id] <~ StronglyConnectedComponent(edges[])
```

### MinimumSpanningForestKruskal
minimum spanning forest. Prim's variant (`MinimumSpanningTreePrim`) accepts a root node
```datalog
edges[from, to, weight] := *cyberlinks{from_particle: from, to_particle: to, weight}
?[from, to, weight] <~ MinimumSpanningForestKruskal(edges[])
```

### TopSort
topological ordering of a DAG. fails on cycles — use [[StronglyConnectedComponent]] to collapse them first
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
?[node, order] <~ TopSort(edges[])
```

## random walks

### RandomWalk
the primitive underlying [[diffusion]]. starts from a node, samples paths by following random edges. `steps` is walk length, `times` is walk count. visit frequency approximates the [[diffusion]] stationary distribution locally
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
?[node] <~ RandomWalk(edges[], "QmStartParticle", steps: 100, times: 10)
```

## cybergraph examples

### find communities of particles
```datalog
edges[from, to, w] := *cyberlinks{from_particle: from, to_particle: to, weight: w}
communities[node, cid] <~ CommunityDetectionLouvain(edges[])
?[community_id, size] := communities[_, community_id], size = count(community_id)
:order -size
:limit 10
```

### compute PageRank on cyberlinks
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
ranked[node, rank] <~ PageRank(edges[], theta: 0.85, epsilon: 1e-6)
?[particle, rank, name] := ranked[particle, rank], *names{particle, name}
:order -rank
:limit 25
```

### find shortest linkchain between two particles
```datalog
edges[from, to, w] := *cyberlinks{from_particle: from, to_particle: to, weight: w}
?[path, cost] <~ ShortestPathDijkstra(edges[], "QmSourceHash", "QmTargetHash")
```

### detect bridge particles via betweenness centrality
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
bridges[node, score] <~ BetweennessCentrality(edges[])
?[particle, score, name] := bridges[particle, score], score > 0.01, *names{particle, name}
:order -score
```

### random walk from a particle
```datalog
edges[from, to] := *cyberlinks{from_particle: from, to_particle: to}
visited[node] <~ RandomWalk(edges[], "QmStartHash", steps: 50, times: 20)
?[node, visit_count] := visited[node], visit_count = count(node)
:order -visit_count
:limit 20
```

see [[datalog]] for language overview. see [[datalog/queries]] for CozoScript syntax. see [[cyberank]] for how PageRank generalizes into the [[tri-kernel]]

discover all [[concepts]]
