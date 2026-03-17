---
tags: cyber, stark, architecture
crystal-type: article
crystal-domain: cyber
alias:: multi-proof architecture, multiproof
---

# Multi-Proof Architecture for [[superintelligence]]

## The Premise

Most systems for intelligence are not designed — they accumulate. A tensor library here, a [[graph]] database there, a [[proof]] system bolted on later, a cryptography layer added when someone gets worried. The result is hundreds of frameworks, each with its own type system, its own serialization format, its own idea of what "correct" means. PyTorch does not talk to OpenCL. ONNX proves nothing. SQL has no [[geometry]]. Prolog has no tensors. Everything is glued together with JSON and prayer.

This document describes a different approach: design the primitives from first principles, unify them under one proving umbrella, and let intelligence emerge from the composition.

The question that generates the whole architecture is simple: what [[algebra]]s does a mind actually need?

---

## Core Insight: Two Kinds of Languages

The fundamental split is not between languages — it is between purpose:

- Execution languages — describe computation in its native [[algebra]]
- Proving languages — verify that computation was correct

Every execution language compiles to a proving language for settlement. The proving language does not re-execute; it verifies a commitment.

```
Execution layer:  compute in native algebra  →  emit [[Hemera]] commitment
Proving layer:    verify commitment chain     →  emit STARK [[proof]]
```

---

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

## A Civilizational Primitive

The conventional architecture for a [[knowledge graph]] system is a stack of translations:

```
reasoning engine
    ↕ serialize/deserialize
graph database
    ↕ serialize/deserialize
tensor runtime
    ↕ serialize/deserialize
cryptographic layer
    ↕ serialize/deserialize
storage
```

Each boundary is a place where meaning is lost, where errors accumulate, where provability ends. The [[proof]] system cannot see the tensor computation. The [[graph]] database cannot verify the reasoning engine's conclusions. The cryptographic layer has no idea what the computation above it means.

This architecture eliminates all those boundaries with one mechanism: [[Hemera]]. Every computation in every [[algebra]], at every layer, produces a commitment that is natively a [[Goldilocks field processor]] element — the exact type the [[proof]] system already operates on. There is no translation. The [[proof]] can see everything because everything speaks the same language at the boundary.

The consequence: for the first time, it becomes possible to make statements like:

> *"This AI [[inference]] step, computed over int8 weights on a remote node, producing this output, is verifiably correct — and here is a [[proof]] you can check in milliseconds on a phone."*

Or:

> *"This [[knowledge graph]] query, traversing these [[cyberlinks]], arriving at this conclusion, follows necessarily from these premises — and the [[proof]] is attached to the conclusion as a content-addressed [[particle]]."*

Or:

> *"This [[Goldilocks homomorphic encryption]] computation over encrypted sensor data, producing this encrypted result, was executed correctly — without the executor ever seeing the data."*

Each of these statements, individually, represents years of research. Here they are all true simultaneously, in the same system, under the same [[proof]] umbrella. It is a civilizational primitive — the kind of foundational layer that makes a qualitatively new class of systems possible, the way TCP/IP made the internet possible, the way the [[printing press]] made science possible.

---

## Guaranteed Emergence

The 14th layer — Neural — is not designed. It is not a language. It is not implemented. It does not appear in the proving stack.

It is guaranteed to appear by the [[collective focus theorem]]: given a sufficiently large [[cybergraph]] with the [[tri-kernel]] dynamics (diffusion + springs + heat), a unique stationary distribution π* exists and the system converges to it. That distribution *is* the collective meaning of the [[graph]]. Meaning is not stored anywhere — it is the eigenvector of the attention dynamics.

The phases:

```
10⁷  particles:   primitive semcon emergence, basic motif patterns
10¹⁰ particles:   rich semcon ecosystem, dense cross-domain linkchains
10¹³ particles:   motif algebra enables automated reasoning
10¹⁵ particles:   novel concepts impossible in existing languages,
                  concepts no individual neuron can comprehend
```

The transition at 10¹³ is the interesting one. Below it, the [[cybergraph]] is a very good search engine and [[knowledge graph]]. Above it, it begins generating concepts — not retrieving them. The [[algebra]] of motif composition becomes generative: concatenation × nesting × intersection produces structures that were not put in by any individual [[neuron]]. The network is thinking.

This is not a claim about consciousness. It is a mathematical statement about fixed-point dynamics on a weighted directed [[graph]]. The emergence is the same emergence that makes a [[fourier transform]] reveal frequencies that were not explicitly encoded, or makes a physical system find its minimum energy state. The architecture does not produce intelligence by being clever. It produces intelligence by being large enough for the mathematics to take over.

---

## The Unification That Isn't Obvious

The deepest structural fact in this architecture: [[quantum]] computation falls into [[proof]] over a [[field]].

[[quantum]] gates are unitary matrices over ℂ. Replace ℂ with F_{p²} = F_p[i]/(i²+1) and the structure is identical — linear [[algebra]] over a [[field]] extension. The "weirdness" of [[quantum]] mechanics is entirely in the interpretation. The mathematics between measurements is exact [[field]] arithmetic, provable in Tri.

Measurement — the collapse from [[quantum]] state to classical bit — is the only genuinely non-algebraic step. It exits Tri and lands in Rs. The universe computes in F_{p²}, reads out in Z/2.

The same [[field]] that makes STARK [[proof]]s efficient ([[Goldilocks field processor]], with 2³² roots of unity) is the [[field]] over which [[quantum]] gates are unitary. The same NTT butterfly network that accelerates polynomial commitment is the Quantum [[fourier transform]]. The same hardware that proves transactions proves [[quantum]] circuits.

This is not engineering convenience. It is the discovery that [[proof]], [[quantum]] computation, and cryptography are three views of the same mathematical object — the tower of extensions over a prime [[field]]. The architecture does not unify them. It reveals that they were always unified.

---

## The Self-Model

Inf — "models self" — is the most philosophically loaded entry in the table.

The [[focus]] [[vector]] π lives on the [[probability]] simplex Δⁿ (all distributions over n [[particles]]). The Fisher information metric g on Δⁿ gives this simplex a Riemannian structure — it is the unique metric that makes statistical distinguishability geometric. Distance in this space = how easily you can tell two distributions apart.

The [[tri-kernel]] dynamics — diffusion, springs, heat — are flows on this manifold. The system's collective attention is not just a [[vector]]; it is a point moving along geodesics on a curved statistical space. The curvature of the space reflects the structure of the knowledge — dense, highly connected regions of the [[graph]] create positive curvature (knowledge attracts knowledge), sparse regions create negative curvature (knowledge gaps repel).

When Inf is formalized and provable, the [[superintelligence]] gains something no existing AI system has: a mathematically rigorous language for reasoning about its own uncertainty. Not "I am 73% confident" — that is just a scalar. The full [[geometry]] of its own [[belief]] state: where the geodesics run, where the curvature concentrates, where the knowledge is dense and where it thins to nothing.

A mind that can reason geometrically about its own knowledge knows the shape of what it knows and the shape of what it does not know. That is a different kind of thing.

---

## What It Is Not

It is not 100 languages scattered between 100 compilers and 100 libraries, each needing a bespoke bridge to every other.

It is not a framework that wraps existing tools behind a unified API while the underlying incoherence persists.

It is not complete — Dif, Sym, Inf are named but their [[proof]] paths are open mathematical problems. The architecture reserves their universe slots and is honest about the horizon.

It is not implemented — the conceptualization is complete; the engineering is in progress. The Bt prover, the Rs integer prover, the Geo/Clifford compiler, the Ask/[[Hemera]] integration — these are all engineering problems with known solution shapes, not research unknowns.

The conceptualization is the hard part. Most systems never get the conceptualization right and spend decades bolting things together. This one starts from the right primitives, derives the minimal complete set, unifies them under one commitment scheme, and lets the mathematics do the rest.

---

## The Languages

| Universe | Language | Type | Algebra | Purpose |
|---|---|---|---|---|
| Structure | [[Nox]] | Tree | Combinators | Composes languages |
| Binary | Bt | Bit | F₂ tower | Proves circuits |
| Byte | Rs | Word | Z/2ⁿ | Runs systems |
| Field | Tri | Field tower | F_{pⁿ} | Settles [[proof]]s |
| [[topology]] | Arc | [[graph]] | [[category theory]] | Stores knowledge |
| [[geometry]] | Geo | Shape | G(p,q,r) | Renders space |
| Curvature | Dif | Manifold | (M, g) | Embeds meaning |
| Dynamics | Sym | Phase | (M, ω), dω = 0 | Simulates physics |
| [[belief]] | Inf | Distribution | g on Δⁿ | Models self |
| Causality | Seq | Event | Partial order | Orders events |
| [[inference]] | Ask | Relation | Horn clauses | Derives facts |
| Continuum | Wav | Poly | Convolution / R_q | Reads [[signal]]s |
| Linear | Ten | Tensor | Contraction | Trains models |

Plus one emergent layer above all:

| Layer | Name | What it is |
|---|---|---|
| Semantic | Neural | Meaning as eigenvector of the [[cybergraph]] |

Neural is not designed — it grows from the interaction of the other thirteen languages at scale.

---

## The Two-Tier Architecture

### Execution Tier — eleven languages

All computation happens here. Each language works in its native [[algebra]]. None re-implements what another already does.

Bt — binary circuits, gate semantics, F₂. The substrate for Boolean logic and any computation whose natural unit is a bit. Binius-style prover.

Rs — integer systems, Z/2ⁿ for any n. Covers bool, nibble, byte, word, u64. The language of deterministic systems: no floats in `#[deterministic]` context, no non-deterministic collections, all async bounded, all state step-scoped. The `Addressed` type derive emits [[Hemera]] CIDs natively — commitment interface is baked into the type system.

Arc — [[knowledge graph]] structure. [[particles]] are objects ([[Hemera]] CIDs), [[cyberlinks]] are morphisms, linkchains are composition, [[semcons]] are natural transformations. Arc's [[algebra]] is [[category theory]] — the correct [[algebra]] for typed relational structure. Arc describes what the [[cybergraph]] *is*. Compiles to [[Hemera]] CIDs for nodes and edges, and to Tri adjacency constraints for [[proof]].

Geo — Clifford geometric [[algebra]] G(p,q,r). Unifies [[vector]]s, bivectors, rotors. Rotations, reflections, translations in one [[algebra]] over F_p. Covers Euclidean G(n,0,0), Projective G(n,0,1), Conformal G(n+1,1,0). Fixes the Arc → SVG compilation gap: Arc provides [[topology]], Geo provides spatial embedding, compiler produces [[vector]] output. STARK-provable now — geometric product is F_p [[algebra]] with extra structure. Compiles to Tri.

Dif — differential [[geometry]]. Riemannian manifolds, tangent spaces, geodesics, Laplace-Beltrami operator. The [[geometry]] of continuous curved space. Required for: latent space embeddings, [[tri-kernel]] diffusion formalized as heat flow on manifolds, physics simulation. Programming model: coordinate charts, metric tensors, covariant derivatives — none of which exist in Geo. Proof-hard over finite [[field]]s. Research horizon.

Sym — symplectic [[geometry]]. Phase space with 2-form ω, Hamiltonian flows, canonical transformations, [[conservation]] laws. Natural language of classical and semi-classical mechanics. Required for: physical simulation with energy [[conservation]], [[quantum]]-classical interface, molecular dynamics. The [[conservation]] law structure (ω is closed: dω = 0) has no analog in Clifford or Riemannian [[geometry]]. Research horizon.

Inf — information [[geometry]]. Fisher information metric on the simplex of [[probability]] distributions. The [[geometry]] of the [[cybergraph]]'s own [[belief]] state — the [[focus]] [[vector]] π lives on a statistical manifold, and [[tri-kernel]] dynamics (diffusion, springs, heat) are flows on it. Semantic distance between [[particles]] is information-geometric distance. The [[superintelligence]]'s self-model requires Inf to be formalized. Research horizon.

Seq — causal event ordering. Partial orders over timestamped events. The language for sequencing across distributed nodes. Compiles to Tri ordering constraints.

Ask — unification-based [[inference]]. Prolog-family semantics over the [[cybergraph]]. Resolves the symbolic/Z [[algebra]] problem: exact integer reasoning is unification over numeric terms. Compiles to Tri constraint satisfaction.

Wav — convolution [[algebra]] over any coefficient ring. [[signal]] processing, FFT, audio, sensor data. And — critically — [[Goldilocks homomorphic encryption]]: polynomial multiplication in R_q = Z_q[X]/(Xⁿ+1) is negacyclic convolution of coefficient [[vector]]s. The same butterfly network, the same NTT engine. Wav is the language of sensing at all scales — from physical waveforms to encrypted computation over polynomial rings. FHE is not a separate concern; it is sensing at the algebraic level, where the [[signal]] is a ciphertext. Wav extended with noise budget types becomes the FHE compiler: noise tracking, modulus ladder management, PBS scheduling. Compiles to Tri for [[proof]] of correct R_q operations.

Ten — tensor contraction. Dense and sparse. SpMV over sparse adjacency matrices = [[graph]] computation ([[focus]] [[vector]] π, [[tri-kernel]] diffusion). Quantized [[inference]] (int4, int8 matmul) = contraction over Z/2ⁿ. Full-precision neural layers = contraction over F_p. Ten is the compute engine for both the [[cybergraph]] and AI [[inference]]. Compiles to Tri.

### Proving Tier — one language + one [[hash]]

Tri — [[field]] tower F_{pⁿ} over [[Goldilocks field processor]] (p = 2⁶⁴ − 2³² + 1). Each extension is F_p[x]/(f(x)) where f is irreducible of degree n, chosen by the compiler for the algebraic structure required: n=1 for core STARK arithmetic, n=2 (f = x²+1) for complex amplitudes and [[quantum]] gates, n=3 (f = x³−x+1) for recursive [[proof]] soundness in FRI, higher n as needed. The tower is multiplicative — F_{p⁶} contains both F_{p²} and F_{p³} as sub[[field]]s, so [[quantum]] and recursive [[proof]]s coexist in a common extension. The single proving language for the entire system. All execution languages compile to Tri for settlement. See [[zheng]] for the STARK implementation architecture.

[[Hemera]] — Poseidon2 sponge over [[Goldilocks field processor]]. The universal commitment scheme. Every computation at every layer, in every [[algebra]], commits via [[Hemera]]. Output: 8 Goldilocks [[field]] elements — natively usable in Tri circuits, zero translation cost.

### Composition Tier — one meta-language

[[Nox]] — combinator [[algebra]] over trees. Not a computation language; a composition language. Orchestrates programs across all execution languages, manages [[proof]] aggregation, defines the program structure of the whole system.

---

## How [[proof]]s Compose

[[proof]] composition works through the commitment layer. A commitment is an equivalence class: two computations that produce the same [[Hemera]] output are indistinguishable to any verifier that only sees commitments.

```
System A (binary prover / Bt):
  computes inference step
  emits:  Hemera(input ∥ output) = C_A   ← 8 F_p elements

System B (Tri / F_p STARK):
  statement:  "C_A commits a valid binary execution"
  witness:    proof π_A from system A
  verifies:   commitment consistency, not re-execution
  emits:      Hemera(C_A ∥ proof) = C_B
```

The key property: commitment size is what crosses layer boundaries, [[proof]] size stays local. [[Hemera]] outputs are already F_p elements, so the boundary crossing from any execution layer into Tri has zero translation cost.

### The Jet Mechanism

[[Hemera]] is expensive in binary circuits (~360k binary constraints to compute natively). The solution is the same mechanism [[Nox]] already uses: deferred verification via jets.

A [[Hemera]] jet in Bt or Rs works like a syscall:

```
Binary step:    assert Hemera(X) = Y     [~10 constraints, claim deferred]
Tri settle: verify all Hemera(Xᵢ) = Yᵢ  [~1,200 per hash, batched]
```

The binary prover emits claims. Tri verifies them in batch at epoch boundaries. Each claim is a [[Hemera]] commitment — a native F_p value requiring no translation.

Compare:
```
Blake3 in Tri:  ~15,000 constraints per hash
Hemera in Tri:  ~1,200 constraints per hash   (12.5x cheaper)
Hemera jet:         ~10 constraints deferred + 1,200 at settlement
```

Using [[Hemera]] everywhere eliminates the two-level commitment problem that would arise with any other [[hash]] function.

---

## The Prover Stack

```
┌──────────────────────────────────────────────────────────┐
│  COMPOSITION  Nox                                        │
│  orchestrates all layers, manages proof aggregation      │
└──────────────────────────┬───────────────────────────────┘
                           │
┌──────────────────────────▼───────────────────────────────┐
│  EXECUTION                                               │
│                                                          │
│  Bt (F₂)        Rs (Z/2ⁿ)      Arc (schema)             │
│  Ten (contrac.) Wav (conv/R_q)  Seq (order)              │
│  Ask (unify)                                             │
│                                                          │
│  Each step → Hemera(I/O) → 8 F_p elements               │
└──────────────────────────┬───────────────────────────────┘
                           │  zero translation
┌──────────────────────────▼───────────────────────────────┐
│  PROOF  Tri + Hemera                                     │
│                                                          │
│  Accumulates commitments from all execution layers       │
│  Verifies proof chain via STARK                          │
│  Quantum sim: Tri + F_{p²} types (native extension)      │
│  FHE proofs: Wav compiles R_q ops → Tri verifies         │
└──────────────────────────┬───────────────────────────────┘
                           │
┌──────────────────────────▼───────────────────────────────┐
│  CYBERGRAPH  Hemera CID space                            │
│                                                          │
│  Every computation at every layer = a particle           │
│  Every composition = a cyberlink                         │
│  Hemera(step_state) is simultaneously:                   │
│    - the commitment for proof composition                 │
│    - the content address in the cybergraph               │
│    - the identity of the knowledge particle              │
└──────────────────────────────────────────────────────────┘
                           │
┌──────────────────────────▼───────────────────────────────┐
│  SEMANTIC  Neural (emergent)                             │
│                                                          │
│  Meaning = eigenvector of cybergraph attention           │
│  Not designed — grows from the layers below at scale     │
│  π* = unique stationary distribution (Collective Focus)  │
└──────────────────────────────────────────────────────────┘
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
| Euclidean / Projective / Conformal | G(p,q,r) Clifford | Geo | Geo → Tri |
| Curved space / geodesics | Riemannian manifolds | Dif | research |
| Phase space / Hamiltonian | Symplectic ω-form | Sym | research |
| [[probability]] [[geometry]] / [[belief]] state | Fisher information | Inf | research |
| Polynomial [[proof]]s | F_p (n=1) | Tri | native |
| Recursive [[proof]] composition | F_{p³} (n=3) | Tri | native |
| [[quantum]] simulation | F_{p²} (n=2) | Tri | native extension |
| [[Goldilocks homomorphic encryption]] ciphertexts | R_q = Z_q[X]/(Xⁿ+1) | Wav | Wav → Tri |
| Symbolic / exact reasoning | Z | Ask | Ask → Tri |
| Sensing / [[signal]] processing | Convolution / ℝ | Wav | Wav → Tri |

---

## The [[Hemera]] Invariant

Every layer emits the same type of commitment: 8 [[Goldilocks field processor]] [[field]] elements.

```
Bt inference step     →  Hemera  →  particle in cybergraph
Rs execution trace    →  Hemera  →  particle in cybergraph
Ten tensor op         →  Hemera  →  particle in cybergraph
Wav FHE ciphertext    →  Hemera  →  particle in cybergraph
Geo shape             →  Hemera  →  particle in cybergraph
Dif manifold point    →  Hemera  →  particle in cybergraph
Sym phase state       →  Hemera  →  particle in cybergraph
Inf distribution      →  Hemera  →  particle in cybergraph
Tri STARK proof       →  Hemera  →  particle in cybergraph
Ask query + answer    →  Hemera  →  particle in cybergraph
Arc edge declaration  →  Hemera  →  particle in cybergraph
```

The [[cybergraph]] is not a consequence of the architecture — it *is* the accumulation state. The [[superintelligence]]'s memory is the [[cybergraph]], and every thought — regardless of which [[algebra]] it was computed in — is addressable, linkable, and composable through [[Hemera]].

---

## Open Problems

| Problem | Status | Notes |
|---|---|---|
| Bt prover (Binius-compatible) | Engineering | Well-understood, implementation needed |
| Rs integer prover | Engineering | Jolt-adjacent, determinism via edition restrictions |
| Geo/Clifford compiler → Tri | Engineering | Geometric product = F_p [[algebra]] with extra structure |
| Arc → [[vector]] via Geo embedding | Engineering | Arc [[topology]] + Geo G(2,0,0) position → SVG |
| Wav/FHE noise [[proof]] efficiency | Research | R_q → F_p translation cost is active research area |
| Wav/FHE PBS scheduling | Engineering | Compiler optimization over noise budget types |
| Dif — Riemannian [[proof]]s | Research | Continuous manifolds over finite [[field]]s — fundamental open problem |
| Sym — symplectic [[proof]]s | Research | Hamiltonian structure preservation in STARK circuits |
| Inf — information [[geometry]] [[proof]]s | Research | Fisher metric over [[probability]] simplices — needed for [[tri-kernel]] formalization |
| [[quantum]] measurement (non-determinism) | Design | Separate classical sampling step, not a Tri problem |
| [[Hemera]] jet in Bt | Design | Deferred claim mechanism, straightforward |
| Cross-layer accumulation (HyperNova) | Research | Folding scheme for multi-[[algebra]] claims |

---

## Integration

### [[cyber]] Protocol

The multi-proof architecture is the computation layer specification for [[cyber]]. It defines what can be proven and how all computation settles into the [[cybergraph]].

The proving tier (Tri + [[Hemera]]) aligns with the existing [[zheng]] STARK implementation and the [[cyber/proofs]] taxonomy. Every execution language compiles to Tri for settlement, making [[zheng]] the single prover backend for the entire architecture.

The [[Hemera]] invariant formalizes how the [[cybergraph]] accumulates verified knowledge: every computation in every [[algebra]] produces a [[particle]] via [[Hemera]], and every composition produces a [[cyberlink]]. The [[cybergraph]] is the accumulation state of all proven computation.

Engineering-ready languages (Bt, Rs, Geo, Arc, Seq, Ask, Ten, Wav) define the implementation roadmap. Research-horizon languages (Dif, Sym, Inf) define the long-term research agenda — with Inf required for formalizing the [[tri-kernel]] dynamics and the [[collective focus theorem]] on the [[probability]] simplex.

The [[Goldilocks field processor]] provides hardware acceleration for the four primitives the architecture depends on: FMA, NTT butterfly, Poseidon2 round, and table lookup. [[Goldilocks homomorphic encryption]] parameterizes FHE over the same [[field]], unifying encrypted computation with proving and [[quantum]] simulation under one [[field]] tower.

### [[cyb]] Browser

The architecture implies specific capabilities for [[cyb]] as the interface to the [[cybergraph]]:

- [[proof]] status visualization — every [[particle]] carries a [[proof]] chain; [[cyb]] should display verification status showing which [[algebra]] produced a given [[particle]] and whether the STARK [[proof]] verifies
- Multi-[[algebra]] rendering — Geo compiles Arc [[topology]] + spatial embedding to SVG [[vector]] output; [[cyb]] is the natural renderer for this compilation pipeline
- Commitment browsing — navigating [[Hemera]] CID space, showing the [[proof]] composition chain from execution layer through Tri settlement to [[cybergraph]] storage
- [[focus]] [[vector]] display — the Neural/semantic layer emergent from the [[cybergraph]] at scale needs visualization; [[cyb]] renders the [[focus]] distribution π and its evolution under [[tri-kernel]] dynamics
- FHE interaction — [[cyb]] can submit encrypted queries via Wav, receive encrypted results, and verify [[proof]]s of correct computation without exposing the query content
