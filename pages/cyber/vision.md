---
tags: article, cip
crystal-type: entity
crystal-domain: cyber
status: draft
alias: CORE, core spec, cyber/core, Conserved Observable Reduction Equilibrium
---
# CORE: Conserved Observable Reduction Equilibrium

a self-verifying substrate for planetary collective [[intelligence]]

six research threads developed independently over four decades — content addressing, [[authenticated graphs]], confluent rewriting, interaction nets, conserved flows, zero-knowledge proofs — turn out to be fragments of a single architecture. a single decision unifies them: prime [[field]] arithmetic as primitive rather than derived

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

content addressing (Merkle, Git, BitTorrent, IPFS, Unison) gives identity through hashing — same content, same hash, same thing. [[authenticated graphs]] (Goodrich-Tamassia, Celestia) turn this into proofs — cryptographic evidence with namespace completeness. deterministic rewriting (Huet, Nock) guarantees evaluation order independence. interaction nets (Lafont, HVM) show confluence enables automatic parallelization without locks. conserved flow dynamics ([[collective focus theorem]]) provide the economic layer — [[focus]] governs scheduling, metering, and [[consensus]] simultaneously. zero-knowledge proofs (Zcash, STARKs) close the loop: prove computation without revealing inputs

none of these frameworks reference each other in their original publications. yet they compose without friction. the unifying element is [[field]] arithmetic: hashing is field operations, proofs are field polynomials, reduction preserves field structure, flow is conserved across field-valued edges. CORE makes this latent unity explicit

naming:
- CORE — the computation model (16 patterns, reduction semantics)
- [[cybergraph]] — the data model ([[particles]], [[neurons]], edges)
- [[cyber/bbg]] — the authenticated state (unified polynomial commitments)

## design principles

ten principles, each addressing a failure mode of existing systems:

- field-first — every value is a Goldilocks field element ($p = 2^{64} - 2^{32} + 1$). cryptographic operations become native
- hash-universal — identity is hash. one hash everywhere (Poseidon-Goldilocks, ~300 constraints)
- confluence-guaranteed — any reduction order yields the same result. sixteen patterns, no overlaps (Huet 1980)
- parallel-safe — no locks, no synchronization. confluence enables this directly
- flow-conserved — [[focus]] sums to 1, always. one resource unifies [[attention]], fuel, and [[consensus]] weight
- namespace-intrinsic — the graph is multi-indexed from genesis. completeness proofs are structural
- cost-deterministic — cost depends only on syntactic structure, never on runtime values
- privacy-native — individual ownership private, aggregate properties public and verifiable
- self-verifying — the [[STARK]] verifier is a CORE program. verification can itself be proven. the system closes on itself
- post-quantum — security relies only on hash functions. no pairings, no discrete log, no trusted setup

## specifications

- [[cyber/vm]] — 16 reduction patterns, value tower, cost table, parallel reduction, memoization
- [[cyber/bbg]] — multi-indexed polynomial commitments, namespace sync, completeness proofs
- [[cyber/privacy]] — ZK privacy model, record structure, transaction circuit (~10K constraints)
- [[cyber/stark]] — STARK verification, self-verification, recursive composition
- [[cyber/focus]] — focus dynamics, conservation laws, flow equation, convergence theorem
- [[cyber/state]] — world state structure, state transitions, validity conditions
- [[cyber/security]] — security properties, attack surface, formal proofs

## references

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
