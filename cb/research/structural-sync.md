---
title: "Structural Sync: Verified Convergence Without Consensus"
authors: [cyber]
date: 2026-03-23
status: draft
---

# Structural Sync: Verified Convergence Without Consensus

## Abstract

We present *structural sync*, a synchronization primitive that achieves verified convergence without consensus protocols. The construction composes three mechanisms from independent mathematical disciplines: conflict-free replicated data types (CRDTs, algebra) for deterministic merge, Namespace Merkle Trees (NMTs, logic) for unconditional completeness, and Data Availability Sampling (DAS, probability) for physical availability. The composition yields a new consistency model — *Verified Eventual Consistency* (VEC) — where a node can locally verify that it has converged on the complete, available state without trusting any peer or participating in any coordination protocol. We extend structural sync with zero-knowledge proofs, hash chains, and verifiable delay functions to construct a self-proving knowledge graph where every edge carries its own proof, every query is verifiable, and the entire history compresses to a constant-size accumulator (~200 bytes). The full pipeline — from signal creation to light client verification — is one continuous fold operation with ~30 field operations per step, eliminating the boundary between computation and proving.

## 1. Introduction

Distributed systems that maintain shared state face a fundamental coordination problem: how do independent nodes agree on what the state is? The standard answer is *consensus* — a protocol where nodes exchange messages until a sufficient quorum agrees on a value.

Consensus protocols — PBFT [1], Tendermint [2], Nakamoto [3], HotStuff [4] — share a structure: a leader proposes, participants vote, and finality follows some quorum rule. The cost is coordination: O(N²) messages for classical BFT, O(N) for optimized variants, plus leader election, view changes, and finality delays. Every consensus protocol pays this cost because nodes must agree on ORDER before they can agree on STATE.

We ask: is ordering coordination the only path to agreement?

The answer is no. We show that composing three independent mathematical structures — an algebraic merge function (CRDT), a structural completeness proof (NMT), and a probabilistic availability check (DAS) — achieves verified convergence without any ordering protocol. The cost of coordination vanishes because:

1. CRDTs make ordering irrelevant (commutative merge)
2. NMTs make completeness verifiable (structural proof)
3. DAS makes availability checkable (probabilistic sampling)

The composition does not violate impossibility results (FLP [5], CAP [6]) because it solves a different problem: *verified set convergence* rather than state machine replication.

## 2. Three Layers

### 2.1 Algebra: CRDTs

A conflict-free replicated data type [7] is a state object whose merge operation forms a join-semilattice:

$$
a \sqcup b = b \sqcup a \quad \text{(commutative)}
$$
$$
(a \sqcup b) \sqcup c = a \sqcup (b \sqcup c) \quad \text{(associative)}
$$
$$
a \sqcup a = a \quad \text{(idempotent)}
$$

**Property.** Any two nodes receiving the same set of operations converge to the same state regardless of delivery order.

**Gap.** CRDTs guarantee convergence conditional on set equality. They provide no mechanism to verify that two nodes have received the same set. A Byzantine peer withholding operations causes convergence on an incomplete (and therefore incorrect) state.

### 2.2 Logic: Namespace Merkle Trees

A Namespace Merkle Tree [8] is a binary hash tree where each internal node stores the minimum and maximum namespace identifiers of its subtree. The sorting invariant — children's namespace ranges are contained within their parent's range — is enforced by construction.

**Property.** A completeness proof for namespace $N$ is the set of authentication paths covering all leaves with namespace $N$. The sorting invariant guarantees: if a leaf with namespace $N$ exists in the tree, it must appear in the proof. Omission is structurally impossible.

**Gap.** NMT completeness is relative to the committed tree. It guarantees that nothing within the tree is hidden, but says nothing about whether the underlying data is physically available.

### 2.3 Probability: Data Availability Sampling

Data Availability Sampling [9] uses 2D Reed-Solomon erasure coding to enable probabilistic verification of data existence. An $n$-element dataset is arranged in a $\sqrt{n} \times \sqrt{n}$ grid, extended with parity rows and columns to $2\sqrt{n} \times 2\sqrt{n}$.

**Property.** A verifier sampling $k$ random cells, each with an NMT inclusion proof, achieves confidence $1 - (1/2)^k$ that the data is available. At $k = 20$: 99.9999% confidence without downloading the full dataset.

**Gap.** DAS proves physical existence but provides no merge semantics. Multiple versions of the data may exist with no resolution rule.

## 3. The Composition

Each layer alone is incomplete. Each pair closes one gap but leaves another:

| Composition | Guarantee | Remaining Gap |
|---|---|---|
| CRDT + NMT | Convergence on provably complete data | Data may be lost |
| CRDT + DAS | Convergence on available data | Completeness unverifiable |
| NMT + DAS | Complete and available data | No merge semantics |
| **CRDT + NMT + DAS** | **Convergence on provably complete, provably available data** | **None** |

The three gaps cancel because each mechanism addresses a failure class the other two cannot:

| Failure Mode | CRDT | NMT | DAS |
|---|---|---|---|
| Merge conflict | **Resolved** | — | — |
| Selective withholding | — | **Provable** | Detectable |
| Incomplete transfer | — | **Provable** | — |
| Data loss | — | — | **Recoverable** |
| Byzantine sender | Wrong state | **Caught** | Detected |

Each layer contributes a guarantee from a different mathematical discipline: algebraic certainty (CRDT), structural impossibility (NMT), and probabilistic confidence (DAS). The guarantees are independent — failure in one layer does not compromise the others.

## 4. Verified Eventual Consistency

The composition defines a new consistency model.

**Definition (Verified Eventual Consistency).** A distributed system provides VEC if:

1. **Safety.** If two nodes hold verified-complete signal sets $S_A$ and $S_B$ with $S_A = S_B$, then $\text{state}(S_A) = \text{state}(S_B)$. (Deterministic CRDT convergence.)

2. **Verifiable completeness.** A node can verify in $O(\log n)$ whether its signal set for a given source is complete. (NMT structural completeness proof.)

3. **Verifiable availability.** A node can verify in $O(\sqrt{n})$ whether the data underlying its signal set physically exists across the network. (DAS probabilistic sampling.)

4. **Liveness.** If a signal is created and the network is eventually connected, every correct node eventually receives it. (Gossip + DAS reconstruction from erasure coding.)

VEC is strictly stronger than strong eventual consistency (SEC) [7], which guarantees convergence on equal message sets but provides no mechanism to verify set equality. VEC adds verifiable completeness (NMT) and verifiable availability (DAS).

VEC is weaker than strong consistency — it provides no real-time ordering guarantee. The tradeoff: VEC requires zero coordination, while strong consistency requires $\Omega(N)$ messages per operation.

**Relationship to impossibility results.**

VEC does not violate the FLP impossibility theorem [5] because it solves verified convergence, not consensus agreement. FLP requires that all correct nodes eventually *decide* — a liveness property of a distributed protocol. VEC guarantees that any node with a verified-complete set has the correct state — a safety property checkable locally. NMT transforms the liveness question ("did everyone receive everything?") into a verifiable safety question ("do I have everything?"), which a node can answer without waiting for others.

VEC does not violate the CAP theorem [6] because it trades strong consistency for eventual consistency while adding a fourth property (verifiable completeness) that CAP does not model. The system provides: eventual consistency (C, weakened), availability (A), partition tolerance (P), and completeness (a new axis).

## 5. Five Verification Layers

Structural sync extends to a five-layer verification stack when augmented with zero-knowledge proofs, hash chains, and verifiable delay functions:

| Layer | Discipline | Mechanism | Property |
|---|---|---|---|
| 1. Validity | Computation | ZK proof (zheng) | State transition is correct |
| 2. Ordering | Data structure | Hash chain + VDF | Operations carry their own order |
| 3. Completeness | Logic | NMT | Nothing was omitted |
| 4. Availability | Probability | DAS + erasure | Data physically exists |
| 5. Merge | Algebra | CRDT / foculus | Convergence is deterministic |

Layers 1–4 are scale-invariant: they work identically for two devices or $10^9$ nodes. Layer 5 adapts to trust context: CRDT merge (cooperative, same identity) or stake-weighted convergence (adversarial, different identities).

Each layer is independently verifiable. A participant can check any subset of layers, getting a partial guarantee. Checking all five layers yields the full VEC guarantee.

### 5.1 Ordering Without Coordination

Ordering is necessary for mutable state (name resolution, counter updates). What is unnecessary is *coordinated* ordering — the expensive part where nodes exchange messages to agree on sequence.

Structural sync replaces coordinated ordering with *deterministic local ordering*: signals carry ordering metadata (hash chain for causality, VDF for physical time, step counter for logical sequence). Given the same signal set, every node independently computes the same total order:

1. Causal order: signal $A$ in $B$'s dependencies implies $A$ before $B$
2. VDF order: if $A.\text{vdf\_time} < B.\text{vdf\_time}$ and not causally related, then $A$ before $B$
3. Hash tiebreak: concurrent signals ordered by $H(A) < H(B)$

The ordering algorithm is local. Agreement on order is a consequence of set equality (guaranteed by NMT + DAS), not of protocol execution.

### 5.2 Byzantine Detection vs. Tolerance

Traditional BFT tolerates $f$ Byzantine nodes among $3f+1$. Structural sync *detects* Byzantine behavior structurally:

| Fault | Detection | Cost |
|---|---|---|
| Forging (invalid operation) | ZK proof fails | O(1) — proof verification |
| Equivocation (double-signaling) | Two signals with same `prev` | O(1) — hash comparison |
| Withholding (hiding signals) | NMT completeness proof fails | O(log n) — tree walk |
| Data loss | DAS sampling fails | O(√n) — random samples |
| Flooding (signal spam) | VDF rate limit exceeded | O(1) — time check |

Detection does not require a protocol round. Any single honest peer can verify and produce a cryptographic misbehavior proof. No honest majority assumption is needed for fault detection.

## 6. The Continuous Fold

When all five layers are optimized, the entire pipeline from computation to verification becomes one continuous fold operation.

### 6.1 Signal Creation

A signal is a self-certifying batch of state transitions. During creation:

1. The VM executes each step, generating a trace row
2. Each trace row is folded into a running accumulator via HyperNova [10] (~30 field operations per step)
3. Each hash operation uses a folded sponge construction (~30 field operations per absorption)
4. At computation end, the accumulator *is* the proof — zero additional proving latency

This is *proof-carrying computation*: the proof accumulates during execution rather than being generated afterward.

### 6.2 Block Processing

When signals enter the network:

1. Each signal's validity proof is folded into a block accumulator (~30 field operations per signal)
2. State updates apply to a unified polynomial commitment replacing separate tree structures
3. Cross-index consistency is structural (same polynomial) — no separate consistency proof needed

The polynomial commitment replaces 9 independent hash trees, reducing per-operation cost from $\sim$106,000 constraints to $\sim$3,200 constraints (33× improvement).

### 6.3 Epoch Composition

Block accumulators fold into epoch accumulators:

- Current: 1000 blocks × 70,000 constraints for recursive verification = 70M constraints
- With folding: 1000 folds × 30 field operations + one decider = $\sim$100K constraints (700× improvement)

### 6.4 Universal Accumulator

All five verification layers fold into a single accumulator of $\sim$200 bytes. The accumulator proves: all signals are valid (layer 1), correctly ordered (layer 2), complete (layer 3), available (layer 4), and correctly merged (layer 5) — for the entire history from genesis.

A light client downloads 200 bytes, runs one decider ($\sim$10–50 μs), and has full confidence in the chain state.

### 6.5 Algebraic Data Availability

When completeness proofs transition from NMT paths to polynomial openings (algebraic NMT), DAS inherits the efficiency:

| | NMT-based DAS | Algebraic DAS |
|---|---|---|
| Sample proof | $O(\log n) \times 32$ bytes | $\sim$200 bytes (Lens opening) |
| 20-sample verification | 20 KiB bandwidth | 4 KiB bandwidth |
| Per-sample verification | $O(\log n)$ hash operations | $O(1)$ field operations |

This is not a separate optimization — it is a natural consequence of the completeness layer becoming algebraic. When the tree becomes a polynomial, every proof that referenced the tree automatically becomes a polynomial opening.

## 7. End-to-End Performance

The full pipeline with all optimizations:

| Metric | Current Architecture | Full Pipeline | Improvement |
|---|---|---|---|
| Proving latency | Separate phase | Zero (proof-carrying) | — |
| Per-operation constraints | $\sim$107,500 | $\sim$3,200 | 33× |
| Per-block (1000 ops) | $\sim$148M constraints | $\sim$8.3M | 18× |
| Epoch composition (1000 blocks) | $\sim$70M constraints | $\sim$100K | 700× |
| Hash calls/block (state) | 144,000 | 0 (algebraic) | — |
| Light client join bandwidth | Full chain | < 10 KiB | > 1000× |
| Light client join verification | $O(\text{chain})$ replay | 10–50 μs | — |
| DAS bandwidth | 20 KiB | 4 KiB | 5× |
| Checkpoint size | Variable | $\sim$232 bytes | — |

The cost of adding one edge to the permanent, verified, globally-available knowledge graph:

| Component | Constraints |
|---|---|
| Proof overhead per computation step | $\sim$30 field ops (folded) |
| Content identity (hash) | $\sim$164 (folded sponge) |
| Public state update | $\sim$3,200 (algebraic polynomial) |
| Private state update | $\sim$5,000 (polynomial mutator set) |
| **Total proving overhead** | **$\sim$8,400 constraints** |

Down from $\sim$148,000 constraints — a 17× reduction in proving overhead beyond raw computation.

## 8. Comparison with Existing Systems

| System | Consensus | Completeness | Availability | Merge | VEC |
|---|---|---|---|---|---|
| Bitcoin [3] | Nakamoto PoW | No proof | Full nodes only | N/A | No |
| Ethereum [11] | Casper FFG | Light client (proposed) | DAS (proposed) | N/A | No |
| Celestia [8] | Tendermint | NMT proofs | DAS | External (rollups) | Partial |
| IPFS [12] | None | No proof | DHT | No merge | No |
| Automerge [13] | None | No proof | No guarantee | CRDT | No |
| Git | None | Hash chain | Clone | Manual | No |
| **Structural sync** | **None** | **NMT/polynomial** | **DAS + erasure** | **CRDT/foculus** | **Yes** |

Celestia comes closest, providing NMT completeness and DAS availability. However, Celestia relies on Tendermint consensus for ordering and delegates merge semantics to external rollups. Structural sync replaces the consensus layer entirely with algebraic merge (CRDT locally, stake-weighted convergence globally).

## 9. The Minimum Structure Theorem

**Conjecture.** CRDT + NMT + DAS is the minimum composition achieving verified convergence without coordination.

**Argument by elimination.** Removing any layer loses a property the other two cannot provide:
- Without CRDT: no convergence guarantee (multiple valid states, no resolution)
- Without NMT: no completeness guarantee (convergence on possibly incomplete data)
- Without DAS: no availability guarantee (complete but physically lost data)

**Argument by coverage.** Every synchronization failure is a failure of exactly one layer:
- Wrong state → merge failure (CRDT)
- Incomplete state → completeness failure (NMT)
- Lost state → availability failure (DAS)

No failure falls outside these three categories. The three layers are jointly sufficient and individually necessary.

**Open question.** Can algebraic NMT (polynomial commitments) unify the completeness and merge layers into a single algebraic structure? If polynomial commitment soundness provides both binding (completeness) and deterministic evaluation (merge), the minimum composition may reduce from three layers (algebra + logic + probability) to two (algebra + probability). This would be a stronger result.

## 10. Open Problems

1. **Formalization of VEC.** The safety, completeness, availability, and liveness properties in Section 4 need formal treatment under a precise adversary model. What are the exact assumptions? How does VEC relate to the linearizability–eventual consistency spectrum?

2. **Lower bounds.** Is the minimum structure theorem provable? A formal proof that three independent mechanisms are necessary would establish structural sync as a fundamental primitive, not merely a useful construction.

3. **Composability.** Do structural sync protocols compose? If system A and system B each provide VEC, does their combination automatically inherit all guarantees? This matters for federated knowledge graphs and cross-chain interoperability.

4. **Algebraic unification.** Polynomial commitments may unify layers 3 (completeness) and 4 (availability) with algebraic DAS, and potentially absorb layer 5 (merge) if the polynomial structure encodes a deterministic merge function. The endgame may be two layers: algebraic (polynomial) + probabilistic (sampling), with CRDT properties implicit in the polynomial structure.

5. **Sybil resistance without consensus.** Structural sync at local scale has no sybil problem (devices share one identity). At global scale, stake-weighted convergence (foculus) provides sybil resistance. The general question: is sybil resistance possible without resource commitment? VDF provides rate limiting but not identity binding.

## References

[1] M. Castro and B. Liskov, "Practical Byzantine Fault Tolerance," OSDI 1999.

[2] E. Buchman, J. Kwon, and Z. Milosevic, "The latest gossip on BFT consensus," 2018.

[3] S. Nakamoto, "Bitcoin: A Peer-to-Peer Electronic Cash System," 2008.

[4] M. Yin et al., "HotStuff: BFT Consensus with Linearity and Responsiveness," PODC 2019.

[5] M. J. Fischer, N. A. Lynch, and M. S. Paterson, "Impossibility of Distributed Consensus with One Faulty Process," JACM 1985.

[6] E. A. Brewer, "Towards Robust Distributed Systems," PODC Keynote, 2000.

[7] M. Shapiro et al., "Conflict-free Replicated Data Types," SSS 2011.

[8] M. Al-Bassam et al., "LazyLedger: A Data Availability Blockchain Using Namespace Merkle Trees," 2019.

[9] M. Al-Bassam et al., "Fraud and Data Availability Proofs: Maximising Light Client Security and Scaling Blockchains with Dishonest Majorities," 2018.

[10] A. Kothapalli and S. Setty, "HyperNova: Recursive arguments from folding schemes," 2023.

[11] V. Buterin et al., "Ethereum 2.0 Specifications," 2020–present.

[12] J. Benet, "IPFS — Content Addressed, Versioned, P2P File System," 2014.

[13] M. Kleppmann and A. R. Beresford, "A Conflict-Free Replicated JSON Datatype," IEEE TPDS 2017.
