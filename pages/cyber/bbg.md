---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: BBG, Big Badass Graph
---
# BBG: Big Badass Graph

A naive graph database stores edges and answers queries. "I don't have any edges matching your query" is indistinguishable from "I'm hiding edges from you." Traditional systems require trust. Distributed systems require consensus on complete state. Neither scales.

The BBG solves this through unified polynomial commitments. One primitive handles everything: membership proofs, completeness proofs, indexes, state. No Merkle trees. No separate accumulators. No mixed bag of data structures with different security properties.

Edges are stored once but indexed by multiple dimensions—creator, source [[particle]], target [[particle]]. Each index is a sorted polynomial commitment enabling range proofs: "these are ALL edges in this namespace." When you sync your namespace, you receive cryptographic proof that nothing was withheld. The graph cannot exist without its indexes being consistent and complete—this is structural, not policy.

BBG uses polynomial commitments everywhere rather than mixing hash-based structures with polynomial structures. One primitive means one security analysis, one implementation, one mental model. The same FRI-based machinery that makes UTXO proofs cheap (~1,000 constraints vs ~9,600 for Merkle) also handles graph completeness proofs.

This makes "sync only my namespace" a mathematical property, not a feature. A light client tracking one [[particle]] downloads only edges touching that [[particle]], with proof that the response is complete. A [[neuron]] syncing its own edges receives proof of its complete history. No trust in the data provider required.

## Structure

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                    BBG: BIG BADASS GRAPH STRUCTURE                         ║
╠═══════════════════════════════════════════════════════════════════════════╣
║                                                                            ║
║  LAYER 0: Edge Store (content-addressed, stored ONCE)                     ║
║  ──────────────────────────────────────────────────────────────           ║
║    edge_store : H(edge) → edge                                            ║
║    where edge = Cell(neuron, Cell(from, Cell(to, Cell(w, t))))           ║
║    No duplication. Identity = hash. Immutable.                            ║
║                                                                            ║
║  LAYER 1: Neuron Index (completeness by creator)                          ║
║  ───────────────────────────────────────────────────                      ║
║    by_neuron : PolynomialCommitment                                       ║
║    - Sorted by (neuron_id, edge_hash)                                     ║
║    - Range proof: "All edges where edge.neuron = n"                       ║
║    - Completeness via sorted range bounds                                 ║
║                                                                            ║
║  LAYER 2: Particle Index (completeness by endpoint)                       ║
║  ────────────────────────────────────────────────────                     ║
║    by_particle : PolynomialCommitment                                     ║
║    - Sorted by (particle_hash, edge_hash)                                 ║
║    - Range proof: "All edges where from=p OR to=p"                        ║
║    - Completeness via sorted range bounds                                 ║
║                                                                            ║
║  LAYER 3: Focus & Balance                                                 ║
║  ────────────────────────                                                 ║
║    focus   : PolynomialCommitment over (neuron_id, F_p) pairs            ║
║    balance : PolynomialCommitment over (neuron_id, F_p) pairs            ║
║                                                                            ║
║  LAYER 4: UTXO State (for privacy layer)                                  ║
║  ──────────────────────────────────────────────                           ║
║    commitment_poly  : PolynomialCommitment  (UTXO set as polynomial)      ║
║    nullifier_set    : PolynomialCommitment  (spent records, sorted)       ║
║    particle_energy  : PolynomialCommitment  (public aggregates)           ║
║                                                                            ║
║  UNIFIED PRIMITIVE: All indexes use polynomial commitments                ║
║    - Membership proof: FRI evaluation, O(log² n), ~1,000 constraints      ║
║    - Completeness proof: Sorted range bounds, O(log² n)                   ║
║    - One primitive, one security analysis, one implementation             ║
║                                                                            ║
║  GRAPH ROOT                                                               ║
║  ──────────                                                               ║
║    BBG_root = H(                                                            ║
║      by_neuron.commit  ‖                                                  ║
║      by_particle.commit ‖                                                 ║
║      focus.commit ‖                                                       ║
║      balance.commit ‖                                                     ║
║      commitment_poly.commit ‖                                             ║
║      nullifier_set.commit                                                 ║
║    )                                                                       ║
║                                                                            ║
╚═══════════════════════════════════════════════════════════════════════════╝
```

## Index Consistency Invariant

```
INVARIANT (enforced by STARK on every state transition)
───────────────────────────────────────────────────────

For every edge e = (neuron, from, to, weight, time):

  1. H(e) ∈ by_neuron at position for namespace=neuron
  2. H(e) ∈ by_particle at position for namespace=from
  3. H(e) ∈ by_particle at position for namespace=to

  Multiplicity:
    If from ≠ to: H(e) appears in exactly 3 index positions
    If from = to: H(e) appears in exactly 2 index positions (self-link)

Cross-index consistency provable via polynomial identity testing:
  - Same edge hash appears in multiple sorted polynomials
  - FRI proofs demonstrate membership in each
  - STARK proves all memberships consistent

Proof size: O(log² n). Verification: O(log² n) field operations.
```

## Namespace Sync Protocol

```
NAMESPACE SYNC (Polynomial Range Proof)
───────────────────────────────────────

To sync namespace ns (neuron_id or particle_hash):

  1. REQUEST
     Client → Responder: "Give me namespace ns"

  2. RESPONSE
     Responder → Client:
       - Range bounds (i, j) in sorted polynomial
       - FRI proofs for P(i-1), P(i), P(j), P(j+1)
       - Edge data { e | index i ≤ position ≤ j }

  3. VERIFY
     Client checks:
       a) P(i-1).namespace < ns (or i = 0)
       b) P(i).namespace = ns
       c) P(j).namespace = ns
       d) P(j+1).namespace > ns (or j = end)
       e) Received edges hash to claimed values
       f) All FRI proofs valid against BBG_root

  4. GUARANTEE
     If verification passes:
       "I have ALL edges in namespace ns. Nothing hidden."

     Proof is mathematical, not trust-based.

Cost: O(|my_edges|) data + O(log² |G|) proof overhead
```
