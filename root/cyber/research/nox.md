---
tags: cyber, research, article
crystal-type: process
crystal-domain: cyber
status: draft
date: 2026-03-24
---
# nox: computation as linking

## abstract

[[nox]] is a minimal VM where computation IS [[cyberlinks|linking]]. the arguments of ask() — subject, formula, object — are the fields of a [[cyberlink]]. evaluating a program IS creating an edge in the [[cybergraph]]. the execution trace IS the [[zheng]] proof witness. the structural hash of the result IS the [[particles|particle]] identity. no boundary between "compute" and "record" — they are the same operation.

nox has 16 deterministic patterns (4 structural, 6 field, 4 bitwise, 1 hash, 1 hint) that fit in 4 bits. the patterns form an orthogonal rewrite system — the result is the same regardless of evaluation order (confluence). the VM is parameterised by algebra: nox<F, W, H> where F is the field, W is the word width, H is the hash function. the canonical instantiation is nox<[[Goldilocks field|Goldilocks]], Z/2³², [[Hemera]]>. the same 16 patterns work over any field — enabling a binary instantiation nox<F₂> for quantised inference and a Goldilocks instantiation for arithmetic, sharing one proof system ([[zheng]]).

## 1. nouns: the universal data type

everything in nox is a noun:

$$\text{noun} = \text{atom}(v) \mid \text{cell}(\text{noun}, \text{noun})$$

an atom holds one [[Goldilocks field|field element]] (64 bits). a cell holds two nouns. this is the simplest possible recursive data type — a binary tree with field elements at the leaves.

three type tags distinguish interpretations:

| tag | name | domain | example |
|---|---|---|---|
| 0x00 | field | $\mathbb{F}_p$ (full Goldilocks) | 42, p-1, 0 |
| 0x01 | word | $[0, 2^{32})$ | 255, 1000, 0 |
| 0x02 | hash | 4 × $\mathbb{F}_p$ (32 bytes) | H("hello"), particle CID |

code and data are the same structure. a formula is a noun. the subject it operates on is a noun. the result is a noun. homoiconicity: programs can inspect, construct, and transform other programs as data.

### identity

every noun has a unique identity via [[Hemera]] structural hash:

$$H(\text{atom}(v)) = \text{hemera\_leaf}(v, \text{type\_tag})$$
$$H(\text{cell}(a, b)) = \text{hemera\_node}(H(a), H(b))$$

this hash IS the [[particles|particle]] identity in the [[cybergraph]]. computing a noun's hash IS creating its CID. the hash is computed incrementally during execution — cons(a, b) computes H(cell(a, b)) as a side effect.

### axis addressing

nouns are binary trees. axis addressing navigates them:

```
axis 1 = identity (the whole tree)
axis 2 = left child
axis 3 = right child
axis 4 = left of left
axis 5 = right of left
axis 6 = left of right
axis 7 = right of right
...
axis n: binary representation of n gives the path (1=start, 0=left, 1=right)
```

axis traversal produces authentication siblings as a side effect — the path from leaf to root is a Merkle proof. the VM builds authenticated trees by construction.

## 2. the 16 patterns

every nox computation is a sequence of reduce(subject, formula) calls. the formula's head is a pattern tag (0-15). the pattern determines what happens.

### structural (0-4): algebra-independent

| # | name | what it does | cost |
|---|---|---|---|
| 0 | axis | navigate the subject tree by address | depth $d$ |
| 1 | quote | return literal, unevaluated | 1 |
| 2 | compose | sequential: evaluate two formulas, apply result of first to result of second | 1 + costs |
| 3 | cons | parallel: evaluate two formulas, pair the results | 1 + costs |
| 4 | branch | conditional: evaluate test, then one of two branches (lazy) | 1 + costs |

these five patterns make nox Turing-complete. compose provides recursion (apply a computed formula to a computed subject). branch provides conditional execution. axis provides data access. cons provides data construction. quote provides constants.

### field arithmetic (5-10): parameterised by F

| # | name | operation | cost |
|---|---|---|---|
| 5 | add | $a + b \mod p$ | 1 |
| 6 | sub | $a - b \mod p$ | 1 |
| 7 | mul | $a \times b \mod p$ | 1 |
| 8 | inv | $a^{-1} \mod p$ (Fermat: $a^{p-2}$) | 64 |
| 9 | eq | $a = b$ ? 0 : 1 | 1 |
| 10 | lt | $a < b$ ? 0 : 1 | 1 |

six operations: a commutative ring (add, sub, mul) completed to a field (inv) with comparisons (eq, lt). this is the minimum for field arithmetic — no redundancy.

inv costs 64 because it requires square-and-multiply ($\log_2(p-2) = 63$ multiplications). but it costs only 1 constraint to VERIFY — the proof checks $a \times a^{-1} = 1$. this asymmetry (expensive to compute, cheap to prove) is fundamental to [[zheng]]'s efficiency.

### bitwise (11-14): parameterised by W

| # | name | operation | cost |
|---|---|---|---|
| 11 | xor | $a \oplus b$ | 1 |
| 12 | and | $a \wedge b$ | 1 |
| 13 | not | $\neg a$ | 1 |
| 14 | shl | $a \ll n$ | 1 |

four operations over W-bit words ($W = 32$ in canonical instantiation). xor + and + not is functionally complete for Boolean logic. shl provides positional manipulation.

in the Goldilocks instantiation, each bitwise operation costs ~32 [[STARK]] constraints (bit decomposition). in the binary instantiation nox<F₂>, they cost 1 constraint each. this 32× gap motivates [[Binius]]: heavy binary workloads run on nox<F₂> for 32-64× fewer constraints.

### hash + hint (15-16)

| # | name | what it does | cost |
|---|---|---|---|
| 15 | hash | $\text{Hemera}(x)$ — cryptographic hash | 300 |
| 16 | hint | prover injects witness (non-deterministic) | 1 |

hash is the bridge between computation and identity. hashing a noun produces its [[particles|particle]] CID. every nox program that creates content necessarily calls hash to produce the identity.

hint is the privacy boundary. the prover knows a secret (e.g., a private key) and injects it. the verifier sees only that the subsequent computation verified correctly — not what was injected. this is how anonymous [[cyberlinks]] work: the neuron proves it has sufficient [[stake]] without revealing which neuron it is.

## 3. computation IS linking

### ask() = cyberlink

the top-level interface:

$$\text{ask}(\nu, \text{Object}, \text{Formula}, \tau, a, v, t) \to \text{Answer}$$

these arguments ARE the [[cyberlink]] 7-tuple:

| ask() argument | cyberlink field | meaning |
|---|---|---|
| $\nu$ | neuron | who is asking |
| Object | from (source particle) | what is the subject |
| Formula | to (target particle) | what operation to apply |
| $\tau$ | token | which denomination pays |
| $a$ | amount | how much focus to spend |
| $v$ | valence | epistemic stance (-1/0/+1) |
| $t$ | time | when |

evaluating a nox program IS creating a cyberlink. the result is a new particle $H(\text{result})$. the computation trace is the [[zheng]] proof. everything happens in one operation.

### memoisation = graph lookup

before reducing, ask() checks the [[cybergraph]]:

$$\text{axon}(H(\text{Formula}), H(\text{Object})) \stackrel{?}{\in} \text{cybergraph}$$

if the axon exists, the result is already known — return it without recomputation. the cybergraph IS the memo table. every past computation is cached as an edge. the more the network computes, the faster future computations become.

this is not optional caching — it is the fundamental architecture. the cybergraph grows denser as computation proceeds. [[focus]] economics determine which results persist (frequently-used results attract reinforcement) and which decay (unused results are pruned by [[temporal decay]]).

### trace IS proof witness

every reduce() call produces one or more rows in the execution trace:

```
row: [tag, H(obj), H(frm), op_a, op_b, result, focus, types..., prev_hash, status]
      r0    r1      r2      r3    r4    r5      r6     r7-r13   r14         r15
```

this trace IS the [[zheng]] proof witness. no separate "proof generation" step:
- the trace is the multilinear polynomial's evaluation table
- [[SuperSpartan]] checks constraints over the trace via [[sumcheck]]
- [[Brakedown]] commits to the trace with O(N) field operations
- [[HyperNova]] folds trace rows during execution ([[proof-carrying computation|proof-carrying]])

the boundary between "execution" and "proving" dissolves. computation produces the proof as a byproduct.

## 4. algebra polymorphism

nox is parameterised: nox<F, W, H>. the same 16 patterns, the same reduction rules, different algebras:

| instantiation | F | W | H | what it serves |
|---|---|---|---|---|
| nox<Goldilocks> | $\mathbb{F}_p$ ($p = 2^{64} - 2^{32} + 1$) | $\mathbb{Z}/2^{32}$ | [[Hemera]] | arithmetic, crypto, consensus |
| nox<F₂> | $\mathbb{F}_2$ tower | $\mathbb{Z}/2$ | external | binary: quantised AI, bitwise crypto |
| nox<F_{p²}> | $\mathbb{F}_{p^2}$ | $\mathbb{Z}/2^{32}$ | Hemera | extension field operations |

the proof system ([[zheng]]) is also polymorphic:
- nox<Goldilocks> → [[Brakedown]] PCS (Goldilocks-native)
- nox<F₂> → [[Binius]] PCS (binary-native)
- both fold into ONE [[HyperNova]] accumulator via universal CCS

14 source languages compile to nox with domain-specific constraint encodings:

```
Goldilocks: Nox, Tri, Tok, Arc, Seq, Inf, Bel, Ren, Dif, Sym, Wav
Binary:     Bt
Split:      Rs, Ten (compiler decides based on workload)
```

## 5. jets: verified acceleration

jets are compositions of Layer 1 patterns recognised by formula hash and replaced with optimised implementations. every jet has an equivalent pure nox program — jets are OPTIMISATION, semantics unchanged.

```
jet_registry: H(formula_noun) → optimised_implementation
```

the registry is a protocol constant. recognition is by structural hash — if the formula matches a known jet, dispatch the fast path. otherwise, reduce via Layer 1 patterns.

verifier jets (recursive composition):

| jet | what it does | pure cost | jet cost | speedup |
|---|---|---|---|---|
| hash | Hemera permutation | ~2,800 | 300 | 9× |
| poly_eval | Horner evaluation | ~2N | N | 2× |
| merkle_verify | Merkle path check | d×310 | d×300 | ~1× |
| fri_fold | polynomial folding round | ~N | N/2 | 2× |
| ntt | Number Theoretic Transform | ~2N log N | N log N | 2× |

binary jets (F₂ workloads):

| jet | what it does | speedup | primary workload |
|---|---|---|---|
| popcount | count set bits | 5× (constraints), 90× (prover SIMD) | all binary accumulation |
| binary_matvec | batched binary matmul | 1,400× over F_p | quantised inference, [[tri-kernel]] SpMV |
| quantize/dequantize | F_p ↔ F₂ boundary | k× per crossing | algebra transitions |
| activation_lut | lookup table for nonlinearities | 2^k/k× per lookup | inference activation functions |

## 6. why these 16 patterns

### sufficiency

five groups cover all algebraic domains [[cyber]] needs:

| group | patterns | domain | why needed |
|---|---|---|---|
| structural | axis, quote, compose, cons, branch | Turing completeness | program construction, data access, recursion, conditionals |
| field | add, sub, mul, inv, eq, lt | $\mathbb{F}_p$ arithmetic | crypto, proofs, consensus, financial |
| bitwise | xor, and, not, shl | Boolean / $\mathbb{Z}/2^W$ | hashing, encryption, binary protocols |
| hash | hash | content addressing | particle identity, Merkle trees, commitments |
| hint | hint | non-determinism | privacy (ZK), witness injection, oracle access |

### minimality

16 patterns = 4 bits. this is maximally dense encoding for a self-proving VM:
- fewer patterns → Turing-incomplete or missing critical domains
- more patterns → wider tag field, more constraint types, larger verifier circuit
- 16 is the sweet spot: complete, dense, and the verifier circuit stays at ~70K constraints (with jets)

### nothing else needed

no floating point (field arithmetic is exact). no strings (nouns represent structured data). no I/O (signals are the I/O mechanism). no exceptions (errors are status codes in r15). no concurrency (parallelism is within compose/cons, not between programs). every "missing" feature is either unnecessary or handled at a different layer.

## 7. self-verification

the [[zheng]] verifier for nox is itself a nox program:

```
verify(proof) requires:
  field arithmetic (patterns 5, 7, 8)     ← native
  hash computation (pattern 15)           ← native
  sumcheck verification (patterns 5, 7, 9) ← field ops only
  Brakedown opening check (patterns 5, 7)  ← field ops only

all are nox-native. the verifier IS a nox program.
```

consequence: a proof-of-proof is a nox program that runs the verifier on a proof. the proof-of-proof is itself provable. recursion to arbitrary depth, constant proof size at every level.

```
program → trace → proof → verifier (nox) → trace → proof → ...
```

the system closes on itself. this is what makes the [[universal accumulator]] possible — every proof type, including proofs about proofs, reduces to nox execution.

## 8. the lineage

```
combinatory logic (Schönfinkel 1924, Curry 1930)
  ↓ applied to programming
lambda calculus (Church 1936)
  ↓ minimised
Nock (Yarvin 2008) — 12 rules, natural number arithmetic, no field
  ↓ upgraded
nox — 16 patterns, field elements, inverse, hash, hint, proof-native
```

nox replaces Nock's natural number increment with field arithmetic and adds:
- inv (field inversion — closes the field)
- hash (cryptographic identity — makes nouns content-addressable)
- hint (non-determinism — enables privacy)
- bitwise operations (practical cryptography)

the cost: 4 more patterns (16 vs 12). the gain: proof-native execution, algebra polymorphism, content-addressed identity, privacy boundary.

see [[zheng]] for the proof system, [[Hemera]] for the hash function, [[BBG]] for the authenticated state, [[cybergraph]] for the knowledge graph, [[structural-sync]] for the sync protocol, [[proof-carrying computation|proof-carrying]] for zero-latency proving
