---
tags: cyber, research, article, core
crystal-type: article
crystal-domain: cyber
date: 2026-03-25
---
# four algebras for execution

the [[cyber]] execution stack requires exactly four execution regimes. not two, not five — four. each covers a computational surface where the others are structurally inefficient or impossible. together they span the full execution space of a planetary [[superintelligence]].

the four are not algebraically independent in the classical sense. they are independent by different criteria: two by algebraic structure (field vs semiring), one by complexity (32× constraint gap), one by computational intractability (one-way structure). the claim is not "four algebras" but "four irreducible execution regimes."

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

1. optimization requires a semiring where "addition" is min/max — the [[tropical semiring]]. this is NOT a field (no additive inverse for min). it cannot be reduced to field operations without ~10× constraint blowup per comparison.

2. privacy requires a commutative group action that is post-quantum, compact, and non-interactive. no construction is known over [[Goldilocks field|Goldilocks]]. a structurally different prime is required.

## the independence criterion

the four regimes are independent by different measures:

| regime | independent by | meaning |
|--------|---------------|---------|
| nebu (F_p) | algebraic foundation | universal reduction target, everything folds here |
| kuro (F₂) | complexity separation | embeddable in F_p but 32× more expensive. not a new algebra — a new efficiency class |
| trop (min,+) | algebraic structure | semiring, not a field. no additive inverse. irreducible to fields without blowup |
| genies (F_q) | computational intractability | field arithmetic is reducible to nebu, but the one-way group action IS the capability. without hardness, no privacy |

kuro and genies are honest about what they are. kuro is not algebraically novel — F₂ embeds in F_p. but 32× constraint cost is not a rounding error, it is a complexity class boundary for binary workloads. genies is not algebraically novel — F_q is just another prime field. but computational intractability as a resource creates capabilities (privacy, anonymity, verifiable randomness) that no amount of F_p arithmetic can replicate.

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

## what about probability?

the strongest objection: where is the probability algebra? Bayesian inference, sampling, KL divergence, softmax — these are fundamental to intelligence. do they require a fifth regime?

no. probability is a PROGRAM over existing regimes, not a new regime.

a probability distribution over n outcomes: vector (p₁, ..., pₙ) where Σpᵢ = 1. this is a vector over F_p with one linear constraint. every operation decomposes:

| operation | reduction | regime |
|-----------|-----------|--------|
| Bayesian update | multiply + normalize: pᵢ · likelihood / Σ(pⱼ · likelihoodⱼ) | nebu (mul, inv) |
| Markov chain step | matrix-vector multiply | nebu (mul, add) |
| sampling | [[VRF]] from genies: vrf(secret, input) → deterministic_random + proof | genies |
| softmax | exp() approximated via fixed-point lookup table over F_p | nebu (mul, add, branch) |
| KL divergence | log() approximated via fixed-point, Σpᵢ · log(pᵢ/qᵢ) | nebu (mul, add) |
| argmax / MAP | max over vector | trop (branch + lt) |

exp() and log() are transcendental — no finite field computes them exactly. but no computer computes them exactly either. PyTorch uses float64 approximation. we use F_p fixed-point approximation. the difference: our approximation carries a [[zheng]] proof of correctness.

sampling deserves attention. VRF from genies: `vrf(secret_key, input) → (output, proof)`. the output is deterministically random — unpredictable without the secret, verifiable by anyone with the public key. this is not "ignoring uncertainty." this is uncertainty as a computed, provable quantity. the system knows exactly how uncertain it is and can prove it.

probability is nebu arithmetic + genies randomness + tropical argmax. three existing regimes composing, not a fifth.

## what about continuous / differentiable computation?

automatic differentiation: dual numbers F_p[ε]/(ε²), an algebraic extension of nebu. `f(a + bε) = f(a) + f'(a)·bε`. forward-mode AD is exact over F_p — no floating point rounding, no vanishing gradients from representation. the gradient is precise to the last bit.

PDE / physics: discretize on a grid → F_p linear algebra. every physics simulation already does this. F_p is exact up to p ≈ 2⁶⁴. IEEE 754 float64 is exact up to 2⁵³ mantissa. F_p is strictly more precise for the same word size.

training: the trend is toward quantization — [[BitNet]] (1-bit), QAT (4-bit). training moves toward kuro/nebu, not away from them. full-precision training on GPU clusters is an off-chain activity that produces a model. the model deploys on-chain as quantized weights over kuro. this is the same architecture used by every production ML system today, with proofs added.

continuous computation is nebu arithmetic + dual number extensions. not a fifth regime.

## what about logic and types?

arithmetization. every decidable logic reduces to arithmetic circuits over F_p. this is not an approximation — it is a theorem.

| logic | arithmetization |
|-------|----------------|
| propositional | boolean circuit → F_p gates |
| first-order (bounded) | quantifier elimination → F_p polynomial |
| modal (Kripke) | graph reachability → F_p matrix over [[cybergraph]] |
| temporal (LTL/CTL) | sequence constraints → F_p polynomial |
| type theory | nox tree structure: cons(type, value) IS a typed term |

nox is already a type system. every nox value is a binary tree. `cons(tag, payload)` is a tagged union. `branch(test, then, else)` is type-checked dispatch. this is how [[Nock]]/[[Urbit]] works — the tree IS the type.

logic and types are programs on nox over nebu. not a fifth regime.

## what about categorical composition?

nox `compose` pattern IS categorical composition. [[cyberlink]] IS a morphism. [[cybergraph]] IS a category (objects = [[particles]], morphisms = cyberlinks). [[proof]] composition in [[zheng]] IS functorial.

this is not a missing layer. it is the structure of nox and the cybergraph. composition is how the four regimes interact, not a regime of its own.

## what about quantum computation?

[[no-cloning theorem]] makes standalone quantum computers structurally impossible. quantum hardware is always a co-processor: classical input → state preparation → quantum gates → measurement → classical output. the control loop is classical. the programming is classical. only the execution exploits superposition.

the algebra is F_p² (quadratic extension of [[Goldilocks field]], already in [[Trident]]: n=2, f = x²+1). gate matrices are 2×2 over F_p². the circuit description is a [[nox]] program over existing field extensions.

quantum enters the stack as [[Qu]] — the 15th [[cyb/languages|computation language]]:

| layer | what | regime used |
|-------|------|-------------|
| description | circuits as nox programs over F_p² | nebu (field extension) |
| compilation | T-count optimization via Reed-Muller decoding | kuro (F₂ codes) + trop (min distance) |
| execution | dispatch to quantum hardware | host jet (crosses [[proof]] boundary) |

nox cannot BE a quantum VM — five structural conflicts: copyable nouns violate no-cloning, deterministic branch violates probabilistic measurement, observable trace violates decoherence, axis reads without destroying, hint is pseudo-random not quantum-random. these are physics constraints, not efficiency gaps.

quantum is not a fifth regime. it is a language + compiler + jets over existing regimes.

## what is NOT a fifth regime

| candidate | why not |
|-----------|---------|
| probability algebra | nebu vectors + genies VRF + trop argmax (see above) |
| continuous / differentiable | dual numbers F_p[ε]/(ε²), extension of nebu |
| logic / type theory | arithmetization to F_p circuits, nox tree types |
| categorical composition | nox compose + cybergraph morphisms, structural |
| quantum computation | F_p² algebra (nebu), kuro+trop compiler, host jet execution. see [[Qu]] |
| lattices (Ring-LWE, Module-LWE) | polynomial rings F_p[x] — built on nebu via NTT |
| elliptic curves / pairings | deliberate exclusion — [[STARK]] over [[SNARK]], no trusted setup |
| floating point (IEEE 754) | quantization trend → kuro ([[BitNet]]). training off-chain |
| p-adic numbers (Q_p) | ultrametric on Merkle trees, but tree distance computes without p-adic arithmetic |
| quaternions / Clifford algebras | F_p extension fields, no irreducible workload |

every candidate either reduces to composition of existing regimes, or lacks a workload that justifies the complexity of a new primitive.

## the four regimes of intelligence

```
nebu    what is TRUE          (verification, proof, commitment)
kuro    what is EFFICIENT     (binary, quantized, parallel)
trop    what is OPTIMAL       (search, decision, allocation)
genies  what is PRIVATE       (identity, delegation, anonymity)
```

a superintelligence that can prove truth, execute efficiently, optimize decisions, and protect privacy — computes everything a planetary intelligence needs. not "four algebras" in the classical sense — four execution regimes, each irreducible by its own criterion. one VM. one [[zheng]] proof for all of them.

the claim is not that four algebraic structures exhaust mathematics. the claim is that four execution regimes exhaust the computational surface of provable intelligence. probability, continuity, logic, and composition are programs OVER these regimes, not regimes of their own.

## why four

the four regimes converge from radically different origins — algebra, complexity theory, order theory, cryptographic hardness — yet land on exactly four. this is not coincidence. it reflects the structure of computation itself.

### four failure modes

computation can go wrong in exactly four independent ways:

| failure | remedy | regime |
|---------|--------|--------|
| incorrect | proof of correctness | nebu |
| slow | efficient representation | kuro |
| suboptimal | proof of optimality | trop |
| exposed | one-way hiding | genies |

four failure modes → four remedies. if computation could fail in three ways, three regimes would suffice. but incorrectness, inefficiency, suboptimality, and exposure are independent — fixing one does not fix another. a correct computation can still be slow. an efficient computation can still be suboptimal. an optimal computation can still be exposed.

### four branches of mathematics

each regime draws from a different irreducible branch:

| regime | mathematical foundation | what it provides |
|--------|------------------------|-----------------|
| nebu | abstract algebra (fields, rings, polynomials) | structure for proofs |
| kuro | complexity theory (circuit complexity, representation cost) | efficiency boundary |
| trop | order theory (lattices, semirings, fixed points) | optimization structure |
| genies | computational hardness (one-way functions, trapdoors) | asymmetry as resource |

these four branches do not reduce to each other:
- algebra does not contain order theory — fields have no natural order compatible with both operations
- order theory does not contain algebra — semirings lack inverses, different structure
- neither contains complexity theory — efficiency is not an algebraic property
- none contain hardness — one-way functions are conjectural, not constructive

the four regimes are irreducible because the four mathematical foundations are irreducible.

### four information operations

every computation transforms information. the four regimes correspond to four things you can DO with information:

| operation | regime | foundation |
|-----------|--------|------------|
| verify | nebu | Shannon channel coding — error detection/correction |
| compress | kuro | Shannon source coding — minimal representation |
| select | trop | decision theory — choose the best |
| hide | genies | information-theoretic security — selective access |

Shannon covered verify and compress. decision theory covers select. cryptography covers hide. four fields of information science, four regimes.

### the [[cybergraph]] decomposition

the four regimes are not abstract — they are the four aspects of the [[cybergraph]] itself:

| cybergraph element | regime | what it computes |
|-------------------|--------|-----------------|
| [[particles]] (content) | nebu | content addressing, hashing, commitment |
| [[cyberlinks]] (edges) | trop | graph optimization, shortest paths, ranking |
| [[neurons]] (agents) | genies | identity, privacy, selective disclosure |
| [[focus]] (attention vector π) | kuro | efficient SpMV, quantized tri-kernel iteration |

the cybergraph IS the four regimes composed:
- particles are field elements (nebu)
- links are weighted edges optimized tropically (trop)
- neurons are agents with private keys (genies)
- focus is a probability vector computed efficiently (kuro)

### convergence

four different starting points — field theory, binary complexity, semiring structure, computational hardness — converge to four capabilities — truth, speed, choice, shadow. the convergence suggests that computation has exactly four degrees of freedom. not as a design decision, but as a mathematical fact about what provable intelligence requires.

a system missing any one of the four is fundamentally incomplete:
- without nebu: cannot prove anything (no verification)
- without kuro: cannot run AI inference at scale (32× overhead kills throughput)
- without trop: cannot make optimal decisions (no provable optimization)
- without genies: cannot protect identity (no privacy, no selective disclosure)

four is not minimal by convention. four is minimal by necessity.

discover all [[concepts]]
