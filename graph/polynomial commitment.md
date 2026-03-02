---
tags: cyber, computer science, cryptography
crystal-type: entity
crystal-domain: computer science
alias: polynomial commitments, polynomial commitment scheme, FRI polynomial commitment, FRI polynomial commitments
---
# polynomial commitment

a cryptographic primitive that allows a prover to commit to a polynomial and later prove evaluations of that polynomial at specific points. in [[cyber]], polynomial commitments use [[FRI]] over the [[Goldilocks field]] — no trusted setup, no pairing-based curves, hash-only security.

## the primitive

```
COMMIT:   C = FRI_commit(P)
          commit to polynomial P(x) of degree ≤ d
          C = Merkle root of evaluation table

OPEN:     proof = FRI_open(P, z)
          prove that P(z) = v for a specific point z

VERIFY:   FRI_verify(C, z, v, proof) → accept/reject
          check the evaluation proof against the commitment
```

## why polynomial commitments

the [[cybergraph]] needs to prove membership ("this edge belongs to neuron N's edge set") and completeness ("these are ALL edges for neuron N"). polynomial commitments handle both efficiently:

| operation | Merkle tree | polynomial commitment |
|---|---|---|
| membership proof | O(log n) hashes, ~9,600 constraints | O(log² n), ~1,000 constraints |
| batch membership (N elements) | N × O(log n), ~9,600 × N | ~1,000 amortized (sublinear) |
| state root update | O(log n) rehash | O(log n) update |
| completeness proof | impossible (standard Merkle) | requires sorted polynomial + [[NMT]] |

the batch proof advantage is decisive for transaction verification: a single [[cyberlink]] touches 3 [[EdgeSets]], and a block contains thousands of cyberlinks. batched FRI openings make this tractable.

## use in cyber

polynomial commitments appear at two levels of the [[BBG]]:

```
Level 1: NMT (Namespaced Merkle Trees)
  → structural completeness: "these are ALL items in namespace N"
  → uses standard Merkle hashing (Hemera)

Level 2: EdgeSets (polynomial commitments via FRI)
  → efficient membership: "this edge belongs to this namespace's set"
  → batched openings: sublinear cost for multi-edge proofs
```

each NMT leaf contains an [[EdgeSet]] — a [[FRI]] polynomial commitment to the set of edge hashes belonging to that namespace. the NMT provides completeness guarantees. the polynomial commitment provides efficient membership queries.

## EdgeSet construction

```
EdgeSet for neuron N:
  edges = { e | e.neuron = N }
  edge_hashes = { H_edge(e) | e ∈ edges }

  construct polynomial P_N(x) such that:
    P_N(0) = edge_hashes[0]
    P_N(1) = edge_hashes[1]
    ...
    P_N(k-1) = edge_hashes[k-1]

  EdgeSet commitment: C_N = FRI_commit(P_N)
```

## one primitive, one security analysis

[[cyber]] uses polynomial commitments everywhere rather than mixing hash-based structures with algebraic structures. one primitive means one security analysis, one implementation, one mental model. the same [[FRI]]-based machinery that makes UTXO proofs cheap (~1,000 constraints) also handles graph completeness proofs.

see [[FRI]] for the low-degree testing protocol, [[EdgeSet]] for edge membership proofs, [[NMT]] for structural completeness, [[BBG]] for the full graph architecture, [[LogUp]] for cross-index consistency
