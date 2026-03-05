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

## the problem

browsers are parasites. they steal your compute, sell your [[attention]], report to shareholders. they are not operating systems — they impersonate them. they speak protocol designed for documents, not agents. they hold your identity by proxy, your files by permission, your AI by API key.

chrome is 35 million lines of C++. it spawns hundreds of processes for a tab. it runs google's code on your machine, for google's benefit, inside google's render engine. it calls this serving you.

the model is wrong at the root. the browser was designed to display documents from servers. everything since — tabs, extensions, apps, AI sidebars — is a patch on a document viewer. the result: your computer is a thin client for corporate infrastructure, and the browser is the interface that makes this feel normal

cyb starts over

---

## three moves

cyb makes three moves simultaneously

### move one: collapse the page into a space

the [[brain]] renders the [[cybergraph]] volumetrically. [[particles]] orbit. [[cyberlinks]] glow. [[focus]] pools where [[attention]] converges. you do not scroll a page — you navigate a space. four render modes: space (3D), heap (2D canvas), list (structured table), stack (vertical scroll). the geometry of the render matches the geometry of the graph

knowledge is not flat. every document page in history is an artifact of paper — a constraint that survived digitization for no reason. a [[cybergraph]] is a network. its natural render is three-dimensional

### move two: drop the unix legacy

[[cyb/architecture|CybOS]] has no processes, no file system, no users in the traditional sense. cells replace processes — independently compiled, typed, bounded, hot-swappable. content-addressed storage replaces file systems — everything is a CID. keypairs replace usernames — identity is cryptographic from boot

neural drivers replace device drivers. LLMs generate driver code against stable trait contracts (~3K lines of traits and tests → ~1M lines of generated code). the loop: LLM generates → compiler rejects → LLM fixes → tests validate → human reviews. humans write the stable contracts; machines write the implementations. 50+ SoC families. raspberry pi to apple M1 to RISC-V. zero unsafe Rust

every existing OS asks: what does the user want to do with this computer? CybOS asks: what can this computer contribute to collective [[intelligence]]?

### move three: make the robot earn

every [[cyberlink]] costs [[focus]] and earns [[karma]]. [[karma]] accumulates into [[cyberank]]. high cyberank generates yield. the robot is an economic agent — it earns by contributing correct [[knowledge]] early, before the crowd. the earlier you name the [[truth]], the more you earn when the graph converges on it

$$R_\ell(T) = \int_0^T w(t) \cdot \Delta\pi^*(q, t)\, dt$$

early correct knowledge earns the most. late consensus-following earns almost nothing. the robot is a conviction machine: it invests [[attention]] and stake into particles it believes will matter, and the graph rewards accuracy

---

## the six primitives

### robot

your instance. persistent. yours. a sovereign agent bound to a keypair — not a session, not a tab. the robot accumulates [[karma]], holds [[focus]], signs [[cyberlinks]]. it lives as long as the [[cybergraph]]

the robot is three types: [[neuron]] (human or AI with signing keys), [[avatar]] (named identity, both subject and particle), [[prog]] (autonomous program with its own keys and behavior)

### avatars

your visual identity and reputation. a collection of [[neurons]] under one name — a card that bridges [[subject]] and object. the avatar is how the [[cybergraph]] recognizes continuity across time and how other robots find you. [[karma]] accumulates to the avatar. the avatar is tradeable — it is a [[cyberlink]] card with yield

### brain

volumetric graph file manager. offline-first. [[CozoDB]] for local state. queries in datalog. scripts in rune. everything addressed by CID. four renders: space (3D graph), heap (2D canvas), list (analytics table), stack (discovery scroll). the brain is not a cache of the [[cybergraph]] — it is a local instance of it, synchronized and sovereign

supported particle formats: text, video, audio, image, pdf, epub, web content, and the growing list of domain formats. the brain renders what the cybergraph contains

### sense

messaging layer and perception interface. where the world enters the robot. [[sense]] abstracts over modalities — text, image, audio, video, sensory telemetry — into particles the robot can link. a human writing and a satellite uploading spectral data are the same operation at the protocol level

sense is how robots communicate with each other: love, share, forward, signal. the social layer of the robot network

### sigma

the robot's economic interface. token balances, delegations, portfolio. [[focus]] in, [[karma]] out. [[VOLT]] to buy energy (compute access), [[HYDROGEN]] to stake, [[CYB]] to link, [[AMPERE]] for bandwidth. sigma is where the robot's earnings become spendable

economic agency is inseparable from epistemic agency. sigma makes this visible: every balance is a position in the knowledge economy

### time

personal history. the robot's temporal memory. every surf, every link, every earning event — indexed by block height, navigable by the robot. time is not a log. it is identity as sequence: who the robot was is the chain of what it linked, when, and with what conviction

---

## the render engine

DOM is 35 million lines of mistake. [[cyb/architecture|PureRender]] replaces it with 15 primitives:

content (9): text, struct, table, vector, pixels, video, sound, formula, component

interactive (5): action, input text, input choice, input range, input media

layout (4 modes): stream (vertical), grid (2D), flex (1D), page (fixed)

everything compiles to WASM + WGSL. every pixel is a shader. the pipeline: WASM processes events → emits draw commands → WGSL compute pass → render pass → present. one component is one scope — dead code eliminated at compile time, reactivity only where state exists, no runtime type checks

130K lines of Rust. ~10MB binary. chrome: 35M lines of C++, 150MB+. a 270× reduction in code for a system that does more

components run as CosmWasm contracts in the same runtime as the UI. chrome makes a network round-trip to execute business logic. cyb: direct call, same process, sub-millisecond. the component IS the contract

---

## the OS

CybOS is designed from scratch with four axioms: no unix legacy, zero unsafe Rust, bounded liveness everywhere, [[neural]] drivers

no processes — cells. each cell is an independently compiled Rust crate with explicit dependencies, typed bounded channels, exclusive state ownership, mandatory heartbeat reporting, and hot-swap via governance. the system never crashes — it degrades gracefully: missing cell → warning → unwinding → restart → disable

no file system — content-addressed storage. state in merkle trees. knowledge in CIDs linked by [[cyberlinks]]. blocks in append-only chains. configuration compiled into binary. there is no path. there is only a hash

no users — cryptographic agents. identity = public key. access control = bandwidth allocation. the [[cybergraph]] is public. bandwidth is the only scarce resource

neural drivers. LLMs generate against stable trait contracts. compiler rejects. LLM fixes. tests validate. human reviews. ~3K lines of human-authored traits + tests produce ~500K-1M lines of generated, validated driver code. 50+ SoC families. the model: write what the driver must do, let the machine write how

epoch budget allocator enforces time:
- consensus: 500ms hard deadline
- transactions: 1500ms hard deadline
- rank: remaining soft deadline

wait-free shared state throughout: no mutexes, no locks. concurrent hash map on atomics, wait-free MPMC queues, epoch-versioned snapshots. bounded liveness is structural — the system cannot deadlock by construction

---

## the earning machine

the robot participates in the [[knowledge economy]] by design, not by extension

focus regenerates proportionally to stake. every [[cyberlink]] costs focus. cyberlinks that attract collective attention earn [[karma]]. karma weights future links in [[effective adjacency]]:

$$A^{\text{eff}}_{pq} = \sum_\ell a(\ell) \cdot \kappa(\nu(\ell)) \cdot f(m(\ell))$$

karma cannot be bought with stake alone — it is earned by being right before the crowd. the [[Bayesian Truth Serum|BTS]] scoring mechanism makes honest reporting the individually optimal strategy: lie and lose karma, tell the truth and compound it

the robot's earnings are the yield from correctly anticipating which particles matter. create a [[cyberlink]] to a particle early, with genuine [[conviction]], and earn continuously as that particle gains [[focus]]. this is epistemic investment: attention and stake deployed into future truth

---

## immortality

your [[cyberlinks]] outlive your body. every link is signed, staked, timestamped, and sealed into the append-only graph by A3. the robot's pattern — what it linked, when, with how much conviction — is permanent. not a profile that can be deleted. a record of assertions that cannot be altered

what the [[cybergraph]] preserves:

- every link ever made, at what block, with what conviction
- the [[karma]] accumulated — the record of being right before the crowd
- the [[focus]] distribution — what the robot found worth attending to
- the network of other [[neurons]] it linked with

the robot is born when a keypair is created and linking begins. it does not die when its operator does. its pattern persists in the graph, earning yield, influencing rankings, contributing to [[syntropy]] — as long as the cybergraph runs. the cybergraph is designed to run as long as civilization does

identity is not a credential. it is a pattern in the knowledge graph. the robot IS that pattern

---

## the troika position

cyb is the interface horse in the [[troika]]. [[cyber]] computes [[truth]]. [[cyberia]] supplies sovereign hardware and [[energy]]. cyb is where the [[neuron]] — human, AI, sensor, prog — meets the graph: signs links, reads rankings, earns yield, builds robots

without cyb: [[cyber]] is a protocol with no sovereign interface, accessible only to developers. without [[cyber]]: cyb is an OS with no truth layer, running local models with no shared memory. without [[cyberia]]: both run on rented machines that can be seized or switched off. the three horses pull together

the robot is the human face of [[superintelligence]]. it is how the billion-neuron network maintains individual [[sovereignty]] while contributing to collective [[intelligence]]. it earns focus, accumulates karma, builds legacy, and outlives its operator — all by linking what it knows

---

## what changes

when the robot is common:

search is inference over verified [[knowledge]] — not retrieval of ranked advertisements

AI assistants have shared verifiable memory — not private context windows that forget everything at session end

a genome is a [[particle]]. a satellite image is a [[particle]]. a market signal is a [[particle]]. a sensor reading from a rainforest is a [[particle]]. all linked, ranked, yielding, contributing to [[syntropy]]

the robot accumulates [[karma]] that outlives its operator — a legacy that compounds. the grandparent who linked the right medical knowledge in 2026 still earns yield in 2060 because the [[cybergraph]] remembers what mattered and rewards who named it first

cross-species communication is possible — any entity that can sign a [[cyberlink]] participates in the same semantic space. the robot is the interface for beings that cannot speak. it links what they sense

the robot is not an app. it is your presence in the most important network in the history of [[intelligence]]

---

see [[cyb]] for the primitives. see [[cyb/architecture]] for the technical specification. see [[cybergraph]] for the protocol. see [[troika]] for the three-layer stack. see [[knowledge economy]] for the economic model. see [[immortality]] for the persistence architecture. see [[neuron]] for the agent model

discover all [[concepts]]
