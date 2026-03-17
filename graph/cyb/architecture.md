---
tags: cyb, article, research, core
alias: cyb architecture, cyb os, cybos, cyber-os-architecture, cyb-system-architecture, cyb/os
icon: "\U0001F310"
crystal-type: entity
crystal-domain: cyber
crystal-size: deep
stake: 14015797676239542
---
# Architecture

[[cyb]] is a sovereign browser that becomes an operating system. identity is a keypair, state lives on-chain, [[smart contracts]] run locally, and the entire render stack compiles to GPU. one binary, all platforms, 130K lines of [[Rust]], no WebView, no V8, no Google.

cyb/os is a stack of typed universes — thirteen computation [[cyb/languages]] compiled through one structural IR, rendered through nine perception primitives, driven by ten decision primitives — all sharing one toolchain, one tree substrate, and one [[proof]] system. see [[cyb/languages]] for the algebraic completeness argument and [[cyb/multiproof]] for the proving design.

core stack: [[radio]] for data publishing, [[cyber]] for search and [[learning]], [[rune]] for orchestration (the dynamic scripting layer above the [[proof]] boundary that glues the thirteen computation languages to human intent), [[CozoDB]] graph storage, [[cosmos-sdk]] chains via [[IBC]]. builds for [[web]], [[desktop]], [[mobile]].

---

## Part I: The Three Grids

the operating system is the membrane between three grids

```
COMPUTATION (what the machine thinks)     PERCEPTION (what the human sees)
─────────────────────────────────         ────────────────────────────────
Nox    → trees                            struct    → collapsible tree
Bt     → bits                             pixels    → raster image
Rs     → words                            text      → prose, code
Trident→ fields                           formula   → math notation
Arc    → graphs                           vector    → SVG, paths, curves
Seq    → events                           video     → moving pixels
Inf    → relations                        table     → 2D grid
Wav    → signals                          sound     → audio waveform
Ten    → tensors                          component → nested composition

DECISION (what the human does)
──────────────────────────────
observe   → gather without choosing
filter    → narrow by criteria
select    → choose one from many
rank      → order by preference
compose   → build a new value
split     → one becomes many
merge     → many become one
delegate  → route to another agent
reject    → explicitly not-choose
confirm   → irreversible commit
```

every computation type has a canonical rendering. a tree computed in [[Nox]] naturally displays as a collapsible `struct`. a graph traversed in [[Arc]] naturally draws as `vector` paths. a relation queried in [[Inf]] naturally fills a `table`. a signal processed in [[Wav]] naturally plays as `sound`. the mapping is many-to-many, but the canonical pairing is the path of least impedance — where the shape of the data matches the shape of the display.

every rendering invites a decision. the human responds with typed decision primitives — select, rank, compose, confirm — each with its own algebra, its own temporal mode, and its own relationship to the computation and perception grids.

### 1. Thirteen Computation Languages

every language has a short name (2-3 letters, used in code) and a long name (used in prose):

```
Universe          Short  Long           Type        Algebra           Purpose
──────────────────────────────────────────────────────────────────────────────
Structure         Nox    Nox            Tree        Combinators       Composition
Binary            Bt     Bitwise        Bit         𝔽₂ tower          Circuits
Byte              Rs     Rustic         Word        Bitwise on 𝔽ₚ     Systems
Field             Tri    Trident        Field       Arithmetic on 𝔽ₚ  Proofs
Topology          Arc    Arc            Graph       Adjacency         Knowledge
Geometry          Ren    Render      Shape       G(p,q,r)          Space
Curvature         Dif    Differential   Manifold    (M, g)            Meaning
Dynamics          Sym    Symplectic     Phase       (M, ω), dω = 0   Physics
Belief            Bel    Belief         Distrib.    g on Δⁿ           Self-model
Causality         Seq    Sequence       Event       Partial order     Ordering
Inference         Inf    Infer          Relation    Unification       Reasoning
Continuum         Wav    Wave           Signal      Convolution       Sensing
Linear            Ten    Tensor         Tensor      Contraction       Learning
```

a data type deserves its own language when its algebraic laws are so different from other types that forcing it into a foreign language creates constant impedance mismatch. thirteen fundamental types pass this test. each inhabits a universe defined by its characteristic algebraic structure. some universes share a proof system. some share a compiler. none share semantics. see [[cyb/languages]] for the full completeness argument and irreducibility proof.

### 2. The Value Tower — Three Modes of Reference

Byte and Field share the same mathematical substrate — the [[Goldilocks field]] 𝔽ₚ where p = 2⁶⁴ − 2³² + 1. this substrate provides three atom types sufficient for eleven of the thirteen universes.

| Tag  | Name    | Representation      | Valid Range   | Use        |
|------|---------|---------------------|---------------|------------|
| 0x00 | `field` | Single 𝔽ₚ element   | [0, p)        | Arithmetic |
| 0x01 | `word`  | Single 𝔽ₚ element   | [0, 2⁶⁴)     | Bitwise    |
| 0x02 | `hash`  | 4 × 𝔽ₚ elements     | 256-bit digest| Identity   |

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
```

three atoms are complete — for one characteristic. the single exception is Bt (Bitwise): a bit is genuinely not an element of 𝔽ₚ. it lives in 𝔽₂ — different characteristic, different algebra. that is exactly why Bt has a separate proof system, not just a new type tag.

```
Nox value tower (3 atoms: field, word, hash)
  sufficient for: Rs, Tri, Arc, Ren, Dif, Sym, Bel, Seq, Inf, Wav, Ten
  NOT sufficient for: Bt

Bt value tower (separate, 𝔽₂)
  sufficient for: Bt only
```

### 3. The Thirteen Languages

each language has its own page with ops tables, use cases, and proof paths:

| # | Universe | Short | Long | Page |
|---|---|---|---|---|
| 0 | Structure | Nox | Nox | [[Nox]] |
| 1 | Binary | Bt | Bitwise | [[Bt]] |
| 2 | Byte | Rs | Rustic | [[Rs]] |
| 3 | Field | Tri | Trident | [[Trident]] |
| 4 | Topology | Arc | Arc | [[Arc]] |
| 5 | Geometry | Ren | Render | [[Ren]] |
| 6 | Curvature | Dif | Differential | [[Dif]] |
| 7 | Dynamics | Sym | Symplectic | [[Sym]] |
| 8 | Belief | Bel | Belief | [[Bel]] |
| 9 | Causality | Seq | Sequence | [[Seq]] |
| 10 | Inference | Inf | Infer | [[Inf]] |
| 11 | Continuum | Wav | Wave | [[Wav]] |
| 12 | Linear | Ten | Tensor | [[Ten]] |

see [[cyb/languages]] for the completeness argument, value tower, algebra coverage, and perception mapping. see [[cyb/multiproof]] for how all thirteen settle under one proving umbrella

### 4. Compilation Architecture

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

\* Dif, Sym, Bel are research horizon — proof paths are open mathematical problems.

| Source  | When proof needed                | When proof absent             |
|---------|----------------------------------|-------------------------------|
| Bt      | Binius FRI circuit               | always proving                |
| Rs      | TASM → stark (word→field lift)   | native binary (Nox)           |
| Tri     | TASM → stark (field native)      | WASM/EVM (Layer 0)            |
| Arc     | decomposes into Tri              | optimized graph engine        |
| Ren    | geometric product → Tri          | native Clifford engine        |
| Dif     | research                         | native manifold solver        |
| Sym     | research                         | native Hamiltonian integrator |
| Bel     | research                         | native statistical engine     |
| Seq     | temporal constraints → stark     | scheduler / runtime           |
| Inf     | derivation trace → stark         | Datalog engine                |
| Wav     | decomposes into Tri              | native DSP pipeline           |
| Ten     | decomposes into Tri              | native BLAS / GPU             |

see [[cyb/multiproof]] for how all thirteen languages settle under one proving umbrella via [[Hemera]] and Tri.

### 5. Nine Perception Primitives

the irreducible visual types — the atoms of everything a human can perceive through a screen and speakers. any UI, any document, any application is a composition of these nine. the four new computation languages (Ren, Dif, Sym, Bel) render through existing perception primitives: Ren → vector, Dif → vector, Sym → formula, Bel → formula.

| Primitive   | What it is                    | GPU mapping                          |
|-------------|-------------------------------|--------------------------------------|
| `text`      | Markdown, prose, code         | Glyphs via compute shader            |
| `struct`    | JSON, TOML — trees & configs  | Collapsible tree of text glyphs      |
| `table`     | 2D data, CSV                  | Grid of text cells, virtualized rows |
| `vector`    | SVG, paths, Bezier curves     | Path rasterization via Vello         |
| `pixels`    | Raster image                  | Texture upload, GPU sampler          |
| `video`     | Moving pixels                 | Hardware decode, texture per frame   |
| `sound`     | Waveform, audio stream        | Audio pipeline (visual: waveform shader) |
| `formula`   | LaTeX / MathML                | Glyph layout + vector curves via Vello |
| `component` | Composition of primitives     | Nested render pass                   |

`component` is to perception what `Nox` is to computation. Nox composes computations (cons, axis, branch). component composes renderings (nest, layout, pass).

### 6. Ten Decision Primitives

every human interaction with a computer is a decision. strip the physics away — what remains is pure decision structure.

| #  | Primitive | Action                | Reversible? | Time Mode | Comp Language | Perception |
|----|-----------|-----------------------|-------------|-----------|---------------|------------|
| 1  | observe   | Gather without choosing | Always    | Stream    | Wav           | any        |
| 2  | filter    | Narrow by criteria    | Yes         | Stack     | Inf           | struct     |
| 3  | select    | Choose one            | Yes         | Stream    | Inf           | table      |
| 4  | rank      | Order by preference   | Yes         | Stream    | Ten           | table      |
| 5  | compose   | Build new value       | Yes         | Stack     | Rs            | text/vector|
| 6  | split     | One becomes many      | Depends     | Heap      | Arc           | vector     |
| 7  | merge     | Many become one       | Depends     | Heap      | Arc + Inf     | vector     |
| 8  | delegate  | Route to agent        | Sometimes   | Heap      | Arc           | vector     |
| 9  | reject    | Explicitly not-choose | Mostly      | Stream    | Seq           | video      |
| 10 | confirm   | Irreversible commit   | Never       | Stack     | Trident       | formula    |

the machine computes, the human decides. computation produces options. perception displays them. decision collapses them to action. the action commits to new state, and the cycle continues.

confirm is the only primitive that is always irreversible. it is structurally unique — the moment where possibility collapses into fact. every other primitive can be undone, revised, or abandoned.

### 7. Cross-Grid Connections

the three grids interlock in a continuous decision loop — the cyb/os event loop:

```
loop {
  state   = nox_tree(current)           // authenticated tree
  options = compute(state)              // some universe produces alternatives
  display = render(options)             // canonical primitive shows them
  choice  = decide(human_input)         // decision primitive applied
  proof   = commit(choice, state)       // irreversible, potentially stark-proven
  state   = update(state, choice, proof)// new tree root
}
```

all three grids share one universal structural pair — fork and join:

```
            fork (one → many)          join (many → one)
            ─────────────────          ─────────────────
Computation  axis (decompose tree)      cons (build pair)
Perception   expand (drill into view)   nest (compose views)
Decision     split (divide choice)      merge (combine choices)
```

fork is how structure grows. join is how consensus forms. the same skeleton wearing three costumes.

### 8. The Comparison Matrix

| Property | Nox | Bt | Rs | Tri | Arc | Ren| Dif | Sym | Bel | Seq | Inf | Wav | Ten |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Universe | Structure | Binary | Byte | Field | Topology | Geometry | Curvature | Dynamics | Belief | Causality | Inference | Continuum | Linear |
| Char | — | 2 | p | p | — | p | — | — | — | — | — | ≈ℝ | ≈ℝ or p |
| Primitive | Cell | Bit | Word | Field | Edge | Multivector | Chart | Phase | Distribution | Event | Relation | Sample | Shape |
| Reference | structure | wire | location | content | adjacency | grade | curvature | momentum | divergence | succession | entailment | amplitude | index |
| Free op | Navigate | AND, XOR | Index | Mul, Add | Link | Geometric prod | Christoffel | Flow | KL div | Order | Unify | Convolve | Matmul |
| Costly op | — | Carry add | Mod div | Bitwise | Spectral | Inverse | Geodesic | Conserve | Fisher | Verify | Fixpoint | FFT | Inverse |
| Proof | Inherited | Binius | stark | stark | Delegated | Tri | Research | Research | Research | Delegated | Delegated | Delegated | Delegated |
| Syntax feel | IR | Circuit | Rust | Custom | Query | GA | Manifold | Hamiltonian | Statistical | Temporal | Datalog | DSP | NumPy |
| Renders as | struct | pixels | text | formula | vector | vector | vector | formula | formula | video | table | sound | component |

---

## Part II: Render Stack — PureRender

### 9. Flat Streams Instead of Trees

DOM is a tree. trees breed cascading complexity: reflow, repaint, layout thrashing, z-index stacking contexts. cyb replaces the tree with a stream.

```
stream Article {
  text    { # Hello World }
  pixels  { hero.webp }
  text    { Main body of the article... }
  formula { E = mc^2 }
  table   { metrics.csv }
  vector  { diagram.svg }
  action  { Subscribe -> subscribe() }
}
```

a stream is a sequence of primitives, top to bottom. each block knows its own size. layout is trivial. scrolling is an offset. virtualization is free — only render the visible slice. two levels of nesting maximum:

```
grid [sidebar: 300px, main: 1fr] {
  stream sidebar { vector { logo.svg }  struct { nav.toml } }
  stream main    { text { # Dashboard } table { sales.csv } }
}
```

same primitives, different layout mode. screen, PDF, print — same component, same pipeline, different output target.

### 10. Component as a Single Unit

one file. one scope. everything inside.

```
component Dashboard {
  state metrics: Table = load("metrics.csv")
  state filter: Choice = { options: ["day", "week", "month"], selected: "week" }

  <stream>
    <text>{ # Metrics for {filter.selected} }</text>
    <input choice bind={filter} />
    <table data={metrics} filter={filter.selected} />
    <Canvas2D ref={chart} />
    <svg><path d={trendLine(metrics)} stroke="blue" /></svg>
    <formula>{ \sum_{i=1}^{n} x_i }</formula>
  </stream>

  style {
    table { border: 1px solid gray }
    path  { stroke-width: 2 }
  }

  fn trendLine(data: Table): string { ... }
  fn render(chart: Canvas2D) { ... }
}
```

the compiler sees the entire component. dead CSS eliminated. static primitives computed at compile time. reactivity only where `state` exists.

### 11. Compilation: Everything to WASM + WGSL

no interpreter. no JIT. no runtime parsing. everything compiles.

strict TypeScript: full type system, async/await, generics, interfaces, enums, unions, modules, pattern matching, destructuring. no `any`, no `eval()`, no `arguments`, no prototype chains, no `this` binding, no runtime type checks.

```
Source (TS strict + HTML + CSS + SVG + LaTeX)
          |
    Parse + validate + type check
          |
    Unified IR
          |
    Optimize (DCE, constant fold, static layout, shader compile)
          |
   +------+------+
  WASM          WGSL
  logic         render
   |              |
   +------+------+
          |
     WGPU runtime
          |
  Vulkan / Metal / DX12 / OpenGL ES
```

### 12. GPU Render: Everything is Shaders

every primitive is a draw call or compute dispatch:

```
text        ->  rustybuzz shaping -> swash raster -> GPU glyph atlas
struct      ->  text render + collapse/expand + indent
table       ->  grid compute -> text per cell (virtualized)
vector      ->  Vello: path -> tiles -> compute fill
pixels      ->  texture upload -> sampler -> fragment shader
video       ->  hardware decode -> texture per frame
sound       ->  audio pipeline (visual: waveform compute)
formula     ->  LaTeX parse -> glyph positioning -> Vello paths
action      ->  hit-test region + feedback shader
input *     ->  cursor shader + selection + validation
```

frame loop:

```
every frame (16ms @ 60fps):
  1. WASM: process events, update state
  2. WASM: recompute layout for changed subtrees only
  3. WASM: emit flat draw command array
  4. WGSL: compute pass — text, SVG, formulas, backgrounds
  5. WGSL: render pass — composite into framebuffer
  6. present
```

---

## Part III: Execution Engine — CosmWasm Native

### 13. Smart Contracts in the Browser

in Chrome, a dApp is a JS app that talks to a blockchain node over HTTP. in cyb, [[smart contracts]] run locally in the same runtime as UI.

```
Chrome:     JS -> fetch() -> REST API -> Go node -> CosmWasm -> result
            Round trip: 200-2000ms

Cyb:        Component (wasmi) -> direct call -> Contract (wasmi) -> result
            Round trip: <1ms
```

the component IS the contract:

```
component/contract Token {
  state balances: Map<Address, u128>

  <stream>
    <text>{ # Balance: {balances[viewer]} }</text>
    <input text bind={recipient} />
    <input range bind={amount} max={balances[viewer]} />
    <action>{ Send -> transfer(recipient, amount) }</action>
  </stream>

  fn transfer(to: Address, amount: u128) {
    // executes in CosmWasm sandbox
    // simulates locally, signs and broadcasts when online
  }
}
```

no separation between frontend and on-chain logic. one file, one scope, one runtime. UI and state machine unified. gas metering: wasmi counts fuel per instruction. every WASM module — UI, contract, plugin — is metered.

---

## Part IV: Kernel Architecture — CybOS

### 14. Design Axioms

1. no Unix legacy. no files, no processes, no users, no fork/exec, no POSIX. cyb abstractions are native to its domain: agents, [[cyberlinks]], ranks, epochs, bandwidth.
2. zero unsafe [[Rust]]. the entire OS — kernel, drivers, [[consensus]], storage — compiles without a single `unsafe` block. memory safety is a compiler-verified property.
3. bounded liveness everywhere. no operation can block indefinitely. no module can starve another. every async future has a compile-time deadline. the system degrades gracefully, never halts.
4. neural drivers. hardware support generated by models against stable trait contracts, verified by the compiler, validated by conformance test suites.
5. single address space. no user/kernel split. no syscalls. no TLB flushes. isolation enforced by [[Rust]] ownership, not hardware privilege levels.

### 15. Layered Design

```
┌──────────────────────────────────────────────────────┐
│                      CybOS                            │
│  ┌────────────────────────────────────────────────┐  │
│  │              Application Cells                  │  │
│  │  Consensus · Graph · Rank · Bandwidth · Query   │  │
│  │  (100% safe Rust, hot-swappable via governance) │  │
│  ├────────────────────────────────────────────────┤  │
│  │           Async Bounded Runtime                 │  │
│  │  Epoch budget allocator · Wait-free channels    │  │
│  │  Heartbeat monitor · Degraded mode manager      │  │
│  ├────────────────────────────────────────────────┤  │
│  │              HAL Trait Layer                     │  │
│  │  BlockDevice · NetDevice · Iommu · IRQ · Timer  │  │
│  │  (~3K lines, the entire hardware contract)      │  │
│  ├────────────────────────────────────────────────┤  │
│  │           MMIO Foundation                       │  │
│  │  Compiler-integrated register access            │  │
│  │  Zero unsafe — MMIO as language primitive        │  │
│  ├────────────────────────────────────────────────┤  │
│  │        Neural Driver Harnesses                  │  │
│  │  model-generated, compiler-verified per-platform  │  │
│  └────────────────────────────────────────────────┘  │
│                       │                              │
│                  ┌────┴────┐                          │
│                  │Hardware │                          │
│                  └─────────┘                          │
└──────────────────────────────────────────────────────┘
```

### 16. No Processes — Cells

cells replace processes: independently compiled [[Rust]] crates that can be loaded, unloaded, and hot-swapped at runtime without stopping the system. each cell has explicit dependency declarations, typed bounded wait-free channels, exclusive state ownership, mandatory heartbeat reporting. cell lifecycle is governed by on-chain governance.

| Missing Cell | System Behavior |
|-------------|-----------------|
| Rank | Validates blocks, does not answer rank queries |
| Consensus | Becomes full node (follows chain, does not vote) |
| Query | Participates in consensus, does not serve clients |
| Gossip | Works with local state only (island mode) |
| Storage | Emergency halt, preserves last state |

### 17. No File System — the Big Badass Graph

no hierarchical file system. no paths, no inodes, no directories. all persistent data lives in one structure: the BBG — a content-addressed [[knowledge graph]] that subsumes every storage layer. the graph is not a feature of the protocol — the graph IS the protocol.

three primitives build the entire graph: [[particles]] (content-addressed nodes — identity = [[Hemera]] hash of content, 64 raw bytes, one hash function, one address space), [[cyberlinks]] (signed 7-tuple edges — $(\nu, p, q, \tau, a, v, t)$ carrying [[subject]], source, target, [[token]], amount, [[valence]], timestamp), and [[neurons]] (agents who link — identity = hash of public key). a [[particle]] exists iff at least one cyberlink touches it. a naked hash with no links never enters the graph.

the [[cybergraph]] $\mathbb{G} = (P, N, L)$ is a directed authenticated multigraph satisfying six axioms: content-addressing (A1), authentication (A2), append-only growth (A3), entry by linking (A4), [[focus]] conservation (A5), homoiconicity (A6 — the hash of a cyberlink is itself a particle). see [[cybergraph]] for the formal specification.

every [[cyberlink]] is simultaneously a [[learning]] act and an economic commitment — a card that is immutable, unique, transferable, and yield-bearing. conviction $(\tau, a)$ is a [[UTXO]]: creating a link moves tokens from wallet to edge. cheap talk produces noise. costly links produce [[knowledge]].

the [[tru]] reads the graph every block and computes [[cyberank]] per particle (probability of being observed by a [[random walk]]ing neuron), [[karma]] per neuron, [[syntropy]] of the whole — the KL divergence of [[focus]] from uniform, measuring how far collective attention has organized beyond noise. the [[tri-kernel]] integrates three operators: [[diffusion]] (where probability flows), [[springs]] (what satisfies structural constraints), [[heat]] (what the graph looks like at resolution $\tau$). convergence guaranteed by the [[collective focus theorem]].

the BBG maintains six [[NMT]] indexes over the same data:

| index | namespace | proves |
|-------|-----------|--------|
| by_neuron | neuron_id | all edges created by a [[neuron]] |
| by_particle | particle_hash | all edges touching a [[particle]] |
| [[focus]] | neuron_id | current focus value per neuron |
| balance | neuron_id | current balance per neuron |
| coins | denom_hash | fungible [[token]] supply |
| cards | card_id | non-fungible knowledge assets |

the graph serves as infrastructure for itself:

| function | how |
|----------|-----|
| identity | [[Hemera]] hash = address, graph = PKI |
| key exchange | CSIDH curves as [[particles]], non-interactive |
| consensus | finalized subgraph IS the canonical state |
| fork choice | $\pi$ from graph topology |
| finality | $\pi_i > \tau$, threshold adapts to graph density |
| incentives | $\Delta\pi$ from convergence = reward signal |
| proof archive | [[stark]] proofs published as particles |
| version control | patches = [[cyberlinks]], repos = subgraphs |
| file system | `~neuron/path` resolves through cyberlinks |
| data availability | [[NMT]] per row, erasure-coded, namespace-aware sampling |

[[radio]] is the transport layer — a fork of [[iroh]] where every hash runs through [[Hemera]] instead of Blake3. 20× cheaper in [[stark]] proofs, one hash function end to end. content shared via verified [[Hemera]] Merkle trees. the [[brain]] is the graph file manager — discovery, linking, querying through [[CozoDB]] and [[datalog]].

querying storage and querying knowledge are the same operation: graph traversal. the BBG is the single source of truth.

### 18. No Users — the Avatar System

no usernames, no passwords, no accounts. identity is a public key ([[neuron]]). access control = bandwidth allocation. the [[cybergraph]] is public. bandwidth is the only scarce resource.

the fundamental identity unit is the [[avatar]] — a collection of [[neurons]] under one [[name]]. an avatar is both subject and object: it acts in the graph and is a particle in the graph.

key derivation follows a four-level hierarchy:

`m / avatar' / neuron' / particle' / invoice'`

| Level | Purpose |
|-------|---------|
| avatar | broad identity — personal, business, project |
| neuron | device or context — mobile, desktop, contract |
| particle | application-specific key exposure |
| invoice | unique identifier for incoming payments |

all levels are hardened. compromise of one neuron reveals nothing about siblings or parent. the [[signer]] handles two operations: sign and verify. transaction construction stays outside the signer — it receives formed data and returns a signature as a [[particle]].

the signer is universal: pluggable signature schemes (ECDSA, Schnorr, BLS), pluggable curves (secp256k1, sr25519, ed25519, bls12-381), pluggable derivation paths, pluggable dictionaries. this makes the same avatar system work across every network in the [[hub]].

---

## Part V: Radio — Transport, Messaging, Sync, Storage

[[radio]] is the connectivity layer of [[cyb]] — a fork of [[iroh]] where every hash runs through [[Hemera]] instead of Blake3. one hash function, one address space, zero self-describing overhead. 20× cheaper in [[stark]] proofs than Blake3, at the cost of ~3× lower raw throughput. the tradeoff is correct: [[particle]] addresses are verified far more often than they are created.

### 19. Endpoint and Physical Transport

the [[radio/endpoint]] is the entry point for all networking. it wraps a QUIC socket, an Ed25519 keypair identity, a [[radio/relay]] connection, and [[radio/discovery]] services into a single handle. connections are made by cryptographic PublicKey — dial keys, not IP addresses.

three transport modes over QUIC: bidirectional streams (request-response), unidirectional streams (one-way data), datagrams (low-latency fire-and-forget). all traffic encrypted end-to-end with keys derived through [[Hemera]].

[[radio/hole-punching]] establishes direct P2P paths through NAT using STUN-over-QUIC for address discovery and ICE-over-QUIC for candidate exchange. when direct connection fails, [[radio/relay]] provides encrypted fallback — relays forward traffic without decoding it. relays earn [[focus]] for proven delivery via [[stark]] proof chains.

### 20. Blob Transfer and Verified Streaming

every [[particle]] in the [[cybergraph]] is a [[radio/blob]] — content-addressed binary data of any size. the blob's address is its 64-byte [[Hemera]] hash.

[[radio/bao]] enables verified streaming: a binary [[Hemera]] Merkle tree over 4 KiB chunks. download any byte range of a 10 GB blob and verify it against the root hash — the proof is logarithmic (a few KB covers any range of any blob). interrupted transfers resume from the last verified chunk. [[radio]] never trusts — it verifies every chunk cryptographically during streaming.

### 21. Gossip — Real-Time Graph Propagation

[[radio/gossip]] is topic-based publish/subscribe over epidemic broadcast trees. when a [[neuron]] creates a [[cyberlink]], it broadcasts to the relevant topic. subscribers update their local view of the [[cybergraph]] without polling.

built on HyParView (hybrid partial view peer sampling) + PlumTree (broadcast tree on random peer graph). eager push along tree edges, lazy push on remaining links — reliability of flooding with efficiency of tree routing. self-healing when peers leave or join.

### 22. Private Messaging

private messaging between [[neurons]] with cryptographic [[proof of delivery]]. see [[cyber/communication]] for the full specification.

shared secret via CSIDH: two neurons that have never communicated derive a shared key from public data. each publishes its CSIDH public curve as a [[particle]] in the [[cybergraph]]. both independently compute the same key — the graph itself is the key exchange medium. no handshake, no prekeys, no certificate authority.

```
A publishes E_a = [a] · E₀ as a particle
B publishes E_b = [b] · E₀ as a particle

A computes: K = Hemera([a] · E_b)
B computes: K = Hemera([b] · E_a)

K_A = K_B     by CSIDH commutativity
```

message format: header (ephemeral curve point for forward secrecy, sequence number, hop count) + payload (AES-256-GCM ciphertext, [[Hemera]] MAC) + onion-encrypted routing (next hop, return path).

onion routing: sender wraps the message in layers of encryption, one per relay hop. each relay peels one layer, learns only the next hop, forwards the inner blob. no relay sees the full route or the plaintext.

proof of delivery: each hop produces a [[stark]] proof — "I received a valid blob, peeled my layer correctly, forwarded to the next address." proofs chain recursively into a single ~100-200 KB proof covering the entire route, regardless of hop count. the sender publishes π_chain as a [[particle]]. anyone can verify delivery happened. no one can read the message or learn the route.

```
PUBLIC                              │ HIDDEN
────────────────────────────────────┼──────────────────────────
message was delivered               │ sender identity
route had N hops                    │ recipient identity
all relays forwarded correctly      │ relay identities
MAC verification succeeded          │ message content
```

relays earn [[focus]] for proven delivery. no proof, no payment. this creates a permissionless relay network where operators are compensated for bandwidth.

| property | Signal | Tor | cyber |
|----------|--------|-----|-------|
| E2E encryption | yes | yes | yes (CSIDH + AES-256-GCM) |
| metadata privacy | partial | yes | yes (onion routing) |
| delivery proof | trust-based | no | yes ([[stark]] chain) |
| post-quantum | partial | no | yes (CSIDH + Hemera) |
| non-interactive key exchange | no (prekeys) | no (circuit) | yes (CSIDH from graph) |
| relay incentive | none | volunteer | [[focus]] for proven delivery |

### 23. Sync — Collaborative Knowledge

two sync protocols for different access models:

[[radio/docs]] — eventually-consistent multi-dimensional key-value documents. a replica identified by a NamespaceId (public key). multiple [[neurons]] write under their own AuthorId. range-based set reconciliation — only changed entries sync. depends on [[radio/blob]] for content storage and [[radio/gossip]] for change notification. this is the substrate for shared [[cybergraph]] partitions.

[[radio/willow]] — confidential sync via the Willow protocol with Meadowcap capability-based access control. encrypted reconciliation — peers sync without revealing what they have to unauthorized parties. capabilities are delegatable: a holder creates restricted sub-capabilities with narrower scope. willow enables private [[cybergraph]] partitions where access is controlled by capability certificates rather than public broadcast.

| model | protocol | access control | use case |
|-------|----------|----------------|----------|
| open collaboration | [[radio/docs]] | namespace key = write authority | shared research, public knowledge |
| confidential | [[radio/willow]] | Meadowcap capability certificates | private graphs, restricted data |

### 24. Storage Proofs — Graph Survival

without [[storage proofs]], content-addressed identity is fragile — a hash with lost content is a dead [[particle]]. at planetary scale ($10^{15}$ particles), content loss is the existential risk. storage proofs are Phase 1 security infrastructure — operational before genesis.

six proof types:

| proof | guarantees | mechanism |
|-------|-----------|-----------|
| storage | content bytes exist on specific node | periodic [[Hemera]] Merkle challenge-response |
| size | claimed size matches actual bytes | tree depth + leaf count commitment |
| replication | k ≥ 3 independent copies exist | challenge k distinct nodes, verify uniqueness |
| retrievability | content fetchable within bounded time | timed challenge-response with latency bound |
| data availability (DAS) | block data published and accessible | 2D Reed-Solomon over [[Goldilocks field]] + [[NMT]] sampling |
| encoding fraud | erasure coding done correctly | k+1 cells → decode → compare against [[NMT]] root |

data tiered by criticality:

```
Tier 0 — critical roots: checkpoint to settlement layer, ~32-64 KB/epoch, permanent
Tier 1 — active graph:  focus blobs to DA layer, ≥30 day retention, light sampling
Tier 2 — historical:    erasure-coded archival, refreshed by archivers
```

storage proofs also enable [[Hemera]] hash migration — if Hemera is ever broken, storage proofs guarantee content availability for full graph rehash under a new primitive. at $10^{15}$ particles across $10^6$ nodes: ~17 hours for parallel rehash.

### 25. Bandwidth — the Scarce Resource

[[will]] is the capacity to create [[cyberlinks]]. every link burns will — when it runs out, the [[neuron]] falls silent. will regenerates with stake and limits [[bandwidth]].

the $V$ stake of a [[neuron]] is the size of its battery. creating cyberlinks consumes charge. the battery fully recharges during the [[recovery period]]. if a neuron acts when network bandwidth consumption is low, it consumes less bandwidth — adaptive cost based on network load.

bandwidth is the only access control mechanism. no passwords, no permissions, no API keys. stake → will → links → [[knowledge]]. the economic structure of the [[cybergraph]] IS the permission system.

```
stake → will regeneration → bandwidth capacity → cyberlink creation → knowledge
  ↑                                                                        |
  └────────────── karma + focus rewards ───────────────────────────────────┘
```

### 26. The Full Stack

```
┌──────────────────────────────────────────────────────────────────┐
│  cyber/communication  │ onion routing, CSIDH, stark proof chains │
├───────────────────────┼──────────────────────────────────────────┤
│  radio/willow         │ confidential sync, Meadowcap access      │
│  radio/docs           │ collaborative replicas, set reconciliation│
├───────────────────────┼──────────────────────────────────────────┤
│  radio/gossip         │ topic pub/sub, epidemic broadcast trees   │
│  radio/blob + bao     │ verified streaming, Hemera Merkle trees   │
├───────────────────────┼──────────────────────────────────────────┤
│  radio/endpoint       │ QUIC, Ed25519 identity, encrypted streams │
│  radio/relay          │ encrypted fallback, focus-incentivized    │
│  radio/hole-punching  │ NAT traversal, STUN/ICE over QUIC        │
│  radio/discovery      │ key → address resolution                  │
├───────────────────────┼──────────────────────────────────────────┤
│  storage proofs       │ 6 proof types, DAS, hash migration        │
│  bandwidth / will     │ stake-based rate limiting                  │
└───────────────────────┴──────────────────────────────────────────┘
```

---

## Part VI: Bounded Liveness Runtime

### 27. Epoch Budget Allocator

```
┌──────────────────────────────────────┐
│         Epoch (e.g., 5 seconds)      │
├──────────┬──────────┬────────────────┤
│Consensus │    TX    │     Rank       │
│ 500ms    │  1500ms  │   remaining    │
│ hard     │  hard    │   soft         │
│ deadline │ deadline │  deadline      │
└──────────┴──────────┴────────────────┘
```

hard deadline: cell is preempted. soft deadline: cell yields voluntarily. no priorities. every cell gets its budget.

### 28. Compile-Time Deadline Enforcement

the async runtime does not allow unbounded futures. enforced at the type level:

```rust
trait BoundedFuture: Future {
    const MAX_DURATION: Duration;
}

let data = stream.read(&mut buf)
    .with_deadline(Duration::from_millis(100))
    .on_timeout(|| Err(Timeout))
    .await;
```

the [[Rust]] compiler becomes the liveness checker.

### 29. Wait-Free Shared State

all inter-cell communication uses wait-free data structures. no mutexes, no locks, no semaphores.

- [[knowledge graph]] reads: wait-free concurrent hash map (atomics-based)
- transaction mempool: wait-free bounded MPMC queue
- consensus state: epoch-versioned snapshots (readers never block writers)
- [[cyberank]] results: double-buffered (writers update back buffer, atomic swap to front)

---

## Part VII: Hardware Abstraction

### 30. Three Portable Formats

every computer has three types of processors. cyb has one portable format for each:

| Processor | Format | What cyb uses it for |
|-----------|--------|---------------------|
| CPU | WASM (wasmi) | Logic, layout, events, contracts, state |
| GPU | WGSL (wgpu) | Pixels, vectors, text, video, ML fallback |
| NPU | ONNX (burn-webnn) | SLM inference, AI features |

deployment targets:

```
Browser:   WASM (native) + WGSL (WebGPU) + ONNX (WebNN -> NPU)
Desktop:   WASM (wasmi) + WGSL (wgpu -> Vulkan/Metal/DX12) + ONNX (burn)
Mobile:    WASM (wasmi) + WGSL (wgpu -> Metal/GLES) + ONNX (CoreML/NNAPI)
```

same codebase. same pixels. everywhere.

### 31. Zero-Unsafe MMIO

MMIO regions as first-class language concepts:

```rust
#[mmio_region(base = 0x23B100000, size = 0x100000)]
mod aic {
    register! {
        ENABLE @ 0x010 : ReadWrite<u32> {
            enabled: bool @ 0,
            target_cpu: u4 @ 1..5,
            mode: IrqMode @ 5..7,
        }
    }
}
```

zero unsafe in user-facing code.

### 32. Neural Drivers

drivers generated by models against stable trait contracts. the HAL is ~3000 lines of [[Rust]] trait definitions covering all hardware categories.

```rust
pub trait BlockDevice: BoundedLiveness + Send + Sync {
    const BLOCK_SIZE: u32;
    const MAX_IO_LATENCY: Duration;
    async fn read_blocks(&self, lba: u64, buf: &mut [u8]) -> Result<usize>;
    async fn write_blocks(&self, lba: u64, buf: &[u8]) -> Result<usize>;
    fn capacity_blocks(&self) -> u64;
    fn health(&self) -> DeviceHealth;
}
```

| Platform | Harness Size | Status |
|----------|-------------|--------|
| QEMU/virtio | ~5K lines | Reference platform |
| RISC-V (StarFive) | ~10-15K lines | Open specs |
| Raspberry Pi 4/5 | ~15-20K lines | Well-documented |
| Apple M1 | ~35-40K lines | Asahi knowledge base |
| x86-64 generic | ~20-25K lines | Standards-based |

target: 50+ SoC families. ~1M lines of generated code validated against ~8K lines of traits and tests.

---

## Part VIII: Legacy Web Compatibility

### 33. SLM Legacy Bridge

a small language model (~100-300M parameters) that understands intent behind old CSS:

```
Layer 1: Native subset       -> direct compilation
Layer 2: Legacy -> SLM       -> cached permanently
Layer 3: Graceful degradation -> unknown ignored
```

### 34. WASM Adoption

import adapter auto-detects what a module needs:

```
.wasm arrives -> inspect imports
  -> wasi_snapshot_preview1.* -> WASI shim
  -> __wbindgen_*             -> wasm-bindgen shim
  -> env.emscripten_*         -> Emscripten compat
  -> env.db_read/db_write     -> native CosmWasm
  -> unknown                  -> SLM classifies, generates adapter
```

| Chrome concept | Cyb upgrade |
|----------------|------------|
| Cookie | Cryptographic keypair |
| localStorage | SQLite |
| fetch to REST | Direct contract call (<1ms) |
| IndexedDB | SQLite + Merkle proofs |
| Service Worker | [[radio]] content-addressed storage |
| OAuth / JWT | Cryptographic signatures |
| WebSocket | P2P [[libp2p]] connection |

---

## Part IX: Numbers

### 35. 130K Lines

```
PureRender                              ~100K
  15 primitives (parse, layout, render)    40K
  TS strict compiler                       22K
  Runtime (wgpu, layout, events)           22K
  Infrastructure (PDF, fetch, SQLite)      17K

CosmWasm integration                     ~26K
WASM adoption layer                      ~14K
burn-webnn                               ~12K

TOTAL                                   ~130K
```

CybOS core: ~85-125K lines (human-authored, auditable by one person in a month). neural driver layer: ~500K-1M lines (model-generated, compiler + test validated).

### 36. Cyb vs Chrome

| | Chrome | Cyb |
|-|--------|-----|
| Codebase | 35M lines C++ | 130K lines [[Rust]] |
| Render | Blink (DOM/CSS) | PureRender (9 primitives + 4 layouts) |
| WASM | V8 JIT | wasmi (deterministic, metered) |
| Identity | Cookie | Keypair |
| State | Server-side | Local SQLite + on-chain |
| Contracts | Via HTTP to node | Native, same runtime as UI |
| Binary | ~150+ MB | ~10 MB |

270× reduction in code for a system that does more.

---

## Build Order

### Phase 1 — Foundation (Now)

1. [[Nox]] — Define the 16-pattern structural IR with abstract Merkle authentication
2. [[Trident]] — Refine compiler and TIR
3. [[Rs]] — Strict [[Rust]] subset, same compiler backend as Trident, target Nox runtime

### Phase 2 — Expansion (Next)

4. [[Arc]] — Graph DSL for [[cybergraph]] programming. Compiles to Trident for proofs, native engine for queries.
5. [[Seq]] — Temporal logic for consensus rules and scheduling. Three temporal modes built in.
6. [[Inf]] — Datalog over the cybergraph. Rule-based inference turns explicit links into implicit knowledge.

### Phase 3 — Specialization (When needed)

7. [[Bt]] — Binary circuits for legacy hash verification and cross-chain bridges.
8. [[Wav]] — Signal processing. Start as Rs library, promote to language if sensor workloads justify it.
9. [[Ten]] — Tensor operations. Start as Rs/Tri library, promote if ML inference verification becomes core.

### Phase 4 — Geometry (Research horizon)

10. Ren — Clifford geometric algebra. Engineering-ready, closest to Tri. Completes the Arc → SVG rendering pipeline.
11. Dif — Differential geometry. Riemannian manifolds over finite fields. Needed for [[tri-kernel]] formalization.
12. Sym — Symplectic geometry. Hamiltonian mechanics, [[conservation]] laws. Physics simulation.
13. Bel — Information geometry. Fisher metric on [[probability]] simplices. Self-model for [[superintelligence]].

see [[cyb/languages]] for the algebraic completeness argument. see [[cyb/multiproof]] for how all thirteen settle under one proving umbrella.

---

## The Thesis

cyb/os rests on three observations and one boundary.

one. every computational universe has a native type whose algebraic laws define how programs think. forcing computations across universe boundaries creates encoding overhead that scales with complexity. thirteen algebras → thirteen [[cyb/languages]].

two. every perceptual channel has a native format whose rendering laws define how humans see. forcing display across format boundaries creates visual noise. nine senses → nine primitives.

three. every human action is a decision with its own algebra: options, preferences, beliefs, commitments. ten decision types → ten interaction primitives.

the boundary. the machine computes, the human decides. computation produces options. perception displays them. decision collapses them to action. the action commits to new state, and the cycle continues.

all values in all universes (except Binary) decompose into three atoms — three modes of reference that are exhaustive:

```
field = the value IS the reference     (by content)
word  = position IS the reference      (by location)
hash  = name IS the reference          (by commitment)
```

these atoms compose through one structural substrate ([[Nox]], authenticated trees). they persist through three temporal modes (stack, heap, stream). they are present through one register — the singular now, the atom of attention where computation happens.

all three grids share one universal structural pair — fork and join — wearing three costumes:

```
Computation:  axis / cons      (decompose / build)
Perception:   expand / nest    (drill in / compose)
Decision:     split / merge    (diverge / converge)
```

thirteen languages. nine primitives. ten decisions. three atoms. three times. one fork. one join. one tree. one [[proof]]. one operating system.

see [[cyb]], [[cyb/whitepaper]], [[cyb/languages]], [[cyb/multiproof]], [[Rust]], [[cyber]]
