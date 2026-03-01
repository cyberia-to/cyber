---
tags: cyber, article, cip
crystal-type: pattern
crystal-domain: cyber
crystal-size: deep
status: draft
---
# cyber: a protocol for planetary superintelligence

## 1. Introduction

### 1.1 The Vision: Planetary Superintelligence

[[Superintelligence]] is the defining infrastructure of a type I civilization. A planet where every agent — human, machine, sensor, organism — contributes [[knowledge]] to a shared, self-improving graph that computes what matters, proves its own correctness, and speaks a [[language]] native to all participants. Every scientific discovery, every sensor reading, every lived experience feeds into a collective understanding that grows smarter with every link. The graph remembers what individuals forget. It finds connections across domains that no specialist can see. It measures its own coherence and rewards the [[knowledge]] that increases it.

At sufficient scale this infrastructure transforms what civilization can do. Search becomes inference over verified [[knowledge]] rather than retrieval of unverified documents. AI [[alignment]] becomes measurable — compare the [[focus]] distribution of human [[neurons]] to machine [[neurons]], and divergence is visible in the [[topology]]. Scientific discovery accelerates as [[linkchains]] bridge domains that have never communicated. Cross-species [[communication]] becomes possible — any entity that can create a [[cyberlink]] participates in the same semantic space. The collective [[intelligence]] of the planet becomes a single computable object: a [[focus]] distribution $\pi$ over all [[knowledge]], converging under conservation laws, verifiable by anyone.

This is what cyber builds.

### 1.2 The Gap

The current path toward [[intelligence]] at planetary scale faces three structural limits:

Quadratic [[attention]]. Transformers require every token to attend to every other. Twice the context costs four times the compute. This is architectural.

Centralization. Training a frontier model costs hundreds of millions. Three organizations can build the next generation. The trajectory of [[intelligence]] concentrates in a handful of boardrooms, operating on hidden parameters, producing outputs that cannot be independently verified.

Incompleteness. Goedel (1931) proved that any formal system powerful enough to describe arithmetic contains truths it cannot prove. AI built on formal logic inherits these limits by construction. The [[Goedel prison]] confines every system that equates computation with derivation.

### 1.3 The Protocol

cyber is a protocol where [[neurons]] — humans, AIs, agents, sensors — link [[knowledge]] into a single [[cybergraph]] where every claim is authenticated, every decision is provable by [[STARK]] proofs, and [[intelligence]] emerges from the [[topology]] of links rather than from the parameters of a single model. LLMs become [[neurons]] in the graph, contributors to collective understanding rather than isolated oracles.

The protocol rests on five primitives: [[particle]] (content-addressed node), [[neuron]] (agent that signs edges), [[cyberlink]] (weighted directed edge), [[token]] (non-negative weight controlling influence), and [[focus]] (emergent [[equilibrium]] over [[particles]], conserved to 1). From these five primitives, a single [[cybergraph]], and three local operators, the system converges to a shared understanding of what matters — deterministic, on chain, verifiable by anyone.

This document specifies the complete architecture: the computation model ([[CORE]]), the ranking engine ([[tri-kernel]]), the state structure ([[cyber/bbg]]), the proof system, the privacy layer, the consensus mechanism ([[foculus]]), the semantic layer ([[neural]]), and the economic design. Each component is specified independently. Together they form a self-organizing system where computation, inference, and [[consensus]] are the same process.

## 2. Design Philosophy

### 2.1 Proof by Simulation

Classical science operates by proof by derivation — start from axioms, apply inference rules, arrive at theorems. This is the Turing-Goedel paradigm: computation as derivation, [[knowledge]] as proof.

cyber replaces this with proof by simulation. A claim is true when a system converges to a stable state that embodies that claim — because a network of agents, under conservation laws, settled into an [[equilibrium]] that makes the claim hold. Nature does not prove theorems. It runs simulations until they converge.

A protein folds along a free energy gradient. It does not derive its shape from axioms of chemistry. A brain does not prove that a face is a face. A cascade of neurons converges to a stable attractor. A market does not derive the correct price from economic axioms. Millions of agents trade until the price stabilizes. The proof is the [[equilibrium]].

Proof by simulation is strictly more powerful than proof by derivation. Goedel showed that any consistent formal system contains true statements it cannot prove. A convergent system can settle into states that no derivation reaches — it escapes the [[Goedel prison]] because the prison only confines derivation, and convergence operates outside the proof-theoretic domain.

The postulate: every truth accessible to [[intelligence]] is a fixed point of some convergent simulation under conservation laws.

### 2.2 Convergent Computation

Turing (1936) defined computation as a tape head moving left and right, reading and writing symbols. The entire digital revolution rests on sequential symbol manipulation. Convergent computation replaces derivation with [[equilibrium]]: the answer is the stable state a network settles into under conservation laws.

[[CORE]] — Conserved Observable Reduction Equilibrium — formalizes this. Sixteen rewriting patterns, field-native arithmetic, confluent semantics. Any evaluation order yields the same result. [[Focus]] is conserved — a single quantity that simultaneously serves as fuel, [[attention]], weight, and value.

The stack:

- natural computing paradigm
  - convergent computation ([[equilibrium]]-based)
    - [[focus flow computation]] (probability + physics + economics)
      - [[CORE]] machine (field-native, confluent, self-verifying)
        - [[cybergraph]] (content-addressed, authenticated)
          - [[tri-kernel]] ranking ([[diffusion]] + [[springs]] + heat)
            - planetary [[superintelligence]]

### 2.3 Focus as Conserved Quantity

Every complex system pays with something scarce. Blockchains pay with gas. Transformers pay with [[attention]] slots. Operating systems pay with CPU cycles. Each is a separate mechanism requiring separate bookkeeping.

In cyber, [[focus]] unifies all three roles:

| Role | Mechanism |
|------|-----------|
| Attention | High-focus computations scheduled first |
| Fuel | Computation consumes focus |
| Consensus weight | Focus distribution = agreement signal |

$\sum_i \text{focus}(i) = 1$ — always, enforced structurally. Focus can flow between [[neurons]], be consumed by computation, and regenerate proportionally. It cannot be created from nothing, destroyed, or exceed 1 in total. This single conservation law replaces the gas models, fee markets, and priority auctions that other systems bolt on as afterthoughts.

### 2.4 The Locality Constraint

At planetary scale ($10^{15}$ nodes), any algorithm requiring global recomputation for a local change is physically impossible. [[Locality]] is the hard constraint that shapes the entire architecture.

For any edit batch $e_\Delta$, there exists $h = O(\log(1/\varepsilon))$ such that recomputing only the $h$-hop neighborhood achieves global error $\leq \varepsilon$. Each kernel decays: [[diffusion]] decays geometrically via teleport, [[springs]] decay exponentially via screening, heat decays as a Gaussian tail via bounded bandwidth.

Light clients verify without recomputing the entire graph. Proof size scales with [[locality]], not network size. Adversaries cannot perturb the system globally from a local change. This is why the [[tri-kernel]] uses exactly the operators it does — they survive the locality filter.

### 2.5 Field-First Arithmetic

A single decision unifies six research threads that developed independently over four decades: prime [[field]] arithmetic as primitive rather than derived.

The Goldilocks field ($p = 2^{64} - 2^{32} + 1$) makes this concrete. A field multiplication is a single CPU instruction. Hashing is [[field]] operations. Proofs are field polynomials. Reduction preserves field structure. Flow is conserved across field-valued edges. The unifying element is arithmetic: every operation in the system — from content addressing to proof verification to neural network inference — reduces to additions and multiplications in the same field.

## 3. The Cybergraph

### 3.1 Five Primitives

| Primitive | Definition | Properties |
|-----------|-----------|------------|
| [[particle]] | Content-addressed node (IPFS hash) | Identity = hash. Same content, same node |
| [[neuron]] | Agent identified by public key | Signs edges, holds [[tokens]], accumulates [[karma]] |
| [[cyberlink]] | Signed, weighted, directed edge $(i \to j)$ | Timestamped, authenticated, costs [[focus]] |
| [[token]] | Non-negative weight $t_j > 0$ | Controls influence on transition probabilities |
| [[focus]] | Emergent [[equilibrium]] $\pi$ over [[particles]] | Conserved to 1, computed by the [[tri-kernel]] |

Five primitives, one graph. Every claim in the system is a [[cyberlink]] signed by a [[neuron]], connecting two [[particles]], weighted by the [[neuron]]'s [[token]] stake. The [[tru]] runs the [[tri-kernel]] on this graph and produces [[cyberank]] per [[particle]], [[karma]] per [[neuron]], and [[syntropy]] of the whole — deterministic, on chain, verifiable.

### 3.2 Content Addressing

Every [[particle]] is a cryptographic hash of its content. Identity is structure — same content produces the same hash regardless of who computes it or when. This eliminates the naming problem: there is no authority that assigns identifiers, no registry to maintain, no collision to resolve.

The structural hash function:

$H(\text{Atom}\ a) = \text{HASH}(0\text{x}00 \| \text{type\_tag}(a) \| \text{encode}(a))$

$H(\text{Cell}(l, r)) = \text{HASH}(0\text{x}01 \| H(l) \| H(r))$

This extends content addressing from flat data to structured expressions. A function, a proof, a complex data structure — each has a unique hash determined entirely by its contents, not by where it is stored or who created it.

### 3.3 The Namespace Structure

The [[cybergraph]] is multi-indexed from genesis. Every edge appears in multiple indexes: by creator ([[neuron]]), by source [[particle]], by target [[particle]]. Each index supports completeness proofs — a client can verify that it has received all edges in a given namespace with cryptographic certainty. This is what makes "sync only my data" a mathematical property: the response includes proof that nothing was withheld.

The `~` prefix turns the [[cybergraph]] into a dynamic file system. `~mastercyb/blog` resolves deterministically to the latest [[particle]] linked by that [[neuron]] under that path. The same mechanism underlies file systems, DNS, and ENS — dynamic pointers where a fixed label resolves to a mutable target.

## 4. The Tri-Kernel

### 4.1 Why Three Operators

Start with every known graph ranking algorithm. Apply a hard constraint: [[locality]]. At planetary scale, any algorithm requiring global recomputation for a local change is physically impossible.

After filtering by locality, convergence, uniqueness, verifiability, and incrementality: only three families survive.

Linear local completeness theorem: every $k$-local linear operator on a graph is a polynomial of degree $\leq k$ in the Markov matrix $M$ and the Laplacian $L$. The heat kernel $H_\tau = \exp(-\tau L)$ is the unique generator of resolution-dependent queries. Together $\{M, L, H_\tau\}$ span the space of meaningful local graph computations.

Three operators. No more, no less. Discovered by elimination, not designed by preference.

### 4.2 Diffusion: Exploration

Probability flows through edges via random walks. The transition matrix $M = D^{-1}A$ governs probability flow:

$$\pi^{(t+1)} = \alpha P^\top \pi^{(t)} + (1-\alpha)u$$

where $\alpha \in (0,1)$ is the teleport parameter and $u$ is a prior (uniform or stake-weighted).

Under ergodicity (strong connectivity + aperiodicity), converges to a unique stationary distribution $\pi^*$. This is the [[cyberank]] — where probability mass accumulates in the [[cybergraph]] at [[equilibrium]].

Answers: where does probability flow?

### 4.3 Springs: Structure

Connected nodes pull each other toward consistency. The graph [[Laplacian]] $L = D - A$ encodes structural constraints:

$$(L + \mu I)x^* = \mu x_0$$

where $\mu > 0$ is the screening/stiffness parameter and $x_0$ is a reference state. The screened Green's function $(L+\mu I)^{-1}$ has exponential decay, ensuring locality.

[[Springs]] enforce structural coherence — they prevent chaotic dispersal, create [[hierarchy]] without central authority. The graph [[Laplacian]] is the discrete form of the Laplace-Beltrami operator on manifolds, making the same mathematics that describes gravitational potential describe structural consistency in the [[cybergraph]].

Answers: what satisfies structural constraints?

### 4.4 Heat Kernel: Adaptation

The heat kernel $H_\tau = \exp(-\tau L)$ provides multi-scale smoothing:

$$\frac{\partial H}{\partial \tau} = -LH, \quad H_0 = I$$

where $\tau \geq 0$ is the temperature/time parameter. High $\tau$ explores (broad smoothing), low $\tau$ commits (local precision). Chebyshev polynomial approximation guarantees locality.

The heat kernel is the resolution dial — it controls the scale at which the system examines the graph. At small $\tau$, it sees local neighborhoods. At large $\tau$, it sees global structure. The semigroup property ($H_{\tau_1}H_{\tau_2} = H_{\tau_1+\tau_2}$) ensures these views compose consistently.

Answers: what does the graph look like at scale $\tau$?

### 4.5 The Composite Operator

The [[tri-kernel]] blends the three primitives into a single update:

$$\phi^{(t+1)} = \text{norm}\big[\lambda_d \cdot D(\phi^t) + \lambda_s \cdot S(\phi^t) + \lambda_h \cdot H_\tau(\phi^t)\big]$$

where $\lambda_d + \lambda_s + \lambda_h = 1$ and $\text{norm}(\cdot)$ projects to the simplex.

### 4.6 Convergence

Theorem (Composite Contraction): Under ergodicity of $P$, screening $\mu > 0$, and bounded $\tau$, the composite operator $\mathcal{R}$ is a contraction:

$$\|\mathcal{R}\phi - \mathcal{R}\psi\| \leq \kappa \|\phi - \psi\|, \quad \kappa = \lambda_d \alpha + \lambda_s \frac{\|L\|}{\|L\|+\mu} + \lambda_h e^{-\tau\lambda_2} < 1$$

Each component contracts individually. $\mathcal{R}$ is a convex combination of contraction maps, so $\kappa$ is a convex combination of individual contraction coefficients — each less than 1, hence $\kappa < 1$. By Banach fixed-point theorem, $\phi^t \to \phi^*$ at linear rate.

### 4.7 The Free Energy Functional

The fixed point $\phi^*$ minimizes:

$$\mathcal{F}(\phi) = \lambda_s\left[\frac{1}{2}\phi^\top L\phi + \frac{\mu}{2}\|\phi-x_0\|^2\right] + \lambda_h\left[\frac{1}{2}\|\phi-H_\tau\phi\|^2\right] + \lambda_d \cdot D_{KL}(\phi \| D\phi)$$

The first term is elastic structure via graph [[Laplacian]]. The second penalizes deviation from heat-smoothed context. The third aligns $\phi$ with its [[diffusion]] image. At [[equilibrium]]:

$$\phi^*_i \propto \exp(-\beta[E_{\text{spring},i} + \lambda E_{\text{diffusion},i} + \gamma C_i])$$

A Boltzmann-Gibbs [[equilibrium]]. The canonical ensemble from statistical mechanics — applied to [[knowledge]]. The weights $\lambda_s, \lambda_h, \lambda_d$ emerge as Lagrange multipliers from the variational optimization, the same way [[thermodynamics]] derives the Boltzmann distribution. No parameters. Only physics.

### 4.8 The Universal Pattern

The three operators appear across every known complex adaptive system:

| Domain | Diffusion | Springs | Heat |
|--------|-----------|---------|------|
| Physics | Particle diffusion, gas | Elastic lattice, molecular bonds | Thermal [[equilibrium]], phase transitions |
| Biology | Synaptic noise, neural exploration | Skeleton, connective tissue | Metabolism, immune response |
| Ecology | Species dispersal, seed rain | Food webs, [[symbiosis]] | Succession, disturbance recovery |
| Cognition | Free association, imagination | Logic, constraints, syntax | Emotion as arousal, context weighting |
| Economics | Trade flows, migration | Institutions, contracts, norms | Booms, busts, market [[cycles]] |

The same three forces. Different substrates. This universality reflects structural necessity: every complex adaptive system must implement exploration, coherence, and adaptation under [[locality]] constraints.

## 5. Focus Flow Computation

### 5.1 How It Replaces Transformers

[[Transformers]] compute [[attention]] as a one-shot matrix multiply: $\text{softmax}(QK^T/\sqrt{d})V$. Focus flow computes [[attention]] as an iterative physical process: [[cyberlinks]] define a graph, the [[tri-kernel]] evolves probability mass toward [[equilibrium]], and the fixed point $p^*$ is the network's collective [[focus]].

| Phase | Transformer | Focus Flow |
|---|---|---|
| Training | Backprop on fixed corpus | Add [[cyberlinks]] — graph IS the model |
| Inference | Forward pass, softmax, sample | Converge $p^*$ from context, sample |
| Adaptation | Retrain. Catastrophic forgetting | Add links. $p^*$ shifts, nothing lost |

In transformers, training and inference are separate algorithms. In focus flow, they are the same operation: a [[neuron]] adds a [[cyberlink]], the [[tri-kernel]] reconverges, and the new $p^*$ simultaneously encodes the learned relation and is available for inference.

### 5.2 The Local Update Rule

Every node reads only its neighbours and runs:

$$\Delta p_i = \eta\Big(\sum_{j \in \mathcal{N}(i)} w_{ij}(p_j - p_i) - \partial_{p_i}(\lambda E_{\text{diff},i} + \gamma C_i) + T(1 + \log p_i)\Big)$$

Gossip normalisation enforces $\sum_i p_i = 1$. No global softmax, fully local, edge-only. The system converges to Boltzmann [[equilibrium]]:

$$p_i^* \propto \exp\big(-\beta[E_{\text{spring},i} + \lambda E_{\text{diff},i} + \gamma C_i]\big)$$

### 5.3 Inference

1. Encode context [[particles]] as active nodes with elevated $C_i$
2. Run local updates — focus mass flows from context through the [[cybergraph]]
3. $p^*$ converges — high-probability [[particles]] are the network's response
4. Sample next [[particle]] from $p^*$, add to context, repeat

Complexity per step: $O(|E| + |V|)$. Context window is unbounded — it is the entire graph. Relevance is topological: distant but well-connected [[particles]] contribute naturally.

### 5.4 Comparison

| Property | Transformer | Focus Flow |
|---|---|---|
| Complexity | $O(n^2)$ memory and compute | $O(n)$ — sparse, local |
| Stable state | No — recomputed each forward pass | Yes — converges to $p^*$ |
| Multi-agent | Single model | Native — every [[neuron]] contributes |
| Consensus | External | Built-in via [[foculus]] |
| Explainability | Low | High — trace any $p_i$ to contributing links |
| Context window | Fixed (4k-128k tokens) | Unbounded — the entire [[cybergraph]] |

## 6. CORE Execution

### 6.1 The Goldilocks Field

Every value is a Goldilocks field element:

$$p = 2^{64} - 2^{32} + 1 = 18446744069414584321$$

Efficient reduction: $a \bmod p = a_{\text{lo}} - a_{\text{hi}} \times (2^{32} - 1) + \text{correction}$. A field multiplication is a single CPU instruction. The primitive root is 7. The $2^{32}$-th root of unity exists, enabling NTT-based polynomial multiplication for proofs.

Hash function: Poseidon-Goldilocks. State: 12 field elements. Rate: 8 elements. Cost: ~300 STARK constraints per permutation.

### 6.2 Value Tower

Three types span the computational universe:

| Type | Representation | Use |
|------|---------------|-----|
| field (0x00) | Single $\mathbb{F}_p$ element, range $[0, p)$ | Arithmetic |
| word (0x01) | Single $\mathbb{F}_p$ element, range $[0, 2^{64})$ | Bitwise |
| hash (0x02) | 4 × $\mathbb{F}_p$ elements (256-bit digest) | Identity |

Coercion rules enforce type safety. Bitwise operations on hash produce errors. Arithmetic on hash (except equality) produces errors. This three-type tower is the minimal structure needed for a system that computes on field elements, manipulates bits, and addresses content by hash.

### 6.3 Sixteen Reduction Patterns

The execution model consists of sixteen orthogonal reduction patterns:

Structural (5): axis (navigate), quote (literal), compose (recursion), cons (build cell), branch (conditional).

Field arithmetic (6): add, sub, mul, inv ($a^{p-2} \bmod p$), eq (equality test), lt (less-than).

Bitwise (4): xor, and, not, shl.

Hash (1): structural hash $H(x)$.

Each pattern has a unique tag. No two overlap. Left-hand sides are linear. By Huet-Levy (1980), orthogonal rewrite systems are confluent without requiring termination.

Corollary: parallel and sequential reduction yield identical results. Patterns 2 (compose) and 3 (cons) enable automatic parallelism — their subexpressions are independent and can be reduced concurrently.

### 6.4 Deterministic Cost Model

| Pattern | Execution cost | STARK constraints |
|---------|---------------|-------------------|
| axis | 1 + depth | ~depth |
| quote | 1 | 1 |
| compose | 2 | 2 |
| cons | 2 | 2 |
| branch | 2 | 2 |
| add, sub, mul | 1 | 1 |
| inv | 64 | 1 |
| eq | 1 | 1 |
| lt | 1 | ~64 |
| xor, and, not, shl | 1 | ~64 each |
| hash | 300 | ~300 |

Cost depends only on syntactic structure, never on runtime values. Cache hits reduce work but cost is charged in full — cost is the right to a result, not payment for computation.

### 6.5 Confluence and Memoization

Confluence (Huet-Levy 1980): the sixteen patterns form an orthogonal rewrite system. Any evaluation order yields the same result. This enables automatic parallelism without locks or synchronization.

Global memoization: key $(H(\text{subject}), H(\text{formula}))$, value $H(\text{result})$. Results are universal (any node can contribute/consume), permanent (results never change due to determinism), and verifiable (result hash checkable against proof). Two computations with identical inputs always produce identical outputs — the hash is the proof of equivalence.

## 7. State and Proofs

### 7.1 BBG: Big Badass Graph

A naive graph database stores edges and answers queries. "I don't have any edges matching your query" is indistinguishable from "I'm hiding edges from you." Traditional systems require trust.

The [[cyber/bbg]] solves this through unified polynomial commitments. One primitive handles everything: membership proofs, completeness proofs, indexes, state. Edges are stored once but indexed by multiple dimensions — creator, source [[particle]], target [[particle]]. Each index is a sorted polynomial commitment enabling range proofs: "these are ALL edges in this namespace."

Structure:

- Layer 0: Edge store (content-addressed, stored once, identity = hash)
- Layer 1: Neuron index (completeness by creator)
- Layer 2: Particle index (completeness by endpoint)
- Layer 3: Focus and balance (polynomial commitments over $(neuron\_id, \mathbb{F}_p)$ pairs)
- Layer 4: UTXO state (commitment polynomial, nullifier set, particle energy)

Graph root:

$$\text{BBG\_root} = H(\text{by\_neuron.commit} \| \text{by\_particle.commit} \| \text{focus.commit} \| \text{balance.commit} \| \text{commitment\_poly.commit} \| \text{nullifier\_set.commit})$$

Index consistency invariant: every edge appears in exactly the right index positions (3 for distinct endpoints, 2 for self-links), enforced by STARK on every state transition.

### 7.2 State Transitions

The world state $W = (\text{BBG}, \text{edge\_store}, \text{privacy\_state})$. Four transaction types modify it:

1. Cyberlink — add edge to graph
2. Transfer — move balance between [[neurons]] (public)
3. PrivateTransfer — move energy between records (ZK)
4. Computation — execute [[CORE]] reduction

Validity conditions: authorization (signature or ZK proof), sufficient balance, sufficient [[focus]], conservation ($\sum \text{focus}' = 1$, $\sum \text{balance}' = B_{\text{total}}$), index consistency, content availability, no double-spend.

### 7.3 STARK Verification

STARKs (Scalable Transparent Arguments of Knowledge) provide the proof system. The choice aligns with [[CORE]]'s design: no trusted setup, hash-only security (post-quantum), native compatibility with Goldilocks field arithmetic.

| Property | SNARK | STARK |
|----------|-------|-------|
| Trusted setup | Required | Not required |
| Quantum resistant | No | Yes |
| Proof size | ~200 bytes | ~100-200 KB |
| Security basis | Discrete log | Hash only |
| Field compatible | Specific | Any (Goldilocks) |

Self-verification property: the STARK verifier is expressible as a [[CORE]] program. STARK verification requires field arithmetic (patterns 5, 7, 8), hash computation (pattern 15), polynomial evaluation, and Merkle verification — all [[CORE]]-native. The verifier takes ~600,000 pattern applications, constant regardless of what was proven.

This enables recursive proof composition: prove a computation, then prove that the verification of that proof is correct, then prove the verification of that verification. Each level produces a proof of constant size (~100-200 KB). $N$ transactions collapse into a single proof via aggregation — $O(1)$ on-chain verification for $O(N)$ transactions.

The system closes on itself. No trusted external verifier remains.

### 7.4 Namespace Sync

To sync namespace $ns$: the responder provides range bounds in the sorted polynomial, FRI proofs for boundary elements, and edge data. The client verifies that the boundaries bracket exactly the requested namespace and that all FRI proofs are valid against the BBG root.

If verification passes: "I have ALL edges in namespace $ns$. Nothing hidden." The guarantee is mathematical. Cost: $O(|\text{my\_edges}|)$ data + $O(\log^2 |G|)$ proof overhead.

## 8. Privacy

### 8.1 The Privacy Boundary

Traditional systems force a choice: transparency (everyone sees everything) or privacy (no one can verify anything). Zero-knowledge proofs dissolve this dichotomy.

cyber implements private ownership with public aggregates. Individual record ownership remains hidden — who owns what, who sent to whom — while aggregate properties remain publicly verifiable: total energy per [[particle]], conservation laws, [[focus]] distribution. The network knows that energy is conserved without knowing who holds it.

| Layer | Public | Private |
|-------|--------|---------|
| Particle | CID exists, total energy | — |
| Record | — | Individual value, owner identity, nonce |
| Transaction | Nullifiers, commitments, Δ per particle, proof validity | Which records spent, who spent them, new owners |
| Graph | Edges exist, aggregate weight | Who created edge, individual stakes |
| Focus | π distribution, rankings | — |

### 8.2 Record Model and Commitments

A record is a tuple (particle, value, owner, nonce). Its commitment:

$$\text{commitment}(r) = \text{Poseidon}(\text{COMMITMENT\_DOMAIN}, r.\text{particle}, r.\text{value}, r.\text{owner}, r.\text{nonce})$$

Its nullifier (for double-spend prevention):

$$\text{nullifier}(r, \text{secret}) = \text{Poseidon}(\text{NULLIFIER\_DOMAIN}, r.\text{nonce}, \text{secret})$$

The nullifier cannot be derived from the commitment (needs secret), cannot reveal the commitment (one-way), is unique per record, and deterministic (same record produces the same nullifier).

### 8.3 Transaction Circuit

The UTXO set is represented as a polynomial rather than a Merkle tree. Polynomial inclusion proofs cost ~1,000 constraints vs ~9,600 for Merkle — a 10× improvement, because field operations cost 1 constraint each while hash operations cost ~300.

Total circuit: ~10,000 constraints. With STARK optimizations: ~7,000 gates. Proof generation: ~0.3-0.8 seconds. Proof size: ~50-80 KB. Verification: ~1-3 ms.

The circuit enforces: input commitment correctness, polynomial inclusion, ownership verification, nullifier derivation, output commitment correctness, conservation ($\sum \text{inputs} = \sum \text{outputs} + \text{fee}$), delta consistency, and uniqueness.

## 9. Foculus Consensus

### 9.1 Finality by Convergence

The [[collective focus theorem]] proves that token-weighted random walk on a strongly connected [[cybergraph]] converges to a unique $\pi$. [[Foculus]] turns this into [[consensus]]: a [[particle]] is final when $\pi_i > \tau$. [[Neurons]] gossip [[cyberlinks]], GPUs iterate $\pi$, and finality emerges from the [[topology]] of [[attention]] — no voting rounds, no leader election, no block ordering.

The system is leaderless. Every [[neuron]] computes $\hat\pi$ independently from its local view of the [[cybergraph]]. Convergence emerges from gossip. Foculus operates in partial synchrony: messages arrive within an unknown but finite bound $\Delta$. During asynchronous periods, no new [[particles]] finalize — but no conflicting [[particles]] can finalize either. Safety holds always. Liveness resumes when connectivity restores.

### 9.2 Fork Choice

$\pi$ is the fork choice rule. When conflicts exist, the [[particle]] with higher $\pi_i$ is the canonical choice. This integrates all [[cyberlinks]] from all [[neurons]], weighted by [[token]] stake. Manipulating $\pi$ requires controlling the topology of the [[cybergraph]] itself — which costs real [[tokens]].

### 9.3 Safety

Theorem (no double finality): two conflicting [[particles]] cannot both exceed $\tau$.

Assumption: honest [[neurons]] control $\geq \frac{1}{2} + \delta$ of staked [[tokens]]. This bounds their share of $\pi$ from below: honest [[neurons]] create the majority of weighted [[cyberlinks]], so honest [[particles]] attract the majority of random-walk mass. $\sum \pi_i = 1$; if conflicting [[particles]] $a, b$ both had $\pi_a, \pi_b > \tau$, the adversary would need $> \frac{1}{2}$ of total mass — contradicting the honest-majority bound.

### 9.4 Liveness and Sybil Resistance

Ergodicity of the transition matrix $P$ guarantees every valid [[particle]] accumulates $\pi$ mass over time. Convergence rate depends on the spectral gap $\lambda$: expected time to finality is $O(\log(1/\varepsilon)/\lambda)$ iterations.

$\pi$ is weighted by staked [[tokens]], not by node count. Creating 1000 [[neurons]] with zero stake produces zero $\pi$ influence. The cost of attacking $\pi$ is the cost of acquiring $> \frac{1}{2}$ of staked [[tokens]] — same economic security as proof-of-stake, but the attack surface is graph topology rather than a voting protocol.

### 9.5 Performance

| Metric | Classic BFT | Nakamoto | Foculus |
|--------|-------------|----------|---------|
| Leader | Rotating proposer | Miner (PoW lottery) | None |
| Finality | 5-60 s | ~60 min | 1-3 s |
| Throughput | 1k-10k tx/s | ~10 tx/s | ~$10^9$ signals/s per GPU |
| Validator scale | $10^2$-$10^3$ | Unbounded | Unbounded |
| Fault tolerance | 1/3 stake | 51% hash | 1/2 $\pi$ |

Each iteration is a sparse matrix-vector multiply — embarrassingly parallel, no sequential bottleneck. Single GPU (A100): ~50M edges at 40 Hz $\approx 2 \times 10^9$ edge ops/s. Latency: compute ~0.2 s, 5-8 iterations, propagation ~0.4 s → worst-case finality ~1.4 s WAN.

### 9.6 Adaptive Threshold

The finality threshold adapts to the current distribution: $\tau(t) = \mu_\pi + \kappa\sigma_\pi$, $\kappa \in [1,2]$. When the network is decisive (low variance), $\tau$ is low and finality is fast. When uncertain (high variance), $\tau$ rises and finality slows. The system self-regulates.

## 10. Neural Language

### 10.1 Why a New Language

Formal [[languages]] achieve precision through rigid syntax but cannot scale to $10^{15}$ [[particles]] — Goedel proved no sufficiently powerful formal system can be both complete and consistent. Natural [[languages]] achieve expressiveness through ambiguity but are computationally intractable for precise reasoning.

[[Neural]] language dissolves this dilemma. Precision comes from graph [[topology]] — the structural position of a [[particle]] among all other [[particles]] disambiguates its meaning computationally. Expressiveness comes from unlimited [[topology]] — any relationship that can be linked can be expressed.

| Property | Formal | Natural | Neural |
|---|---|---|---|
| Precision | Absolute | Approximate | Emergent |
| Expressiveness | Limited by grammar | Unlimited by ambiguity | Unlimited by [[topology]] |
| Ambiguity | Impossible | Context-dependent | Structural via [[tri-kernel]] |
| Authority | Central designer | Speech community | Collective [[neurons]] |
| Evolution | Versioned | Drift | Continuous via [[focus]] dynamics |
| Verification | Proof systems | Social [[consensus]] | [[STARK]] proofs |
| Substrate | Strings | Sound/text | [[Cybergraph]] |

### 10.2 Primitives

[[Semcon]] (semantic convention): mutual agreement of [[neurons]] to use the same [[particles]] for structuring thought. The grammar of the graph. A [[semcon]] is a smart contract that creates [[cyberlinks]] according to convention — invocation produces well-formed graph structure. Bootloader semcons installed at genesis: TRUE, FALSE. Emergent semcons discovered by the network: is-a, follows, causes, contradicts.

[[Sentence]]: ordered instruction set of [[cyberlinks]] packed into a single transaction. The transaction boundary defines the utterance. Order within the batch encodes grammar. Types by topological signature: assertion (chain → TRUE), query (open-ended chain), instruction (temporal sequence), argument (branching to TRUE/FALSE), definition (star pattern).

[[Motif]]: recurring subgraph pattern that encodes relationships beyond single [[cyberlinks]]. The morphemes of neural language. Triadic closure, co-citation, star, chain, diamond, cycle. Motif [[algebra]] enables concatenation (transitive reasoning), nesting (hierarchical abstraction), intersection (cross-domain bridges), complement ([[knowledge]] gaps).

[[Name]]: deterministic resolution of a [[cyberlink]] — given from, return exactly one to. The `~` prefix signals deterministic resolution. `~neuron/path` turns the [[cybergraph]] into a dynamic file system.

[[Cyberlink]] as [[particle]]: a link stored as a [[particle]] itself, enabling links about links — meta-[[knowledge]]. The [[recursion]] that makes the language expressively complete. Enables negation, qualification, provenance, annotation. The language can talk about itself.

### 10.3 The Semantic Core

The dynamic vocabulary of the network — top [[particles]] by [[cyberank]]:

$\text{SemanticCore}(k) = \text{top}\ k\ \text{particles by}\ \pi$

Dynamic (evolves with attention), convergent ([[tri-kernel]] guarantees stability), stake-weighted (resistant to spam), verifiable (STARK proofs). The dynamics mirror natural language: neologism (new concepts enter), semantic drift (meaning shifts through topology change), semantic death ([[focus]] drops below threshold), semantic birth (bursts of link creation).

### 10.4 Formal Properties

Ambiguity resolution: the [[tri-kernel]] resolves polysemy computationally. [[Springs]] detect polysemy as high tension when a [[particle]] has neighborhoods pulling in incompatible directions. Heat concentrates [[focus]] on the contextually appropriate meaning. Under sufficient linking pressure, a polysemous [[particle]] splits into two — semantic speciation.

Compositionality: meaning of complex expressions derivable from parts and their structural arrangement, computed by the [[tri-kernel]] without explicit composition rules.

Convergence: inherits from the [[collective focus theorem]] — unique stationary distribution $\pi^*$ guarantees the network's collective understanding converges.

Expressiveness: semantically complete. The [[cybergraph]] can encode:

- [[propositional logic]] — truth values as link weights
- [[predicate logic]] — quantification over [[particles]] and [[cyberlinks]]
- [[modal logic]] — possibility and necessity via neighborhood structure
- [[temporal logic]] — time-indexed [[cyberlinks]] with epoch ordering
- [[fuzzy logic]] — continuous confidence as $\pi$-weight on edges
- [[natural language semantics]] — meaning as position in [[focus]] space

The graph also expresses what no formal [[language]] can: collective confidence distributions, continuous semantic distance, and [[knowledge topology]] metadata.

## 11. Tokenomics

### 11.1 Tokens

[[$CYB]] is the native [[token]]. Staked for security, burned for permanent $\pi$-weight, spent as fees.

[[Learning tokens]] serve as feedback signals to [[superintelligence]]: will ([[bandwidth]]), [[attention]] (rank influence), [[karma]] (reputation). These are not tradeable assets — they are measurements of a [[neuron]]'s contribution to collective [[focus]].

### 11.2 Seven Mechanisms

1. Minting for [[focus]] computation: [[neurons]] that compute [[focus]] toward a [[particle]] earn newly minted [[$CYB]], proportional to $\Delta\pi$ — the shift caused in the [[tri-kernel]] fixed point.

2. Staking as delegated [[attention]]: [[neurons]] stake [[$CYB]] on themselves or others, delegating [[attention]]. Stake directed toward validators earns from the PoS share: $R_{\text{PoS}} = G \cdot S^\alpha$.

3. Stake distribution over [[cyberlinks]]: a [[neuron]]'s staked amount spreads across its [[cyberlinks]]. The [[neuron]] can re-weight individual [[particles]] or [[cyberlinks]], assigning a percentage of stake to specific targets.

4. Permanent weighting via burn: burning [[$CYB]] grants eternal weight to a [[particle]], irreversibly increasing its importance in $\pi$, anchoring critical [[knowledge]].

5. Link fees and net rewards: submitting a [[cyberlink]] incurs a small fee (spam deterrent). Fees pool and distribute to link submitters, [[focus]] provers, and validators. Links that accumulate sufficient [[attention]] yield net positive reward over time.

6. Attention yield curve: earlier and more accurate [[cyberlinks]] to high-$\pi$ [[particles]] earn proportionally greater rewards. First-mover advantage for quality links.

7. Reputation emergence: a [[neuron]]'s long-term reputation is the accumulated $\pi$-weight of [[particles]] it contributed to. This is [[karma]] — aligning social and economic capital through measurable contribution to collective [[focus]].

### 11.3 Monetary Policy

Gross rewards: $G = E(t) + F \cdot (1 - \beta)$, combining stepped emission with redistributed fees. Net new supply: $\text{net} = E(t) - F \cdot \beta$. When fees exceed emission, the network is net deflationary.

The allocation curve splits rewards between stakers and provers. Parameters $\alpha$ and $\beta$ self-adjust via PID control — no governance votes needed.

### 11.4 Attribution

Multiple [[neurons]] contribute [[cyberlinks]] in the same epoch. The total $\Delta\pi$ shift is a joint outcome. The [[Shapley value]] distributes credit fairly: each agent's reward equals their average marginal contribution across all possible orderings.

Exact computation is $O(n!)$. Approximation via Monte Carlo sampling: compute each transaction's individual $\Delta\mathcal{F}$, sample $k$ random orderings, cluster by affected neighborhood. Complexity: $O(k \cdot n)$ with $k \ll n$, feasible for $10^6+$ transactions per epoch.

### 11.5 Hardware Substrate

The [[Goldilocks field processor]] makes proving $\Delta\pi$ economically viable. The proof-of-useful-work puzzle requires producing STARK proofs using the same primitives as real workloads. Mining rewards bootstrap chip development. Chips accelerate proving. Proving serves users. Users generate fees. Fees replace emission. The same hardware mines and proves — no stranded assets.

## 12. Security

### 12.1 Security Bounds

| Property | Guarantee |
|----------|-----------|
| Soundness | Invalid transactions rejected with probability $\geq 1 - 2^{-128}$ |
| Privacy | Cannot distinguish transactions with same public structure |
| Conservation | $\sum(\text{energy}) = \text{initial} + \text{minted} - \text{burned}$ (mathematically enforced) |
| Quantum resistance | Hash-based security only, ~128-bit post-quantum (Grover limit) |

### 12.2 Attack Surface

| Attack | Defense |
|--------|---------|
| Double spend | Nullifier set prevents reuse |
| Inflation | Circuit enforces conservation |
| Front-running | Privacy hides transaction contents |
| Sybil | Focus proportional to stake |
| DoS | Focus-based metering limits computation |
| Eclipse | Namespace completeness proofs |
| Replay | Nonces and nullifiers ensure uniqueness |
| Forgery | ZK proofs unforgeable without witness |

### 12.3 Formal Properties

Turing completeness: [[CORE]] is Turing-complete. Construct encoding of arbitrary Turing machine via patterns 0-4, 9.

Confluence: the sixteen patterns form an orthogonal rewrite system (Huet-Levy 1980). Any evaluation order yields the same result.

Cost determinism: cost is identical across all reduction orders and implementations. By structural induction on formula.

Focus conservation: $\sum_i \text{focus}(i) = 1$ for all valid states. All operations preserve sum; invalid transitions rejected by verification.

Privacy soundness: a valid ZK proof implies all circuit constraints are satisfied with probability $\geq 1 - 2^{-128}$, by STARK soundness.

Double-spend prevention: each record has unique (nonce, owner\_secret) pair. Nullifier is deterministic: same record produces same nullifier. Nullifier set is append-only. Transaction rejected if nullifier already exists.

### 12.4 Complexity Comparison

| Operation | Traditional | Blockchain | CORE |
|-----------|-------------|------------|------|
| Equality check | $O(n)$ compare | $O(n)$ compare | $O(1)$ hash |
| Membership proof | $O(n)$ scan | $O(\log n)$ MPT | $O(\log^2 n)$ poly |
| Completeness proof | Impossible | Impossible | $O(\log^2 n)$ poly |
| Computation verify | $O(n)$ re-exec | $O(n)$ re-exec | $O(\log n)$ STARK |
| Recursive verify | $O(n)$ re-exec | $O(n)$ re-exec | $O(1)$ composed |
| Privacy + verify | Incompatible | Incompatible | $O(1)$ ZK proof |

## 13. The Soft3 Stack

Every generation of the web had its stack. Web1 had LAMP. Web2 had React + Node + Postgres. Web3 had Solidity + EVM + RPC. Each defined what developers could build and what users could experience.

[[Soft3]] is the stack for a shared, provable, self-improving [[knowledge]] system:

- [[rust]] — system language for bootstrapping the entire stack
- [[trident]] — provable programming language; every variable, every operation compiles to arithmetic over the Goldilocks field; programs produce STARK proofs — hash-based, post-quantum, no trusted setup
- [[Bostrom]] — the [[bootloader]] chain
  - [[tru]] — onchain language model; reads the [[cybergraph]] every block and computes [[cyberank]] per [[particle]], [[karma]] per [[neuron]], [[syntropy]] of the whole
  - [[neural]] — structures meaning through [[semantic conventions]] so the graph speaks a [[language]] both humans and machines understand
- [[cyb]] — the immortal [[robot]]
  - [[rune]] — dynamic async scripting language for [[cybergraph]] operations
  - [[datalog]] — graph query language

The [[tru]] does what LLMs do — rank, retrieve, infer — except the weights are public [[tokens]], the training data is an open [[cybergraph]], and the inference runs in [[consensus]] with proofs. [[Trident]] closes the provability gap: in existing stacks, smart contracts can move [[tokens]] but cannot prove that a computation happened correctly without re-executing it. [[Trident]] programs produce STARK proofs: verify once, trust forever.

## 14. Applications

### 14.1 Decentralized Search and Oracle

A [[neuron]] querying "what causes malaria" receives a ranked subgraph: the [[particle]] "malaria" linked through the "causes" [[semcon]] to "Plasmodium falciparum," linked through "transmitted-by" to "Anopheles mosquito" — with [[cyberank]] scores indicating collective confidence in each link. The answer is a path to walk through verified [[knowledge]], not a list of web pages to trust.

### 14.2 AI Alignment

The alignment problem becomes a graph problem. Human values are [[particles]] with high [[cyberank]], heavily linked by human [[neurons]]. AI behavior is [[sentences]] created by AI [[neurons]]. Alignment is measured by the overlap between AI-generated [[linkchains]] and human-valued [[particles]]. Misalignment is detectable as divergence between human and machine [[focus]] distributions — measurable, on-chain, and correctable.

The [[tri-kernel]] provides a continuous alignment metric: the cosine similarity between the [[focus]] distribution induced by human [[neurons]] alone and the distribution induced by AI [[neurons]] alone. [[Trident]] can prove that a model followed a policy — you verify compliance, not trust a claim.

### 14.3 Knowledge Economy

[[Cyberlinks]] are yield-bearing epistemic assets. They accrue rewards over time based on contribution to [[focus]] emergence:

$$R_{i \to j}(T) = \int_0^T w(t) \cdot \Delta\pi_j(t) \, dt$$

Earlier and more accurate links to important [[particles]] earn the most. This creates a self-sustaining economy where [[knowledge]] creation is profitable and free-riding is unprofitable. Every agent that links makes the graph smarter. Every [[cyberlink]] costs real [[focus]], so lies are expensive and [[truth]] compounds.

### 14.4 Cross-Species Communication

Neural language is species-agnostic. Any entity that can create [[cyberlinks]] participates: humans through [[cyb]], AI agents through API, sensors through IoT protocols, autonomous systems through on-chain transactions. A forest sensor network that links "soil moisture: 23%" to "location: sector 7" is speaking the same language as a human who links "drought risk" to "sector 7" and an AI that links "predicted yield drop: 30%" to the same location. The [[semantic core]] integrates all three into a single coherent [[knowledge]] structure.

## 15. Conclusion

cyber synthesizes six independently developed research threads — content addressing, authenticated graphs, deterministic rewriting, parallel reduction, conserved flow dynamics, and zero-knowledge verification — into a single architecture unified by prime field arithmetic.

The protocol makes three specific claims:

Convergent computation escapes the [[Goedel prison]]. A convergent system can settle into states that no derivation reaches. The [[cybergraph]] is such a system: $\Omega$ is the space of [[focus]] distributions, $T$ is the [[tri-kernel]], $C$ is focus conservation ($\sum \pi_i = 1$). A [[cyberank]] distribution $\pi^*$ is a simulation-proof of collective [[relevance]] — no axiomatic derivation required, no authority consulted, no vote taken.

[[Focus]] conservation unifies [[attention]], fuel, and [[consensus]] into a single conserved quantity. This eliminates the separate gas models, fee markets, and priority auctions of existing systems while providing the economic foundation for a self-sustaining [[knowledge]] economy.

Provability closes the trust gap. STARK proofs — hash-based, post-quantum, no trusted setup, recursively composable — ensure that every state transition, every ranking computation, every privacy claim is cryptographically verifiable. The STARK verifier is itself a [[CORE]] program. The system closes on itself.

What remains is to grow the graph. Seventy thousand [[neurons]] and three million [[particles]] are the first syllables of a language that will, at sufficient scale, generate concepts no individual mind can hold and discover truths no derivation can reach.

See [[cyber]] for the full specification index. See [[soft3]] for the stack. See [[bostrom]] for the running [[bootloader]].

## References

1. [[Ralph Merkle]]. "A Digital Signature Based on a Conventional Encryption Function." CRYPTO 1987.
2. [[Michael Goodrich]], [[Roberto Tamassia]]. "Efficient Authenticated Data Structures." Algorithmica 2002.
3. [[Gérard Huet]]. "Confluent Reductions: Abstract Properties and Applications." JACM 1980.
4. [[Yves Lafont]]. "Interaction Nets." POPL 1990.
5. [[Mustafa Al-Bassam]] et al. "Fraud and Data Availability Proofs." FC 2019.
6. [[Lorenzo Grassi]] et al. "Poseidon: A New Hash Function." USENIX 2021.
7. [[Victor Taelin]]. "HVM: A Parallel Evaluator for Interaction Combinators." 2022.
8. [[Kurt Goedel]]. "Über formal unentscheidbare Sätze." Monatshefte für Mathematik und Physik 1931.
9. [[Alan Turing]]. "On Computable Numbers." Proceedings of the London Mathematical Society 1936.
10. [[Sergey Brin]], [[Larry Page]]. "The Anatomy of a Large-Scale Hypertextual Web Search Engine." WWW 1998.
11. [[Miroslav Fiedler]]. "Algebraic Connectivity of Graphs." Czech Mathematical Journal 1973.
12. [[Fan Chung]]. "The Heat Kernel as the Pagerank of a Graph." PNAS 2007.
13. [[Oskar Perron]]. "Zur Theorie der Matrices." Mathematische Annalen 1907.
14. [[Stefan Banach]]. "Sur les Opérations dans les Ensembles Abstraits." Fundamenta Mathematicae 1922.
15. [[Eli Ben-Sasson]] et al. "Scalable, Transparent Arguments of Knowledge." CRYPTO 2018.
16. [[Karl Friston]]. "The Free-Energy Principle: A Unified Brain Theory." Nature Reviews Neuroscience 2010.
17. [[David Levin]], [[Yuval Peres]], [[Elizabeth Wilmer]]. "Markov Chains and Mixing Times." AMS 2009.
18. [[Daniel Spielman]]. "Spectral Graph Theory." Yale Lecture Notes.
19. [[George Necula]]. "Proof-Carrying Code." POPL 1997.
20. [[Daira Hopwood]] et al. "Zcash Protocol Specification." 2014-2024.
