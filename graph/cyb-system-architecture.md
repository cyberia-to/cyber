---
tags: cyb, cyber, browser, rust, wasm, research
icon: "\U0001F310"
stake: 2603654508531670
---

# Cyb

## The Sovereign Browser. 130K Lines of Rust.

---

The web browser is the most used piece of software on Earth and the least sovereign. Chrome runs 35 million lines of C++ controlled by an advertising company. Your identity is a cookie. Your data lives on someone else's server. Your computation is metered by their rules.

Cyb replaces this with a different architecture. A browser where identity is a keypair, state lives on-chain, smart contracts run locally, and the entire render stack compiles to GPU. One binary. All platforms. 130K lines of Rust. No WebView. No V8. No Google.

---

## Part I: Render Stack — PureRender

---

### 1. Fifteen Primitives

Every interface ever built is a combination of fifteen things.

#### Content — 9 primitives

| Primitive | What it is | GPU mapping |
|-----------|-----------|-------------|
| **text** | Markdown, prose, code | Glyphs via compute shader rasterization |
| **struct** | JSON, TOML — trees and configs | Collapsible tree of text glyphs |
| **table** | 2D data, CSV | Grid of text cells, virtualized rows |
| **vector** | SVG, paths, Bezier curves | Path rasterization via Vello |
| **pixels** | Raster image | Texture upload, GPU sampler |
| **video** | Moving pixels | Hardware decode, texture per frame |
| **sound** | Waveform, audio stream | Audio pipeline (visual: waveform shader) |
| **formula** | LaTeX / MathML — math notation | Glyph layout + vector curves via Vello |
| **component** | Composition of primitives | Nested render pass |

#### Interactive — 5 primitives

| Primitive | What it is | How it maps |
|-----------|-----------|-------------|
| **action** | Tap, click, press | Any content primitive + onClick handler |
| **input text** | String, textarea, code editor | text + editable flag + cursor + IME |
| **input choice** | Select, radio, checkbox, toggle | struct (options) + selectable flag |
| **input range** | Slider, scroll, zoom | number + bounds + draggable |
| **input media** | Camera, microphone, file upload | pixels/sound + capture pipeline |

#### Layout — 4 modes

| Mode | What it is | Use case |
|------|-----------|----------|
| **stream** | Vertical sequence, scrollable | Blogs, feeds, articles, chat |
| **grid** | 2D spatial container | Dashboards, layouts, galleries |
| **flex** | 1D flexible row or column | Navbars, toolbars, card rows |
| **page** | Fixed canvas, pagination | PDF, print, scientific papers |

A button is `text` + `action`. A dashboard is `grid` of `table` + `vector`. A scientific paper is `page` of `text` + `formula` + `table`. An IDE is `grid` of `text` + `struct`. Every UI ever made is a combination of these fifteen primitives and four layout modes.

---

### 2. Flat Streams Instead of Trees

DOM is a tree. Trees breed cascading complexity: reflow, repaint, layout thrashing, z-index stacking contexts. Cyb replaces the tree with a stream.

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

A stream is a sequence of primitives, top to bottom. Each block knows its own size. Layout is trivial. Scrolling is an offset. Virtualization is free — only render the visible slice. Two levels of nesting maximum:

```
grid [sidebar: 300px, main: 1fr] {
  stream sidebar { vector { logo.svg }  struct { nav.toml } }
  stream main    { text { # Dashboard } table { sales.csv } }
}
```

Page layout for documents:

```
page [size: A4, margin: 2cm] {
  text     { # Research Paper Title }
  formula  { \nabla \times E = -\frac{\partial B}{\partial t} }
  table    { experimental_results.csv }
  vector   { figure1.svg }
}
```

Same primitives, different layout mode. This is how Cyb reads and produces PDFs, prints documents, and renders scientific papers — natively. Screen, PDF, print — same component, same pipeline, different output target.

---

### 3. Component as a Single Unit

One file. One scope. Everything inside.

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

The compiler sees the entire component. Dead CSS eliminated. Static primitives computed at compile time. Reactivity only where `state` exists.

---

### 4. Compilation: Everything to WASM + WGSL

No interpreter. No JIT. No runtime parsing. Everything compiles.

#### Strict TypeScript

**We take:** full type system, async/await, generics, interfaces, enums, unions, modules, pattern matching, destructuring.

**We leave behind:** `any`, `eval()`, `arguments`, `with`, dynamic object keys, `Proxy`, `Reflect`, prototype chains, `this` binding, runtime type checks.

No type — no compilation. Types are instructions for the AOT compiler. What V8 guesses through speculative JIT is known at compile time.

#### Pipeline

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

---

### 5. GPU Render: Everything is Shaders

Every primitive is a draw call or compute dispatch.

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

Frame loop:

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

---

### 6. Smart Contracts in the Browser

In Chrome, a dApp is a JS app that talks to a blockchain node over HTTP. In Cyb, smart contracts run locally in the same runtime as UI.

```
Chrome:     JS -> fetch() -> REST API -> Go node -> CosmWasm -> result
            Round trip: 200-2000ms

Cyb:        Component (wasmi) -> direct call -> Contract (wasmi) -> result
            Round trip: <1ms
```

#### How CosmWasm works inside Cyb

A CosmWasm contract is a `.wasm` binary with entry points:

```
instantiate(env, info, msg) -> Response
execute(env, info, msg)     -> Response
query(env, msg)             -> Binary
migrate(env, msg)           -> Response
```

The contract talks to the host through imports — functions Cyb injects into the WASM sandbox:

```
db_read(key)           -> value | null
db_write(key, value)
db_remove(key)
db_scan(start, end, order) -> iterator_id
db_next(iterator_id)   -> key, value
addr_validate(addr)    -> canonical
secp256k1_verify(hash, sig, pubkey) -> bool
ed25519_verify(msg, sig, pubkey) -> bool
query_chain(request)   -> response
```

That's the entire interface. Five database functions, crypto, chain query. Nothing else.

#### Storage

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

No Go. No Tendermint. No full node. Just Rust all the way down to SQLite.

#### The component IS the contract

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

No separation between frontend and on-chain logic. One file, one scope, one runtime. UI and state machine unified.

#### Gas metering

wasmi counts fuel per instruction. Every WASM module in Cyb — UI, contract, plugin — is metered. A malicious page can't infinite-loop. Cyb sets gas budgets per component, per contract, per page.

```
1 WASM instruction = 1 fuel unit
db_read:   ~1000 gas + 3 gas/byte
db_write:  ~2000 gas + 30 gas/byte
```

#### Contract-to-contract messaging

Contracts return messages, not direct calls. The VM processes them in order. If any fails, everything rolls back.

```
Execute contract A
  -> returns SubMsg to contract B
  -> VM executes contract B
    -> returns SubMsg to contract C
  -> all succeed: commit to SQLite
  -> any fails: rollback everything
```

This maps to PureRender reactivity:

```
Component emits action
  -> triggers contract execution (local wasmi)
  -> contract returns state changes
  -> SQLite updated
  -> reactive system detects change
  -> component re-renders affected primitives
  -> GPU draws new frame
  -> all within one 16ms frame
```

---

## Part III: Hardware Abstraction

---

### 7. Three Portable Formats

Every computer has three types of processors. Cyb has one portable format for each.

```
+-----------+-----------+------------------------------+
| Processor | Format    | What Cyb uses it for         |
+-----------+-----------+------------------------------+
| CPU       | WASM      | Logic, layout, events,       |
|           |           | contracts, state             |
+-----------+-----------+------------------------------+
| GPU       | WGSL      | Pixels, vectors, text,       |
|           |           | video, ML fallback           |
+-----------+-----------+------------------------------+
| NPU       | ONNX      | SLM legacy bridge,           |
|           |           | AI features                  |
+-----------+-----------+------------------------------+
```

Zero architecture-specific code. No x86, no ARM, no Vulkan, no Metal in the source. WASM is the only instruction set. WGSL is the only shader language. ONNX is the only model format.

```
Traditional app:   compile for x86, ARM, RISC-V x GLSL, HLSL, MSL
                   = N x M platform matrix

Cyb app:           .wasm + .wgsl + .onnx
                   = 1 artifact, runs everywhere
```

### 8. Deployment Targets

```
Browser:   WASM (native execution, browser JITs it)
           + WGSL (WebGPU API)
           + ONNX (WebNN API -> NPU)
           = opens in a URL, bypasses entire DOM/CSS engine

Desktop:   WASM (wasmi, pure Rust interpreter)
           + WGSL (wgpu -> Vulkan/Metal/DX12)
           + ONNX (burn -> wgpu GPU compute)
           = single binary, no system deps

Mobile:    WASM (wasmi)
           + WGSL (wgpu -> Metal/OpenGL ES)
           + ONNX (burn-webnn -> CoreML/NNAPI -> NPU)
           = same binary as desktop
```

Same codebase. Same pixels. Everywhere.

### 9. One GPU Layer for Everything

wgpu is the single GPU abstraction. It serves four purposes:

```
wgpu
 |
 +---> Vello        (2D rendering: text, SVG, boxes, formulas)
 +---> burn-wgpu    (ML inference: SLM, classification)
 +---> Canvas 2D    (user draw calls)
 +---> Video        (texture decode, frame display)
```

One GPU abstraction. Four uses. No duplicate compute frameworks.

### 10. NPU Access: burn-webnn

NPU hardware is fragmented — every vendor has a proprietary SDK. We solve this with a new burn backend that implements WebNN spec in Rust.

```
.onnx model (one file)
       |
       v
   burn (Rust ML framework)
       |
       +---> burn-webnn (new backend)
       |        +---> Browser: wasm-bindgen -> WebNN API -> NPU
       |        +---> macOS/iOS: FFI -> CoreML -> Apple Neural Engine
       |        +---> Windows: FFI -> DirectML -> Intel/AMD/Qualcomm NPU
       |        +---> Android: FFI -> NNAPI -> Qualcomm/MediaTek NPU
       |        +---> Linux: FFI -> OpenVINO -> Intel NPU
       |
       +---> burn-wgpu (fallback: same GPU that renders UI)
       |
       +---> burn-ndarray (last resort: CPU, always works)
```

Automatic routing:

```
burn::inference(model.onnx)
  -> NPU available?  -> burn-webnn -> NPU (max energy efficiency)
  -> no? GPU exists? -> burn-wgpu  -> GPU (max speed)
  -> nothing?        -> burn-ndarray -> CPU (always works)
```

---

## Part IV: Legacy Web Compatibility

---

### 11. SLM Legacy Bridge

A small language model (~100-300M parameters) that understands intent behind old CSS. Not rules — understanding.

```
Layer 1: Native subset       -> direct compilation, max performance
Layer 2: Legacy -> SLM       -> "float:left + width:50%" becomes "display: flex"
                              -> cached permanently, same pipeline
Layer 3: Graceful degradation -> unknown ignored, text readable
```

Runs locally via burn: NPU if available, GPU if not, CPU as fallback. Classification, not generation. Caches forever.

### 12. WASM Adoption: Run Anything

The critical challenge: existing WASM modules expect browser APIs. Cyb must run them without rewrite.

#### What WASM modules expect

```
1. wasm-bindgen (Rust -> WASM)     expects __wbindgen_* imports
2. Emscripten (C/C++ -> WASM)      expects env.emscripten_* imports
3. WASI (universal system interface) expects wasi_snapshot_preview1.*
4. CosmWasm (blockchain)            expects env.db_read, env.db_write
```

Cyb runs #4 natively. For #1-#3, an import adapter layer auto-detects what a module needs and provides it:

#### Import adapter

```
Any .wasm module arrives
       |
  Cyb inspects imports
       |
       +---> wants wasi_snapshot_preview1.*
       |     -> Cyb provides WASI shim
       |        fd_write -> console / virtual fs
       |        clock_time_get -> system clock
       |        random_get -> crypto random
       |
       +---> wants __wbindgen_*
       |     -> Cyb provides wasm-bindgen shim
       |        DOM calls -> PureRender primitives
       |        fetch -> native fetch
       |        canvas -> PureRender Canvas2D
       |
       +---> wants env.emscripten_*
       |     -> Cyb provides Emscripten compat
       |        malloc/free -> wasmi memory
       |        printf -> console
       |
       +---> wants env.db_read / env.db_write
       |     -> native CosmWasm, runs directly
       |
       +---> unknown imports
             -> SLM classifies intent, generates adapter
```

#### DOM -> PureRender translation

```
wasm-bindgen calls:                    Cyb translates to:
document.createElement("div")     ->  create stream block
element.textContent = "hello"     ->  text primitive
element.style.display = "flex"    ->  flex layout container
element.appendChild(child)        ->  append to stream
canvas.getContext("2d")           ->  PureRender Canvas2D
element.addEventListener("click") ->  action primitive
fetch(url)                        ->  native fetch
localStorage.setItem(k,v)        ->  SQLite write
document.cookie                   ->  cyb.identity.pubkey
```

Not pixel-perfect DOM compat — intent-based translation. Same philosophy as the SLM legacy CSS bridge.

#### Adoption tiers

```
Tier 1: Works immediately (push and run)
  Pure computation WASM (crypto, math, parsing)
  WASI modules (CLI tools compiled to WASM)
  CosmWasm contracts

Tier 2: Works with auto-adaptation (~80% of Rust-to-WASM)
  Simple wasm-bindgen modules (fetch, basic DOM)
  Cyb auto-detects imports, provides shims

Tier 3: Works with SLM assistance (~60-70% of all WASM)
  Complex DOM manipulation
  Heavy Canvas usage
  SLM analyzes patterns, generates adapters, caches

Tier 4: Won't work (~10-20%)
  WebGL-heavy games expecting raw GPU access
  Modules deeply coupled to Chrome-specific APIs
  Full POSIX emulation via Emscripten
```

#### Web API upgrades

When Cyb translates legacy Web APIs, it doesn't just emulate — it upgrades:

```
Chrome concept         ->  Cyb upgrade
Cookie / session       ->  Cryptographic identity (keypair)
localStorage           ->  SQLite (queryable, persistent)
fetch to REST API      ->  Direct contract call (local, <1ms)
IndexedDB              ->  SQLite with Merkle proofs
Service Worker cache   ->  IPFS / local content-addressed storage
OAuth / JWT            ->  Cryptographic signatures
WebSocket to server    ->  P2P libp2p connection
```

---

## Part V: Two Scripting Layers

---

### 13. TypeScript (strict) — For Applications

Developers write components, business logic, UI. AOT compiled to WASM. Millions already know it. Zero learning curve.

### 14. Rune — For the Engine Itself

Internal Rust-native scripting:

- **Hot reload** — configs, plugins, themes without recompilation
- **Dev console** — REPL, DOM inspection, profiling
- **Engine extensions** — custom primitives, formatters
- **Legacy transpilation** — SLM generates Rune scripts for old CSS/DOM
- **Sandboxed** — safe execution, near-free Rust interop

Like Lua in game engines, but with Rust types and async.

---

## Part VI: Cyb vs Chrome

---

### 15. Two Different Architectures for Two Different Webs

```
                          Chrome              Cyb
Codebase                  35M lines C++       130K lines Rust
WASM execution            V8 JIT              wasmi interpreter
Render engine             Blink (DOM/CSS)     PureRender (15 primitives)
Identity                  Cookie              Keypair
State                     Server-side         Local SQLite + on-chain
Smart contracts           Via HTTP to node    Native, same runtime as UI
Storage                   IndexedDB           SQLite + Merkle proofs
Trust model               Sandboxed           Sandboxed + gas metered
Execution                 Non-deterministic   Deterministic
Security surface          ~35M lines C++      ~130K lines Rust
GPU access                WebGL/WebGPU        WGSL via wgpu (render + ML)
NPU access                WebNN               burn-webnn (browser + native)
PDF                       pdf.js (JS)         Native, same pipeline
Formulas                  MathJax (JS)        Native, Vello
Binary size               ~150+ MB            ~10 MB
```

### 16. Speed: The Honest Assessment

wasmi interpreter is slower than V8 JIT. But not 10x slower — that's an outdated myth from stack-based wasmi. Register-based wasmi 2.0 is realistic:

```
Workload                  V8 JIT    wasmi 2.0    Real impact
Short contract call       0.05ms    0.08-0.15ms  Imperceptible
UI logic per frame        0.5ms     1-2ms        Within 16ms budget
JSON parsing              0.1ms     0.3-0.5ms    Imperceptible
Crypto (sig verify)       0.5ms     1.5-2.5ms    Acceptable
Heavy computation         50ms      150-250ms    Noticeable, rare
Tight numeric loop        1x        8-15x        Worst case, synthetic
```

For typical use — contract calls, UI updates, data parsing — the difference is **2-3x, invisible to the user**. The bottleneck is GPU rendering and network latency, not CPU interpretation.

wasmi advantages over V8:
- No JIT warmup — predictable from first call
- Deterministic — same input = same output = same fuel cost
- Tiny attack surface — no JIT compiler exploits
- Gas metering — built-in, not bolted on

### 17. What We Don't Take

| Skip | Why |
|------|-----|
| V8 / SpiderMonkey | wasmi — deterministic, metered, secure |
| Blink / Gecko | PureRender — 15 primitives, GPU-native |
| DOM tree | Flat streams |
| CSS cascade | Scoped styles per component |
| Shadow DOM | Component = automatic scoping |
| 30 input types | 5 interactive primitives |
| Quirks mode | One rendering mode |
| Cookies | Cryptographic keypairs |
| IndexedDB | SQLite |
| pdf.js | Native PDF, same pipeline |
| MathJax/KaTeX | Native formulas |
| Electron/WebView | No OS dependencies |
| Cranelift/LLVM | WASM is the target, not native code |
| ONNX Runtime (C++) | burn (pure Rust) |
| Chromium process model | wasmi sandboxing + gas metering |

---

## Part VII: Numbers

---

### 18. 130K Lines

```
Module                                        Lines of Rust

PureRender (100K)
───────────────────────────────────────────────────────────
15 primitives (parse, layout, render, interact)
  text + struct + table                          6,000
  vector (Vello core, stripped)                  12,000
  pixels + video                                 4,000
  sound                                          3,000
  formula (LaTeX -> Vello)                       4,000
  component system + reactivity                  6,000
  5 interactive primitives                       5,000
                                                 40,000

TS strict compiler
  Parser (stripped SWC fork)                    12,000
  Type checker (strict subset)                   5,000
  AOT backend -> WASM                            5,000
                                                 22,000

Runtime
  wgpu setup + render loop                       5,000
  Layout engine (Taffy core + stream + page)     8,000
  Event loop + input handling                    4,000
  Platform layer (winit, clipboard, IME)         5,000
                                                 22,000

Infrastructure
  PDF read/write                                 5,000
  Networking (fetch, WebSocket)                  2,000
  SQLite integration                             1,500
  Rune bridge + plugin API                       3,000
  SLM legacy bridge                              2,000
  Dev tools (inspector, REPL)                    3,500
                                                 17,000

CosmWasm integration (20K)
───────────────────────────────────────────────────────────
  cosmwasm-vm (wasmi integration, gas)           8,000
  cosmwasm-std (types, traits)                   5,000
  cw-storage-plus (typed abstractions)           3,000
  Host impl (db -> SQLite, crypto)               3,000
  State sync (light client, proofs)              5,000
  Component bridge (contract state -> UI)        2,000
                                                 26,000

WASM adoption layer (10K)
───────────────────────────────────────────────────────────
  WASI shim                                      3,000
  wasm-bindgen adapter                           4,000
  DOM -> PureRender translation                  5,000
  Emscripten compat                              2,000
                                                 14,000

burn-webnn (12K)
───────────────────────────────────────────────────────────
  Core trait implementation                      3,000
  Browser WebNN bindings                         1,500
  Native adapters (CoreML, DirectML, NNAPI)      8,000
                                                 12,500

───────────────────────────────────────────────────────────
TOTAL                                          ~130,000
```

### Context

```
Cyb:            ~130K lines    (sovereign browser, full stack)
Zed editor:     ~200K lines    (just a text editor)
SQLite:         ~150K lines    (just a database)
Chromium:    35,000,000 lines  (270x more)
Firefox:     25,000,000 lines  (190x more)
```

130K lines. One engineer reads the whole codebase in a week. That's the test. If one person can't hold it in their head, there's junk left.

---

### 19. Timeline

```
Month 1-3:    Skeleton — 10K lines, 2-3 people
              Window + wgpu + Vello + "hello world" stream
              text + vector primitives rendering
              wasmi running basic contracts

Month 4-8:    Core — 40K lines, 4-6 people
              All 15 primitives
              Component system
              CSS subset, page layout, PDF
              CosmWasm contracts executing locally

Month 9-14:   Compiler + Contracts — 80K lines, 5-8 people
              Strict TS -> WASM AOT
              Interactive primitives
              Contract-component bridge
              WASI + wasm-bindgen adoption shims

Month 15-20:  Intelligence + Polish — 130K lines, 5-8 people
              burn-webnn backend
              SLM legacy bridge
              DOM -> PureRender adapter
              Browser deployment (WASM + WebGPU)
              Mobile platform layer
              Dev tools, Rune REPL
              Performance, accessibility
```

### Why NOW

Five years ago, none of this was possible:

- `wgpu` didn't exist (2020)
- `vello` was experimental
- `taffy` wasn't production-ready
- `SWC` wasn't mature for TS parsing in Rust
- `burn` didn't exist (2022)
- `wasmi` 2.0 register-based wasn't done
- WebGPU wasn't shipping in browsers
- WebNN didn't exist
- Small language models didn't exist
- CosmWasm hadn't moved to wasmi 2.0
- WASM component model wasn't standardized

Today every brick is ready. The window is open.

---

## Part VIII: Architecture

---

### 20. The Full Stack

```
+──────────────────────────────────────────────────────+
|                     SOURCE                            |
|  TS strict + HTML + CSS + SVG + MD + JSON + TOML     |
|  + LaTeX + Rust (contracts)                          |
+──────────────────────┬───────────────────────────────+
                       |
          +────────────+────────────+
          |   AOT Compiler (Rust)   |
          |   parse -> IR -> opt    |
          +────────┬──────┬────────+
                   |      |
          +────────+  +───+────+
          |  WASM  |  |  WGSL  |
          |        |  |        |
          | UI     |  | text   |
          | layout |  | vector |
          | events |  | pixels |
          | state  |  | video  |
          | fetch  |  | canvas |
          | SQLite |  | formula|
          | audio  |  | compose|
          | PDF    |  |        |
          +───┬────+  +───┬────+
              |           |
+─────────────+───────────+────────────────────────────+
|                      wgpu                             |
|           One GPU layer for everything                |
|  Vello (render) + burn (ML) + Canvas + Video          |
+──────────────────────────────────────────────────────+
|                    burn-webnn                          |
|           NPU auto-routing via ONNX                   |
+──────────────────────┬───────────────────────────────+
                       |
+──────────────────────+───────────────────────────────+
|                    wasmi                               |
|        One WASM runtime for everything                |
|  UI components + CosmWasm contracts + plugins          |
|  Gas metered + sandboxed + deterministic               |
+──────────────────────┬───────────────────────────────+
                       |
+──────────────────────+───────────────────────────────+
|              WASM Adoption Layer                       |
|  WASI shim + wasm-bindgen + Emscripten + DOM adapter  |
|  Push any .wasm, it runs                              |
+──────────────────────┬───────────────────────────────+
                       |
+──────────────────────+───────────────────────────────+
|                  CosmWasm                              |
|  Contracts + SQLite storage + state sync              |
|  Light client + Merkle proofs + P2P broadcast         |
+──────────────────────┬───────────────────────────────+
                       |
       +───────────────+───────────────+
       |               |               |
+──────+──────+ +──────+──────+ +──────+──────+
|   Browser   | |   Desktop   | |    Mobile   |
|             | |             | |             |
| WASM:native | | WASM:wasmi  | | WASM:wasmi  |
| WGSL:WebGPU | | WGSL:Vulkan | | WGSL:Metal  |
| ONNX:WebNN  | |      Metal  | |      GLES   |
|             | |      DX12   | | ONNX:CoreML |
|             | | ONNX:burn   | |      NNAPI  |
+─────────────+ +─────────────+ +─────────────+
```

### 21. The Formula

```
15 primitives
  x 4 layout modes
  x strict TS -> AOT -> WASM
  x WGSL (portable GPU)
  x ONNX (portable NPU)
  x wasmi (one runtime: UI + contracts + plugins)
  x CosmWasm (native smart contracts)
  x burn + wgpu (one GPU: render + ML)
  x burn-webnn (NPU on all platforms)
  x Rune (engine scripting)
  x SLM (legacy bridge)
  x WASM adoption layer (run anything)
  = 130K lines of Rust
  = sovereign browser
```

Not a browser that connects to blockchain. **A browser where blockchain is the runtime.** Identity is a keypair. State is on-chain. Contracts execute locally. UI renders from the same primitives. Deterministic, metered, verifiable. 130K lines. One engineer reads it in a week.

The browser you can trust because you can read every line.
