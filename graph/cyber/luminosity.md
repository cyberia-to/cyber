---
tags: cyber, cybernomics, cip
crystal-type: entity
crystal-domain: cyber
alias: luminosities, knowledge luminosity
---
# Luminosity

Luminosity is a node-level metric in the [[cyber]] knowledge graph defined as the product of content size and [[focus]] probability:

$$L_i = s_i \cdot \pi_i$$

where s_i is the size of page i (in bytes or words) and π_i is its stationary [[focus]] probability from the [[tri-kernel]].

## Physical Analogy

In astrophysics, luminosity L = σ × Φ (cross-section × flux) — how much energy a star radiates. The knowledge graph analogy is precise:

| Physics | Knowledge Graph |
|---------|----------------|
| Cross-section σ | Content size s_i |
| Photon flux Φ | Attention flux π_i |
| Luminosity L | Knowledge radiated into the network |

A page with large content but zero [[focus]] radiates nothing — a dark body. A page with high [[focus]] but no content radiates nothing — a singularity. Luminosity captures what the network actually receives from each node.

## Utility

Luminosity answers a question neither size nor [[focus]] can answer alone: how much knowledge does this node contribute to the network per unit time?

| Metric | Measures | Blind Spot |
|--------|----------|------------|
| Size | Content volume | Ignores whether anyone reads it |
| [[Focus]] | Attention probability | Ignores whether there is content to absorb |
| Luminosity | Knowledge throughput | None — captures both dimensions |

Applications:
- Identify over-invested nodes: high size, low luminosity → content that nobody reaches
- Identify under-invested nodes: high [[focus]], low luminosity → attention bottlenecks that need content
- Resource allocation: luminosity-weighted distribution rewards nodes that actually deliver knowledge

## HR Diagram for Knowledge Graphs

In astronomy the Hertzsprung-Russell diagram plots luminosity vs temperature, classifying stars into main sequence, giants, dwarfs, supergiants. The knowledge graph analogue plots luminosity vs [[focus]]:

```
   L ↑
     |  ★ Red Giants               ★ Supergiants
     |    (big content,               (big content,
     |     moderate focus)             high focus)
     |
     |       · · · Main Sequence · · ·
     |         (content proportional to focus)
     |
     |  · White Dwarfs
     |    (small content, high focus)
     |
     +————————————————————————————→ π
```

| Class | Profile | Example |
|-------|---------|---------|
| Red Giant | Large s, moderate π | Verbose page that accumulated content but lost structural centrality |
| White Dwarf | Small s, high π | Hub page — compact, highly linked, concentrates [[attention]] |
| Supergiant | Large s, high π | Core spec page — comprehensive and central |
| Main Sequence | s ∝ π | Healthy pages — content matches the [[attention]] they receive |

Pages off the main sequence signal structural imbalance: either content should be pruned (red giants) or expanded (white dwarfs).

## Conservation

Since Σ π_i = 1, total luminosity equals the [[focus]]-weighted average size:

$$L_{total} = \sum_i s_i \cdot \pi_i = \mathbb{E}_\pi[s]$$

This is the expected content size encountered by a random walker — the effective knowledge bandwidth of the graph. Maximizing L_total means either growing content on high-focus pages or increasing focus on content-rich pages.

## Relation to [[gravity]]

[[Luminosity]] is a node metric (what a node radiates). [[Gravity]] is a pair metric (how strongly two nodes attract each other). Together they form a complete picture: luminosity determines what each node contributes, gravity determines the structural skeleton through which contributions flow.

## Implementation

Computed as a derived metric from [[focus]] and file size, available in the publisher build pipeline:

```
L_i = size_bytes(i) × π_i
```

Displayed in the files table alongside [[focus]] probability π%.
