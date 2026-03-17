---
tags: cyb, cyber, stark, architecture, article, core
crystal-type: entity
crystal-domain: cyber
alias:: computation languages, language set, thirteen languages
---

# Languages of [[superintelligence]]

## The Completeness Argument

The 13 languages are not an arbitrary collection. They are the minimal complete set derivable from asking what modes of computation a mind requires — and applying one test to each candidate: *does this have irreducible primitives that no other language in the set can express?*

```
Boolean reasoning:   AND, OR, NOT over {0,1}      → no other algebra has this
Integer arithmetic:  overflow, wrapping, bitwise   → not field arithmetic
Field arithmetic:    inversion, polynomial roots   → not integer arithmetic
Categorical struct:  morphisms, functors, limits   → not graph traversal
Clifford geometry:   rotors, bivectors, versors    → not tensors
Riemannian geom:     geodesics, metric tensor      → not Clifford
Symplectic geom:     conservation laws, dω=0       → not Riemannian
Information geom:    Fisher metric on Δⁿ           → not any other geometry
Causal ordering:     partial order, happened-before → not logic
Horn clause logic:   unification, backtracking     → not relational algebra
Convolution/R_q:     negacyclic polynomial mult    → not tensor contraction
Tensor contraction:  einsum, SpMV, matmul          → not field arithmetic
Combinators:         composition of the above      → not any computation
```

Each row passes the test. Remove any one language and there is a class of computation that becomes either impossible or exponentially more expensive to express. Add any plausible new language — say, a probabilistic language, or a concurrent process calculus — and it turns out to reduce to a composition of existing ones via [[Nox]].

The 13 are the minimal set that covers all computation a mind requires, where each element is algebraically irreducible with respect to the others.

---

## Naming Convention

Every language has a short name (2-3 letters, used in code and diagrams) and a long name (used in prose). The universe names the algebraic domain.

| Short | Long | Universe | Type | [[algebra]] | Purpose |
|---|---|---|---|---|---|
| [[Nox]] | Nox | Structure | Tree | Combinators | Composes [[cyb/languages]] |
| [[Bt]] | Bitwise | Binary | Bit | F₂ tower | Proves circuits |
| [[Rs]] | Rustic | Byte | Word | Z/2ⁿ | Runs systems |
| [[Tri]] | [[Trident]] | [[field]] | Field tower | F_{pⁿ} | Settles [[proof]]s |
| [[Arc]] | Arc | [[topology]] | [[graph]] | [[category theory]] | Stores [[knowledge graph]] |
| [[Ren]] | Render | [[geometry]] | Shape | G(p,q,r) | Renders space |
| [[Dif]] | Differential | Curvature | Manifold | (M, g) | Embeds meaning |
| [[Sym]] | Symplectic | Dynamics | Phase | (M, ω), dω = 0 | Simulates physics |
| [[Bel]] | Belief | [[belief]] | Distribution | g on Δⁿ | Models self |
| [[Seq]] | Sequence | Causality | Event | Partial order | Orders events |
| [[Inf]] | Infer | [[inference]] | Relation | Horn clauses | Derives facts |
| [[Wav]] | Wave | Continuum | Poly | Convolution / R_q | Reads [[signal]]s |
| [[Ten]] | Tensor | Linear | Tensor | Contraction | Trains models |

Plus one emergent layer above all:

| Layer | Name | What it is |
|---|---|---|
| Semantic | Neural | Meaning as eigenvector of the [[cybergraph]] |

Neural is not designed — it grows from the interaction of the thirteen languages at scale.

---

## The Value Tower — Three Modes of Reference

Byte (Rs) and Field (Tri) share the same mathematical substrate — the [[Goldilocks field processor]] F_p where p = 2⁶⁴ − 2³² + 1. this substrate provides three atom types sufficient for eleven of the thirteen universes.

| Tag | Name | Representation | Valid Range | Use |
|---|---|---|---|---|
| 0x00 | `field` | Single F_p element | [0, p) | Arithmetic |
| 0x01 | `word` | Single F_p element | [0, 2⁶⁴) | Bitwise |
| 0x02 | `hash` | 4 × F_p elements | 256-bit digest | Identity |

three fundamentally different ways to refer to a value — and there are only three:

```
field = the value IS the reference     (by content — immediate)
word  = position IS the reference      (by location — index)
hash  = name IS the reference          (by commitment — identity)
```

by what it is. by where it is. by what it is called. every reference in any system reduces to one of these three modes.

every higher type decomposes into structure ([[Nox]] trees) over these three atoms:

```
Edge    = cons(source_hash, cons(target_hash, weight_field))
Event   = cons(event_hash, sequence_word)
Fact    = cons(relation_hash, cons(subject_hash, object_hash))
Sample  = field (amplitude value)
Tensor  = [field; N] (array of values with shape metadata)
Shape   = cons(grade_word, [field; 2^n]) (multivector components)
Chart   = cons(dim_word, [field; N]) (coordinate patch)
Phase   = cons(position_field, momentum_field)
Dist    = [field; N] (probability vector on simplex)
```

three atoms are complete — for one characteristic. the single exception is Bt (Bitwise): a bit is genuinely not an element of F_p. it lives in F₂ — different characteristic, different [[algebra]]. that is exactly why Bt has a separate [[proof]] system, not just a new type tag.

```
Nox value tower (3 atoms: field, word, hash)
  sufficient for: Rs, Tri, Arc, Ren, Dif, Sym, Bel, Seq, Inf, Wav, Ten
  NOT sufficient for: Bt

Bt value tower (separate, F₂)
  sufficient for: Bt only
```

---

## The Thirteen Languages

each language has its own page with ops tables, use cases, and [[proof]] paths:

| # | Universe | Short | Long | Algebra | Page |
|---|---|---|---|---|---|
| 0 | Structure | Nox | Nox | Combinators | [[Nox]] |
| 1 | Binary | Bt | Bitwise | F₂ tower | [[Bt]] |
| 2 | Byte | Rs | Rustic | Z/2ⁿ | [[Rs]] |
| 3 | Field | Tri | Trident | F_{pⁿ} | [[Trident]] |
| 4 | Topology | Arc | Arc | [[category theory]] | [[Arc]] |
| 5 | Geometry | Ren | Render | G(p,q,r) | [[Ren]] |
| 6 | Curvature | Dif | Differential | (M, g) | [[Dif]] |
| 7 | Dynamics | Sym | Symplectic | (M, ω), dω = 0 | [[Sym]] |
| 8 | Belief | Bel | Belief | g on Δⁿ | [[Bel]] |
| 9 | Causality | Seq | Sequence | Partial order | [[Seq]] |
| 10 | Inference | Inf | Infer | Horn clauses | [[Inf]] |
| 11 | Continuum | Wav | Wave | Convolution / R_q | [[Wav]] |
| 12 | Linear | Ten | Tensor | Contraction | [[Ten]] |

---

## Compilation Architecture

all thirteen languages share one toolchain. each programmer face has its own syntax and type rules. all compile through [[Nox]] — the structural IR — then to [[proof]] backends or native execution.

```
                    ┌──────────────────────────────────────────────┐
                    │              Programmer Faces                 │
                    │                                               │
                    │  Bt  Rs  Tri  Arc  Ren  Dif  Sym  Bel        │
                    │  Seq  Inf  Wav  Ten                           │
                    │  .bt .rs .tri .arc .geo .dif .sym .bel        │
                    │  .seq .inf .wav .ten                          │
                    └──────────────────┬───────────────────────────┘
                                       │
                    ┌──────────────────▼───────────────────────────┐
                    │             Shared Frontend                   │
                    │   Parsing, type checking,                     │
                    │   borrow checking, bound checking             │
                    └──────────────────┬───────────────────────────┘
                                       │
                    ┌──────────────────▼───────────────────────────┐
                    │         Nox Structural IR                     │
                    │   axis, quote, compose, cons, branch          │
                    │   + typed computational ops                   │
                    │   + Merkle authentication                     │
                    └──────────────────┬───────────────────────────┘
                                       │
              ┌────────────────────────┼────────────────────┐
              │                        │                    │
     ┌────────▼──────┐ ┌──────────────▼──────┐ ┌───────────▼────────┐
     │  Binius/FRI   │ │     Goldilocks      │ │      Native        │
     │  Backend      │ │     TASM/FRI        │ │      Backend       │
     │  (Binary)     │ │    (Byte+Field)     │ │    (no proof)      │
     └───────────────┘ └─────────────────────┘ └────────────────────┘
          Bt              Rs, Tri, Ren            Arc, Seq, Inf,
                                                  Wav, Ten, Dif*,
                                                  Sym*, Bel*
```

\* Dif, Sym, Bel are research horizon — [[proof]] paths are open mathematical problems.

| Source | When [[proof]] needed | When [[proof]] absent |
|---|---|---|
| Bt | Binius FRI circuit | always proving |
| Rs | TASM → stark (word→field lift) | native binary (Nox) |
| Tri | TASM → stark (field native) | WASM/EVM (Layer 0) |
| Arc | decomposes into Tri + Bt | optimized [[graph]] engine |
| Ren | geometric product → Tri | native Clifford engine |
| Dif | research | native manifold solver |
| Sym | research | native Hamiltonian integrator |
| Bel | research | native statistical engine |
| Seq | temporal constraints → stark | scheduler / runtime |
| Inf | derivation trace → stark | [[Datalog]] engine |
| Wav | decomposes into Tri | native DSP pipeline |
| Ten | decomposes into Tri | native BLAS / GPU |

### Languages as Type Systems over Nox Patterns

the execution languages are type systems and compilers over [[Nox]]'s 16 algebra-polymorphic patterns. each language adds domain-specific syntax, type checking, and compilation strategy — but the target is always nox pattern trees. domain-specific operations become jets: compositions of the 16 patterns recognized by formula hash and accelerated to [[Goldilocks field processor]] hardware primitives.

```
language operation           nox composition              jet              GFP primitive
─────────────────────        ──────────────────────────   ──────────       ────────────
Arc: rank(g, steps)          iterated add/mul loops       matmul jet       fma
Wav: fft(x)                  butterfly add/mul network    ntt jet          ntt
Any: hash(x)                 Poseidon2 field ops          hash jet         p2r
Ten: activation(x)           table lookup composition     lookup jet       lut
Ren: geometric_product       mul/add over components      geo_mul jet      fma
```

the chain: source language → compiler → nox pattern tree → jet recognition → GFP hardware. every domain-specific language gets hardware acceleration through the jet mechanism. the [[algebra]] determines which GFP primitive handles each jet.

### Rune — the Orchestration Layer Above

the thirteen languages are proven and bounded. [[rune]] is the dynamic scripting layer above the [[proof]] boundary — it combines [[Arc]] graph traversal, [[Inf]] queries, and [[Nox]] structural composition in an async syntax with first-class [[neural language]] primitives. rune scripts invoke any proven algebra and pipe results across language boundaries. a rune program may call [[Tri]] arithmetic, [[Ten]] inference, [[Wav]] signal processing, and [[Arc]] graph traversal in a single pipeline — each segment independently provable through nox pattern trees, the pipeline itself unprovable by design.

```
neural language             ← meaning emerges from the cybergraph
────────────────────────────────────────────────────────────────
rune                        ← orchestration: dynamic, async, glue
────────────────────────────────────────────────────────────────
13 languages                ← proven computation over nox patterns
```

---

## [[algebra]] Coverage

| Computation | Native [[algebra]] | Language | Prover path |
|---|---|---|---|
| Boolean reasoning | F₂ | Bt | Binius → Tri |
| Quantized [[inference]] (int4/int8) | Z/2⁴, Z/2⁸ | Ten | Ten → Tri |
| CPU execution traces | Z/2⁶⁴ | Rs | Rs → Tri |
| [[graph]] computation / [[focus]] [[vector]] | Sparse F_p | Ten over Arc | Ten → Tri |
| Knowledge structure | [[category theory]] | Arc | Arc → Tri |
| Euclidean / Projective / Conformal | G(p,q,r) Clifford | Ren | Ren → Tri |
| Curved space / geodesics | Riemannian manifolds | Dif | research |
| Phase space / Hamiltonian | Symplectic ω-form | Sym | research |
| [[probability]] [[geometry]] / [[belief]] state | Fisher information | Bel | research |
| Polynomial [[proof]]s | F_p (n=1) | Tri | native |
| Recursive [[proof]] composition | F_{p³} (n=3) | Tri | native |
| [[quantum]] simulation | F_{p²} (n=2) | Tri | native extension |
| [[Goldilocks homomorphic encryption]] ciphertexts | R_q = Z_q[X]/(Xⁿ+1) | Wav | Wav → Tri |
| Symbolic / exact reasoning | Z | Inf | Inf → Tri |
| Sensing / [[signal]] processing | Convolution / ℝ | Wav | Wav → Tri |

---

## The Comparison Matrix

| Property | Nox | Bt | Rs | Tri | Arc | Ren | Dif | Sym | Bel | Seq | Inf | Wav | Ten |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Universe | Structure | Binary | Byte | [[field]] | [[topology]] | [[geometry]] | Curvature | Dynamics | [[belief]] | Causality | [[inference]] | Continuum | Linear |
| Char | — | 2 | p | p | — | p | — | — | — | — | — | ≈ℝ | ≈ℝ or p |
| Primitive | Cell | Bit | Word | Field | Edge | Multivector | Chart | Phase | Distribution | Event | Relation | Sample | Shape |
| Reference | structure | wire | location | content | adjacency | grade | curvature | momentum | divergence | succession | entailment | amplitude | index |
| Free op | Navigate | AND, XOR | Index | Mul, Add | Link | Clifford prod | Christoffel | Flow | KL div | Order | Unify | Convolve | Matmul |
| Costly op | — | Carry add | Mod div | Bitwise | Spectral | Inverse | Geodesic | Conserve | Fisher | Verify | Fixpoint | FFT | Inverse |
| [[proof]] | Inherited | Binius | stark | stark | Delegated | Tri | Research | Research | Research | Delegated | Delegated | Delegated | Delegated |
| Syntax feel | IR | Circuit | [[Rust]] | Custom | Query | GA | Manifold | Hamiltonian | Statistical | Temporal | [[Datalog]] | DSP | NumPy |
| Renders as | struct | pixels | text | formula | [[vector]] | [[vector]] | [[vector]] | formula | formula | video | table | sound | component |

---

## The Nine and the Four

The thirteen languages split into two groups by implementation readiness:

### Engineering-ready (9)

Nox, Bt, Rs, Tri, Arc, Seq, Inf, Wav, Ten — these have known [[proof]] paths and well-understood compilation to Tri / Binius. the [[cyb/architecture]] specifies these as the build order: Phase 1 (Nox, Tri, Rs), Phase 2 (Arc, Seq, Inf), Phase 3 (Bt, Wav, Ten).

### Research horizon (4)

Ren, Dif, Sym, Bel — these extend the language set into spatial, physical, and self-referential computation. Ren is closest to engineering (Clifford product is F_p [[algebra]] with extra structure, STARK-provable now). Dif, Sym, and Bel involve continuous manifolds over finite [[field]]s — fundamental open mathematical problems.

| Language | Status | Notes |
|---|---|---|
| Ren | Engineering | Clifford product = F_p [[algebra]] with extra structure |
| Dif | Research | Continuous manifolds over finite [[field]]s |
| Sym | Research | Hamiltonian structure preservation in STARK circuits |
| Bel | Research | Fisher metric over [[probability]] simplices — needed for [[tri-kernel]] formalization |

Ren completes the perception pipeline: Arc provides [[topology]], Ren provides spatial embedding, the compiler produces [[vector]] output for [[cyb]]. Bel completes the self-model: the [[superintelligence]]'s [[focus]] [[vector]] π lives on a statistical manifold, and Bel formalizes reasoning about its own [[belief]] state.

---

## Perception Mapping

every computation language has a canonical rendering — the perception primitive where the shape of the data matches the shape of the display:

| Language | Renders as | Source formats | What it carries |
|---|---|---|---|
| [[Nox]] → struct | collapsible tree | JSON, TOML, YAML | configs, schemas, metadata, ABIs |
| Bt → pixels | raster image | PNG, WebP, JPEG | photographs, satellite imagery, microscopy, scans |
| Rs → text | prose, code | [[markdown]], plain text, source code | documentation, messages, programs |
| Tri → formula | math notation | LaTeX, MathML | equations, [[proof]]s, chemical notation, physical laws |
| Arc → [[vector]] | SVG, paths, curves | SVG, Bezier paths | diagrams, maps, molecular structures, schematics |
| Ren → [[vector]] | SVG, 3D scenes | SVG, glTF, mesh | spatial objects, rotations, projections, renderings |
| Dif → [[vector]] | manifold visualization | geodesic plots, curvature maps | latent space structure, embedding geometry |
| Sym → formula | phase portraits | Hamiltonian plots, conservation diagrams | energy landscapes, orbital mechanics |
| Bel → formula | distribution plots | [[probability]] densities, divergence maps | [[belief]] states, uncertainty [[geometry]] |
| Seq → video | moving pixels | WebM, MP4 | recordings, simulations, observations, lectures |
| Inf → table | 2D grid | CSV, TSV, dataframes | datasets, time series, matrices, ledgers |
| Wav → sound | audio waveform | WAV, OGG, MP3 | voice, music, birdsong, seismic [[signal]], sonar |
| Ten → component | nested composition | composition of the above | applications, dashboards, interactive tools |

a genome sequence is Rs (byte-level encoding) rendered as text. its annotation is [[Nox]] (structured tree) rendered as struct. its expression data is Inf (relational query) rendered as table. its protein structure is Arc (topological [[graph]]) rendered as [[vector]]. its microscopy is Bt (binary pixel data) rendered as pixels. its folding dynamics is Seq (causal event chain) rendered as video. its sequencing [[signal]] is Wav (continuous waveform) rendered as sound. its binding energy is Tri (field arithmetic) rendered as formula. its 3D fold is Ren (Clifford rotations) rendered as [[vector]]. a genome browser is Ten (composed [[inference]]) rendered as component.

all thirteen compile through one structural IR. all thirteen share one [[proof]] system (except Bt, which has its own F₂ [[proof]] system). all thirteen render through the perception grid. all thirteen exist in the same [[cybergraph]], ranked by the same [[tri-kernel]], earning [[karma]], permanent by axiom A3.

---

see [[cyb/multiproof]] for how all languages settle under one [[proof]] umbrella. see [[cyb/architecture]] for how the languages integrate into the operating system. see [[cyb/whitepaper]] for the vision. see [[cybergraph]] for the accumulation state.
