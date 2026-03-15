---
tags: cyber, computer science, cryptography
crystal-type: entity
crystal-domain: computer science
alias: polynomial commitments, polynomial commitment scheme, WHIR polynomial commitment, WHIR polynomial commitments, FRI polynomial commitment, FRI polynomial commitments
---
# polynomial commitment

a cryptographic primitive that allows a prover to commit to a polynomial and later prove evaluations at specific points. in [[cyber]], polynomial commitments use [[WHIR]] over the [[Goldilocks field]] — no trusted setup, no pairing-based curves, hash-only security, sub-millisecond verification. WHIR serves as both a univariate and multilinear PCS.

## the primitive

```
COMMIT:   C = WHIR_commit(P)
          commit to polynomial P
          C = Merkle root of evaluation table

OPEN:     proof = WHIR_open(P, z)
          prove that P(z) = v for a specific point z

VERIFY:   WHIR_verify(C, z, v, proof) → accept/reject
          check the evaluation proof against the commitment
```

WHIR operates in two modes:
- univariate: P(x) of degree ≤ d, used for [[EdgeSet]] membership proofs
- multilinear: P(x₁, ..., x_k) with degree ≤ 1 per variable, used for [[stark]] trace commitments

in the multilinear mode, WHIR commits the entire [[nox]] execution trace as a single polynomial. the [[SuperSpartan]] IOP reduces all AIR constraint checks to one evaluation at one random point, which WHIR opens. see [[stark]] for the full pipeline.

## why polynomial commitments

the [[cybergraph]] needs to prove membership ("this edge belongs to neuron N's edge set") and completeness ("these are ALL edges for neuron N"). polynomial commitments handle both efficiently:

| operation | Merkle tree | polynomial commitment |
|---|---|---|
| membership proof | O(log n) hashes, ~9,600 constraints | O(log² n), ~1,000 constraints |
| batch membership (N elements) | N × O(log n), ~9,600 × N | ~1,000 amortized (sublinear) |
| state root update | O(log n) rehash | O(log n) update |
| completeness proof | impossible (standard Merkle) | requires sorted polynomial + [[NMT]] |

the batch proof advantage is decisive for transaction verification: a single [[cyberlink]] touches 3 [[EdgeSets]], and a block contains thousands of cyberlinks. batched [[WHIR]] openings make this tractable.

## use in cyber

polynomial commitments appear at two levels of the [[BBG]]:

```
Level 1: NMT (Namespaced Merkle Trees)
  → structural completeness: "these are ALL items in namespace N"
  → uses standard Merkle hashing (Hemera)

Level 2: EdgeSets (polynomial commitments via WHIR)
  → efficient membership: "this edge belongs to this namespace's set"
  → batched openings: sublinear cost for multi-edge proofs
```

each NMT leaf contains an [[EdgeSet]] — a [[WHIR]] polynomial commitment to the set of edge hashes belonging to that namespace. the NMT provides completeness guarantees. the polynomial commitment provides efficient membership queries.

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

  EdgeSet commitment: C_N = WHIR_commit(P_N)
```

## one primitive, one security analysis

[[cyber]] uses polynomial commitments everywhere rather than mixing hash-based structures with algebraic structures. one primitive means one security analysis, one implementation, one mental model. the same [[WHIR]]-based machinery that makes UTXO proofs cheap (~1,000 constraints) also handles graph completeness proofs.

see [[WHIR]] for the low-degree testing protocol, [[stark]] for the multilinear stark pipeline, [[EdgeSet]] for edge membership proofs, [[NMT]] for structural completeness, [[BBG]] for the full graph architecture, [[LogUp]] for cross-index consistency
