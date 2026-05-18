---
tags: cyber, research, article, core
crystal-type: article
crystal-domain: cyber
date: 2026-03-25
---
# polynomial proof system

a proof system where the polynomial is the universal primitive. commit to data, prove computation, identify content, sample availability, fold composition — one operation on one object. no [[hash]] trees. no Merkle paths. no separate identity scheme. the polynomial IS the [[proof]], the [[state model|state]], the identity, and the [[erasure coding|erasure code]].

## definition

a polynomial proof system is a transparent argument of knowledge where:

1. **the witness is a multilinear polynomial.** the [[nox]] execution trace, the [[BBG|state]], the [[particles|content]] — all are multilinear polynomials over the Boolean hypercube $\{0,1\}^k$
2. **the commitment is a linear-code encoding.** no hash tree. the prover encodes the polynomial via an expander graph ([[Brakedown]]). binding from one [[hemera]] call. opening from [[recursive brakedown|recursive tensor decomposition]]
3. **the constraint check is a [[sumcheck]].** the verifier reduces an exponential sum to one evaluation. the prover's table halves each round. total prover work: O(N)
4. **composition is [[folding]].** multiple proof instances fold into one [[HyperNova]] accumulator with ~30 field operations. verification happens once at the end
5. **identity is the commitment.** the Lens commitment to content IS the content's identity ([[particles|CID]]). accessing content IS opening the commitment. proving IS committing. one primitive

properties: transparent (no setup), post-quantum (code-based), linear-time prover, logarithmic proof size, Merkle-free, algebraically composable.

## the five operations

one Lens. five uses.

**commit.** encode the polynomial via expander graph. O(N) [[Goldilocks field|field]] operations. one [[hemera]] call for binding. 32 bytes.

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

multilinear polynomials over $\{0,1\}^k$ are isomorphic to binary trees with $2^k$ leaves. a tree IS a polynomial. axis (tree navigation) IS polynomial evaluation. cons (tree construction) IS variable prepend. the data structure and the proof structure are the same mathematical object. see [[polynomial nouns]] for the full theory.

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

the proof contains zero hashes. verification is pure [[Goldilocks field|field]] arithmetic. on a [[Goldilocks field processor]]: multiply-accumulate at clock speed. no hash pipeline stall.

## why folding

recursive proof composition (proof-of-proof) requires verifying a proof INSIDE a circuit: ~50K-200K constraints per level. N composition steps = N × verifier_cost.

folding replaces verification with accumulation. two CCS instances combine into one with ~30 field operations. the combined instance is satisfiable iff both originals are. verification happens ONCE at the end.

```
recursive verify:   N levels × ~8K constraints = ~8NK total
folding:            N folds × ~30 field ops + 1 decider (~8K) = ~30N + 8K total

at N = 1000:  recursive = ~8M constraints.  folding = ~38K.  210× cheaper.
at N = 10⁶:   recursive = ~8B constraints.  folding = ~30M.  267× cheaper.
```

folding enables [[proof-carrying computation|proof-carrying]]: each [[nox]] VM step folds one trace row into the accumulator during execution. the proof is ready when the program finishes. zero additional proving latency.

## the polynomial IS the identity

in hash-based systems, content identity ([[hash]]) and proof commitment (lens) are separate primitives. a file's CID is SHA-256 of its bytes. a proof's commitment is [[FRI]] over its trace. two different operations. two different security analyses.

in a polynomial proof system, they merge:

```
content → multilinear polynomial → Brakedown.commit → 32 bytes

this 32 bytes IS:
  the content identity (CID)
  the proof commitment (for any claim about the content)
  the DAS commitment (for availability sampling)
  the state binding (for authenticated queries)
```

accessing byte range [a,b] of a [[particles|particle]] = opening the particle's polynomial at positions [a,b]. the proof is ~75 bytes per position. no download of the full content. no separate verification step.

this unification means: every content-addressed object in the system — every [[particles|particle]], every formula, every [[signal]] — is simultaneously identifiable, provable, and sampleable through one operation.

## DAS is native

a multilinear polynomial over $\{0,1\}^k$ evaluates naturally on the larger domain $\mathbb{F}_p^k$. the evaluations beyond the Boolean hypercube ARE redundant information — the polynomial is determined by its $2^k$ values on $\{0,1\}^k$.

reshape as $\sqrt{N} \times \sqrt{N}$ bivariate polynomial. the extension to $2\sqrt{N} \times 2\sqrt{N}$ is standard 2D Reed-Solomon. any $\sqrt{N} \times \sqrt{N}$ submatrix reconstructs the original.

[[DAS]] sampling = Lens opening at random positions. each sample: ~75 bytes. 20 samples for 99.9999% confidence: ~1.5 KiB. no separate [[erasure coding]] pipeline. no separate commitment scheme. the content polynomial IS the erasure code.

## the numbers

for N = 2²⁰ (typical execution trace or large particle):

```
commit:          O(N) field ops, ~40 ms single core
proof size:      ~1.3 KiB (lens) + ~0.5 KiB (sumcheck) + ~0.3 KiB (eval) = ~2 KiB
verify:          ~660 field ops, ~5 μs
fold:            ~30 field ops, ~0.2 μs
prover memory:   O(√N) via tensor compression
decider:         ~89 constraints, ~0.1 μs (with decider jet)
```

composition at scale:

```
1000-transaction block:    1000 folds + 1 decider = 30K + 89 field ops
1000-block epoch:          1000 folds + 1 decider = 30K + 89
1M-block chain history:    1 accumulator = ~200 bytes, verify ~0.1 μs
```

## the decider jet

the decider — the ONE verification that runs at the end of all folding — is a fixed computation: [[sumcheck]] replay + [[Brakedown]] spot-checks + CCS evaluation. three optimizations reduce it from ~8K constraints to ~89:

**CCS jet for the decider.** the decider always does the same thing. express it as a direct CCS encoding instead of a [[nox]] trace:

```
sumcheck replay:    k rounds of g_i(0) + g_i(1) = prev           20 ASSERT_EQ
CCS evaluation:     Σ c_j × ∏(M_i · z)(r) = 0                   34 MUL + ASSERT_EQ
Brakedown checks:   λ × log log N spot-checks                    1,280 MUL + ASSERT_EQ
hemera seed:        1 call                                        736 constraints
                                                                  ─────────
total with CCS jet:                                               ~2,070 constraints
```

**batch spot-checks.** the 1,280 Brakedown spot-checks are repetitive — same operation, different inputs. one batched [[sumcheck]] replaces 640 independent checks:

```
640 multiplications → 1 batched sumcheck
  log₂(640) = 10 rounds × 3 elements + 1 assertion = ~35 constraints
                                                      (was 1,280)
```

**algebraic Fiat-Shamir.** the [[Brakedown]] commitment already contains a [[hemera]] binding hash. derive challenges from the existing hash — zero additional hemera calls in the decider:

```
sumcheck:     20 constraints
CCS eval:     34 constraints
Brakedown:    35 constraints (batched)
hemera:        0 (challenges from existing Lens commitment)
              ────
total:        ~89 constraints
```

~89 constraints to verify ALL history from genesis. cheaper than one [[hemera]] permutation (736). the entire chain reduces to a ~200-byte accumulator verifiable in ~100 nanoseconds.

```
                    generic     + CCS jet    + batch      + algebraic FS
decider:            ~8,000      ~2,070       ~825         ~89
verify ALL history: ~5 μs       ~1.5 μs      ~0.6 μs      ~0.1 μs
cost vs hemera:     11× hash    3× hash      1× hash      0.12× hash
```

## what this makes possible

**self-proving computation.** every [[nox]] VM step carries its proof via [[proof-carrying computation|proof-carrying]]. no separate proving phase. no prover infrastructure. the computation IS the proof.

**O(1) content access.** any byte range of any [[particles|particle]] verified by one Lens opening. no download of full content. a phone verifies a 1 GB model's layer 47 weights with a 75-byte proof.

**240-byte chain checkpoint.** the [[universal accumulator]] ([[BBG]] root + [[HyperNova]] folding accumulator + height) proves ALL history. join the network: download 240 bytes, verify in ~0.1 μs (~89 constraints). cheaper than hashing 56 bytes. full confidence from genesis.

**native [[DAS|data availability]].** no separate [[erasure coding]]. the polynomial IS the code. DAS = Lens opening at random positions. 20 samples, ~1.5 KiB, 99.9999% confidence.

**[[programmable state|programmable authenticated state]].** deploy new tables by writing a [[nox]] program. standard operations (INSERT, UPDATE, TRANSFER) get automatic [[state jets|CCS jet]] optimization: 3-5 constraints per operation. no protocol upgrade.

**[[provable consensus]].** the [[tri-kernel]] computation (1.42B constraints) fits at 33% of polynomial proof capacity. validators prove they computed φ* correctly. [[consensus]] = computation + proof.

## the complete stack

```
one field:      Goldilocks (p = 2⁶⁴ - 2³² + 1)
one hash:       hemera (~3 calls per execution, trust anchor)
one Lens:        recursive Brakedown (everything: proof, state, identity, DAS)
one VM:         nox (18 patterns: 16 compute + call + look, polynomial nouns)
one state:      BBG_poly(10 dims) + A(x) + N(x), all lens-committed
one sync:       structural sync (CRDT + lens + DAS native)
one identity:   hemera(Lens.commit(content) ‖ tag) — 32 bytes
```

see also [[Goldilocks field]], [[hemera]], [[Brakedown]], [[nox]], [[BBG]], [[structural-sync]], [[particles]]

seven components. every pair shares at least one primitive. the system is algebraically closed: proofs about proofs, commitments to commitments, identities of identities — all reduce to polynomial evaluation over one field.

see [[nox]] for the VM, [[hemera]] for the hash, [[zheng]] for the proof system, [[BBG]] for polynomial state, [[structural-sync]] for sync, [[recursive brakedown]] for the Lens, [[polynomial nouns]] for the data model
