---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: nox, cyber/nox, nox vm, three-layer architecture, nox virtual machine
stake: 43936669831471920
---
# nox

the virtual machine of [[cyber]]. sixteen reduction patterns over a [[Goldilocks field]]. every computation in the protocol — identity, linking, ranking, payment, inference, compilation — reduces to these sixteen patterns. the VM produces a [[stark]] proof of correct execution as a byproduct. running a program and proving it ran correctly are the same act.

## lineage

nox descends from [[Nock]] ([[Urbit]]), which descends from combinatory logic (Schonfinkel 1924, Curry 1930s). the lineage is a search for the smallest universal instruction set — the fewest rules that still express all computation.

Nock operates on natural numbers with decrement as its arithmetic primitive. nox replaces natural numbers with [[Goldilocks field]] elements and decrement with field inverse. this is the fundamental mutation: from counting to algebra, from number theory to finite field arithmetic. the consequence: every nox computation is native to [[stark]] arithmetization. there is no translation layer between "the program" and "the proof" — the execution trace IS the algebraic constraint system.

```
combinatory logic (1924)   S, K combinators            pure abstraction
  → lambda calculus (1936) Church's untyped lambda      computable functions
  → Nock (2016)            natural numbers + decrement  deterministic VM for Urbit
  → nox (2026)             field elements + inverse     proof-native VM for cyber
```

each step in this lineage trades generality for structure. nox reaches the point where structure and proof coincide: the instruction set is simultaneously a programming language, an algebraic constraint system, and a content-addressable computation substrate.

## the triple

every nox reduction takes three inputs:

```
reduce(subject, formula, focus) → result
```

subject is what the program knows — the environment, the data, the context. formula is what the program does — the code, the instructions, the transformation. focus is what the program costs — the resource bound, the attention budget.

the subject-formula duality is deeper than code-data separation. in nox, programs are nouns (binary trees of field elements). a formula is a noun. a subject is a noun. the result of applying a formula to a subject is a noun. the same data structure represents knowledge, computation, and output. programs transform data, and programs ARE data.

focus is the metering mechanism. every pattern costs focus. when focus reaches zero, computation halts. this is the resource theory of [[cyber]]: the same [[focus]] that weights [[cyberlinks]] in the [[cybergraph]] also meters computation. attention and computation are the same currency. a [[neuron]] spends focus to think (run nox programs) and to speak (create cyberlinks). the budget is unified.

## nouns: atoms and cells

everything in nox is a noun. a noun is either an atom (a single [[Goldilocks field]] element) or a cell (an ordered pair of two nouns). there is nothing else.

```
noun = atom                     a single field element: 0, 1, 42, p-1
     | (noun . noun)            a cell: an ordered pair

examples:
  7                             atom
  (7 . 13)                      cell of two atoms
  ((1 . 2) . 3)                 cell of a cell and an atom
  ((0 . 2) . ((1 . 0) . (5 . ((0 . 2) . (0 . 3)))))    a program
```

this is the entire data model. a noun is a binary tree where every leaf is a field element. a program is a noun. a subject is a noun. the output of a computation is a noun. a [[cyberlink]] is a noun. a [[stark]] proof serialized for verification is a noun. the [[cybergraph]] state, encoded as nested pairs, is a noun. one structure for everything.

the cell `(a . b)` is the cons operation — pattern 3 in the sixteen patterns. axis — pattern 0 — navigates the binary tree: axis 2 takes the left child (head), axis 3 takes the right child (tail), and deeper paths follow the binary representation (axis 2n = head of axis n, axis 2n+1 = tail of axis n). every data access is tree navigation. every data construction is pairing.

### three types, one representation

atoms carry a type tag distinguishing three uses of the same underlying field element:

```
field (0x00)    arithmetic: a + b, a × b, a⁻¹         range [0, p)
word  (0x01)    bitwise: a XOR b, a AND b, a << n      range [0, 2⁶⁴)
hash  (0x02)    identity: 4 field elements = 256 bits   Hemera output
```

field and word share the same representation (one Goldilocks element) but different operations. a field element wraps around modulo p; a word wraps around modulo 2^64. the distinction is semantic, enforced by the type system: bitwise operations on a hash are an error, arithmetic on a hash (except equality) is an error. the type tag costs nothing in the [[stark]] — it is a constraint selector, not runtime data.

the hash type (four field elements) is the identity primitive. `H(noun)` produces a hash. `axis(s, 0)` returns `H(s)` — a noun can introspect its own identity. this is how content-addressing works at the VM level: every noun carries its hash as an implicit property, accessible via pattern 0.

### homoiconicity

a nox formula is a cell `(tag . body)` where tag is the pattern number (0-16) and body contains the operands. a formula is a noun. a subject is a noun. the result is a noun. the distinction between code and data is purely contextual — the same noun can be a subject in one reduction and a formula in another.

this is deeper than conventional homoiconicity (Lisp, Clojure). in those languages, code is data within the language runtime. in nox, code is data at the level of the proof system. the [[stark]] proves that a specific noun (the formula) was applied to a specific noun (the subject). the proof refers to the same binary tree structure that the execution operated on. there is no separate representation for "the circuit" vs "the program" — they are the same noun.

## the three layers

```
┌────────────────────────────────────────────────────────────────────┐
│  LAYER 1: WHAT IS                                                  │
│  sixteen deterministic patterns                                    │
│  confluent, memoizable, parallel-safe                              │
│  both prover and verifier execute identically                      │
│                                                                    │
│  the ground truth of computation                                   │
├────────────────────────────────────────────────────────────────────┤
│  LAYER 2: WHAT MIGHT BE                                            │
│  one non-deterministic pattern: hint                               │
│  prover injects, Layer 1 constraints verify                        │
│  verifier never executes hint — checks via stark                   │
│                                                                    │
│  the origin of privacy and search                                  │
├────────────────────────────────────────────────────────────────────┤
│  LAYER 3: HOW FAST                                                 │
│  five jets: hash, poly_eval, merkle_verify, fri_fold, ntt          │
│  observationally equivalent to Layer 1 compositions                │
│  optimization without changing meaning                             │
│                                                                    │
│  the pragmatics of recursive verification                          │
└────────────────────────────────────────────────────────────────────┘
```

this separation is ontological. Layer 1 defines truth — the irreducible semantics of computation. Layer 2 defines the boundary between prover and verifier — the point where private knowledge enters the system. Layer 3 defines performance — the concession to physics without compromise to meaning.

remove Layer 3: every program still runs, every proof still verifies. slower, identical results. remove Layer 2: the system loses privacy and search — computation becomes fully transparent, no ZK. remove Layer 1: nothing remains.

## sixteen patterns

five structural patterns make the VM Turing-complete. six field arithmetic patterns give it algebraic power. four bitwise patterns give it binary manipulation. one hash pattern gives it identity.

```
STRUCTURAL (5)                       what they provide
─────────────────────────────────    ──────────────────────
0  axis — navigate a noun tree       data access
1  quote — return a literal          constants
2  compose — chain two computations  recursion, all control flow
3  cons — build a pair               data construction
4  branch — conditional evaluation   decision, lazy evaluation

FIELD ARITHMETIC (6)
─────────────────────────────────
5  add    6  sub    7  mul           ring operations
8  inv                               field completeness (division)
9  eq     10 lt                      comparison

BITWISE (4)
─────────────────────────────────
11 xor    12 and    13 not           boolean algebra
14 shl                               shifting (multiplication by 2^n)

HASH (1)
─────────────────────────────────
15 hash                              identity, commitment, Merkle trees
```

every pattern has a purpose. none is redundant. the first five are structurally necessary (axis navigates, quote creates constants, compose enables recursion, cons builds structure, branch decides). the next six are algebraically necessary (a [[stark]] operates over a field, so the VM must speak the field's language natively). the four bitwise patterns handle the binary world (network protocols, bit manipulation, flag testing). the hash closes the loop — a nox program can compute identities, build Merkle trees, derive commitments, all within the same sixteen-pattern framework.

the choice of exactly sixteen is deliberate: four bits index the pattern. the encoding is dense. a nox program is a binary tree where each internal node's tag fits in a nibble. this compactness matters for content-addressing — shorter programs hash faster, cache more efficiently, transmit more cheaply.

## confluence

Layer 1 patterns form an orthogonal rewrite system: each pattern has a unique tag, left-hand sides are linear (no variable appears twice), patterns are non-overlapping. by the Huet-Levy theorem (1980), orthogonal term rewriting systems are confluent without requiring termination.

confluence means: the result depends only on what the program IS, never on how it was evaluated. parallel reduction, lazy reduction, eager reduction, any mixture — the answer is the same. this is why nox programs are content-addressable. `H(subject, formula)` uniquely determines `H(result)`, regardless of which machine computed it, when, or in what order.

this property propagates upward. a [[cyberlink]] is a nox computation. the cyberlink's identity (its hash) determines its output. two nodes that independently compute the same cyberlink produce the same result. the [[cybergraph]] is a deterministic function of its inputs, verified by anyone, reproducible everywhere.

Layer 2 (`hint`) breaks confluence intentionally — multiple valid witnesses may satisfy the same constraints. this is the non-determinism that makes [[zero knowledge proofs]] possible. soundness is preserved: any witness that passes the Layer 1 constraint check is valid.

## hint: the boundary of knowledge

`hint` is one instruction, but it is the entire mechanism of privacy, search, and oracle access.

the prover knows something the verifier does not. the prover injects that knowledge via `hint`. Layer 1 constraints verify it. the verifier checks the [[stark]] proof — which confirms the constraints were satisfied — without ever learning what was injected.

```
identity:       hint injects the secret behind a neuron address
                Layer 1 checks: Hemera(secret) = address
                verifier learns: address is valid. nothing else.

private transfer: hint injects record details (owner, value, nonce)
                  Layer 1 checks: conservation, ownership, nullifier freshness
                  verifier learns: transfer is valid. nothing about who or how much.

AI inference:   hint injects neural network weights
                Layer 1 checks: forward pass with those weights produces claimed output
                verifier learns: inference is correct. weights remain private.

optimization:   hint injects an optimal solution
                Layer 1 checks: solution satisfies constraints AND is optimal
                verifier learns: result is correct. search strategy is private.
```

[[trident]]'s `divine()` compiles to `hint`. in quantum compilation, `hint` maps to a quantum oracle query — Grover's algorithm turns O(N) witness search into O(sqrt(N)). the same instruction bridges classical proving, zero-knowledge privacy, and quantum search.

## content-addressed computation

because Layer 1 is confluent and programs are nouns, every computation has a canonical identity:

```
key:   (H(subject), H(formula))
value: H(result)
```

this is a global, permanent, verifiable cache. any node in the network that has ever computed the same program on the same data can share the result. the cache entry is universal — it is true for every evaluator, on every machine, for all time.

the cache is the seed of a planetary computation substrate. as more nodes compute more programs, the cache grows. common computations (identity verification, link validation, rank updates) are computed once and cached forever. the network converges toward a state where routine operations are memory lookups rather than recomputations.

Layer 2 computations (`hint`-containing) are excluded from the global cache — their results depend on the prover's private knowledge. pure subexpressions within a hint-containing computation remain memoizable.

## every execution is a proof

the nox execution trace — the sequence of register states across all reduction steps — is directly the AIR (Algebraic Intermediate Representation) for [[stark]] proving. there is no separate "compile to circuit" step.

```
nox execution trace          →    stark witness
register state at each step  →    trace row
pattern tag                  →    constraint selector
pattern semantics            →    transition constraint polynomial
```

each of the sixteen patterns becomes an AIR transition constraint. pattern 5 (add): `reg[out]_{t+1} = reg[a]_t + reg[b]_t`. pattern 7 (mul): `reg[out]_{t+1} = reg[a]_t × reg[b]_t`. pattern 15 (hash): Poseidon2 round constraints spanning consecutive rows at degree 7. the [[SuperSpartan]] IOP handles all degrees natively via [[CCS]].

the entire trace encodes as one multilinear polynomial over the [[Goldilocks field]]. [[WHIR]] commits it. [[sumcheck]] verifies the constraints. the output: a [[stark]] proof that the program ran correctly. the proof is ~60-157 KiB regardless of how large the computation was.

this is why the field choice matters so deeply. nox arithmetic IS [[Goldilocks field]] arithmetic. the execution trace IS a table of Goldilocks elements. the stark proof IS over Goldilocks. there is no impedance mismatch at any layer. see [[cyber/stark]] for the complete pipeline.

## jets: optimization without compromise

Layer 3 provides five operations that are semantically equivalent to Layer 1 compositions but execute faster. the selection criterion: the [[stark]] verifier bottleneck.

recursive proof composition requires running the stark verifier inside nox and proving that run. the unoptimized verifier costs ~600,000 patterns. with jets: ~70,000. this 8.5x reduction is what makes recursive composition practical — a proof-of-proof at every block, O(1) on-chain verification for O(N) transactions.

```
jet              │ what it accelerates         │ why it matters
─────────────────┼─────────────────────────────┼──────────────────────────
hash             │ Fiat-Shamir challenges      │ 83% of unjetted verifier
                 │ Merkle tree paths           │ is hash operations
poly_eval        │ constraint evaluation       │ WHIR query verification
merkle_verify    │ commitment tree proofs      │ the single largest cost
fri_fold         │ WHIR folding rounds         │ log(N) rounds per verify
ntt              │ polynomial multiplication   │ commitment, aggregation
```

the jet semantic contract: every jet MUST have an equivalent pure Layer 1 expression producing identical output on all inputs. this is testable — a harness compares jet output against pure-pattern output on random inputs. if a jet is removed, the system remains correct. only slower.

the five jets map to four [[Goldilocks field processor]] hardware primitives (fma, ntt, p2r, lut). the stack is continuous: nox pattern → software jet → hardware primitive. the same computation, three speeds, identical semantics at every level.

## self-verification

the stark verifier for nox is itself a nox program. every operation the verifier needs — field arithmetic, hashing, polynomial evaluation, Merkle path checking — is native to the sixteen patterns (or their jet equivalents).

this closure is the deepest property of the design. the VM can verify proofs about its own executions. a proof-of-proof is a nox program that runs the verifier on a proof. the proof-of-proof is itself provable. recursion to arbitrary depth, constant proof size at every level.

```
program → trace → stark proof → verifier (nox program) → trace → stark proof → ...
```

the system closes on itself. see [[cyber/stark]] for recursive composition details, [[cyber/proofs]] for the full proof taxonomy.

## the name

Nox (Latin: "night"). the dark twin of computation — where execution and proof are indistinguishable, where privacy and verification coexist, where the program disappears into its proof. from Nock (the predecessor) to nox: the same letter shift as from natural numbers to field elements, from counting to algebra, from light to the productive darkness where proofs are born.

see [[nox/spec]] for the formal specification, [[cyber/stark]] for the proof pipeline, [[trident]] for the high-level language, [[Goldilocks field]] for the arithmetic, [[Goldilocks field processor]] for hardware acceleration, [[cyber/proofs]] for the proof taxonomy
