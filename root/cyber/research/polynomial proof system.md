---
tags: cyber, research, article, core
crystal-type: article
crystal-domain: cyber
date: 2026-03-25
---
# polynomial proof system

a proof system where the polynomial is the universal primitive. commit to data, prove computation, identify content, sample availability, fold composition — one operation on one object. no hash trees. no Merkle paths. no separate identity scheme. the polynomial IS the proof, the state, the identity, and the erasure code.

## definition

a polynomial proof system is a transparent argument of knowledge where:

1. **the witness is a multilinear polynomial.** the execution trace, the state, the content — all are multilinear polynomials over the Boolean hypercube $\{0,1\}^k$
2. **the commitment is a linear-code encoding.** no hash tree. the prover encodes the polynomial via an expander graph. binding from one hash call. opening from recursive tensor decomposition
3. **the constraint check is a sumcheck.** the verifier reduces an exponential sum to one evaluation. the prover's table halves each round. total prover work: O(N)
4. **composition is folding.** multiple proof instances fold into one accumulator with ~30 field operations. verification happens once at the end
5. **identity is the commitment.** the PCS commitment to content IS the content's identity (CID). accessing content IS opening the commitment. proving IS committing. one primitive

properties: transparent (no setup), post-quantum (code-based), linear-time prover, logarithmic proof size, Merkle-free, algebraically composable.

## the five operations

one PCS. five uses.

**commit.** encode the polynomial via expander graph. O(N) field operations. one hash call for binding. 32 bytes.

```
C = Brakedown.commit(f)
  = hemera(expander_encode(eval_table(f)))
  cost: O(d × N) field multiplications, d ≈ 6-10
```

**open.** prove the polynomial evaluates to v at point r. recursive: commit the √N opening vector, recurse. log log N levels. O(log N + λ) proof size.

```
proof = Brakedown.open(f, r)
  recursive: commit opening vector → open that → ... → send O(λ) directly
  proof size: ~1.3 KiB at N = 2²⁰
```

**verify.** check the opening proof. O(λ log log N) field operations. ~5 μs. no hashing.

```
accept = Brakedown.verify(C, r, v, proof)
  cost: ~660 field operations at N = 2²⁰
```

**fold.** combine two proof instances into one accumulator. ~30 field operations. the accumulator stores the combined claim. verification deferred to one final check.

```
acc' = HyperNova.fold(acc, instance)
  cost: ~30 field multiplications + 1 hash
  size: ~200 bytes (constant)
```

**identify.** the commitment IS the content identity. wrapped with domain separation for collision resistance across contexts.

```
CID = hemera(C ‖ domain_tag)
  32 bytes. universal. supports O(1) opening at any position.
  the same CID that identifies the content also proves any claim about it.
```

## the architecture

```
f: {0,1}^k → F_p                     the data (any data: trace, state, content)
    ↓ commit
C = Brakedown.commit(f)               32 bytes (the identity AND the commitment)
    ↓ constrain
SuperSpartan.check(C, CCS)            sumcheck reduces to one evaluation
    ↓ open
Brakedown.open(f, r) → proof          ~1.3 KiB (recursive, Merkle-free)
    ↓ fold
HyperNova.fold(acc, proof)            ~30 field ops (defer verification)
    ↓ decide
SuperSpartan.decide(acc) → final      one check, ~8K constraints, ~5 μs
```

## why multilinear

univariate polynomials (degree N, one variable) require FFT for evaluation: O(N log N). the prover cannot be linear.

multilinear polynomials (degree 1 per variable, k variables where $N = 2^k$) are evaluated by the sumcheck protocol. each round fixes one variable and halves the domain:

$$2^k + 2^{k-1} + 2^{k-2} + \ldots + 1 = 2^{k+1} - 1 = O(N)$$

the prover IS linear. no FFT. no NTT. a shrinking table is the only data structure.

multilinear polynomials over $\{0,1\}^k$ are isomorphic to binary trees with $2^k$ leaves. a tree IS a polynomial. axis (tree navigation) IS polynomial evaluation. cons (tree construction) IS variable prepend. the data structure and the proof structure are the same mathematical object.

## why linear codes

Merkle trees commit to evaluations by hashing: O(N log N) hash calls. opening requires O(log N) authentication path hashes. 77% of a FRI-based proof is Merkle paths.

expander-graph linear codes commit by sparse matrix-vector multiplication: O(N) field operations. opening by recursive tensor decomposition: O(log N + λ) field elements. zero hash calls in the proof. the bottleneck shifts from hashing to field arithmetic.

| | Merkle (FRI/WHIR) | linear code (Brakedown) |
|---|---|---|
| commit | O(N log N) hash | O(N) field ops |
| open | O(log² N) hash | O(log N + λ) field ops |
| proof content | hash paths (77%) | field elements (100%) |
| verify | hash + field | field only |
| prover bottleneck | hash function | field arithmetic |
| hardware | hash accelerator | multiply-accumulate |

the proof contains zero hashes. verification is pure field arithmetic. on a Goldilocks field processor: multiply-accumulate at clock speed. no hash pipeline stall.

## why folding

recursive proof composition (proof-of-proof) requires verifying a proof INSIDE a circuit: ~50K-200K constraints per level. N composition steps = N × verifier_cost.

folding replaces verification with accumulation. two CCS instances combine into one with ~30 field operations. the combined instance is satisfiable iff both originals are. verification happens ONCE at the end.

```
recursive verify:   N levels × ~8K constraints = ~8NK total
folding:            N folds × ~30 field ops + 1 decider (~8K) = ~30N + 8K total

at N = 1000:  recursive = ~8M constraints.  folding = ~38K.  210× cheaper.
at N = 10⁶:   recursive = ~8B constraints.  folding = ~30M.  267× cheaper.
```

folding enables proof-carrying computation: each VM step folds one trace row into the accumulator during execution. the proof is ready when the program finishes. zero additional proving latency.

## the polynomial IS the identity

in hash-based systems, content identity (hash) and proof commitment (PCS) are separate primitives. a file's CID is SHA-256 of its bytes. a proof's commitment is FRI over its trace. two different operations. two different security analyses.

in a polynomial proof system, they merge:

```
content → multilinear polynomial → Brakedown.commit → 32 bytes

this 32 bytes IS:
  the content identity (CID)
  the proof commitment (for any claim about the content)
  the DAS commitment (for availability sampling)
  the state binding (for authenticated queries)
```

accessing byte range [a,b] of a particle = opening the particle's polynomial at positions [a,b]. the proof is ~75 bytes per position. no download of the full content. no separate verification step.

this unification means: every content-addressed object in the system — every particle, every formula, every signal — is simultaneously identifiable, provable, and sampleable through one operation.

## DAS is native

a multilinear polynomial over $\{0,1\}^k$ evaluates naturally on the larger domain $\mathbb{F}_p^k$. the evaluations beyond the Boolean hypercube ARE redundant information — the polynomial is determined by its $2^k$ values on $\{0,1\}^k$.

reshape as $\sqrt{N} \times \sqrt{N}$ bivariate polynomial. the extension to $2\sqrt{N} \times 2\sqrt{N}$ is standard 2D Reed-Solomon. any $\sqrt{N} \times \sqrt{N}$ submatrix reconstructs the original.

DAS sampling = PCS opening at random positions. each sample: ~75 bytes. 20 samples for 99.9999% confidence: ~1.5 KiB. no separate erasure coding pipeline. no separate commitment scheme. the content polynomial IS the erasure code.

## the numbers

for N = 2²⁰ (typical execution trace or large particle):

```
commit:          O(N) field ops, ~40 ms single core
proof size:      ~1.3 KiB (PCS) + ~0.5 KiB (sumcheck) + ~0.3 KiB (eval) = ~2 KiB
verify:          ~660 field ops, ~5 μs
fold:            ~30 field ops, ~0.2 μs
prover memory:   O(√N) via tensor compression
decider:         ~8K constraints, ~5 μs
```

composition at scale:

```
1000-transaction block:    1000 folds + 1 decider = 30K field ops + 8K constraints
1000-block epoch:          1000 folds + 1 decider = 30K + 8K
1M-block chain history:    1 accumulator = ~200 bytes, verify ~5 μs
```

## what this makes possible

**self-proving computation.** every VM step carries its proof. no separate proving phase. no prover infrastructure. the computation IS the proof.

**O(1) content access.** any byte range of any particle verified by one PCS opening. no download of full content. a phone verifies a 1 GB model's layer 47 weights with a 75-byte proof.

**240-byte chain checkpoint.** the universal accumulator (BBG_root + folding accumulator + height) proves ALL history. join the network: download 240 bytes, verify in 5 μs. full confidence from genesis.

**native data availability.** no separate erasure coding. the polynomial IS the code. DAS = PCS opening at random positions. 20 samples, ~1.5 KiB, 99.9999% confidence.

**programmable authenticated state.** deploy new tables by writing a nox program. standard operations (INSERT, UPDATE, TRANSFER) get automatic CCS jet optimization: 3-5 constraints per operation. no protocol upgrade.

**provable consensus.** the tri-kernel computation (1.42B constraints) fits at 33% of polynomial proof capacity. validators prove they computed π* correctly. consensus = computation + proof.

## the complete stack

```
one field:      Goldilocks (p = 2⁶⁴ - 2³² + 1)
one hash:       hemera (~3 calls per execution, trust anchor)
one PCS:        recursive Brakedown (everything: proof, state, identity, DAS)
one VM:         nox (16 patterns, polynomial nouns)
one state:      BBG_poly(10 dims) + A(x) + N(x), all PCS-committed
one sync:       structural sync (CRDT + PCS + DAS native)
one identity:   hemera(PCS.commit(content) ‖ tag) — 32 bytes
```

seven components. every pair shares at least one primitive. the system is algebraically closed: proofs about proofs, commitments to commitments, identities of identities — all reduce to polynomial evaluation over one field.

see [[nox]] for the VM, [[hemera]] for the hash, [[zheng]] for the proof system, [[BBG]] for polynomial state, [[structural-sync]] for sync, [[recursive brakedown]] for the PCS, [[polynomial nouns]] for the data model
