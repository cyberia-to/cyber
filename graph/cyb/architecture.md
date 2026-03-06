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

cyb/os is a stack of typed universes — nine computation languages compiled through one structural IR, rendered through nine perception primitives, driven by ten decision primitives — all sharing one toolchain, one tree substrate, and one proof system.

core stack: [[radio]] for data publishing, [[cyber]] for search and [[learning]], [[rune]] for dynamic execution, [[CozoDB]] graph storage, [[cosmos-sdk]] chains via [[IBC]]. builds for [[web]], [[desktop]], [[mobile]].

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
Ask    → relations                        table     → 2D grid
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

every computation type has a canonical rendering. a tree computed in [[Nox]] naturally displays as a collapsible `struct`. a graph traversed in [[Arc]] naturally draws as `vector` paths. a relation queried in [[Ask]] naturally fills a `table`. a signal processed in [[Wav]] naturally plays as `sound`. the mapping is many-to-many, but the canonical pairing is the path of least impedance — where the shape of the data matches the shape of the display.

every rendering invites a decision. the human responds with typed decision primitives — select, rank, compose, confirm — each with its own algebra, its own temporal mode, and its own relationship to the computation and perception grids.

### 1. Nine Computation Languages

```
Universe          Language    Type        Algebra           Purpose
─────────────────────────────────────────────────────────────────────
Structure         Nox         Tree        Combinators       Composition
Binary            Bt          Bit         𝔽₂ tower          Circuits
Byte              Rs          Word        Bitwise on 𝔽ₚ     Systems
Field             Trident     Field       Arithmetic on 𝔽ₚ  Proofs
Topology          Arc         Graph       Adjacency         Knowledge
Causality         Seq         Event       Partial order     Ordering
Inference         Ask         Relation    Unification       Reasoning
Continuum         Wav         Signal      Convolution       Sensing
Linear            Ten         Tensor      Contraction       Learning
```

a data type deserves its own language when its algebraic laws are so different from other types that forcing it into a foreign language creates constant impedance mismatch. nine fundamental types pass this test. each inhabits a universe defined by its characteristic algebraic structure. some universes share a proof system. some share a compiler. none share semantics.

### 2. The Value Tower — Three Modes of Reference

Byte and Field share the same mathematical substrate — the [[Goldilocks field]] 𝔽ₚ where p = 2⁶⁴ − 2³² + 1. this substrate provides three atom types sufficient for seven of the nine universes.

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

three atoms are complete — for one characteristic. the single exception is [[Bt]]: a bit is genuinely not an element of 𝔽ₚ. it lives in 𝔽₂ — different characteristic, different algebra. that is exactly why Bt has a separate proof system, not just a new type tag.

```
Nox value tower (3 atoms: field, word, hash)
  sufficient for: Rs, Trident, Arc, Seq, Ask, Wav, Ten
  NOT sufficient for: Bt

Bt value tower (separate, 𝔽₂)
  sufficient for: Bt only
```

### 3. Universe 0 — Nox (Structure)

the continuation of the Nock idea — everything is a binary tree, code is data, computation is tree rewriting. five structural operations define how values compose regardless of what those values are:

| Op        | Action                          | Analogy              |
|-----------|---------------------------------|----------------------|
| `axis`    | Navigate into a subtree by path | Array index          |
| `quote`   | Treat code as data              | String literal       |
| `compose` | Chain two computations          | Function composition |
| `cons`    | Build a pair                    | Struct constructor   |
| `branch`  | Conditional selection           | If-then-else         |

the critical difference from Nock: Nox's tree is a Merkle tree by construction. every `cons(a, b)` computes `hash(a, b)` and stores the digest at the parent node. `axis` produces a Merkle proof as a side effect. the authentication scheme is abstract — pluggable backends (Tip5, Poseidon2, SHA-256, Verkle, SMT).

Nox is simultaneously the structural IR (the grammar all languages compile through) and the node runtime (the production binary that runs the [[cyber]] blockchain).

### 4. Universe 1 — Bt (Binary)

| Type    | Field        | Size   | Native ops           |
|---------|--------------|--------|----------------------|
| `Bit`   | 𝔽₂           | 1 bit  | AND (mul), XOR (add) |
| `Bit2`  | 𝔽₂²          | 2 bits | Extension field ops  |
| `Bit8`  | 𝔽₂⁸          | 1 byte | AES-native           |
| `Bit32` | 𝔽₂³²         | 4 bytes| Hash-native          |
| `Bit64` | 𝔽₂⁶⁴         | 8 bytes| Double word          |
| `Bit128`| 𝔽₂¹²⁸        | 16 bytes| Security parameter  |

characteristic: 2. proof system: FRI-Binius. AND is multiplication. XOR is addition. both are free. what Bt cannot do cheaply: integer arithmetic. 3 + 5 = 6 in 𝔽₂ (XOR), not 8. to perform actual addition with carry, you must build ripple-carry adders from AND/XOR gates.

use cases: BLAKE3/SHA-256 circuits (proving legacy hashes), Keccak verification (Ethereum bridge), AES circuits, binary Merkle tree verification, binary protocol parsing.

### 5. Universe 2 — Rs (Byte) & Universe 3 — Trident (Field)

Byte and Field share the Goldilocks substrate but present opposite mental models. a byte programmer thinks in registers and bit patterns. a field programmer thinks in algebraic constraints. same representation, opposite intent.

Rs is [[Rust]] with everything dynamically-sized removed. no heap. no `Vec`. no `String`. no unbounded recursion. every value has a known size at compile time. every loop has a known bound. the hidden truth: every `u64` in Rs is secretly a `word` — type tag 0x01 — which is secretly a field element with a range constraint. the programmer writes conventional-looking systems code, but every operation is field-compatible.

```
Rust        → full language, heap, strings, anything
  ↓ restrict
Rs          → strict subset, bounded, looks like systems code
  ↓ reveal
Trident     → same restrictions, but the field is visible
```

[[Trident]] is where the field is visible and the programmer thinks in constraints. division is exact (multiplicative inverse). every operation becomes a polynomial constraint in the STARK execution trace. Trident-only primitives: `divine()` (inject prover witness), `hash()` (Tip5, single constraint), `merkle_step()`, `seal` (hashed/private event emission).

Trident layer architecture:

| Layer | Scope           | Types available           | Compilation targets      |
|-------|-----------------|---------------------------|--------------------------|
| 0     | Execute Anywhere | U32, Bool, structs, arrays | TASM, EVM, [[CosmWasm]], SVM |
| 1     | Prove Anywhere   | + Field, Digest, divine() | TASM (Triton VM)         |
| 2     | Platform Powers  | + chain-specific stdlib   | Single target            |

```
.rs file  → parser (Rust subset) → TIR → TASM / backend
.tri file → parser (Trident)     → TIR → TASM / backend
                                    ↑
                              same IR, same value tower
```

### 6. Universe 4 — Arc (Topology)

the graph language. makes graphs first-class — the primitive is a connection, not a number.

| Op              | Action                                         |
|-----------------|-------------------------------------------------|
| `link(a, b, w)` | Create weighted directed edge                  |
| `walk(start, n)`| Random walk of n steps                         |
| `reach(a, b)`   | Test if path exists                            |
| `neighbors(n)`  | Return adjacent nodes                          |
| `rank(g, steps)`| Compute stationary distribution (PageRank)     |
| `spectral(g, k)`| Extract top-k eigenvectors                     |
| `match(g, pat)` | Subgraph pattern matching                      |

the [[cybergraph]] is not a data structure that lives inside a program. the cybergraph IS the program. every [[cyberlink]] is an `Edge`. every CID is a `Node`. CYBERRANK is `rank()`. Arc decomposes into [[Trident]] (field ops for matrix math), [[Bt]] (hash verification for node identities), and [[Nox]] (tree encoding of topology).

### 7. Universe 5 — Seq (Causality)

the event language. time in distributed systems is not a clock — it is the ordering. the causal structure that determines what could have influenced what. three temporal modes:

```
Stack   = nested time     = depth     = LIFO  = after { after { after } }
Heap    = concurrent time = chaos     = random = concurrent(a, b, c)
Stream  = linear time     = flow      = FIFO  = before(a, b), before(b, c)
```

| Domain      | Stack                | Heap                  | Stream              |
|-------------|----------------------|-----------------------|---------------------|
| Hardware    | Call stack           | RAM allocation        | I/O bus             |
| OS          | Process call depth   | Dynamic memory        | Pipes, sockets      |
| Network     | Protocol nesting     | Concurrent connections| Packet flow         |
| Consensus   | Nested validation    | Parallel validators   | Block sequence      |
| UI          | Modal dialogs, undo  | Independent windows   | Scrolling, typing   |

events form a partial order — not a total order. Seq preserves the partial order and only totalizes when consensus demands it.

### 8. Universe 6 — Ask (Inference)

the query language. relations and unification — [[Datalog]] at its core. the only language that derives truth rather than transforming values.

```
reachable(X, Y) :- link(X, Y).
reachable(X, Z) :- link(X, Y), reachable(Y, Z).
?- reachable(a, X), linked_by(d, X).
```

Arc is what is connected (topology). Ask is what follows (entailment). together they form a complete knowledge system: structure + inference. the Datalog restriction ensures bounded inference, guaranteed termination, proof-compatible. because Ask is bounded, any derivation can be encoded as a [[Trident]] computation and proven with a STARK. zero-knowledge inference over a private knowledge graph.

### 9. Universe 7 — Wav (Continuum)

the signal language. a signal is a waveform — a continuous function sampled at discrete points. primitive operations: `fft`, `ifft`, `convolve`, `lowpass`, `resample`, `correlate`, `energy`, `peak_detect`. use cases: sensor data processing, audio, seismic, environmental monitoring.

### 10. Universe 8 — Ten (Linear)

the tensor language. `Tensor<[D1, D2, ..., Dk]>` where dimensions are compile-time constants. shape mismatches are compile errors. primitive operations: `matmul`, `einsum`, `reshape`, `broadcast`, `transpose`, `reduce`, `conv2d`, `softmax`. CYBERRANK is literally repeated `matmul`.

### 11. Compilation Architecture

```
                    ┌──────────────────────────────────────┐
                    │          Programmer Faces             │
                    │                                       │
                    │  Bt  Rs  Trident Arc Seq Ask Wav Ten  │
                    │  .bt .rs .tri    .arc .seq .ask .wav .ten
                    └──────────┬───────────────────────────┘
                               │
                    ┌──────────▼───────────────────────────┐
                    │         Shared Frontend               │
                    │   Parsing, type checking,             │
                    │   borrow checking, bound checking     │
                    └──────────┬───────────────────────────┘
                               │
                    ┌──────────▼───────────────────────────┐
                    │     Nox Structural IR                 │
                    │     axis, quote, compose,             │
                    │     cons, branch                      │
                    │     + typed computational ops         │
                    │     + Merkle authentication           │
                    └──────────┬───────────────────────────┘
                               │
              ┌────────────────┼────────────────┐
              │                │                │
     ┌────────▼──────┐ ┌──────▼──────┐ ┌───────▼───────┐
     │  Binius/FRI   │ │  Goldilocks │ │   Native      │
     │  Backend      │ │  TASM/FRI   │ │   Backend     │
     │  (Binary)     │ │ (Byte+Field)│ │   (no proof)  │
     └───────────────┘ └─────────────┘ └───────────────┘
          Bt              Rs, Trident       Arc, Seq, Ask,
                                            Wav, Ten
```

| Source  | When proof needed                | When proof absent           |
|---------|----------------------------------|-----------------------------|
| Bt      | Binius FRI circuit               | always proving              |
| Rs      | TASM → STARK (word→field lift)   | native binary (Nox)         |
| Trident | TASM → STARK (field native)      | WASM/EVM (Layer 0)          |
| Arc     | decomposes into Trident + Bt     | optimized graph engine      |
| Seq     | temporal constraints → STARK     | scheduler / runtime         |
| Ask     | derivation trace → STARK         | Datalog engine              |
| Wav     | decomposes into Trident          | native DSP pipeline         |
| Ten     | decomposes into Trident          | native BLAS / GPU           |

### 12. Nine Perception Primitives

the irreducible visual types — the atoms of everything a human can perceive through a screen and speakers. any UI, any document, any application is a composition of these nine.

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

### 13. Ten Decision Primitives

every human interaction with a computer is a decision. strip the physics away — what remains is pure decision structure.

| #  | Primitive | Action                | Reversible? | Time Mode | Comp Language | Perception |
|----|-----------|-----------------------|-------------|-----------|---------------|------------|
| 1  | observe   | Gather without choosing | Always    | Stream    | Wav           | any        |
| 2  | filter    | Narrow by criteria    | Yes         | Stack     | Ask           | struct     |
| 3  | select    | Choose one            | Yes         | Stream    | Ask           | table      |
| 4  | rank      | Order by preference   | Yes         | Stream    | Ten           | table      |
| 5  | compose   | Build new value       | Yes         | Stack     | Rs            | text/vector|
| 6  | split     | One becomes many      | Depends     | Heap      | Arc           | vector     |
| 7  | merge     | Many become one       | Depends     | Heap      | Arc + Ask     | vector     |
| 8  | delegate  | Route to agent        | Sometimes   | Heap      | Arc           | vector     |
| 9  | reject    | Explicitly not-choose | Mostly      | Stream    | Seq           | video      |
| 10 | confirm   | Irreversible commit   | Never       | Stack     | Trident       | formula    |

the machine computes, the human decides. computation produces options. perception displays them. decision collapses them to action. the action commits to new state, and the cycle continues.

confirm is the only primitive that is always irreversible. it is structurally unique — the moment where possibility collapses into fact. every other primitive can be undone, revised, or abandoned.

### 14. Cross-Grid Connections

the three grids interlock in a continuous decision loop — the cyb/os event loop:

```
loop {
  state   = nox_tree(current)           // authenticated tree
  options = compute(state)              // some universe produces alternatives
  display = render(options)             // canonical primitive shows them
  choice  = decide(human_input)         // decision primitive applied
  proof   = commit(choice, state)       // irreversible, potentially STARK-proven
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

### 15. The Comparison Matrix

| Property    | Nox      | Bt       | Rs       | Trident  | Arc      | Seq      | Ask       | Wav      | Ten      |
|-------------|----------|----------|----------|----------|----------|----------|----------|----------|----------|
| Universe    | Structure| Binary   | Byte     | Field    | Topology | Causality| Inference | Continuum| Linear   |
| Char        | —        | 2        | p        | p        | —        | —        | —         | ≈ℝ       | ≈ℝ or p  |
| Primitive   | Cell     | Bit      | Word     | Field    | Edge     | Event    | Relation  | Sample   | Shape    |
| Reference   | structure| wire     | location | content  | adjacency| succession| entailment| amplitude| index    |
| Free op     | Navigate | AND, XOR | Index    | Mul, Add | Link     | Order    | Unify     | Convolve | Matmul   |
| Costly op   | —        | Carry add| Mod div  | Bitwise  | Spectral | Verify   | Fixpoint  | FFT      | Inverse  |
| Proof       | Inherited| Binius   | STARK    | STARK    | Delegated| Delegated| Delegated | Delegated| Delegated|
| Syntax feel | IR       | Circuit  | Rust     | Custom   | Query    | Temporal | Datalog   | DSP      | NumPy    |
| Renders as  | struct   | pixels   | text     | formula  | vector   | video    | table     | sound    | component|

---

## Part II: Render Stack — PureRender

### 16. Flat Streams Instead of Trees

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

### 17. Component as a Single Unit

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

### 18. Compilation: Everything to WASM + WGSL

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

### 19. GPU Render: Everything is Shaders

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

### 20. Smart Contracts in the Browser

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

### 21. Design Axioms

1. no Unix legacy. no files, no processes, no users, no fork/exec, no POSIX. cyb abstractions are native to its domain: agents, [[cyberlinks]], ranks, epochs, bandwidth.
2. zero unsafe [[Rust]]. the entire OS — kernel, drivers, [[consensus]], storage — compiles without a single `unsafe` block. memory safety is a compiler-verified property.
3. bounded liveness everywhere. no operation can block indefinitely. no module can starve another. every async future has a compile-time deadline. the system degrades gracefully, never halts.
4. neural drivers. hardware support generated by LLMs against stable trait contracts, verified by the compiler, validated by conformance test suites.
5. single address space. no user/kernel split. no syscalls. no TLB flushes. isolation enforced by [[Rust]] ownership, not hardware privilege levels.

### 22. Layered Design

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
│  │  LLM-generated, compiler-verified per-platform  │  │
│  └────────────────────────────────────────────────┘  │
│                       │                              │
│                  ┌────┴────┐                          │
│                  │Hardware │                          │
│                  └─────────┘                          │
└──────────────────────────────────────────────────────┘
```

### 23. No Processes — Cells

cells replace processes: independently compiled [[Rust]] crates that can be loaded, unloaded, and hot-swapped at runtime without stopping the system. each cell has explicit dependency declarations, typed bounded wait-free channels, exclusive state ownership, mandatory heartbeat reporting. cell lifecycle is governed by on-chain governance.

| Missing Cell | System Behavior |
|-------------|-----------------|
| Rank | Validates blocks, does not answer rank queries |
| Consensus | Becomes full node (follows chain, does not vote) |
| Query | Participates in consensus, does not serve clients |
| Gossip | Works with local state only (island mode) |
| Storage | Emergency halt, preserves last state |

### 24. No File System — the Graph

no hierarchical file system. no paths, no inodes, no directories. all persistent data lives in one structure: a content-addressed [[knowledge graph]].

every piece of data is a node. every relation is a [[cyberlink]]. CIDs are the universal address space. the graph IS the storage:

- state: Merkle trees over graph nodes — blockchain state is a subgraph
- knowledge: CIDs linked by cyberlinks — the primary structure of all data
- blocks: append-only chain — a linearly ordered subgraph of consensus events
- configuration: compiled into the binary at build time — the only exception

there is no boundary between "storage" and "database". the graph holds particles, links, ranks, proofs, programs. querying storage and querying knowledge are the same operation: graph traversal.

### 25. No Users — the Avatar System

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

### 26. Purpose-Built Networking

three network protocols only:

| Protocol | Purpose | Transport |
|----------|---------|-----------|
| Gossip | Propagate transactions and blocks | UDP/QUIC |
| Consensus | Validator voting, proposals, prevotes | UDP/QUIC |
| Query | Client requests for rank, graph data, proofs | QUIC streams |

~15K lines instead of ~100K+ for full TCP/IP + HTTP + TLS. each protocol is a separate cell with its own bounded budget.

---

## Part V: Bounded Liveness Runtime

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

## Part VI: Hardware Abstraction

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

drivers generated by LLMs against stable trait contracts. the HAL is ~3000 lines of [[Rust]] trait definitions covering all hardware categories.

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

## Part VII: Legacy Web Compatibility

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

## Part VIII: Numbers

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

CybOS core: ~85-125K lines (human-authored, auditable by one person in a month). neural driver layer: ~500K-1M lines (LLM-generated, compiler + test validated).

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
6. [[Ask]] — Datalog over the cybergraph. Rule-based inference turns explicit links into implicit knowledge.

### Phase 3 — Specialization (When needed)

7. [[Bt]] — Binary circuits for legacy hash verification and cross-chain bridges.
8. [[Wav]] — Signal processing. Start as Rs library, promote to language if sensor workloads justify it.
9. [[Ten]] — Tensor operations. Start as Rs/Trident library, promote if ML inference verification becomes core.

---

## The Thesis

cyb/os rests on three observations and one boundary.

one. every computational universe has a native type whose algebraic laws define how programs think. forcing computations across universe boundaries creates encoding overhead that scales with complexity. nine algebras → nine languages.

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

nine languages. nine primitives. ten decisions. three atoms. three times. one fork. one join. one tree. one proof. one operating system.

see [[cyb]], [[cyb/whitepaper]], [[Rust]], [[cyber]]
