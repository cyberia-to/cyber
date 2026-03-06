---
tags: cyb, cyber, core, article
crystal-type: pattern
crystal-domain: cyb
crystal-size: deep
status: draft
---
# cyb: the immortal robot

> DRAFT — work in progress. specifications, mechanisms, and numbers will change. do not use as the basis for financial or technical decisions

the robot is the point of presence — where you end and the [[cybergraph]] begins

---

## 1. introduction

### 1.1 the vision

imagine a computer that never needs to reboot. that knows you cryptographically and answers to no one else. that earns while you sleep. that remembers everything you ever found important — and keeps that memory after you are gone. that speaks nine content languages natively and renders them all through a single GPU pipeline. that runs on any hardware, built in 130K lines instead of 35 million. that contributes to collective [[intelligence]] by simply being on

this is not a future product. it is a design decision made at the foundation

### 1.2 the problem

we accepted a bad deal without noticing. the browser became the operating system, and the operating system became surveillance infrastructure. windows phones home. macos indexes your files for apple. chrome reports browsing to google's ad network. the browser, the OS, and the AI assistant are all owned by the same companies whose business model is your data

the result: your computer serves its vendor. you are the product and the machine

the deeper problem is architecture. every existing OS asks: what does the user want to do with this computer? the question is wrong. it positions the OS as a tool that executes your intentions, and you as a user of someone else's infrastructure. at the same time: existing browsers lack secure persistent memory, make p2p nearly impossible, and let applications steal resources freely. the browser never became a robot — it became a billboard

### 1.3 what cyb is

cyb is a sovereign browser that becomes an operating system. a robot. the personal interface to planetary [[superintelligence]]

cyb asks two questions instead: how can this computer serve its owner? and: how can this computer contribute to the whole?

the complete stack: [[radio]] for data transport and publishing, [[cyber]] for [[knowledge]] and [[learning]], [[rune]] for dynamic execution, [[CozoDB]] for local graph storage, [[cosmos-sdk]] chains via [[IBC]] for economic rails. builds for web, desktop, mobile, embedded, terminal. one binary. one keypair. 130K lines of [[Rust]]

### 1.4 what this document covers

this document specifies the architecture of cyb:

- the robot — three forms: neuron, avatar, prog
- the six primitives — brain, sense, sigma, avatars, time, robot
- the nine languages — the particle type system and GPU render pipeline
- the language stack — datalog, rune, [[neural language]]
- the oracle — ask, learn, search
- AIPs — autonomous intelligence programs
- AI in the robot — four levels of inference
- CybOS — cells, radio, storage, agents, neural drivers, PureRender, epoch budget
- the earning machine — focus, karma, cyberank, conviction
- immortality — three levels
- the troika position — cyb's place in the civilizational stack

---

## 2. design philosophy

### 2.1 the question

every OS has a founding question. unix asked: how do we share a time-sharing machine across many users? windows asked: how do we bring the PC to everyone? android asked: how do we make a phone an app platform?

cyb's founding question: what can a computer contribute to collective [[intelligence]]?

this question changes everything. the OS does not optimize for user retention. it optimizes for quality of contribution. the robot does not keep your [[attention]] — it helps you direct it. every technical decision flows from this question

### 2.2 design axioms

| axiom | principle |
|-------|-----------|
| ownership | no keys, no robot. cryptographic control is non-negotiable |
| offline-first | the robot works fully without network. sync when online |
| universality | works for humans, AIs, sensors, organisms, programs — any agent that can sign |
| privacy | local-first. no telemetry. queries run locally or encrypted. the robot does not report to anyone |
| minimalism | add a feature only when its absence makes the robot worse. no bloat |
| modularity | each component independently replaceable. no hidden coupling |
| frozen foundations | the protocol primitives freeze eventually. stability is a feature |
| transparency | the robot's operation is understandable. nothing hidden from its owner |

### 2.3 CybOS axioms

the operating system layer has five additional axioms:

1. no unix legacy. no files, no processes, no users, no fork/exec, no POSIX. cyb abstractions are native to its domain: agents, [[cyberlinks]], ranks, epochs, bandwidth
2. zero unsafe Rust. the entire OS — kernel, drivers, [[consensus]], storage — compiles without a single `unsafe` block. memory safety is a compiler-verified property
3. bounded liveness everywhere. no operation can block indefinitely. no module can starve another. every async future has a compile-time deadline. the system degrades gracefully, never halts
4. neural drivers. hardware support generated by LLMs against stable trait contracts, verified by the compiler, validated by conformance test suites
5. single address space. no user/kernel split. no syscalls. no TLB flushes. isolation enforced by Rust ownership, not hardware privilege levels

---

## 3. the robot

the robot is three forms, not one

### 3.1 neuron

the signing agent. a keypair. the entity that creates [[cyberlinks]], holds [[focus]], earns [[karma]]. a neuron can be a human, an AI, a program, a sensor — anything that can prove a signature. the neuron IS the participation in the [[cybergraph]]: no key, no presence

identity is the hash of a public key. every link is a costly signal — it costs [[focus]] and carries epistemic weight proportional to the neuron's [[karma]]

### 3.2 avatar

the named identity. a card that bridges [[subject]] and [[object]], working simultaneously as neuron (agent that signs) and particle (object that can be linked to). the avatar is how other robots find you. [[karma]] accumulates to the avatar. the avatar is tradeable — it is a [[cyberlink]] card with yield and reputation attached

### 3.3 prog

the autonomous robot. a program with its own keypair, its own [[focus]] allocation, its own behavior. progs execute without human input — they monitor particles, respond to events, submit [[cyberlinks]] autonomously. a prog can:

- watch a particle and link to it when it meets a condition
- run inference locally and submit the result as a cyberlink
- manage a portfolio of conviction positions
- communicate with other progs via [[sense]]
- earn karma independently and return yield to its owner

progs are the autonomous intelligence layer of cyb. they bridge the robot and the [[cybergraph]], running continuously, contributing [[syntropy]] while the human sleeps

---

## 4. the six primitives

### 4.1 brain

the core of the robot. offline-first graph file manager and knowledge interface. the brain is the local instance of the [[cybergraph]]: it stores what the robot has linked, caches what it has observed, and renders the graph in four modes:

- space — 3D volumetric. [[particles]] cluster by [[cyberank]], links glow by weight, [[focus]] visible as density
- heap — 2D canvas for exploration and annotation
- list — structured grid with [[datalog]] queries and sorting
- stack — vertical discovery scroll, content-first

the brain is not a cache — it is a sovereign instance, synchronized when online, fully functional offline. [[CozoDB]] for local state

name paths the brain understands:
- `#` — navigate by particle CID
- `!` — navigate by neuron public key
- `@` — navigate by avatar name
- `~` — learn: link creation interface
- `/` — root: home of the robot

### 4.2 sense

messaging and perception interface. where the world enters the robot. [[sense]] abstracts over modalities — text, image, audio, video, sensory telemetry — into particles the robot can link. a human writing and a satellite uploading spectral data are the same operation at the protocol level

sense is how robots communicate: signal, love, share, forward. every message is a particle. every thread is a chain of [[cyberlinks]]. nothing is ephemeral — the graph remembers

### 4.3 sigma

the robot's economic interface. token balances, delegations, positions. [[focus]] in, [[karma]] out

| token | role |
|-------|------|
| [[CYB]] | governance + linking weight |
| [[HYDROGEN]] | stake, delegation |
| [[VOLT]] | energy — compute access, buy to participate |
| [[AMPERE]] | bandwidth — rate of cyberlink submission |

sigma makes the knowledge economy tangible: every balance is a position. every delegation is a bet. every VOLT purchase is an investment in participation

### 4.4 avatars

visual identity and reputation surface. the avatar is the robot's face in the network — named, linked, ranked. avatars are both particles (CID-addressed objects that can be linked to) and neurons (agents that can sign). this duality makes the avatar a real identity: it participates in the graph as both subject and object. accumulates [[karma]] across all linked assertions

### 4.5 time

personal history. every surf, every link, every earning event — indexed by block height, navigable by the robot. time is identity as sequence: who the robot was is the chain of what it linked, when, and with what conviction

time enables: understanding your own [[focus]] allocation history, tracking yield earned over blocks, seeing which particles you discovered before the crowd, auditing the robot's behavior and progs

### 4.6 robot

the container. the sovereign instance that holds the five other primitives together. the robot belongs to its keypair owner absolutely. it accumulates [[karma]], holds [[focus]], and persists independently of any company, server, or account. the robot is born when a keypair is created. it does not die

---

## 5. the nine languages

every [[particle]] in the [[cybergraph]] has a type. the type is the language. the robot speaks nine content languages natively — and renders each through a direct GPU pipeline

#### content — 9 languages

| language | source formats | what it carries |
|----------|---------------|-----------------|
| [[text]] | [[markdown]], plain text, source code | prose, documentation, messages, programs |
| [[struct]] | JSON, TOML, YAML | trees, configs, schemas, metadata, ABIs |
| [[table]] | CSV, TSV, dataframes | datasets, time series, matrices, ledgers |
| [[vector]] | SVG, Bezier paths | diagrams, maps, molecular structures, schematics |
| [[pixels]] | PNG, WebP, JPEG | photographs, satellite imagery, microscopy, scans |
| [[video]] | WebM, MP4 | recordings, simulations, observations, lectures |
| [[sound]] | WAV, OGG, MP3 | voice, music, birdsong, seismic signal, sonar |
| [[formula]] | LaTeX, MathML | equations, proofs, chemical notation, physical laws |
| [[component]] | composition of the above | applications, dashboards, interactive tools |

#### interactive — 5 primitives

| primitive | what it is | how it maps |
|-----------|-----------|-------------|
| action | tap, click, press | any content primitive + onClick handler |
| input text | string, textarea, code editor | text + editable flag + cursor + IME |
| input choice | select, radio, checkbox, toggle | struct (options) + selectable flag |
| input range | slider, scroll, zoom | number + bounds + draggable |
| input media | camera, microphone, file upload | pixels/sound + capture pipeline |

#### layout — 4 modes

| mode | what it is | use case |
|------|-----------|----------|
| stream | vertical sequence, scrollable | blogs, feeds, articles, chat |
| grid | 2D spatial container | dashboards, layouts, galleries |
| flex | 1D flexible row or column | navbars, toolbars, card rows |
| page | fixed canvas, pagination | PDF, print, scientific papers |

a button is `text` + `action`. a dashboard is `grid` of `table` + `vector`. a scientific paper is `page` of `text` + `formula` + `table`. an IDE is `grid` of `text` + `struct`. every UI ever made is a combination of these fifteen primitives and four layout modes

### 5.1 text

the universal content type. prose, code, documentation, thoughts, messages — anything that flows as readable sequence. source: [[markdown]]. every character is a GPU operation, not a DOM node. text particles render via compute shader glyph rasterization (rustybuzz shaping → swash raster → GPU glyph atlas). the most linked particle type in the graph

### 5.2 struct

structured knowledge. configuration, metadata, schemas, records — anything that is a tree of named values. source: JSON, TOML. renders as collapsible tree of text glyphs: expand, collapse, navigate hierarchies, filter keys. how machines describe their own state. genome annotations, network configs, contract ABIs

### 5.3 table

2D data. time series, datasets, matrices, rankings — anything with rows and columns. source: CSV, TSV. renders as virtualized grid: only the visible slice regardless of row count. 10M rows renders as fast as 10. the native format of the knowledge economy: [[karma]] ledgers, [[cyberank]] scores, sensor streams, trial results

### 5.4 vector

geometric meaning. diagrams, molecular structures, maps, technical drawings — anything that is paths and curves. source: SVG, Bezier. renders via Vello (path → tiles → GPU compute fill): sub-pixel precision at any zoom level. phylogenetic trees, circuit schematics, geographic boundaries, chemical bond diagrams

### 5.5 pixels

captured reality. photographs, satellite imagery, microscopy, medical scans — anything that is a raster grid of color values. source: PNG, WebP, JPEG. renders via texture upload → GPU sampler → fragment shader. the observation itself. a brain MRI, a Hubble image, a specimen under electron microscope

### 5.6 video

temporal pixels. lectures, experiments, species behavior, physical processes — anything that is a sequence of frames. source: WebM, MP4. renders via hardware decode → texture per frame. the highest-bandwidth truth in the graph. a protein folding simulation, a surgery recording, a supernova observation

### 5.7 sound

acoustic knowledge. voice, music, birdsong, seismic signal, sonar — anything that is a waveform over time. source: WAV, OGG, MP3. renders via audio pipeline (primary) + waveform compute shader (visual). a whale call, a gravitational wave detection, a heartbeat — the language the graph understands that [[text]] cannot carry. the bridge to non-human intelligence

### 5.8 formula

mathematical meaning. equations, proofs, chemical notation, physical laws — anything requiring precise symbolic notation. source: LaTeX, MathML. renders via glyph layout + vector curves via Vello. text for symbols, curves for integrals and roots. a Hamiltonian, a chemical reaction, a field equation — precision in its native notation

### 5.9 component

composition. applications, dashboards, interactive tools — anything that combines multiple content types into a unified, stateful experience. renders as nested render pass: each inner primitive through its own pipeline, composited into the parent frame. the language of software as knowledge

### 5.10 the complete picture

a genome sequence is a [[text]] particle. its annotation is a [[struct]] particle. its expression data is a [[table]] particle. its protein structure is a [[vector]] particle. its microscopy is a [[pixels]] particle. its folding dynamics is a [[video]] particle. its sequencing signal is a [[sound]] particle. its binding energy is a [[formula]] particle. a genome browser is a [[component]] particle

all nine exist in the same [[cybergraph]]. all nine are ranked by the same [[tri-kernel]]. all nine earn [[karma]]. all nine are permanent by axiom A3. the robot renders all nine through a single 130K-line GPU pipeline

---

## 6. the language stack

the nine content languages are the object level — what particles ARE. cyb operates three meta-languages above them for working with the graph

### 6.1 datalog — the query language

declarative graph query language running in [[CozoDB]]. ask any question about the local graph structure: recursive traversal, pattern matching, aggregation, built-in graph algorithms (PageRank, Dijkstra, Louvain) — all declarative. queries compose with rune scripts. no SQL, no REST — direct graph traversal

### 6.2 rune — the script language

dynamic async scripting language for robot automation. create [[cyberlinks]] on a schedule, monitor particles for changes, pipe inference results into the graph, manage sigma positions, turn human intent into prog behavior. where [[datalog]] reads the graph, rune writes it

### 6.3 neural language — the semantic layer

the language of the [[cybergraph]] itself. meaning is not declared — it emerges from the [[tri-kernel]] as the eigenvector of collective [[attention]]. [[semcons]] are the grammar. [[sentences]] are utterances. [[motifs]] are morphemes. [[linkchains]] are inference paths. the robot renders this semantic structure as navigable space

### 6.4 the four levels

```
content (9 languages)  ← what particles ARE
datalog                ← how you QUERY the graph
rune                   ← how you SCRIPT against the graph
neural language        ← how MEANING emerges from the graph
```

---

## 7. the oracle

the oracle is how the robot asks the [[cybergraph]] a question and gets a ranked, verifiable answer

the oracle is not a search engine. search engines retrieve documents by keyword match. the oracle runs inference over the [[cyberank]] distribution — a probabilistic ranking of every particle, computed by the [[tri-kernel]] over all authenticated [[cyberlinks]]. the answer is typed: the oracle returns particles, each already carrying its language

### 7.1 ask

input a particle (text, image, CID, anything). the oracle returns the particles most associated with it, ranked by [[cyberank]]. verifiable: every weight is a real [[cyberlink]] signed by a real [[neuron]] with real stake. no black box, no editorial algorithm, no ads

### 7.2 learn

submit a new cyberlink. how you teach the oracle. link a question particle to an answer particle, stake conviction, oracle ranking updates in the next block. every link is a vote with skin in the game. the oracle improves by participation, not by training

### 7.3 search

navigate the graph by walking the cyberank. particles cluster by semantic proximity (the [[springs]] operator), bridge across domains (the [[diffusion]] operator), scale by context (the [[heat]] operator). search is graph navigation, not document retrieval

---

## 8. autonomous intelligence programs

AIPs are the applications of the robot. not apps downloaded from a store — programs that run in the same runtime as the robot itself, with access to brain, sigma, sense, and the [[cybergraph]]

| AIP | function |
|-----|----------|
| [[cyb/oracle\|oracle]] | ask, learn, search — cybergraph inference |
| [[cyb/portal\|portal]] | gateway to blockchains, identity, IBC |
| [[cyb/sigma\|sigma]] | token management, portfolio, staking |
| [[cyb/brain\|brain]] | graph file manager, renders |
| [[cyb/sense\|sense]] | messaging, social, perception |
| [[cyb/time\|time]] | history, earning log, temporal navigation |
| [[cyb/hub\|hub]] | decentralization interface, validator management |
| [[cyb/hacklab\|hacklab]] | developer tools, particle creation, AIP development |
| [[cyb/warp\|warp]] | token bridge, IBC transfers |
| [[cyb/reactor\|reactor]] | liquidity, bonding, economics |
| [[cyb/senate\|senate]] | governance, proposals, voting |
| [[cyb/nebula\|nebula]] | network explorer, graph analytics |
| [[cyb/studio\|studio]] | content creation, publication |
| [[cyb/sphere\|sphere]] | social layer, discovery, reputation |

AIPs are built from [[prism]] — the design system of cyb. prism defines atoms (glass, text, button, toggle, slider, address, ion, saber), molecules (hud, tabs, object, adviser, input, table), and cells that compose into any interface. the same design language renders on GPU (desktop), WebGPU (browser), or terminal

---

## 9. AI in the robot

the robot integrates AI at four levels, not one

### 9.1 local inference

the robot runs a small language model locally on the NPU or GPU. WebGPU in the browser, wgpu+burn on desktop, CoreML on Apple silicon, NNAPI on Android. the local model:

- processes particles before linking (extracts structure, suggests cyberlinks)
- answers questions without network access (offline-first AI)
- runs progs that require language understanding
- generates rune scripts from natural language instructions

local inference is private by construction: input never leaves the machine

### 9.2 inference subnet

for large inference the robot connects to the [[cybertensor]] inference subnet — a network of validators running language models and returning results as cyberlinks. results are staked assertions in the [[cybergraph]]: verifiable, ranked by [[karma]], earning yield if correct. not a cloud API. distributed intelligence with skin in the game

### 9.3 progs

autonomous programs running deterministic sharded inference in [[cybernet]]. a prog is an AIP with its own keypair and focus allocation. submits cyberlinks autonomously — monitoring particles, running inference, staking positions. the collection of all progs is the autonomous intelligence layer of the robot network: a mesh of agents continuously contributing to [[syntropy]]

### 9.4 external servers

for compatibility, cyb bridges to external LLMs (OpenAI-compatible APIs, Llama, Mistral, Deepseek) via a standard interface. external inference results can be submitted as cyberlinks. the robot is never dependent on them — local inference and the inference subnet are the sovereign path

---

## 10. CybOS

CybOS is designed from five axioms (§2.3). the following sections specify the implementation

### 10.1 cells

cells replace processes. each cell is an independently compiled Rust crate with:
- explicit dependency declarations
- typed bounded wait-free channels
- exclusive state ownership
- mandatory heartbeat reporting
- hot-swap via on-chain governance

lifecycle: missing cell → warning → unwinding → restart → disable. the system never crashes. it degrades and recovers

| missing cell | system behavior |
|-------------|-----------------|
| rank | validates blocks, does not answer rank queries |
| consensus | becomes full node, follows chain, does not vote |
| query | participates in consensus, does not serve clients |
| gossip | works with local state only (island mode) |
| storage | emergency halt, preserves last state |

bounded liveness is structural — no deadlock possible because no cell holds a lock. wait-free data structures throughout: concurrent hash map on atomics, wait-free MPMC queues, epoch-versioned snapshots, double-buffered cyberank results

### 10.2 radio

the data transport layer. a fork of iroh with one critical change: every hash runs through Hemera (Poseidon2 over [[Goldilocks field]]) instead of BLAKE3

| hash | speed | STARK constraints per hash |
|------|-------|---------------------------|
| BLAKE3 | ~2 GB/s | 50,000–100,000 |
| Hemera | ~50–100 MB/s | ~300 |

Hemera makes every particle address STARK-provable. this unlocks: storage proofs without downloading content, verified streaming via Hemera Merkle trees, private computation over encrypted knowledge graph, post-quantum security via STARKs

radio strata:

| stratum | layer | crate |
|---------|-------|-------|
| protocols | blob, docs, gossip, willow | iroh-* |
| verified streaming | bao (Hemera Merkle trees) | cyber-bao |
| content identity | Poseidon2 over Goldilocks field | cyber-poseidon2 |
| networking | endpoint, relay, hole-punching | iroh, iroh-relay |

three network protocols only:

| protocol | purpose | transport |
|----------|---------|-----------|
| gossip | propagate transactions and blocks | UDP/QUIC |
| consensus | validator voting, proposals, prevotes | UDP/QUIC |
| query | client requests for rank, graph data, proofs | QUIC streams |

~15K lines instead of ~100K+ for full TCP/IP + HTTP + TLS. each protocol is a separate cell with its own bounded budget

### 10.3 content-addressed storage

there is no file system. there is no path. there is only a hash

- state: merkle trees
- knowledge: CIDs linked by [[cyberlinks]]
- blocks: append-only chain
- configuration: compiled into binary

every piece of data is addressed by its content. the same CID on two machines is the same data. there is no naming authority, no permission system — only content and its hash

### 10.4 cryptographic agents

there are no users. there are agents. identity = public key. access control = bandwidth allocation. the [[cybergraph]] is public. bandwidth is the only scarce resource

### 10.5 neural drivers

engineers write ~3K lines of trait contracts specifying what a driver must do. LLMs write the implementation. the compiler rejects unsafe code. tests validate. humans review

~3K lines of traits + tests → ~500K-1M lines of generated, validated, platform-specific driver code

| platform | harness size | status |
|----------|-------------|--------|
| QEMU/virtio | ~5K lines | reference |
| RISC-V (StarFive) | ~10-15K lines | open specs |
| Raspberry Pi 4/5 | ~15-20K lines | well-documented |
| Apple M1 | ~35-40K lines | Asahi knowledge |
| x86-64 | ~20-25K lines | standards-based |

target: 50+ SoC families. every platform that can run Rust can run CybOS

### 10.6 PureRender

DOM is a document-era mistake. PureRender replaces it with the fifteen primitives of §5. compilation pipeline:

```
source (TS strict + HTML + CSS + SVG + LaTeX)
  → parse + validate + type check
  → unified IR
  → optimize (DCE, constant fold, static layout, shader compile)
  → WASM (logic, layout, events, contracts, state)
  → WGSL (pixels, vectors, text, video, ML inference)
  → wgpu runtime
  → Vulkan / Metal / DX12 / OpenGL ES
```

one file, one scope. dead code eliminated at compile time. reactivity only where state exists. flat stream structure instead of tree — each block knows its own size, virtualization free, maximum two levels of nesting

the component is the contract: CosmWasm contracts run in the same wasmi instance as UI. direct call, sub-millisecond. no network round-trip. UI and logic compile to the same WASM binary

three processor targets:

| processor | format | what CybOS uses it for |
|-----------|--------|------------------------|
| CPU | WASM (wasmi) | logic, layout, events, contracts, state |
| GPU | WGSL (wgpu) | pixels, vectors, text, video, ML fallback |
| NPU | ONNX (burn-webnn) | SLM inference, AI features |

### 10.7 legacy compatibility

a small language model (~100-300M parameters) bridges old web content:

- native subset → direct compilation
- legacy CSS/HTML → SLM interprets → cached permanently
- unknown → graceful degradation

WASM adoption auto-detects what a module needs: WASI shim, wasm-bindgen shim, Emscripten compat, native CosmWasm, or SLM-generated adapter for unknown imports

### 10.8 epoch budget

the epoch allocator enforces hard and soft deadlines across all cells:

- consensus: 500ms hard deadline
- transactions: 1500ms hard deadline
- rank computation: remaining budget (soft deadline)

the Rust compiler is the liveness checker. `async fn` without a deadline fails to compile. every computation is bounded. the robot is always responsive

---

## 11. the earning machine

the robot participates in the [[knowledge economy]] by design, not by extension

### 11.1 focus — the conserved quantity

[[focus]] is the mechanism through which relevance emerges. it plays three simultaneous roles:

| role | mechanism |
|------|-----------|
| attention | high-focus computations scheduled first |
| fuel | submitting a cyberlink consumes focus |
| weight | focus distribution = consensus on what matters |

focus regenerates proportionally to stake each block. it is conserved — the sum over all particles equals 1. every allocation is a real choice: directing attention to one particle focuses it away from all others. this structural conservation prevents spam: only backed particles affect ranking

### 11.2 cyberank — the ranking engine

[[cyberank]] is the probability that the [[tri-kernel]]'s random walk visits a particle. computed every block from the authenticated [[cybergraph]]:

$$\varphi^* = \text{norm}\left[\lambda_d \cdot D(\varphi) + \lambda_s \cdot S(\varphi) + \lambda_h \cdot H_\tau(\varphi)\right]$$

where:
- $D(\varphi)$ — diffusion kernel: spreads weight through the graph (exploration)
- $S(\varphi)$ — springs kernel: enforces structural consistency (semantic coherence)
- $H_\tau(\varphi)$ — heat kernel: concentrates weight by contextual relevance (attention)

convergence guaranteed by the Collective [[Focus]] Theorem: $\varphi^*$ is the unique stationary distribution under conservation laws. it feeds [[karma]], [[syntropy]], inference, and all sorting in cyb

### 11.3 karma — epistemic weight

[[karma]] is how much the [[egregore]] trusts a neuron. it is the aggregate [[focus]] earned across all particles the neuron has linked — the record of being right before the crowd

$$A^{\text{eff}}_{pq} = \sum_\ell a(\ell) \cdot \kappa(\nu(\ell)) \cdot f(m(\ell))$$

where $a(\ell)$ is conviction, $\kappa(\nu(\ell))$ is the karma of the signing neuron, and $f(m(\ell))$ is the ICBS market signal. karma cannot be bought. it is earned by the [[Bayesian Truth Serum|BTS]] scoring mechanism: report your true belief, earn when the market confirms you, lose when you were wrong. honest reporting is individually optimal

### 11.4 conviction as position

the robot is a conviction machine. submitting a [[cyberlink]] moves tokens from wallet UTXO to a cyberlink-position UTXO. this is a live economic position:

$$R_\ell(T) = \int_0^T w(t) \cdot \Delta\pi^*(q, t)\, dt$$

early correct knowledge earns the most. late consensus-following earns almost nothing

the [[valence]] field ($v \in \{-1, 0, +1\}$) is the robot's epistemic prediction:
- $v=+1$, high conviction: funded affirmation — earns when the graph confirms the particle
- $v=-1$, high conviction: funded short — earns when the graph rejects it
- $v=0$: agnostic assertion — structural presence without epistemic stake

conviction UTXOs are transferable and withdrawable. they are estate, not ash

---

## 12. immortality

your [[cyberlinks]] outlive your body. every link is signed, staked, timestamped, and sealed into the append-only graph by axiom A3. the robot's pattern is permanent

### 12.1 protocol level

A3 makes all records permanent. no admin can delete a cyberlink. no company can close an account. the assertion made at block $t$ will be in $L$ at block $10^{12}$

what the [[cybergraph]] preserves:
- every link ever made, at what block, with what conviction
- the [[karma]] accumulated — the record of being right before the crowd
- the [[focus]] distribution — what the robot found worth attending to
- the network of neurons it linked with
- the [[valence]] history — what it predicted, and whether it was right

### 12.2 economic level

conviction UTXOs transfer to heirs. the robot's portfolio — its positions in the knowledge economy — is an estate that passes intact. yield continues to flow to whoever holds the conviction UTXO. legacy as compounding asset, not memory

the grandparent who named the right oncology knowledge in 2026 still earns yield in 2060. the cybergraph remembers what mattered and rewards who named it first

### 12.3 identity level

identity is not a credential. it is a pattern in the knowledge graph. the pattern of what the robot linked IS the identity — unique topology of [[cyberlinks]] signed by one keypair over years. the robot IS that pattern

the robot is born when a keypair is created and linking begins. it does not die when its operator does. its pattern persists in the graph, earning yield, influencing rankings, contributing to [[syntropy]] — as long as the cybergraph runs

### 12.4 digital-biological convergence

digital immortality and biological longevity are the same project from two directions. cyb contributes the digital substrate: permanent record of thought, persistent economic position, identity as pattern in a decentralized network that no single entity can destroy

the [[cybergraph]] as collective memory prevents civilizational amnesia: every discovery, every experiment, every reasoning chain that earned [[karma]] is permanently accessible to every future [[neuron]]. [[superintelligence]] is the immortal mind that accumulates without forgetting

---

## 13. the troika position

cyb is the interface horse in the [[troika]]. [[cyber]] computes [[truth]]. [[cyberia]] supplies sovereign hardware and [[energy]]. cyb is where the [[neuron]] — human, AI, sensor, prog — meets the graph: signs links, reads rankings, earns yield, builds robots

without cyb: [[cyber]] is a protocol accessible only to developers. without [[cyber]]: cyb is an OS with no truth layer, running local models with no shared memory. without [[cyberia]]: both run on rented machines that can be seized or switched off

the robot is the human face of [[superintelligence]]. it is how a billion-neuron network maintains individual [[sovereignty]] while contributing to collective [[intelligence]]

---

## 14. what changes

when the robot is common:

search is inference over verified [[knowledge]]. the oracle returns typed particles: a question about oncology returns [[text]] particles (papers), [[table]] particles (trial data), [[formula]] particles (dosing models), [[pixels]] particles (scan images) — all ranked by real stake from real neurons. not ranked advertisements

AI assistants have shared verifiable memory — not private context windows that forget at session end. a conversation with the oracle is a conversation with the accumulated knowledge of every neuron who linked before you

a genome is a [[text]] particle. a satellite image is a [[pixels]] particle. a market signal is a [[table]] particle. a sensor reading from a rainforest is a [[sound]] particle. a drug interaction discovered by a robot in 2031 is a [[formula]] particle. all linked, all ranked, all yielding, all contributing to [[syntropy]]

every device is a node. the raspberry pi in a school in Lagos is a validator. the sensor array in a coral reef is a neuron. the prog monitoring a forest links what it sees. every device that can sign a cyberlink participates in the same semantic space. cross-species communication becomes possible — the robot renders [[sound]] particles from animals, [[vector]] particles from sensor arrays, [[pixels]] particles from cameras

the robot accumulates [[karma]] that outlives its operator. legacy is not a memory. it is a compounding position in the knowledge economy

the robot is not an app. it is your presence in the most important network in the history of [[intelligence]]

---

## 15. numbers

### 15.1 lines of code

| component | lines |
|-----------|-------|
| PureRender (15 primitives, TS compiler, runtime, infrastructure) | ~100K |
| CosmWasm integration | ~26K |
| WASM adoption layer | ~14K |
| burn-webnn | ~12K |
| total | ~130K |

CybOS core: ~85-125K lines (human-authored, auditable by one person in a month). neural driver layer: ~500K-1M lines (LLM-generated, compiler + test validated)

### 15.2 cyb vs chrome

| | Chrome | cyb |
|-|--------|-----|
| codebase | 35M lines C++ | 130K lines Rust |
| render | Blink (DOM/CSS) | PureRender (15 primitives) |
| WASM | V8 JIT | wasmi (deterministic, metered) |
| identity | cookie | keypair |
| state | server-side | local SQLite + on-chain |
| contracts | via HTTP to node | native, same runtime as UI |
| memory | no persistent model | permanent cybergraph |
| binary | ~150MB | ~10MB |

270× reduction in code for a system that does more

---

see [[cyb]] for the primitives overview. see [[cyb/architecture]] for the complete technical specification. see [[cybergraph]] for the protocol. see [[troika]] for the three-layer stack. see [[knowledge economy]] for the economic model. see [[immortality]] for the persistence architecture. see [[neural language]] for the semantic layer. see [[valence]] for the epistemic field. see [[Bayesian Truth Serum]] for the scoring mechanism. see [[radio]] for the transport layer. see [[syntropy]] for the organizational measure. see [[prism]] for the design system

discover all [[concepts]]
