---
tags: cyber, cyb, language, article
alias: rune future, rune evolution, rune unified, rune k140
crystal-type: pattern
crystal-domain: cyber
crystal-size: deep
---
# rune/future

what [[rune]] becomes when it absorbs the three correct ideas: [[hoon]]'s subject-oriented evaluation, [[Rs]]'s human-readable surface, [[cybermark]]'s sigil-based address layer. one [[language]] with one model, three syntactic registers — familiar at one end, pure at the other, with a walkable gradient between. compiles to [[Nox]]. provable by construction. dynamic, async, host-capable where the program crosses the [[proof]] boundary

this page is the design vision. the principles are stable; specific spellings may shift as the implementation lands

---

## three parents

### from [[hoon]] — the model

the model is correct. the surface taxes adoption. take the model, keep the original surface as one of three registers

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

### from [[Rs]] and [[rune]] — the surface

what [[rune]] today gets right: a familiar surface running on [[Nox]]

| inherit | what it gives |
|---------|--------------|
| Rust-shape syntax option | minutes to onboard anyone with ALGOL or Rust background |
| `hint` for async | explicit yield/resume tied to [[cybergraph]] events |
| `host()` for FFI | clear boundary to WASM, wGPU, ONNX |
| `eval()` ergonomic form | syntax sugar over `.*` for everyday code |
| millisecond start | tree construction stands in for compilation |
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

one [[language]] with one AST, parsed from three syntactic registers. all three lower to the same [[Nox]] noun. files can mix registers freely. the gradient from familiar to alien is walkable, not a cliff

| register | tag | who writes it |
|----------|-----|--------------|
| classic | `rune` | every Rust, Go, or TypeScript programmer day one |
| mid | `rune-mid` | adopters comfortable with mixed sigils |
| pure | `rune-pure` | systems programmers, [[semcons|semcon]] authors, agent kernels |

a `rune fmt --register=pure` converts mechanically. nothing forces movement. all three coexist in the codebase forever

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

## sigil grammar

two families share the alphabet:

### address sigils (cybermark, unchanged)

| sigil | name | arity | family |
|-------|------|-------|--------|
| `#` | hax | 1 path or CID | particle |
| `@` | pat | 1 name or hash | neuron |
| `~` | sig | 1 name | alias |
| `/` | fas | n segments | scope |
| `$` | bus | 1 symbol | token |
| `^` | ket | 1 concept | abstract |
| `!` | zap | n verb + args | action |
| `.` | dot | n stages | pipeline |

### computation digraphs (new, ordered by family)

| family | sigil | name | arity | what it does |
|--------|-------|------|-------|--------------|
| core | `\|=` | bartis | 2 | dry gate — function |
| core | `\|*` | bartar | 2 | wet gate — generic |
| core | `\|-` | barhep | 1 | trap — recursion point |
| core | `\|%` | barcen | 1 | door — multi-arm core |
| core | `++` | luslus | 2 | arm definition |
| bind | `=/` | tisfas | 3 | let binding |
| bind | `=.` | tisdot | 3 | rebind subject slot |
| bind | `=>` | tisgar | 2 | compose — evaluate right against left |
| test | `?:` | wutcol | 3 | if-then-else |
| test | `?-` | wuthep | 2 | switch by tag |
| test | `?=` | wuttis | 2 | pattern match |
| test | `?~` | wutsig | 3 | null test |
| eval | `.*` | dottar | 2 | evaluate formula against subject |
| eval | `.+` | dotlus | 1 | increment |
| eval | `.^` | dotket | 2 | scry — read from graph |
| cell | `:-` | colhep | 2 | cell pair |
| cell | `:_` | colcab | 2 | flipped cell |
| cell | `:+` | collus | 3 | triple |
| cast | `^-` | kethep | 2 | cast to mold |
| cast | `^+` | ketlus | 2 | cast to value's mold |
| hint | `~&` | sigpam | 2 | trace |
| hint | `~_` | sigcab | 2 | hint pair |
| build | `/+` | faslus | 1 | import library |
| build | `/-` | fashep | 1 | import types |
| crash | `!!` | zapzap | 0 | crash |

every digraph carries a fixed arity. precedence disappears. tall and flat forms parse identically. categorization is regular — first character marks family. `?:` reads as "this is a test." `=/` reads as "this is a binding." visual structure mirrors semantic structure

### digraphs do not collide with cybermark sigils

cybermark sigils stand alone — `#name`, `@alice`, `~truth`. they prefix a noun

computation digraphs are exactly two ASCII symbols with no whitespace between — `|=`, `?:`, `=/`. they prefix code

the parser distinguishes by structure: digraph = two-symbol prefix; cybermark = single-symbol prefix followed by name or path. zero ambiguity, small parser

### pronounceable

every ASCII symbol carries a one-syllable name. `bar`, `wut`, `tis`, `col`, `lus`, `dot`, `sig`, `zap`, `ket`, `fas`, `tar`, `hep`, `hax`, `pat`, `bus`, `pam`, `cab`. every digraph reads as two syllables. `bartis`, `wutcol`, `tisfas`, `dottar`. pair programming over voice works. teaching out loud works. search-by-name works

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
#           :: any particle CID mold
@neuron     :: neuron mold
```

molds extend cleanly across the five [[strata]] algebras. arithmetic is mold-polymorphic via wet gates — `add` works on any algebra, the compiler picks the right [[lens]] backend per call site

---

## three execution modes

inherited from [[rune]] and made first-class in the unified model

### pure — proven by construction

every reduction in the pure subset has a [[Nox]] trace, which has a [[zheng]] proof. pure rune IS provable computation

```
|=  [x=@nebu y=@nebu]
(add (mul x x) (mul y y))
```

### async — hint as event

a `hint` yields execution until matching data arrives. the program is a function of `(subject, event) -> (new-subject, effects)`. this is the solid-state interpreter pattern, with cybergraph events as the input stream

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

### host — escape the proof boundary

a host call runs native code outside [[Nox]] — WASM module, wGPU shader, ONNX inference. the call returns a noun. the proof system records "host result was N" as a witness; the surrounding pure computation is provable, the host call itself is an asserted input

```
=/  features  ~host:%infer.model.input
=/  ranked    (rank-by-features features)
```

pure rune is provable. host is the named, typed, explicit escape

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

## one model, three registers — examples

every program shown three ways. all three parse to the same AST and lower to the same [[Nox]] noun

### double a number

```rust
// classic
fn double(x: @nebu) -> @nebu { x * 2 }
```

```
:: mid
fn double  x:@nebu  ->  @nebu
  *(x 2)
```

```
:: pure
|=  x=@nebu  (mul x 2)
```

### graph reactor — watch, rank, link

```rust
// classic
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
:: pure
|=  p=#
|-
=/  _   ~hint:%particle.p
=/  r   !rank.p
?:  (gth r 0.5)
  !cyberlink.p.^truth.+1
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

## compilation pipeline

```
rune source (any register)
     │
     ▼  parse
shared AST (rune-core)
     │
     ▼  lower
[[Nox]] noun (16 patterns + hints)
     │
     ▼  jets recognized? substitute
runtime execution
```

the AST is small enough to publish as a stable artifact. the AST itself is a noun. so the AST IS a particle. one more level of self-reference: programs and their parses are both particles, addressable by [[cybermark]]

---

## Kelvin discipline

`rune-core` (AST + Nox lowering) is Kelvin-versioned. starts at K140. decreases over time. each decrement is a sealed spec — code written against K100 runs identically against K90

```
rune K140    :: working spec, mutations expected
rune K50     :: production spec, narrow corrections only
rune K0      :: frozen forever, the planetary commitment
```

at K0 the language is the protocol. no syntax additions, no behavior changes, no breakage. programs written today run unchanged for as long as anyone runs a [[Nox]] interpreter

surface registers can evolve at higher kelvins. adding a syntactic spelling does not change the AST, the lowering, or the [[Nox]] noun. the core is what freezes

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

cyber already uses this triangle for [[Nox]]. rune makes it explicit at the language level

---

## transition path

walking from where [[rune]] is today (Rs syntax over Nox with hint+host+eval) to the unified vision is incremental. nothing breaks at any step

| phase | what changes | what stays |
|-------|--------------|------------|
| 0 — today | classic register exists, Rs-shape with extensions | — |
| 1 — formalize subject | introduce `~self`, `~here`, `~mem` as accessible slots | existing rune code unchanged; subject hidden by default |
| 2 — add mid register | `=/`, `?:`, `\|-` available as alternative spellings of `let`, `if`, `loop` | classic still parses, mid is opt-in per block |
| 3 — add pure register | full sigil grammar via fenced block or file pragma | all three registers coexist; AST shared |
| 4 — Kelvin freeze | declare `rune-core` stable at some Kelvin number | surface registers can still evolve above the freeze |

each phase keeps earlier-phase code running. the gradient is real and walkable. teams move at their own pace

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

| layer | role |
|-------|------|
| [[cybermark]] | the address sigil set — addresses everywhere in rune |
| [[Nox]] | the target — every rune AST lowers to a Nox noun |
| [[Rs]] | the inspiration for classic register surface |
| [[hoon]] | the inspiration for the subject model and pure register |
| [[trident]] | a sibling — Tri is field-typed for proofs, rune is general-purpose |
| [[zheng]] | the prover — every pure Nox trace becomes a proof |
| [[cyb/robot]] | the canonical consumer — kernel functions in rune |
| [[semcons]] | first-class rune constructs |
| [[bbg]] | event log storage substrate |
| [[radio]] | source of async hints from the network |

---

## open questions

1. register interleaving — can a single function body mix classic and pure on a per-line basis, or must each block commit? leaning toward per-block via fenced markdown
2. mold inference across algebras — `@nebu` and `@kuro` are distinct molds; wet gates inline at call site, but does the type system need explicit algebra parameters or can it always infer from sample
3. subject capability model — what restricts which code can read or write subject slots? capability tokens in subject itself? per-block declared imports? security needs design
4. markdown hosting — rune lives primarily inside markdown fenced blocks (graph-native) or as `.rn` files (file-native)? probably both, cybergraph canonical
5. wet gate caching — wet gates re-typecheck at every call site; need build-system caching tied to particle hashes for large programs
6. hint event matching — by type, by selector pattern, by subject slot path? leans toward selector patterns over [[cybermark]] addresses
7. scry pure-or-async — `.^` reads from cybergraph; pure when graph slice is local, hint when remote? unified primitive with mode inferred from address scope
8. proof generation cost — pure reduction produces proof; hot loops need batching and proof-on-demand discipline

---

discover all [[concepts]]
