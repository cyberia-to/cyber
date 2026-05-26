---
tags: cyber, research, article
crystal-type: process
crystal-domain: cyber
status: draft
date: 2026-03-24
---
# polynomial nouns: all-algebraic computation and state

## the observation

binary trees and multilinear polynomials over the Boolean hypercube are the same mathematical object. a binary tree with N leaves is a function $\{0,1\}^k \to \mathbb{F}_p$ where $k = \log_2 N$. that function has a unique multilinear extension — a polynomial of degree 1 in each variable.

```
tree:         cell(cell(1, 2), cell(3, 4))

as function:  f(0,0) = 1    f(0,1) = 2    f(1,0) = 3    f(1,1) = 4

as polynomial: f(x₁, x₂) = 1·(1-x₁)(1-x₂) + 2·(1-x₁)x₂ + 3·x₁(1-x₂) + 4·x₁x₂
```

every [[nox]] noun is a binary tree of [[Goldilocks field]] elements. therefore every noun IS a multilinear polynomial. the tree structure is a REPRESENTATION. the polynomial is the same data, different encoding.

## what this changes

if nouns are polynomials, then noun operations become polynomial operations:

| nox operation | tree model | polynomial model |
|---|---|---|
| axis(noun, 2) — left child | follow left pointer, O(depth) | evaluate $f(0, \ldots)$, O(1) |
| axis(noun, 3) — right child | follow right pointer, O(depth) | evaluate $f(1, \ldots)$, O(1) |
| axis(noun, path) — deep access | traverse path, O(depth) | evaluate at binary point, O(1) |
| cons(a, b) — construction | allocate cell, set pointers, O(1) | $g(x_1, \ldots) = (1-x_1) \cdot a(x_2, \ldots) + x_1 \cdot b(x_2, \ldots)$, O(1) |
| H(noun) — identity | [[hemera]] recursive hash, O(N) | Lens.commit(polynomial), O(N) |
| verify axis | O(depth) hemera hashes | O(1) Lens opening, ~200 bytes |

the structural operations (axis, cons) are unchanged semantically. the COST changes: axis drops from O(depth) to O(1). identity drops from recursive hemera hashing to a single Lens commitment.

## the new digest

a particle CID becomes a Lens commitment instead of a hemera hash:

```
current:    CID = hemera(content)                           32 bytes
algebraic:  CID = hemera(Lens.commit(content) ‖ domain_tag)  32 bytes
```

same size. same content-addressing property. same collision resistance (Brakedown uses hemera internally for the binding hash). but the algebraic CID supports operations that hemera hashes cannot:

| capability | hemera CID | algebraic CID |
|---|---|---|
| content verification | rehash all content, compare | recommit all content, compare |
| partial access | **impossible** — must download everything | Lens.open(position) → ~200 byte proof |
| random access | **impossible** | evaluate at any point, O(1) |
| range proof | **impossible** | batch Lens opening over range |
| algebraic composition | **impossible** | add commitments, multiply by scalar |
| streaming verification | O(log N) × 32B per chunk (BAO) | ~200B per chunk (Lens opening), 5× smaller |

the Lens commitment is a strict superset of hemera hash for content addressing. everything hemera can do, the Lens can do — plus random access, range proofs, and algebraic composition.

## domain separation

hemera's structured capacity provides domain separation: different modes for different purposes (tree nodes vs leaves, commitment vs nullifier). Lens commitments don't have native domain separation.

the solution: hemera WRAPS the Lens commitment.

```
raw Lens commitment:  C = Brakedown.commit(polynomial)         32 bytes
domain-separated:    CID = hemera(C ‖ domain_tag)              32 bytes

domain_tag ∈ {PARTICLE, FORMULA, COMMITMENT, NULLIFIER, SIGNAL, ...}
```

one hemera call per noun, not per node. hemera provides the domain tag. lens provides the algebraic commitment. the CID has both properties: domain separation (from hemera) and algebraic openings (from lens, via the raw commitment C which is recoverable from context).

## BAO streaming, algebraic

BAO (BLAKE3 Authenticated Output) verifies content chunk-by-chunk during streaming download, providing per-chunk authentication paths.

polynomial nouns provide algebraic streaming:

```
BAO (hemera tree):
  per-chunk proof:   O(log N) × 32 bytes auth path ≈ 1 KiB
  order:             sequential (tree builds bottom-up)
  seek to chunk k:   must process chunks 0..k-1 first

algebraic streaming:
  per-chunk proof:   Lens.open(commitment, k) ≈ 200 bytes
  order:             any (evaluate at any point)
  seek to chunk k:   directly, O(1)
```

5× smaller per-chunk proofs AND random-access seeking. for a video: stream from any position, verify each chunk independently. for a neural network: verify any layer's weights without downloading the full model.

the commitment (32 bytes) must be known before streaming begins. this is trivially satisfied — the particle CID IS the commitment (or wraps it). you always know the CID before requesting content.

## implications for [[nox]]

### axis becomes O(1)

axis is the most common nox operation. every data access, every formula parse, every tree traversal uses axis. with polynomial nouns:

```
current:    axis(noun, path) = walk tree for |path| steps, O(depth)
            verify: O(depth) hemera hashes for Merkle authentication

algebraic:  axis(noun, path) = evaluate polynomial at binary(path), O(1)
            verify: one Lens opening, ~200 bytes, O(1) field ops
```

for a deep noun (depth 32, e.g., a particle index with $2^{32}$ entries):

```
current:    axis costs 32 tree hops × hemera = 32 × 736 = ~23,500 constraints
algebraic:  axis costs 1 Lens opening ≈ ~200 field operations
```

100× reduction in the most common operation.

### cons is unchanged

cons(a, b) creates a new noun. in tree model: allocate cell, set left=a, right=b. in polynomial model: prepend a variable.

```
cons(a, b) as polynomial:
  g(x₁, x₂, ..., xₖ₊₁) = (1-x₁) · a(x₂, ..., xₖ₊₁) + x₁ · b(x₂, ..., xₖ₊₁)
```

cost: O(1) — one variable prepend. the resulting polynomial is one degree higher in a new variable. no actual computation — the cons is a structural change to the polynomial's variable list.

the commitment: Lens.commit(g) can be computed from Lens.commit(a) and Lens.commit(b) if the Lens supports homomorphic operations. Brakedown's linear structure allows this:

```
commit(g) = (1-r₁) · commit(a) + r₁ · commit(b)    for challenge r₁
```

one field operation to combine two existing commitments.

### the 16 compute patterns stay

all 16 compute patterns operate on nouns. nouns become polynomials. but the PATTERNS don't change — their semantics are defined over the noun abstraction (atom | cell), not over the representation (tree vs polynomial).

```
pattern 0 (axis):     evaluate polynomial at binary point     (was: traverse tree)
pattern 1 (quote):    return polynomial unchanged              (was: return tree unchanged)
pattern 2 (compose):  compose polynomial operations            (unchanged)
pattern 3 (cons):     prepend variable to polynomial           (was: allocate cell)
pattern 4 (branch):   conditional on polynomial evaluation     (unchanged)
pattern 5-10:         field arithmetic on atom values           (unchanged)
pattern 11-14:        bitwise on word values                    (unchanged)
pattern 15 (hash):    hemera(Lens.commit ‖ domain_tag)           (was: hemera recursive)
pattern 16 (call):    prover injects witness                    (unchanged)
pattern 17 (look):    deterministic BBG polynomial read         (unchanged)
```

18 patterns total (16 compute + call + look). same semantics. different performance characteristics for axis and hash.

## implications for [[hemera]]

hemera's role collapses to three calls:

```
role                          hemera calls     was
─────                         ────────────     ───
1. Fiat-Shamir seed           1 per proof      1 per proof (unchanged)
2. domain separation          1 per noun       O(N/8) per noun (was recursive tree hash)
3. Brakedown binding hash     1 per lens commit inside Brakedown (internal, unchanged)

total: ~3 hemera calls per proof-carrying nox execution
was:   hundreds to thousands per execution
```

hemera goes from the dominant cost to negligible. the hash function becomes the trust anchor — the cryptographic root that makes Lens binding secure — not the hot-path computation.

all hemera optimizations (folded sponge, batched proving, algebraic Fiat-Shamir, constraint-free MDS, partial round collapse) become less critical for performance because there are so few hemera calls. they remain valuable for the remaining calls but the pressure is off.

## implications for [[BBG]]

### particles ARE polynomials

a particle's content is stored as a multilinear polynomial. the particle CID is hemera(Lens.commit(content_poly) ‖ PARTICLE). accessing any byte range of the particle is a Lens opening. no download needed for partial verification.

```
current:    verify particle content → download all, rehash → O(N) hemera
algebraic:  verify byte range [a,b] → Lens.open(CID, [a,b]) → ~200 bytes, O(1)
```

for a 1 GB neural network model stored as a particle: verify any 4 KB chunk (one layer's weights) with a 200-byte proof. no download of the other 999.996 MB.

### DAS becomes native

content availability is already about proving chunks exist. with polynomial nouns, every particle IS a polynomial with native openings. DAS sampling = opening the particle polynomial at random positions.

```
current DAS:    particle → erasure-code → commit chunks → sample → verify
algebraic DAS:  particle IS polynomial → sample = Lens.open(random_position)
                no separate erasure coding step — the polynomial IS the code
```

the Reed-Solomon erasure coding and the polynomial commitment MERGE. a multilinear polynomial over $\{0,1\}^k$ evaluated on a larger domain $\mathbb{F}_p^k$ IS Reed-Solomon encoded. the polynomial naturally extends beyond the Boolean hypercube, providing redundancy.

this means: DAS is FREE for polynomial nouns. no separate erasure coding. no separate commitment. the noun's Lens commitment IS the DAS commitment. sampling IS Lens opening. verification IS Lens.verify.

### BBG_poly and particle polynomials share a lens

BBG_poly (state polynomial) and particle content polynomials all use the same Brakedown lens. one commitment scheme for everything: state queries, content verification, DAS sampling, axis evaluation, proof generation.

```
current stack:
  hemera → noun identity (content addressing)
  hemera → tree hashing (NMT, Merkle)
  Brakedown → state commitment (BBG_poly)
  Brakedown → proof commitment (zheng)
  separate DAS scheme (2D Reed-Solomon + NMT)

algebraic stack:
  Brakedown → EVERYTHING (nouns, state, proofs, DAS)
  hemera → domain separation wrapper (3 calls per execution)
```

## implications for [[zheng]]

### traces are polynomial evaluations

the nox trace (16 registers × N rows) is already a multilinear polynomial that [[SuperSpartan]] operates on. with polynomial nouns, the VALUES in the trace registers are themselves polynomial commitments. the trace becomes a polynomial whose entries are polynomial commitments — a polynomial over polynomials.

this enables:
- trace compression via polynomial composition (represent nested polynomials compactly)
- axis verification inside the trace at O(1) cost (Lens opening, not hemera path)

### proof-carrying computation stays the same

each reduce() folds one trace row into the [[HyperNova]] accumulator (~30 field ops). polynomial nouns don't change this — the fold mechanism operates on CCS instances regardless of whether the witness values are tree hashes or Lens commitments.

### verifier circuit shrinks

the [[zheng]] verifier (a nox program) currently spends most constraints on hemera hash verification (Merkle paths for lens, Fiat-Shamir challenges). with polynomial nouns:

```
current verifier:    ~12K constraints (Brakedown)
                     dominated by Fiat-Shamir hemera calls

algebraic verifier:  ~5K constraints (estimated)
                     fewer hemera calls → fewer constraints
                     axis operations in verification are O(1) Lens openings
```

## implications for [[structural-sync|structural sync]]

### layer 3 (completeness) unifies with content model

completeness proofs = Lens openings. content access = Lens openings. they're the same operation. "prove this namespace is complete" and "access this particle's byte range" use the same primitive.

### layer 4 (availability) becomes native

DAS = lens sampling on polynomial nouns. no separate erasure coding infrastructure. the noun IS the erasure code (polynomial extends beyond Boolean hypercube naturally).

```
current:    5 verification layers, each with its own mechanism
algebraic:  5 verification layers, layers 3+4 merge into lens operations
```

## the cost: no threshold needed

the Lens overhead for small content is NEGLIGIBLE. Brakedown encoding is O(d×N) field ops where d is expander degree (~6-10). for small N, this is a handful of multiplications — drowned by the hemera wrap.

```
"cat" = 1 field element:
  lens path:    Brakedown.commit(1 element) ≈ 6 field ops + hemera wrap = ~736 constraints
  hemera path: hemera("cat") = 1 permutation = ~736 constraints
  IDENTICAL COST.

sentence = 10 field elements (80 bytes):
  lens path:    Brakedown.commit(10 elements) ≈ 60 field ops + hemera wrap = ~740 constraints
  hemera path: hemera(80 bytes) = 2 permutations (80/56 rate) = ~1,472 constraints
  lens IS CHEAPER. (hemera needs 2 absorption blocks; Brakedown encoding is trivial.)

document = 1000 field elements (8 KiB):
  lens path:    Brakedown.commit(1000) ≈ 6,000 field ops + hemera wrap = ~6,736 constraints
  hemera path: hemera(8 KiB) = ~143 permutations = ~105K constraints
  lens IS 15× CHEAPER.

neural net = 1M field elements (8 MB):
  lens path:    Brakedown.commit(1M) ≈ 6M field ops + hemera wrap = ~6M constraints
  hemera path: hemera(8 MB) = ~143K permutations = ~105M constraints
  lens IS 17× CHEAPER.
```

the crossover is at 56 bytes (one hemera rate block). below 56 bytes: identical cost. above: lens path wins because hemera needs multiple absorption blocks while Brakedown encoding scales as cheap field ops.

**decision: `hemera(Lens.commit(content) ‖ tag)` for EVERYTHING.** zero threshold. zero type dispatch. one identity scheme. one code path.

the shortest particles ("cat", "dog", "love") have the MOST links — they are the hottest nodes in the graph. making their identity lens-based means graph operations on the most-queried particles are algebraically composable with everything else. no boundary crossing. no dual paths.

## the unified primitive

the endgame: one cryptographic primitive (Brakedown lens) for all purposes.

```
noun identity:       Lens.commit(noun_polynomial)
state commitment:    Lens.commit(BBG_poly)
private records:     Lens.commit(A), Lens.commit(N)
proof commitment:    Lens.commit(trace_polynomial)
content access:      Lens.open(noun_commitment, position)
state query:         Lens.open(BBG_poly_commitment, (dim, key, t))
DAS sampling:        Lens.open(noun_commitment, random_position)
completeness proof:  Lens.open(commitment, namespace_range)

hemera's remaining role:
  domain_tag:        hemera(Lens.commit ‖ tag) — one call per identity
  Fiat-Shamir:       hemera seed — one call per proof
  Brakedown binding: hemera internally — one call per commit
```

one Lens. one field ([[Goldilocks field|Goldilocks]]). one hash ([[hemera]], now ~3 calls per execution). one proof system ([[zheng]]). one VM ([[nox]], 18 patterns: 16 compute + call + look). trees and polynomials are the same object. computation and state share one commitment scheme. proving and accessing share one opening protocol.

## open questions

1. **~~small noun overhead.~~** RESOLVED. Brakedown encoding of small content is negligible (~6 field ops for 1 element). the hemera wrap dominates either way (~736 constraints). use `hemera(Lens.commit(content) ‖ tag)` for everything. zero threshold. the shortest particles ("cat") have the most links — uniformity of identity scheme benefits the hottest nodes most.

2. **cons homomorphism.** can Brakedown.commit(cons(a,b)) be computed from commit(a) and commit(b) without re-encoding? Brakedown's linearity suggests yes: $\text{commit}(g) = (1-r) \cdot \text{commit}(a) + r \cdot \text{commit}(b)$ for a challenge $r$. needs formal verification.

3. **polynomial degree management.** cons increases variable count by 1. deep nesting → high-variable polynomial. Brakedown commitment cost grows with variable count. is there a practical limit on nesting depth? (current nox: focus limits depth implicitly.)

4. **migration.** all existing particle CIDs (hemera hashes) would change. the cybergraph needs either migration (rehash everything as Lens commitments) or dual identity support (hemera CID for legacy, lens CID for new). signal-first architecture helps: replay signals with new identity scheme → reconstruct graph with lens CIDs.

5. **Reed-Solomon from polynomial extension.** the claim that "polynomial extension beyond Boolean hypercube IS erasure coding" needs formal proof. specifically: does evaluating a $k$-variate multilinear polynomial on $\mathbb{F}_p^k$ (instead of $\{0,1\}^k$) provide the same reconstruction guarantees as 2D Reed-Solomon? the degree structure differs — multilinear has degree 1 per variable, not degree $k$ overall.

6. **SIMD and hardware.** hemera maps to the p2r (Poseidon2 round) GFP hardware primitive. Brakedown maps to fma (field multiply-accumulate). going all-algebraic shifts the hardware bottleneck from p2r to fma. the [[Goldilocks field processor]] design may need rebalancing.

## implementation plan

### phase 1: nox reference (data model)

- [ ] nox/reference/nouns.md — add polynomial representation: noun = multilinear polynomial over {0,1}^k. identity = hemera(Lens.commit(content) ‖ tag) for ALL nouns. no threshold, no dual identity
- [ ] nox/reference/vm.md — axis = polynomial evaluation O(1) via Lens opening. one code path for all nouns
- [ ] nox/reference/encoding.md — identity = hemera(Lens.commit(content) ‖ domain_tag). one scheme for everything
- [ ] nox/reference/patterns.md — pattern 0 (axis): O(1) for lens nouns. pattern 15 (hash): hemera wraps lens commit for large content
- [ ] nox/reference/trace.md — trace registers may contain Lens commitments as values
- [ ] nox/reference/jets.md — state jets use O(1) axis. add: lens noun jets
- [ ] nox/reference/reduction.md — proof-carrying: ~3 hemera calls per execution (was hundreds)
- [ ] nox/reference/state-operations.md — READ = O(1) polynomial evaluation

### phase 2: hemera reference (role collapse)

- [ ] hemera/reference/README.md — role: ~3 calls per execution (domain separation, Fiat-Shamir, Brakedown binding)
- [ ] hemera/reference/tree.md — tree hashing for small nouns only. large nouns use Lens commitment

### phase 3: bbg reference (particles ARE polynomials)

- [ ] bbg/reference/architecture.md — particles are polynomials with native Lens openings. axis on particle content = O(1)
- [ ] bbg/reference/data-availability.md — DAS native: polynomial extension beyond hypercube IS erasure code. no separate 2D Reed-Solomon step
- [ ] bbg/reference/storage.md — ShardStore serves polynomial nouns. particle storage = polynomial evaluation table
- [ ] bbg/reference/indexes.md — polynomial access via Lens opening

### phase 4: bbg + nox explanation

- [ ] bbg/docs/explanation/architecture-overview.md — pipeline: axis O(1), hemera ~3 calls, DAS native
- [ ] bbg/docs/explanation/data-availability.md — DAS = polynomial extension (no separate erasure step)
- [ ] bbg/docs/explanation/why-polynomial-state.md — add: nouns ARE polynomials (not just state)

### phase 5: roadmap cleanup

- [ ] DELETE hemera/roadmap/batched-proving.md — ~3 hemera calls, batching is pointless
- [ ] DELETE hemera/roadmap/folded-sponge.md — ~3 hemera calls, folding is pointless
- [ ] DEPRIORITIZE hemera/roadmap/algebraic-fiat-shamir.md — still relevant (1 of 3 calls is FS)
- [ ] DEPRIORITIZE hemera/roadmap/constraint-free-mds.md — marginal with 3 calls
- [ ] DEPRIORITIZE hemera/roadmap/partial-round-collapse.md — marginal with 3 calls

see [[nox]] for the 18 patterns (16 compute + call + look), [[hemera]] for the hash primitive, [[BBG]] for polynomial state, [[zheng]] for the proof system, [[Brakedown]] for the Lens, [[data structures for polynomial state]] for storage architecture, [[algebraic state commitments]] for why polynomial state is natural, [[structural-sync]] for the five verification layers
