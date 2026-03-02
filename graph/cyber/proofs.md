---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: STARK verification, nox STARKs, cyber/stark, STARK, STARK proofs, proof system, cyber proofs
stake: 29173948768097356
---
# proofs

every action in [[cyber]] produces a [[STARK]] proof. one proof system. one hash. one field. the table below catalogs every proof type the protocol generates.

```
PROOF TAXONOMY
══════════════

CATEGORY              │ PROOF TYPE                │ WHAT IT PROVES                          │ CONSTRAINTS
──────────────────────┼───────────────────────────┼─────────────────────────────────────────┼────────────
identity              │ preimage knowledge         │ neuron knows secret behind address       │ ~300
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
                      │ storage                    │ content remains retrievable              │ ~5,000
                      │ FRI low-degree             │ committed polynomial has bounded degree  │ ~10,000
──────────────────────┼───────────────────────────┼─────────────────────────────────────────┼────────────
recursive             │ proof aggregation          │ N proofs are all valid                   │ ~70,000
                      │ recursive composition      │ proof-of-proof, constant size            │ ~70,000
```

every proof in the table is a [[STARK]]. no SNARKs, no trusted setup, no curves. one hash ([[Hemera]]), one VM ([[nox]]), one field ([[Goldilocks field]]).

## the proof system

[[nox]] uses STARKs (Scalable Transparent Arguments of Knowledge). the choice follows from alignment with nox's design: no trusted setup, [[Hemera]]-only security (post-quantum), native compatibility with [[Goldilocks field]] arithmetic.

```
Property          │ SNARK         │ STARK
──────────────────┼───────────────┼─────────────────
Trusted setup     │ Required      │ NOT REQUIRED
Quantum resistant │ No            │ Yes
Proof size        │ ~200 bytes    │ ~100-200 KB
Security basis    │ Discrete log  │ Hash only
Field compatible  │ Specific      │ Any (Goldilocks)
```

### self-verification

```
THEOREM: The STARK verifier for nox is expressible as a nox program.

STARK verification requires:
  1. Field arithmetic (patterns 5, 7, 8)
  2. Hash computation (pattern 15)
  3. Polynomial evaluation (patterns + recursion)
  4. Merkle verification (pattern 15 + conditionals)

All are nox-native. QED.

CONSEQUENCE:
  verify(proof) can itself be proven
  This enables recursive proof composition
  O(1) verification regardless of computation size
```

the system closes on itself. no trusted external verifier remains.

### verifier complexity

```
STARK VERIFIER COMPONENTS       │ Layer 1 only │ With Layer 3 jets
────────────────────────────────┼──────────────┼──────────────────
1. Parse proof                  │     ~1,000   │    ~1,000
2. Fiat-Shamir challenges       │    ~30,000   │    ~5,000  (hash jet)
3. Merkle verification          │   ~500,000   │   ~50,000  (merkle_verify jet)
4. Constraint evaluation        │    ~10,000   │    ~3,000  (poly_eval jet)
5. FRI verification             │    ~50,000   │   ~10,000  (fri_fold + ntt jets)
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
auth = STARK_proof(∃ x : Hemera(x) = neuron_address)
```

the preimage proof costs ~300 constraints. the full lock script verification (with [[nox]] jets) costs ~70,000 constraints. programmable lock scripts extend this to multisig, timelocks, delegation, and recovery — all via the same mechanism.

see [[cyber/identity]] for the full specification.

### anonymous cyberlinks

a [[neuron]] proves it is valid, has sufficient [[stake]], and has not double-linked — without revealing which neuron it is. the circuit (~13,000 constraints) covers:

1. identity: `Hemera(secret) ∈ neuron_set` (~1,000 via FRI membership)
2. stake: `stake(Hemera(secret)) ≥ weight` (~1,000 via FRI lookup)
3. nullifier: `nullifier == Hemera(secret ∥ source ∥ target)` (~300)
4. freshness: `nullifier ∉ spent_set` (~3,000 via SWBF check)

the graph sees edges and weights. the graph does not see authors. see [[cyber/identity]] for the privacy boundary.

## delivery proofs

[[cyber/communication]] uses chained STARK proofs for proof of delivery. each relay hop produces a proof attesting correct forwarding. proofs compose recursively:

```
π₁ = STARK(R₁ received blob, peeled layer, forwarded to R₂)
π₂ = STARK(R₂ received blob, peeled layer, forwarded to R₃)
π₃ = STARK(R₃ received blob, peeled layer, forwarded to B)
π_B = STARK(B received blob, decrypted plaintext, MAC verified)

π_chain = STARK(verify(π₁) ∧ verify(π₂) ∧ verify(π₃) ∧ verify(π_B))
```

one proof (~100-200 KB) covers the entire route. O(1) verification regardless of hop count. the sender publishes π_chain as a [[particle]] in the [[cybergraph]]. anyone can verify delivery happened. no one can read the message or learn the route.

relays earn [[focus]] for proven delivery. no proof, no payment.

## execution proofs

every [[nox]] program produces a STARK proof of correct execution. this generalizes to:

| proof type | what runs | where used |
|---|---|---|
| correct execution | any [[nox]] program | every [[cyberlink]], every transaction |
| correct inference | neural network forward pass | [[trident]] verifiable AI |
| correct compilation | compiler pipeline | [[trident]] self-optimizing compilation |
| correct optimization | optimizer transforms | [[trident]] verified optimizations |
| equivalence | two programs on all inputs | formal verification via [[nox]] |
| termination | bounded step count | resource metering, DoS prevention |

[[trident]] extends this to AI: a STARK proof that a neural network inference was computed correctly. the verifier checks the proof without re-running the network. this enables verifiable AI at scale — trustless inference, auditable models, provable predictions.

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

polynomial commitments use FRI (Fast Reed-Solomon Interactive Oracle Proofs) for low-degree testing. FRI proofs demonstrate that a committed polynomial has bounded degree — the foundation for all [[BBG]] operations.

### storage proofs

[[radio]] provides storage proofs: cryptographic evidence that content remains retrievable. a storage proof verifies that a [[particle]]'s content hash matches its claimed data, using [[Hemera]] Merkle trees for verified streaming. storage proofs guarantee content availability for rehashing and replication across the network.

## consensus proofs

[[cyber]] uses [[proof of stake]] via [[tendermint]] for block production. the broader landscape:

| mechanism | what it proves | energy cost | assumption |
|---|---|---|---|
| [[proof of work]] | computational effort expended | high | honest majority (51%) |
| [[proof of stake]] | economic commitment at risk | low | honest majority (67%) |
| STARK execution proof | computation ran correctly | minimal | hash collision resistance |

[[cyber]] layers STARK execution proofs on top of [[proof of stake]] consensus. validators produce blocks (PoS), and every state transition within those blocks carries a STARK proof of correct execution. the combination: economic security from stake, computational integrity from proofs.

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

## the proof stack

```
┌─────────────────────────────────────────────────────────┐
│  epistemological    proof by simulation (cybics)         │
│                     convergence → fixed point → truth    │
├─────────────────────────────────────────────────────────┤
│  application        identity, delivery, inference,       │
│                     anonymity, storage, range, ownership │
├─────────────────────────────────────────────────────────┤
│  recursive          proof aggregation, composition       │
│                     O(1) verification for O(N) proofs    │
├─────────────────────────────────────────────────────────┤
│  proof system       STARK (transparent, post-quantum)    │
│                     ~70,000 constraints with jets        │
├─────────────────────────────────────────────────────────┤
│  primitives         Hemera (hash), nox (VM),             │
│                     Goldilocks field (arithmetic)        │
└─────────────────────────────────────────────────────────┘
```

one hash. one VM. one field. one proof system. every proof in [[cyber]] — from a single [[cyberlink]] to a chained delivery receipt to a trillion-parameter neural network inference — reduces to: run a [[nox]] program, produce a [[STARK]], verify with [[Hemera]].

see [[cyber/identity]] for authentication and anonymity, [[cyber/communication]] for delivery proofs, [[BBG]] for polynomial commitment architecture, [[trident]] for verifiable AI, [[cybics]] for proof by simulation, [[cyber/security]] for formal guarantees
