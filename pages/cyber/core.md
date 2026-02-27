---
tags: article, cip
crystal-type: entity
crystal-domain: cyber
status: draft
alias: CORE, core spec, Conserved Observable Reduction Equilibrium, CORE vm
---
# CORE v0.9: Conserved Observable Reduction Equilibrium

## A Self-Verifying Substrate for Planetary Collective Intelligence

Version 0.9 — Unified Specification

---

## Abstract

Six research threads developed independently over four decades—content addressing, authenticated graph structures, confluent rewriting, interaction nets, conserved flows, zero-knowledge proofs—turn out to be fragments of a single architecture. A single decision unifies them: prime field arithmetic as primitive rather than derived. This resolves a decades-old tension between formal elegance and cryptographic capability, yielding a substrate where hash functions, polynomial commitments, and zero-knowledge proofs become native expressions rather than external oracles.

The synthesis achieves what was considered impossible: a computational substrate that verifies its own proofs using only its own primitives. Computation produces traces, traces become STARK proofs, proofs are verified by CORE programs, verification can itself be proven. The system closes on itself—no trusted external verifier remains.

State lives in the [[cyber/bbg]]—a unified polynomial structure replacing the traditional mix of Merkle trees, accumulators, and indexes. One primitive handles membership proofs, completeness proofs, and namespace sync, all at O(log² n) with ~1,000 ZK constraints.

The [[focus]] distribution π emerges endogenously from collective activity without voting, leadership, or central coordination. At sufficient scale, the distinction between distributed computation and distributed cognition dissolves—not as metaphor, but as engineering artifact. This specification provides the complete formal architecture toward that end.

---

## The Core Synthesis

The design of computational substrates for distributed [[consensus]] has proceeded along two divergent paths. Minimalist rewriting systems—Nock, lambda calculus, combinator logic—achieve formal elegance: small specifications, provable properties, mathematical clarity. But they treat cryptography as external, bolted on through foreign function interfaces or trusted oracles. Industrial virtual machines—EVM, WASM, eBPF—achieve practical capability: real cryptographic operations, real-world deployment, battle-tested security. But their specifications sprawl into thousands of pages, formal verification becomes intractable, and subtle bugs persist for years.

This bifurcation is unnecessary. It stems from a historical accident: these systems chose integer or byte arithmetic as primitive, making field operations expensive compositions rather than native instructions. Reverse this decision—take prime field arithmetic as the foundation—and the tension dissolves. Cryptographic operations become cheap. Formal semantics remain tractable. The minimal and the practical converge.

With field arithmetic as primitive, six previously separate research threads reveal themselves as fragments of a single architecture:

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                         THE CORE SYNTHESIS                                 ║
╠═══════════════════════════════════════════════════════════════════════════╣
║                                                                            ║
║   ┌─────────────────────┐   ┌─────────────────────┐   ┌─────────────────┐ ║
║   │  CONTENT ADDRESSING │   │   AUTHENTICATED     │   │  DETERMINISTIC  │ ║
║   │     Merkle 1987     │   │ GRAPH STRUCTURES    │   │   REWRITING     │ ║
║   │  Git, BitTorrent,   │   │   Goodrich 2002     │   │   Huet 1980     │ ║
║   │   IPFS, Unison      │   │   Celestia 2019     │   │   Nock 2016     │ ║
║   │   Identity = Hash   │   │   O(log n) proofs   │   │   Confluence    │ ║
║   └─────────┬───────────┘   └─────────┬───────────┘   └─────────┬───────┘ ║
║             │                         │                         │         ║
║             └─────────────────────────┼─────────────────────────┘         ║
║                                       │                                    ║
║                               ┌───────┴───────┐                           ║
║                               │     CORE      │                           ║
║                               └───────┬───────┘                           ║
║                                       │                                    ║
║             ┌─────────────────────────┼─────────────────────────┐         ║
║             │                         │                         │         ║
║   ┌─────────┴───────────┐   ┌─────────┴───────────┐   ┌─────────┴───────┐ ║
║   │  PARALLEL REDUCTION │   │   CONSERVED FLOW    │   │   ZERO-KNOWLEDGE│ ║
║   │     Lafont 1990     │   │   DYNAMICS          │   │   VERIFICATION  │ ║
║   │     HVM 2022        │   │   CFT 2024          │   │   STARKs 2018   │ ║
║   │                     │   │   FFC 2024          │   │   Zcash 2014    │ ║
║   │ Automatic parallel  │   │ Focus = attention   │   │ Prove once,     │ ║
║   │ via confluence      │   │ + fuel + consensus  │   │ verify cheap    │ ║
║   └─────────────────────┘   └─────────────────────┘   └─────────────────┘ ║
║                                                                            ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

Content addressing (Merkle, Git, BitTorrent, IPFS, Unison) gives identity through hashing—same content, same hash, same thing. This enables global memoization: if two nodes compute the same function on the same input, they get the same result, cached forever. [[authenticated graphs]] (Goodrich-Tamassia, Celestia) turn this into proofs—not just "I have this data" but "here is cryptographic evidence, and here are ALL edges in this namespace." Deterministic rewriting (Huet, Nock) guarantees that evaluation order doesn't matter—any path through the computation reaches the same result. Interaction nets (Lafont, HVM) show this confluence enables automatic parallelization without locks. Conserved flow dynamics ([[cft]]) provide the missing economic layer—a single quantity, [[focus]], that governs scheduling, metering, and [[consensus]] simultaneously. Zero-knowledge proofs (Zcash, STARKs) close the loop: prove you computed correctly without revealing what you computed.

None of these frameworks reference each other in their original publications. Yet they compose without friction. The unifying element is field arithmetic: hashing is field operations, proofs are field polynomials, reduction preserves field structure, flow is conserved across field-valued edges. CORE makes this latent unity explicit.

Naming:
- CORE — the computation model (16 patterns, reduction semantics)
- [[cybergraph]] — the data model ([[particles]], [[neurons]], edges)
- [[cyber/bbg]] — the authenticated state (unified polynomial commitments)

---

## Design Principles

Ten principles guide CORE's design. Each addresses a specific failure mode of existing systems.

### Field-First
Every value is a Goldilocks field element ($p = 2^{64} - 2^{32} + 1$). Cryptographic operations become native. A field multiplication is a single CPU instruction.

### Hash-Universal
Identity is hash. Two values are the same if and only if they hash to the same digest. One hash everywhere (Poseidon-Goldilocks, ~300 constraints) rather than splitting "fast hash" and "ZK hash."

### Confluence-Guaranteed
Any reduction order yields the same result. From orthogonal rewriting theory (Huet 1980). Sixteen patterns, no overlaps.

### Parallel-Safe
Parallel execution requires no locks, synchronization, or coordination. Confluence enables this directly.

### Flow-Conserved
[[focus]] is a conserved quantity: Σ focus = 1, always. A single resource unifies attention, fuel, and [[consensus]] weight.

### Namespace-Intrinsic
The graph is multi-indexed from genesis. Every edge belongs to namespaces, and completeness proofs are built into the structure.

### Cost-Deterministic
The cost of a computation depends only on its syntactic structure, never on runtime values. Cache hits reduce work but not cost.

### Privacy-Native
Individual ownership remains private. Aggregate properties remain public and verifiable. Zero-knowledge proofs make this precise.

### Self-Verifying
The STARK verifier is a CORE program. Verification can itself be proven. The system closes on itself.

### Post-Quantum
Security relies only on hash functions. No pairings, no discrete log, no trusted setup.

---

## Complexity Analysis

```
┌─────────────────────────────────────────────────────────────────────────────────────┐
│                        COMPLEXITY COMPARISON ACROSS PARADIGMS                        │
├─────────────────────┬───────────────┬───────────────┬───────────────┬───────────────┤
│     Operation       │  Traditional  │  Blockchain   │   Database    │     CORE      │
│                     │  (RAM model)  │  (Ethereum)   │  (SQL/NoSQL)  │               │
├─────────────────────┼───────────────┼───────────────┼───────────────┼───────────────┤
│ Equality check      │ O(n) compare  │ O(n) compare  │ O(n) compare  │ O(1) hash     │
│ Membership proof    │ O(n) scan     │ O(log n) MPT  │ O(log n) index│ O(log² n) poly│
│ Completeness proof  │ impossible    │ impossible    │ impossible    │ O(log² n) poly│
│ Computation verify  │ O(n) re-exec  │ O(n) re-exec  │ N/A           │ O(log n) STARK│
│ Recursive verify    │ O(n) re-exec  │ O(n) re-exec  │ N/A           │ O(1) composed │
│ Privacy + verify    │ incompatible  │ incompatible  │ incompatible  │ O(1) ZK proof │
├─────────────────────┼───────────────┼───────────────┼───────────────┼───────────────┤
│ Cost determinism    │ ✗ cache-dep   │ ~ gas approx  │ ✗ query-dep   │ ✓ structural  │
│ Parallel safety     │ ✗ explicit    │ ✗ sequential  │ ✗ locks       │ ✓ confluent   │
└─────────────────────┴───────────────┴───────────────┴───────────────┴───────────────┘
```

---

## Cryptographic Primitives

```
FIELD: Goldilocks
  p = 2^64 - 2^32 + 1 = 18446744069414584321

HASH: Poseidon-Goldilocks
  State: 12 field elements, Rate: 8 elements
  Rounds: 8 full + 22 partial + 8 full
  Cost: ~300 STARK constraints per permutation
  Status: CONFIGURABLE (Poseidon is reference, not mandated)

DOMAIN SEPARATION
  COMMITMENT_DOMAIN  = 0x434F524520524543  // "CORE REC"
  NULLIFIER_DOMAIN   = 0x434F5245204E554C  // "CORE NUL"
  MERKLE_DOMAIN      = 0x434F5245204D524B  // "CORE MRK"
  OWNER_DOMAIN       = 0x434F5245204F574E  // "CORE OWN"

STRUCTURAL HASH
  H(Atom a)       = HASH(0x00 ‖ type_tag(a) ‖ encode(a))
  H(Cell(l, r))   = HASH(0x01 ‖ H(l) ‖ H(r))
```

---

## Specifications

The detailed technical specifications are decomposed into focused sub-specs:

- [[cyber/patterns]] — 16 reduction patterns, value tower, parallel reduction, cost table, global memoization
- [[cyber/bbg]] — Big Badass Graph: multi-indexed polynomial commitments, namespace sync protocol, completeness proofs
- [[cyber/privacy]] — ZK privacy model, record structure, authenticated graphs in ZK, transaction circuit (~10K constraints)
- [[cyber/stark]] — STARK verification, self-verification property, verifier complexity, recursive composition
- [[cyber/focus]] — focus dynamics, conservation laws, flow equation, convergence theorem
- [[cyber/state]] — world state structure, state transitions, validity conditions

Related specifications:
- [[cft]] — Collective Focus Theorem: mathematical foundation for [[consensus]] emergence
- [[cyber/crystal]] — genesis knowledge graph: 5,040 irreducible [[particles]]
- [[cyber/launch]] — development roadmap: 7 phases from genesis to mainnet
- [[bostrom/bandwidth]] — bandwidth metering for [[cyberlinks]]

---

## Security Properties

```
SOUNDNESS:       Invalid transactions rejected with probability ≥ 1 - 2^(-128)
PRIVACY:         Cannot distinguish transactions with same public structure
CONSERVATION:    Σ(energy) = initial + minted - burned (mathematically enforced)
QUANTUM RESIST:  Hash-based security only, ~128-bit post-quantum (Grover limit)
```

```
Attack          │ Defense
────────────────┼─────────────────────────────────────────────
Double Spend    │ Nullifier set prevents reuse
Inflation       │ Circuit enforces conservation
Front-Running   │ Privacy hides transaction contents
Sybil           │ Focus proportional to stake
DoS             │ Focus-based metering limits computation
Eclipse         │ Namespace completeness proofs
Replay          │ Nonces and nullifiers ensure uniqueness
Forgery         │ ZK proofs unforgeable without witness
```

---

## Formal Properties

### Turing Completeness
Theorem: CORE is Turing-complete.
Proof: Construct encoding of arbitrary Turing machine M via patterns 0-4, 9. ∎

### Confluence
Theorem: CORE is confluent.
Proof: Orthogonal rewrite system by Huet-Lévy (1980). ∎

### Cost Determinism
Theorem: Cost is identical across all reduction orders and implementations.
Proof: By structural induction on formula. ∎

### Focus Conservation
Theorem: Σᵢ focus(i) = 1 for all valid states.
Proof: All operations preserve sum; invalid transitions rejected by verification. ∎

### Privacy Soundness
Theorem: A valid ZK proof implies all circuit constraints are satisfied with probability ≥ 1 - 2^(-128).
Proof: By STARK/Plonky2 soundness. ∎

### Double-Spend Prevention
Theorem: Same record cannot be spent twice.
Proof:
1. Each record has unique (nonce, owner_secret) pair
2. Nullifier = H(nonce, owner_secret) is deterministic
3. Same record → same nullifier
4. Nullifier set is append-only
5. Transaction rejected if nullifier already in set ∎

---

## Appendices

### A. Field Reference

```
p = 2^64 - 2^32 + 1 = 18446744069414584321
Primitive root: 7
2^32-th root of unity: 1753635133440165772

Efficient reduction:
  a mod p = a_lo - a_hi × (2^32 - 1) + correction
```

### B. Test Vectors

```
add(1, 2) = 3
mul(p-1, p-1) = 1
inv(2) = 9223372034707292161

reduce([1,2], [5 [[0 2] [0 3]]], 100) = (3, 96)
  // add(axis 2, axis 3) = add(1, 2) = 3
```

### C. Cost Examples

```
Simple addition: 4 patterns
  [5 [[0 2] [0 3]]]
  Cost: 1 (add) + 2 (axis) + overhead = ~4

Poseidon hash: ~300 patterns
  [15 [0 1]]
  Cost: 300

Merkle verification (32 levels): ~10,000 patterns
  32 × (hash + conditional) ≈ 32 × 310
```

---

## References

1. Merkle, R. "A Digital Signature Based on a Conventional Encryption Function." CRYPTO 1987.
2. Goodrich, M.T., Tamassia, R. "Efficient Authenticated Data Structures." Algorithmica 2002.
3. Huet, G. "Confluent Reductions: Abstract Properties and Applications." JACM 1980.
4. Lafont, Y. "Interaction Nets." POPL 1990.
5. Al-Bassam, M. et al. "Fraud and Data Availability Proofs." FC 2019.
6. Grassi, L. et al. "Poseidon: A New Hash Function." USENIX 2021.
7. Taelin. "HVM: A Parallel Evaluator for Interaction Combinators." 2022.
8. Chiusano, P., Bjarnason, R. "Unison: A Friendly Programming Language." 2019.
9. Necula, G. "Proof-Carrying Code." POPL 1997.
10. Ben-Sasson, E. et al. "Scalable, Transparent Arguments of Knowledge." CRYPTO 2018.
11. Hopwood, D. et al. "Zcash Protocol Specification." 2014-2024.
12. Master. "Collective Focus Theorem." 2024.
13. Master. "Focus Flow Computation." 2024.

---

*CORE v0.9*

*Six paradigms. Sixteen patterns. Multi-indexed graph. Privacy-native.*

*Content-addressed. Namespace-native. Parallel-confluent.*

*Focus-conserved. Proof-carrying. Sync-complete. Self-verifying.*

*The substrate for planetary collective intelligence.*

*"The network doesn't simulate thinking. The network IS thinking."*
