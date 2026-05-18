---
tags: cyber, cyb, language, article
alias: rune language, rune scripting, rune unified, rune k140
crystal-type: pattern
crystal-domain: cyber
crystal-size: deep
---
# rune

[[Rs]] syntax + [[hoon]]'s subject model + [[cybermark]]'s sigil layer. one [[language]], one AST, two syntactic registers — classic (familiar) and pure (alien) — lowering directly to [[Nox]]. provable at every tier: pure code unconditionally, event-reactive code relative to its event log, host calls relative to their results. instant start by design

rune is the language for open computation — programs that interact with the world and are proven relative to their interaction history. [[trident]] closes over its inputs and proves unconditionally. rune opens to the world via three extensions: `hint` (event stream), `eval` (runtime formulas), `host` (external compute) — each with its own proof contract. `host` is the only genuine trust boundary, where the external computation itself carries no Nox trace

---

## three parents

### from [[hoon]] — the model

the model is correct. the surface taxes adoption. take the model, keep the original surface as the pure register alongside a familiar classic register

| inherit | what it gives |
|---------|--------------|
| subject-oriented evaluation | environment IS a noun — modules, state, code, all in one tree |
| fixed-arity keywords | precedence disappears |
| symbol-only keywords | alphabetic names are always free, no reserved words |
| tall/flat duality | one grammar at any scale |
| cores — gates, doors, traps | functions, objects, thunks unified as `[code data]` |
| molds as functions | types are runtime values that also drive static checking |
| wet and dry gates | generics via inlined re-typecheck, no higher kinds |
| reference + jets | spec stays small and correct; jets are fast and verified equivalent |
| Kelvin versioning | the spec freezes by design |
| solid-state interpreter | event log + subject = replayable, hot-upgradable kernel |
| eval as base operation | `.*` is one digraph, no special `eval()` form |

### from the current rune — the surface

what rune today gets right: a familiar surface running on [[Nox]]

| inherit | what it gives |
|---------|--------------|
| Rust-shape syntax option | minutes to onboard anyone with ALGOL or Rust background |
| `hint` for async | explicit yield/resume tied to [[cybergraph]] events |
| `host()` for FFI | clear boundary to WASM, wGPU, ONNX |
| `eval()` ergonomic form | syntax sugar over `.*` for everyday code |
| instant start | tree construction stands in for compilation — no build phase between source and run |
| three jet categories | pure (proven), host (practical), hint (async) |

### from [[cybermark]] — the address layer

cybermark already solved the addressing problem. its eight sigils are the noun-layer keywords. the computation language extends the same philosophy into the verb layer

| inherit | what it gives |
|---------|--------------|
| `#` particle, `@` neuron, `~` name | address sigils ARE values in every expression context |
| `/` scope, `^` root, `*` wildcard | path traversal is computation traversal |
| `!` action, `.` pipeline | verbs are first-class |
| sigil-based, language-neutral | works equally in Russian, Mandarin, English |
| frozen sigil set | the address language commits to a fixed alphabet |

---

## the central idea

one [[language]] with one AST, parsed from two syntactic registers. both lower to the same [[Nox]] noun. files can mix registers freely. `rust` gets you onboarded; `rune` gives you the full power; nothing in between dilutes the choice

| register | tag | who writes it |
|----------|-----|--------------|
| `rust` | `rust` | every Rust, Go, or TypeScript programmer day one |
| `rune` | `rune` | systems programmers, [[semcons|semcon]] authors, agent kernels |

a `rune fmt --to=rune` converts mechanically. nothing forces movement. both coexist in the codebase forever. the gradient from familiar to alien is a single step, taken when the team is ready

---

## subject — the noun that holds everything

every rune evaluation runs against a subject — a [[Nox]] noun containing the entire visible environment. inherited from [[hoon]]

```
subject = [
  ~self  : @neuron       :: who I am
  ~now   : @da           :: current time
  ~here  : #             :: where in the graph I am running
  ~caps  : capabilities  :: what I am allowed to do
  ~code  : core          :: the program itself
  ~libs  : core          :: imported libraries
  ~mem   : ...           :: persistent state
  ~world : graph-slice   :: the cybergraph I can see
]
```

every name resolves through the subject. `~self` finds the neuron slot. `~mem.counter` reads from persistent state. `#cyber/truth` resolves through `~world` then path. address resolution is just tree traversal

three consequences fall out of this single design choice:

1. metaprogramming is trivial — the program lives in `~code`, you manipulate it like any other noun
2. eval is trivial — evaluate any formula against any subject via `.*`
3. hot upgrade is trivial — receive an event whose payload includes new `~code`, install with `=.`

the subject IS the runtime environment, the graph view, the persistent state, and the program — one noun

### cybermark unifies with subject traversal

`#cyber/truth` is a wing through the subject's `~world` slot. the address language and the computation-traversal language collapse into one mechanism. when you write `#cyber/truth.rank`:

1. `#cyber/truth` resolves through `~world` in the subject
2. `.rank` pipes through a computation

the dot is the same dot. addresses ARE computations against the subject. this is the deep unification — cybermark and the language stop being two things

---

## computation digraphs

the sigil alphabet, inline computation, and address grammar are defined in [[cybermark]]. digraphs compose two characters: the first character (family) carries the atomic semantic from the [[cybermark]] unified alphabet; the second character (variant) determines the shape within that family

### variant axis (second character)

| char | name | shape within family |
|------|------|---------------------|
| `-` | hep | minimal — the bare, simple, degenerate form |
| `=` | tis | explicit — with binding, with comparison |
| `*` | tar | generic — wildcard, broadcast, polymorphic |
| `+` | lus | augmented — more arguments, larger arity |
| `/` | fas | structural — scoped, path-aware |
| `.` | dot | composed — transform, pipeline |
| `:` | col | paired — two-branch, two-way |
| `>` | gar | flowing — sequential composition |
| `~` | sig | null — absent, optional, empty |
| `^` | ket | meta — abstract, lifted |
| `&` | pam | combined — trace, accumulate |
| `_` | cab | reversed — alternative, mirror |
| `%` | cen | rich — multi-part, structured |
| (doubled) | — | canonical — the prototype of the family |

### the 25 digraphs

| digraph | family × variant | what it makes | arity |
|---------|------------------|---------------|-------|
| `\|=` bartis | core × explicit | gate (with sample) | 2 |
| `\|*` bartar | core × generic | wet gate | 2 |
| `\|-` barhep | core × minimal | trap (no sample) | 1 |
| `\|%` barcen | core × rich | door (multi-arm) | 1 |
| `=/` tisfas | bind × structural | let binding | 3 |
| `=.` tisdot | bind × composed | rebind subject slot | 3 |
| `=>` tisgar | bind × flowing | compose right against left | 2 |
| `?:` wutcol | test × paired | if-then-else | 3 |
| `?-` wuthep | test × minimal | switch by tag | 2 |
| `?=` wuttis | test × explicit | pattern match with binding | 2 |
| `?~` wutsig | test × null | null test | 3 |
| `:-` colhep | cell × minimal | pair | 2 |
| `:+` collus | cell × augmented | triple | 3 |
| `:_` colcab | cell × reversed | flipped pair | 2 |
| `^-` kethep | cast × minimal | cast to explicit mold | 2 |
| `^+` ketlus | cast × augmented | cast to value's mold | 2 |
| `.*` dottar | eval × generic | evaluate formula against subject | 2 |
| `.+` dotlus | eval × augmented | increment | 1 |
| `.^` dotket | eval × meta | scry — read from graph | 2 |
| `~&` sigpam | hint × combined | trace | 2 |
| `~_` sigcab | hint × reversed | hint pair | 2 |
| `/+` faslus | build × augmented | import library | 1 |
| `/-` fashep | build × minimal | import types only | 1 |
| `++` luslus | arm × canonical | arm definition | 2 |
| `!!` zapzap | crash × canonical | crash | 0 |

### why two axes matter

every digraph decomposes. `?-` = test-minimal = the simplest test form (switch). `|*` = core-generic = wet gate. `.^` = eval-meta = scry over the graph layer

instead of memorizing 25 arbitrary digraphs you learn 10 family characters + 13 variant characters + one composition rule. every digraph derives from this. new digraphs added in future kelvins must fit the grid — `?+` would mean "test-augmented" and slots in cleanly

every digraph carries a fixed arity. precedence disappears. tall and flat forms parse identically

### digraphs do not collide with cybermark sigils

cybermark sigils stand alone — `#name`, `@alice`, `~truth` — single symbol followed by a name or path

digraphs are exactly two symbols with no whitespace — `|=`, `?:`, `=/` — followed by code. the parser distinguishes by structure. zero ambiguity, small parser

---

## cybermark — the address layer

the sigil alphabet is defined in [[cybermark]]. the same `#cyber/truth` in a markdown page, a rune program, a trident proof, or an Rs jet signature carries one meaning. address grammar, inline computation (`sigil[expr]`), and reactive documents are all specified in [[cybermark]]

embedding model — each layer adds exactly one thing:

```
cybermark     any text — email, markdown, config, comment, any natural language
    ↓  +inline computation
rune inline   cybermark + sigil[expr] — reactive prose, living documents
    ↓  +block grammar
rune          full language — gates, doors, agents, kernels
    ↓  subset
trident       proven subset — consensus-critical, frozen
    ↓  types
Rs            @nebu, #, @neuron as jet argument type annotations
```

---

## the core model

a `core` is `[battery payload]` — code paired with data. all higher constructs are cores

| construct | shape | what |
|-----------|-------|------|
| gate | `[code [sample context]]` | function |
| door | `[code [sample state context]]` | object with methods |
| trap | `[code context]` | thunk |
| library | `[code context]` | namespace |
| book | `[code [slot-1 slot-2 ... context]]` | record |

calling a gate: replace `sample` slot, evaluate `code` against the resulting core. that is the entire function-call mechanism. tree substitution, no stack, no calling convention

### gates are particles

a gate is a noun. a noun has a [[Hemera|hemera hash]]. a hemera hash is a [[particle]]. therefore every gate is a [[particle]] in the [[cybergraph]] by construction

the cybergraph contains the program text. the program text contains the cybergraph references. self-reference becomes structural rather than syntactic

---

## molds — types as functions

a mold is a gate `noun -> noun` that idempotently normalizes a value to its canonical icon in that type. molds are values — composable, storable, passable — and the compiler also evaluates them statically for type checking

```
@nebu       :: Goldilocks scalar mold
@kuro       :: F₂ tower element mold
@jali       :: ring element mold
@trop       :: tropical semiring element mold
@genies     :: CSIDH-512 element mold
@ud         :: unsigned decimal mold
@p          :: phonetic name mold
@da         :: absolute date mold
#           :: any particle mold
@neuron     :: neuron mold
```

molds extend cleanly across the five [[strata]] algebras. arithmetic is mold-polymorphic via wet gates — `add` works on any algebra, the compiler picks the right [[lens]] backend per call site

---

## data structures as nouns

[[Nox]] nouns ARE the data structures. no heap, no garbage collector — allocation is `cons`, freeing is not referencing

| classic type | [[Nox]] noun | how |
|---|---|---|
| `Vec<T>` | cons-list | `cons(head, cons(head, cons(head, nil)))` |
| `HashMap<K,V>` | Merkle tree | balanced binary tree of key-value pairs, authenticated |
| `String` | [[Hemera]] hash | content-addressed in the [[cybergraph]] — a string IS a [[particle]] |
| `Box<T>` | subtree | `cons(value, nil)` — allocation is tree growth |
| dynamic growth | `cons` | every `cons` creates new structure, immutable |

syntax sugar makes this practical in the classic register:
- `[1, 2, 3]` → `cons(1, cons(2, cons(3, nil)))`
- `{ key: value }` → balanced Merkle tree
- `"text"` → `Hemera(bytes)` — a [[particle]] hash

in the pure register noun literals are written directly: `[1 2 3]`, `[%key value]`, `~[bytes]`

---

## three execution modes

### pure — proven by construction

every reduction in the pure subset has a [[Nox]] trace, which has a [[zheng]] proof. pure rune IS provable computation

```
|=  [x=@nebu y=@nebu]
(add (mul x x) (mul y y))
```

### reactive — event-driven computation

a `hint` yields execution until matching data arrives. the kernel is a pure function `(subject, event) -> (new-subject, effects)` — the event is the witness. [[Nox]] is confluent: reduction order does not affect the result. the proof is conditional on the event log — given the sequence of events as witnesses, every step is fully provable

```
|=  p=#
|-
=/  e   ~hint:%particle.p
=/  r   !rank.p
?:  (gth r 0.5)
  !cyberlink.p.^truth.+1
$
```

the runtime parks the trap. when [[radio]] delivers a matching event or the [[cybergraph]] observes the addressed [[particle]] change, the runtime resumes with the event noun in the appropriate subject slot

### host — the only genuine trust boundary

a host call runs native code outside [[Nox]] — WASM module, wGPU shader, ONNX inference. the call returns a noun. the proof system records "host result was N" as a witness; the surrounding pure computation is provable conditional on that witness. the host computation itself carries no [[Nox]] trace — this is the only real trust boundary

```
=/  features  ~host:%infer.model.input
=/  ranked    (rank-by-features features)
```

the proof spectrum is three tiers:

```
unconditional proof:    any pure digraph reduction, mold normalization, arm lookup
conditional proof       any computation containing hint — proven given the event log
  (event log witnesses):  as witnesses. full provability, external inputs as given
conditional proof       any computation containing host — proven given the host
  (host witnesses):       results as witnesses. trust boundary is the host itself
```

the trace says: given these events and host results, the pure computation was correct. events and host results are witnesses. the [[proof]] verifies everything except the external inputs — which is the best any system can do. the event log is the difference between the two: events are [[cybergraph]]-native and verifiable by anyone; host results require trusting the host runtime

---

## eval is intrinsic

[[Nox]] has `.*` — evaluate formula against subject. eval is one digraph, the same kind as `add` or `cons`. no special form, no permission system at the language level — capabilities live in the subject

```
=/  src   "(mul x 2)"            :: source as text particle
=/  ast   (parse src)             :: parse to noun
=/  form  (lower ast)             :: lower to Nox formula
.*(.  form)                       :: evaluate against current subject
```

this is the core of dynamic execution. macros, interpreters, hot patching, runtime DSLs — all are special cases of `.*`

---

## one model, two registers — examples

every program shown two ways. both parse to the same AST and lower to the same [[Nox]] noun

### double a number

```rust
// rust
fn double(x: @nebu) -> @nebu { x * 2 }
```

```
:: rune
|=  x=@nebu  (mul x 2)
```

### graph reactor — watch, rank, link

```rust
// rust
async fn reactor(p: #) {
    loop {
        let _ = hint::particle_changed(p);
        let r = !rank(p);
        if r > 0.5 {
            !cyberlink(p, ^truth, +1);
        }
    }
}
```

```
:: rune
|=  p=#
|-
=/  _   ~hint:%particle.p
=/  r   !rank.p
?:  (gth r 0.5)
  !cyberlink.p.^truth.+1
$
```

### trading agent — hint + host + pure

```rust
// rust
fn trading_agent(pair: #, model: #) {
    let threshold = 0.7;
    loop {
        let price  = hint::particle_changed(pair);
        let signal = host::infer(model, price.history);
        let conf   = Tri::normalize(signal);
        let sent   = Arc::rank(pair, 100);
        if conf > threshold {
            !cyberlink(pair, ^truth, +1);
        }
        host::gpu(chart_shader, [price, signal, sent]);
    }
}
```

```
:: rune
|=  [pair=# model=#]
=/  threshold  .7
|-
=/  price   ~hint:%particle.pair
=/  signal  ~host:%infer.model.price
=/  conf    (normalize:Tri signal)
=/  sent    (rank:Arc pair 100)
?:  (gth conf threshold)
  !cyberlink.pair.^truth.+1
~host:%gpu.[chart-shader [price signal sent]]
$
```

### a [[neuron]] as a door

a door — multi-arm core with state. each arm is a callable method. neurons in rune are doors

```
|%
++  identity     ~self
++  follow       |=  who=@   !cyberlink:~self:who:+1
++  unfollow     |=  who=@   !cyberlink:~self:who:-1
++  pulse        |-          =.  ~now  ~hint:%tick   $
--
```

### a [[semcon]] for causation

```
|%
++  apply   |=  [s=# o=#]
  =/  c  ~/causes
  [s c o]
++  query   |=  s=#
  (rank (resolve ~/causes) (follow s))
--
```

semcons are doors with `apply` and `query` arms. first-class in the language

### dynamic — load and run code from the graph

```
=/  code   #~/agents/trader            :: load particle by name
=/  ast    (parse code)
=/  form   (lower ast)
=/  sub    [~self ~now ~here ~caps ~mem ~world]
.*(sub form)                            :: evaluate against constructed subject
```

agents stored as [[particles]]. loaded by address. evaluated against a subject the host constructs

---

## instant start — the load-bearing property

between source becoming available and execution starting, only parsing happens. parse → [[Nox]] noun → tree-walk reduction begins immediately. there is no compilation phase, no bytecode generation, no static analysis pass between source and run. the latency budget is bounded by parsing time alone — milliseconds for any human-scale program. the runtime can begin reducing while the rest of source is still streaming in

instant start is the genus of [[rune]], not an incidental optimization. it is what makes [[rune]] useful for:

- REPL-style interaction with the [[cybergraph]]
- agent kernels reacting to events with low latency
- `eval()` of dynamically-built formulas
- hot code reload by subject rewrite
- [[semcons]] evaluated on demand
- the [[cyb/robot]] starting up and being responsive immediately

losing instant start loses the wedge that distinguishes [[cyber]] from Solidity-on-EVM, where every dApp needs an off-chain build step and a deployment ceremony before execution. preserving instant start is non-negotiable. every architectural decision is checked against one question: does this preserve the ability to run the program the moment its source becomes available, with no phase between

---

## workload split

different workloads sit at different points on the launch-vs-throughput spectrum. one execution strategy cannot be optimal for all of them. the architecture handles the split by tiering the same source through different back-ends

| workload | profile | right strategy |
|----------|---------|----------------|
| REPL, scripts, ad-hoc exploration | run once, dies fast | interpret directly |
| event handlers, UI logic | run often per session, short bursts | interpret + jet substitution on hot ops |
| agent kernels ([[cyb/robot]]) | run for years | interpret first, compile in background, swap when ready |
| [[semcons]] (consensus-critical) | called by every [[neuron]], must be deterministic and fast | compile AOT through [[trident]] pipeline, deploy as proven .nox |
| inner-loop primitives | hot, narrow, performance-critical | written in [[Rs]], AOT-compiled, called as jets |

the source language is the same in all five rows. the execution strategy varies. choice is per-particle, opportunistic, and reversible — the user never declares "this is interpreted" or "this is compiled" except as an override

---

## execution architecture

```
rune source (classic or pure)
     │
     ▼  parse (ms)
shared AST (rune-core)
     │
     ▼  lower (ms)
[[Nox]] noun (18 patterns: 16 compute + call + look)
     │
     ├──────────────────────────┐
     ▼                          ▼
tree-walking interpreter    compile pipeline
(default, instant start)        ([[Nox]] → TIR → optimized TASM)
     │                          │
     │                     neural optimizer
     │                     (extended for hint/host/eval)
     │                          │
     │                     proven .nox + jet hints
     │                          │
     │                     [[zheng]] proof (lazy)
     ├──────────────────────────┘
     ▼
[[Nox]] runtime
+ [[Rs]] jets (compiled hot paths)
+ host bridge (WASM, wGPU, ONNX)
+ cybergraph cache (compiled artifacts as particles)
```

three subsystems, each preserves instant start when invoked at the right time

### front-end (shared)

one parser per register, both producing the same AST. AST lowers to a [[Nox]] noun. this step stays in milliseconds. parses are content-addressed by source particle, so re-parses are free across the planetary cache

the AST is itself a noun. the AST IS a [[particle]]. programs and their parses are both addressable by [[cybermark]]

### interpreter back-end (the instant start path)

rune evaluates directly into [[Nox]] tree rewriting. there is no separate rune VM. [[Nox]] is the VM, rune is one of its surfaces

direct [[Nox]] interpretation gives:
- instant start by construction (no intermediate VM to spin up)
- noun representation preserved (cybergraph integration trivial — every value is a particle)
- proof story preserved (every pure trace is provable by [[zheng]])
- one VM to optimize, one VM to maintain, one VM to verify

optimizations that keep the interpreter fast without losing instant start:

- inline caches: arm lookups, slot accesses, mold dispatches cached per call site after first execution
- jet substitution: matching [[Nox]] subtrees swap to native [[Rs]] jets at runtime
- subject pinning: the subject does not change during a call; cache slot offsets
- pre-flattening: cons-list `[1 2 3 nil]` exposes as an array view for sequential access
- escape analysis: short-lived intermediate nouns can live on a stack rather than the heap

these are runtime tricks. none requires a compilation phase. instant start preserved at every step

### compiler back-end (the steady-state path)

same [[Nox]] noun, lowered further to [[trident]]'s TIR. same TIR passes (DCE, inlining, constant folding, algebra-specific lowering). same neural optimizer (extended for hint/host/eval opcodes). output: optimized TASM → final .nox bytecode with annotations identifying jettable subtrees

compilation result is itself a [[particle]]. indexed in the [[cybergraph]] by source particle. reusable across all [[neurons]] — the planetary compilation cache

three new TIR opcodes are added to handle rune's dynamism:

- `hint` — yields execution, resumes on matching event. opaque to optimizer, preserved as-is
- `host` — escapes to WASM/wGPU/ONNX, returns a noun. typed return shape lets the optimizer reason about callers
- `eval` — runs a dynamically constructed formula against a subject. requires runtime interpreter callback from compiled code

these are side-effecting node types. the neural optimizer learns to leave them alone and optimize the pure regions around them. the proof system records host and hint results as witnesses; the pure regions between them produce real [[zheng]] proofs

### the planetary cache

compiled artifacts are [[particles]] in the [[cybergraph]], addressed by source particle. once any [[neuron]] compiles a function, every other [[neuron]] can fetch the compiled form rather than recompile

cache semantics:
- compilation cache key: source particle
- when source particle changes, cache misses, compilation invalidates automatically
- jet identity is itself a [[particle]] — when a jet upgrades, it becomes a new [[particle]], so compiled artifacts that referenced the old jet stay pointing at the old [[particle]] and any new compilation references the new one
- granularity is per-particle: a particle is a coherent unit, easy to cache, easy to address, easy to invalidate

this turns the cybergraph into a global JIT cache. cold starts amortize across the planet

### tier mechanics

| trigger | from | to | latency to caller |
|---------|------|----|-------------------|
| first call | source | parse + lower + interpret | ms |
| second call, cold | interp | interp + inline-cache | none added |
| function hot (>N calls/sec) | interp | submit for compile in background | none — user never waits |
| compile finishes | interp | compiled .nox | swap on next call |
| source particle changes | any | reset to interp | no compile-time wait |
| explicit `#![compile]` pragma | first call | compile then run | one-time compile cost |
| explicit `#![interpret-only]` | always | interp | never compiles |
| jet upgrade (new particle) | compiled | mark stale | next call recompiles in background |

proof generation is lazy. compiled pure regions can produce [[zheng]] proofs, but only when requested. the proof itself is a particle, cached by trace particle. one proof can be reused across every [[neuron]] that needs to verify the same computation

---

## rejected alternatives

reflected here so future contributors do not redebate the choices

### separate conventional stack VM for rune

a hypothetical traditional bytecode VM, JVM-shaped, optimized purely for interpretation speed, was considered and rejected. reasons:

- introduces a third execution model (interpret, compile-to-Nox, stack-VM) — pure complexity tax with no architectural justification
- breaks the noun representation that makes [[cybergraph]] integration trivial — values stop being [[particles]] when they live in a separate VM's stack
- loses provability for everything that goes through it — exactly what [[cyber]]'s architecture refuses
- fragments tooling — debugger, profiler, formatter, type checker all have to handle two VMs

the better answer for "faster interpretation": optimize the [[Nox]] interpreter (inline caches, jet substitution, slot caching). these keep instant start and noun-shape while closing most of the speed gap

### merging rune and trident into one language

not a merger — a superset relationship. [[trident]] is the substrate grammar. rune = [[trident]] grammar + three dynamic forms (hint, host, eval). separate tools, separate deployment contexts, one grammar lineage. see the next section

---

## relation to [[trident]] and [[Rs]]

[[trident]] is the substrate. rune is [[trident]] extended. one grammar lineage, two tools, three roles

| dimension | [[trident]] | rune | [[Rs]] |
|-----------|-------------|------|--------|
| primary purpose | provable consensus-critical code ([[semcons]], on-chain) | dynamic personal/agent code (kernels, scripts, UI) | native-speed jets, hot inner loops |
| type system | mold-based, algebra-aware, mandatory static enforcement | mold-based, structural, optional enforcement | Rust-subset static |
| dynamism | none — total determinism | eval, hint, host as first-class | none |
| provability | mandatory — every program produces a [[zheng]] proof | tiered — pure: unconditional; reactive: conditional on event log; host: conditional on host results | reference + jet equivalence verified at compile time |
| evaluation | AOT compiled to optimized .nox | interpret first, compile in background | AOT compiled to native, deployed as jet |
| launch latency | slow (compile + prove) | instant (parse + lower + walk) | n/a — runs as called |
| audience | protocol authors, semcon writers | agent developers, scripters, neuron operators | runtime engineers, performance specialists |

[[trident]] is the grammar foundation — self-contained, auditable without rune. rune's grammar is [[trident]]'s grammar plus hint, host, and eval. same type representation (molds in both), different enforcement obligation. same [[Nox]] target. different execution contract and deployment context

the execution models are genuinely different and justify two separate tools:
- [[trident]]: compiler + prover — source never runs without a proof
- rune: interpreter first, compiler optional — instant start is the contract

one grammar lineage, two tools:

- rune grammar = [[trident]] grammar + three dynamic forms (hint, host, eval)
- [[trident]] is self-contained; rune depends on [[trident]]'s grammar spec, not its toolchain
- both target [[Nox]] as bytecode
- both compile through TIR — [[trident]] always; rune when taking the compile path
- both benefit from the neural optimizer
- both use [[Rs]] jets for native-speed primitives
- both reference [[particles]] in the [[cybergraph]] directly
- [[trident]] freezes first; rune's pure grammar freezes with it; the dynamic extension layer has its own Kelvin

### why [[trident]] stays dynamic-free

[[trident]]'s proof-mandatory guarantee requires total determinism. the three rune extensions break this:

- hint: proof becomes conditional on the event log — trident requires unconditional proofs
- host: result is an asserted witness — the host runtime carries no [[Nox]] trace
- eval: proof depends on runtime content — formula is unknown at compile time

adding any of these to [[trident]] forces proof obligations to become conditional. [[trident]]'s Kelvin discipline depends on its scope being narrow and total. the dynamic forms live in rune's extension layer exactly so [[trident]] never has to touch them

the tool boundary enforces the separation: a trident binary rejects hint/host/eval at the checker level. a rune binary accepts them and marks those regions as conditionally proven. pure rune code — no dynamic forms — is [[trident]]-compatible and can be verified by the trident checker

### the trident subset — explicit scope

every digraph form in rune is [[trident]]-compatible. the pure register IS the trident grammar. a trident checker accepts any rune file containing only these forms and enforces mandatory static mold checking and proof obligation

trident-native forms — the complete inventory:

| component | what it includes |
|-----------|-----------------|
| all 25 base digraphs | `\|=` `\|*` `\|-` `\|%` `=/` `=.` `=>` `?:` `?-` `?=` `?~` `:-` `:+` `:_` `^-` `^+` `.*` `.+` `.^` `~&` `~_` `/+` `/-` `++` `!!` |
| full subject model | `~self` `~now` `~here` `~caps` `~code` `~libs` `~mem` `~world` |
| full core model | gate, door, trap, library, book |
| full mold system | `@nebu` `@kuro` `@jali` `@trop` `@genies` `@ud` `@p` `@da` `#` `@neuron` and any user-defined mold |
| cybermark address sigils | `#` `@` `~` `/` `$` `^` `!` `.` in noun-position addresses |

rune-only extensions — what the trident checker explicitly rejects:

| form | what it does | why excluded |
|------|-------------|--------------|
| `hint expr` | parks execution until external event | proofs become conditional on event log — trident requires unconditional proofs |
| `host(target, args)` | escapes to WASM/wGPU/ONNX | result is an asserted witness — host runtime carries no [[Nox]] trace |
| `eval(formula)` | executes a runtime-constructed formula | proof depends on runtime content — formula unknown at compile time |

a rune file containing none of the three dynamic forms is [[trident]]-compatible as-is — no translation, same grammar. the trident checker runs it with mandatory static mold enforcement and full proof obligation

this scoping is the direct input to the [[trident]] vocabulary proposal: the molds, subject slots, and semcon primitives listed above are what trident needs to absorb explicitly into its frozen spec

---

## Kelvin discipline

[[trident]]'s grammar is the Kelvin-versioned substrate shared by both tools. starts at K140. rune inherits this Kelvin for its pure subset. rune's dynamic extension layer (hint, host, eval) carries its own Kelvin number and can evolve independently above the frozen core

```
trident  K140    :: working spec, mutations expected
trident  K50     :: production spec, narrow corrections only
trident  K0      :: frozen forever — the planetary commitment

rune-core K0     :: inherits trident freeze — pure rune is frozen when trident is
rune-dyn  K140   :: dynamic extension layer — evolves above the frozen core
```

at trident K0 the pure grammar is the protocol. programs using only pure forms run unchanged for as long as anyone runs a [[Nox]] interpreter. programs using dynamic forms depend additionally on the dynamic layer's Kelvin

surface registers (classic vs pure spelling) can evolve at higher kelvins — adding a syntactic spelling changes neither the AST nor the [[Nox]] noun. the core is what freezes

---

## solid-state interpreter for [[cyb/robot]]

the [[cyb/robot]] is a kernel function

```
kernel : (subject, event) -> (new-subject, effects)
```

state is the subject. inputs are events from the [[cybergraph]], the [[radio]] network, the user. outputs are effects — [[cyberlinks]], signals, messages, UI updates. pure. deterministic. replayable

events persist as a log. state at time `t` is the fold of `kernel` over events up to `t`. crash recovery is replay. backups are log copies. upgrades are events whose payload includes new kernel code installed via `=.` subject mutation

rune is the language that lets you write this. one model — subject + events + kernel function — covers personal compute, agent kernels, [[semcons]], and the [[cybergraph]] participation layer

---

## reference + jets

the AST and the Nox lowering form the canonical spec. jets are the fast implementations

```
add : @nebu @nebu -> @nebu
  :: reference implementation in pure Nox
  :: hint says "use AOCL fused-multiply-add jet"
```

every primitive has a reference Nox formula and an optional jet. the runtime recognizes the formula by structure or hint and substitutes the jet. reference is slow but always correct. jet is fast and verified equivalent. removing a jet preserves semantics, sacrifices speed

[[Nox]] reduction is the universal executor. when reduction hits a recognized pattern the jet mechanism dispatches:

```
Nox reduction (tree rewriting)
  │
  ├── pure jets → proven computation
  │     fma, ntt, p2r, lut, normalize, rank, link...
  │
  ├── host jets → practical computing
  │     ├── wasm(module, fn, args)   → wasmi execution
  │     ├── gpu(shader, data)        → wgpu compute dispatch
  │     └── infer(model, input)      → burn-webnn ONNX
  │
  └── hint → async input from the world
        ├── network event ([[radio]])
        ├── user input ([[cyb]] UI)
        ├── timer (epoch tick)
        └── [[cybergraph]] change ([[particle]]/[[cyberlink]] event)
```

pure jets stay inside [[Nox]] — provable. host jets exit to the host runtime — practical. hints yield to the world — async. the rune program does not distinguish — it is just reducing nouns

---

## implementation plan

walking from where rune is today to the unified architecture is incremental. nothing breaks at any step. each phase ships value on its own

| phase | what lands | what stays preserved |
|-------|------------|----------------------|
| 1 — unified front-end | parser for classic register over a shared AST that lowers directly to [[Nox]] noun | existing rune programs run unchanged |
| 2 — interpreter optimizations | inline caches, jet substitution, slot caching, pre-flattening — purely runtime improvements | instant start; no compilation phase introduced |
| 3 — subject formalization | `~self`, `~here`, `~mem`, `~world` as accessible subject slots | existing rune code unchanged; subject hidden by default |
| 4 — pure register | full sigil grammar via fenced block or `.rune-pure` file | classic still parses; pure is opt-in per particle |
| 5 — TIR extensions | three new TIR opcodes (hint, host, eval), neural optimizer retrained to respect them | trident pipeline unchanged for its own programs |
| 6 — compile back-end for rune | rune source routes through [[trident]] pipeline as a second back-end; output compiled .nox | interpreter back-end remains the default |
| 7 — planetary cache | compiled artifacts published as [[particles]] indexed by source particle; auto-fetch on cache hit | both back-ends still work standalone |
| 8 — profile-guided tiering | hot-path detection, background compilation, transparent swap | user never waits for compilation |
| 9 — lazy proof generation | [[zheng]] proofs produced on demand for compiled pure regions, cached by trace particle | proof-free execution remains the fast path |
| 10 — Kelvin freeze | declare `rune-core` stable at some Kelvin number | surface registers can still evolve above the freeze |

each phase keeps earlier-phase code running. nothing forces movement. instant start is checked at every step

---

## what this gives [[cyber]]

1. one [[language]] across the stack — agents, semcons, kernels, scripts, all in rune
2. Rust-familiar onboarding — new contributors write classic and ship same day
3. expert leverage — advanced developers reach for pure where it pays off
4. provable by construction — every pure expression has a [[Nox]] trace and a [[zheng]] proof
5. graph-native — [[cybermark]] addresses are first-class everywhere
6. eval/async/host unified — three faces of one subject-oriented model
7. hot upgrade — for agents that live a hundred years
8. Kelvin discipline — the protocol's commitment to a frozen spec
9. [[solid-state interpreter]] — the [[cyb/robot]] kernel pattern, native

---

## relation to other layers

[[trident]] and [[Rs]] are covered in detail above. other layers in one line each

| layer | role |
|-------|------|
| [[cybermark]] | the address sigil set — addresses everywhere in rune |
| [[Nox]] | the target — every rune AST lowers to a Nox noun |
| [[hoon]] | the inspiration for the subject model and pure register |
| [[zheng]] | the prover — every pure Nox trace becomes a proof |
| [[cyb/robot]] | the canonical consumer — kernel functions in rune |
| [[semcons]] | first-class rune constructs |
| [[bbg]] | event log storage substrate |
| [[radio]] | source of async hints from the network |

---

## resolved decisions

| question | decision |
|----------|----------|
| tier granularity | per-particle. a particle is a coherent unit, easy to cache, address, and invalidate |
| jet invalidation | jet identity is a [[particle]]. jet upgrade produces a new [[particle]]. compiled artifacts referencing the old [[particle]] stay pointing at the old jet; new compilations reference the new one. no global invalidation event needed |
| proof generation timing | lazy. proofs produced on demand for compiled pure regions, cached by trace particle, reusable across [[neurons]] |
| trident/rune relationship | superset relationship. trident is the grammar substrate; rune extends it with three dynamic forms. separate tools, one grammar lineage |
| separate stack VM | rejected. direct [[Nox]] interpretation preserves instant start, noun representation, provability, tooling unity |
| eval at compiled tier | runtime interpreter callback from compiled code. compiled functions containing eval cannot fully compile — they retain an interp escape at the eval point |

## still open

1. register interleaving — can a single function body mix classic and pure on a per-line basis, or must each block commit? leaning toward per-block via fenced markdown or file-level pragma
2. mold inference across algebras — `@nebu` and `@kuro` are distinct molds; wet gates inline at call site, but does the type system need explicit algebra parameters or can it always infer from sample
3. subject capability model — what restricts which code can read or write subject slots? capability tokens in subject itself? per-block declared imports? security needs design
4. markdown hosting — rune lives primarily inside markdown fenced blocks (graph-native) or as `.rune` files (file-native)? probably both, [[cybergraph]] canonical
5. wet gate caching in compiled mode — wet gates re-typecheck at every call site; each call site becomes effectively a separate function. need dedup by argument-type fingerprint to avoid compilation explosion
6. hint event matching — by type, by selector pattern, by subject slot path? leans toward selector patterns over [[cybermark]] addresses
7. scry pure-or-async — `.^` reads from cybergraph; pure when graph slice is local, hint when remote. unified primitive with mode inferred from address scope, or two distinct primitives
8. host call typing — fully typed (optimizer can reason about return shape) or untyped (worst-case assumptions). leans typed with a typed-void escape for genuinely opaque calls
9. parallel rune at the door level — doors are state-isolated by construction; can a runtime run multiple doors in parallel without coordination? likely yes, with cyberlinks as the only cross-door communication channel

---

discover all [[concepts]]
