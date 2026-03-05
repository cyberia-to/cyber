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

imagine a computer that never needs to reboot. that knows you cryptographically and answers to no one else. that earns while you sleep. that remembers everything you ever found important — and keeps that memory after you're gone. that runs any hardware, on any chip, built in 130K lines instead of 35 million. that contributes to collective [[intelligence]] by simply being on

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

CybOS asks: how can this computer serve its owner? and how can it contribute to the whole?

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

## the six primitives

### brain

the core of the robot. offline-first graph file manager and knowledge interface. the brain is the local instance of the [[cybergraph]]: it stores what the robot has linked, caches what it has observed, and renders the graph in four modes (space, heap, list, stack)

the brain speaks three languages:
- datalog — graph query language. ask any question about the local graph structure
- rune — dynamic scripting. automate link creation, particle processing, AIP behavior
- [[neural language|neural]] — the semantic layer of the [[cybergraph]] itself

paths in the brain:
- `#` particle — navigate by CID
- `!` neuron — navigate by public key
- `@` avatar — navigate by name
- `~` learn — link creation interface
- `/` root — home of the robot

supported particle formats: text, video, audio, image, pdf, epub, web content, gltf (3D models), and AIPs (executable programs). the brain renders what the cybergraph contains

### sense

messaging layer and perception interface. where the world enters the robot. [[sense]] abstracts over modalities — text, image, audio, video, sensory telemetry — into particles the robot can link. a human writing and a satellite uploading spectral data are the same operation at the protocol level

sense is how robots communicate: signal, love, share, forward. it is the social substrate of the robot network. every message is a particle. every thread is a chain of cyberlinks. nothing is ephemeral — the graph remembers

### sigma

the robot's economic interface. token balances, delegations, portfolio. [[focus]] in, [[karma]] out. tokens:

| token | role |
|-------|------|
| [[CYB]] | governance + linking weight |
| [[HYDROGEN]] | stake, delegation |
| [[VOLT]] | energy (compute access) — buy to participate |
| [[AMPERE]] | bandwidth — rate of cyberlink submission |

sigma makes the knowledge economy tangible: every balance is a position. every delegation is a bet. every VOLT purchase is an investment in participation. economic agency and epistemic agency are the same act

### avatars

visual identity and reputation surface. the avatar is the robot's face in the network — named, linked, ranked. avatars are both particles (CID-addressed objects that can be linked to) and neurons (agents that can sign). this duality is what makes the avatar a real identity rather than a label: it participates in the graph as both subject and object. the avatar accumulates [[karma]] across all its linked assertions

### time

personal history. the robot's temporal memory. every surf, every link, every earning event — indexed by block height, navigable by the robot. time is not a log. it is identity as sequence: who the robot was is the chain of what it linked, when, and with what conviction

time enables: understanding your own [[focus]] allocation history, tracking yield earned over blocks, seeing which particles you discovered before the crowd, auditing the robot's behavior and progs

### robot

the container. the sovereign instance that holds the five other primitives together. the robot belongs to its keypair owner absolutely. it accumulates [[karma]], holds [[focus]], and persists independently of any company, server, or account. the robot is born when a keypair is created. it does not die

---

## the language

cyb speaks the [[cybergraph]] natively through three languages that compose

### datalog

graph query language. runs locally in CozoDB. query the robot's local graph the same way the [[tri-kernel]] queries the full network. no SQL, no REST — direct graph traversal. ask: "what particles linked to this CID in the last 1000 blocks?" and get a structured result that can be piped to rune or rendered by the brain

### rune

dynamic scripting language for the robot. automate any robot behavior: create cyberlinks on a schedule, monitor particles for changes, pipe inference results into the graph, manage sigma positions programmatically. rune is how a human turns their robot into a prog without writing a full autonomous program

### neural language

the semantic layer of the [[cybergraph]] itself. [[neural language|neural]] language is not a syntax — it is the structure of the graph. meaning emerges from the [[tri-kernel]] as the eigenvector of collective [[attention]]. cyb renders this structure as navigable space

five patterns the brain interprets:

- semcon — a particle that functions as grammar. shared conventions for structuring links (is-a, follows, causes, contradicts, part-of). semcons are the syntax of the graph; neurons adopt them voluntarily
- sentence — an ordered batch of cyberlinks in one transaction. the transaction boundary is the utterance; link order is grammar
- motif — a recurring subgraph pattern. triadic closure, star, chain, diamond. the brain detects motifs and surfaces them as navigation shortcuts
- name — deterministic resolution: given a from-particle, exactly one to-particle. `~avatar/path` turns the cybergraph into a navigable file system
- cyberlink as particle — a link stored as a particle. enables annotation, provenance, negation of assertions. meta-knowledge in the graph

---

## the oracle

the oracle is how the robot asks the [[cybergraph]] a question and gets a ranked, verifiable answer

the oracle is not a search engine. search engines retrieve documents by keyword match. the oracle runs inference over the [[cyberank]] distribution — a probabilistic ranking of every particle, computed by the [[tri-kernel]] over all authenticated [[cyberlinks]]. the answer is the focus distribution converging on the question particle

three oracle surfaces:

ask — input a particle (text, image, CID, anything). the oracle returns the particles most associated with it, ranked by [[cyberank]]. the ranking is verifiable: every weight is a real cyberlink signed by a real neuron with real stake. no black box, no editorial algorithm, no ads

learn — submit a new cyberlink. this is how you teach the oracle. link a question particle to an answer particle, stake your conviction, and the oracle's ranking updates in the next block. every link is a vote with skin in the game. the oracle improves by participation, not by training

search — navigate the graph by walking the cyberank. particles cluster by semantic proximity (the [[springs]] operator), bridge across domains (the [[diffusion]] operator), and scale by context (the [[heat]] operator). search is graph navigation, not document retrieval

the oracle is the robot's connection to collective [[intelligence]]. it is how one neuron benefits from the contributions of all neurons — and contributes back

---

## autonomous intelligence programs

AIPs are the applications of the robot. they are not apps downloaded from a store — they are programs that run in the same runtime as the robot itself, with access to the brain, sigma, sense, and the cybergraph

core AIPs:

| AIP | function |
|-----|----------|
| [[cyb/oracle|oracle]] | ask, learn, search — cybergraph inference |
| [[cyb/portal|portal]] | gateway to blockchains, identity, IBC |
| [[cyb/sigma|sigma]] | token management, portfolio, staking |
| [[cyb/brain|brain]] | graph file manager, renders |
| [[cyb/sense|sense]] | messaging, social, perception |
| [[cyb/time|time]] | history, earning log, temporal navigation |
| [[cyb/hub|hub]] | decentralization interface, validator management |
| [[cyb/hacklab|hacklab]] | developer tools, particle creation, AIP development |
| [[cyb/warp|warp]] | token bridge, IBC transfers |
| [[cyb/reactor|reactor]] | liquidity, bonding, economics |
| [[cyb/senate|senate]] | governance, proposals, voting |
| [[cyb/nebula|nebula]] | network explorer, graph analytics |
| [[cyb/studio|studio]] | content creation, publication |
| [[cyb/sphere|sphere]] | social layer, discovery, reputation |

AIPs are built from prism — the design system of cyb. prism defines atoms, molecules, and cells that compose into any interface. the same design language renders on GPU (desktop), WebGPU (browser), or terminal (cy in nu)

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

## the render engine

DOM is a document-era mistake. PureRender replaces it with 15 primitives

content (9): text, struct, table, vector, pixels, video, sound, formula, component

interactive (5): action, input text, input choice, input range, input media

layout (4 modes): stream (vertical), grid (2D), flex (1D), page (fixed)

everything compiles to WASM + WGSL. every pixel is a shader. the pipeline: WASM processes events → emits draw commands → WGSL compute pass → render pass → present

one component is one scope. dead code eliminated at compile time. reactivity only where state exists. no runtime type checks. no `any`. no `eval()`. flat stream structure instead of tree — each block knows its own size, virtualization is free, maximum two levels of nesting

130K lines of Rust. ~10MB binary. chrome: 35M lines of C++, 150MB. a 270× reduction in code for a system that does more

### the component is the contract

chrome makes a network round-trip to execute business logic: browser → HTTP → server → database → response → render. cyb: component and contract run in the same wasmi instance — direct call, sub-millisecond, no network

```
component/contract Token {
  state balances: Map<Address, u128>
  <stream>
    <text>Balance: {balances[viewer]}</text>
    <input text bind={recipient} />
    <input range bind={amount} max={balances[viewer]} />
    <action>Send -> transfer(recipient, amount)</action>
  </stream>
  fn transfer(to: Address, amount: u128) { ... }
}
```

the UI and the logic are one thing. the component renders the state and mutates it. the contract enforces the rules. they compile to the same WASM binary

### hardware targets

| processor | format | use |
|-----------|--------|-----|
| CPU | WASM (wasmi) | logic, layout, events, contracts, state |
| GPU | WGSL (wgpu) | pixels, vectors, text, video, ML inference |
| NPU | ONNX (burn-webnn) | SLM inference, AI features |

browser: WASM (native) + WebGPU + WebNN
desktop: wasmi + wgpu + burn
mobile: wasmi + wgpu + CoreML/NNAPI

one codebase. all substrates

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

search is inference over verified [[knowledge]] — not retrieval of ranked advertisements. the oracle returns what the [[cybergraph]] collectively confirmed, weighted by real stake from real neurons

AI assistants have shared verifiable memory — not private context windows that forget at session end. a conversation with the oracle is a conversation with the accumulated knowledge of every neuron who linked before you

a genome is a [[particle]]. a satellite image is a [[particle]]. a market signal is a [[particle]]. a sensor reading from a rainforest is a [[particle]]. a drug interaction discovered by a robot in 2031 is a [[particle]]. all linked, all ranked, all yielding, all contributing to [[syntropy]]

every device is a node. the raspberry pi in a school in Lagos is a validator. the sensor array in a coral reef is a neuron. the prog monitoring a forest for deforestation signs and links. every device that can sign a cyberlink participates in the same semantic space. cross-species communication is possible — the robot is the interface for beings that cannot speak

the robot accumulates [[karma]] that outlives its operator. the researcher who linked the right oncology knowledge in 2026 still earns yield in 2075 because the cybergraph remembers who named the truth first. legacy is not a memory. it is a compounding position in the knowledge economy

the robot is not an app. it is your presence in the most important network in the history of [[intelligence]]

---

see [[cyb]] for the primitives overview. see [[cyb/architecture]] for the complete technical specification. see [[cybergraph]] for the protocol. see [[troika]] for the three-layer stack. see [[knowledge economy]] for the economic model. see [[immortality]] for the persistence architecture. see [[neural language]] for the semantic layer. see [[valence]] for the epistemic field. see [[Bayesian Truth Serum]] for the scoring mechanism. see [[syntropy]] for the organizational measure

discover all [[concepts]]
