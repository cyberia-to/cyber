---
tags: cyber, cip
alias: zheng, 証, proof system, cyber stark, cyber starks, Whirlaway implementation
crystal-type: entity
crystal-domain: cyber
subgraph: true
repo: ../zheng
exclude: ".claude/**, target/**, CLAUDE.md"
stake: 38544821775428340
---
the proof system for [[cyber]]. implements the Whirlaway architecture: [[SuperSpartan]] IOP + [[WHIR]] PCS + [[sumcheck]] protocol. zero trusted setup. post-quantum. sub-millisecond verification.

zheng (証 — proof/evidence in Japanese) provides the cryptographic machinery that turns [[nox]] execution traces into compact, verifiable proofs. one commitment, one opening, one proof.

five primitives, one architecture:

```
COMPONENT         │ ROLE                          │ INSTANCE
──────────────────┼───────────────────────────────┼─────────────────────
hash              │ Fiat-Shamir, Merkle trees     │ Hemera (Poseidon2)
field             │ arithmetic substrate           │ Goldilocks (2⁶⁴ − 2³² + 1)
VM                │ execution trace generation     │ nox (16 patterns + hint + 5 jets)
IOP               │ constraint verification        │ SuperSpartan (CCS/AIR via sumcheck)
PCS               │ polynomial commitment          │ WHIR (multilinear)
```

architecture: Whirlaway = [[SuperSpartan]] IOP + [[WHIR]] PCS. LambdaClass (2025).

## dependency graph

```
nebu (field)
  ↓
hemera (hash)
  ↓
zheng (proofs) ← this repo
  ↓
bbg (state)
```

## the pipeline

```
1. EXECUTE
   nox program runs on subject with bounded focus
   → execution trace: 2ⁿ rows × 16 register columns
   each row = one reduction step
   each column = one register value over Goldilocks

2. ARITHMETIZE
   nox's 16 patterns → AIR transition constraints
   pattern 5 (add):  reg[out]_{t+1} = reg[a]_t + reg[b]_t                (degree 1)
   pattern 7 (mul):  reg[out]_{t+1} = reg[a]_t × reg[b]_t                (degree 2)
   pattern 4 (branch): selector_t × (next_t − yes_t) + (1 − selector_t) × (next_t − no_t) = 0  (degree 2)
   pattern 15 (hash): Poseidon2 round constraints across consecutive rows  (degree 7)

   boundary constraints pin inputs and outputs:
     reg[0] at row 0 = program_input
     reg[0] at last row = program_output

3. ENCODE
   entire trace (2ⁿ × 2⁴) → ONE multilinear polynomial f(x₁, ..., x_{n+4})
   row index: n boolean variables
   column index: 4 boolean variables
   every variable has degree ≤ 1

4. COMMIT
   WHIR_commit(f) = C
   Hemera Merkle tree over evaluations on the boolean hypercube
   commitment size: one Hemera digest (64 bytes)

5. PROVE CONSTRAINTS
   SuperSpartan sumcheck:
     claim: Σ_{x ∈ {0,1}^n} constraint_polynomial(f, x) = 0
     n + 4 rounds of interaction (Fiat-Shamir in practice)
     reduces to: evaluate f at ONE random point r ∈ F^{n+4}

6. OPEN
   WHIR_open(f, r) = (v, π)
   prover demonstrates f(r) = v with proximity proof
   proof: ~60–157 KiB depending on security level

7. VERIFY
   verifier checks:
     a) sumcheck transcript is consistent (field arithmetic only)
     b) WHIR_verify(C, r, v, π) accepts (hash operations only)
   total: O(log² N) operations. sub-millisecond.
```

## why multilinear

classical univariate starks interpolate each trace column as a separate polynomial, check constraints via zerofier division, prove via [[FRI]]. M columns → M commitments → M openings.

multilinear starks encode the entire trace as one polynomial. constraints checked via [[sumcheck]]. one commitment. one opening. the advantage:

```
                           │ univariate         │ multilinear (cyber)
───────────────────────────┼────────────────────┼─────────────────────
commitments                │ 16 (one/column)    │ 1
openings                   │ 16                 │ 1
prover (constraints)       │ O(N log N) per col │ O(N) total
prover tool                │ FFT/NTT            │ field ops only
trace representation       │ interpolation      │ boolean hypercube (natural)
constraint system          │ AIR only           │ CCS (AIR + R1CS + Plonkish)
```

the [[sumcheck]] protocol is the mechanism. it converts a sum over 2^k terms (all trace rows) into k rounds, each a low-degree univariate. the verifier's work per round: check one polynomial evaluation. total verifier work: O(k) field ops + one [[WHIR]] opening.

## AIR from nox

each of [[nox]]'s sixteen reduction patterns becomes an AIR transition constraint — a polynomial equation that must hold between consecutive trace rows.

```
PATTERN → CONSTRAINT                                        │ DEGREE │ CONSTRAINTS
────────────────────────────────────────────────────────────┼────────┼────────────
0  axis     navigation through subject tree                │ 1      │ ~depth
1  quote    output = literal constant                       │ 1      │ 1
2  compose  chain: eval x, eval y, eval result on x's output│ 1      │ 2
3  cons     pair two sub-results                            │ 1      │ 2
4  branch   selector × (next − yes) + (1−sel) × (next − no)│ 2      │ 2
5  add      out = a + b mod p                               │ 1      │ 1
6  sub      out = a − b mod p                               │ 1      │ 1
7  mul      out = a × b mod p                               │ 2      │ 1
8  inv      out × input = 1 mod p (Fermat verification)    │ 2      │ 1
9  eq       (a − b) × inv = (a ≠ b), out = 1 − (a ≠ b)    │ 2      │ 1
10 lt       range decomposition into bits                   │ 1      │ ~64
11 xor      bit decomposition + XOR per bit                 │ 2      │ ~64
12 and      bit decomposition + AND per bit                 │ 2      │ ~64
13 not      bitwise complement                              │ 1      │ ~64
14 shl      shift via multiplication by 2^n                 │ 2      │ ~64
15 hash     Poseidon2 round function across rows            │ 7      │ ~300
16 hint     constraint check (Layer 1 verification)         │ varies │ varies
```

[[SuperSpartan]] handles AIR constraints of any degree via [[CCS]]. high-degree constraints (pattern 15: degree 7) cost only field operations in the prover — no cryptographic cost increase over degree-1 constraints. this is the [[CCS]] advantage: the Poseidon2 rounds inside the hash pattern are free in the IOP layer.

## constraint budget

```
PROOF TYPE                       │ TOTAL CONSTRAINTS │ DOMINANT COST
─────────────────────────────────┼───────────────────┼──────────────────────
identity (preimage)              │ ~300              │ one Hemera hash
anonymous cyberlink              │ ~13,000           │ WHIR membership + SWBF
delivery (per hop)               │ ~60,000           │ decryption + forwarding
private transfer (BBG)           │ ~50,000           │ AOCL/SWBF verification
stark recursive verification     │ ~70,000 (jets)    │ Merkle + WHIR verify
stark recursive (no jets)        │ ~600,000          │ Merkle verification
```

constraint count determines prover time. at ~10⁶ constraints/second on commodity hardware: a 13K-constraint anonymous [[cyberlink]] proves in ~13 ms. a 70K-constraint recursive step proves in ~70 ms. the [[Goldilocks field processor]] targets 10× acceleration.

## Hemera as the stark hash

every hash operation inside a stark — Fiat-Shamir challenges, Merkle trees in WHIR, commitment randomness — uses [[Hemera]]. the choice of hash is the single largest factor in stark performance.

```
HASH                │ CONSTRAINTS PER CALL │ stark OVERHEAD
────────────────────┼──────────────────────┼────────────────
SHA-256             │ ~25,000              │ baseline
Keccak-256          │ ~150,000             │ 6× worse
Poseidon (original) │ ~4,000               │ 6× cheaper
Hemera (Poseidon2)  │ ~1,200               │ 20× cheaper
```

Hemera's ~1,200 constraints per hash means Merkle verification at depth 32 costs ~38,400 constraints instead of ~800,000 with SHA-256. this 20× reduction is what makes recursive stark composition practical at 70,000 total constraints.

the hash is also the field: Hemera operates natively on [[Goldilocks field]] elements. no bit-packing, no field conversion, no endianness gymnastics. eight elements in, eight elements out. the output is directly usable in polynomial commitments, constraint evaluations, and [[nox]] arithmetic.

## recursive composition

the stark verifier is a [[nox]] program. it can be proven by the same stark system. this self-referential property is the foundation of scalability in [[cyber]] — without it, verification cost grows linearly with transaction count.

### self-verification theorem

the stark verifier requires four operations. all are [[nox]]-native:

```
VERIFIER OPERATION           │ NOX PATTERNS USED          │ WHY IT WORKS
─────────────────────────────┼────────────────────────────┼──────────────────────
field arithmetic             │ 5 (add), 6 (sub), 7 (mul) │ Goldilocks is the native field
                             │ 8 (inv)                    │
hash computation             │ 15 (hash) / hash jet      │ Hemera IS the nox hash
sumcheck verification        │ 5, 7, 9 (field ops only)  │ sumcheck is pure arithmetic
WHIR opening verification    │ 15 + 4 (conditionals)     │ Merkle paths + polynomial eval
                             │ poly_eval, merkle_verify,  │
                             │ fri_fold jets              │
```

no external primitive enters the verification loop. the verifier is closed under the same instruction set that produced the original proof. consequence: `verify(proof)` can itself be proven, and `verify(verify(proof))` too, to arbitrary depth.

### verifier cost breakdown

```
COMPONENT               │ LAYER 1 ONLY │ WITH JETS  │ REDUCTION │ WHAT IT DOES
────────────────────────┼──────────────┼────────────┼───────────┼──────────────────────
parse proof             │     ~1,000   │    ~1,000  │  1×       │ deserialize proof bytes
Fiat-Shamir challenges  │    ~30,000   │    ~5,000  │  6×       │ hash transcript → random challenges
Merkle verification     │   ~500,000   │   ~50,000  │ 10×       │ verify WHIR commitment tree paths
constraint evaluation   │    ~10,000   │    ~3,000  │  3×       │ evaluate AIR polynomials at challenge
WHIR verification       │    ~50,000   │   ~10,000  │  5×       │ FRI folding rounds + final check
────────────────────────┼──────────────┼────────────┼───────────┼──────────────────────
TOTAL                   │   ~600,000   │   ~70,000  │  8.5×     │
```

Merkle verification dominates without jets (83% of cost). the merkle_verify jet reduces it 10×. this single jet is what makes recursion practical — without it, each recursion level would cost 600K constraints, limiting practical depth to 1-2 levels.

### recursion mechanics

```
Level 0: prove computation C → proof π₀        (constraint count = |C|)
Level 1: prove verify(π₀)  → proof π₁          (~70K constraints, ~70 ms)
Level 2: prove verify(π₁)  → proof π₂          (~70K constraints, ~70 ms)
  ...
Level k: proof π_k — same size regardless of k  (~60-157 KiB)
```

each level costs exactly ~70,000 constraints with jets, regardless of what the original computation was. a proof of a million-constraint neural network inference, once recursed, becomes a 70K-constraint verification problem — identical in cost to verifying a 300-constraint identity proof.

proof size remains constant across levels: ~60-157 KiB. the verifier at each level reads one proof and produces one proof. no growth, no degradation.

### aggregation patterns

cyber uses three aggregation patterns depending on the workload:

```
PATTERN 1: TREE AGGREGATION (block proofs)
──────────────────────────────────────────
N transactions in a block, each with proof πᵢ

  π₁  π₂  π₃  π₄  π₅  π₆  π₇  π₈       N leaf proofs
   \  /    \  /    \  /    \  /
   π₁₂    π₃₄    π₅₆    π₇₈             N/2 pair verifications
     \    /        \    /
     π₁₋₄         π₅₋₈                  N/4 pair verifications
        \        /
        π_block                           1 block proof

depth: log₂(N)
total prover work: O(N) — each level halves the count
latency: O(log N) sequential steps (parallelizable within levels)
result: one proof per block, O(1) on-chain verification


PATTERN 2: SEQUENTIAL FOLDING (epoch proofs)
────────────────────────────────────────────
blocks B₁, B₂, ..., B_E across an epoch

  acc₀ = initial
  acc₁ = fold(acc₀, π_B₁)    one field operation + one hash
  acc₂ = fold(acc₁, π_B₂)    one field operation + one hash
  ...
  acc_E = fold(acc_{E-1}, π_B_E)
  π_epoch = decider(acc_E)    one stark proof at the end

cost per fold: ~O(1) field ops (no full verification at each step)
final decider: ~70K constraints (one stark)
result: one proof covers an entire epoch of blocks


PATTERN 3: DAG MERGING (cross-shard, multi-agent)
─────────────────────────────────────────────────
shard A proves subgraph → π_A
shard B proves subgraph → π_B
boundary node merges: π_AB = PCD_merge(π_A, π_B)

applicable to:
  - cross-shard cybergraph queries spanning multiple shards
  - multi-validator proving where different validators prove different block parts
  - delivery proof chains where each relay proves its hop independently
```

### what recursion enables in cyber

| use case | without recursion | with recursion |
|---|---|---|
| block verification | verify every transaction proof individually: O(N) | verify one block proof: O(1) |
| light client sync | download and verify all block proofs since last sync | verify one epoch proof covering thousands of blocks |
| [[cyberlink]] throughput | on-chain verification bottleneck limits TPS | off-chain proving, on-chain O(1) verification |
| delivery proofs | each hop proof verified separately: O(hops) | one chained proof covers the entire route: O(1) |
| cross-epoch queries | re-verify from genesis or trust a checkpoint | one proof covers any historical range |
| [[trident]] inference | verify full neural network execution on-chain | verify one recursive proof of inference: O(1) |

the chain sees one proof per block. the proof covers every cyberlink, every transfer, every state transition within that block. validators produce the proof. light clients verify it in sub-millisecond time. this is the scaling mechanism — computation happens off-chain, integrity travels on-chain as a single proof.

## IVC and folding

[[incrementally verifiable computation]] chains proofs sequentially: each step absorbs the previous proof via [[folding]] into an [[accumulator]]. the accumulated proof at step i guarantees all steps 1..i are valid. the key insight: folding replaces full proof verification at each step with a single field operation, deferring the expensive check to one final decider proof.

```
FULL RECURSION (expensive):
  step i: run stark_verify(π_{i-1}) inside nox → prove it → π_i
  cost per step: ~70,000 constraints

FOLDING (cheap):
  step i: fold(accumulator, instance_i) → accumulator'
  cost per step: ~O(1) field operations + one hash
  final step: stark_prove(decider(accumulator)) → π_final
  cost once: ~70,000 constraints

savings: (N-1) × 70,000 constraints eliminated
```

[[proof-carrying data]] generalizes IVC from linear chains to DAGs. in [[cyber]], the [[cybergraph]] is a DAG — PCD matches this topology. different [[validators]] prove different subgraphs, then merge proofs at shard boundaries. the merge operation itself is a fold: absorb two accumulators into one.

[[HyperNova]] folding over [[CCS]] is the natural fit: [[CCS]] already powers [[SuperSpartan]], so the folding scheme and the stark system share the same constraint language. fold a [[cyberlink]] insertion proof? same [[CCS]] instance type. fold a rank update? same [[CCS]]. fold a cross-shard merge? same [[CCS]]. one framework for every proof in the taxonomy.

### folding in practice

```
CYBERGRAPH STATE TRANSITIONS
────────────────────────────
epoch starts with state commitment S₀

  block 1: insert 1000 cyberlinks
    fold each insertion into accumulator
    acc₁ = fold(fold(...fold(acc₀, link₁)..., link₁₀₀₀))
    cost: 1000 field ops + 1000 hashes ≈ microseconds

  block 2: insert 800 cyberlinks
    acc₂ = fold(acc₁, block₂_insertions)

  ...

  block E: last block of epoch
    acc_E = fold(acc_{E-1}, block_E_insertions)
    π_epoch = stark(decider(acc_E))    ← one proof, ~70K constraints

S₁ = apply(S₀, π_epoch)  — state transition is one proof verification
```

the epoch proof guarantees: every cyberlink insertion was valid (correct neuron, sufficient stake, fresh nullifier), every state update was consistent (index updates, focus recomputation), and conservation laws held across every transaction. one proof. one verification. all of it.

## integration with BBG

the [[BBG]] uses [[WHIR]]-based polynomial commitments for all indexes. the same WHIR instance that serves as the stark PCS also handles:

```
OPERATION              │ MECHANISM                        │ CONSTRAINTS
───────────────────────┼──────────────────────────────────┼────────────
EdgeSet membership     │ WHIR evaluation proof            │ ~1,000
namespace completeness │ sorted range bounds + WHIR opens │ ~10,000
cross-index consistency│ LogUp via sumcheck               │ ~5,000
focus commitment       │ polynomial over (neuron, π)      │ ~1,000
balance commitment     │ polynomial over (neuron, balance)│ ~1,000
```

[[LogUp]] lookup arguments use the [[sumcheck]] protocol — the same sumcheck that powers [[SuperSpartan]]. cross-index consistency (every edge appearing in neuron index, source index, and target index) reduces to a sumcheck over logarithmic multiplicities. one protocol, two uses.

## security

```
ASSUMPTION              │ WHAT BREAKS IF VIOLATED
────────────────────────┼─────────────────────────────────────
Hemera collision resistance │ forge commitments, fake Merkle proofs
Goldilocks field hardness   │ extract witnesses from proofs
sumcheck soundness          │ false constraint satisfaction
WHIR proximity              │ evaluate committed polynomial incorrectly
Fiat-Shamir (random oracle) │ predictable challenges → forge proofs
```

all assumptions reduce to: collision resistance of [[Hemera]]. no discrete log, no pairings, no trusted setup. post-quantum: a quantum adversary with Grover's algorithm squares the brute-force effort on Hemera from 2^128 to 2^64 — but Hemera's 256-bit output provides 128-bit post-quantum security margin.

see [[stark]] for the general theory, [[cyber/proofs]] for the full proof taxonomy, [[nox]] for the VM specification, [[WHIR]] for the PCS, [[SuperSpartan]] for the IOP, [[sumcheck]] for the core protocol, [[Hemera]] for the hash, [[Goldilocks field]] for the arithmetic, [[BBG]] for the graph structure
