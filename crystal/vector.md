---
tags: cyb, cyber, core
alias: vector particle, svg, paths, diagrams
crystal-type: entity
crystal-domain: cyb
---
paths, curves, and geometric meaning as [[particle]]. the native format for diagrams, structures, maps, and visual knowledge that must scale without degradation

source format: SVG — any content defined by geometric paths, Bezier curves, and coordinate transformations

---

## rendering

```
svg source → parse → path decomposition → Vello tiling → GPU compute fill → fragment composite
```

Vello rasterizes SVG paths via a GPU compute pipeline: paths decompose into tiles, each tile fills independently in parallel. the result is sub-pixel precision at any zoom level — a molecular structure diagram renders crisply at 1px or at 10,000px. no pixelation. no blur. infinite resolution

## in the cybergraph

vector is how spatial and structural knowledge lives in the graph. diagrams carry meaning that prose cannot — and the [[cybergraph]] makes every diagram a first-class linked object

types of vector particles: molecular structure diagrams, phylogenetic trees, circuit schematics, geographic boundaries, architectural floor plans, network topologies, mathematical graphs, organism anatomy, astronomical charts, chemical reaction diagrams, protein domain maps, flow charts, transport networks

a single vector particle can carry the complete structural knowledge of a domain: the 3D protein fold rendered in 2D, the ecosystem food web, the supply chain graph, the clade tree of an evolutionary lineage

## properties

- resolution-independent — vector particles look identical at any display size or print resolution
- semantically linked — nodes and paths in SVG can carry `id` attributes that [[datalog]] queries can resolve to other particle CIDs. a molecular diagram where each atom links to its particle in the graph
- composable — vector particles nest inside [[component]] particles. a dashboard may contain a live-updating vector chart that pulls from a [[table]] particle
- annotation-ready — the [[cybergraph]] allows meta-linking: a [[cyberlink]] can point to a specific region of a vector particle, making diagram annotation a first-class operation

## relation to other languages

vector is the visual language of structure. [[pixels]] captures reality as it is; vector describes it as it must be understood. [[formula]] states a physical law; vector shows the geometry it describes. together they are the diagram and the equation

see [[svg]] for the source format. see [[pixels]] for raster content. see [[formula]] for mathematical notation that vector paths can render

discover all [[concepts]]
