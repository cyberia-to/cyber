---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: stark verification, nox starks, stark proofs, proof system, cyber proofs
---
# proofs

every action in [[cyber]] produces a [[stark]] proof. one proof system. one hash. one field. the table below catalogs every proof type the protocol generates.

```
PROOF TAXONOMY
══════════════

CATEGORY              │ PROOF TYPE                │ WHAT IT PROVES                          │ CONSTRAINTS
──────────────────────┼───────────────────────────┼─────────────────────────────────────────┼────────────
identity              │ preimage knowledge         │ neuron knows secret behind address       │ ~736
                      │ set membership             │ neuron belongs to valid set              │ ~1,000
                      │ stake sufficiency          │ neuron has enough stake for action       │ ~1,000
                      │ nullifier freshness        │ action has not been performed before     │ ~3,000
──────────────────────┼───────────────────────────┼─────────────────────────────────────────┼────────────
cybergraph            │ anonymous cyberlink        │ valid neuron linked, identity hidden     │ ~13,000
                      │ ownership                  │ neuron possesses resource / UTXO         │ ~5,000
                      │ completeness               │ response includes everything, nothing    │ ~10,000
                      │                            │ withheld                                 │
                      │ range                      │ value falls within bounds                │ ~2,000
──────────────────────┼───────────────────────────┼─────────────────────────────────────────┼────────────
communication         │ delivery (per hop)         │ relay forwarded correctly                │ ~60,000
                      │ delivery (chained)         │ message reached recipient through N hops │ ~320,000
                      │ receipt                    │ recipient decrypted and verified MAC     │ ~70,000
──────────────────────┼───────────────────────────┼─────────────────────────────────────────┼────────────
execution             │ correct execution          │ nox program ran correctly                │ varies
                      │ correct inference          │ neural network output matches inputs     │ varies
                      │ correct compilation        │ compiler produced valid output           │ varies
                      │ correct optimization       │ optimized program equivalent to original │ varies
                      │ equivalence                │ two programs produce identical results   │ varies
                      │ termination                │ program halts in bounded steps           │ varies
──────────────────────┼───────────────────────────┼─────────────────────────────────────────┼────────────
data structures       │ Merkle inclusion           │ element exists in tree                   │ ~9,600
                      │ polynomial inclusion       │ element exists in committed polynomial   │ ~1,000
                      │ non-membership             │ element is absent from set               │ ~3,000
                      │ WHIR low-degree            │ committed polynomial has bounded degree  │ ~10,000
──────────────────────┼───────────────────────────┼─────────────────────────────────────────┼────────────
storage &             │ storage                    │ content bytes exist on specific node     │ ~5,000
availability          │ size                       │ claimed content size matches actual bytes │ ~2,000
                      │ replication                │ k independent copies exist               │ ~5,000 × k
                      │ retrievability             │ content fetchable within bounded time    │ ~5,000
                      │ data availability (DAS)    │ block data was published, is accessible  │ ~8,000
                      │ encoding fraud             │ erasure coding was done correctly        │ O(k log n)
──────────────────────┼───────────────────────────┼─────────────────────────────────────────┼────────────
recursive             │ proof aggregation          │ N proofs are all valid                   │ ~70,000
                      │ recursive composition      │ proof-of-proof, constant size            │ ~70,000
──────────────────────┼───────────────────────────┼─────────────────────────────────────────┼────────────
location              │ RTT consistency            │ node is at claimed geohash               │ O(N²) RTT
                      │ medium declaration         │ link uses declared transmission medium   │ O(N) verify
                      │ observer bootstrap         │ absolute coordinates from single origin  │ MDS + A1
```

every proof in the table is a [[stark]]. no SNARKs, no trusted setup, no curves. one hash ([[Hemera]]), one VM ([[nox]]), one field ([[Goldilocks field]]).

## the proof system

[[cyber]] uses multilinear [[starks]] via the Whirlaway architecture: [[SuperSpartan]] IOP + [[WHIR]] as the multilinear polynomial commitment scheme. no trusted setup, [[Hemera]]-only security (post-quantum), native [[Goldilocks field]] arithmetic.

```
Property          │ SNARK         │ stark (multilinear)
──────────────────┼───────────────┼─────────────────────
Trusted setup     │ Required      │ NOT REQUIRED
Quantum resistant │ No            │ Yes
Proof size        │ ~200 bytes    │ ~60-157 KB
Security basis    │ Discrete log  │ Hash only
Field compatible  │ Specific      │ Any (Goldilocks)
Prover (constr.)  │ O(N log N)    │ O(N) linear
Verifier          │ O(1) pairing  │ O(log² N) hash
```

### the pipeline

```
nox execution → trace (2ⁿ steps × registers)
  → encode as ONE multilinear polynomial f(x₁, ..., x_{n+m})
  → WHIR_commit(f) = C
  → SuperSpartan sumcheck: verify AIR constraints hold for all rows
  → reduces to: evaluate f at ONE random point r
  → WHIR_open(f, r) = (v, π)
  → verifier: check sumcheck transcript + WHIR_verify(C, r, v, π)
```

the [[nox]] VM's sixteen reduction patterns map to AIR transition constraints — each pattern becomes a polynomial equation relating register state before and after a reduction step. [[SuperSpartan]] handles AIR natively via CCS (Customizable Constraint Systems), with linear-time prover and logarithmic-time verifier.

see [[zheng]] for the concrete implementation (AIR from nox, constraint budget, Hemera as stark hash, recursive composition, BBG integration). see [[stark]] for the general architecture (AIR, CCS, SuperSpartan, Whirlaway).

### self-verification

```
THEOREM: The stark verifier for nox is expressible as a nox program.

stark verification requires:
  1. Field arithmetic (patterns 5, 7, 8)
  2. Hash computation (pattern 15)
  3. Sumcheck verification (patterns 5, 7, 9 — field ops only)
  4. WHIR opening verification (pattern 15 + conditionals + poly_eval)

All are nox-native. QED.

CONSEQUENCE:
  verify(proof) can itself be proven
  This enables recursive proof composition
  O(1) verification regardless of computation size
```

the system closes on itself. no trusted external verifier remains.

### verifier complexity

```
stark VERIFIER COMPONENTS       │ Layer 1 only │ With Layer 3 jets
────────────────────────────────┼──────────────┼──────────────────
1. Parse proof                  │     ~1,000   │    ~1,000
2. Fiat-Shamir challenges       │    ~30,000   │    ~5,000  (hash jet)
3. Merkle verification          │   ~500,000   │   ~50,000  (merkle_verify jet)
4. Constraint evaluation        │    ~10,000   │    ~3,000  (poly_eval jet)
5. WHIR verification            │    ~50,000   │   ~10,000  (fri_fold + ntt jets)
────────────────────────────────┼──────────────┼──────────────────
TOTAL                           │   ~600,000   │   ~70,000

~8.5× reduction. This cost is CONSTANT regardless of what was proven.
Layer 3 jets make recursive composition practical.
```

### recursive composition

```
Level 0: Prove computation C → proof π₀
Level 1: Prove verify(π₀) → proof π₁ (~100-200 KB)
Level 2: Prove verify(π₁) → proof π₂ (same size)

AGGREGATION:
  N transactions → N proofs
  Verify all N in one nox program
  Prove that verification → single proof

  Result: O(1) on-chain verification for O(N) transactions
```

## identity proofs

a [[neuron]] proves itself by demonstrating knowledge of a secret that hashes to its address. no [[signature]] scheme. one hash, one proof.

```
neuron_secret → Hemera(neuron_secret) = neuron_address
auth = stark_proof(∃ x : Hemera(x) = neuron_address)
```

the preimage proof costs ~736 constraints. the full lock script verification (with [[nox]] jets) costs ~70,000 constraints. programmable lock scripts extend this to multisig, timelocks, delegation, and recovery — all via the same mechanism.

see [[cyber/identity]] for the full specification.

### anonymous cyberlinks

a [[neuron]] proves it is valid, has sufficient [[stake]], and has not double-linked — without revealing which neuron it is. the circuit (~13,000 constraints) covers:

1. identity: `Hemera(secret) ∈ neuron_set` (~1,000 via WHIR membership)
2. stake: `stake(Hemera(secret)) ≥ weight` (~1,000 via WHIR lookup)
3. nullifier: `nullifier == Hemera(secret ∥ source ∥ target)` (~736)
4. freshness: `nullifier ∉ spent_set` (~3,000 via SWBF check)

the graph sees edges and weights. the graph does not see authors. see [[cyber/identity]] for the privacy boundary.

## delivery proofs

[[cyber/communication]] uses chained stark proofs for proof of delivery. each relay hop produces a proof attesting correct forwarding. proofs compose recursively:

```
π₁ = stark(R₁ received blob, peeled layer, forwarded to R₂)
π₂ = stark(R₂ received blob, peeled layer, forwarded to R₃)
π₃ = stark(R₃ received blob, peeled layer, forwarded to B)
π_B = stark(B received blob, decrypted plaintext, MAC verified)

π_chain = stark(verify(π₁) ∧ verify(π₂) ∧ verify(π₃) ∧ verify(π_B))
```

one proof (~100-200 KB) covers the entire route. O(1) verification regardless of hop count. the sender publishes π_chain as a [[particle]] in the [[cybergraph]]. anyone can verify delivery happened. no one can read the message or learn the route.

relays earn [[focus]] for proven delivery. no proof, no payment.

## execution proofs

every [[nox]] program produces a stark proof of correct execution. this generalizes to:

| proof type | what runs | where used |
|---|---|---|
| correct execution | any [[nox]] program | every [[cyberlink]], every transaction |
| correct inference | neural network forward pass | [[trident]] verifiable AI |
| correct compilation | compiler pipeline | [[trident]] self-optimizing compilation |
| correct optimization | optimizer transforms | [[trident]] verified optimizations |
| equivalence | two programs on all inputs | formal verification via [[nox]] |
| termination | bounded step count | resource metering, DoS prevention |

[[trident]] extends this to AI: a stark proof that a neural network inference was computed correctly. the verifier checks the proof without re-running the network. this enables verifiable AI at scale — trustless inference, auditable models, provable predictions.

## data structure proofs

the [[cybergraph]] uses polynomial commitments ([[BBG]]) instead of Merkle trees for most operations. the cost difference:

```
OPERATION                    │ Merkle tree  │ Polynomial commitment
─────────────────────────────┼──────────────┼──────────────────────
membership / inclusion       │ ~9,600       │ ~1,000
non-membership               │ ~9,600       │ ~3,000
batch proof (N elements)     │ ~9,600 × N   │ ~1,000 (amortized)
state root update            │ ~9,600       │ ~1,000
completeness (nothing hidden)│ impossible   │ ~10,000
```

polynomial commitments use [[WHIR]] as a multilinear PCS. WHIR proofs demonstrate that a committed polynomial has bounded degree and open evaluations at specific points — the foundation for all [[BBG]] operations and for the [[stark|multilinear stark]] pipeline itself.

## storage and availability proofs

at planetary scale, content loss is the existential risk. if the content behind a [[particle]] hash is lost, the particle is dead — its identity exists but its meaning is gone. six proof types prevent this:

| proof | what it guarantees | mechanism |
|---|---|---|
| storage proof | content bytes exist on specific storage | periodic challenges against content hash |
| size proof | claimed content size matches actual bytes | Hemera tree structure commitment + padding check |
| replication proof | k independent copies exist | challenge distinct replicas, verify uniqueness |
| retrievability proof | content fetchable within bounded time | timed challenge-response with latency bound |
| data availability proof | block data was published and is accessible | erasure coding + random sampling (DAS) |
| encoding fraud proof | erasure coding was done correctly | decode k+1 cells, compare against row commitment |

storage proofs verify individual [[particle]] content. size proofs bind [[particles]] to their dimensions — a hash commits to identity, a size proof commits to byte count. the two together prevent storage fee inflation and ensure erasure coding grids have correct dimensions. data availability proofs verify that batches of [[cyberlinks]] and state transitions were published and accessible to all participants. the three are complementary — storage ensures content survives, size ensures claims are honest, DA ensures state transitions are visible.

### layered data availability

```
Tier 0 — critical roots    checkpoint posted to settlement layer    immutable forever
                            ~32-64 KB per epoch                     ultimate recovery

Tier 1 — active graph      focus blobs (~10K cyberlinks + proofs)   ≥ 30 days retention
                            posted to DA layer                      verified by light sampling

Tier 2 — historical tails  erasure-coded archival                   deep replay, rehashing
                            refreshed by archivers                  persistent storage
```

### namespace-aware DAS

light clients verify data availability without downloading full data. the [[BBG]]'s [[NMT]] structure enables namespace-aware sampling: a client requesting "give me everything for [[neuron]] N" receives data plus a completeness proof — O(√n) random samples for 99.9% confidence.

### encoding fraud proofs

if a block producer encodes a row incorrectly in the 2D Reed-Solomon erasure grid:

```
1. obtain k+1 of 2k cells from the row
2. attempt Reed-Solomon decoding
3. decoded polynomial ≠ row NMT root → fraud proof
4. any verifier checks: decode(cells) ≠ row commitment → block rejected

proof size: O(k) cells with O(log n) proofs each
verification: O(k log n)
```

### hash migration

storage proofs are Phase 1 security infrastructure. if [[Hemera]] is ever broken, storage proofs enable full graph rehash under a new hash function. without them, the hash choice is irreversible. with them, [[Hemera]] becomes a replaceable component.

```
10¹⁵ particles ÷ 10⁶ nodes = 10⁹ particles per node
at ~310K hashes/s per core → ~17 hours for full parallel rehash
bottleneck: storage proof coverage and network bandwidth
```

see [[storage proofs]] for the full specification, [[radio]] for the transport layer, [[NMT]] for namespace-aware sampling, [[data structure for superintelligence]] for DAS architecture.

## consensus proofs

[[cyber]] uses [[proof of stake]] via [[tendermint]] for block production. the broader landscape:

| mechanism | what it proves | energy cost | assumption |
|---|---|---|---|
| [[proof of work]] | computational effort expended | high | honest majority (51%) |
| [[proof of stake]] | economic commitment at risk | low | honest majority (67%) |
| stark execution proof | computation ran correctly | minimal | hash collision resistance |

[[cyber]] layers stark execution proofs on top of [[proof of stake]] consensus. validators produce blocks (PoS), and every state transition within those blocks carries a stark proof of correct execution. the combination: economic security from stake, computational integrity from proofs.

## epistemological proofs

[[cybics]] introduces proof by simulation — a paradigm where convergence replaces derivation.

```
PROOF BY DERIVATION (classical)
  axioms → inference rules → theorem
  limitation: Goedel incompleteness

PROOF BY SIMULATION (cybics)
  initial state → convergent dynamics → fixed point
  the fixed point IS the proof
```

the [[cybergraph]] generates three epistemological proofs:

| proof | mechanism | what it establishes |
|---|---|---|
| proof of relevance | [[tri-kernel]] convergence to [[focus]] distribution π* | collective understanding of what matters |
| proof of commitment | [[focus]] spent on [[cyberlinks]] | skin in the game — irreversible resource allocation |
| proof of measurement | [[Hemera]] hash of content | information-theoretic reduction — the hash is the measurement |

a [[cyberank]] distribution π* is a simulation-proof of collective [[relevance]]: no axioms, no authority, no vote. convergence under conservation laws.

## location proofs

[[proof_of_location]] provides cryptographically verifiable geolocation without trusted anchors, GPS, or certificate authorities. the construction uses RTT measurements across declared transmission media, [[verifiable delay functions]], and [[Merkle]] causal clocks. nodes self-organize into a 3D coordinate embedding via multidimensional scaling, calibrated to Earth's circumference.

| proof | what it guarantees | mechanism |
|---|---|---|
| RTT consistency | node is physically at claimed geohash | pairwise RTT mesh normalized by declared c_medium, verified by MDS embedding |
| medium declaration | link uses claimed transmission medium | RTT consistency cross-check against canonical propagation speeds |
| observer bootstrap | absolute coordinates from single origin | one observer asserts position (A1), spherical constraint forces unique embedding |

[[sybil attacks|Sybil]] resistance is physical: faking RTT consistency with a dense global mesh across multiple media is impossible. economic enforcement via latency-weighted relay fees makes honest reporting a [[dominant strategy]] [[equilibrium]] — stronger than [[nash equilibrium]].

see [[proof_of_location]] for the full specification.

## the proof stack

```
┌─────────────────────────────────────────────────────────┐
│  epistemological    proof by simulation (cybics)         │
│                     convergence → fixed point → truth    │
├─────────────────────────────────────────────────────────┤
│  application        identity, delivery, location,        │
│                     inference, anonymity, storage,       │
│                     range, ownership                     │
├─────────────────────────────────────────────────────────┤
│  recursive          proof aggregation, composition       │
│                     O(1) verification for O(N) proofs    │
├─────────────────────────────────────────────────────────┤
│  IOP               SuperSpartan (CCS/AIR via sumcheck)  │
│                     linear-time prover, log-time verifier│
├─────────────────────────────────────────────────────────┤
│  PCS               WHIR (multilinear polynomial commit)  │
│                     290 μs verify, ~157 KiB proofs       │
├─────────────────────────────────────────────────────────┤
│  primitives         Hemera (hash), nox (VM),             │
│                     Goldilocks field (arithmetic)        │
└─────────────────────────────────────────────────────────┘
```

one hash. one VM. one field. one IOP. one PCS. every proof in [[cyber]] — from a single [[cyberlink]] to a chained delivery receipt to a trillion-parameter neural network inference — reduces to: run a [[nox]] program, commit trace via [[WHIR]], verify constraints via [[sumcheck]], produce a [[stark]].

see [[cyber/identity]] for authentication and anonymity, [[cyber/communication]] for delivery proofs, [[proof_of_location]] for anchor-free geolocation, [[BBG]] for polynomial commitment architecture, [[trident]] for verifiable AI, [[cybics]] for proof by simulation, [[cyber/security]] for formal guarantees