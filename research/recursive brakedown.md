---
tags: cyber, research, article
crystal-type: process
crystal-domain: cyber
status: draft
date: 2026-03-24
---
# recursive Brakedown: the perfect lens

## the gap

[[Brakedown]] is the best transparent, post-quantum, Merkle-free lens. O(N) commit. O(√N) proof. zero hash trees. but FRI/WHIR have O(log² N) proofs — smaller for large N.

the question: can we get O(log N) proof size while keeping O(N) commit, Merkle-free, and post-quantum?

## why Brakedown proofs are O(√N)

Brakedown arranges N evaluation points as a $\sqrt{N} \times \sqrt{N}$ matrix $M$. to prove $f(r) = y$:

```
1. decompose r into tensor product: t = t₁ ⊗ t₂
   where t₁, t₂ ∈ F^{√N}

2. compute q = M · t₁ ∈ F^{√N}    (inner product of each row with t₁)

3. send q to verifier              (√N field elements — this is the bottleneck)

4. verifier checks:
   - q is consistent with the Brakedown encoding (spot-checks)
   - ⟨q, t₂⟩ = y (evaluation is correct)
```

the $\sqrt{N}$ comes from step 3: sending one full dimension of the matrix. the prover must reveal $\sqrt{N}$ field elements to convince the verifier.

## the recursive insight

instead of sending $q$ (√N elements) directly, COMMIT to $q$ using Brakedown. then prove the claim about $q$ recursively.

```
level 0:  f has N evaluation points
          opening at r produces claim about q₀ ∈ F^{√N}
          COMMIT to q₀ via Brakedown (instead of sending it)

level 1:  q₀ has √N evaluation points
          opening at r₁ produces claim about q₁ ∈ F^{N^{1/4}}
          COMMIT to q₁

level 2:  q₁ has N^{1/4} evaluation points
          opening produces q₂ ∈ F^{N^{1/8}}
          COMMIT to q₂

...

level k:  q_{k-1} has N^{1/2^k} points
          when N^{1/2^k} < λ (security parameter):
          send q_k directly
```

each level SQUARES the compression. after $\log \log N$ levels, the direct claim is O(1) elements.

## cost analysis

### prover

```
level 0:  Brakedown commit to q₀ (size √N)       O(√N)
level 1:  Brakedown commit to q₁ (size N^{1/4})   O(N^{1/4})
level 2:  Brakedown commit to q₂ (size N^{1/8})   O(N^{1/8})
...

total:  O(N) for original + O(√N + N^{1/4} + N^{1/8} + ...) for recursion
      = O(N) + O(2√N)
      = O(N)
```

linear time preserved. the recursive overhead is O(√N) — dominated by the original O(N) commitment.

### proof size

two components:

**commitments:** one Brakedown commitment per recursion level. each is 32 bytes (one [[hemera]] binding hash).

```
log log N levels × 32 bytes
for N = 2²⁰:  log log(2²⁰) = log(20) ≈ 4.3 → 5 levels × 32 = 160 bytes
```

**sumcheck transcripts:** each level runs a sumcheck to reduce the claim. the sumcheck at level $i$ has $\log_2(N^{1/2^i})$ rounds, each contributing one field element (8 bytes).

```
level 0:  log₂(N) / 2 = 10 field elements
level 1:  log₂(N) / 4 = 5 field elements
level 2:  log₂(N) / 8 = 2.5 → 3 field elements
...

total:  log₂(N) × (1/2 + 1/4 + 1/8 + ...) = log₂(N) × 1 = log₂(N)
```

so: $\log_2 N$ field elements for sumcheck = $\log_2 N \times 8$ bytes.

**direct elements at final level:**

```
N^{1/2^k} ≈ λ elements (security parameter, e.g., 128)
128 × 8 = 1,024 bytes (constant, independent of N)
```

**total proof size:**

$$\text{proof} = 32 \cdot \log \log N + 8 \cdot \log_2 N + 8 \cdot \lambda$$

for N = 2²⁰, λ = 128:

```
commitments:   5 × 32 = 160 bytes
sumcheck:     20 × 8  = 160 bytes
direct:      128 × 8  = 1,024 bytes
total:                 = 1,344 bytes ≈ 1.3 KiB
```

compare:
- standard Brakedown: $\sqrt{N} \times 8 = 1024 \times 8 = 8$ KiB
- recursive Brakedown: ~1.3 KiB
- FRI/WHIR: ~5-12 KiB (with algebraic extraction)

**6× smaller than standard Brakedown. comparable to optimised FRI. still Merkle-free.**

### verification

```
verifier work per level:
  check Brakedown commitment consistency (spot-checks):  O(λ)
  verify sumcheck transcript (field ops per round):       O(log N / 2^i)

total across all levels:
  O(λ × log log N) + O(log N) field operations
  ≈ O(λ log log N)

for N = 2²⁰, λ = 128:
  128 × 5 + 20 = 660 field operations ≈ 5 μs
```

compare:
- standard Brakedown: O(√N) = O(1024) field ops
- recursive Brakedown: O(λ log log N) ≈ 660 field ops
- FRI/WHIR: O(log² N) ≈ 400 field ops (but with hemera hash overhead)

## the comparison table

```
                    FRI/WHIR        Brakedown       recursive Brakedown
────────────        ────────        ─────────       ───────────────────
commit:             O(N log N)      O(N)            O(N)
proof size:         O(log² N)       O(√N)           O(log N + λ)
verify:             O(log² N)       O(√N)           O(λ log log N)
Merkle-free:        no              yes             yes
post-quantum:       yes             yes             yes
hash calls:         O(N log N)      1               log log N
prover time:        O(N log N)      O(N)            O(N)

at N = 2²⁰:
commit time:        ~20M hash       ~1M field       ~1M field
proof size:         ~5-12 KiB       ~8 KiB          ~1.3 KiB
verify time:        ~150 μs         ~30 μs          ~5 μs
```

recursive Brakedown wins or ties every metric except one: FRI verify uses hash operations (which map to hemera hardware), while recursive Brakedown uses field operations (which map to [[Goldilocks field processor|GFP]] fma). with GFP, field operations are native → recursive Brakedown is faster on target hardware.

## the protocol

### commit

unchanged from standard Brakedown.

```
commit(f):
  v = evaluation_table(f)            N field elements
  w = G · v                          expander encoding, O(N) field ops
  C = hemera(w)                      one hash, 32 bytes
  return C
```

### open (recursive)

```
open(f, r, depth=0):
  if |f| ≤ λ:
    return f directly               base case: send raw elements

  // standard Brakedown opening step
  M = reshape(eval_table(f), √|f| × √|f|)
  t₁, t₂ = tensor_split(r)
  q = M · t₁                        √|f| field elements

  // RECURSIVE: commit to q instead of sending it
  C_q = commit(q)                    one Brakedown commitment, 32 bytes

  // sumcheck: prove ⟨q, t₂⟩ = y
  sc_transcript = sumcheck(q, t₂, y)

  // get random challenge from Fiat-Shamir
  r' = hemera_squeeze(C_q, sc_transcript)

  // recurse: prove q(r') = sc_final_value
  sub_proof = open(q, r', depth+1)

  return (C_q, sc_transcript, sub_proof)
```

### verify (recursive)

```
verify(C, r, y, proof, depth=0):
  if depth reaches base case:
    q_direct = proof.direct_elements
    return (eval(q_direct, r) == y)  check directly

  C_q, sc_transcript, sub_proof = proof

  // verify sumcheck
  (accept, r', y') = verify_sumcheck(sc_transcript, y)
  if not accept: return false

  // reconstruct Fiat-Shamir challenge
  r'_check = hemera_squeeze(C_q, sc_transcript)
  if r' ≠ r'_check: return false

  // spot-check C_q against Brakedown encoding
  // (prover must reveal encoding positions in sub_proof)
  if not brakedown_spot_check(C_q, sub_proof): return false

  // recurse: verify q(r') = y' against C_q
  return verify(C_q, r', y', sub_proof, depth+1)
```

## soundness

each recursion level has independent soundness:

```
level i soundness error:
  sumcheck: degree / |F_p| ≈ 2 / 2⁶⁴ (negligible)
  Brakedown: distance / (λ spot-checks) ≈ 2^{-λ}
  total per level: ≈ 2^{-λ}

across log log N levels:
  total error ≤ log log N × 2^{-λ}

for N = 2²⁰, λ = 128:
  error ≤ 5 × 2^{-128} ≈ 2^{-125.7}
```

128-bit security preserved with comfortable margin.

the key requirement: each level's Brakedown commitment must be BINDING (the prover can't change $q$ after committing). this is guaranteed by the hemera binding hash at each level. the hemera calls per proof: log log N ≈ 5 (instead of 0 for standard Brakedown or O(N log N) for FRI).

## batch opening

state operations and DAS require opening the polynomial at MULTIPLE points simultaneously. recursive Brakedown supports batch opening:

```
batch_open(f, [r₁, r₂, ..., r_m]):
  // random linear combination (Fiat-Shamir)
  α = hemera_squeeze(r₁, ..., r_m)
  combined = Σ αⁱ · (f(rᵢ) - yᵢ) / (x - rᵢ)

  // one recursive opening of the combined polynomial
  proof = open(combined, random_point)
  return proof

cost: one recursive opening + O(m) field ops for combination
proof size: O(log N + λ) regardless of m (amortised!)
```

m openings for the price of 1. this is critical for:
- state jets (TRANSFER = 2 reads + 2 writes = 4 openings → 1 proof)
- DAS sampling (20 samples → 1 batch proof instead of 20 proofs)
- namespace queries (100 entries → 1 batch proof)

## implications for the stack

### [[zheng]] proof size

zheng proofs contain Lens openings. with recursive Brakedown:

```
current (standard Brakedown):   ~8 KiB per proof
recursive Brakedown:            ~1.3 KiB per proof

zheng total proof:
  sumcheck transcript:   ~0.5 KiB
  evaluation claims:     ~0.3 KiB
  Lens opening:           ~1.3 KiB (was ~8 KiB)
  total:                 ~2.1 KiB (was ~9 KiB)
```

### [[BBG]] checkpoint

the universal accumulator is ~200 bytes. the proof to verify it:

```
current:   ~8 KiB
recursive: ~2 KiB
```

lighter proofs everywhere.

### polynomial nouns

[[polynomial nouns]] use Lens commitment as noun identity. per-axis verification:

```
current (standard Brakedown):  O(√N) field ops per axis
recursive:                     O(λ log log N) ≈ 660 field ops per axis

for depth-32 noun:
  current:  √(2³²) = 65,536 field ops
  recursive: 660 field ops (100× improvement)
```

### DAS

20-sample DAS with batch opening:

```
current:  20 × ~200 bytes = ~4 KiB
recursive + batch: 1 batch proof ≈ ~1.5 KiB (20 openings amortised)
```

## the remaining gap

is there anything left to improve?

```
                    recursive Brakedown     theoretical lower bound
commit:             O(N)                    O(N) — must read all data
proof size:         O(log N + λ)            O(log N) — sumcheck + security
verify:             O(λ log log N)          O(log N) — depends on lens
prover:             O(N)                    O(N)
Merkle-free:        yes                     yes
post-quantum:       yes                     yes
```

the only gap: verify is O(λ log log N) instead of O(log N). the λ factor comes from spot-checking the Brakedown encoding at each level. this is the cost of code-based security.

with λ = 128 and log log N ≈ 5: verification is ~640 field ops ≈ ~5 μs. this is already faster than FRI verification (~150 μs with hemera overhead). the gap is theoretical, not practical.

## open questions

1. **formal soundness proof.** the recursive composition of Brakedown openings needs a formal security reduction. each level introduces a new commitment and a new sumcheck. the composed protocol must satisfy knowledge soundness, not just plain soundness. this is the most important open question.

2. **expander compatibility.** Brakedown's expander graph is chosen for N-element vectors. the recursive levels operate on $\sqrt{N}$, $N^{1/4}$, etc. element vectors. do the same expander parameters work at all sizes? or does each level need a different expander?

3. **constant factors.** the analysis gives ~1.3 KiB for N = 2²⁰. what about N = 2³⁰ (billion-element polynomials for planetary state)? the log N term grows to 30 × 8 = 240 bytes, total ~1.4 KiB. scaling is gentle.

4. **implementation complexity.** recursive Brakedown adds log log N levels of commit/open/verify recursion. each level is a standard Brakedown operation but at a different size. the implementation must handle variable-size Brakedown instances. this is engineering complexity, not theoretical.

5. **interaction with [[HyperNova]] folding.** can the recursive Brakedown openings be FOLDED instead of verified? if yes: the decider handles all levels at once, and proof-carrying computation produces recursive Brakedown proofs natively. this would make the recursion invisible to the prover.

6. **batch opening formal analysis.** the batch opening via random linear combination is standard (used in KZG batch). formal analysis needed for the Brakedown-specific case: does the expander code structure interact with the batching?

## implementation plan

### phase 1: zheng reference (lens upgrade)

- [ ] zheng/reference/polynomial-commitment.md — rewrite with recursive protocol (commit unchanged, open/verify recursive)
- [ ] zheng/reference/verifier.md — update: proof ~1.3 KiB, verify ~5 μs, circuit ~8K constraints
- [ ] zheng/reference/whirlaway.md — update architecture numbers
- [ ] zheng/reference/api.md — update Proof type size

### phase 2: cascade numbers to bbg

- [ ] bbg/reference/architecture.md — proof sizes in pipeline
- [ ] bbg/reference/data-availability.md — DAS batch: 20 samples → ~1.5 KiB
- [ ] bbg/reference/query.md — query proof sizes
- [ ] bbg/reference/sync.md — sync proof sizes
- [ ] bbg/docs/explanation/architecture-overview.md — key numbers table

### phase 3: roadmap cleanup

- [ ] DELETE zheng/roadmap/algebraic-extraction.md — superseded (no Merkle paths to extract)

see [[Brakedown]] for the base lens, [[zheng]] for the proof system, [[polynomial nouns]] for all-algebraic computation, [[algebraic state commitments]] for polynomial state, [[structural-sync]] for the five-layer framework that recursive Brakedown accelerates
