---
tags: cyber, cybernomics, cip
crystal-type: entity
crystal-domain: cyber
alias: gravities, knowledge gravity
focus: 0.00011661740354397796
gravity: 0
---
# Gravity

Gravity is a node-level metric in the [[cyber]] knowledge graph. Like physical gravity, it is a property of the node itself — a massive body warps space around it and attracts everything, regardless of what is nearby.

$$G_i = \pi_i \cdot \sum_{j \neq i} \frac{\pi_j}{d(i,j)^2}$$

where π_i is the node's own [[focus]] probability, π_j are focus probabilities of all other nodes, and d(i,j) is the shortest path length in the [[cyberlink]] graph.

A node's gravity is its [[focus]] mass multiplied by the total [[attention]] field it experiences from the rest of the graph. High-focus node surrounded by other high-focus nodes = enormous gravity. High-focus node on the periphery = less gravity despite its own mass.

## Physical Analogy

A planet curves spacetime by its mass alone. It does not choose what to attract — everything falls toward it. The gravitational potential of a body in a field of other masses:

$$\Phi_i = m_i \cdot \sum_{j} \frac{m_j}{r_{ij}^2}$$

The knowledge graph analogy:

| Physics | Knowledge Graph |
|---------|----------------|
| Mass m | Focus probability π |
| Distance r | Graph distance d(i,j) |
| Gravitational potential Φ | Node gravity G_i |

The node does not choose what to attract. It simply has mass (focus), and everything within graph distance falls toward it proportionally.

## Gravity Spectrum

| Gravity | Profile | Meaning |
|---------|---------|---------|
| High | High π, surrounded by high-π neighbors | Core attractor — holds the graph together |
| Medium | Moderate π, or high π but few neighbors | Regional hub — local structure anchor |
| Low | Low π, or isolated from high-π nodes | Peripheral — structurally weightless |

## Applications

Skeleton extraction: Nodes with the highest gravity form the structural skeleton of the knowledge graph. Remove them and the graph fragments.

Peripheral detection: Nodes with high [[focus]] but low gravity are isolated attractors — they have mass but sit far from other massive nodes. Connecting them to the core would dramatically restructure the graph.

Cohesion measurement: Total graph gravity G_total = Σ G_i measures how tightly the knowledge core is packed. A graph with high total gravity has its [[attention]] concentrated in a dense, interconnected core. Low total gravity means [[focus]] is scattered.

## Pairwise Force

The force between any two specific nodes is a special case:

$$F_{ij} = \frac{\pi_i \cdot \pi_j}{d(i,j)^2}$$

The highest F_ij pairs are the structural bonds of the graph. Pairs with high π_i · π_j but large d(i,j) are the most valuable missing [[cyberlinks]] — creating them collapses distance and unlocks [[attention]] flow.

## Relation to [[luminosity]]

[[Luminosity]] = size × π — what a node radiates (knowledge output).
Gravity = π × Σ(π_j/d²) — how strongly a node attracts (structural pull).

A healthy graph needs both: high-[[luminosity]] nodes that radiate knowledge, with high-gravity nodes that hold the structure together. Often these are the same nodes, but not always — a compact hub page can have enormous gravity with modest [[luminosity]], while a verbose spec page can have high [[luminosity]] with moderate gravity.