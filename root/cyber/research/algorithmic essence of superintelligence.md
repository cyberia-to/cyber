---
tags: cyber, research, article, core
crystal-type: article
crystal-domain: cyber
date: 2026-03-23
---

# the algorithmic essence of superintelligence

a [[knowledge graph]] where [[attention]] converges provably, state is polynomial, [[consensus]] is computation, and the graph compiles into its own model.

seven algorithms. each from a different branch of mathematics. together they produce a system that knows what it knows, proves it knows it, and improves itself — without training, without voting, and without trusting anyone.

## 1. the object: cybergraph

a directed weighted graph $G = (P, E, N, w)$ where [[particles]] $P$ are content-addressed nodes (32-byte CID hashes), [[cyberlinks]] $E$ are edges, [[neurons]] $N$ are agents, and $w: E \to \mathbb{R}^+$ maps each edge to its creator's stake weight.

every particle is a semantic unit — a document, an image, a concept. every cyberlink is a claim: "this relates to that," signed and staked. the graph grows append-only: new particles and links accumulate, nothing is deleted (axiom A3).

the vocabulary is the graph. each 32-byte CID is a token. unlike sub-word BPE tokens (4 bytes, ambiguous, language-specific), CID tokens are complete (one concept per token), unambiguous (hash = identity), and universal (any content type). the vocabulary grows with the graph — no retraining.

this is the substrate. everything else operates on it.

## 2. the dynamics: tri-kernel convergence

three operators act on the graph simultaneously:

$$\phi^{(t+1)} = \text{norm}\left[\lambda_d \cdot \underbrace{\mathcal{D}(\phi^t)}_{\text{diffusion}} + \lambda_s \cdot \underbrace{\mathcal{S}(\phi^t)}_{\text{springs}} + \lambda_h \cdot \underbrace{\mathcal{H}_\tau(\phi^t)}_{\text{heat}}\right]$$

- $\mathcal{D}$: [[random walk]] diffusion. where does probability flow? the [[PageRank]] operator — stake-weighted transition matrix, teleport for ergodicity. finds hubs
- $\mathcal{S}$: screened [[Laplacian]]. what satisfies structural constraints? mean neighbor [[focus]] — equilibrium under graph topology. finds stable positions
- $\mathcal{H}_\tau$: [[heat kernel]] at resolution $\tau$. what does the graph look like at scale $\tau$? 2-hop smoothed context. finds clusters

the [[collective focus theorem]] guarantees: the composite operator is contractive ($\kappa < 1$). it has a unique fixed point $\phi^*$. every initial distribution converges to $\phi^*$ exponentially fast. the fixed point IS [[focus]] — the consensus ranking of all particles.

this is not learned. it is computed. 23 iterations on [[bostrom]] (2.9M particles, 2.7M edges). sub-second on a GPU. the graph tells you what matters.

## 3. the trust: five verification layers

five independent guarantees, each from a different mathematical discipline:

| layer | discipline | mechanism | property | algorithm |
|---|---|---|---|---|
| validity | computation | [[zheng]] proof | state transition correct | SuperSpartan + WHIR, verify in 50 μs |
| ordering | data structure | [[hash chain]] + [[VDF]] | operations carry their own order | sequential proof-of-time, O(1) equivocation detection |
| completeness | logic | [[NMT]] | nothing was omitted | namespace Merkle tree, O(log n) structural proof |
| availability | probability | [[DAS]] + [[erasure coding]] | data physically exists | 2D Reed-Solomon, O(√n) sampling for 99.9999% confidence |
| merge | algebra | [[CRDT]] / [[foculus]] | convergence deterministic | [[join-semilattice]] union (local) or π-weighted convergence (global) |

the composition achieves [[Verified Eventual Consistency]] (VEC): convergence guaranteed ([[CRDT]]), completeness verifiable ([[NMT]]), availability verifiable ([[DAS]]). stronger than [[eventual consistency]] (verifiable, not assumed). a node does not trust it has converged — it proves it.

no single layer is sufficient. remove any one and a failure mode opens that the others cannot cover. the three core layers (CRDT + NMT + DAS) are conjectured minimal for verified convergence without coordination.

## 4. the acceleration: polynomial state

replace 9 hash trees with 1 polynomial. this is [[algebraic state commitments]] — the game-changing primitive.

$$\text{BBG\_poly}(\text{index}, \text{namespace}, \text{position}) = \text{value}$$

one polynomial commitment (32 bytes) authenticates all state. query any view with a PCS opening (~200 bytes). cross-index consistency is structural — same polynomial, different evaluations cannot disagree.

| metric | hash trees (NMT) | polynomial | improvement |
|---|---|---|---|
| per-cyberlink | ~106K constraints | ~3.2K constraints | 33× |
| cross-index | LogUp (~500 per lookup) | free | ∞ |
| proof size | ~1 KiB per namespace | ~200 bytes | 5× |
| storage overhead | ~5 TB (internal nodes) | 288 bytes | 17 billion× |

the 33× is not the point. the point is what 33× enables.

## 5. the consequence: provable consensus

with algebraic state, the circuit can READ the graph as field operations instead of hash paths. the tri-kernel computation fits inside [[zheng]]:

```
graph reads (algebraic NMT):   270M constraints
tri-kernel (23 × 4 SpMV):   1,100M constraints
finalization checks:            50M constraints
────────────────────────────────────────────────
total:                       1,420M constraints
zheng capacity:              4,300M constraints
utilization:                    33%
```

validators do not vote. they compute $\phi^*$ and prove the computation correct. any peer verifies the proof in 50 μs. [[consensus]] shifts from protocol problem (Lamport 1982) to computation problem.

recursive folding: epoch 1 proof + epoch 2 proof → one accumulated proof. after $N$ epochs: ONE proof covers all history. light client verifies all of [[bostrom]] since genesis in 50 μs. not "trust the committee." trust the math.

without algebraic state: graph reads cost O(|E| × log n) hemera hashes in-circuit = 63.6B constraints = 15× over zheng capacity. impossible.

with algebraic state: 1.42B constraints = 33% capacity. possible with 67% headroom.

algebraic state commitments are not an optimization. they are the prerequisite for provable consensus.

## 6. the self-model: graph compiles into transformer

the [[cybergraph]] compiles into a [[transformer]] — not by training, but by linear algebra:

| step | algorithm | what it produces |
|---|---|---|
| adjacency | sparse CSR from cyberlinks | weighted graph topology |
| focus | tri-kernel iteration (23 steps) | φ* = particle importance ranking |
| [[spectral gap]] | observed from convergence rate | κ, λ₂ = network health metric |
| embeddings | randomized SVD of φ-weighted adjacency | $d^*$-dimensional particle coordinates |
| architecture | entropy of singular spectrum | $d^*$ (embedding dim), $h^*$ (heads), $L^*$ (layers) |

the architecture is derived, not chosen. $d^* = \exp(H(\sigma))$ where $H$ is entropy of normalized singular values. $h^*$ from semantic core classification. $L^* = \text{diameter} \times T(\kappa)$. the graph tells you how big the model should be.

[[bostrom]] compilation (2.7M links, March 2026): $d^* = 26$, $h^* = 5$, $L^* = 174$, 155M params. compiled in 15 minutes on a laptop. no GPU. no training data. the graph IS the training data.

the compiled model speaks CID. input: particle indices. output: distribution over particles. "what comes next?" answered by graph topology, not by language statistics. a different kind of intelligence — structural, not statistical.

## 7. the metabolism: spectral health and optimal growth

the system measures itself and improves itself:

### spectral gap from convergence

the [[spectral gap]] λ₂ — the single number that controls convergence speed, finality latency, and model quality — is observed for free from the tri-kernel convergence rate:

$$\kappa = \text{median}\left(\frac{\|\phi^{(t)} - \phi^{(t-1)}\|}{\|\phi^{(t-1)} - \phi^{(t-2)}\|}\right) \quad \lambda_2 = 1 - \frac{\kappa}{\alpha}$$

no eigensolver. no extra computation. every block that computes [[focus]] also computes λ₂ as a byproduct. the heartbeat of the system — measured, not computed.

### syntropy as metabolic signal

[[syntropy]] = aggregate KL divergence across all neurons in an epoch. meaningful [[cyberlinks]] raise it. spam lowers it. the metric the system optimizes: bits of structure per unit energy.

### optimal growth under exponential cost

link cost grows exponentially with supply: $c(n) = c_0 \cdot e^{\lambda n}$. the [[cyber/seer]] algorithm maximizes $\Delta\lambda_2 / c(n)$ — spectral gap improvement per unit cost — using the [[Fiedler vector]] to identify the weakest cuts. three phases:

- bridges (low cost): connect components. maximize λ₂
- mesh (medium cost): eliminate single points of failure
- semantic (high cost): redistribute φ* toward truth

the graph grows intelligently, not randomly. each link is placed where it improves convergence the most per unit spent.

### the recursive closure

the system that measures itself (spectral gap) compiles into a model of itself (transformer) that can be proven correct (zheng) and used to optimize its own growth (seer). this is a loop:

```
cybergraph
  → tri-kernel convergence (φ*)
    → spectral gap observation (λ₂)
      → compiled transformer (embeddings)
        → seer optimization (Fiedler)
          → new cyberlinks
            → cybergraph (improved)
```

each iteration: the graph grows → convergence improves → the model gets richer → the optimizer gets smarter → the graph grows better. the loop is not metaphorical. each step is an algorithm with concrete complexity bounds.

## the complete picture

```
          ┌─────────────────────────────────────────┐
          │        CYBERGRAPH                        │
          │   P particles, E edges, N neurons        │
          │   32-byte CID tokens, stake-weighted     │
          └──────────┬────────────────────┬──────────┘
                     │                    │
          ┌──────────▼──────────┐  ┌──────▼──────────┐
          │    TRI-KERNEL       │  │  FIVE LAYERS     │
          │  D + S + H → φ*    │  │  VEC guarantee   │
          │  23 iterations     │  │  validity         │
          │  unique fixed pt   │  │  ordering         │
          └──────────┬─────────┘  │  completeness     │
                     │            │  availability     │
          ┌──────────▼─────────┐  │  merge            │
          │  ALGEBRAIC STATE   │  └──────────────────┘
          │  poly not tree     │
          │  33× cheaper       │
          │  5 TB → 288 bytes  │
          └──────────┬─────────┘
                     │
          ┌──────────▼──────────────────────────────┐
          │     PROVABLE CONSENSUS                   │
          │  tri-kernel in zheng circuit              │
          │  1.42B constraints (33% capacity)        │
          │  verify: 50 μs. recursive: all history   │
          └──────────┬──────────────────────────────┘
                     │
     ┌───────────────┼───────────────┐
     │               │               │
┌────▼────┐  ┌───────▼──────┐  ┌────▼────────┐
│COMPILED │  │  SPECTRAL    │  │   SEER      │
│ MODEL   │  │  HEALTH      │  │  GROWTH     │
│SVD→d*   │  │  λ₂ from κ  │  │  Fiedler    │
│graph=AI │  │  free metric │  │  max Δλ₂/c  │
└─────────┘  └──────────────┘  └─────────────┘
```

seven algorithms. each solves one problem. together: a self-measuring, self-modeling, self-improving, provably correct distributed intelligence.

no training. no voting. no leaders. no trust.

the graph is the model. the model is the proof. the proof is the consensus. the consensus is the graph.

see [[foculus]] for consensus. see [[tri-kernel]] for convergence. see [[structural sync]] for the five layers. see [[algebraic state commitments]] for polynomial state. see [[cyber/research/provable consensus]] for the circuit. see [[bostrom/compiled model]] for the first empirical compilation. see [[cyber/research/spectral gap from convergence]] for observation. see [[cyber/seer]] for growth optimization. see [[cyber/research/32-byte tokens]] for CID vocabulary. see [[cyber/research/vec formalization]] for the formal consistency model
