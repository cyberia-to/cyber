---
icon: 👙
tags: cyber
alias: soft3 stack, cyb stack, software stack, proof pipeline
crystal-type: entity
crystal-domain: cyber
---
# soft3

every generation of the web had its stack. web1 had LAMP. web2 had React + Node + Postgres. web3 had Solidity + EVM + RPC. each defined what developers could build and what users could experience

soft3 is the stack for a shared, provable, self-improving [[knowledge]] system where every computation leaves a [[cryptographic proof]] and every piece of meaning has a measurable weight

[[neurons]] — humans, AIs, sensors, agents — link [[knowledge]] into the [[cybergraph]]. the [[tru]] reads this graph every block and computes what matters: [[cyberank]] per [[particle]], [[karma]] per [[neuron]], [[syntropy]] of the whole. every result is deterministic, on chain, verifiable by anyone. [[trident]] compiles any logic into [[zheng]] proofs — hash-based, post-quantum, no trusted setup. [[neural]] structures meaning through [[semantic conventions]] so the graph speaks a [[language]] both humans and machines understand. [[cyb]] makes all of it accessible — a personal [[cyb/robot]] that queries, scripts, and navigates the graph

the [[tru]] is an onchain [[language]] model. it does what models do — rank, retrieve, infer — except the weights are public [[tokens]], the training data is an open [[cybergraph]], and the inference runs in [[consensus]] with proofs. no API keys, no corporate weights, no black boxes. the model improves when anyone links useful [[knowledge]], and the improvement is measurable as rising [[syntropy]]

[[trident]] closes the provability gap. in existing stacks, smart contracts can move [[tokens]] but cannot prove that a computation happened correctly without re-executing it. [[trident]] programs produce [[zheng]] proofs: verify once, trust forever. this makes the stack suitable for [[AI alignment]] — you can prove that a model followed a policy, not just trust that it did

see [[cyber]] for the full stack breakdown and [[specifications]]

## the core

fourteen repos form the core. [[cybergraph]] is the vertebra — everything attaches to it. [[strata]] is the floor — every proof reduces to operations in its five algebras. the boundary is sharp: below it, Rust bootstrap required. above it, everything is pure [[trident]].

fourteen core repos. plus [[rune]]: the async layer above.

| # | repo | verb | what it does | release |
|---|------|------|-------------|---------|
| 0 | [[strata]] | math | 4 tiers × 5 algebras | — |
| 1 | [[hemera]] | hash | [[Poseidon2]] sponge. particle identity | v0.2.0 |
| 2 | [[lens]] | commit | 5 PCS backends, one per algebra | — |
| 3 | [[trident]] | compile | .tri → .nox | v0.1.0 |
| 4 | [[nox]] | run | 18 patterns (16 compute + call + look) + jets | — |
| 5 | [[zheng]] | prove & verify | [[SuperSpartan]] + Brakedown + [[sumcheck]] | — |
| 6 | [[cybergraph]] | link | jets, memos, types, knowledge | — |
| 7 | [[bbg]] | store | 1 polynomial, 10 dims. ~200B proofs | — |
| 8 | [[tru]] | converge | .graph → .model. φ*, eigenvectors, cyberank | — |
| 9 | [[glia]] | infer | universal .model runtime | — |
| 10 | [[mir]] | render | positions + features → [[R-1.0]] world | — |
| 11 | [[mudra]] | encrypt | KEM, dCTIDH, AEAD, TFHE, threshold | — |
| 12 | [[radio]] | transmit | QUIC + BAO streaming + gossip | — |
| 13 | [[foculus]] | agree | [[collective focus theorem]] → finality | — |
| + | [[rune]] | eval | Rs + hint + host + eval. dynamic async layer | — |

## foundation — math, identity, commitment

---

## strata — math

the floor. every proof, every hash, every commitment reduces to operations in one of five algebras. four trait tiers — each consumed by a different set of core components:

| tier | crate | traits | consumed by |
|------|-------|--------|------------|
| 1: universal | strata-core | Codec, Semiring, Ring, Field | hemera, lens, nox, zheng, bbg, mudra |
| 2: proofs | strata-proof | Reduce (bytes→F), Dot (Σaᵢbᵢ) | lens, zheng |
| 3: compute | strata-compute | Spectral (NTT roots), Bits | nox, jali |
| 4: structure | strata-ext | Extension (tower), Batch (Montgomery inv), Blind (ct-ops) | lens, mudra, genies |

five algebras — each maps to one lens construction:

| algebra | structure | lens | construction | regime |
|---------|-----------|------|-------------|--------|
| [[nebu]] | F_p (Goldilocks) | scalar | Brakedown | truth — field polynomials, execution traces |
| [[kuro]] | F₂ tower → F₂¹²⁸ | binary | Binius | efficiency — binary witnesses, quantized AI |
| [[jali]] | R_q = F_p[x]/(xⁿ+1) | ring | Ikat | encrypted computation — TFHE, FHE bootstrapping |
| [[trop]] | (min, +) semiring | tropical | Assayer | optimality — optimization witnesses, dual certificates |
| [[genies]] | F_q (CSIDH-512) | isogeny | Porphyry | privacy — curve polynomials, stealth addresses |

zheng is decomposed by strata: SuperSpartan constraint evaluation = Dot products over Field; Fiat-Shamir challenges = Reduce over hemera output bytes; Brakedown = expander encoding. see [[strata]]

## hemera — hash

[[Poseidon2]] sponge over [[Goldilocks field]]. t=16, Rf=8 (x⁷), Rp=16 (x⁻¹), r=8, c=8. 24 rounds. 32-byte output. ~736 constraints in a [[zheng]] proof (vs ~50,000 for Blake3).

hemera gives [[particles]] their identity. every CID in the [[cybergraph]] is a hemera output. see [[hemera]]

## lens — commit

five polynomial commitment backends — one per strata algebra. same three operations (commit, open, verify), different algebraic structures.

| lens | algebra | construction | regime |
|------|---------|-------------|--------|
| scalar | [[nebu]] F_p (Goldilocks) | Brakedown | truth — field polynomials, execution traces |
| binary | [[kuro]] F₂ tower → F₂¹²⁸ | Binius | efficiency — binary witnesses, quantized AI |
| ring | [[jali]] R_q = F_p[x]/(xⁿ+1) | Ikat | encrypted computation — TFHE, FHE bootstrapping |
| tropical | [[trop]] (min, +) semiring | Assayer | optimality — optimization witnesses, dual certificates |
| isogeny | [[genies]] F_q (CSIDH-512) | Porphyry | privacy — curve polynomials, stealth addresses |

see [[lens]]

## programs — compile, run, prove

---

## trident — compile

the provable language. .tri source compiles to .nox. every trident construct maps to exactly one nox pattern. 57K LOC, 24 VM targets, self-hosts in Stage 2 of the [[bootstrap plan]].

trident's compiler backend includes a neural optimizer: a GNN+Transformer (~13M params, GATv2 encoder + 6-layer decoder) that optimizes TIR→TASM at compile time. classical lowering always runs; neural output accepted only when stack-verified equivalent and strictly cheaper. speculative, not required.

without trident, nox is a bare CPU with no assembler. trident already targets 28 VMs — including inefficient legacy ones. nox is the only efficient destination.

fourteen languages compile to nox:

| short | long | universe | type | algebra | purpose |
|-------|------|----------|------|---------|---------|
| [[Nox]] | Nox | Structure | Tree | Combinators | composes all languages |
| [[Bt]] | Bitwise | Binary | Bit | F₂ tower | proves circuits |
| [[Rs]] | Rustic | Byte | Word | Z/2ⁿ | runs systems |
| [[Tri]] | [[Trident]] | [[field]] | Field tower | F_{pⁿ} | settles proofs |
| [[Arc]] | Arc | [[topology]] | [[graph]] | [[category theory]] | stores [[knowledge graph]] |
| [[Ren]] | Render | [[geometry]] | Shape | G(p,q,r) | renders space |
| [[Dif]] | Differential | Curvature | Manifold | (M, g) | embeds meaning |
| [[Sym]] | Symplectic | Dynamics | Phase | (M, ω), dω = 0 | simulates physics |
| [[Bel]] | Belief | [[belief]] | Distribution | g on Δⁿ | models self |
| [[Seq]] | Sequence | Causality | Event | Partial order | orders events |
| [[Inf]] | Infer | [[inference]] | Relation | Horn clauses | derives facts |
| [[Wav]] | Wave | Continuum | Poly | Convolution / R_q | reads [[signals]] |
| [[Ten]] | Tensor | Linear | Tensor | Contraction | trains models |
| [[Tok]] | Token | Resource | [[UTXO]] | Conservation | prices computation |

see [[trident]] and [[cyb/languages]]

## rune — eval

sits above the fourteen — the dynamic, async layer. where the fourteen languages are deterministic and provable, rune is interactive and side-effectful: [[Rs]] syntax executed via [[nox]] tree rewriting, extended with three capabilities none of the fourteen have:

- `hint` — async input. yields execution, resumes when data arrives (network event, user input, epoch tick, graph change)
- `host(target, args)` — calls WASM or wGPU. exits the proof boundary, returns a noun
- `eval(noun)` — runtime metaprogramming. execute a dynamically constructed formula

rune accommodates all fourteen languages — any .tri, .arc, .ten, .tok program can run inside rune. the difference is context: the fourteen languages run in consensus with proofs; rune runs locally, fast, with access to the outside world. millisecond start. the nervous system of [[cyb]].

## runtimes

four runtimes. two native with dedicated compilers, two legacy migrating toward native:

| runtime | compiler | backend | what runs | status |
|---------|----------|---------|-----------|--------|
| [[nox]] | trident (.tri → .nox) | native (Rs, AMX) | proven .nox programs, jets | primary |
| [[glia]] | tru (.graph → .model) | ANE, AMX, Metal | .model inference | primary |
| [[wasm]] | — | wasmi | WASM modules, rune host jets | → nox |
| [[wgpu]] | — | Metal, Vulkan, WebGPU | GPU compute shaders | → glia |

trident knows .tri. nox knows nothing about .tri — it just runs .nox. tru knows .graph. glia knows nothing about graphs — it just runs .model. mir reads both.

migration paths: WASM module → Rs → nox (with [[zheng]] proof). wGPU shader → [[Ten]]/[[Wav]] → glia kernel.

## nox — run

eighteen patterns total: sixteen deterministic compute over hemera-authenticated trees — five structural (axis, quote, compose, cons, branch), six field (add, sub, mul, inv, eq, lt), four bitwise (xor, and, not, shl), one hash — plus call (non-deterministic witness injection) and look (deterministic BBG read). 16 compute + call + look = 18.

nox core is frozen (18 patterns, [[checkpoint]] 0). jets are external — looked up in the [[cybergraph]] by formula hash during reduction. adding a jet does not change nox. removing all jets does not break nox (just slower).

computation IS linking: `ask(ν, subject, formula, τ, a, v, t)` — seven arguments = seven fields of a [[cyberlink]]. the [[cybergraph]] is a universal memo cache. see [[nox]]

## zheng — prove & verify

[[SuperSpartan]] IOP + Brakedown PCS + [[sumcheck]]. a fundamentally new proof type covering all five execution regimes through one verification backbone. zero trusted setup, post-quantum, sub-millisecond verification.

every nox computation produces a [[zheng]] proof. recursive composition via field tower F_{p³}. see [[zheng]]

## knowledge — link, store

---

## cybergraph — link

the vertebra. the universal linker. everything in [[cyber]] is [[particles]] connected by [[cyberlinks]] — and the [[cybergraph]] is the totality of these connections.

a [[cyberlink]] is a directed edge from one [[particle]] to another. every assertion, dependency, memo, and fact is a cyberlink. [[neurons]] create cyberlinks by submitting [[signals]] — cyberlinks with stake attached. a signal carries seven fields:

`ask(ν, p, q, τ, a, v, t)` — neuron, from-particle, to-particle, type, amount, valence, time

the stake (field `a`) makes the link weighted. valence (field `v`) makes it signed. together they compose the graph that [[tru]] reads to compute φ* — the attention distribution over all particles. every cyberlink is both a semantic assertion and a vote for importance.

### signal pipeline

every signal fans out to direct readers, then flows downstream:

| component | reads from | produces |
|-----------|-----------|---------|
| [[nox]] | signal: p as formula, q as subject | result particle + [[zheng]] proof |
| [[tru]] | signal: p, q as graph edges; a × v per link | .model artifacts + field state (φ*, eigenvectors) |
| [[bbg]] | signal: all fields | persistent storage across all 10 dimensions |
| [[glia]] | tru: .model artifacts | inference outputs (neural features) |
| [[mir]] | tru: positions + φ* · glia: neural features | [[R-1.0]] world |

`signal.a` is raw stake amount — focus is always computed, never stored. [[tru]] runs the [[tri-kernel]] to convert stake-weighted cyberlinks into φ*.

### neural language

the [[cybergraph]] is the substrate of [[neural]] — the semantic language where meaning emerges from structure. primitives:

| primitive | what |
|-----------|------|
| [[semcon]] | semantic convention — mutual agreement to use the same particles for structuring thought. the grammar of the graph |
| [[sentence]] | ordered set of cyberlinks packed into one transaction — the utterance boundary defines grammar |
| [[motif]] | recurring subgraph pattern encoding relationships: chain, star, diamond, co-citation, triadic closure |
| [[name]] | `~neuron/path` — deterministic resolution of a cyberlink. turns the cybergraph into a dynamic file system |
| [[cyberlink]] as [[particle]] | a link stored as a particle — links about links. negation, qualification, provenance. the language talks about itself |

see [[cybergraph]]

## bbg — store

the Big Badass Graph. one polynomial, all state. BBG_poly(index, key, t) = value. 10 dimensions (particles, axons_out, axons_in, neurons, locations, coins, cards, files, time, signals). cross-index consistency is structural — same polynomial, different dimensions. ~200 bytes per proof, 10-50 μs verification.

bbg is to [[cybergraph]] what a database engine is to a schema. cybergraph defines WHAT. bbg implements HOW. see [[bbg]]

## particles

the atomic unit of the [[cybergraph]]. anything can be a particle — a keyword, an image, a genome, a model, a program. identity = [[hemera]] hash of content. a naked hash with no links never enters the graph — at least one [[cyberlink]] required. meaning comes from four things: cyberlinks (directed edges), path (where it lives in a domain tree), name (human alias), type (declared via extension).

every particle earns a [[cyberank]] — its probability of being observed.

### .cyb — universal knowledge container

one file. self-describing. human-readable index. editable as text. native particle format for [[hemera]]. three rules, frozen:

1. TOML frontmatter until first `~~~`
2. `~~~name` separates every file inside
3. binary files have `size` in frontmatter

```
anything.cyb
├── frontmatter (TOML)     ← what is inside
├── ~~~config              ← text file (readable)
├── ~~~program             ← text file (readable)
├── ~~~weights             ← binary file (size in frontmatter)
└── ~~~image               ← binary file (until EOF)
```

.cyb-compatible extensions:

| extension | what |
|-----------|------|
| .cyb | universal container (this spec) |
| .model | [[tru/specs/model]] — compiled transformer checkpoint |
| .graph | [[cybergraph/specs/graph]] — cybergraph snapshot |
| .vocab | [[tru/specs/vocab]] — particle dictionary |

any .cyb file is a valid hemera particle. `head -50 file.cyb` tells you everything inside.

### [[markup|cybermark]] — address language

eight sigils form the complete address space for navigating particles:

| sigil | name | meaning |
|-------|------|---------|
| `#` | particle | content node — CID or path |
| `@` | neuron | agent, avatar, identity |
| `~` | name | human alias layer |
| `/` | scope | path containment |
| `$` | token | economic unit |
| `^` | root | abstract / generalize |
| `!` | action | execution, verb |
| `.` | pipeline | process-with, transform |

every address resolves to a particle. every connection is a cyberlink. the markup is the graph. see [[markup]]

## intelligence — compile model, run model, render

---

## tru — converge

convergence VM. where [[nox]] derives and [[glia]] executes, tru iterates to a fixed point — φ* emerges, not derived.

| vm | execution model | gödel status |
|----|-----------------|--------------|
| [[nox]] | derivation | confined |
| [[zheng]] | verification | confined |
| [[glia]] | forward pass | confined |
| [[tru]] | field convergence | free |

two jobs, one engine: compile and field.

compile: reads the [[cybergraph]] as a weighted graph and compiles it to a `.model` artifact — the CT-0.1 model that [[glia]] will run. `.graph` is one compiler target; tru is the compiler that understands graphs.

field: runs graph field computation over every signal. reads signal.a (raw stake) and signal.v (valence) → [[tri-kernel]] → φ* (focus distribution). runs the eigensolver (LOBPCG on the screened Laplacian) → particle positions in spectral space. computes [[cyberank]], [[karma]], [[syntropy]].

two outputs:
- runtime state — φ*, eigenvectors, focus → consumed by [[mir]] every epoch
- compiled model — .model artifacts → handed to [[glia]] for inference

tru closes the feedback loop: [[neurons]] create [[cyberlinks]] → bbg stores → tru reads signal.a × signal.v → tri-kernel → φ* → feeds back into memoization, ranking, markets. see [[tru]]

## glia — infer

universal `.model` runtime. graph-agnostic: no knowledge of [[cybergraph]], [[particles]], or [[cyberlinks]]. runs any `.model` → outputs (tensors, features, neural activations).

`.graph` is one compiler target — tru compiles it. glia does not know this. glia receives a `.model` and runs it. the same runtime that executes a graph-compiled model executes any other model.

hardware: [[rane]] (ANE) for NRF head inference; [[acpu]] (AMX) for heavy matrix ops. outputs neural features → consumed by [[mir]]. see [[glia]]

## mir — render

Russian мир: world, peace, community. the thing that makes it physical.

reads two inputs: tru's field state (particle positions, φ*, focus) and glia's inference outputs (neural features). produces the [[R-1.0]] deterministic 3D world — every neuron running mir on the same inputs sees the same world.

mir knows nothing about graphs or models. it receives coordinates and features and makes them visible. rendering tiers T0–T3 (content entry, labels, analytic impostors, Gaussian splats) + T∞ (neural radiance field, Phase 2+). heat-kernel BVH for LOD. epoch/frame split: heavy geometry frozen per epoch, luminosity and flow animate per frame.

hardware: [[aruminium]] (Metal GPU) for all draw calls; [[unimem]] IOSurface for zero-copy frame handoff. see [[mir]]

## network — encrypt, transmit, consensus

---

## mudra — encrypt

post-quantum cryptographic primitives. KEM (key encapsulation), dCTIDH (CSIDH-based key exchange, constant-time isogeny), AEAD (authenticated encryption), TFHE (fully homomorphic encryption over booleans), threshold protocols. consumed by [[plumb]] (private state), [[identity]] (key proofs), [[glia]] (encrypted model weights). see [[mudra]]

## radio — transmit

P2P transport layer. QUIC for reliable encrypted streams, BAO for content-addressed streaming with incremental verification, gossip for signal propagation across the [[cybergraph]]. the nervous system that carries signals between [[neurons]]. see [[radio]]

## foculus — agree

[[collective focus theorem]]: focus topology determines finality. when the φ* distribution converges to a stable attractor, the network has reached consensus. no leader election, no voting rounds — consensus emerges from the same field equations that drive [[tru]]. see [[foculus]]

---

## genesis crystal

the [[cybergraph]] starts empty. core semcons cannot deploy without tokens. tokens cannot exist without the plumb semcon. the [[bootstrap plan]] resolves this with a genesis crystal — a .tri program that runs once with unlimited focus:

```
genesis.tri:
  create_token(CYB, HYDROGEN, VOLT, AMPERE)
  register_semcon(plumb, identity, social, geo)
  distribute(initial_balances)
  // genesis focus expires. normal rules apply.
```

the crystal is the seed structure that determines the growth pattern. without it — empty graph, no rules. with it — economics, types, constraints. even genesis is a proven .tri program.

## core semcons (protocol layer)

the first inhabitants of the spine. consensus-critical [[trident]] programs that define what [[cyber]] IS. not kernel, not apps — protocol.

| semcon | what it defines | stack depth |
|--------|----------------|-------------|
| [[plumb]] | tokens, staking, delegation, conservation, UTXO | zheng (proofs) + bbg (private state) + nox (metering) |
| identity | neuron registration, key proof, ownership | zheng (proofs) + bbg (neuron index) |
| social | following, reputation edges | bbg (social index) |
| geo | location proofs, physical attestation | zheng (geo proofs) |

these are "heavy" semcons that reach deep into the spine — conservation laws in zheng, private state in bbg, metering in nox. [[plumb]] alone requires support from every spine element.

## interface

[[prysm]] is the component library for building the world. atoms → molecules → cells → aips. every component maps to a protocol concept: [[particle]] → content renderer, [[neuron]] → identity card, [[cyberlink]] → navigation action, [[cyberank]] → ordering.

[[optica]] is the 2D renderer and publisher. scanner → parser → graph builder → [[tri-kernel]] → renderer → output. any markdown graph with [[wiki-links]] publishes with `optica serve .`. [[tri-kernel]] ranking, namespace hierarchy, live reload, search index, graph visualization.

[[mir]] is the 3D renderer and publisher. reads tru's field state (particle positions, φ*, focus) and glia's inference outputs (neural features) → produces the [[R-1.0]] deterministic world.

## drivers

hardware abstraction layer — platform-specific backends consumed by the intelligence tier.

| driver | what |
|--------|------|
| [[honeycrisp]] | high performance drivers for apple silicon |

---

[presentation from cosmosverse](https://cyb.ai/oracle/ask/QmTsBLAHC1Lk7n76GX4P3EvbAfNjBmZxwjknWy41SJZBGg)

[video translation](https://www.youtube.com/watch?v=bd_PziPbl74&t=29810s)

discover all [[concepts]]
