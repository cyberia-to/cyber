---
tags: cyb, article, research
alias: cyb architecture, cyb os, cybos, cyber-os-architecture, cyb-system-architecture
icon: "\U0001F310"
crystal-type: entity
crystal-domain: cyber
crystal-size: deep
stake: 14015797676239542
---
# Architecture

[[cyb]] is a sovereign browser that becomes an operating system. identity is a keypair, state lives on-chain, [[smart contracts]] run locally, and the entire render stack compiles to GPU. one binary, all platforms, 130K lines of [[rust]], no WebView, no V8, no Google.

at the OS layer: a purpose-built runtime for nodes of a decentralized [[knowledge graph]], designed from first principles in [[rust]] with zero unsafe code, bounded liveness guarantees, and LLM-generated hardware support.

core stack: [[radio]] for data publishing, [[cyber]] for search and [[learning]], [[rune]] for dynamic execution, [[cozodb]] graph storage, gamified with [[aos]], [[cosmos-sdk]] chains via [[ibc]]. builds for [[web]], [[desktop]], [[mobile]].

---

## Part I: Render Stack — PureRender

### 1. Fifteen Primitives

every interface ever built is a combination of fifteen things.

#### Content — 9 primitives

| Primitive | What it is | GPU mapping |
|-----------|-----------|-------------|
| text | [[markdown]], prose, code | Glyphs via compute shader rasterization |
| struct | JSON, TOML — trees and configs | Collapsible tree of text glyphs |
| table | 2D data, CSV | Grid of text cells, virtualized rows |
| vector | SVG, paths, Bezier curves | Path rasterization via Vello |
| pixels | Raster image | Texture upload, GPU sampler |
| video | Moving pixels | Hardware decode, texture per frame |
| sound | Waveform, audio stream | Audio pipeline (visual: waveform shader) |
| formula | LaTeX / MathML — math notation | Glyph layout + vector curves via Vello |
| component | Composition of primitives | Nested render pass |

#### Interactive — 5 primitives

| Primitive | What it is | How it maps |
|-----------|-----------|-------------|
| action | Tap, click, press | Any content primitive + onClick handler |
| input text | String, textarea, code editor | text + editable flag + cursor + IME |
| input choice | Select, radio, checkbox, toggle | struct (options) + selectable flag |
| input range | Slider, scroll, zoom | number + bounds + draggable |
| input media | Camera, microphone, file upload | pixels/sound + capture pipeline |

#### Layout — 4 modes

| Mode | What it is | Use case |
|------|-----------|----------|
| stream | Vertical sequence, scrollable | Blogs, feeds, articles, chat |
| grid | 2D spatial container | Dashboards, layouts, galleries |
| flex | 1D flexible row or column | Navbars, toolbars, card rows |
| page | Fixed canvas, pagination | PDF, print, scientific papers |

a button is `text` + `action`. a dashboard is `grid` of `table` + `vector`. a scientific paper is `page` of `text` + `formula` + `table`. an IDE is `grid` of `text` + `struct`. every UI ever made is a combination of these fifteen primitives and four layout modes.

### 2. Flat Streams Instead of Trees

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

same primitives, different layout mode. this is how cyb reads and produces PDFs, prints documents, and renders scientific papers — natively. screen, PDF, print — same component, same pipeline, different output target.

### 3. Component as a Single Unit

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

### 4. Compilation: Everything to WASM + WGSL

no interpreter. no JIT. no runtime parsing. everything compiles.

strict TypeScript: full type system, async/await, generics, interfaces, enums, unions, modules, pattern matching, destructuring. no `any`, no `eval()`, no `arguments`, no prototype chains, no `this` binding, no runtime type checks. no type — no compilation. types are instructions for the AOT compiler. what V8 guesses through speculative JIT is known at compile time.

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

### 5. GPU Render: Everything is Shaders

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

## Part II: Execution Engine — CosmWasm Native

### 6. Smart Contracts in the Browser

in Chrome, a dApp is a JS app that talks to a blockchain node over HTTP. in cyb, [[smart contracts]] run locally in the same runtime as UI.

```
Chrome:     JS -> fetch() -> REST API -> Go node -> CosmWasm -> result
            Round trip: 200-2000ms

Cyb:        Component (wasmi) -> direct call -> Contract (wasmi) -> result
            Round trip: <1ms
```

a [[CosmWasm]] contract is a `.wasm` binary with entry points: `instantiate`, `execute`, `query`, `migrate`. the contract talks to the host through imports — five database functions, crypto, chain query.

storage stack:

```
Contract code:      BALANCES.load(storage, &addr)
       |
cw-storage-plus:    typed abstractions (Item, Map, IndexedMap)
       |
cosmwasm-vm:        db_read(key) -> host call
       |
Cyb host:           Rust implementation
       |
SQLite:             local persistent storage
       |
(Optional):         ics23 Merkle proofs for state verification
```

no Go. no Tendermint. no full node. just [[rust]] all the way down to SQLite.

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

no separation between frontend and on-chain logic. one file, one scope, one runtime. UI and state machine unified.

gas metering: wasmi counts fuel per instruction. every WASM module in cyb — UI, contract, plugin — is metered. a malicious page cannot infinite-loop.

---

## Part III: Kernel Architecture — CybOS

### 7. Design Axioms

1. No Unix legacy. no files, no processes, no users, no fork/exec, no POSIX. cyb abstractions are native to its domain: agents, [[cyberlinks]], ranks, epochs, bandwidth.

2. Zero unsafe [[rust]]. the entire OS — kernel, drivers, [[consensus]], storage — compiles without a single `unsafe` block. memory safety is a compiler-verified property.

3. Bounded liveness everywhere. no operation can block indefinitely. no module can starve another. every async future has a compile-time deadline. the system degrades gracefully, never halts.

4. Neural drivers. hardware support generated by LLMs against stable trait contracts, verified by the compiler, validated by conformance test suites, improved through network-wide telemetry.

5. Single address space. no user/kernel split. no syscalls. no TLB flushes. isolation enforced by [[rust]] ownership, not hardware privilege levels.

### 8. Layered Design

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

### 9. No Processes — Cells

inspired by Theseus OS, cyb replaces processes with cells: independently compiled [[rust]] crates that can be loaded, unloaded, and hot-swapped at runtime without stopping the system.

each cell is a single [[rust]] crate with explicit dependency declarations. communicates through typed, bounded, wait-free channels. owns its state exclusively. can be replaced at runtime via [[rust]] `Into` trait for state migration. reports health via mandatory heartbeat.

cell lifecycle is governed by on-chain governance: a proposal passes → new cell binary is verified → old cell is drained → state is migrated → new cell activates. validators never go offline for upgrades.

### 10. No File System — Content-Addressed Storage

no hierarchical file system. no paths, no inodes, no directories. all persistent data is content-addressed:

- state storage: Merkle trees of blockchain state
- [[knowledge graph]]: CIDs linked by [[cyberlinks]]
- block storage: append-only chain of blocks
- configuration: compiled into the binary at build time

```
trait ContentStore: BoundedLiveness {
    async fn get(&self, cid: &Cid) -> Result<Option<Block>>;
    async fn put(&self, data: &[u8]) -> Result<Cid>;
    async fn merkle_root(&self) -> Hash;
    async fn prove_inclusion(&self, cid: &Cid) -> Result<MerkleProof>;
}
```

### 11. No Users — Cryptographic Agents

identity is a public key. an agent is defined by: a public key (address in the [[knowledge graph]]), a bandwidth allocation (determined by staked tokens), a set of [[cyberlinks]] created by this agent. access control: an agent can create [[cyberlinks]] up to their bandwidth limit. the [[knowledge graph]] is public. bandwidth is the only scarce resource.

### 12. Purpose-Built Networking

three network protocols only:

| Protocol | Purpose | Transport |
|----------|---------|-----------|
| Gossip | Propagate transactions and blocks | UDP/QUIC |
| [[Consensus]] | Validator voting, proposals, prevotes | UDP/QUIC |
| Query | Client requests for rank, graph data, proofs | QUIC streams |

~15K lines instead of ~100K+ for full TCP/IP + HTTP + TLS. each protocol is a separate cell with its own bounded budget. gossip cannot starve [[consensus]]. queries cannot starve gossip.

---

## Part IV: Bounded Liveness Runtime

### 13. Epoch Budget Allocator

[[time]] is divided into epochs (aligned with block production). each cell receives a guaranteed time budget per epoch:

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

hard deadline: cell is preempted if it exceeds its budget. soft deadline: cell yields voluntarily when budget is low. no priorities. every cell gets its budget. no cell is "more important."

### 14. Compile-Time Deadline Enforcement

the async runtime does not allow unbounded futures. enforced at the type level:

```rust
trait BoundedFuture: Future {
    const MAX_DURATION: Duration;
}

fn spawn<F: BoundedFuture>(future: F) { ... }

let data = stream.read(&mut buf)
    .with_deadline(Duration::from_millis(100))
    .on_timeout(|| Err(Timeout))
    .await;
```

the [[rust]] compiler becomes the liveness checker.

### 15. Wait-Free Shared State

all inter-cell communication uses wait-free data structures. no mutexes, no locks, no semaphores.

- [[knowledge graph]] reads: wait-free concurrent hash map (atomics-based)
- transaction mempool: wait-free bounded MPMC queue
- [[consensus]] state: epoch-versioned snapshots (readers never block writers)
- [[cyberank]] results: double-buffered (writers update back buffer, atomic swap to front)

every data structure guarantees: any thread completes its operation in O(1) steps regardless of contention.

### 16. Heartbeat and Graceful Degradation

every cell sends a heartbeat. missed heartbeat → warning → unwinding → restart → disable. the system never crashes, it degrades:

| Missing Cell | System Behavior |
|-------------|-----------------|
| Rank | Node validates blocks, does not answer rank queries |
| [[Consensus]] | Node becomes full node (follows chain, does not vote) |
| Query | Node participates in [[consensus]], does not serve clients |
| Gossip | Node works with local state only (island mode) |
| Storage | Emergency: system halts gracefully, preserves last state |

---

## Part V: Hardware Abstraction

### 17. Three Portable Formats

every computer has three types of processors. cyb has one portable format for each:

| Processor | Format | What cyb uses it for |
|-----------|--------|---------------------|
| CPU | WASM | Logic, layout, events, contracts, state |
| GPU | WGSL | Pixels, vectors, text, video, ML fallback |
| NPU | ONNX | SLM legacy bridge, AI features |

zero architecture-specific code. WASM is the only instruction set. WGSL is the only shader language. ONNX is the only model format.

### 18. Deployment Targets

```
Browser:   WASM (native) + WGSL (WebGPU) + ONNX (WebNN -> NPU)
Desktop:   WASM (wasmi) + WGSL (wgpu -> Vulkan/Metal/DX12) + ONNX (burn)
Mobile:    WASM (wasmi) + WGSL (wgpu -> Metal/GLES) + ONNX (CoreML/NNAPI)
```

same codebase. same pixels. everywhere.

### 19. One GPU Layer

wgpu is the single GPU abstraction:

```
wgpu
 +---> Vello        (2D rendering: text, SVG, boxes, formulas)
 +---> burn-wgpu    (ML inference: SLM, classification)
 +---> Canvas 2D    (user draw calls)
 +---> Video        (texture decode, frame display)
```

### 20. NPU Access: burn-webnn

```
.onnx model -> burn -> burn-webnn
                         +---> Browser: WebNN API -> NPU
                         +---> macOS/iOS: CoreML -> Apple Neural Engine
                         +---> Windows: DirectML
                         +---> Android: NNAPI
                         +---> Linux: OpenVINO
                       burn-wgpu (fallback: GPU)
                       burn-ndarray (last resort: CPU)
```

### 21. Zero-Unsafe MMIO

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

fn configure_interrupts() {
    aic::ENABLE.write(|r| {
        r.set_enabled(true);
        r.set_target_cpu(0);
    });
}
```

zero unsafe in user-facing code.

### 22. Neural Drivers

hardware drivers are the largest part of any OS. Linux has 30M+ lines; 70%+ is drivers. cyb inverts this: drivers generated by LLMs against stable trait contracts.

the HAL is ~3000 lines of [[rust]] trait definitions covering all hardware categories: block storage, networking, interrupts, IOMMU, timer, GPIO, I2C, SPI, serial, DMA, thermal, power, crypto acceleration, entropy.

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

LLM generates → compiler rejects → LLM fixes → tests fail → LLM fixes → tests pass → human reviews → merge. drivers that improve through [[consensus]] of the network.

| Platform | Harness Size | Status |
|----------|-------------|--------|
| QEMU/virtio | ~5K lines | Reference platform |
| RISC-V (StarFive) | ~10-15K lines | Open specs |
| Raspberry Pi 4/5 | ~15-20K lines | Well-documented |
| Apple M1 | ~35-40K lines | Asahi knowledge base |
| x86-64 generic | ~20-25K lines | Standards-based |

target: 50+ SoC families. ~1M lines of generated code validated against ~8K lines of traits and tests.

---

## Part VI: Legacy Web Compatibility

### 23. SLM Legacy Bridge

a small language model (~100-300M parameters) that understands intent behind old CSS:

```
Layer 1: Native subset       -> direct compilation
Layer 2: Legacy -> SLM       -> cached permanently
Layer 3: Graceful degradation -> unknown ignored
```

### 24. WASM Adoption

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

### 25. Scripting

TypeScript (strict) for applications. AOT compiled to WASM. [[rune]] for the engine itself — hot reload, dev console, extensions, legacy transpilation.

---

## Part VII: Numbers

### 26. 130K Lines

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

Chrome: 35M lines (270x). Firefox: 25M lines (190x). cyb: 130K lines.

### 27. Cyb vs Chrome

| | Chrome | Cyb |
|-|--------|-----|
| Codebase | 35M lines C++ | 130K lines [[rust]] |
| Render | Blink (DOM/CSS) | PureRender (15 primitives) |
| WASM | V8 JIT | wasmi (deterministic, metered) |
| Identity | Cookie | Keypair |
| State | Server-side | Local SQLite + on-chain |
| Contracts | Via HTTP to node | Native, same runtime as UI |
| Binary | ~150+ MB | ~10 MB |

---

## Philosophical Note

every existing OS asks: "What does the user want to do with this computer?"

cyb asks: "What can this computer contribute to collective [[intelligence]]?"

the first question leads to 30 million lines of code and infinite attack surface. the second leads to 130K lines of auditable code, zero unsafe, and hardware support that grows through the [[intelligence]] of the network.

a node is a [[neuron]] in a planetary nervous system. cyb is its membrane.

see [[cyb/philosophy]], [[cyb]], [[rust]], [[cyber]]
