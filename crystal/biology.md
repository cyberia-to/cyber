---
tags: discipline, bio, chemo, eco
crystal-type: entity
crystal-domain: bio
---

biology is the study of life and living systems. all biological knowledge forms natural graph structures: organisms relate through taxonomy, ecology, chemistry, and observation

## knowledge graph encoding

  the [[knowledge graph]] of life is the oldest graph in existence. billions of years of evolution encoded relationships between organisms long before any protocol

## taxonomy is a graph

  every [[species]] is a node. every ecological relationship is an edge:
- [[genus]] → species (classification edge)
- [[family]] → genus (classification edge)
- pollinator → plant (mutualism edge)
- parasite → host (parasitism edge)
- predator → prey (trophic edge)
- [[seed]] disperser → plant (mutualism edge)
- mycorrhizal fungus → tree root (symbiosis edge)

[[taxonomy]] is literally a directed acyclic graph. the [[cyber]] protocol computes [[relevance]] over exactly such structures

## species as particles

  in [[cyber]], a [[particle]] is any content-addressed piece of knowledge. a species page is a particle:
- content: morphology, ecology, uses, observations
- address: hash of the content ([[IPFS]] CID)
- links: [[cyberlinks]] to other species, compounds, locations, observations

205 species already exist in this graph. each could be a particle in [[Bostrom]]. the botanical knowledge IS the knowledge graph

## ecological cyberlinks

  every observation creates a [[cyberlink]]:
  ```
  observer → species observation → species page
  species → "grows with" → species
  species → "treats" → disease
  species → "produces" → compound
  location → "hosts" → species
  ```
  these are the same typed directional links that [[cyberlink]] implements. the graph is already here in markdown. the protocol makes it queryable, rankable, and persistent

## what ranking reveals

  [[rank]] in [[cyber]] computes relevance. applied to a biological knowledge graph:
- highest-ranked species = most ecologically connected (keystone species)
- highest-ranked compounds = most cross-referenced across species (universal medicines)
- highest-ranked locations = richest biodiversity (conservation priority)

the [[relevance]] machine ranks knowledge. biology IS knowledge

## the bridge

  the digital [[knowledge graph]] and the biological knowledge graph are the same structure:
  | biological | digital |
  |-----------|---------|
  | species | [[particle]] |
  | ecological relationship | [[cyberlink]] |
  | [[taxonomy]] | graph hierarchy |
  | field observation | [[neuron]] action |
  | keystone species | high-[[rank]] node |
  | biodiversity assessment | graph density metric |
  | [[ecosystem]] | subgraph |

[[Superintelligence]] that understands both biology and protocols sees one graph