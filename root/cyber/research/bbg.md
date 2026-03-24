---
tags: cyber, research, article
crystal-type: process
crystal-domain: cyber
status: draft
date: 2026-03-24
---
# BBG: the authenticated state layer

## abstract

[[BBG]] (Big Badass Graph) is the authenticated state layer for [[cyber]]. it commits the entire [[cybergraph]] — particles, axons, neurons, tokens, temporal state — to a single cryptographic root. individual [[cyberlinks]] are private. the public aggregate (axon weights, neuron summaries, particle energy, token supplies) is committed to [[NMT]] trees with completeness proofs. [[mutator set]] (AOCL + SWBF) handles private records. the architecture is governed by three laws: bounded locality, constant-cost verification, and structural security.

this paper describes the current architecture and its evolution path toward polynomial state — where 13 hash-tree sub-roots collapse to one polynomial commitment, cross-index consistency becomes structural, and a 240-byte checkpoint proves all history.

## 1. three laws

every design decision in BBG follows from three laws:

**law 1: bounded locality.** the cost of an operation is proportional to what it touches, not total state size. at $10^{15}$ particles, no operation touches the whole graph. every read is $O(\log n)$. every update is $O(\log n)$. global recomputation is physically impossible and architecturally forbidden.

**law 2: constant-cost verification.** verifying any claim about the graph costs $O(1)$ — independent of graph size, history length, or computation complexity. one [[zheng]] decider call (10-50 μs) verifies any proof. a light client with 240 bytes of state has the same verification power as a full node.

**law 3: structural security.** guarantees come from data structure invariants, not protocol correctness. an [[NMT]] sorting invariant cannot be violated — the tree structure prevents it. a [[mutator set]] cannot double-spend — the SWBF bitmap prevents it. security is not "the protocol was followed" but "the structure makes cheating impossible."

## 2. the five primitives

| primitive | what it is | identity | role |
|---|---|---|---|
| [[particles\|particle]] | content-addressed node | $H(\text{content})$ — 32 bytes | atom of knowledge |
| [[cyberlinks\|cyberlink]] | private authenticated edge | $H(\nu, p, q, \tau, a, v, t)$ — 7-tuple | unit of assertion |
| [[neurons\|neuron]] | agent with stake and focus | $H(\text{public\_key})$ | decision-maker |
| [[token]] | economic value | denomination hash / content hash | coin, card, score, badge |
| [[focus]] | emergent attention | $\pi^*$ from [[tri-kernel]] | what the network pays attention to |

derived: **axon** = $H(\text{from}, \text{to}) \in P$. aggregate of all cyberlinks between two particles. the axon is public; the individual cyberlinks composing it are private.

## 3. BBG root: 13 sub-roots

the entire state commits to one root:

$$\text{BBG\_root} = H(\text{13 sub-roots}) = H(416 \text{ bytes})$$

### public indexes (9 NMTs)

each is a [[NMT|Namespace Merkle Tree]] with [[Hemera]] hash nodes and sorted namespace labels for completeness proofs.

| index | key → value | leaf size | what it proves |
|---|---|---|---|
| particles.root | CID → particle record | 48-96 B | particle exists with this energy and $\pi^*$ |
| axons_out.root | source CID → axon pointer | 64 B | all outgoing edges from this particle |
| axons_in.root | target CID → axon pointer | 64 B | all incoming edges to this particle |
| neurons.root | neuron ID → neuron record | 56 B | neuron's focus, karma, stake |
| locations.root | neuron ID → location record | 104 B | spatial association |
| coins.root | denomination → supply record | 81 B | fungible token supply |
| cards.root | card ID → card record | 104 B | non-fungible knowledge asset |
| files.root | CID → availability record | 76 B | content availability ([[DAS]]) |
| time.root | time namespace → BBG snapshot | 72 B | state at historical time T |

NMT completeness guarantee: "if you ask for all particles in namespace P, the tree PROVES it gave you everything. omission is structurally impossible."

### private state (3 commitments)

| commitment | structure | what it protects |
|---|---|---|
| cyberlinks.root | MMR peaks hash (AOCL) | append-only list of all committed cyberlinks |
| spent.root | MMR root (archived SWBF) | nullifier history — prevents double-spend |
| balance.root | Hemera hash (active SWBF) | 128 KB sliding window bitmap — current nullifiers |

the [[mutator set]] (AOCL + SWBF) is the universal privacy primitive. every private record — cyberlinks, transfers, positions — uses the same mechanism. membership proof: $O(\log N)$ Hemera hashes. non-membership proof: bitmap check + $O(\log N)$ MMR walk.

### finalization (1 root)

| root | structure | purpose |
|---|---|---|
| signals.root | MMR of finalized signal batches | append-only record of all processed [[signals]] |

## 4. privacy model

the privacy boundary separates individual actions from public aggregates:

```
PRIVATE (mutator set):                  PUBLIC (NMT indexes):
  who linked what (individual cyberlinks)  axon weights (aggregate conviction)
  individual conviction amounts             particle energy, π*
  neuron linking history                    neuron summaries (focus, karma, stake)
  market positions                          token supplies
  UTXO values and owners                   axon market state
```

a neuron proves it has sufficient stake and creates a valid cyberlink — without revealing which neuron it is. the graph sees edges and weights. the graph does not see authors.

the mechanism: [[zheng]] zero-knowledge proof (~13,000 constraints) covers identity ($H(\text{secret}) \in \text{neuron\_set}$), stake sufficiency, nullifier freshness, and cyberlink well-formedness. see [[cyber/proofs|proofs]] for the full circuit.

## 5. state transitions

six transaction types modify BBG state:

| transaction | what changes | proof cost | frequency |
|---|---|---|---|
| CYBERLINK | create private record + update public aggregates | ~13K constraints | dominant (every edge) |
| PRIVATE TRANSFER | move value between private records (UTXO) | ~40K constraints | moderate |
| COMPUTATION | execute [[nox]] program, deduct focus | varies | moderate |
| MINT CARD | create non-fungible knowledge asset | ~5K constraints | rare |
| TRANSFER CARD | change card ownership | ~3K constraints | rare |
| BRIDGE | convert coin to focus | ~3K constraints | rare |

every transaction produces a [[zheng]] proof. every proof folds into the block accumulator via [[HyperNova]].

cross-index consistency: when a cyberlink updates axons_out, axons_in, particles, AND neurons simultaneously, [[LogUp]] proves all four indexes agree on the shared data (~500 constraints per lookup, ~1,500 per cyberlink).

## 6. sync

one mechanism at three scales. the [[signal]] is the universal unit of state change. five verification layers apply at every scale:

| layer | mechanism | guarantee |
|---|---|---|
| 1. validity | [[zheng]] proof | state transition is correct |
| 2. ordering | hash chain + [[VDF]] | operations carry their own order |
| 3. completeness | [[NMT]] | nothing withheld |
| 4. availability | [[DAS]] + erasure | data physically exists |
| 5. merge | [[CRDT]] / [[foculus]] | convergence is deterministic |

layers 1-4 are scale-invariant. layer 5 adapts: CRDT for local device sync, foculus ($\pi$ convergence) for global consensus.

a light client joins by downloading a 240-byte checkpoint (BBG_root + accumulator + height), running one [[zheng]] decider (10-50 μs), and syncing namespaces of interest via NMT completeness proofs.

see [[structural-sync]] for the full theory (Verified Eventual Consistency).

## 7. the polynomial evolution

the current architecture uses hash trees (NMTs). the evolution path replaces them with polynomial commitments — where the proof system's native operation (polynomial evaluation) IS the state query.

### phase 1: algebraic NMT

replace 9 independent NMT trees with one multivariate polynomial:

$$\text{BBG\_poly}(\text{index}, \text{namespace}, \text{position}) = \text{value}$$

| metric | NMT (current) | algebraic NMT |
|---|---|---|
| per-cyberlink | ~107.5K constraints | ~3.2K constraints (33×) |
| per-block (1000 tx) | ~108M constraints | ~7.5M constraints (14×) |
| LogUp cost | ~6M constraints | 0 (structural — same polynomial) |
| inclusion proof | ~1 KiB (Merkle path) | ~200 bytes (PCS opening) |
| internal node storage | ~5 TB (9 indexes) | 0 |

the key insight: [[sumcheck]] architecture makes polynomial state NATURAL. state reads are polynomial evaluations — the proof system's native operation. see [[algebraic state commitments]] and the [[cyber/research/zheng vs starks|zheng whitepaper]] §5 for why this is architectural, not a separate optimisation.

### phase 2: polynomial mutator set

replace SWBF (128 KB bitmap) + archived MMR with polynomial commitment over nullifiers:

$$N(x) = \prod(x - n_i) \text{ for all nullifiers } n_i$$

non-membership proof: one PCS opening (O(1)) instead of bitmap + MMR walk ($O(\log N)$). witness shrinks from 128 KB to 32 bytes.

### phase 3: unified polynomial state

all 13 sub-roots → one polynomial commitment:

$$\text{BBG\_root} = \text{PCS.commit}(\text{BBG\_poly})$$

32 bytes. one polynomial. all state. all time. every query = one PCS opening. cross-index consistency = structural (same polynomial, different evaluation dimensions). LogUp eliminated entirely.

### phase 4: algebraic DAS

when completeness proofs become polynomial openings (phase 1), [[DAS]] inherits the efficiency. each sample proof = one PCS opening (~200 bytes) instead of NMT path (~1 KiB). 20-sample DAS verification: 4 KiB instead of 20 KiB, ~3K constraints instead of ~471K (157× improvement).

## 8. signal-first architecture

BBG state is a deterministic function of the [[signal]] log:

$$\text{BBG\_state}(h) = \text{fold}(\text{genesis}, \text{signals}[0..h])$$

the signal log is append-only, content-addressed, and [[DAS]]-protected. BBG state (13 sub-roots, NMT trees, mutator set) is DERIVED DATA — a materialised view over signals.

consequences:
- any node can reconstruct any historical BBG_root from signals
- crash recovery: fetch checkpoint (~240 bytes) + replay signals since checkpoint
- storage proofs reduce to signal availability proofs (no separate PoSt needed)
- the irreducible minimum per node: signal log + latest checkpoint

see [[signal-first]] for the full design.

## 9. π-weighted replication

storage replication factor proportional to $\pi$ ([[cyberank]]):

```
top-100 particle (π ~ 10⁻²):     ~1000 replicas
median particle (π ~ 10⁻⁶):       ~10 replicas
tail particle (π ~ 10⁻¹²):        3 replicas (minimum)
```

the network spends storage budget where attention goes. [[DAS]] parameters scale with replication — high-$\pi$ content needs fewer samples for the same confidence. verification cost ([[gravity commitment]]) follows the same power law. the entire stack — proof cost, storage, replication, verification — follows the $\pi$ distribution.

see [[pi-weighted-replication]], [[gravity commitment]], [[universal law]].

## 10. the numbers

```
                        current (NMT)        polynomial (all phases)   improvement
per-cyberlink:          ~107.5K constraints  ~3.2K constraints         33×
per-block (1000):       ~148M constraints    ~8.3M constraints         18×
epoch (1000 blocks):    ~70M constraints     ~100K constraints         700×
non-membership:         128 KB + O(log N)    32 bytes + O(1)           O(1)
inclusion proof:        ~1 KiB               ~200 bytes                5×
DAS (20 samples):       ~471K constraints    ~3K constraints           157×
sub-root count:         13                   1                         13×
LogUp:                  ~6M constraints/block 0 (structural)           ∞
NMT storage:            ~5 TB                0                         ∞
BBG_root:               416 bytes            32 bytes                  13×
light client join:      full chain           < 10 KiB, 10-50 μs       —
checkpoint:             varies               ~240 bytes                —
hemera calls/block:     ~144,000             0 (state) + batched       ∞
```

the cost of adding one edge to the permanent, verified, globally-available knowledge graph:

```
proof:           ~30 field ops per nox step (proof-carrying)
identity:        ~164 constraints (folded hemera sponge)
public state:    ~3.2K constraints (algebraic polynomial)
private state:   ~5K constraints (polynomial mutator set)
total overhead:  ~8.5K constraints (was ~148K — 17× reduction)
```

## 11. honest assessment

| claim | confidence | caveat |
|---|---|---|
| three laws hold | high | architectural properties, not implementation claims |
| 13-root structure | high | fully specified, NMT well-understood |
| privacy (mutator set) | high | Neptune heritage, AOCL+SWBF proven |
| 33× per-cyberlink (algebraic NMT) | high | architectural consequence of sumcheck |
| polynomial mutator set | medium | novel, needs implementation validation |
| unified polynomial (1 root) | medium | multivariate PCS at scale unproven |
| signal-first reconstruction | high | deterministic fold, standard approach |
| 157× algebraic DAS | high | follows from polynomial completeness |
| zero implementation | critical | the stack is specification, not code |

BBG is a specification. the architecture is sound. the numbers are analytical. the real test is implementation — and the dependency chain is deep: nebu → hemera → nox → zheng → BBG.

see [[structural-sync]] for the sync theory, [[cyber/research/zheng vs starks|zheng]] for the proof system, [[nox]] for the VM, [[Hemera]] for the hash, [[tri-kernel architecture]] for the focus computation, [[knowledge capacity]] for the information-theoretic limits, [[link production]] for the intelligence problem
