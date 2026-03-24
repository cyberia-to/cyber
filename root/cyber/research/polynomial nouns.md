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
| H(noun) — identity | [[hemera]] recursive hash, O(N) | PCS.commit(polynomial), O(N) |
| verify axis | O(depth) hemera hashes | O(1) PCS opening, ~200 bytes |

the structural operations (axis, cons) are unchanged semantically. the COST changes: axis drops from O(depth) to O(1). identity drops from recursive hemera hashing to a single PCS commitment.

## the new digest

a particle CID becomes a PCS commitment instead of a hemera hash:

```
current:    CID = hemera(content)                           32 bytes
algebraic:  CID = hemera(PCS.commit(content) ‖ domain_tag)  32 bytes
```

same size. same content-addressing property. same collision resistance (Brakedown uses hemera internally for the binding hash). but the algebraic CID supports operations that hemera hashes cannot:

| capability | hemera CID | algebraic CID |
|---|---|---|
| content verification | rehash all content, compare | recommit all content, compare |
| partial access | **impossible** — must download everything | PCS.open(position) → ~200 byte proof |
| random access | **impossible** | evaluate at any point, O(1) |
| range proof | **impossible** | batch PCS opening over range |
| algebraic composition | **impossible** | add commitments, multiply by scalar |
| streaming verification | O(log N) × 32B per chunk (BAO) | ~200B per chunk (PCS opening), 5× smaller |

the PCS commitment is a strict superset of hemera hash for content addressing. everything hemera can do, the PCS can do — plus random access, range proofs, and algebraic composition.

## domain separation

hemera's structured capacity provides domain separation: different modes for different purposes (tree nodes vs leaves, commitment vs nullifier). PCS commitments don't have native domain separation.

the solution: hemera WRAPS the PCS commitment.

```
raw PCS commitment:  C = Brakedown.commit(polynomial)         32 bytes
domain-separated:    CID = hemera(C ‖ domain_tag)              32 bytes

domain_tag ∈ {PARTICLE, FORMULA, COMMITMENT, NULLIFIER, SIGNAL, ...}
```

one hemera call per noun, not per node. hemera provides the domain tag. PCS provides the algebraic commitment. the CID has both properties: domain separation (from hemera) and algebraic openings (from PCS, via the raw commitment C which is recoverable from context).

## BAO streaming, algebraic

BAO (BLAKE3 Authenticated Output) verifies content chunk-by-chunk during streaming download, providing per-chunk authentication paths.

polynomial nouns provide algebraic streaming:

```
BAO (hemera tree):
  per-chunk proof:   O(log N) × 32 bytes auth path ≈ 1 KiB
  order:             sequential (tree builds bottom-up)
  seek to chunk k:   must process chunks 0..k-1 first

algebraic streaming:
  per-chunk proof:   PCS.open(commitment, k) ≈ 200 bytes
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
            verify: one PCS opening, ~200 bytes, O(1) field ops
```

for a deep noun (depth 32, e.g., a particle index with $2^{32}$ entries):

```
current:    axis costs 32 tree hops × hemera = 32 × 736 = ~23,500 constraints
algebraic:  axis costs 1 PCS opening ≈ ~200 field operations
```

100× reduction in the most common operation.

### cons is unchanged

cons(a, b) creates a new noun. in tree model: allocate cell, set left=a, right=b. in polynomial model: prepend a variable.

```
cons(a, b) as polynomial:
  g(x₁, x₂, ..., xₖ₊₁) = (1-x₁) · a(x₂, ..., xₖ₊₁) + x₁ · b(x₂, ..., xₖ₊₁)
```

cost: O(1) — one variable prepend. the resulting polynomial is one degree higher in a new variable. no actual computation — the cons is a structural change to the polynomial's variable list.

the commitment: PCS.commit(g) can be computed from PCS.commit(a) and PCS.commit(b) if the PCS supports homomorphic operations. Brakedown's linear structure allows this:

```
commit(g) = (1-r₁) · commit(a) + r₁ · commit(b)    for challenge r₁
```

one field operation to combine two existing commitments.

### the 16 patterns stay

all 16 nox patterns operate on nouns. nouns become polynomials. but the PATTERNS don't change — their semantics are defined over the noun abstraction (atom | cell), not over the representation (tree vs polynomial).

```
pattern 0 (axis):     evaluate polynomial at binary point     (was: traverse tree)
pattern 1 (quote):    return polynomial unchanged              (was: return tree unchanged)
pattern 2 (compose):  compose polynomial operations            (unchanged)
pattern 3 (cons):     prepend variable to polynomial           (was: allocate cell)
pattern 4 (branch):   conditional on polynomial evaluation     (unchanged)
pattern 5-10:         field arithmetic on atom values           (unchanged)
pattern 11-14:        bitwise on word values                    (unchanged)
pattern 15 (hash):    hemera(PCS.commit ‖ domain_tag)           (was: hemera recursive)
pattern 16 (hint):    prover injects witness                    (unchanged)
```

16 patterns. same semantics. different performance characteristics for axis and hash.

## implications for [[hemera]]

hemera's role collapses to three calls:

```
role                          hemera calls     was
─────                         ────────────     ───
1. Fiat-Shamir seed           1 per proof      1 per proof (unchanged)
2. domain separation          1 per noun       O(N/8) per noun (was recursive tree hash)
3. Brakedown binding hash     1 per PCS commit inside Brakedown (internal, unchanged)

total: ~3 hemera calls per proof-carrying nox execution
was:   hundreds to thousands per execution
```

hemera goes from the dominant cost to negligible. the hash function becomes the trust anchor — the cryptographic root that makes PCS binding secure — not the hot-path computation.

all hemera optimizations (folded sponge, batched proving, algebraic Fiat-Shamir, constraint-free MDS, partial round collapse) become less critical for performance because there are so few hemera calls. they remain valuable for the remaining calls but the pressure is off.

## implications for [[BBG]]

### particles ARE polynomials

a particle's content is stored as a multilinear polynomial. the particle CID is hemera(PCS.commit(content_poly) ‖ PARTICLE). accessing any byte range of the particle is a PCS opening. no download needed for partial verification.

```
current:    verify particle content → download all, rehash → O(N) hemera
algebraic:  verify byte range [a,b] → PCS.open(CID, [a,b]) → ~200 bytes, O(1)
```

for a 1 GB neural network model stored as a particle: verify any 4 KB chunk (one layer's weights) with a 200-byte proof. no download of the other 999.996 MB.

### DAS becomes native

content availability is already about proving chunks exist. with polynomial nouns, every particle IS a polynomial with native openings. DAS sampling = opening the particle polynomial at random positions.

```
current DAS:    particle → erasure-code → commit chunks → sample → verify
algebraic DAS:  particle IS polynomial → sample = PCS.open(random_position)
                no separate erasure coding step — the polynomial IS the code
```

the Reed-Solomon erasure coding and the polynomial commitment MERGE. a multilinear polynomial over $\{0,1\}^k$ evaluated on a larger domain $\mathbb{F}_p^k$ IS Reed-Solomon encoded. the polynomial naturally extends beyond the Boolean hypercube, providing redundancy.

this means: DAS is FREE for polynomial nouns. no separate erasure coding. no separate commitment. the noun's PCS commitment IS the DAS commitment. sampling IS PCS opening. verification IS PCS.verify.

### BBG_poly and particle polynomials share a PCS

BBG_poly (state polynomial) and particle content polynomials all use the same Brakedown PCS. one commitment scheme for everything: state queries, content verification, DAS sampling, axis evaluation, proof generation.

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
- axis verification inside the trace at O(1) cost (PCS opening, not hemera path)

### proof-carrying computation stays the same

each reduce() folds one trace row into the [[HyperNova]] accumulator (~30 field ops). polynomial nouns don't change this — the fold mechanism operates on CCS instances regardless of whether the witness values are tree hashes or PCS commitments.

### verifier circuit shrinks

the [[zheng]] verifier (a nox program) currently spends most constraints on hemera hash verification (Merkle paths for PCS, Fiat-Shamir challenges). with polynomial nouns:

```
current verifier:    ~12K constraints (Brakedown)
                     dominated by Fiat-Shamir hemera calls

algebraic verifier:  ~5K constraints (estimated)
                     fewer hemera calls → fewer constraints
                     axis operations in verification are O(1) PCS openings
```

## implications for [[structural-sync|structural sync]]

### layer 3 (completeness) unifies with content model

completeness proofs = PCS openings. content access = PCS openings. they're the same operation. "prove this namespace is complete" and "access this particle's byte range" use the same primitive.

### layer 4 (availability) becomes native

DAS = PCS sampling on polynomial nouns. no separate erasure coding infrastructure. the noun IS the erasure code (polynomial extends beyond Boolean hypercube naturally).

```
current:    5 verification layers, each with its own mechanism
algebraic:  5 verification layers, layers 3+4 merge into PCS operations
```

## the cost threshold

PCS commitment is expensive for small data. hemera is cheaper for tiny nouns:

```
cell(1, 2) = 2 atoms:
  hemera: 1 permutation = ~736 constraints
  PCS:    Brakedown encode of 2 elements = ~thousands of field ops (overhead)

particle with 1000 elements:
  hemera: ~125 permutations (1000/8 rate blocks) = ~92K constraints
  PCS:    Brakedown encode of 1000 elements = ~1000 field ops

particle with 1M elements:
  hemera: ~125K permutations = ~92M constraints
  PCS:    Brakedown encode of 1M elements = ~1M field ops (92× cheaper)
```

the crossover: PCS wins above ~100 elements (~800 bytes). below that, hemera is cheaper.

practical strategy:

```
atom (8 bytes):          hemera hash — trivially cheap
small noun (< 100 elements): hemera hash — O(N/8) permutations
large noun (≥ 100 elements): PCS commitment — O(N) field ops, O(1) access
```

the 16 nox patterns handle both representations transparently. axis on a hemera-identified noun = tree traversal. axis on a PCS-identified noun = polynomial evaluation. the pattern semantics are the same. the jet dispatches differently based on the identity type.

## the unified primitive

the endgame: one cryptographic primitive (Brakedown PCS) for all purposes.

```
noun identity:       PCS.commit(noun_polynomial)
state commitment:    PCS.commit(BBG_poly)
private records:     PCS.commit(A), PCS.commit(N)
proof commitment:    PCS.commit(trace_polynomial)
content access:      PCS.open(noun_commitment, position)
state query:         PCS.open(BBG_poly_commitment, (dim, key, t))
DAS sampling:        PCS.open(noun_commitment, random_position)
completeness proof:  PCS.open(commitment, namespace_range)

hemera's remaining role:
  domain_tag:        hemera(PCS.commit ‖ tag) — one call per identity
  Fiat-Shamir:       hemera seed — one call per proof
  Brakedown binding: hemera internally — one call per commit
```

one PCS. one field ([[Goldilocks field|Goldilocks]]). one hash ([[hemera]], now ~3 calls per execution). one proof system ([[zheng]]). one VM ([[nox]], 16 patterns). trees and polynomials are the same object. computation and state share one commitment scheme. proving and accessing share one opening protocol.

## open questions

1. **small noun overhead.** Brakedown encoding of 2-element polynomials is wasteful. can a lightweight commitment (hemera hash) coexist with heavyweight commitment (PCS) under one identity scheme? the domain tag could encode the commitment type: DOMAIN_HASH vs DOMAIN_PCS. axis dispatch checks the type.

2. **cons homomorphism.** can Brakedown.commit(cons(a,b)) be computed from commit(a) and commit(b) without re-encoding? Brakedown's linearity suggests yes: $\text{commit}(g) = (1-r) \cdot \text{commit}(a) + r \cdot \text{commit}(b)$ for a challenge $r$. needs formal verification.

3. **polynomial degree management.** cons increases variable count by 1. deep nesting → high-variable polynomial. Brakedown commitment cost grows with variable count. is there a practical limit on nesting depth? (current nox: focus limits depth implicitly.)

4. **migration.** all existing particle CIDs (hemera hashes) would change. the cybergraph needs either migration (rehash everything as PCS commitments) or dual identity support (hemera CID for legacy, PCS CID for new). signal-first architecture helps: replay signals with new identity scheme → reconstruct graph with PCS CIDs.

5. **Reed-Solomon from polynomial extension.** the claim that "polynomial extension beyond Boolean hypercube IS erasure coding" needs formal proof. specifically: does evaluating a $k$-variate multilinear polynomial on $\mathbb{F}_p^k$ (instead of $\{0,1\}^k$) provide the same reconstruction guarantees as 2D Reed-Solomon? the degree structure differs — multilinear has degree 1 per variable, not degree $k$ overall.

6. **SIMD and hardware.** hemera maps to the p2r (Poseidon2 round) GFP hardware primitive. Brakedown maps to fma (field multiply-accumulate). going all-algebraic shifts the hardware bottleneck from p2r to fma. the [[Goldilocks field processor]] design may need rebalancing.

see [[nox]] for the 16 patterns, [[hemera]] for the hash primitive, [[BBG]] for polynomial state, [[zheng]] for the proof system, [[Brakedown]] for the PCS, [[data structures for polynomial state]] for storage architecture, [[algebraic state commitments]] for why polynomial state is natural, [[structural-sync]] for the five verification layers
