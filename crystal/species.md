---
tags: term
icon: 🌈
crystal-type: entity
crystal-domain: cybics
---
[[plants]]: hundreds of species in [[citadel genesis]] and [[batuka]]

[[animals]]: dozens of species in [[citadel genesis]] and [[batuka]]

[[fungi]]: dozens of species in [[batuka]]

system of tagging

- abundance: yes, limited, trial, none, gone
- supply: yes, later, wishlist, no
- margin: high, mid, low, none
- autonomy: support, staple, extra

## [[species/all]]

## [[species/research]]

sets

- [[staple]]
- [[available]]
- [[super]]
- [[major]]

## encoding

  practical spec for encoding the botanical knowledge graph into [[cyber]]

### one species = one particle

  each of the 205 [[species]] pages in this graph:
- has content: description, ecology, uses, observations, images
- gets content-addressed via [[IPFS]] → CID
- becomes a [[particle]] in [[Bostrom]]
- can be [[cyberlinked]] to anything: other species, locations, compounds, observations

example: [[coffea arabica]]
  ```
  particle: QmXk7f...  (IPFS CID of the species page)
  
  cyberlinks from this particle:
    coffea arabica → "family" → Rubiaceae
    coffea arabica → "grows_at" → cyb.land
    coffea arabica → "needs" → shade
    coffea arabica → "companion" → calliandra calothyrsus
    coffea arabica → "produces" → caffeine
    coffea arabica → "observed_by" → [neuron address]
  ```

### observation cyberlinks

  every time a [[neuron]] observes a [[species]] in the field:
  ```
  neuron → "observed" → photo_cid
  photo_cid → "depicts" → species_particle
  photo_cid → "location" → gps_cid
  photo_cid → "timestamp" → block_height
  ```
  the observation is permanent, verifiable, and linked to the [[knowledge graph]]

### what 205 species create

  with ~10 [[cyberlinks]] per species (conservative):
- 2050 cyberlinks encoding ecological relationships
- a queryable biological [[knowledge graph]] inside [[Bostrom]]
- [[rank]] computation reveals: which species is most connected (ecologically central), which location has highest biodiversity, which compounds appear across most species

### [[search]] queries that become possible

- "nitrogen fixing [[tree]]" → ranked list of species by [[relevance]]
- "companion for [[coffea arabica]]" → species connected by "companion" cyberlinks
- "medicinal [[fungi]]" → intersection of fungi particles and medicine cyberlinks
- "what grows at 1500m elevation" → location-linked species subgraph

### bulk encoding

  the 205 species pages can be batch-uploaded:
  ```
  for each .md file in pages/ where tags contain "species":
      cid = ipfs.add(file)
      cyberlink(neuron, cid)                    // "created" link
      cyberlink(cid, genus_cid, "belongs_to")   // taxonomy edge
      cyberlink(cid, location_cid, "found_at")  // geography edge
  ```
  cost: ~600 [[cyberlink]] transactions. at current [[bandwidth]] rates, achievable with moderate [[CYB]] stake via [[investmint]]

### from graph to protocol

  this graph is a prototype. the [[species]] pages, the `[[wiki-links]]`, the tags — they ARE a knowledge graph. the step from markdown to [[Bostrom]] is mechanical:
- markdown page → [[IPFS]] CID → [[particle]]
- `[[wiki-link]]` → [[cyberlink]]
- tag → typed edge
- the graph is already built. it just needs to be committed to the protocol