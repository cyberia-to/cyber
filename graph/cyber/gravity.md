---
tags: cyber, cybernomics, cip
crystal-type: entity
crystal-domain: cyber
alias: gravities, knowledge gravity
---
# Gravity

Gravity is a pair-level metric in the [[cyber]] knowledge graph defined as the product of two nodes' [[focus]] probabilities divided by the square of their graph distance:

$$G_{ij} = \frac{\pi_i \cdot \pi_j}{d(i,j)^2}$$

where π_i, π_j are stationary [[focus]] probabilities from the [[tri-kernel]] and d(i,j) is the shortest path length in the [[cyberlink]] graph.

## Physical Analogy

Newton's gravitational force between two masses: F = G·m₁·m₂/r². In the knowledge graph:

| Physics | Knowledge Graph |
|---------|----------------|
| Mass m | Focus probability π |
| Distance r | Graph distance d(i,j) |
| Force F | Conceptual binding strength |

Two high-focus nodes that are close in the graph exert strong mutual gravity — they form a conceptual bond. Two high-focus nodes far apart represent latent structure: a missing bridge that would collapse distance and unlock knowledge flow.

## Structural Roles

The gravity spectrum classifies node pairs:

| Gravity | Profile | Meaning |
|---------|---------|---------|
| High | Both high π, short d | Structural pillars — the skeleton of the graph |
| Medium | One high π, short d | Satellite relationship — one concept orbits another |
| Low but nonzero | High π, large d | Latent bridge — connecting these would restructure the graph |
| Near zero | Low π on either side | Peripheral pair — structurally irrelevant to each other |

## Applications

Skeleton extraction: Sort all pairs by gravity descending. The top-k pairs form the minimum structural skeleton of the knowledge graph — the bonds that hold the graph together. Removing any of these bonds fragments the graph into disconnected clusters.

Bridge discovery: Pairs with high π_i · π_j but large d(i,j) are the most valuable missing links. Creating a [[cyberlink]] between them collapses their distance and restructures [[attention]] flow across the graph.

Cluster detection: Nodes form natural clusters when intra-cluster gravity exceeds inter-cluster gravity. The gravitational clustering threshold provides a principled way to discover topic boundaries.

## Total Gravity

The total gravitational energy of the graph:

$$G_{total} = \sum_{i < j} \frac{\pi_i \cdot \pi_j}{d(i,j)^2}$$

Maximizing G_total means concentrating [[focus]] on nodes that are close to each other — a compact, dense knowledge core. A graph with high total gravity is structurally coherent. A graph with low total gravity is fragmented.

## Relation to [[luminosity]]

[[Luminosity]] is a node metric (size × π — what a node radiates). Gravity is a pair metric (π × π / d² — how nodes bind to each other). Luminosity measures knowledge output. Gravity measures structural cohesion. A healthy graph needs both: high-luminosity nodes that radiate knowledge, connected by high-gravity bonds that form a coherent skeleton.
