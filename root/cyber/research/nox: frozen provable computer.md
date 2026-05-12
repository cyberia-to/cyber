---
tags: cyber, research, article
crystal-type: process
crystal-domain: cyber
alias: nox research, computation as linking, provable computer
---

# nox: frozen provable computer

computation IS [[cyberlinks|linking]]. evaluating a program IS creating an edge in the [[cybergraph]]. the execution trace IS the [[zheng]] proof witness. the structural hash of the result IS the [[particles|particle]] identity. no boundary between "compute" and "record."

## pure functions with state access

nox programs are pure functions — no side effects, no I/O, no mutable state. yet they have full read access to the entire [[bbg]] state via look(namespace, key). this resolves the classic tradeoff: pure functions are provable but useless without state. impure functions access state but are unprovable.

nox is both: pure (deterministic, provable, composable) AND stateful (reads any key from polynomial state). write is implicit — the Order output IS a [[cyberlink]], which updates state. the program never "writes" — it returns a result, and the kernel records it.

```
read:   look(key) → value          deterministic, Brakedown opening proof
write:  return result → cyberlink   implicit, kernel handles persistence
effect: none                        the program is a pure function
state:  all of BBG                  available via look()
```

no effects. all features. provable by construction.

## nouns

everything is a noun: atom(v) or cell(noun, noun). binary tree with [[Goldilocks field]] elements at leaves. code and data are the same structure. every noun has identity via [[hemera]] structural hash — computing a noun IS creating its CID. see [[nox/nouns]].

## 18 instructions

frozen forever.

| group | # | patterns | what | why |
|-------|---|----------|------|-----|
| structural | 0-4 | axis, quote, compose, cons, branch | Turing completeness | data access, recursion, conditionals |
| field | 5-10 | add, sub, mul, inv, eq, lt | F_p arithmetic | crypto, proofs, consensus |
| bitwise | 11-14 | xor, and, not, shl | Boolean / Z/2^W | hashing, encryption |
| hash | 15 | hemera(x) | content addressing | particle identity, Merkle trees |
| hint | 16 | prover injects witness | non-determinism | privacy (ZK), oracle access |
| look | 17 | read from [[bbg]] | state access | pure functions with full state |

16 deterministic compute patterns + hint (non-deterministic) + look (state read). fewer → incomplete. more → wider tag, larger verifier. verifier circuit ~70K constraints.

## computation = linking

```
ask(ν, p, q, τ, a, v) → Answer    -- t (block height) comes from the containing signal
```

the arguments ARE the [[cyberlink]] 6-tuple. evaluating IS creating an edge. before reducing, check [[cybergraph]]: if axon(H(formula), H(subject)) exists, return cached result. the graph IS the memo table. the more the network computes, the faster it gets.

## trace = proof

every reduce() produces rows in the execution trace. this trace IS the [[zheng]] witness — no separate proof generation. [[SuperSpartan]] checks constraints via [[sumcheck]]. [[Brakedown]] commits with O(N) field ops. [[HyperNova]] folds rows during execution. the boundary between execution and proving dissolves.

## five execution regimes

nox is parameterised: nox<F, W, H>. same 16 patterns, different algebras. five [[lens]] backends commit to traces over different fields:

| regime | algebra | what it serves | lens |
|--------|---------|---------------|------|
| scalar | [[nebu]] (F_p) | arithmetic, crypto, consensus | Brakedown |
| binary | [[kuro]] (F₂) | quantized AI, bitwise, 32× cheaper bits | Binius |
| tropical | [[trop]] (min,+) | optimization, shortest paths, decisions | encoded via F_p |
| privacy | [[genies]] (F_q) | stealth, key exchange, post-quantum | dedicated PCS |
| encrypted | [[jali]] (R_q) | TFHE, encrypted computation | Ikat |

all fold into ONE [[HyperNova]] accumulator. five algebras, one proof.

## jets

compositions of Layer 1 patterns recognized by formula hash, replaced with optimized implementations. semantics unchanged — jets are acceleration, not extension.

```
jet_registry: H(formula_noun) → optimized_impl
```

the registry lives in [[cybergraph]] as cyberlinks: formula particle → implementation particle. nox without jets works. just slower. see [[cyb/mind]] for how jets map to the four-tier cognitive architecture.

## 15 languages compile to nox

domain-specific constraint encodings. one VM, many frontends:

| language | domain | example |
|----------|--------|---------|
| [[Tri]] | general purpose | semcons, progs, kernel |
| [[Tok]] | tokenomics | plumb conservation, staking |
| [[Arc]] | state machines | BBG transitions, consensus |
| [[Seq]] | sequences | time series, signal processing |
| [[Inf]] | inference | neural network forward pass |
| [[Bel]] | belief | Bayesian update, probabilistic |
| [[Ren]] | rendering | UI layout, visualization |
| [[Dif]] | differentiation | gradients, optimization |
| [[Sym]] | symbolic | algebra, equation solving |
| [[Wav]] | wavelets | signal analysis, compression |
| [[Bt]] | binary | kuro quantized models |
| [[Rs]] | systems | low-level, hardware interaction |
| [[Ten]] | tensors | matrix ops, ML training |
| Nox | raw | hand-written noun formulas |

all compiled by [[Trident]]. all proven by [[zheng]]. all stored in [[bbg]].

## self-verification

the [[zheng]] verifier is itself a nox program. proof-of-proof = nox program running verifier on proof. recursive to arbitrary depth, constant proof size. the system closes on itself.

## lineage

```
combinatory logic (Schönfinkel 1924)
  ↓
lambda calculus (Church 1936)
  ↓
Nock (Yarvin 2008) — 12 rules, natural numbers, no field
  ↓
nox — 16 patterns, field elements, hash, hint, proof-native
```

cost: 4 more patterns (16 vs 12). gain: proof-native execution, five algebra regimes, content-addressed identity, privacy boundary.

see [[nox]] for specs. see [[Trident]] for the compiler. see [[zheng]] for proofs. see [[cybergraph]] for the knowledge graph. see [[cyb/soma]] for the machine mind that runs nox programs. see [[cyb/order]] for the execution unit.
