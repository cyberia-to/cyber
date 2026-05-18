---
tags: cyber, cyb, core
alias: cybermark, cyber markup, markup language
icon: "\U0000270D"
crystal-type: entity
crystal-domain: cyber
crystal-size: article
---
# cybermark

a markup language for the [[cybergraph]]. text-based, human-readable, graph-native. built on the principle that all [[knowledge]] is [[particles]] connected by [[cyberlinks]] — cybermark is how you write, address, and navigate that structure

cybermark is the address and inline-computation language that sits above the fourteen [[cyb/languages|computation languages]]. it names, navigates, and computes inline. every address in cybermark resolves to a [[particle]]. every connection is a [[cyberlink]]. the markup is the graph

---

## foundations

### everything is a [[particle]]

a [[particle]] is the atomic unit — any text-based thing with a [[Hemera|hemera hash]]. [[particles]] have no inherent type or location. meaning comes from:

1. [[cyberlinks]] — directed edges connecting [[particles]]
2. path — where the [[particle]] lives in a domain tree
3. name — the human alias assigned to it
4. type — declared via dot-extension

### [[cyberlinks]] are the only primitive

all structure — hierarchy, naming, typing, ownership — is expressed as [[cyberlinks]]. the markup language makes these links writable and readable by humans

---

## the unified alphabet

thirteen characters, one atomic semantic each. position determines syntactic role — meaning is invariant across positions

| sigil | name | atomic semantic | address form | inline computation | [[rune]] digraph family |
|-------|------|-----------------|--------------|-------------------|------------------------|
| `#` | hax | content, particle identity | `#path` or `#hash` | `#[expr]` → particle | — |
| `@` | pat | identity, agent | `@neuron` | `@[expr]` → neuron | — |
| `~` | sig | annotation, label, side-info | `~alias` | `~[expr]` annotated value | hint |
| `/` | fas | scope, contain, structure | `/path/segment` | path literal | build |
| `$` | buc | economic, value-bearing | `$token` | `$[amount token]` | — |
| `^` | ket | lift, abstract, establish-as | `^concept` | `^[mold expr]` | cast |
| `!` | zap | effect, imperative | `!action(args)` | reactive subscription | crash |
| `.` | dot | transform, apply | `.step` | `.[expr\|step\|step]` | eval |
| `\|` | bar | composition, code-with-data | — | `\|[gate args]` | core |
| `=` | tis | binding, equivalence | — | `=[name val]` | bind |
| `?` | wut | test, decision | — | `?[cond true false]` | test |
| `:` | col | pair, two-way | `a:b` chaining | `:[a b]` pair | cell |
| `+` | lus | augment, plus | `+1` weight modifier | `+[n]` augment | arm |

eight sigils have address forms (`#`, `@`, `~`, `/`, `$`, `^`, `!`, `.`). all thirteen have inline computation forms. ten are [[rune]] digraph families. five overlap address and digraph (`~`, `/`, `^`, `!`, `.`) — the atomic semantic is the same in both positions

### two positions, one meaning

address form — single sigil prefix followed by a name, path, or hash. no brackets. static: names things in the [[cybergraph]]

inline computation form — single sigil followed by `[expression]`. the bracket switches from address mode to evaluation mode. evaluates a [[rune]] expression and embeds the result in the document

block computation — [[rune]] digraphs use the sigil as the semantic family (first character). see [[rune]] for the full digraph grammar

### combinators

| form | meaning |
|------|---------|
| `*` | wildcard — all instances matching pattern |
| `~/` | home — personal namespace root |
| `../` | parent scope |

### pronounceable names

every sigil has a one-syllable name: `hax` `pat` `sig` `fas` `buc` `ket` `zap` `dot` `bar` `tis` `wut` `col` `lus`

---

## the `#` duality

`#` means [[particle]] in both block and inline contexts. context (line-start vs inline) determines rendering

### block: header and nesting

at line-start, `#` declares the current document node and its depth:

```
# truth
```
→ this document is the [[particle]] `truth` at root depth

```
# cyber

## truth

### market
```
→ tree: `cyber` → `cyber/truth` → `cyber/truth/market`
the header depth maps directly to path depth. the document structure is a tree

### inline: [[particle]] reference

anywhere inside text, `#` is a link to another [[particle]]:

```
the concept of #truth is central to #cyber/rank
```

`#QmXyz...` — reference by hash (immutable, content-addressed)
`#cyber/truth` — reference by path (mutable, human-navigable)

both resolve to the same [[particle]] if the name mapping exists

---

## path and scope `/`

`/` expresses containment. a path is a chain of scopes:

```
cyber/truth/market
```

reads as: `market` scoped under `truth` scoped under `cyber`. the path is semantic context. the same [[particle]] name under different paths is a different instantiation of the same concept

### home scope

```
~/              my neuron's root namespace
~/cyber/truth   particle in my personal cyber/truth scope
~/@alice        alice's home namespace
```

`~/` is the personal root. every [[neuron]] has one

---

## name layer `~`

`~` is the [[cyberlink]] that gives a [[particle]] a human-readable name. it is deterministic: for any [[neuron]], `~name` resolves to one [[particle]]

```
~market            my name for some particle
~/@alice/market    alice's name for market
```

name is separate from path. the same [[particle]] can have:
- a hash (`#QmXyz...`)
- a path (`cyber/truth/market`)
- a name (`~market`)

all three point to the same thing via different resolution layers

---

## [[token]] reference `$`

`$` addresses economic units as first-class [[particles]]:

```
$BOOT                  BOOT token
$BOOT/supply           property of BOOT
$BOOT~hydrogen         BOOT named "hydrogen"
$*                     all tokens
@alice/$BOOT           alice's BOOT balance
```

[[tokens]] live in the same address space as content and agents

---

## actions `!`

`!` is the only verb. everything else navigates or addresses

```
!cyberlink(#Qm1, #Qm2)        create a cyberlink
!mint($LI, @alice)             mint tokens to neuron
!burn($BOOT, #QmXyz)           burn tokens to weight a cyberlink
!rank(^truth)                  compute rank across all truth instances
!search(*/market)              query all market nodes
```

actions are composable with the full address grammar:

```
!rank(*/truth, $BOOT)          rank all truth instances weighted by BOOT
!cyberlink(~/thought, ^truth)  link my thought to the abstract truth concept
```

---

## processing pipeline `.`

`.` chains transformations. it does not change address — it changes rendering:

```
#cyber/truth.graph             render cyber/truth as a graph
#QmXyz.md                      render particle as markdown
~/knowledge.render.map         my knowledge namespace as a visual map
cyber/truth/market.token       market concept typed as token
```

pipeline is composable:

```
cyber/truth/market.render.graph    scope → type → render → visualize
```

---

## inline computation — living documents

`sigil[expr]` evaluates a [[rune]] expression and embeds the result in the document. the bracket switches any sigil from address mode to evaluation mode

| form | produces | example |
|------|----------|---------|
| `#[expr]` | particle | `top concept: #[first (ranked ~world)]` |
| `@[expr]` | neuron | `most connected: @[most-linked ~world]` |
| `~[expr]` | annotated value | `entropy: ~[entropy ~world]` |
| `.[expr\|step\|step]` | pipeline result | `rank: .[#cyber/truth \| rank \| pct]` |
| `?[cond true false]` | conditional text | `market is ?[gth .price .threshold "bull" "bear"]` |
| `=[name val]` | binds into document scope | `=[threshold 0.7]` — `~threshold` usable thereafter |
| `$[amount token]` | token expression | `fee: $[current-fee CYB]` |
| `\|[gate args]` | gate application result | `doubled: \|[double 42]` |

expressions run against the page's [[rune]] subject — `~world` is the local [[cybergraph]] slice, `~self` is the authoring [[neuron]], `~now` is the current time. every inline result IS a [[particle]]. the page itself IS a [[particle]]

### reactive documents

a page mentioning `#cyber/truth` subscribes to that particle. when it changes in the [[cybergraph]], inline expressions depending on it re-evaluate automatically. this is Jupyter's "re-run all cells" — except fine-grained, dependency-tracked at the particle level, and without a run step

---

## dimensional navigation

every [[particle]] exists in four dimensions simultaneously:

```
^truth                   vertical-up:    abstract root concept
cyber/truth              vertical-down:  scoped instance
cyber/*                  horizontal:     siblings in same domain
*/truth                  cross-domain:   all homonyms by name
```

### homonym resolution

same name under different paths is signal, not collision

```
*/market                 all nodes named market across all paths
cyber/*/market           all nodes named market within cyber domain
^market                  abstract root — gathering node for all */market
market/*                 all children of any market node
../market                market in parent scope
```

`^` generalizes: lifts from scoped instance to abstract concept.
`*` enumerates: expands to all matching instances.
together they make name-collision the most powerful navigation primitive in the system

### wikilink syntax

```
[[cyber/truth]]                 link by path
[[#QmXyz]]                      link by CID
[[cyber/truth|truth]]           scoped link, display local name
[[$BOOT]]                       token link
[[@alice]]                      neuron link
[[^truth]]                      abstract concept link
[[*/truth]]                     query link — all truth instances
[[!mint($LI, @alice)]]          inline action link
[[cyber/truth.graph]]           link with render pipeline
```

---

## rendering rules

rendering is path-aware. what you see depends on where you are

### at `^truth` (root concept)

```
[ ^truth ]                       ← primary, full weight
  ├── cyber/truth                ← secondary, lower weight
  ├── bio/truth                  ← secondary, lower weight
  └── philosophy/truth           ← secondary, lower weight
```

### at `cyber/truth` (scoped instance)

```
cyber / truth                    ← breadcrumb always visible

[ cyber/truth ]                  ← primary, full weight

horizontal peers (same domain, solid):
  cyber/market · cyber/rank · cyber/attention

root context (vertical-up, reduced weight):
  ^truth

cross-domain homonyms (dashed, lowest weight):
  philosophy/truth · bio/truth
```

### rendering priority stack

| priority | what | weight |
|---------|------|--------|
| 1 | current node | full |
| 2 | horizontal peers (same domain) | solid |
| 3 | vertical parent (root/abstract) | reduced |
| 4 | cross-domain homonyms | lowest, dashed |

the path is always rendered as a clickable breadcrumb chain. you always know where you are

---

## [[particle]] front-matter

a [[particle]] may declare its own position in the graph:

```
---
path: cyber/truth
type: concept
name: truth
tokens: [$BOOT]
---
```

this is itself a set of [[cyberlinks]] — the front-matter is not metadata separate from the graph, it is graph structure expressed inline

---

## grammar summary

```
:: address forms
particle        #QmXyz | #path/to/concept
neuron          @alice | @QmNeuron
name            ~concept | ~/@alice/concept
home            ~/ | ~/@alice
token           $BOOT | $BOOT/property
action          !verb(args)
pipeline        particle.transform.render
abstract        ^concept
wildcard        */name | domain/* | domain/*/name
parent          ../concept
wikilink        [[target|display]]
header          # name (block) → declares particle + depth

:: inline computation forms  (sigil[rune-expr])
#[expr]         evaluate → particle
@[expr]         evaluate → neuron
~[expr]         evaluate → annotated value
.[expr|steps]   evaluate → pipeline result
?[c t f]        conditional text
=[name val]     bind name into document scope
$[amt tok]      inline token expression
|[gate args]    inline gate application
```

every address in cybermark resolves to a [[particle]]. every [[particle]] is content-addressed. every connection is a [[cyberlink]]. the markup is the graph

---

## relation to [[cyb/languages]]

cybermark is the address and inline-computation language — the shared vocabulary embedded in every layer of the [[cyber]] stack

| layer | what | cybermark role |
|-------|------|---------------|
| [[cyb/languages]] (14) | computation | cybermark addresses their inputs and outputs |
| [[rune]] | language | extends cybermark — adds digraph computation grammar on top of the sigil alphabet |
| [[trident]] | proven subset of rune | uses cybermark sigils as type annotations and graph references |
| [[Rs]] | native jets | uses cybermark mold annotations (`@nebu`, `#`, `@neuron`) in jet signatures |
| any text | prose, email, config | cybermark is embeddable anywhere — parser is the only dependency |

the sigil alphabet defined here IS the shared vocabulary. [[rune]] adds the block computation grammar (digraphs). everything else resolves addresses against the [[cybergraph]]

---

## future work

open proposals and ideas not yet specified. listed in priority order — the first four are architectural and may require breaking changes if added late

### 1. time dimension

the system is purely spatial. [[particles]] are immutable CIDs but versions exist — there is no syntax for temporal navigation:

```
#cyber/truth@2024        truth as it was at a point in time
#cyber/truth@genesis     truth at first cyberlink
#QmXyz~prev              previous version of this particle
```

without time, the graph has no history navigation. this is a fully missing dimension

### 2. typed edges / relation predicates

currently a [[cyberlink]] is just `from → to` with no semantic on the edge itself. relations have meaning that the graph cannot currently express:

```
A is-a B
A contradicts B
A extends B
A cites B
A created-by @alice
```

without [[typed cyberlinks|typed edges]] the graph is rich in nodes but blind about the nature of relations. critical for reasoning, [[inference]], and [[epistemic markets|epistemic markets]]

### 3. queries as [[particles]]

`*/market` is a query but cannot itself be addressed, named, or linked to:

```
~market-map = */market          save query as named particle
[[*/market]]                    transclude live query result
!cyberlink(~market-map, #doc)   link to a live view
```

if queries are not [[particles]], the graph cannot link to live views of itself — only to static content. this makes the system less self-referential than it should be

### 4. negation / anti-link

no way to express that one [[particle]] disputes another. [[epistemic markets|epistemic markets]] specifically require explicit contradiction, not just absence of a link:

```
#cyber/truth ≠ #bio/truth       explicit contradiction
#claim~disputed                 mark as disputed
!anti-cyberlink(#Qm1, #Qm2)    create a weighted counter-link
```

without negation, the graph can represent agreement and [[relevance]] but cannot represent disagreement or falsification. see [[valence]] and [[cyber/truth/false]] for the current solution via [[coupling|ICBS]] markets

### 5. weight and confidence

all inline references are currently equal — weight is only computed post-hoc by [[cyberank]]. authors may want to express confidence or [[relevance]] at write time:

```
#cyber/truth:0.9         high confidence reference
#cyber/truth:?           uncertain, exploratory link
#cyber/truth:!           strong assertion
```

### 6. annotations without modification

no distinction between a standalone [[particle]] and an annotation on another [[particle]]. currently both are just [[cyberlinks]]. a dedicated annotation primitive would let the graph distinguish commentary from original content:

```
@>#cyber/truth           this particle is an annotation of cyber/truth
```

### 7. collections / sets

path nesting gives containment. but an arbitrary set — [[particles]] that belong together without a shared path — has no primitive:

```
{#cyber/truth, #cyber/rank, #cyber/attention}    unnamed set
~trilogy = {#p1, #p2, #p3}                       named set
```

tags in other systems partially solve this. may also be expressible via a [[token]] ($) representing set membership

### 8. permissions and visibility

all [[particles]] are currently public by default. no syntax for access control:

```
#cyber/truth!private             only my neuron
#cyber/truth!cohort              scoped to a group
#QmEncrypted.decrypt(@alice)     encrypted particle
```

### 9. [[proof]] and attestation

`@` identifies a [[neuron]] but does not express cryptographic [[proof]] of authorship. for trust in [[epistemic markets|epistemic markets]], signed [[particles]] need markup-level expression:

```
#QmXyz@signed:@alice             particle attested by alice's key
#QmXyz@verified                  protocol-verified authorship
```

### non-issues (resolved by design)

language / locale — translation is a rendering artifact. any [[particle]] can be rendered in any language on the fly. no syntax needed; locale belongs in the view layer, not the address layer

discover all [[concepts]]
