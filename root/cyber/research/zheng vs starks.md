---
title: "zheng: a self-proving proof system"
authors: [cyber]
date: 2026-03-23
status: draft
---

# zheng: a self-proving proof system

## Abstract

zheng is a proof system where computation IS proving. a nox program executes, generating an execution trace. the trace is a table of field elements — 16 registers wide, N rows deep. a multilinear polynomial interpolates this table over the Boolean hypercube. CCS constraints express what a valid trace looks like. SuperSpartan reduces constraint checking to a sumcheck over the constraint polynomial. Brakedown commits to the trace polynomial using expander-graph codes — no Merkle trees, no hashing, O(N) field operations. HyperNova folds multiple proof instances into a running accumulator with ~30 field operations per fold. the accumulator IS the proof — run one decider to verify everything.

the result: proving overhead is ~30 field operations per computation step. a 240-byte checkpoint proves the entire history of a planetary-scale knowledge graph. verification takes 10–50 μs.

this paper shows HOW each mechanism works, not just what it achieves.

## 1. From Computation to Proof

the pipeline has seven stages. each transforms data into a form the next stage can process:

```
nox program              "compute the cybergraph update"
    ↓ execute
execution trace          16 × N table of Goldilocks field elements
    ↓ interpolate
multilinear polynomial   f: {0,1}^k → F_p  where N = 2^k
    ↓ constrain
CCS instance             matrices M_i encoding "valid trace"
    ↓ sumcheck
claimed evaluation       "f satisfies all constraints" reduced to one field equation
    ↓ commit + open
Brakedown proof          polynomial commitment + evaluation proof (~1-5 KiB)
    ↓ fold
accumulator              running HyperNova claim (~200 bytes)
    ↓ decide
final proof              one STARK proof verifiable in 10-50 μs
```

## 2. Execution Trace

### 2.1 What the trace IS

a nox program is a sequence of reduce() calls. each call dispatches one of 17 patterns (axis, quote, compose, cons, branch, add, sub, mul, inv, eq, lt, xor, and, not, shl, hash, hint). each pattern produces one or more rows in the execution trace.

the trace is a matrix T with 16 columns (registers r0–r15) and N rows:

```
row     r0    r1    r2    r3    r4    r5    ...   r15
─────   ────  ────  ────  ────  ────  ────  ────  ────
0       tag   h_obj h_frm op_a  op_b  res   ...   status
1       tag   h_obj h_frm op_a  op_b  res   ...   status
...
N-1     tag   h_obj h_frm op_a  op_b  res   ...   status
```

register assignments:

| register | name | content |
|---|---|---|
| r0 | tag | pattern identifier (0–16) |
| r1 | h_obj | hemera hash of the subject noun |
| r2 | h_frm | hemera hash of the formula noun |
| r3–r4 | operands | pattern-specific input values |
| r5 | result | pattern output |
| r6 | focus | remaining focus after this step |
| r7–r8 | type tags | operand and result type information |
| r9–r13 | auxiliary | pattern-specific intermediate values |
| r14 | prev_hash | hash of previous row (chain integrity) |
| r15 | status | 0 = ok, nonzero = error code |

every value is a Goldilocks field element (64 bits, p = 2⁶⁴ − 2³² + 1).

### 2.2 How patterns generate rows

simple patterns (add, mul, eq) produce ONE row:

```
add(a, b):
  r0 = 5 (pattern tag for add)
  r3 = a, r4 = b
  r5 = a + b mod p
  r6 = focus - 1
  → 1 row, 1 reduce() call
```

complex patterns produce MULTIPLE rows:

```
axis(subject, path) at depth d:
  row 0: start traversal, r3 = subject_hash, r4 = path
  row 1: descend one level, r3 = child_hash
  ...
  row d-1: final level, r5 = result_hash
  → d rows, 1 reduce() call

hash(x):
  rows 0–199: Poseidon2 permutation steps
  each row = one round of the permutation
  → ~200 rows, 1 reduce() call
```

N (total rows) depends on the computation. a cyberlink creation with 10 reduce() calls and 2 hash operations: ~10 + 2×200 = ~410 rows.

## 3. Arithmetization: CCS

### 3.1 What CCS IS

a Customizable Constraint System (CCS) generalizes R1CS, Plonkish, and AIR constraints into one format. a CCS instance is:

```
CCS = (M₁, M₂, ..., M_t, S₁, S₂, ..., S_q, c₁, c₂, ..., c_q)

where:
  M_i ∈ F^{m×n}    sparse matrices (t of them)
  S_j ⊆ {1,...,t}   sets of matrix indices (q of them)
  c_j ∈ F           scalar coefficients (q of them)
  m = number of constraints
  n = number of variables (= 16 × N for trace)
```

the CCS is satisfied by witness vector z ∈ F^n if:

$$\sum_{j=1}^{q} c_j \cdot \bigodot_{i \in S_j} M_i \cdot z = 0$$

where ⊙ denotes Hadamard (element-wise) product.

### 3.2 How nox constraints become CCS

each nox pattern defines constraints over consecutive trace rows. example for add (pattern 5):

```
constraint: r5[t] = r3[t] + r4[t]    (result = operand_a + operand_b)

in CCS form:
  M_result · z - M_op_a · z - M_op_b · z = 0

where:
  M_result selects column r5 at each row
  M_op_a selects column r3
  M_op_b selects column r4
```

pattern-specific constraints are activated by the tag column r0:

```
row t has r0[t] = 5 (add pattern):
  constraint: r5[t] - r3[t] - r4[t] = 0        (add semantics)
  constraint: r6[t] = r6[t-1] - 1                (focus decrements)
  constraint: r15[t] = 0                          (no error)

row t has r0[t] = 7 (mul pattern):
  constraint: r5[t] - r3[t] × r4[t] = 0          (mul semantics)
  constraint: r6[t] = r6[t-1] - 1                 (focus decrements)
```

the tag acts as a SELECTOR: constraints for pattern k are multiplied by eq(r0[t], k). this is the CCS mechanism — the matrices M_i encode the selector logic.

### 3.3 Transition constraints (AIR)

adjacent rows are linked by transition constraints:

```
for all t:
  r14[t] = H(r0[t-1], r1[t-1], ..., r15[t-1])   (chain integrity)
  r6[t] ≤ r6[t-1]                                  (focus monotonically decreasing)

for multi-row patterns (axis, hash):
  r1[t] relates to r1[t-1]                          (state continuity)
```

these are AIR (Algebraic Intermediate Representation) constraints — they reference both row t and row t−1.

### 3.4 The constraint count

```
per-row constraints:
  pattern semantics:     1–3 constraints (depends on pattern)
  focus accounting:      1 constraint
  chain integrity:       1 constraint (hemera hash check)
  selector activation:   1 constraint (tag matching)
  type checking:         1–2 constraints
  total per row:         ~5–8 constraints

boundary constraints:
  r6[0] = initial_focus
  r1[0] = H(input)
  r5[N-1] = H(output)
  r15[N-1] = 0 (no error)

total: ~8N constraints for N-row trace
```

## 4. Multilinear Extension

### 4.1 What the MLE IS

the trace T is an N × 16 matrix. we view it as a function:

$$T: \{0,...,N-1\} \times \{0,...,15\} \to \mathbb{F}_p$$

to make this work with sumcheck, we need a POLYNOMIAL that agrees with T on all input points. the multilinear extension (MLE) does this.

represent row index i in binary: $i = (b_1, b_2, ..., b_k)$ where $k = \log_2 N$.
represent column index j in binary: $j = (b_{k+1}, ..., b_{k+4})$ (4 bits for 16 columns).

the MLE is:

$$\tilde{T}(x_1, ..., x_{k+4}) = \sum_{(b_1,...,b_{k+4}) \in \{0,1\}^{k+4}} T[b] \cdot \tilde{eq}(x, b)$$

where $\tilde{eq}(x, b) = \prod_{i=1}^{k+4} (x_i b_i + (1-x_i)(1-b_i))$

key properties:
- $\tilde{T}$ is multilinear (degree 1 in each variable)
- $\tilde{T}(b) = T[b]$ for all binary inputs (agrees with trace on the hypercube)
- $\tilde{T}$ can be evaluated at ANY point in $\mathbb{F}_p^{k+4}$, not just binary

this extension is unique. any multilinear polynomial that agrees with T on $\{0,1\}^{k+4}$ IS $\tilde{T}$.

### 4.2 Why multilinear, not univariate

FRI-based systems use UNIVARIATE polynomials (one variable, degree N). the evaluation domain is a multiplicative subgroup, and commitment uses FFT + Merkle tree.

zheng uses MULTILINEAR polynomials (k+4 variables, degree 1 per variable). the evaluation domain is the Boolean hypercube $\{0,1\}^{k+4}$, and commitment uses Brakedown (expander codes, no tree).

consequence: sumcheck operates by fixing one variable at a time, each round halving the domain. k+4 rounds for a trace of size $N \times 16 = 2^{k+4}$. the prover's work per round is proportional to the current domain size, which halves each round: total = $2^{k+4} + 2^{k+3} + ... + 1 = 2^{k+5} - 1 = O(N)$.

this is why the prover is O(N), not O(N log N). multilinear + sumcheck = linear prover.

## 5. SuperSpartan: The IOP

### 5.1 What SuperSpartan does

SuperSpartan is an Interactive Oracle Proof (IOP) that checks whether the CCS is satisfied:

$$\sum_{j=1}^{q} c_j \cdot \bigodot_{i \in S_j} M_i \cdot z = 0 \quad \text{at every row}$$

rather than checking each row individually (O(N) checks), SuperSpartan reduces this to ONE sumcheck:

"does the constraint polynomial sum to zero over the Boolean hypercube?"

### 5.2 The protocol

**step 1: random linear combination.** the verifier sends random $\tau \in \mathbb{F}_p^{\log m}$. this selects a random linear combination of all m constraints:

$$Q(\tau) = \sum_{x \in \{0,1\}^{\log N}} \left[ \sum_{j=1}^{q} c_j \cdot \prod_{i \in S_j} (\widetilde{M_i \cdot z})(x) \right] \cdot \tilde{eq}(\tau, x)$$

if the CCS is satisfied, Q(τ) = 0 for every τ with overwhelming probability (Schwartz-Zippel).

**step 2: sumcheck.** the prover and verifier run the sumcheck protocol on Q(τ). this reduces the exponential sum (over $2^{\log N}$ terms) to a single evaluation at a random point $r$.

**step 3: evaluation check.** the verifier needs $\widetilde{M_i \cdot z}(r)$ for each matrix $M_i$. the prover provides these values plus opening proofs via the PCS (Brakedown).

the verifier checks: the claimed values are consistent with the committed polynomial, and the sumcheck transcript is correct.

### 5.3 Cost

```
prover:   O(t × N) field operations
          t = number of CCS matrices (~17 for nox)
          N = trace rows
          total: ~17N field operations

verifier: O(t × log N) field operations
          ~17 × k multiplications to check evaluation
          + sumcheck verification (k rounds, O(1) per round)
          + PCS verification (Brakedown: O(√N))
          total: ~300 field operations + O(√N)

proof:    sumcheck transcript: ~2 KiB
          evaluation claims: ~1 KiB
          PCS opening: ~1-5 KiB (Brakedown)
          total: ~4-8 KiB
```

## 6. Sumcheck: The Engine

### 6.1 What sumcheck proves

given a k-variate polynomial $g: \mathbb{F}_p^k \to \mathbb{F}_p$, sumcheck proves:

$$H = \sum_{b \in \{0,1\}^k} g(b)$$

the prover claims H. the verifier checks this claim in k rounds, doing O(1) work per round.

### 6.2 Round by round

**round 1.** the prover sends univariate polynomial:

$$g_1(X_1) = \sum_{b_2,...,b_k \in \{0,1\}^{k-1}} g(X_1, b_2, ..., b_k)$$

the verifier checks: $g_1(0) + g_1(1) = H$ (the claimed sum splits correctly).
the verifier sends random $r_1 \in \mathbb{F}_p$.

**round 2.** the prover sends:

$$g_2(X_2) = \sum_{b_3,...,b_k \in \{0,1\}^{k-2}} g(r_1, X_2, b_3, ..., b_k)$$

the verifier checks: $g_2(0) + g_2(1) = g_1(r_1)$ (round 2 is consistent with round 1).
the verifier sends random $r_2$.

**rounds 3 through k.** same pattern. each round:
- prover sends univariate $g_i(X_i)$
- verifier checks $g_i(0) + g_i(1) = g_{i-1}(r_{i-1})$
- verifier sends random $r_i$

**final check.** after round k, the verifier has random point $r = (r_1, ..., r_k)$. the verifier checks:

$$g_k(r_k) = g(r_1, r_2, ..., r_k)$$

the right side requires evaluating g at one point. the verifier asks the prover for the evaluation and checks it via the PCS.

### 6.3 Why this is sound

if the prover cheated (claimed wrong H), then the constraint $g_1(0) + g_1(1) = H$ requires $g_1$ to be wrong. but a wrong univariate of degree d disagrees with the correct one at all but d points. the verifier's random $r_1$ hits the wrong value with probability d/|F_p|. over Goldilocks: d/2⁶⁴ ≈ negligible.

the argument cascades: each round catches cheating with probability ≥ 1 - d/|F_p|. over k rounds, total soundness error ≤ k·d/|F_p|.

### 6.4 Prover efficiency

the prover maintains a bookkeeping table that halves each round:

```
round 1:  table has 2^k entries → compute g_1 in O(2^k) time
round 2:  table has 2^{k-1} entries → O(2^{k-1}) time
round 3:  table has 2^{k-2} entries → O(2^{k-2}) time
...
round k:  table has 1 entry → O(1) time

total: 2^k + 2^{k-1} + ... + 1 = 2^{k+1} - 1 = O(N)
```

this is the source of O(N) proving. no FFT. no NTT. just a table that shrinks.

## 7. Brakedown: Merkle-Free Commitment

### 7.1 Why not Merkle trees

WHIR (and FRI) commit to polynomials by building a Merkle tree over the evaluation table. this costs:

- commitment: O(N log N) hemera hash calls
- opening: O(log² N) hemera hash calls (authentication path)
- proof size: 77% of the proof is Merkle paths

Brakedown eliminates the tree entirely.

### 7.2 How Brakedown commits

**the expander graph.** choose a bipartite expander graph G with N left nodes, c·N right nodes, left-degree d. the graph has the expansion property: every subset S of left nodes has ≥ (1-ε)·d·|S| distinct right neighbors.

**encoding.** treat the polynomial's evaluation table as a vector $v \in \mathbb{F}_p^N$. compute the encoded vector:

$$w = G \cdot v \in \mathbb{F}_p^{c \cdot N}$$

this is a sparse matrix-vector multiply — O(d·N) field operations. since d is a constant (typically 6-10), this is O(N).

**the commitment IS the encoded vector.** no hashing. no tree. just $w$.

(in practice, the verifier doesn't receive all of $w$. the prover sends a hash of $w$ as a binding commitment — one hemera call total. the opening protocol then reveals selected entries.)

### 7.3 How Brakedown opens

to prove $f(r) = y$ (the polynomial evaluates to y at point r):

**step 1.** represent the evaluation as a tensor product:

$$f(r) = \langle v, t_1 \otimes t_2 \otimes ... \otimes t_k \rangle$$

where $t_i = (1 - r_i, r_i)$. the tensor product selects the evaluation at r from the evaluation table v.

**step 2.** the prover arranges v as a $\sqrt{N} \times \sqrt{N}$ matrix and sends the inner products of each row with the first half of the tensor product: a vector $q \in \mathbb{F}_p^{\sqrt{N}}$.

**step 3.** the verifier checks:
- $q$ is consistent with the encoding (random spot-checks on $w$)
- $\langle q, \text{second-half-tensor} \rangle = y$ (evaluation is correct)

### 7.4 The numbers

```
N = 2²⁰ (1M trace entries), security = 128 bits:

commitment:     1 hemera call (hash of encoded vector)
                + O(N) field operations for encoding
                = O(N) total, zero tree construction

opening:        √N = 1024 field elements revealed
                + spot-check consistency proofs
                = ~8 KiB proof

verification:   O(√N) field operations
                + 1 hemera call (check commitment hash)
                = ~1,100 field operations total

compare WHIR:
  commitment:   O(N log N) hemera calls (tree)
  opening:      O(log² N) hemera calls (paths)
  verification: O(log² N) hemera calls
  proof:        ~157 KiB
```

Brakedown reduces hemera usage from dominant cost to negligible (one call for binding). the bottleneck becomes nebu field arithmetic — exactly where Goldilocks excels.

## 8. Fiat-Shamir: Making It Non-Interactive

### 8.1 The problem

the sumcheck protocol is INTERACTIVE: the verifier sends random challenges, the prover responds. in a real system, the proof must be a self-contained byte string — no interaction.

### 8.2 The Fiat-Shamir transform

replace the verifier's random challenges with hemera hash outputs:

```
transcript = hemera sponge (absorb-squeeze)

round 1:
  absorb(commitment, statement)
  absorb(g_1 coefficients)
  r_1 = squeeze()                    ← deterministic "random" challenge

round 2:
  absorb(g_2 coefficients)
  r_2 = squeeze()

...

round k:
  absorb(g_k coefficients)
  r_k = squeeze()
```

the proof is the transcript: all the prover's messages concatenated. the verifier replays the transcript, recomputing challenges, checking consistency.

### 8.3 hemera in Fiat-Shamir

hemera's Poseidon2 sponge (16-element state, 8-element capacity) absorbs field elements directly. no serialization overhead — the native data type IS the field element.

```
Fiat-Shamir cost:
  current: ~20 hemera squeeze calls per proof = 20 × 736 = ~14,720 constraints
  hemera-3 algebraic FS: 1 hemera seed + 19 polynomial challenges = ~1,686 constraints
```

hemera-3's algebraic Fiat-Shamir derives subsequent challenges from polynomial evaluations rather than additional hash calls — 8.7× reduction in Fiat-Shamir cost.

## 9. HyperNova: Folding Instead of Verifying

### 9.1 The recursion problem

a blockchain with 1000 transactions per block needs to compose 1000 proofs. naive approach: verify each proof inside a circuit and prove the verification.

```
cost of verifying one proof inside a circuit:
  sumcheck replay:      ~5K constraints
  Brakedown opening:    ~5K constraints
  Fiat-Shamir:          ~1.7K constraints
  total:                ~12K constraints (with Brakedown)

1000 transactions:      1000 × 12K = 12M constraints for block proof
```

even with Brakedown (vs WHIR's 70K), recursive verification is expensive.

### 9.2 What folding does

HyperNova does NOT verify proofs. it FOLDS them:

given two CCS instances $(E_1, w_1)$ and $(E_2, w_2)$, produce a SINGLE instance $(E', w')$ such that:

- if both originals are satisfied, the fold is satisfied
- if either is unsatisfied, the fold is unsatisfied (with overwhelming probability)
- cost: ~30 field operations + 1 hemera hash

the fold does not check anything. it combines claims. verification happens ONCE at the end.

### 9.3 How folding works

**step 1.** the verifier (or Fiat-Shamir) provides random scalar $\rho$.

**step 2.** the folded instance combines the two linearly:

$$E' = E_1 + \rho \cdot E_2$$
$$w' = w_1 + \rho \cdot w_2$$

**step 3.** the cross-term error $u$ captures the interaction:

$$u = \text{cross-term}(E_1, w_1, E_2, w_2, \rho)$$

the cross-term is a small polynomial — computing it requires ~30 field multiplications.

**step 4.** the accumulator stores $(E', w', u)$. size: ~200 bytes.

### 9.4 The accumulator

after folding N instances:

```
fold 1:   (E_1, w_1) + (E_2, w_2) → acc_1           cost: ~30 field ops
fold 2:   acc_1 + (E_3, w_3) → acc_2                 cost: ~30 field ops
...
fold N-1: acc_{N-2} + (E_N, w_N) → acc_{N-1}         cost: ~30 field ops

total: (N-1) × 30 ≈ 30N field operations
```

the accumulator acc_{N-1} is ONE CCS instance. verify it ONCE:

```
decide(acc_{N-1}):
  run SuperSpartan + sumcheck + Brakedown on the folded instance
  cost: ~12K constraints (one proof)
  proves: ALL N original instances were satisfied
```

### 9.5 The composition numbers

```
                        recursive verification     HyperNova folding
per-step cost:          ~12K constraints            ~30 field ops
block (1000 tx):        12M constraints             30K field ops + 12K decide
epoch (1000 blocks):    12M × 1000 = 12B            30K + 12K = ~42K
improvement:            —                           ~285,000×

(with WHIR instead of Brakedown: per-step = ~70K constraints, improvement = 2,300×)
```

### 9.6 Cross-algebra folding

the CCS can include selectors for different algebras:

```
universal_ccs = {
  sel_Fp:   activates Goldilocks constraint rows
  sel_F2:   activates binary constraint rows
}
```

a Goldilocks and a binary instance fold into the SAME accumulator. the decider handles both. boundary cost: ~766 F_p constraints per algebra crossing.

## 10. Proof-Carrying Computation

### 10.1 Fold during execution

instead of: execute → generate trace → prove (three separate phases),
do: execute AND fold simultaneously (one phase).

```
reduce_with_proof(subject, formula, focus, accumulator):
  let (result, trace_row) = dispatch(subject, formula)
  let accumulator' = fold(accumulator, trace_row)
  return (result, focus - 1, accumulator')
```

each reduce() call:
1. executes the pattern (application logic)
2. generates one trace row (the witness)
3. folds the row into the accumulator (~30 field operations)

at computation end: the accumulator IS the proof. run decide() to produce the final verifiable proof.

### 10.2 Cost

```
computation with N steps:

current pipeline:
  execute:  N reduce() calls                         N × cost(pattern)
  trace:    N rows generated                         N × 16 field elements
  commit:   Brakedown encode                         O(N) field ops
  prove:    SuperSpartan sumcheck                    O(17N) field ops
  total:    ~18N field operations + execution

proof-carrying:
  execute + fold:  N reduce() calls + N folds        N × (cost(pattern) + 30)
  decide:          1 × SuperSpartan                  ~12K constraints
  total:           ~N × 30 additional field ops + 12K

overhead per step: 30 field operations (~2 μs on commodity hardware)
proving latency: ZERO (proof ready when computation finishes)
```

### 10.3 Folded sponge

hemera hash operations during execution (content addressing, authentication) also fold:

```
hemera absorption of K blocks:
  current:  K independent permutations × 736 constraints = 736K
  folded:   K folds × 30 field ops + 1 decide = 30K + 736

  for K = 10 (2.5 KiB input):  7,360 → 1,036 constraints (7× savings)
  for K = 74 (4 KiB particle): 54,464 → 2,956 constraints (18× savings)
```

the hemera sponge's sequential structure (output of block N → input of block N+1) maps perfectly to HyperNova folding.

## 11. The Universal Accumulator

### 11.1 Five layers, one fold

distributed knowledge systems require five independent guarantees (structural sync):

| layer | mechanism | what gets folded |
|---|---|---|
| 1. validity | zheng proof per signal | signal proof → fold into accumulator |
| 2. ordering | hash chain + VDF | chain verification → fold |
| 3. completeness | polynomial commitment | state update proof → fold |
| 4. availability | algebraic DAS | erasure commitment → fold |
| 5. merge | CRDT / foculus | merge transition → fold |

every proof obligation from every layer folds into ONE accumulator via HyperNova.

### 11.2 The checkpoint

```
checkpoint = {
  BBG_commitment:   32 bytes    polynomial commitment over all state
  universal_acc:    ~200 bytes  accumulator proving all history
  height:           8 bytes     block height
}
total: ~240 bytes
```

a light client:
1. downloads 240 bytes
2. runs decide() — one SuperSpartan + sumcheck + Brakedown verification (10–50 μs)
3. gets: all signals valid, all ordered, nothing withheld, data available, merge correct
4. from genesis to now

## 12. Hemera: Trust Anchor, Not Prover

### 12.1 hemera-2 parameters

```
hash:            Poseidon2 over Goldilocks
S-box:           x⁻¹ (field inversion)
full rounds:     4 + 4 = 8
partial rounds:  16
state:           16 Goldilocks elements (128 bytes)
capacity:        8 elements (64 bytes)
rate:            8 elements (64 bytes)
output:          4 elements = 32 bytes
constraints:     ~736 per permutation
algebraic degree: 2^1046
```

### 12.2 Where hemera is used

```
role                    hemera calls        evolution
────                    ────────────        ─────────
content identity        1 per particle      always hemera (identity must persist)
private records         1 per UTXO          always hemera (privacy binding)
Fiat-Shamir seed        1 per proof         always hemera (transcript binding)
Fiat-Shamir challenges  19 per proof        hemera-3: algebraic (polynomial)
NMT tree hashing        144K per block      algebraic NMT: 0 (polynomial state)
DAS inclusion proofs    640 per verify      algebraic DAS: 0 (PCS openings)
Brakedown commitment    1 per proof         always hemera (binding hash)
```

hemera starts as dominant cost (>90% of constraints) and converges to trust anchor (~1% of constraints). Brakedown eliminates hemera from PCS. algebraic NMT eliminates hemera from state. algebraic DAS eliminates hemera from availability. what remains: content identity and privacy — the irreducible cryptographic anchors.

## 13. Polynomial State and Algebraic DAS

### 13.1 Why polynomial state is natural

sumcheck operates on multilinear polynomials. if the STATE is also a multilinear polynomial, then state reads are polynomial evaluations — the proof system's native operation.

```
state read (FRI architecture):
  walk Merkle tree → O(log N) hemera hashes → ~23,552 constraints (depth 32)

state read (zheng architecture):
  evaluate polynomial → O(1) field operations → ~100-200 constraints

ratio: ~100×
```

cross-index consistency (multiple indexes over the same data):

```
FRI:     separate LogUp proof per cross-reference (~500 constraints each)
zheng:   same polynomial, different evaluation dimensions → free (structural)
```

### 13.2 Algebraic DAS

DAS requires proving "this cell at position (row, col) is committed in the erasure-coded block."

```
FRI-based DAS:    NMT inclusion proof per sample → O(log N) hemera hashes
zheng-based DAS:  PCS opening per sample → O(1) field operations

20 samples:
  FRI:    ~25 KiB bandwidth, ~471K constraints
  zheng:  ~9 KiB bandwidth, ~3K constraints

ratio: 157×
```

the erasure-coded grid P(row, col) IS a bivariate polynomial. Brakedown commits to it with O(N) field operations. each DAS sample is one PCS opening.

## 14. The Landscape and Honest Assessment

### 14.1 Production systems (early 2026)

| system | field | PCS | proof | verify | prover | status |
|---|---|---|---|---|---|---|
| Stwo | M31 | FRI+Merkle | ~200 KiB | 10–50 ms | O(N log N) | production |
| Plonky3 | Goldilocks | FRI+Merkle | 100–200 KiB | 10–50 ms | O(N log N) | production |
| SP1 | BabyBear | FRI | ~150 KiB | ~30 ms | O(N log N) | production |
| RISC Zero | BabyBear | FRI+Merkle | ~200 KiB | ~50 ms | O(N log N) | production |
| Binius | F₂ tower | sumcheck | — | — | O(N) | research |
| **zheng** | **Gold + F₂** | **Brakedown + Binius** | **1–5 KiB** | **10–50 μs** | **O(N)** | **spec** |

all production systems share: FRI + univariate + Merkle. zheng: sumcheck + multilinear + Brakedown.

### 14.2 Honest weaknesses

| weakness | severity | reality |
|---|---|---|
| zero production hours | critical | biggest risk. algorithms are peer-reviewed. implementation is zero |
| hemera cost | medium | 736 vs ~300 constraints (2.4×). permanent 256-bit security tradeoff. Brakedown eliminates hemera from PCS |
| 64-bit field | medium | 2–4× slower per op vs BabyBear. compensated by fewer total ops (O(N) vs O(N log N)) and GFP hardware |
| Brakedown maturity | medium | less battle-tested than FRI. fewer years of cryptanalysis |
| single implementation | low-medium | formal spec enables independent implementations |

### 14.3 Structural advantages (asymptotic, not constant-factor)

```
state read:     O(1) vs O(log N)         architecture
prover time:    O(N) vs O(N log N)        sumcheck vs FFT
recursion:      30 ops vs 12K+ constraints folding vs verification
DAS verify:     O(1) vs O(log N)          PCS opening vs Merkle path
proof hemera:   1 call vs O(N log N)      Brakedown vs Merkle tree
```

these cannot be closed by optimizing FRI. they require changing the foundation.

## 15. Open Problems

1. **Brakedown expander construction.** which family (Margulis, Ramanujan, random) achieves optimal security/size for Goldilocks? concrete parameters for 128-bit security.

2. **Brakedown bivariate openings.** algebraic DAS needs efficient openings on 2D erasure grids. does Brakedown's linear code structure support bivariate evaluation natively?

3. **Cross-algebra soundness.** universal CCS with selectors folds F_p and F₂ instances. formal proof that this preserves HyperNova's security.

4. **Tensor rank of nox traces.** tensor compression assumes rank ≈ 32. empirical validation on real workloads needed.

5. **VEC formalization.** Verified Eventual Consistency (safety + verifiable completeness + verifiable availability + liveness) needs formal treatment under a precise adversary model.

6. **Brakedown + Binius unification.** both are linear-code-based PCS over different fields. can they share infrastructure?

7. **Row-by-row folding for AIR.** proof-carrying computation folds one trace row at a time. AIR transition constraints reference adjacent rows. sliding-window (width 2) folding needs formal correctness proof.

## References

[1] S. Setty, "SuperSpartan: Doubly-efficient SNARKs without preprocessing," Crypto 2023.
[2] A. Kothapalli and S. Setty, "HyperNova: Recursive arguments from folding schemes," 2023.
[3] G. Haböck et al., "WHIR: Reed-Solomon Proximity Testing with Super-Fast Verification," 2024.
[4] A. Golovnev et al., "Brakedown: Linear-time and field-agnostic SNARKs for R1CS," Crypto 2023.
[5] B. Diamond and J. Posen, "Succinct Arguments over Towers of Binary Fields," 2024.
[6] L. Grassi et al., "Poseidon2: A Faster Version of the Poseidon Hash Function," 2023.
[7] M. Shapiro et al., "Conflict-free Replicated Data Types," SSS 2011.
[8] M. Al-Bassam et al., "LazyLedger: Namespace Merkle Trees," 2019.
[9] M. Al-Bassam et al., "Fraud and Data Availability Proofs," 2018.
[10] S. Lavaur et al., "The Sum-Check Protocol over Fields of Small Characteristic," 2024.
