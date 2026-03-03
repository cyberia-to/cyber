# cyb/os

## The Discovery

Every data type that deserves computation deserves its own language. Every data type that deserves perception deserves its own rendering primitive. Every human action is not a gesture but a decision. The operating system is the membrane between all three.

cyb/os is not a conventional operating system. It is a stack of typed universes — nine computation languages compiled through one structural IR, rendered through nine perception primitives, driven by ten decision primitives — all sharing one toolchain, one tree substrate, and one proof system.

---

## The Three Grids

```
COMPUTATION (what the machine thinks)     PERCEPTION (what the human sees)
─────────────────────────────────         ────────────────────────────────
Nox    → trees                            struct    → collapsible tree
Bt     → bits                             pixels    → raster image
Rs     → words                            text      → prose, code
Trident→ fields                           formula   → math notation
Arc    → graphs                           vector    → SVG, paths, curves
Seq    → events                           video     → moving pixels
Ask    → relations                        table     → 2D grid
Wav    → signals                          sound     → audio waveform
Ten    → tensors                          component → nested composition

DECISION (what the human does)
──────────────────────────────
observe   → gather without choosing
filter    → narrow by criteria
select    → choose one from many
rank      → order by preference
compose   → build a new value
split     → one becomes many
merge     → many become one
delegate  → route to another agent
reject    → explicitly not-choose
confirm   → irreversible commit
```

Every computation type has a canonical rendering. A tree computed in Nox naturally displays as a collapsible `struct`. A graph traversed in Arc naturally draws as `vector` paths. A relation queried in Ask naturally fills a `table`. A signal processed in Wav naturally plays as `sound`. The mapping is many-to-many, but the canonical pairing is the path of least impedance — where the shape of the data matches the shape of the display.

Every rendering invites a decision. The human responds not with gestures but with typed decision primitives — select, rank, compose, confirm — each with its own algebra, its own temporal mode, and its own relationship to the computation and perception grids.

---

# Grid 1: Computation — Nine Languages, Nine Universes

## The Map

```
Universe          Language    Type        Algebra           Purpose
─────────────────────────────────────────────────────────────────────
Structure         Nox         Tree        Combinators       Composition
Binary            Bt          Bit         𝔽₂ tower          Circuits
Byte              Rs          Word        Bitwise on 𝔽ₚ     Systems
Field             Trident     Field       Arithmetic on 𝔽ₚ  Proofs
Topology          Arc         Graph       Adjacency         Knowledge
Causality         Seq         Event       Partial order     Ordering
Inference         Ask         Relation    Unification       Reasoning
Continuum         Wav         Signal      Convolution       Sensing
Linear            Ten         Tensor      Contraction       Learning
```

## The Principle

A data type deserves its own language when its algebraic laws are so different from other types that forcing it into a foreign language creates constant impedance mismatch — the programmer spends more time encoding and decoding than computing. Nine fundamental types pass this test. Each inhabits a universe defined by its characteristic algebraic structure. Some universes share a proof system. Some share a compiler. None share semantics.

## The Value Tower — Three Modes of Reference

Byte and Field share the same mathematical substrate — the Goldilocks prime field 𝔽ₚ where p = 2⁶⁴ − 2³² + 1. This substrate provides three atom types that are sufficient for seven of the nine universes.

| Tag  | Name    | Representation      | Valid Range   | Use        |
|------|---------|---------------------|---------------|------------|
| 0x00 | `field` | Single 𝔽ₚ element   | [0, p)        | Arithmetic |
| 0x01 | `word`  | Single 𝔽ₚ element   | [0, 2⁶⁴)     | Bitwise    |
| 0x02 | `hash`  | 4 × 𝔽ₚ elements     | 256-bit digest| Identity   |

All three share the same physical representation. The type tag determines which operations are legal. A `word` is a `field` with a range check. A `hash` is four `field` elements treated as an opaque name.

**Why exactly three?** These aren't three data types arbitrarily chosen. They're three fundamentally different ways to **refer** to a value — and there are only three:

```
field = the value IS the reference     (by content — immediate)
word  = position IS the reference      (by location — index)
hash  = name IS the reference          (by commitment — identity)
```

By what it is. By where it is. By what it's called. That's exhaustive. Every reference in any system reduces to one of these three modes.

**Why three atoms are sufficient for all nine universes:**

Every higher type decomposes into structure (Nox trees) over these three atoms:

```
Edge    = cons(source_hash, cons(target_hash, weight_field))
Event   = cons(event_hash, sequence_word)
Fact    = cons(relation_hash, cons(subject_hash, object_hash))
Sample  = field (amplitude value)
Tensor  = [field; N] (array of values with shape metadata)
```

Arc doesn't need a native `edge` atom. Arc needs **rules** that say: when you cons two hashes with a field, that's an edge, and here's what `neighbors()` means over that structure. The language provides the semantics. The value tower provides the atoms. Nox provides the glue.

Three atoms are **complete** — for one characteristic. The single exception is **Bt**: a bit is genuinely not an element of 𝔽ₚ. It lives in 𝔽₂ — different characteristic, different algebra. That's exactly why Bt has a separate proof system, not just a new type tag.

```
Nox value tower (3 atoms: field, word, hash)
  sufficient for: Rs, Trident, Arc, Seq, Ask, Wav, Ten
  NOT sufficient for: Bt

Bt value tower (separate, 𝔽₂)
  sufficient for: Bt only
```

**The register insight:** A register is not a type and not a temporal mode. A register is the **atom of attention** — the point where computation is happening right now, inside whatever universe you're in. Every universe has registers:

```
Nox:      st0–st15 (Triton VM's 16 stack registers)
Bt:       current bit pattern on the circuit wire
Rs:       the word being operated on
Trident:  the field element on top of stack
Arc:      the current node being visited
Seq:      the current event being processed
Ask:      the current binding being unified
Wav:      the current sample
Ten:      the current tensor slice
```

A register is **presence** — the singular now. `field` is the present value. `word` is the present position. `hash` is the present name. Three ways of being present at a leaf of the tree. Time (stack/heap/stream) is how those present moments connect across succession.

---

## Universe 0 — Structure

### Nox — The Language for Trees

**Type:** Cell | Atom
**Characteristic:** None (pre-algebraic)
**Proof system:** Inherited from the language operating within it

Nox is the continuation of the Nock idea — everything is a binary tree, code is data, computation is tree rewriting. Five structural operations define how values compose regardless of what those values are:

| Op        | Action                          | Analogy              |
|-----------|---------------------------------|----------------------|
| `axis`    | Navigate into a subtree by path | Array index          |
| `quote`   | Treat code as data              | String literal       |
| `compose` | Chain two computations          | Function composition |
| `cons`    | Build a pair                    | Struct constructor   |
| `branch`  | Conditional selection           | If-then-else         |

These five ops compute on **shape**, not values. For arithmetic, hashing, and I/O, Nox delegates to the other languages. Nox is the grammar. The other languages are the vocabulary.

**The critical difference from Nock:** Nock's tree is plain — no authentication. Nox's tree is a **Merkle tree** by construction. Every `cons(a, b)` computes `hash(a, b)` and stores the digest at the parent node. The tree proves its own structure. `axis` produces a Merkle proof as a side effect.

The authentication scheme is abstract — not hardwired to any specific hash function:

```
Nox structural ops (abstract)
         │
         ▼
    Tree trait
         │
    ┌────┼──────────┬───────────┬──────────┐
    │    │          │           │          │
  Tip5  Poseidon2  SHA-256    Verkle     SMT
  Merkle Merkle    Merkle    (IPA/KZG)  (sparse)
```

The `Tree` interface requires exactly three operations from any backend:

1. `commit(left, right) → digest`
2. `open(digest, path) → (value, proof)`
3. `verify(digest, path, value, proof) → bool`

Any cryptographic tree that implements these plugs into Nox. Programs don't change. Only proof shape and cost change. The `hash` type in the value tower (4 × Field = 256-bit digest) is the commitment format. When a better tree shows up, swap the backend — no program touches.

**Relationship to other languages:** Nox is the intermediate representation. Rs and Trident programs compile down to Nox structural operations combined with their respective computational operations. The programmer never writes `cons` or `axis` directly — they write structs, arrays, and match statements, and the compiler emits Nox.

**Why it deserves its own identity:** Without Nox, each language would reinvent composition. With Nox, all languages share one canonical way to build and navigate nested data, ensuring that a value constructed in Rs can be traversed by Trident and vice versa.

**Role in cyb/os:** Nox is simultaneously the structural IR (the grammar all languages compile through) and the node runtime (the production binary that runs the Cyber blockchain).

---

## Universe 1 — Binary

### Bt — The Language for Bits

**Type tower:**

| Type    | Field        | Size   | Native ops           |
|---------|--------------|--------|----------------------|
| `Bit`   | 𝔽₂           | 1 bit  | AND (mul), XOR (add) |
| `Bit2`  | 𝔽₂²          | 2 bits | Extension field ops  |
| `Bit8`  | 𝔽₂⁸          | 1 byte | AES-native           |
| `Bit32` | 𝔽₂³²         | 4 bytes| Hash-native          |
| `Bit64` | 𝔽₂⁶⁴         | 8 bytes| Double word          |
| `Bit128`| 𝔽₂¹²⁸        | 16 bytes| Security parameter  |

**Characteristic:** 2
**Proof system:** FRI-Binius
**Key property:** AND is multiplication. XOR is addition. Both are free.

In the binary tower, boolean logic is the native algebra. There is no impedance between "logic" and "arithmetic" — they are the same thing. What costs is moving up the tower: lifting a `Bit32` to `Bit128` for cryptographic security requires extension field multiplication.

**Primitive operations:**

```
xor(a, b)                    → field addition (free)
and(a, b)                    → field multiplication (free)
not(a)                       → xor with all-ones (free)
rotate_left(x, n: const)     → bit rewiring (free)
rotate_right(x, n: const)    → bit rewiring (free)
shift_left(x, n: const)      → rewire + zero fill (free)
tower_up(x: Bit32) → Bit64   → embed in larger field
tower_down(x: Bit64) → (Bit32, Bit32) → decompose
```

**What Bt cannot do cheaply:** Integer arithmetic. 3 + 5 = 6 in 𝔽₂ (XOR), not 8. To perform actual addition with carry, you must build ripple-carry adders from AND/XOR gates — the exact inverse of Byte/Field where arithmetic is free but bitwise is expensive.

**Use cases:**

- BLAKE3 / BLAKE2 / SHA-256 circuits — rotation + XOR + modular addition (built from gates). Proving legacy hashes in Bt is orders of magnitude cheaper than in Byte/Field.
- Keccak verification — Ethereum state proof verification, bridge to EVM world.
- AES circuits — AES operates natively on 𝔽₂⁸. Proving symmetric encryption is natural here.
- Binary Merkle tree verification — Bitcoin, Ethereum, Git all use SHA-based Merkle trees.
- Binary protocol parsing — verifying that binary-encoded messages conform to a format specification.

**Why a separate universe:** A bit is not an element of 𝔽ₚ. The fields have different characteristic. You cannot embed 𝔽₂ arithmetic faithfully into 𝔽ₚ without range checks on every operation — which is exactly the impedance mismatch that justifies a separate language.

---

## Universe 2 — Byte & Universe 3 — Field (shared substrate)

Byte and Field share the Goldilocks substrate but present opposite mental models. A byte programmer thinks in registers and bit patterns. A field programmer thinks in algebraic constraints. Same representation, opposite intent.

### Rs — The Language for Words

**Primary type:** `word` (tag 0x01)
**Syntax:** Strict Rust subset
**Face:** Systems programming
**Proof system:** STARK (via shared substrate), but the programmer does not see it

Rs is Rust with everything dynamically-sized removed. No heap. No `Vec`. No `String`. No unbounded recursion. Every value has a known size at compile time. Every loop has a known bound. The cost of any program can be determined before execution.

**What Rs keeps from Rust:**

- `u64` (mapped to `word`)
- `bool`
- `[T; N]` fixed-size arrays
- `struct` with named fields
- `enum` with variants
- `match`, `if/else`, bounded `while`
- Functions, modules, explicit types
- Ownership, borrowing, move semantics
- `const` generics for array sizes

**What Rs removes from Rust:**

- `Vec<T>`, `String`, `Box<T>` — heap allocation
- `Rc`, `Arc`, `RefCell` — dynamic ownership
- `dyn Trait` — dynamic dispatch
- Unbounded `loop`, recursion
- `async/await` — unbounded futures
- `unsafe` — manual memory management
- `panic!` — unrecoverable runtime failure

**The hidden truth:** Every `u64` in Rs is secretly a `word` — type tag 0x01 — which is secretly a field element with a range constraint. The programmer writes conventional-looking systems code, but every operation is field-compatible. When proof generation is needed, the compiler already has the algebraic representation. Rs is not a language for bytes. Rs is the **disguise that fields wear to look like bytes**.

**Rs as the bridge from Rust to fields:** A Rust developer writes Rs and sees familiar syntax. They do not need to understand field arithmetic or STARK proofs. When they are ready, they peel back the layer and discover that their `u64` was always a field element — and the step to Trident is just revealing what was already there.

```
Rust        → full language, heap, strings, anything
  ↓ restrict
Rs          → strict subset, bounded, looks like systems code
  ↓ reveal
Trident     → same restrictions, but the field is visible
```

**Primary target:** Nox (the Cyber blockchain node runtime). If Nox is written in Rs instead of full Rust, every line of consensus-critical code lives in the value tower natively — flip the type tag from word→field to generate a STARK proof of the node's execution.

**Use cases:**

- Blockchain node runtime (Nox)
- Consensus-critical logic
- Driver and I/O encoding
- Network protocol handling
- Any systems code that must be deterministic and bounded

### Trident — The Language for Fields

**Primary type:** `field` (tag 0x00)
**Syntax:** Custom (`.tri` files)
**Face:** Provable computation
**Proof system:** STARK (Goldilocks + FRI), explicitly visible

Trident is where the field is visible and the programmer thinks in constraints. Division is exact (multiplicative inverse). Every operation becomes a polynomial constraint in the STARK execution trace. The cost model is constraints, not cycles.

**Core types:**

| Type     | Structure           | Operations                         |
|----------|---------------------|------------------------------------|
| `Field`  | Single 𝔽ₚ element   | +, −, ×, ÷ (exact), comparison     |
| `Digest` | 4 × Field (hash)    | Equality, Merkle authentication    |
| `XField` | 3 × Field (cubic ext)| Extension field arithmetic         |
| `U32`    | Field with range    | Arithmetic with overflow check     |
| `Bool`   | Field ∈ {0, 1}      | Logic                              |

**Trident-only primitives:**

- `divine()` — inject prover witness (secret input)
- `hash()` — Tip5 hash (single instruction, single constraint)
- `merkle_step()` — Merkle proof verification
- `sponge_*` — variable-length hashing
- `seal` — hashed/private event emission

**Layer architecture:**

| Layer | Scope           | Types available           | Compilation targets      |
|-------|-----------------|---------------------------|--------------------------|
| 0     | Execute Anywhere | U32, Bool, structs, arrays | TASM, EVM, CosmWasm, SVM |
| 1     | Prove Anywhere   | + Field, Digest, divine() | TASM (Triton VM)         |
| 2     | Platform Powers  | + chain-specific stdlib   | Single target            |

**Use cases:**

- On-chain logic (Neptune transactions, UTXO validation)
- STARK proof generation for any computation
- Recursive proof verification
- Zero-knowledge programs
- Consensus rule enforcement

**Relationship to Rs:** Same compiler, same IR, same value tower, different syntax and different subset of permitted operations. An `.rs` file and a `.tri` file both produce the same TIR opcodes. The compiler validates different type constraints based on which face the programmer chose.

```
.rs file  → parser (Rust subset) → TIR → TASM / backend
.tri file → parser (Trident)     → TIR → TASM / backend
                                    ↑
                              same IR, same value tower
```

---

## Universe 4 — Topology

### Arc — The Language for Graphs

**Type:** `Node`, `Edge`, `Path`, `Subgraph`, `Weight`
**Characteristic:** None (combinatorial, not algebraic)
**Proof system:** Delegates to Trident/Bt for cryptographic proofs

In every other language in this system, graphs are encoded as something else. In Trident, a graph becomes an adjacency matrix — field elements in a 2D array. In Rs, it becomes a serialized edge list — words in a buffer. In Nox, it becomes nested binary trees — cells and atoms. Every encoding loses something: the matrix loses sparsity awareness, the edge list loses traversal semantics, the tree loses the notion of cycles.

Arc makes graphs first-class. The primitive is not a number or a bit but a connection.

**Primitive types:**

| Type       | What it is                              |
|------------|-----------------------------------------|
| `Node`     | A vertex with an identity (Digest)      |
| `Edge`     | A directed connection between two nodes |
| `Path`     | An ordered sequence of edges            |
| `Subgraph` | A bounded subset of nodes and edges     |
| `Weight`   | A field element attached to an edge     |

**Primitive operations:**

| Op              | Action                                         |
|-----------------|-------------------------------------------------|
| `link(a, b, w)` | Create weighted directed edge                  |
| `walk(start, n)`| Random walk of n steps                         |
| `reach(a, b)`   | Test if path exists                            |
| `neighbors(n)`  | Return adjacent nodes                          |
| `degree(n)`     | Count edges (in/out)                           |
| `contract(sg)`  | Collapse subgraph to single node               |
| `expand(n)`     | Reveal internal structure of contracted node   |
| `rank(g, steps)`| Compute stationary distribution (PageRank)     |
| `spectral(g, k)`| Extract top-k eigenvectors                     |
| `merge(g1, g2)` | Union two graphs with identity resolution      |
| `match(g, pat)` | Subgraph pattern matching                      |

**Why Arc matters for Cyber:**

The cybergraph is not a data structure that *lives inside* a program. The cybergraph *is* the program. Every cyberlink is an `Edge`. Every CID is a `Node`. CYBERRANK is `rank()`. The entire value proposition of Bostrom — decentralized knowledge ranking — is a graph computation forced into non-graph languages.

When a developer writes CYBERRANK in Rust today, they build adjacency lists from serialized storage, iterate manually, track convergence by hand, and serialize back. In Arc:

```
let ranked = rank(cybergraph, 100)
let top = ranked |> sort_by(weight) |> take(10)
```

The compiler decides the representation: sparse matrix for large graphs, adjacency list for small ones, GPU-accelerated for production.

**Interaction with proofs:** Arc decomposes into Trident (field ops for matrix math), Bt (hash verification for node identities), and Nox (tree encoding of topology).

```
Arc program
  ↓ compile
Graph operations decomposed into:
  ├── Trident field ops (matrix multiply, accumulation)
  ├── Bt circuits (hash verification for node identities)
  └── Nox structural ops (tree encoding of graph topology)
```

---

## Universe 5 — Causality

### Seq — The Language for Time

**Type:** `Event`, `Interval`, `Schedule`, `Order`, `Epoch`
**Characteristic:** None (relational, not algebraic)
**Proof system:** Delegates to Trident

Time in distributed systems is not a clock. There is no global clock. There are only events and the relationships between them: this happened before that, these were concurrent, this must eventually follow that. Every other language treats time as a value — a `word` holding a timestamp. But a timestamp is not time. Time itself is the **ordering** — the causal structure that determines what could have influenced what.

**The three temporal modes:**

Time is not one thing. It has three fundamental disciplines — three different structural relationships to the present moment.

```
Stack   = nested time     = depth     = LIFO  = after { after { after } }
Heap    = concurrent time = chaos     = random = concurrent(a, b, c)
Stream  = linear time     = flow      = FIFO  = before(a, b), before(b, c)
```

**Stack (nested time).** Last in, first out. You go deeper, then unwind. Every entry has a matching exit. A function calls a function calls a function, then returns, returns, returns. Time has depth. In Seq: `after(a) { after(b) { after(c) { ... } } }` — nested causality. The innermost event resolves first.

**Heap (concurrent time).** No ordering. Events are created and destroyed independently. Allocation and deallocation happen at arbitrary moments with no structural relationship. Time has chaos. In Seq: `concurrent(a, b, c)` — explicitly no causal relation. Things exist simultaneously without depending on each other.

**Stream (linear time).** First in, first out. Events arrive in order, are processed in order, and flow forward. Time has direction but no depth. In Seq: `before(a, b), before(b, c), before(c, d)` — total linear order. A pipeline.

These are not metaphors. They are the three modes that hardware, operating systems, networks, and user interfaces all independently converge on:

| Domain      | Stack                | Heap                  | Stream              |
|-------------|----------------------|-----------------------|---------------------|
| Hardware    | Call stack           | RAM allocation        | I/O bus             |
| OS          | Process call depth   | Dynamic memory        | Pipes, sockets      |
| Network     | Protocol nesting     | Concurrent connections| Packet flow         |
| Consensus   | Nested validation    | Parallel validators   | Block sequence      |
| UI          | Modal dialogs, undo  | Independent windows   | Scrolling, typing   |

**Primitive types:**

| Type        | What it is                                        |
|-------------|---------------------------------------------------|
| `Event`     | An atomic occurrence with a causal identity        |
| `Interval`  | The span between two events                        |
| `Schedule`  | A set of ordering constraints                      |
| `Order`     | A partial order over events (causal history)       |
| `Epoch`     | A bounded sequence of events (consensus round)     |

**Primitive operations organized by temporal mode:**

| Mode   | Ops                                                    |
|--------|--------------------------------------------------------|
| Stack  | `after(a) { ... }`, `within(interval, body)`          |
| Heap   | `concurrent(a, b)`, `eventually(predicate)`            |
| Stream | `before(a, b)`, `periodic(n, body)`, `until(p, q)`    |
| Any    | `always(predicate)` (invariant across all modes)        |

Additional operations: `since(p, q)` — p has held at every state since q was true. `fork(a, branches)` — a is followed by one of several possible continuations. `merge(events) → event` — multiple causal paths converge.

**The algebra of time:** Events form a **partial order** — not a total order. In a distributed system, two events on different nodes may have no causal relationship. Forcing them into a total order (as blockchains do with block height) destroys information. Seq preserves the partial order and only totalizes when consensus demands it. The key algebraic structure is a **lattice** of causal histories. The meet of two histories is their last common ancestor. The join is their merge point. Consensus is the process of computing joins.

**What Seq cannot do:** Compute values. Seq does not add numbers, hash data, or traverse graphs. It only reasons about *when* and *whether* things happen. Seq is the **conductor** — it tells each language when to play.

**Why Seq is not just a state machine DSL:** State machines describe *what states exist* and *what transitions are legal*. Seq describes *when transitions may fire* and *what temporal relationships must hold*. The temporal dimension is orthogonal to the state dimension.

**Use cases:**

- Consensus protocol specification — block validity as temporal constraints
- cyb/os process scheduling — `periodic(10, recompute_rank)`
- UTXO spending rules — output must be created before consumed (type-level guarantee)
- Timeout and liveness — `within(100_epochs, proof_submitted)`
- Distributed coordination — atomic swaps, threshold signatures, governance votes
- Upgrade governance — `after(vote_passed) { after(delay_period) { apply(upgrade) } }`

---

## Universe 6 — Inference

### Ask — The Language for Relations

**Type:** `Fact`, `Rule`, `Query`, `Substitution`, `Constraint`
**Characteristic:** None (logical, not algebraic)
**Proof system:** Delegates to Trident; internally uses resolution

Every other language in this system transforms values. Ask is the only one that derives **truth**. The primitive is not a number, a bit, an edge, or an event — it is a **relation**: a statement that something holds between things.

In Prolog and Datalog, the insight is: you don't tell the computer *how* to find an answer. You tell it *what is true* and *what follows from what*, and the computer searches for derivations.

**Primitive types:**

| Type           | What it is                                              |
|----------------|---------------------------------------------------------|
| `Fact`         | A ground truth: `link(cid_a, cid_b)`                   |
| `Rule`         | An implication: `reachable(X, Z) :- link(X, Y), reachable(Y, Z)` |
| `Query`        | A question: `?- reachable(bali, X)`                     |
| `Substitution` | A binding of variables to values (the answer)           |
| `Constraint`   | A condition on variables: `X > 0, X < 1000`            |

**Primitive operations:**

| Op                       | Action                                         |
|--------------------------|------------------------------------------------|
| `assert(fact)`           | Add a fact to the knowledge base               |
| `retract(fact)`          | Remove a fact                                  |
| `query(pattern)`         | Find all substitutions satisfying the pattern  |
| `unify(a, b)`            | Find substitution making a and b identical     |
| `derive(rules, facts)`   | Compute fixpoint — all facts derivable from rules |
| `stratify(rules)`        | Order rules by negation layers                 |
| `aggregate(query, op)`   | Compute aggregate over query results           |
| `explain(fact)`          | Return derivation tree (proof trace)           |

**Why Ask is not Arc:** Arc stores and traverses explicit structure — nodes, edges, paths. Arc can tell you "there is an edge from A to B." But Arc cannot tell you "every node reachable from A in two hops that is also linked to by D" without you manually writing the traversal loop. Ask treats the graph as a database of facts and applies rules to derive implicit knowledge:

```
reachable(X, Y) :- link(X, Y).
reachable(X, Z) :- link(X, Y), reachable(Y, Z).
?- reachable(a, X), linked_by(d, X).
```

The engine finds all solutions. You never wrote a loop. Arc is **what is connected** (topology). Ask is **what follows** (entailment). Together they form a complete knowledge system: structure + inference.

**The Datalog restriction:** Full Prolog is Turing-complete — queries can diverge. Ask is Datalog, not Prolog. Bounded inference, guaranteed termination, proof-compatible:

- No function symbols (only constants and variables)
- No negation in recursion (stratified negation only)
- Finite set of derivable facts

This matches Rs's bounded loops and Trident's bounded execution. The strict subset pattern holds across every language.

**Use cases for Cyber:**

Knowledge inference over the cybergraph — today cyberlinks are explicit, someone creates each one. With Ask, you define rules once and the system derives implicit links:

```
similar(A, B) :- link(A, Topic), link(B, Topic), A \= B.
category(X, Cat) :- link(X, Y), category(Y, Cat).
expert(Agent, Topic) :- links_by(Agent, X), about(X, Topic), count > 10.
```

The knowledge graph becomes self-amplifying — explicit links generate implicit knowledge through rules. That's the difference between a database and an intelligence.

Access control — who can do what is naturally a logic problem:

```
can_access(User, Resource) :- owns(User, Resource).
can_access(User, Resource) :- member(User, Group), granted(Group, Resource).
can_access(User, Resource) :- delegated(Delegator, User), can_access(Delegator, Resource).
```

Verified inference — because Ask is bounded, any derivation can be encoded as a Trident computation and proven with a STARK. Zero-knowledge inference over a private knowledge graph: prove "fact X follows from knowledge base K under rules R" without revealing K.

Semantic search — structural queries: "find all concepts that are two links away from 'blockchain' and one link from 'privacy'" — expressed as a rule, resolved by the engine.

Ontology enforcement — rules define well-formedness constraints. Run `derive` and get all violations — the graph validates itself.

---

## Universe 7 — Continuum

### Wav — The Language for Signals

**Type:** `Signal<N>` where N is sample count
**Characteristic:** ℝ approximated over 𝔽ₚ (fixed-point)
**Proof system:** Delegates to Trident

A signal is not an array of numbers. A signal is a **waveform** — a continuous function sampled at discrete points. The natural operations are convolution, filtering, and spectral transformation.

**Primitive types:**

| Type          | What it is                        |
|---------------|-----------------------------------|
| `Signal<N>`   | N-sample waveform                 |
| `Spectrum<N>` | Frequency-domain representation   |
| `Filter`      | Transfer function                 |
| `Window`      | Windowing function (Hann, etc.)   |

**Primitive operations:**

| Op                 | Action                          |
|--------------------|---------------------------------|
| `fft(s)`           | Time → frequency domain         |
| `ifft(s)`          | Frequency → time domain         |
| `convolve(s, f)`   | Apply filter in time domain     |
| `lowpass(hz, s)`   | Frequency cutoff                |
| `resample(s, m)`   | Change sample rate              |
| `correlate(a, b)`  | Cross-correlation               |
| `energy(s)`        | Signal power                    |
| `peak_detect(s)`   | Find amplitude peaks            |

**Use cases:** Sensor data processing (seismic, acoustic, environmental), mesh network signal analysis, audio processing, vibration analysis for construction monitoring, geothermal sensor streams.

**Why separate from Rs:** In Rs, you write nested loops over arrays of `u64`, manually managing FFT butterfly operations and twiddle factors. In Wav, you write `lowpass(440, input)` and the compiler selects the implementation.

**Status:** Library-viable within Rs for most applications. Promote to full language if cyb/os becomes a sensor-heavy platform.

---

## Universe 8 — Linear Algebra

### Ten — The Language for Tensors

**Type:** `Tensor<[D1, D2, ..., Dk]>` where dimensions are compile-time constants
**Characteristic:** Inherited (𝔽ₚ or ℝ approximation)
**Proof system:** Delegates to Trident

A tensor is a multidimensional array where the shape is part of the type. `Tensor<[3, 224, 224]>` is not `Vec<Vec<Vec<f32>>>` — it is a type-level guarantee that every operation preserves dimensional consistency. Shape mismatches are compile errors.

**Primitive types:**

| Type                  | What it is                    |
|-----------------------|-------------------------------|
| `Tensor<shape>`       | N-dimensional array           |
| `Scalar`              | 0-dimensional tensor (= Field)|
| `Vector<N>`           | 1-dimensional                 |
| `Matrix<M, N>`        | 2-dimensional                 |

**Primitive operations:**

| Op                    | Action                         |
|-----------------------|--------------------------------|
| `matmul(a, b)`        | Matrix multiplication          |
| `einsum(spec, ...)`   | Einstein summation (universal) |
| `reshape(t, shape)`   | Reinterpret dimensions         |
| `broadcast(t, shape)` | Expand dimensions              |
| `transpose(t, axes)`  | Permute axes                   |
| `reduce(t, axis, op)` | Collapse axis (sum, max, etc.) |
| `conv2d(input, kernel)`| 2D convolution                |
| `softmax(t, axis)`    | Normalize to probability       |

**Use cases:** ML model inference verification, CYBERRANK as matrix-vector multiplication (PageRank is literally repeated `matmul`), scientific simulation, embedding space operations.

**Why separate from Rs:** The shape algebra is rich enough that generic code (matmul, einsum, broadcasting) requires either dependent types or a dedicated type system. In Rs, every matrix operation is a manual loop with manual bounds checking. In Ten, the compiler verifies dimensional consistency and selects optimal algorithms.

**Status:** Library-viable within Rs/Trident for basic linear algebra. Promote if ML inference verification becomes core.

---

# Grid 2: Perception — Nine Content Primitives

These are the irreducible visual types — the atoms of everything a human can perceive through a screen and speakers. Any UI, any document, any application is a composition of these nine.

| Primitive   | What it is                    | GPU mapping                          |
|-------------|-------------------------------|--------------------------------------|
| `text`      | Markdown, prose, code         | Glyphs via compute shader            |
| `struct`    | JSON, TOML — trees & configs  | Collapsible tree of text glyphs      |
| `table`     | 2D data, CSV                  | Grid of text cells, virtualized rows |
| `vector`    | SVG, paths, Bezier curves     | Path rasterization via Vello         |
| `pixels`    | Raster image                  | Texture upload, GPU sampler          |
| `video`     | Moving pixels                 | Hardware decode, texture per frame   |
| `sound`     | Waveform, audio stream        | Audio pipeline (visual: waveform shader) |
| `formula`   | LaTeX / MathML                | Glyph layout + vector curves via Vello |
| `component` | Composition of primitives     | Nested render pass                   |

**The structural parallel:** `component` is to perception what `Nox` is to computation. Nox composes computations (cons, axis, branch). Component composes renderings (nest, layout, pass). Same structural pattern at different levels.

**Implementation:** The nine primitives are implemented in the cyb browser — pure Rust, GPU-native via wgpu, Vello for vector rasterization. Each primitive maps to a specific GPU pipeline. The `component` primitive enables recursive composition — any primitive can contain any other.

---

# Grid 3: Decision — Ten Decision Primitives

## The Insight

Every human interaction with a computer is not a gesture. It is a **decision**. A tap is not a finger motion — it is a choice. A scroll is not a displacement — it is a rejection of everything above and below the viewport. Typing is not keystrokes — it is a sequence of compositional choices. Strip the physics away. What remains is pure decision structure.

## The Algebra of Decision

Decision theory gives us the formal vocabulary:

| Concept     | What it is                                           |
|-------------|------------------------------------------------------|
| Options     | Finite set of alternatives                           |
| Preference  | Partial order over outcomes (not total — undecided is valid) |
| Belief      | Probability distribution over states (uncertainty)   |
| Utility     | Value assigned to an outcome (field element)         |
| Action      | Irreversible commitment that collapses options to one|
| Regret      | Difference between chosen utility and best available |
| Information | Observation that updates beliefs before choosing     |

The key structural property: **decisions are irreversible**. Computation can be replayed. Perception can be refreshed. But once you commit — sign, send, stake, delete — the state has changed. This irreversibility is what makes decisions fundamentally different from computation and display.

## The Ten Primitives

### 1. Observe — Gather without choosing

The zero-decision. Intake information without committing to any action. Change beliefs without changing state.

```
observe(signal: Stream<Info>) → UpdatedBeliefs
```

Scroll a feed. Read a document. Listen. Watch. Hover to preview. Observe is the only primitive that is fully reversible and leaves no trace. But it updates the agent's internal model — beliefs shift, preferences clarify, new options become visible.

Observe is to decisions what a register is to computation — the present moment of attention before any action is taken.

### 2. Filter — Narrow by criteria

Not choosing a specific item but choosing a **constraint** that eliminates items. The decision is about boundaries, not points.

```
filter(set: Set<T>, predicate: T → Bool) → Set<T>
```

Search. Faceted browsing. Price sliders. "Show me only X." Filter is a decision about what to **not see** — it shapes the option space before select or rank applies.

### 3. Select — Choose one from many

The atomic decision. A finite set of options, pick exactly one. Everything else is rejected.

```
select(options: Set<T>) → T
```

Radio buttons. Menu items. "Which restaurant?" This is the primitive that most UI frameworks treat as the only input type.

### 4. Rank — Order by preference

Stronger than select — not just "which one" but "in what order." Produces a total order over a subset of options.

```
rank(options: Set<T>) → List<T>
```

Drag-to-sort. Priority lists. "What matters most?" Rank reveals more preference information than select — it's select applied transitively.

### 5. Compose — Build a new value

The decision to create something that didn't exist in the option set. Not choosing from alternatives but constructing an alternative.

```
compose(parts: Stream<Part>) → T
```

Typing text. Drawing a path. Recording audio. Writing code. Compose is an open-ended sequence of micro-decisions (each character, each stroke) that accumulates into a new value.

### 6. Split — One becomes many

Divide a single value, resource, or path into parts. The decision is not what to choose but **how to divide**.

```
split(value: T, partition: Partition) → Set<T>
```

Split a payment across recipients. Fork a conversation into threads. Branch a project into parallel workstreams. Allocate attention across tabs. Split is the decision primitive that creates concurrency — after a split, the parts exist independently in heap-time.

### 7. Merge — Many become one

The inverse of split. Take multiple inputs, values, or paths and combine them into a single result. The decision is **how to combine** — which parts to weight, which to discard, how to resolve conflicts.

```
merge(parts: Set<T>, resolution: Conflict → T) → T
```

Combine votes into a result. Merge pull requests. Reach consensus. Aggregate sensor readings into one signal. Merge is the decision primitive that resolves concurrency — after a merge, independent heap-time paths converge into one.

### 8. Delegate — Route to another agent

The decision to not decide. Assign the choice to someone (or something) else. This is not indecision — it's an active choice about **who** should choose.

```
delegate(decision: Decision, agent: Agent) → Pending<T>
```

Share for input. Assign a task. Vote (delegate to the majority). Ask an AI. In cyb/os, delegate is how decisions flow through the social graph — from individual to group, from human to machine, from local to consensus.

### 9. Reject — Explicitly not-choose

The opposite of select. Not "I haven't decided" (that's observe) but "I have decided: **no**."

```
reject(option: T) → ()
```

Dismiss. Skip. Swipe away. Press back. Cancel. Reject is informationally rich — it tells the system (and the knowledge graph) what the agent actively does not want. Scroll-past is implicit reject. Dismiss is explicit reject. Both update preferences.

### 10. Confirm — Irreversible commit

The moment of no return. Everything before confirm is reversible — browse, select, rank, filter, compose, change your mind. Confirm collapses possibility into fact.

```
confirm(choice: T, state: State) → (NewState, Proof)
```

Sign a transaction. Send a message. Deploy a contract. Delete permanently. In cyb/os, confirm can produce a cryptographic proof — a STARK attestation that this decision was made by this agent at this time. Confirm is where decision meets Trident.

**Confirm is the only primitive that is always irreversible.** This makes it structurally unique — it is the backbone of the decision grid, the moment where possibility collapses into fact. Every other primitive can be undone, revised, or abandoned. Confirm cannot.

## The Structural Relations

The ten primitives are not independent. They form a dependency structure:

```
observe          ← gathers information (pre-decision)
  ↓
filter           ← narrows option space
  ↓
select / rank    ← chooses from narrowed space
  ↓
compose          ← builds new value if nothing fits
  ↓
split / merge    ← structures the decision (diverge/converge)
  ↓
delegate         ← routes to another agent if needed
  ↓
confirm          ← irreversible commit
  ↓
reject           ← can interrupt at any stage
```

Not every decision follows the full path. A quick tap is `observe → select → confirm` compressed into one gesture. A complex governance vote is `observe → filter → rank → delegate → merge → confirm` over days or weeks.

## Decision vs. Computation — The Boundary

The nine computation languages answer questions of **truth** — what follows from what, what the value is, what the structure contains. They are objective, reproducible, provable.

The ten decision primitives are fundamentally **subjective**. Select depends on preference. Rank depends on values. Confirm depends on willingness. No computation can derive a decision — it can only inform one.

This is the critical architectural boundary: **the machine computes, the human decides**. The computation grid can suggest, derive, prove, rank. The perception grid can display, highlight, compare, animate. But the decision grid is where agency lives. Making it a computation language would mean the machine decides. That's not cyb/os — that's the system cyb/os replaces.

The exception is `delegate` — the decision to let another agent (possibly a machine) decide. Even then, the delegation itself is a human decision. You choose to trust the AI. The AI doesn't choose to be trusted.

---

# Cross-Grid Connections

## The Canonical Pairings

Every computation language has a natural rendering — the path of least impedance between what the machine computed and what the human should see.

```
Computation      Canonical Render     Why
─────────────    ────────────────     ───
Nox  (trees)     struct               tree displays as collapsible tree
Bt   (bits)      pixels               bit grids display as raster
Rs   (words)     text                 word sequences display as prose/code
Trident (fields) formula              field arithmetic displays as math
Arc  (graphs)    vector               node-link diagrams display as paths
Seq  (events)    video                time-sequenced events display as frames
Ask  (relations) table                fact tables display as grids
Wav  (signals)   sound                waveforms display as audio
Ten  (tensors)   component            composite ML outputs need nested rendering
```

The mapping is many-to-many. A Trident proof can display as `formula` (canonical), `struct` (proof tree), `table` (constraint values), or `text` (explanation). But the canonical pairing is the default — what you see when no override is specified.

## Reference Mode Rendering

The three atoms of the value tower have canonical visual representations:

```
Reference Mode       Atom      Renders as          Interaction
────────────────     ────      ──────────          ───────────
By content (is)      field     formula / number    compute
By location (where)  word      text / label        navigate
By commitment (name) hash      link / icon         follow
```

When a UI element displays a `field`, it shows the value itself. When it displays a `word`, it shows the position. When it displays a `hash`, it shows an identity — a clickable link, a commitment you can follow to its content.

## Temporal Mode Rendering

The three modes of time have canonical renderings and interactions:

```
Temporal Mode        Seq               Renders as      Interaction
────────────────     ───               ──────────      ───────────
Stack (depth)        after { ... }     struct          push/pop, drill in/out
Heap  (concurrent)   concurrent(a,b)   component       independent windows, tabs
Stream (flow)        before(a, b)      video / sound   scroll, play, scrub
```

The user navigates nested time by drilling into and out of collapsible structures (stack). The user perceives concurrent time as independent panels and windows (heap). The user experiences linear time as scrolling feeds, playing media, typing text (stream).

## Decision ↔ Computation Mapping

Each decision primitive naturally invokes specific computation:

| Decision    | Computation                                              |
|-------------|----------------------------------------------------------|
| observe     | Wav — passive signal intake, update beliefs              |
| filter      | Ask — query with constraints, return matching set        |
| select      | Ask — which option satisfies the rules?                  |
| rank        | Ten — preference as vector, sort by utility              |
| compose     | Rs — build the new value, byte by byte                   |
| split       | Arc — partition a graph, route edges to subgraphs        |
| merge       | Arc + Ask — resolve conflicts, merge knowledge           |
| delegate    | Arc — find the right agent in the social graph           |
| reject      | Seq — mark event as rejected, update temporal state      |
| confirm     | Trident — generate STARK proof of commitment             |

## Decision ↔ Perception Mapping

Each decision primitive has a canonical rendering:

| Decision    | Canonical Primitive | Why                                    |
|-------------|--------------------|-----------------------------------------|
| observe     | any primitive      | Whatever is being observed, unchanged  |
| filter      | struct (tree)      | Collapsible facets, nested criteria     |
| select      | table / struct     | Options displayed as enumerated choices |
| rank        | table (sortable)   | Draggable rows, reorderable            |
| compose     | text / vector / sound | The thing being built, live          |
| split       | vector             | Division paths shown as branching lines |
| merge       | vector             | Converging paths shown as joining lines |
| delegate    | vector (graph)     | Agent network, routing shown as edges  |
| reject      | video              | Dismiss animation, removal in time     |
| confirm     | formula            | The commitment shown as formal statement|

## Decision ↔ Reference Mode Mapping

The three value tower atoms connect to decisions by what kind of thing is being decided about:

```
field (content)    → observe, rank, filter
                     Decisions about VALUE — what something is worth,
                     how it compares, what criteria matter

word (location)    → select, compose, reject
                     Decisions about POSITION — which slot to fill,
                     what to put where, what to remove from where

hash (identity)    → confirm, delegate, split, merge
                     Decisions about COMMITMENT — who to trust,
                     what to sign, how to divide, how to combine
```

Confirm is fundamentally a hash operation — it produces an identity commitment. Delegate routes to an identity. Split and merge transform the identity graph — one hash becomes many, many hashes become one.

## Decision ↔ Temporal Mode Mapping

```
Temporal Mode    Decision Primitives           Why
────────────     ────────────────────          ───
Stack (depth)    confirm, compose, filter      Nested: compose is a stack of micro-decisions,
                                               filter narrows scope (push), confirm resolves (pop)

Heap (concurrent) split, merge, delegate       Concurrent: split creates parallel paths,
                                               delegate creates parallel agents,
                                               merge resolves them

Stream (flow)    observe, select, rank, reject Linear: scroll through a feed (observe),
                                               pick as you go (select), order over time (rank),
                                               dismiss as they pass (reject)
```

---

# The Universal Structural Pair: Fork / Join

All three grids share the same fundamental structural operation: **divergence and convergence**. One becomes many, many become one. This is the deepest pattern in cyb/os — the same skeleton wearing three costumes.

```
            fork (one → many)          join (many → one)
            ─────────────────          ─────────────────
Computation  axis (decompose tree)      cons (build pair)
Perception   expand (drill into view)   nest (compose views)
Decision     split (divide choice)      merge (combine choices)
```

The universal vocabulary is `fork` / `join`. Each grid specializes:

- **Nox** says `axis` / `cons` — tree decomposition / tree construction
- **component** says `expand` / `nest` — view decomposition / view composition
- **confirm** says `split` / `merge` — choice divergence / choice convergence

This is not a metaphor. Fork/join is the pattern that process scheduling, git branching, thread parallelism, parliamentary procedure, and neural attention all converge on. It's how structure grows (fork) and how consensus forms (join).

In cyb/os, fork/join connects the three grids:

```
split (decision)  → creates concurrent computation paths
  ↓
axis (computation) → decomposes each path's tree into subtrees
  ↓
expand (perception) → drills into each result for display
  ↓
nest (perception) → composes partial views back together
  ↓
cons (computation) → builds the merged result tree
  ↓
merge (decision) → human resolves which combined result to accept
```

---

# The Decision Loop

The three grids interlock in a continuous cycle:

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  COMPUTE         PERCEIVE         DECIDE          COMMIT        │
│                                                                 │
│  Nine languages  Nine primitives   Ten decisions   Back to      │
│  produce         display           collapse        computation  │
│  options         options           options         with new     │
│                                    to action       state        │
│                                                                 │
│  Nox tree     →  GPU pipeline  →  Human choice → Nox tree      │
│  (typed values)  (rendered)       (irreversible)  (updated)     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

This is the cyb/os event loop. Not `requestAnimationFrame()`. A **decision loop**:

```
loop {
  state   = nox_tree(current)           // authenticated tree
  options = compute(state)              // some universe produces alternatives
  display = render(options)             // canonical primitive shows them
  choice  = decide(human_input)         // decision primitive applied
  proof   = commit(choice, state)       // irreversible, potentially STARK-proven
  state   = update(state, choice, proof)// new tree root
}
```

---

# The Compilation Architecture

```
                    ┌──────────────────────────────────────┐
                    │          Programmer Faces             │
                    │                                       │
                    │  Bt  Rs  Trident Arc Seq Ask Wav Ten  │
                    │  .bt .rs .tri    .arc .seq .ask .wav .ten
                    └──────────┬───────────────────────────┘
                               │
                    ┌──────────▼───────────────────────────┐
                    │         Shared Frontend               │
                    │   Parsing, type checking,             │
                    │   borrow checking, bound checking     │
                    └──────────┬───────────────────────────┘
                               │
                    ┌──────────▼───────────────────────────┐
                    │     Nox Structural IR                 │
                    │     axis, quote, compose,             │
                    │     cons, branch                      │
                    │     + typed computational ops         │
                    │     + Merkle authentication           │
                    └──────────┬───────────────────────────┘
                               │
              ┌────────────────┼────────────────┐
              │                │                │
     ┌────────▼──────┐ ┌──────▼──────┐ ┌───────▼───────┐
     │  Binius/FRI   │ │  Goldilocks │ │   Native      │
     │  Backend      │ │  TASM/FRI   │ │   Backend     │
     │  (Binary)     │ │ (Byte+Field)│ │   (no proof)  │
     └───────────────┘ └─────────────┘ └───────────────┘
          Bt              Rs, Trident       Arc (query),
                                            Seq (scheduler),
                                            Ask (inference),
                                            Wav, Ten
```

## Compilation Routes

| Source  | When proof needed                | When proof not needed     |
|---------|----------------------------------|---------------------------|
| Bt      | → Binius FRI circuit             | N/A (always proving)      |
| Rs      | → TASM → STARK (Byte→Field lift) | → native binary (Nox)    |
| Trident | → TASM → STARK (Field native)    | → WASM/EVM (Layer 0)    |
| Arc     | → decomposes into Trident + Bt   | → optimized graph engine  |
| Seq     | → temporal constraints → STARK   | → scheduler / runtime     |
| Ask     | → derivation trace → STARK       | → Datalog engine          |
| Wav     | → decomposes into Trident        | → native DSP pipeline     |
| Ten     | → decomposes into Trident        | → native BLAS / GPU       |

---

# Universe Interactions

Programs span multiple languages. A single cyb/os operation might flow through six universes, render through three primitives, and involve four decision types:

```
                          COMPUTATION
                          ───────────
Seq:     after(new_cyberlinks_submitted) {
           within(next_epoch, {

Ask:         derive(inference_rules, updated_facts)
               ↓ implicit knowledge discovered

Arc:         let subgraph = neighbors(node, 2)
             let ranked = rank(subgraph, 50)
               ↓ compiles to

Trident:     matrix operations over Field for ranking
               ↓ needs to verify Bitcoin SPV proof

Bt:          BLAKE3 hash circuit verification
               ↓ result flows back to

Arc:         merge(ranked, verified_external_data)
           })
           eventually(rank_updated)
         }
           ↓ stored via
Nox:     cons(result, existing_state)
           ↓ node runtime handles I/O
Rs:      serialize_and_broadcast(state)


                          PERCEPTION
                          ──────────
Arc result    → vector (graph visualization)
Ask result    → table  (derived facts)
Trident proof → formula (constraint summary)
Seq timeline  → video  (epoch progression)
Full output   → component (all nested together)


                          DECISION
                          ────────
User: observe (view the ranked results)
    → filter  (narrow to topics of interest)
    → select  (choose a subgraph to explore)
    → confirm (stake on a ranking outcome)
    → Trident generates STARK proof of the stake
    → new Nox tree root committed
```

---

# The Complete Membrane

```
Grid 1: Computation (what is true)
  Backbone: Nox (structural IR, authenticated trees)
  Faces: Bt, Rs, Trident, Arc, Seq, Ask, Wav, Ten
  Atoms: field, word, hash
  Structural pair: axis / cons (decompose / build)

Grid 2: Perception (what you see)
  Backbone: component (nested render pass)
  Faces: text, struct, table, vector, pixels, video, sound, formula
  Atoms: glyph, pixel, sample
  Structural pair: expand / nest (drill in / compose)

Grid 3: Decision (what you do)
  Backbone: confirm (irreversible commitment)
  Faces: observe, filter, select, rank, compose, split, merge, delegate, reject
  Atoms: option, preference, action
  Structural pair: split / merge (diverge / converge)

Universal pattern: fork / join
```

---

# The Comparison Matrix

| Property        | Nox      | Bt       | Rs       | Trident  | Arc      | Seq      | Ask       | Wav      | Ten      |
|-----------------|----------|----------|----------|----------|----------|----------|----------|----------|----------|
| **Universe**    | Structure| Binary   | Byte     | Field    | Topology | Causality| Inference | Continuum| Linear   |
| **Char**        | —        | 2        | p        | p        | —        | —        | —         | ≈ℝ       | ≈ℝ or p  |
| **Primitive**   | Cell     | Bit      | Word     | Field    | Edge     | Event    | Relation  | Sample   | Shape    |
| **Reference**   | structure| wire     | location | content  | adjacency| succession| entailment| amplitude| index    |
| **Register**    | st0–st15 | wire bit | current word| top of stack| current node| current event| current binding| current sample| current slice|
| **Time mode**   | —        | —        | stack    | stack    | heap     | all three| heap      | stream   | stack    |
| **Free op**     | Navigate | AND, XOR | Index    | Mul, Add | Link     | Order    | Unify     | Convolve | Matmul   |
| **Costly op**   | —        | Carry add| Mod div  | Bitwise  | Spectral | Verify   | Fixpoint  | FFT      | Inverse  |
| **Proof**       | Inherited| Binius   | STARK    | STARK    | Delegated| Delegated| Delegated | Delegated| Delegated|
| **Syntax feel** | IR       | Circuit  | Rust     | Custom   | Query    | Temporal | Datalog   | DSP      | NumPy    |
| **Renders as**  | struct   | pixels   | text     | formula  | vector   | video    | table     | sound    | component|
| **Leaf atoms**  | all three| 𝔽₂ (own) | word     | field    | hash+field| hash+word| hash×3   | field    | field[]  |
| **Human-facing**| No       | Yes      | Yes      | Yes      | Yes      | Yes      | Yes       | Maybe    | Maybe    |

## Decision Primitives Summary

| #  | Primitive | Action                | Reversible? | Time Mode | Comp Language | Perception | Reference |
|----|-----------|-----------------------|-------------|-----------|---------------|------------|-----------|
| 1  | observe   | Gather without choosing | Always    | Stream    | Wav           | any        | field     |
| 2  | filter    | Narrow by criteria    | Yes         | Stack     | Ask           | struct     | field     |
| 3  | select    | Choose one            | Yes         | Stream    | Ask           | table      | word      |
| 4  | rank      | Order by preference   | Yes         | Stream    | Ten           | table      | field     |
| 5  | compose   | Build new value       | Yes         | Stack     | Rs            | text/vector| word      |
| 6  | split     | One becomes many      | Depends     | Heap      | Arc           | vector     | hash      |
| 7  | merge     | Many become one       | Depends     | Heap      | Arc + Ask     | vector     | hash      |
| 8  | delegate  | Route to agent        | Sometimes   | Heap      | Arc           | vector     | hash      |
| 9  | reject    | Explicitly not-choose | Mostly      | Stream    | Seq           | video      | word      |
| 10 | confirm   | Irreversible commit   | **No**      | Stack     | Trident       | formula    | hash      |

---

# Build Order

## Phase 1 — Foundation (Now)

1. **Nox** — Define the 16-pattern structural IR with abstract Merkle authentication
2. **Trident** — Already exists, refine compiler and TIR
3. **Rs** — Strict Rust subset, same compiler backend as Trident, target Nox runtime

## Phase 2 — Expansion (Next)

4. **Arc** — Graph DSL for cybergraph programming. Compiles to Trident for proofs, native engine for queries. Essential for making cybergraph programming natural.
5. **Seq** — Temporal logic for consensus rules and scheduling. Causal dependencies rather than procedural checks. Three temporal modes (stack/heap/stream) built in.
6. **Ask** — Datalog over the cybergraph. Rule-based inference turns explicit links into implicit knowledge. The knowledge graph becomes self-amplifying.

## Phase 3 — Specialization (When needed)

7. **Bt** — Binary circuits for legacy hash verification and cross-chain bridges. Build when Bitcoin/Ethereum interop becomes a priority.
8. **Wav** — Signal processing. Start as Rs library, promote to language if cyb/os sensor workloads justify it.
9. **Ten** — Tensor operations. Start as Rs/Trident library, promote to language if ML inference verification becomes core.

## Perception Layer (Parallel track)

Nine content primitives implemented in the cyb browser — pure Rust, GPU-native via wgpu, Vello for vector rasterization. Each maps to a GPU pipeline. `component` enables recursive composition.

## Decision Layer (Parallel track)

Ten decision primitives implemented as the interaction model for cyb browser. Each decision primitive maps to specific computation languages for resolution and specific perception primitives for display. The decision loop replaces the conventional event loop.

---

# The Thesis

cyb/os rests on three observations and one boundary.

**One.** Every computational universe has a native type whose algebraic laws define how programs think. Forcing computations across universe boundaries creates encoding overhead that scales with complexity. Nine algebras → nine languages.

**Two.** Every perceptual channel has a native format whose rendering laws define how humans see. Forcing display across format boundaries creates visual noise. Nine senses → nine primitives.

**Three.** Every human action is a decision, not a gesture. Decisions have their own algebra: options, preferences, beliefs, commitments. Ten decision types → ten interaction primitives.

**The boundary.** The machine computes, the human decides. Computation produces options. Perception displays them. Decision collapses them to action. The action commits to new state, and the cycle continues.

All values in all universes (except Binary) decompose into three atoms — three modes of reference that are exhaustive:

```
field = the value IS the reference     (by content)
word  = position IS the reference      (by location)
hash  = name IS the reference          (by commitment)
```

These atoms compose through one structural substrate (Nox, authenticated trees). They persist through three temporal modes (stack, heap, stream). They are present through one register — the singular now, the atom of attention where computation happens.

All three grids share one universal structural pair — fork and join — wearing three costumes:

```
Computation:  axis / cons      (decompose / build)
Perception:   expand / nest    (drill in / compose)
Decision:     split / merge    (diverge / converge)
```

The programmer chooses the face. The compiler finds the proof. The renderer finds the pixels. The human makes the choice. Time finds the order.

Nine languages. Nine primitives. Ten decisions. Three atoms. Three times. One fork. One join. One tree. One proof. One operating system.
