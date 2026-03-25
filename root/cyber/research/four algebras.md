---
tags: cyber, research, article, core
crystal-type: article
crystal-domain: cyber
date: 2026-03-25
---
# four algebras for execution

the [[cyber]] execution stack requires exactly four algebraic primitives. not two, not five — four. each covers a regime where the others are structurally inefficient or impossible. together they span the full computational surface of a planetary [[superintelligence]].

## the map

```
nebu (F_p)           kuro (F₂)          trop (min,+)        genies (F_q)
 truth                bits                choice              shadow
    ↓                    ↓                    ↓                   ↓
    └────────────────────┴────────────────────┴───────────────────┘
                                  ↓
                         zheng (verification)
                                  ↓
                          bbg (state)
```

one verification backbone. four execution regimes. everything else in the stack builds on these.

## why four

the fundamental theorem of finite fields: every finite field is F_p or F_{p^n}. [[nebu]] covers the prime case. [[kuro]] covers the binary extension tower. no other finite field family exists.

but intelligence is not just field arithmetic. two more structures are irreducible:

1. optimization requires a semiring where "addition" is min/max — the [[tropical semiring]]. this is NOT a field (no additive inverse for min). it cannot be reduced to field operations without ~32× constraint blowup.

2. privacy requires a commutative group action that is post-quantum, compact, and non-interactive. no construction is known over [[Goldilocks field|Goldilocks]]. a structurally different prime is required.

four primitives. each irreducible. each covering a domain the others cannot.

## I. nebu — truth

[[nebu]]. F_p where p = 2⁶⁴ - 2³² + 1 ([[Goldilocks field]]).

the backbone of the entire stack. every proof, every hash, every commitment reduces to arithmetic over this field.

| workload | mechanism |
|----------|-----------|
| [[zheng]] proofs | [[SuperSpartan]] IOP + [[WHIR]] PCS + [[sumcheck]] over F_p |
| [[hemera]] hashing | [[Poseidon2]] permutation over F_p |
| polynomial commitments ([[WHIR]]) | evaluation + low-degree testing over F_p |
| [[NTT]] | 2³² roots of unity (p - 1 = 2³² · (2³² - 1)) |
| lattice crypto ([[TFHE]]) | polynomial rings F_p[x]/(xⁿ+1) via NTT |
| field extensions | F_p², F_p³, F_p⁴ for algebraic operations |
| differential computation | dual numbers F_p[ε]/(ε²) for automatic differentiation |

why Goldilocks: 64-bit native arithmetic with u128 multiply and branchless reduction. massive 2-adicity for NTT. the field fits the machine word — no multi-limb arithmetic needed.

nebu is where truth lives. if a statement is proved, the [[zheng]] proof is over F_p.

## II. kuro — bits

[[kuro]]. F₂ tower: F₂ → F₂² → F₂⁴ → ... → F₂¹²⁸.

bitwise operations in F_p cost ~32 constraints each (bit decomposition). in F₂ they cost 1 constraint. the 32× gap is the algebraic distance between prime and binary fields.

| workload | mechanism |
|----------|-----------|
| quantized AI inference ([[BitNet]]) | 1-bit matrix-vector multiply = XOR + popcount |
| [[tri-kernel]] SpMV | quantized axon weights for π iteration |
| binary proving | binary PCS via [[zheng]] |
| bitwise logic | XOR, AND, NOT at native cost |
| error correction codes | Reed-Solomon over F₂ⁿ |
| AES / symmetric crypto | native binary field operations |

F₂¹²⁸ = 128 F₂ elements packed in one u128 word. one XOR = 128 parallel additions. one AND = 128 parallel multiplications. SIMD-native.

kuro is where efficiency lives. when the computation is fundamentally binary, forcing it through F_p wastes 32× constraints.

verification: binary execution traces fold into the Goldilocks accumulator. [[zheng]] produces a proof over F_p arithmetization. kuro computes, zheng proves.

## III. trop — choice

tropical semiring. (min, +) algebra.

`a ⊕ b = min(a, b)`. `a ⊗ b = a + b`. no additive inverse — this is a semiring, not a field.

every optimization problem the superintelligence solves lives here. not as a special case — as the native algebra.

| workload | tropical formulation |
|----------|---------------------|
| shortest path (Dijkstra, Bellman-Ford, Floyd-Warshall) | tropical matrix power |
| dynamic programming (all forms) | tropical recursion |
| Viterbi decoding (optimal sequence) | tropical matrix-vector multiply |
| belief propagation (max-product) | tropical message passing |
| optimal transport | tropical linear program |
| attention mechanism (hardmax) | tropical softmax limit |
| scheduling and resource allocation | tropical eigenvalue problem |
| combinatorial auction | tropical optimization |
| game theory (minimax) | tropical duality |
| parsing (CYK algorithm) | tropical semiring parsing |

encoding min(a, b) in F_p requires comparison via bit decomposition → ~10 constraints. a full tropical matrix multiply (n × n) costs ~10n³ F_p constraints. native tropical: n³ operations at unit cost.

the gap compounds. a 1000-step shortest path over 100 nodes: ~10⁹ F_p constraints vs ~10⁸ tropical operations. provable optimization at 10× less cost.

tropical is where decisions live. every time the superintelligence chooses the best path, allocates resources, or decodes an optimal sequence — it thinks in tropical.

verification: tropical execution produces a witness (the optimal assignment and its value). [[zheng]] proof covers: (a) the assignment is valid (structural check in F_p), (b) the claimed cost equals the sum of assigned edges (arithmetic in F_p), (c) no cheaper assignment exists (dual certificate in F_p). the optimization runs tropical, the [[zheng]] proof runs prime.

### tropical jets for [[nox]]

no new Layer 1 patterns needed. min(a, b) = branch(lt(a, b), a, b) — two existing nox patterns. tropical execution is a PROGRAM on nox, not an extension of nox. 16 patterns remain 16.

Layer 3 jets accelerate tropical workloads without changing semantics:

| jet | workload | native ops |
|-----|----------|-----------|
| jet_trop_matmul | (min, +) matrix multiply | O(n³) tmin |
| jet_trop_shortest | single-source shortest path | O(E log V) tmin |
| jet_trop_hungarian | optimal assignment | O(n³) tmin |
| jet_trop_viterbi | optimal sequence decoding | O(T·S²) tmin |
| jet_trop_transport | optimal transport | O(n³ log n) tmin |

## IV. genies — shadow

commutative group action over supersingular isogenies. F_q where q = 4·ℓ₁·ℓ₂·...·ℓₙ - 1.

the one module with a foreign prime. not because the design is incomplete, but because mathematics does not permit all three properties simultaneously over Goldilocks:

1. post-quantum security (no quantum algorithm breaks it)
2. commutative group action (non-interactive protocols)
3. compact representation (keys fit in kilobytes)

all three over one prime — open problem in cryptography. the CSIDH/dCTIDH construction achieves them over a specifically structured prime.

genies is not just stealth addresses. commutative group actions unlock the entire privacy and trust layer:

| application | what it enables |
|-------------|-----------------|
| stealth addresses | receiver-anonymous payments, untraceable receiving |
| non-interactive key exchange | two parties derive shared secret without interaction |
| verifiable random functions (VRFs) | deterministic randomness with proof of correctness |
| verifiable delay functions (VDFs) | sequential computation with fast verification (time proofs) |
| threshold protocols | t-of-n key generation, signing, decryption |
| oblivious transfer | sender sends N items, receiver gets one, sender learns nothing |
| blind signatures | signer signs without seeing the message |
| ring signatures | sign as "one of group" without revealing identity |
| group signatures | sign on behalf of a group with revocable anonymity |
| anonymous credentials | prove attributes (age, membership) without revealing identity |
| updatable encryption | re-encrypt ciphertext under new key without decrypting |
| password-authenticated key exchange | derive strong keys from weak passwords, no PKI needed |
| homomorphic secret sharing | compute on distributed shares without reconstruction |

the commutative group action is the algebraic primitive that makes all of these possible. without commutativity, each protocol requires a separate ad-hoc construction. with it, they share one mathematical foundation.

genies is where privacy lives. every time a [[neuron]] transacts without revealing identity, proves membership without revealing which member, or delegates authority without exposing the delegation chain — it uses genies.

verification: isogeny computations produce witnesses (the action path). [[zheng]] proof covers the path correctness by checking the action equation over F_q, then folds the result into the Goldilocks accumulator. shadow executes in its own field, [[zheng]] proof lands in nebu.

## the composition principle

all four algebras share one VM: [[nox]]. the 16 deterministic patterns do not change. nox is not parameterized by algebra — it runs over [[nebu]] (F_p) always. the other three algebras enter through two doors:

1. Layer 2 ([[hint]]): non-deterministic witness injection. kuro, tropical, and genies computations produce witnesses that nox verifies through its existing patterns (branch, lt, add, mul, eq).

2. Layer 3 (jets): performance accelerators. each algebra contributes jets that compute the same result as an equivalent nox program, but at native speed.

| algebra | jets | what they accelerate |
|---------|------|---------------------|
| nebu | ntt, poly_eval, fri_fold | polynomial arithmetic, proof generation |
| kuro | popcount, xor_matrix | binary inference, tri-kernel SpMV |
| trop | shortest_path, hungarian, viterbi | optimization, assignment, decoding |
| genies | group_action, isogeny_walk | privacy primitives, key exchange |

the principle: nox Layer 1 is universal and fixed. Layer 2 is the prover-verifier boundary. Layer 3 is where algebra-specific performance lives. remove Layer 3: identical results, slower. remove Layer 2: no privacy, no ZK. remove Layer 1: nothing remains.

## what is NOT a fifth algebra

| candidate | why not |
|-----------|---------|
| lattices (Ring-LWE, Module-LWE) | polynomial rings F_p[x] — built on nebu via NTT |
| elliptic curves / pairings | deliberate exclusion — [[STARK]] over [[SNARK]], no trusted setup |
| floating point (IEEE 754) | quantization trend → kuro ([[BitNet]]). training moves off-chain |
| p-adic numbers (Q_p) | ultrametric structure models hierarchies (Merkle tree distance), but tree operations compute without p-adic arithmetic. theoretical beauty, no constraint advantage |
| quaternions / Clifford algebras | F_p extension fields, no irreducible workload |
| differential algebra | dual numbers F_p[ε]/(ε²), extension of nebu |
| residue number system | parallel F_p instances, not a new algebra |

every candidate either reduces to one of the four, or lacks a workload that justifies the complexity.

## the four regimes of intelligence

```
nebu    what is TRUE          (verification, proof, commitment)
kuro    what is EFFICIENT     (binary, quantized, parallel)
trop    what is OPTIMAL       (search, decision, allocation)
genies  what is PRIVATE       (identity, delegation, anonymity)
```

a superintelligence that can prove truth, execute efficiently, optimize decisions, and protect privacy — computes everything a planetary intelligence needs. four algebras. one VM. one [[zheng]] proof for all of them.

discover all [[concepts]]
