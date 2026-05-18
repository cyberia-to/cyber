---
tags: cyber, hardware, architecture
crystal-type: article
crystal-domain: cyber
alias: hardware architecture, compute-memory split
---
# hardware architecture

the brain has one compute primitive (neuron) and different wiring (connectome). nox has the same structure: 16 algebra-polymorphic patterns are the neuron types, the noun tree topology is the wiring. intelligence emerges from connectivity, not from diverse compute elements.

this insight splits hardware into two independent concerns: a small universal compute substrate and a large algebra-dependent memory system.

## compute is small and universal

all 16 nox patterns reduce to five hardware units. four are the [[Goldilocks field processor|GFP]] primitives:

| unit | GFP primitive | patterns served | silicon cost |
|------|--------------|-----------------|--------------|
| field ALU | fma | 5-10 (field arithmetic) | 256 MAC units |
| transform engine | ntt | butterfly networks | dedicated datapath |
| hash pipeline | p2r | pattern 15 (Poseidon2 rounds) | 4-deep pipeline |
| lookup engine | lut | table evaluation | 4 x 64K SRAM tables |
| binary ALU | — | 11-14 (AND, XOR, shifts) | trivial gate array |

the binary ALU is not a GFP primitive because it requires no field arithmetic — raw AND/XOR gates suffice. total compute silicon: roughly 200mm² at 7nm. this covers every computation nox can express across all thirteen [[languages]].

the key property: compute does not change when the algebra changes. Arc, Ten, Wav, Geo — all execute on the same four primitives. only the instruction sequence differs. the GFP ISA (10 instructions) is stable because the mathematical operations are stable (§1.4 of the [[Goldilocks field processor|GFP spec]]).

## memory is large and algebra-dependent

the noun store is where algebras diverge. every nox value is a noun (binary tree of atoms). the tree topology determines how data connects to computation. different algebras produce different tree shapes:

| algebra | typical noun shape | atom width | access pattern |
|---------|--------------------|------------|----------------|
| Arc | deep, irregular (graph adjacency) | 64-bit F_p | random traversal |
| Ten | wide, regular (tensors as nested lists) | 64-bit F_p | dense sequential |
| Bt | compact, balanced (binary trees) | 1-bit F₂ | structured descent |
| Wav | butterfly-structured (FFT coefficients) | 64-bit F_p | stride-2^k access |
| Any | hash-width leaves (Poseidon2 digests) | 512-bit hash | Merkle path lookup |

content-addressed storage means every unique noun is stored exactly once. the [[bbg]] layer provides this with polynomial commitment indexes for cryptographic completeness proofs. the storage system must handle:

- **leaf-width adaptation**: F₂ atoms pack 64 per word, F_p atoms use one word, hash atoms span 8 words. the same tree traversal hardware must handle all three widths efficiently
- **Merkle path caching**: hot paths (frequently accessed subtrees) stay in L1 SRAM. bbg's NMT indexing determines which paths are hot
- **noun prefetch**: tree structure is known before traversal begins (the formula determines which axes will be accessed). prefetch the path before the compute unit needs it

memory dominates silicon area and power. the GFP compute units occupy ~200mm². the memory hierarchy (8 MB L1 SRAM + HBM controller + content-addressed lookup logic) occupies the rest of the die budget and consumes the majority of power.

## storage IS wiring

in a content-addressed system, the noun tree topology IS the connectivity graph between operations and data. there is no separate "bus" or "interconnect" — the tree structure determines what connects to what.

`axis(s, 2)` means "follow this wire to the left child." `axis(s, 7)` means "go right, right, left." these are not memory accesses in the conventional sense — they are wire-following operations through a content-addressed graph.

this means:

- **changing the algebra changes the wiring**. Arc produces deep irregular trees (sparse random connectivity). Ten produces wide regular trees (dense grid connectivity). the same GFP compute units see completely different data flow patterns depending on which algebra generated the nouns
- **optimizing tree traversal optimizes everything**. every algebra uses `axis` to navigate nouns. faster content-addressed lookup, better Merkle path caching, smarter prefetch — these accelerate all thirteen languages simultaneously
- **the noun store is the connectome**. just as brain function depends more on white matter topology than on individual neuron properties, nox execution performance depends more on noun store efficiency than on ALU throughput

## the complete hardware stack

```
┌─────────────────────────────────────────────┐
│              application layer               │
│    13 languages × domain-specific jets       │
├─────────────────────────────────────────────┤
│              nox execution                   │
│    18 patterns → instruction sequences       │
├──────────────────────┬──────────────────────┤
│    GFP (compute)     │    bbg (memory)       │
│                      │                       │
│  fma: field MAC      │  noun store:          │
│  ntt: transforms     │    content-addressed  │
│  p2r: hashing        │    leaf-width-adaptive│
│  lut: lookups        │    Merkle-cached      │
│  binary: AND/XOR     │    NMT-indexed        │
│                      │                       │
│  ~200mm² silicon     │  8MB SRAM + HBM       │
│  ~40% of power       │  ~60% of power        │
├──────────────────────┴──────────────────────┤
│              physical substrate              │
│    7nm/5nm ASIC, PCIe / M.2 / SoC / USB     │
└─────────────────────────────────────────────┘
```

GFP handles compute. bbg handles memory. together they form the universal substrate for all thirteen execution languages. no language-specific hardware exists — the jet mechanism maps language operations to GFP primitives, and the noun store adapts its access patterns to each algebra's tree topology.

## jets bridge the gap

domain-specific operations compile to nox pattern sequences. the jet system recognizes these sequences by formula hash and dispatches them to optimized GFP instruction streams:

```
language    operation              jet         hardware path
────────    ─────────              ───         ─────────────
Arc         rank(g, steps)         matmul      fma array (batch MAC)
Wav         fft(x)                 ntt         ntt engine (butterfly)
Any         hash(x)               hash        p2r pipeline
Ten         activation(x)         lookup      lut engine
Geo         geometric_product     geo_mul     fma array (mul/add)
STARK       verify(proof)         verifier    all four primitives
```

the jet mechanism is the same one that accelerates the STARK verifier. every domain-specific language benefits from the same acceleration path. the [[multiproof architecture]] composes these jets into batch proofs across algebras.

## design implications

**for GFP development**: the compute side is fully specified. the four primitives plus binary ALU cover all 16 patterns. focus GFP engineering on throughput, power efficiency, and the memory interface — not on adding instructions.

**for bbg development**: the memory side is where algebra-specific optimization lives. noun store layout, caching policy, prefetch strategy, and leaf-width packing are the levers that differentiate performance across algebras. bbg's polynomial commitment indexes (NMT, EdgeSet) determine memory access efficiency.

**for nox development**: the jet compiler maps pattern sequences to GFP instructions. jet quality determines how much of the theoretical GFP throughput is realized. the jet recognition system (formula hash matching) must be fast — it runs on every function call.

**for chip architects**: the compute-memory split means GFP and bbg can be developed and optimized independently. compute scales with transistor density (Moore's law). memory scales with packaging (HBM generations, 3D stacking). the interface between them (the noun access protocol) is the critical contract.

---

## cross-references

- [[Goldilocks field processor]] — compute substrate specification
- [[bbg]] — authenticated state layer (memory substrate)
- [[nox]] — execution model (18 patterns: 16 compute + call + look, jet system)
- [[multiproof architecture]] — batch proof composition across algebras
- [[languages]] — the thirteen execution algebras
- [[rosetta stone]] — why the four primitives unify all domains