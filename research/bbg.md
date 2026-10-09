---
title: BBG polynomial authenticated state
alias: bbg research, bbg polynomial state
tags: cyber, research, article
crystal-type: process
crystal-domain: cyber
status: draft
date: 2026-03-24
---
> superseded: this page describes the retired zheng design (Merkle-free Brakedown, unchecked HyperNova fold, ~2 KiB / ~5 μs figures). It is unsound and its numbers do not hold. The repair and the measured numbers live in [[soft3/proposals/proof-system-repair|soft3 proposals/proof-system-repair]].

# BBG: polynomial authenticated state

## abstract

[[BBG]] (Big Badass Graph) is the authenticated state layer for [[cyber]]. the entire [[cybergraph]] — [[particles]], [[axons]], [[neurons]], [[tokens]], temporal state, private records — commits to a single polynomial:

$$\text{BBG\_root} = \text{Lens.commit}(\text{BBG\_poly})$$

32 bytes. one polynomial. all state. every query is a polynomial opening. cross-index consistency is structural — different evaluation dimensions of the same polynomial cannot disagree. [[LogUp]] is unnecessary. a checkpoint (BBG_root + one decider proof of the [[universal accumulator]] + height) proves all history from genesis (goal ≤ 64 KB, verify ≤ 1 ms; measured TODO(F-numbers)).

the architecture follows from [[zheng]]'s [[sumcheck]] foundation: the proof system operates on multilinear polynomials natively. making the STATE a polynomial means state reads ARE the proof system's native operation. this is not optimisation — it is alignment between state and proof architecture.

## 1. three laws

**law 1: bounded locality.** operation cost $\propto$ what it touches, not total state size. at $10^{15}$ particles, a single [[cyberlink]] costs $O(\log n)$ polynomial path updates. global recomputation is physically impossible and architecturally forbidden.

**law 2: constant-cost verification.** verifying any claim about the graph costs $O(1)$: one PCS opening (succinct profile; goal ≤ 64 KB, verify ≤ 1 ms). independent of graph size, history length, or computation complexity. a light client holding one checkpoint has the same verification power as a full node.

**law 3: structural security.** guarantees come from mathematical structure, not protocol correctness. polynomial binding prevents lying — a committed polynomial evaluates to a unique value at each point. the lens PCS (WHIR over Reed–Solomon, committed with hemera Merkle trees) is post-quantum (hash-only, no pairings). privacy comes from the [[mutator set]] (SWBF bitmap prevents double-spend by construction).

## 2. five primitives

| primitive | identity | role |
|---|---|---|
| [[particles\|particle]] | $H(\text{content})$ — 32 bytes | content-addressed node, atom of knowledge |
| [[cyberlinks\|cyberlink]] | $H(p, q, \tau, a, v)$ — 5-tuple | private authenticated edge |
| [[neurons\|neuron]] | $H(\text{public\_key})$ | agent with stake and [[focus]] budget |
| [[token]] | denomination hash | economic value (coin, card, score, badge) |
| [[focus]] | $\phi^*$ from [[tri-kernel]] | emergent attention distribution |

derived: **axon** = $H(\text{from}, \text{to})$. aggregate of all cyberlinks between two particles. the axon is public; individual cyberlinks are private.

## 3. the polynomial state

### BBG_poly

all state encodes as evaluations of a single multivariate polynomial:

$$\text{BBG\_poly}(\text{index}, \text{key}, t) = \text{value}$$

three dimensions:
- **index** $\in \{0..9\}$ — which data domain (particles, axons_out, axons_in, neurons, locations, coins, cards, files, time, signals)
- **key** $\in \mathbb{F}_p$ — namespace key (particle CID, neuron ID, denomination hash, etc.)
- **$t$** $\in \mathbb{N}$ — block height (temporal dimension)

committed via the [[lens]] PCS (WHIR over Reed–Solomon, hemera Merkle root):

$$\text{BBG\_root} = \text{Lens.commit}(\text{BBG\_poly}) \quad \text{(32 bytes)}$$

### what each dimension encodes

| index | domain | key | value |
|---|---|---|---|
| 0: particles | content-addressed nodes | CID | energy, $\phi^*$, axon fields |
| 1: axons_out | outgoing edges by source | source CID | axon pointer, weight, market state |
| 2: axons_in | incoming edges by target | target CID | axon pointer, weight |
| 3: neurons | agent state | neuron ID | focus, karma, stake |
| 4: locations | spatial association | neuron ID | geohash, attestation |
| 5: coins | fungible tokens | denomination | supply, parameters |
| 6: cards | non-fungible assets | card ID | owner, content CID, metadata |
| 7: files | content availability | CID | DAS commitment, chunk count |
| 8: time | historical snapshots | time namespace | BBG_root at that time |
| 9: signals | finalized signal batches | step | signal hash |

### why one polynomial

nine independent data structures (the old NMT approach) force redundant computation:

```
old (NMTs):    cyberlink touches 4-5 trees → 4.5 × O(log n) hemera hashes
               cross-index: LogUp proves trees agree (~1,500 constraints)
               total: ~107,500 constraints per cyberlink

new (one poly): cyberlink updates polynomial at 4-5 evaluation points
               cross-index: STRUCTURAL (same polynomial, different dimensions)
               total: ~3,200 constraints per cyberlink
```

the polynomial makes cross-index consistency FREE. axons_out and axons_in are different evaluation dimensions of BBG_poly. they CANNOT disagree because they are the same committed object. [[LogUp]] — which cost ~6M constraints per block — is eliminated entirely.

### state reads

a state read IS a polynomial evaluation:

```
"what is the energy of particle P?"
= Lens.open(BBG_root, (particles, P, t_now))
= one PCS opening (succinct profile; goal ≤ 64 KB, verify ≤ 1 ms)

"all outgoing axons from particle P?"
= Lens.open(BBG_root, (axons_out, P, t_now))
= one PCS opening, completeness guaranteed by Lens binding
```

compare with the hash-tree approach: $O(\log n) \times 32$ bytes Merkle path, $O(\log n)$ [[Hemera]] hashes to verify. the polynomial approach gives one opening per query regardless of index structure. the PCS still verifies hemera Merkle paths inside the opening — a lower bound for a hash-only system — but no per-index tree is maintained.

### state updates

a [[cyberlink]] updates the polynomial at multiple evaluation points:

```
cyberlink (p, q, τ, a, v):         // ν and t come from the containing signal
  BBG_poly(particles, p, t)    ← energy update for source
  BBG_poly(particles, q, t)    ← energy update for target
  BBG_poly(axons_out, p, t)    ← outgoing axon update
  BBG_poly(axons_in, q, t)     ← incoming axon update
  BBG_poly(neurons, ν, t)      ← focus deduction

each update: O(log n) polynomial path operations × ~100 field ops
total: ~3,200 constraints per cyberlink
```

with the lens PCS, the update cost is O(N) for batch recommit at block boundary. the commitment is a hemera Merkle tree over the Reed–Solomon codeword, so hemera hashing remains (a lower bound for hash-only); what disappears is the per-index tree maintenance of the NMT approach.

## 4. private state

individual [[cyberlinks]] are private. the polynomial state handles this:

**commitment polynomial** $A(x)$: all committed private records. $A(c_i) = v_i$ for commitment $c_i$ with value $v_i$. membership proof: one Lens opening — O(1).

**nullifier polynomial** $N(x) = \prod(x - n_i)$: all spent nullifiers. $N(n) = 0$ iff nullifier $n$ is spent. non-membership proof: one Lens opening showing $N(c) \neq 0$ — O(1).

```
old (SWBF + MMR):
  membership:      O(log N) hemera hashes (AOCL MMR)
  non-membership:  128 KB witness (SWBF bitmap) + O(log N) MMR walk
  update:          bitmap flip + periodic archive
  total:           ~40,000 constraints per spend

new (polynomial):
  membership:      one Lens opening — O(1)
  non-membership:  one Lens opening — O(1)
  update:          N'(x) = N(x) × (x - n_new) — O(1) polynomial extend
  witness:         32 bytes (Lens commitment, was 128 KB)
  total:           ~5,000 constraints per spend
```

privacy is preserved: Lens opening proofs are zero-knowledge. opening $A(c_i)$ reveals nothing about other commitments. opening $N(n)$ reveals nothing about other nullifiers.

## 5. temporal state

the temporal dimension $t$ in BBG_poly enables continuous-time queries:

```
"what was φ* of particle P at block 1000?"
= Lens.open(BBG_root, (particles, P, 1000))
= one Lens opening — no separate time index needed
```

the old approach used a time.root NMT with 7 namespaces (steps, seconds, hours, days, weeks, moons, years). the polynomial absorbs time as a native dimension — any historical query is one evaluation.

with [[gravity commitment]]: recent + high-$\phi^*$ queries are cheapest (low-degree polynomial terms). old + low-$\phi^*$ queries cost more (high-degree terms). verification cost follows the [[universal law|exponential]] — important facts are cheaper to verify.

## 6. algebraic DAS

[[DAS|Data Availability Sampling]] uses the same polynomial infrastructure. the erasure-coded block is a bivariate polynomial $P(\text{row}, \text{col})$. each DAS sample is one Lens opening:

```
sample: Lens.open(block_commitment, (row_i, col_i)) → value + proof

old (NMT-based DAS):
  per sample:  O(log n) × 32 bytes NMT path, O(log n) hemera hashes
  20 samples:  ~25 KiB bandwidth, ~471K constraints

algebraic DAS:
  per sample:  one Lens opening (succinct profile; goal ≤ 64 KB)
  20 samples:  one batched opening; measured size TODO(F-numbers)

improvement: to be measured — the earlier 157× / 6× figures assumed a ~0.2 KB opening without Merkle paths that does not exist
```

the same lens serves state queries AND availability sampling. one commitment scheme for everything.

## 7. signal-first architecture

BBG_poly is DERIVED DATA. the source of truth is the [[signal]] log:

$$\text{BBG\_poly}(t) = \text{fold}(\text{genesis\_poly}, \sigma[0..t])$$

each signal updates the polynomial at specific evaluation points. the fold is deterministic. any node can reconstruct BBG_poly at any height by replaying signals.

consequences:
- crash recovery: download checkpoint (state root + one decider proof) + replay signals since checkpoint
- storage proofs: prove signal availability ([[DAS]]), derive everything else
- the irreducible minimum per node: signal log + latest checkpoint
- BBG_poly is a materialised view, not primary data

see [[signal-first]] for the full design.

## 8. sync

one mechanism at three scales. five verification layers ([[structural-sync]]):

| layer | mechanism | what it costs |
|---|---|---|
| 1. validity | [[zheng]] proof per [[signal]] | verify goal ≤ 1 ms (succinct profile measured 7.96 ms at hash.tri) |
| 2. ordering | hash chain + [[VDF]] | O(1) per signal |
| 3. completeness | Lens opening (polynomial completeness) | one PCS opening per namespace |
| 4. availability | algebraic [[DAS]] (Lens samples) | one batched opening for 20 samples |
| 5. merge | [[CRDT]] (local) / [[foculus]] (global) | deterministic convergence |

a light client joins:

```
1. download checkpoint                    state root + one decider proof (goal ≤ 64 KB, measured TODO(F-numbers))
2. verify (one zheng decider)             goal ≤ 1 ms
3. sync namespaces (Lens openings)         one PCS opening each
4. DAS sample (algebraic)                 one batched opening
5. maintain (accumulate each block)       one accumulation step / block

total: a few openings of ≤ 64 KB each (goal), ZERO trust
```

this is [[structural-sync|Verified Eventual Consistency]] (VEC): convergence guaranteed (CRDT), completeness verifiable (lens), availability verifiable (DAS). no consensus protocol needed.

## 9. φ*-weighted everything

$\phi^*$ ([[cyberank]] from [[tri-kernel]]) is the master distribution. the entire stack follows it:

| what | how it follows φ* |
|---|---|
| verification cost | [[gravity commitment]]: high-$\phi^*$ particles verify cheaper |
| storage replication | [[pi-weighted-replication]]: replicas $\propto \phi^*$ |
| DAS parameters | high-$\phi^*$: fewer samples needed (more replicas = higher base availability) |
| temporal decay | low-$\phi^*$ links decay faster (nobody reinforces them) |
| query routing | hot queries (high-$\phi^*$) served from low-degree polynomial (fast) |

one distribution governs proof cost, storage, availability, decay, and query performance. the [[universal law]] predicts this: given finite resources, exponential allocation minimises total cost.

## 10. the numbers

| metric | value |
|---|---|
| BBG_root | 32 bytes (one Lens commitment) |
| checkpoint | root + one decider proof + height (goal ≤ 64 KB, measured TODO(F-numbers)) |
| checkpoint verification | one zheng decider (goal ≤ 1 ms, measured TODO(F-numbers)) |
| per-cyberlink | ~3,200 constraints (public) + ~5,000 (private) = ~8,200 total |
| per-block (1000 tx) | ~8.3M constraints |
| epoch (1000 blocks) | one accumulation step per block (ARC-style accumulation, constant accumulator) |
| inclusion proof | one Lens opening (succinct profile; goal ≤ 64 KB) |
| non-membership | one Lens opening (was 128 KB SWBF witness) |
| DAS (20 samples) | one batched opening (measured TODO(F-numbers)) |
| hemera calls/block (state) | Merkle hashing inside the PCS only (no per-index trees) |
| light client join | checkpoint + a few openings (goal ≤ 64 KB each) |
| cross-index consistency | 0 constraints (structural — same polynomial) |

cost of one cyberlink in the permanent, verified, globally-available knowledge graph:

```
proof:           one accumulation step per nox step (proof-carrying)
identity:        ~164 constraints (folded hemera sponge)
public state:    ~3,200 constraints (polynomial update)
private state:   ~5,000 constraints (polynomial mutator set)
total overhead:  ~8,400 constraints
```

## 11. state transitions

six transaction types modify BBG_poly:

| transaction | what it does | constraints |
|---|---|---|
| CYBERLINK | update public aggregates + create private record | ~8,200 |
| PRIVATE TRANSFER | move value between private records | ~10,000 |
| COMPUTATION | execute [[nox]] program, deduct focus | varies |
| MINT CARD | create non-fungible knowledge asset | ~5,000 |
| TRANSFER CARD | change card ownership | ~3,000 |
| BRIDGE | convert coin to focus | ~3,000 |

every transaction produces a [[zheng]] proof via [[proof-carrying computation|proof-carrying]]. every proof is accumulated into the block accumulator by ARC-style accumulation of Reed–Solomon evaluation claims (one accumulation step per proof; zheng phase 3).

## 12. privacy model

```
PRIVATE (polynomial commitments):          PUBLIC (BBG_poly dimensions):
  who linked what (individual cyberlinks)    axon weights (aggregate conviction)
  individual conviction amounts               particle energy, φ*
  neuron linking history                      neuron summaries (focus, karma, stake)
  market positions                            token supplies
  UTXO values and owners                     axon market state
```

anonymous [[cyberlinks]]: a [[neurons|neuron]] proves identity ($H(\text{secret}) \in$ neuron set), stake sufficiency, nullifier freshness — without revealing which neuron. ~13,000 constraint [[zheng]] proof. the graph sees edges and weights. not authors.

## 13. honest assessment

| claim | confidence | basis |
|---|---|---|
| three laws | high | architectural properties |
| one polynomial for all state | medium-high | multivariate lens well-understood, scale unproven |
| polynomial mutator set | medium | novel, needs implementation |
| ~3,200 constraints/cyberlink | high | follows from sumcheck + lens PCS architecture |
| algebraic DAS (157×) | high | follows from polynomial completeness |
| signal-first reconstruction | high | deterministic fold |
| constant-size checkpoint | medium | ARC-style accumulation lands in zheng phase 3; goal ≤ 64 KB, measured TODO(F-numbers) |
| zero implementation | critical | specification only, no code |

the dependency chain: [[nebu]] → [[Hemera]] → [[nox]] → [[zheng]] → BBG. nothing runs until the stack beneath it runs.

see [[structural-sync]] for the sync theory, [[cyber/research/zheng vs starks|zheng]] for the proof system, [[cyber/research/nox|nox]] for the VM, [[Hemera]] for the hash, [[tri-kernel architecture]] for focus, [[knowledge capacity]] for limits, [[link production]] for the intelligence problem, [[algebraic state commitments]] for why polynomial state is natural
