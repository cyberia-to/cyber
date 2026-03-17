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
| [[Nox]] | Nox | Structure | Tree | Combinators | Composes languages |
| Bt | Bitwise | Binary | Bit | F₂ tower | Proves circuits |
| Rs | Rustic | Byte | Word | Z/2ⁿ | Runs systems |
| Tri | [[Trident]] | [[field]] | Field tower | F_{pⁿ} | Settles [[proof]]s |
| Arc | Arc | [[topology]] | [[graph]] | [[category theory]] | Stores knowledge |
| Geo | Geometric | [[geometry]] | Shape | G(p,q,r) | Renders space |
| Dif | Differential | Curvature | Manifold | (M, g) | Embeds meaning |
| Sym | Symplectic | Dynamics | Phase | (M, ω), dω = 0 | Simulates physics |
| Inf | Informatic | [[belief]] | Distribution | g on Δⁿ | Models self |
| Seq | Sequence | Causality | Event | Partial order | Orders events |
| Ask | Ask | [[inference]] | Relation | Horn clauses | Derives facts |
| Wav | Wave | Continuum | Poly | Convolution / R_q | Reads [[signal]]s |
| Ten | Tensor | Linear | Tensor | Contraction | Trains models |

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
  sufficient for: Rs, Tri, Arc, Geo, Dif, Sym, Inf, Seq, Ask, Wav, Ten
  NOT sufficient for: Bt

Bt value tower (separate, F₂)
  sufficient for: Bt only
```

---

## The Thirteen Languages

### Universe 0 — Nox (Structure)

the composition language. the continuation of the Nock idea — everything is a binary tree, code is data, computation is tree rewriting. five structural operations define how values compose regardless of what those values are:

| Op | Action | Analogy |
|---|---|---|
| `axis` | Navigate into a subtree by path | Array index |
| `quote` | Treat code as data | String literal |
| `compose` | Chain two computations | Function composition |
| `cons` | Build a pair | Struct constructor |
| `branch` | Conditional selection | If-then-else |

the critical difference from Nock: Nox's tree is a Merkle tree by construction. every `cons(a, b)` computes `hash(a, b)` and stores the digest at the parent node. `axis` produces a Merkle proof as a side effect. the authentication scheme is abstract — pluggable backends (Tip5, Poseidon2, SHA-256, Verkle, SMT).

[[Nox]] is simultaneously the structural IR (the grammar all languages compile through), the node runtime (the production binary that runs the [[cyber]] blockchain), and the composition tier that orchestrates programs across all execution languages, manages [[proof]] aggregation, and defines the program structure of the whole system.

### Universe 1 — Bt (Bitwise)

binary circuits, gate semantics, F₂. the substrate for Boolean logic and any computation whose natural unit is a bit.

| Type | [[field]] | Size | Native ops |
|---|---|---|---|
| `Bit` | F₂ | 1 bit | AND (mul), XOR (add) |
| `Bit2` | F₂² | 2 bits | Extension field ops |
| `Bit8` | F₂⁸ | 1 byte | AES-native |
| `Bit32` | F₂³² | 4 bytes | Hash-native |
| `Bit64` | F₂⁶⁴ | 8 bytes | Double word |
| `Bit128` | F₂¹²⁸ | 16 bytes | Security parameter |

characteristic: 2. [[proof]] system: FRI-Binius. AND is multiplication. XOR is addition. both are free. what Bt cannot do cheaply: integer arithmetic. 3 + 5 = 6 in F₂ (XOR), not 8. to perform actual addition with carry, you must build ripple-carry adders from AND/XOR gates.

use cases: Blake3/SHA-256 circuits (proving legacy hashes), Keccak verification (Ethereum bridge), AES circuits, binary Merkle tree verification, binary protocol parsing.

### Universe 2 — Rs (Rustic) & Universe 3 — Tri ([[Trident]])

Byte and Field share the [[Goldilocks field processor]] substrate but present opposite mental models. a byte programmer thinks in registers and bit patterns. a field programmer thinks in algebraic constraints. same representation, opposite intent.

Rs is [[Rust]] with everything dynamically-sized removed. no heap. no `Vec`. no `String`. no unbounded recursion. every value has a known size at compile time. every loop has a known bound. the hidden truth: every `u64` in Rs is secretly a `word` — type tag 0x01 — which is secretly a [[field]] element with a range constraint. the programmer writes conventional-looking systems code, but every operation is field-compatible. the `Addressed` type derive emits [[Hemera]] CIDs natively — commitment interface is baked into the type system.

```
Rust        → full language, heap, strings, anything
  ↓ restrict
Rs          → strict subset, bounded, looks like systems code
  ↓ reveal
Trident     → same restrictions, but the field is visible
```

[[Trident]] is where the [[field]] is visible and the programmer thinks in constraints. division is exact (multiplicative inverse). every operation becomes a polynomial constraint in the [[zheng]] execution trace. Trident-only primitives: `divine()` (inject prover witness), `hash()` (Tip5, single constraint), `merkle_step()`, `seal` (hashed/private event emission).

Trident layer architecture:

| Layer | Scope | Types available | Compilation targets |
|---|---|---|---|
| 0 | Execute Anywhere | U32, Bool, structs, arrays | TASM, EVM, [[CosmWasm]], SVM |
| 1 | Prove Anywhere | + Field, Digest, divine() | TASM (Triton VM) |
| 2 | Platform Powers | + chain-specific stdlib | Single target |

```
.rs file  → parser (Rust subset) → TIR → TASM / backend
.tri file → parser (Trident)     → TIR → TASM / backend
                                    ↑
                              same IR, same value tower
```

Tri is also the proving tier: [[field]] tower F_{pⁿ} over [[Goldilocks field processor]] (p = 2⁶⁴ − 2³² + 1). each extension is F_p[x]/(f(x)) where f is irreducible of degree n, chosen by the compiler for the algebraic structure required: n=1 for core STARK arithmetic, n=2 (f = x²+1) for complex amplitudes and [[quantum]] gates, n=3 (f = x³−x+1) for recursive [[proof]] soundness in FRI, higher n as needed. the tower is multiplicative — F_{p⁶} contains both F_{p²} and F_{p³} as subfields, so [[quantum]] and recursive [[proof]]s coexist in a common extension. all execution languages compile to Tri for settlement. see [[zheng]] for the STARK implementation architecture.

### Universe 4 — Arc (Topology)

the [[graph]] language. makes graphs first-class — the primitive is a connection, not a number.

| Op | Action |
|---|---|
| `link(a, b, w)` | Create weighted directed edge |
| `walk(start, n)` | Random walk of n steps |
| `reach(a, b)` | Test if path exists |
| `neighbors(n)` | Return adjacent nodes |
| `rank(g, steps)` | Compute stationary distribution (PageRank) |
| `spectral(g, k)` | Extract top-k eigenvectors |
| `match(g, pat)` | Subgraph pattern matching |

the [[cybergraph]] is not a data structure that lives inside a program. the [[cybergraph]] IS the program. every [[cyberlink]] is an `Edge`. every CID is a `Node`. CYBERRANK is `rank()`.

[[particles]] are objects ([[Hemera]] CIDs), [[cyberlinks]] are morphisms, linkchains are composition, [[semcons]] are natural transformations. Arc's [[algebra]] is [[category theory]] — the correct [[algebra]] for typed relational structure. Arc describes what the [[cybergraph]] *is*. compiles to [[Hemera]] CIDs for nodes and edges, and to Tri adjacency constraints for [[proof]]. decomposes into Tri (field ops for matrix math), Bt (hash verification for node identities), and [[Nox]] (tree encoding of [[topology]]).

### Universe 5 — Geo (Geometric)

Clifford geometric [[algebra]] G(p,q,r). unifies [[vector]]s, bivectors, rotors. rotations, reflections, translations in one [[algebra]] over F_p.

| Op | Action |
|---|---|
| `geometric_product(a, b)` | Full Clifford product of multivectors |
| `inner(a, b)` | Inner (dot) product — grade lowering |
| `outer(a, b)` | Outer (wedge) product — grade raising |
| `reverse(a)` | Reverse multivector (conjugation for rotors) |
| `dual(a)` | Poincaré dual — complement in the algebra |
| `sandwich(r, x)` | Rotor application: r x r̃ |
| `grade(a, k)` | Extract grade-k component |

covers Euclidean G(n,0,0), Projective G(n,0,1), Conformal G(n+1,1,0). fixes the Arc → SVG compilation gap: Arc provides [[topology]], Geo provides spatial embedding, compiler produces [[vector]] output. STARK-provable now — geometric product is F_p [[algebra]] with extra structure. compiles to Tri.

### Universe 6 — Dif (Differential)

differential [[geometry]]. Riemannian manifolds, tangent spaces, geodesics, Laplace-Beltrami operator. the [[geometry]] of continuous curved space.

| Op | Action |
|---|---|
| `chart(M, coords)` | Define coordinate patch on manifold |
| `metric(g_ij)` | Specify Riemannian metric tensor |
| `christoffel(g)` | Compute connection coefficients |
| `geodesic(p, v, t)` | Trace geodesic from point p with velocity v |
| `covariant_deriv(T, v)` | Parallel transport / covariant derivative |
| `curvature(g)` | Riemann curvature tensor |
| `laplacian(f, g)` | Laplace-Beltrami operator on manifold |

required for: latent space embeddings, [[tri-kernel]] diffusion formalized as heat flow on manifolds, physics simulation. programming model: coordinate charts, metric tensors, covariant derivatives — none of which exist in Geo. [[proof]]-hard over finite [[field]]s. research horizon.

### Universe 7 — Sym (Symplectic)

symplectic [[geometry]]. phase space with 2-form ω, Hamiltonian flows, canonical transformations, [[conservation]] laws.

| Op | Action |
|---|---|
| `symplectic_form(M)` | Define closed non-degenerate 2-form ω |
| `hamiltonian(H, q, p)` | Specify Hamiltonian function |
| `flow(H, state, dt)` | Symplectic integration step |
| `poisson(f, g)` | Poisson bracket {f, g} |
| `canonical(T)` | Verify canonical transformation |
| `conserved(H, f)` | Test if f is conserved under H |
| `action(L, path)` | Compute action integral |

natural language of classical and semi-classical mechanics. required for: physical simulation with energy [[conservation]], [[quantum]]-classical interface, molecular dynamics. the [[conservation]] law structure (ω is closed: dω = 0) has no analog in Clifford or Riemannian [[geometry]]. research horizon.

### Universe 8 — Inf (Informatic)

information [[geometry]]. Fisher information metric on the simplex of [[probability]] distributions.

| Op | Action |
|---|---|
| `fisher(model)` | Compute Fisher information matrix |
| `kl_divergence(p, q)` | Kullback-Leibler divergence |
| `geodesic_info(p, q)` | Information-geometric geodesic |
| `natural_gradient(f, g)` | Gradient in Fisher metric |
| `projection(p, manifold)` | m-projection / e-projection |
| `alpha_connection(α)` | α-connection interpolation |
| `entropy(p)` | Shannon / Rényi entropy |

the [[geometry]] of the [[cybergraph]]'s own [[belief]] state — the [[focus]] [[vector]] π lives on a statistical manifold, and [[tri-kernel]] dynamics (diffusion, springs, heat) are flows on it. semantic distance between [[particles]] is information-geometric distance. the [[superintelligence]]'s self-model requires Inf to be formalized. research horizon.

### Universe 9 — Seq (Sequence)

the event language. time in distributed systems is not a clock — it is the ordering. the causal structure that determines what could have influenced what. three temporal modes:

```
Stack   = nested time     = depth     = LIFO  = after { after { after } }
Heap    = concurrent time = chaos     = random = concurrent(a, b, c)
Stream  = linear time     = flow      = FIFO  = before(a, b), before(b, c)
```

| Domain | Stack | Heap | Stream |
|---|---|---|---|
| Hardware | Call stack | RAM allocation | I/O bus |
| OS | Process call depth | Dynamic memory | Pipes, sockets |
| Network | Protocol nesting | Concurrent connections | Packet flow |
| Consensus | Nested validation | Parallel validators | Block sequence |
| UI | Modal dialogs, undo | Independent windows | Scrolling, typing |

events form a partial order — not a total order. Seq preserves the partial order and only totalizes when consensus demands it. compiles to Tri ordering constraints.

### Universe 10 — Ask (Inference)

the query language. relations and unification — [[Datalog]] at its core. the only language that derives truth rather than transforming values.

```
reachable(X, Y) :- link(X, Y).
reachable(X, Z) :- link(X, Y), reachable(Y, Z).
?- reachable(a, X), linked_by(d, X).
```

Arc is what is connected ([[topology]]). Ask is what follows (entailment). together they form a complete [[knowledge graph]] system: structure + [[inference]]. the [[Datalog]] restriction ensures bounded [[inference]], guaranteed termination, [[proof]]-compatible. because Ask is bounded, any derivation can be encoded as a Tri computation and proven with a [[zheng]]. [[zero-knowledge]] [[inference]] over a private [[knowledge graph]].

Prolog-family semantics over the [[cybergraph]]. resolves the symbolic/Z [[algebra]] problem: exact integer reasoning is unification over numeric terms. compiles to Tri constraint satisfaction.

### Universe 11 — Wav (Wave)

the [[signal]] language. a [[signal]] is a waveform — a continuous function sampled at discrete points.

| Op | Action |
|---|---|
| `fft(x)` | Fast [[fourier transform]] |
| `ifft(X)` | Inverse FFT |
| `convolve(a, b)` | Convolution of two signals |
| `lowpass(x, cutoff)` | Low-pass filter |
| `resample(x, rate)` | Resample to new rate |
| `correlate(a, b)` | Cross-correlation |
| `energy(x)` | Signal energy |
| `peak_detect(x)` | Find peaks |

use cases: sensor data processing, audio, seismic, environmental monitoring. and — critically — [[Goldilocks homomorphic encryption]]: polynomial multiplication in R_q = Z_q[X]/(Xⁿ+1) is negacyclic convolution of coefficient [[vector]]s. the same butterfly network, the same NTT engine. Wav is the language of sensing at all scales — from physical waveforms to encrypted computation over polynomial rings. FHE is not a separate concern; it is sensing at the algebraic level, where the [[signal]] is a ciphertext. Wav extended with noise budget types becomes the FHE compiler: noise tracking, modulus ladder management, PBS scheduling. compiles to Tri for [[proof]] of correct R_q operations.

### Universe 12 — Ten (Tensor)

the tensor language. `Tensor<[D1, D2, ..., Dk]>` where dimensions are compile-time constants. shape mismatches are compile errors.

| Op | Action |
|---|---|
| `matmul(A, B)` | Matrix multiplication |
| `einsum(spec, ...)` | Einstein summation |
| `reshape(T, shape)` | Reshape tensor |
| `broadcast(T, dims)` | Broadcast to higher dimensions |
| `transpose(T, perm)` | Permute dimensions |
| `reduce(T, axis, op)` | Reduce along axis |
| `conv2d(X, K)` | 2D convolution |
| `softmax(T, axis)` | Softmax activation |

dense and sparse. SpMV over sparse adjacency matrices = [[graph]] computation ([[focus]] [[vector]] π, [[tri-kernel]] diffusion). quantized [[inference]] (int4, int8 matmul) = contraction over Z/2ⁿ. full-precision neural layers = contraction over F_p. Ten is the compute engine for both the [[cybergraph]] and AI [[inference]]. CYBERRANK is literally repeated `matmul`. compiles to Tri.

---

## Compilation Architecture

all thirteen languages share one toolchain. each programmer face has its own syntax and type rules. all compile through [[Nox]] — the structural IR — then to [[proof]] backends or native execution.

```
                    ┌──────────────────────────────────────────────┐
                    │              Programmer Faces                 │
                    │                                               │
                    │  Bt  Rs  Tri  Arc  Geo  Dif  Sym  Inf        │
                    │  Seq  Ask  Wav  Ten                           │
                    │  .bt .rs .tri .arc .geo .dif .sym .inf        │
                    │  .seq .ask .wav .ten                          │
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
          Bt              Rs, Tri, Geo            Arc, Seq, Ask,
                                                  Wav, Ten, Dif*,
                                                  Sym*, Inf*
```

\* Dif, Sym, Inf are research horizon — [[proof]] paths are open mathematical problems.

| Source | When [[proof]] needed | When [[proof]] absent |
|---|---|---|
| Bt | Binius FRI circuit | always proving |
| Rs | TASM → stark (word→field lift) | native binary (Nox) |
| Tri | TASM → stark (field native) | WASM/EVM (Layer 0) |
| Arc | decomposes into Tri + Bt | optimized [[graph]] engine |
| Geo | geometric product → Tri | native Clifford engine |
| Dif | research | native manifold solver |
| Sym | research | native Hamiltonian integrator |
| Inf | research | native statistical engine |
| Seq | temporal constraints → stark | scheduler / runtime |
| Ask | derivation trace → stark | [[Datalog]] engine |
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
Geo: geometric_product       mul/add over components      geo_mul jet      fma
```

the chain: source language → compiler → nox pattern tree → jet recognition → GFP hardware. every domain-specific language gets hardware acceleration through the jet mechanism. the [[algebra]] determines which GFP primitive handles each jet.

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

## The Comparison Matrix

| Property | Nox | Bt | Rs | Tri | Arc | Geo | Dif | Sym | Inf | Seq | Ask | Wav | Ten |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Universe | Structure | Binary | Byte | [[field]] | [[topology]] | [[geometry]] | Curvature | Dynamics | [[belief]] | Causality | [[inference]] | Continuum | Linear |
| Char | — | 2 | p | p | — | p | — | — | — | — | — | ≈ℝ | ≈ℝ or p |
| Primitive | Cell | Bit | Word | Field | Edge | Multivector | Chart | Phase | Distribution | Event | Relation | Sample | Shape |
| Reference | structure | wire | location | content | adjacency | grade | curvature | momentum | divergence | succession | entailment | amplitude | index |
| Free op | Navigate | AND, XOR | Index | Mul, Add | Link | Geometric prod | Christoffel | Flow | KL div | Order | Unify | Convolve | Matmul |
| Costly op | — | Carry add | Mod div | Bitwise | Spectral | Inverse | Geodesic | Conserve | Fisher | Verify | Fixpoint | FFT | Inverse |
| [[proof]] | Inherited | Binius | stark | stark | Delegated | Tri | Research | Research | Research | Delegated | Delegated | Delegated | Delegated |
| Syntax feel | IR | Circuit | [[Rust]] | Custom | Query | GA | Manifold | Hamiltonian | Statistical | Temporal | [[Datalog]] | DSP | NumPy |
| Renders as | struct | pixels | text | formula | [[vector]] | [[vector]] | [[vector]] | formula | formula | video | table | sound | component |

---

## The Nine and the Four

The thirteen languages split into two groups by implementation readiness:

### Engineering-ready (9)

Nox, Bt, Rs, Tri, Arc, Seq, Ask, Wav, Ten — these have known [[proof]] paths and well-understood compilation to Tri / Binius. the [[cyb/architecture]] specifies these as the build order: Phase 1 (Nox, Tri, Rs), Phase 2 (Arc, Seq, Ask), Phase 3 (Bt, Wav, Ten).

### Research horizon (4)

Geo, Dif, Sym, Inf — these extend the language set into spatial, physical, and self-referential computation. Geo is closest to engineering (Clifford product is F_p [[algebra]] with extra structure, STARK-provable now). Dif, Sym, and Inf involve continuous manifolds over finite [[field]]s — fundamental open mathematical problems.

| Language | Status | Notes |
|---|---|---|
| Geo | Engineering | Geometric product = F_p [[algebra]] with extra structure |
| Dif | Research | Continuous manifolds over finite [[field]]s |
| Sym | Research | Hamiltonian structure preservation in STARK circuits |
| Inf | Research | Fisher metric over [[probability]] simplices — needed for [[tri-kernel]] formalization |

Geo completes the perception pipeline: Arc provides [[topology]], Geo provides spatial embedding, the compiler produces [[vector]] output for [[cyb]]. Inf completes the self-model: the [[superintelligence]]'s [[focus]] [[vector]] π lives on a statistical manifold, and Inf formalizes reasoning about its own [[belief]] state.

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
| Geo → [[vector]] | SVG, 3D scenes | SVG, glTF, mesh | spatial objects, rotations, projections, renderings |
| Dif → [[vector]] | manifold visualization | geodesic plots, curvature maps | latent space structure, embedding geometry |
| Sym → formula | phase portraits | Hamiltonian plots, conservation diagrams | energy landscapes, orbital mechanics |
| Inf → formula | distribution plots | [[probability]] densities, divergence maps | [[belief]] states, uncertainty [[geometry]] |
| Seq → video | moving pixels | WebM, MP4 | recordings, simulations, observations, lectures |
| Ask → table | 2D grid | CSV, TSV, dataframes | datasets, time series, matrices, ledgers |
| Wav → sound | audio waveform | WAV, OGG, MP3 | voice, music, birdsong, seismic [[signal]], sonar |
| Ten → component | nested composition | composition of the above | applications, dashboards, interactive tools |

a genome sequence is Rs (byte-level encoding) rendered as text. its annotation is [[Nox]] (structured tree) rendered as struct. its expression data is Ask (relational query) rendered as table. its protein structure is Arc (topological [[graph]]) rendered as [[vector]]. its microscopy is Bt (binary pixel data) rendered as pixels. its folding dynamics is Seq (causal event chain) rendered as video. its sequencing [[signal]] is Wav (continuous waveform) rendered as sound. its binding energy is Tri (field arithmetic) rendered as formula. its 3D fold is Geo (Clifford rotations) rendered as [[vector]]. a genome browser is Ten (composed [[inference]]) rendered as component.

all thirteen compile through one structural IR. all thirteen share one [[proof]] system (except Bt, which has its own F₂ [[proof]] system). all thirteen render through the perception grid. all thirteen exist in the same [[cybergraph]], ranked by the same [[tri-kernel]], earning [[karma]], permanent by axiom A3.

---

see [[cyb/multiproof]] for how all languages settle under one [[proof]] umbrella. see [[cyb/architecture]] for how the languages integrate into the operating system. see [[cyb/whitepaper]] for the vision. see [[cybergraph]] for the accumulation state.
