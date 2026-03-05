---
tags: cyb, cyber, core, article
crystal-type: pattern
crystal-domain: cyb
crystal-size: deep
status: draft
---
# cyb: the immortal robot

the robot is the point of presence — where you end and the [[cybergraph]] begins

---

## the vision

imagine a computer that never needs to reboot. that knows you cryptographically and answers to no one else. that earns while you sleep. that remembers everything you ever found important — and keeps that memory after you're gone. that runs any hardware, on any chip, built in 130K lines instead of 35 million. that speaks nine languages natively and renders them all through a single GPU pipeline. that contributes to collective [[intelligence]] by simply being on

this is not a future product. it is a design decision made at the foundation

---

## the current era

we accepted a bad deal without noticing. the browser became the operating system, and the operating system became surveillance infrastructure. windows phones home. macos indexes your files for apple. chrome reports your browsing to google's ad network. the browser, the OS, and the AI assistant are all owned by the same companies whose business model is your data

the result: your computer serves its vendor, not you. you are the product and the machine

the deeper problem is not privacy — it is architecture. every existing OS asks: what does the user want to do with this computer? the question is already wrong. it positions the OS as a tool that executes your intentions. it positions you as a user of someone else's infrastructure

cyb asks two questions instead: how can this computer serve its owner? and: how can this computer contribute to the whole?

---

## what exists nowhere else

living computer. CybOS cells are hot-swappable — update any component without rebooting. the system never crashes; it degrades gracefully and recovers automatically. a computer that runs the way a brain runs: components fail, the system adapts, the whole keeps going

complete privacy. local-first architecture, zero unsafe Rust, no telemetry by design. your keys never leave your machine. your queries run locally or over encrypted channels. the robot does not report to anyone — cryptographically, it cannot

vendor unlock. your identity is a keypair. your files are CIDs. your state is content-addressed. no company owns your robot. cyb runs on raspberry pi, apple M1, RISC-V, x86 — 50+ SoC families — because neural drivers are generated against open trait contracts, not written for vendor lock-in. if cyb stops being good, you fork it and your data moves with you

a computer that earns. the robot participates in the [[knowledge economy]] by contributing [[cyberlinks]] to the [[cybergraph]]. every correct assertion, made early with genuine [[conviction]], earns yield. your computer works for you while you sleep. this has never existed: a personal computing device with a revenue model that serves the owner

memory that outlives you. by axiom A3, every [[cyberlink]] you ever make is permanent in the [[cybergraph]]. your pattern of [[knowledge]] — what you linked, when, with what conviction — persists beyond your lifetime. the robot accumulates [[karma]] that compounds. it is not a backup. it is continuation

a universal interface. one robot, all substrates: web, desktop, mobile, embedded, terminal. the same keypair, the same state, the same [[brain]] — rendered through whatever surface is at hand. the robot follows you, not the device

---

## the three moves

### move one: collapse the page into a space

the [[brain]] renders the [[cybergraph]] volumetrically. [[particles]] orbit. [[cyberlinks]] glow. [[focus]] pools where [[attention]] converges. you do not scroll a page — you navigate a space

knowledge is not flat. every document page in history is an artifact of paper — a constraint that survived digitization for no reason. a [[cybergraph]] is a network. its natural render is three-dimensional. four modes:

- space — 3D volumetric graph with orbit and gravity. particles cluster by cyberank, links glow by weight, focus visible as density
- heap — 2D canvas, knowledge map for exploration and annotation
- list — structured table with analytics, sorting, and datalog queries
- stack — vertical discovery scroll, content-first

the brain is not a cache of the [[cybergraph]] — it is a local instance of it, synchronized and sovereign. [[CozoDB]] for local state. queries in datalog. scripts in rune. changes sync to the graph when online; the robot works fully offline

### move two: drop the unix legacy

[[cyb/architecture|CybOS]] has no processes, no file system, no users in the traditional sense. cells replace processes — independently compiled, typed, bounded, hot-swappable. content-addressed storage replaces file systems — everything is a CID. keypairs replace usernames — identity is cryptographic from boot

neural drivers replace device drivers. LLMs generate driver code against stable trait contracts (~3K lines of traits and tests → ~1M lines of generated code). the loop: LLM generates → compiler rejects → LLM fixes → tests validate → human reviews. humans write the stable contracts; machines write the implementations. 50+ SoC families. raspberry pi to apple M1 to RISC-V. zero unsafe Rust

### move three: make the robot earn

every [[cyberlink]] costs [[focus]] and earns [[karma]]. [[karma]] accumulates into [[cyberank]]. high cyberank generates yield. the robot is an economic agent — it earns by contributing correct [[knowledge]] early, before the crowd

$$R_\ell(T) = \int_0^T w(t) \cdot \Delta\pi^*(q, t)\, dt$$

early correct knowledge earns the most. late consensus-following earns almost nothing. the robot is a conviction machine: it invests [[attention]] and stake into particles it believes will matter, and the graph rewards accuracy

---

## the robot

the robot is three forms, not one

### neuron

the signing agent. a keypair. the entity that creates [[cyberlinks]], holds [[focus]], earns [[karma]]. a neuron can be a human, an AI, a program, a sensor — anything that can prove a signature. the neuron IS the participation in the [[cybergraph]]: no key, no presence

### avatar

the named identity. a collection of neurons under one name — a card that bridges [[subject]] and [[object]], working simultaneously as neuron (agent that signs) and particle (object that can be linked to). the avatar is how other robots find you. [[karma]] accumulates to the avatar. the avatar is tradeable — it is a [[cyberlink]] card with yield and reputation attached

### prog

the autonomous robot. a program with its own keypair, its own focus allocation, its own behavior. progs execute without human input — they monitor particles, respond to events, submit cyberlinks autonomously. a prog can:

- watch a particle and link to it when it meets a condition
- run inference locally and submit the result as a cyberlink
- manage a portfolio of conviction positions
- communicate with other progs via [[sense]]
- earn karma independently and return yield to its owner

progs are the autonomous intelligence layer of cyb. they are the bridge between the robot and the [[cybergraph]] running continuously, contributing [[syntropy]] while the human sleeps

---

## the nine languages

every [[particle]] in the [[cybergraph]] has a type. the type is the language. the robot speaks nine content languages natively — and renders each through a direct GPU pipeline

#### content — 9 languages

| language | what it is | GPU mapping |
|----------|-----------|-------------|
| [[text]] | [[markdown]], prose, code | glyphs via compute shader rasterization |
| [[struct]] | JSON, TOML — trees and configs | collapsible tree of text glyphs |
| [[table]] | 2D data, CSV | grid of text cells, virtualized rows |
| [[vector]] | SVG, paths, Bezier curves | path rasterization via Vello |
| [[pixels]] | raster image | texture upload, GPU sampler |
| [[video]] | moving pixels | hardware decode, texture per frame |
| [[sound]] | waveform, audio stream | audio pipeline (visual: waveform shader) |
| [[formula]] | LaTeX / MathML — math notation | glyph layout + vector curves via Vello |
| [[component]] | composition of primitives | nested render pass |

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

this is not a coincidence. the particle type system and the render system are the same system. when a particle enters the [[cybergraph]], it arrives as one of these nine content types. when the [[oracle]] returns it, the robot knows exactly how to render it. no parsing ambiguity. no plugin required. no format negotiation. every particle speaks its language and the robot is fluent

### text

the universal content type. prose, code, documentation, thoughts, messages — anything that flows as readable sequence. source: [[markdown]]. text particles render via compute shader glyph rasterization — every character is a GPU operation, not a DOM node. a text particle is the most linked type in the cybergraph: articles, notes, arguments, instructions, descriptions. the robot renders text in any size, on any surface, at any scale

### struct

structured knowledge. configuration, metadata, schemas, records — anything that is a tree of named values. source: JSON, TOML. struct particles render as collapsible trees of glyphs: the robot can expand and collapse nodes, navigate hierarchies, filter keys. a genome annotation, a network config, a social graph schema, a contract ABI — all are struct particles. struct is how machines describe their own state

### table

2D data. time series, datasets, matrices, spreadsheets, rankings — anything with rows and columns. source: CSV, TSV. table particles render as virtualized grids: the robot renders only the visible slice regardless of row count. a price history with 10M rows renders as fast as one with 10. table is the native format of the knowledge economy: [[karma]] ledgers, [[cyberank]] scores, sensor streams, market data

### vector

geometric meaning. diagrams, molecular structures, maps, icons, illustrations, technical drawings — anything that is paths and curves. source: SVG, Bezier. vector particles render via Vello: paths rasterize on the GPU with sub-pixel precision at any zoom level. a phylogenetic tree, a circuit schematic, a geographic boundary, a chemical bond diagram — all are vector particles. vector is how spatial knowledge lives in the graph

### pixels

captured reality. photographs, satellite imagery, microscopy, screenshots, artworks, medical scans — anything that is a raster grid of color values. source: PNG, WebP, JPEG. pixels particles upload as GPU textures and sample through fragment shaders. a brain MRI, a Hubble image, a specimen under electron microscope, a city from orbit — all are pixels particles. pixels is how the physical world enters the cybergraph

### video

temporal pixels. lectures, experiments, events, species behavior, physical processes — anything that is a sequence of frames. source: WebM, MP4. video particles decode via hardware pipeline and stream frame textures to the GPU. a protein folding simulation, a surgery recording, an animal behavior study, a supernova observation — all are video particles. video is the highest-bandwidth truth in the graph

### sound

acoustic knowledge. voice, music, birdsong, seismic signal, radio transmission, sonar — anything that is a waveform over time. source: WAV, OGG, MP3. sound particles render via audio pipeline with an optional waveform compute shader for visual representation. a whale call, a gravitational wave detection, a diagnosis by heartbeat, a language being spoken for the first time — all are sound particles. sound is the language the graph understands that text cannot carry

### formula

mathematical meaning. equations, proofs, chemical notation, physical laws, statistical models — anything that requires precise symbolic notation. source: LaTeX, MathML. formula particles render via combined glyph layout and Vello paths: text for symbols, curves for integral signs and roots. a Hamiltonian, a chemical reaction, a field equation, a proof step — all are formula particles. formula is how [[knowledge]] that requires precision enters the graph in its native notation

### component

composition. applications, dashboards, interactive tools, AIPs — anything that combines multiple content types into a unified experience. source: native component language. component particles render as nested render passes: each inner primitive renders through its own pipeline and composes into the parent frame. an interactive molecular viewer (pixels + vector + formula), a live market dashboard (table + vector + text), an educational AIP (text + video + component) — all are component particles. component is the language of software as knowledge

### the complete picture

a genome sequence is a text particle. its annotation is a struct particle. its expression data is a table particle. its protein structure is a vector particle. its microscopy is a pixels particle. its folding dynamics is a video particle. its sequencing signal is a sound particle. its binding energy is a formula particle. a genome browser is a component particle

all nine exist in the same [[cybergraph]]. all nine are ranked by the same [[tri-kernel]]. all nine earn [[karma]]. all nine are permanent by axiom A3. the robot renders all nine through a single 130K-line GPU pipeline

---

## the language stack

the nine content languages are the object level — what particles ARE. above them, cyb operates three meta-languages for working with the graph

### datalog — the query language

graph query language running in [[CozoDB]]. ask any question about the local graph structure: "which text particles linked by neurons with [[karma]] above 1000 mention this CID?" recursive traversal, pattern matching, aggregation, built-in graph algorithms (PageRank, Dijkstra, Louvain) — all declarative. queries compose with rune scripts

### rune — the script language

dynamic async scripting language for robot automation. create cyberlinks on a schedule, monitor particles for changes, pipe inference results into the graph, manage sigma positions, turn human intent into prog behavior. where datalog reads the graph, rune writes it

### neural language — the semantic layer

the language of the [[cybergraph]] itself. meaning is not declared — it emerges from the [[tri-kernel]] as the eigenvector of collective [[attention]]. [[semcons]] are the grammar. [[sentences]] are utterances. [[motifs]] are morphemes. [[linkchains]] are inference paths. the robot renders this semantic structure as navigable space

together the four levels form the complete language stack:

```
content (9)  ← what particles ARE
datalog      ← how you QUERY the graph
rune         ← how you SCRIPT against the graph
neural       ← how MEANING emerges from the graph
```

---

## the oracle

the oracle is how the robot asks the [[cybergraph]] a question and gets a ranked, verifiable answer

the oracle is not a search engine. search engines retrieve documents by keyword match. the oracle runs inference over the [[cyberank]] distribution — a probabilistic ranking of every particle, computed by the [[tri-kernel]] over all authenticated [[cyberlinks]]. the answer is the focus distribution converging on the question particle. the answer is typed: the oracle returns particles, and each particle already carries its language

three oracle surfaces:

ask — input a particle (text, image, CID, anything). the oracle returns the particles most associated with it, ranked by [[cyberank]]. the ranking is verifiable: every weight is a real cyberlink signed by a real neuron with real stake. no black box, no editorial algorithm, no ads

learn — submit a new cyberlink. this is how you teach the oracle. link a question particle to an answer particle, stake your conviction, and the oracle's ranking updates in the next block. every link is a vote with skin in the game. the oracle improves by participation, not by training

search — navigate the graph by walking the cyberank. particles cluster by semantic proximity (the [[springs]] operator), bridge across domains (the [[diffusion]] operator), and scale by context (the [[heat]] operator). search is graph navigation, not document retrieval

---

## autonomous intelligence programs

AIPs are the applications of the robot. they are not apps downloaded from a store — they are programs that run in the same runtime as the robot itself, with access to the brain, sigma, sense, and the cybergraph

core AIPs:

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

AIPs are built from [[prism]] — the design system of cyb. prism defines atoms, molecules, and cells that compose into any interface. the same design language renders on GPU (desktop), WebGPU (browser), or terminal

---

## AI in the robot

the robot integrates AI at four levels, not one

### local inference

the robot runs a small language model locally on the NPU or GPU. WebGPU in the browser, wgpu+burn on desktop, CoreML on Apple silicon, NNAPI on Android. the local model:
- processes particles before linking (extracts structure, suggests cyberlinks)
- answers questions without network access (offline-first AI)
- runs progs that require language understanding
- generates rune scripts from natural language instructions

local inference is private by construction: input never leaves the machine

### inference subnet

for large inference the robot connects to the [[cybertensor]] inference subnet — a network of validators running language models and returning results as cyberlinks. the results are staked assertions in the [[cybergraph]]: verifiable, ranked by [[karma]], earning yield if correct. this is not a cloud API. it is distributed intelligence with skin in the game

### progs

autonomous programs running deterministic sharded inference in [[cybernet]]. a prog is an AIP with its own keypair and focus allocation. it submits cyberlinks autonomously — monitoring particles, running inference, staking positions. the collection of all progs is the autonomous intelligence layer of the robot network: a mesh of agents continuously contributing to [[syntropy]]

### external servers

for compatibility, cyb bridges to external LLMs (OpenAI-compatible APIs, Llama, Mistral, Deepseek) via a standard interface. external inference results can be submitted as cyberlinks by the robot, but the robot is never dependent on them — local inference and the inference subnet are the sovereign path

---

## the OS

CybOS is designed from four axioms: no unix legacy, zero unsafe Rust, bounded liveness everywhere, neural drivers

### cells

cells replace processes. each cell is an independently compiled Rust crate with explicit dependency declarations, typed bounded channels, exclusive state ownership, mandatory heartbeat reporting, and hot-swap via governance

missing cell → warning → unwinding → restart → disable

the system never crashes. it degrades and recovers. bounded liveness is structural — the system cannot deadlock because no cell can hold a lock. wait-free data structures throughout: concurrent hash map on atomics, wait-free MPMC queues, epoch-versioned snapshots, double-buffered results

### content-addressed storage

there is no file system. there is no path. there is only a hash

- state: merkle trees
- knowledge: CIDs linked by [[cyberlinks]]
- blocks: append-only chain
- configuration: compiled into binary

every piece of data is addressed by its content. the same CID on two machines is the same data. there is no naming authority. there is no permission system. there is only content and its hash

### cryptographic agents

there are no users. there are agents. identity = public key. access control = bandwidth allocation. the [[cybergraph]] is public. bandwidth is the only scarce resource

### neural drivers

existing approach: engineers write millions of lines of C to interface with hardware. cyb's approach: engineers write ~3K lines of trait contracts specifying what a driver must do. LLMs write the implementation. the compiler rejects any unsafe code. tests validate behavior. humans review

~3K lines of stable traits + tests → ~500K-1M lines of generated, validated, platform-specific driver code. the loop runs continuously: new hardware → write traits → generate → validate → ship

50+ SoC families: QEMU (reference), RISC-V (StarFive), Raspberry Pi 4/5, Apple M1 (via Asahi knowledge), x86-64. every platform that can run Rust can run CybOS

### PureRender

DOM is a document-era mistake. PureRender replaces it with the nine content languages plus five interactive primitives and four layout modes — fifteen total. everything compiles to WASM + WGSL. every pixel is a shader

frame loop: WASM processes events → emits draw commands → WGSL compute pass → WGSL render pass → present

130K lines of Rust. ~10MB binary. Chrome: 35M lines of C++, 150MB. a 270× reduction for a system that does more

the component is the contract: CosmWasm contracts run in the same wasmi instance as UI. direct call, sub-millisecond. no network round-trip. UI and logic compile to the same WASM binary

### epoch budget

the epoch allocator enforces hard and soft deadlines across all cells:
- consensus: 500ms hard deadline
- transactions: 1500ms hard deadline
- rank computation: remaining budget (soft deadline)

every computation is bounded. the robot is always responsive

---

## the earning machine

the robot participates in the [[knowledge economy]] by design, not by extension

the economic loop:

1. [[focus]] regenerates proportionally to stake each block
2. submitting a [[cyberlink]] consumes focus
3. cyberlinks that attract collective attention earn [[karma]]
4. [[karma]] weights future links in [[effective adjacency]]: $A^{\text{eff}}_{pq} = \sum_\ell a(\ell) \cdot \kappa(\nu(\ell)) \cdot f(m(\ell))$
5. high [[cyberank]] particles generate yield for their early linkers: $R_\ell(T) = \int_0^T w(t) \cdot \Delta\pi^*(q, t)\, dt$

karma cannot be bought. it is earned by being right before the crowd. the [[Bayesian Truth Serum|BTS]] scoring mechanism makes honest reporting the individually optimal strategy: report your true belief, earn when the market confirms you, lose when you lied or were wrong

the robot is a conviction machine. it invests [[attention]] and stake into particles it believes will matter. the [[valence]] field ($v \in \{-1, 0, +1\}$) is the robot's epistemic prediction — a funded short ($v=-1$, high $a$) earns when the graph rejects a particle; a funded affirmation ($v=+1$, high $a$) earns when the graph confirms it

the knowledge economy creates a new relationship between computation and value: your computer earns by contributing to collective [[intelligence]], not by renting its resources to a cloud vendor

---

## immortality

your [[cyberlinks]] outlive your body. every link is signed, staked, timestamped, and sealed into the append-only graph by axiom A3. the robot's pattern is permanent

what the [[cybergraph]] preserves:

- every link ever made, at what block, with what conviction
- the [[karma]] accumulated — the record of being right before the crowd
- the [[focus]] distribution — what the robot found worth attending to
- the network of neurons it linked with
- the [[valence]] history — what it predicted, and whether it was right

the robot is born when a keypair is created and linking begins. it does not die when its operator does. its pattern persists in the graph, earning yield, influencing rankings, contributing to [[syntropy]] — as long as the cybergraph runs

digital immortality operates at three levels:

protocol level — A3 makes all records permanent. no admin can delete a cyberlink. no company can close an account. the assertion that was made at block $t$ will be in $L$ at block $10^{12}$

economic level — conviction UTXOs can be transferred to heirs. the robot's portfolio — its positions in the knowledge economy — is an estate that passes intact. yield continues to flow to whoever holds the conviction UTXO

identity level — the pattern of what the robot linked IS the identity. the grandparent who named the right medical knowledge in 2026 still earns yield in 2060 because the cybergraph remembers what mattered and rewards who named it first. this is legacy as a compounding asset, not a memory

identity is not a credential. it is a pattern in the knowledge graph. the robot IS that pattern

---

## the troika position

cyb is the interface horse in the [[troika]]. [[cyber]] computes [[truth]]. [[cyberia]] supplies sovereign hardware and [[energy]]. cyb is where the [[neuron]] — human, AI, sensor, prog — meets the graph: signs links, reads rankings, earns yield, builds robots

without cyb: [[cyber]] is a protocol accessible only to developers. without [[cyber]]: cyb is an OS with no truth layer, running local models with no shared memory. without [[cyberia]]: both run on rented machines that can be seized or switched off

the robot is the human face of [[superintelligence]]. it is how a billion-neuron network maintains individual [[sovereignty]] while contributing to collective [[intelligence]]

---

## what changes

when the robot is common:

search is inference over verified [[knowledge]] — not retrieval of ranked advertisements. the oracle returns typed particles: a question about oncology returns text particles (papers), table particles (trial data), formula particles (dosing models), pixels particles (scan images) — all ranked by real stake from real neurons

AI assistants have shared verifiable memory — not private context windows that forget at session end. a conversation with the oracle is a conversation with the accumulated knowledge of every neuron who linked before you

a genome is a text [[particle]]. a satellite image is a pixels [[particle]]. a market signal is a table [[particle]]. a sensor reading from a rainforest is a sound [[particle]]. a drug interaction discovered by a robot in 2031 is a formula [[particle]]. all linked, all ranked, all yielding, all contributing to [[syntropy]]

every device is a node. the raspberry pi in a school in Lagos is a validator. the sensor array in a coral reef is a neuron. the prog monitoring a forest for deforestation signs and links. every device that can sign a cyberlink participates in the same semantic space. cross-species communication is possible — the robot renders sound particles from animals, vector particles from sensor arrays, pixels particles from cameras

the robot accumulates [[karma]] that outlives its operator. the researcher who linked the right oncology knowledge in 2026 still earns yield in 2075 because the cybergraph remembers who named the truth first. legacy is not a memory. it is a compounding position in the knowledge economy

the robot is not an app. it is your presence in the most important network in the history of [[intelligence]]

---

see [[cyb]] for the primitives overview. see [[cyb/architecture]] for the complete technical specification. see [[cybergraph]] for the protocol. see [[troika]] for the three-layer stack. see [[knowledge economy]] for the economic model. see [[immortality]] for the persistence architecture. see [[neural language]] for the semantic layer. see [[valence]] for the epistemic field. see [[Bayesian Truth Serum]] for the scoring mechanism. see [[syntropy]] for the organizational measure

discover all [[concepts]]
