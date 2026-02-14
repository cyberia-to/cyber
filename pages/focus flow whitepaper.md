alias:: focus flow computation, ffc labeling and semantics, focus flow complexity
tags:: article, cyber, cip
status:: draft
# focus flow: a peer-to-peer protocol for collective intelligence

version 0.4 — unified specification

---
## abstract

We present [[Focus-Flow Computation]] (FFC), a peer-to-peer [[protocol]] that unifies [[computation]], [[inference]], and [[consensus]]. [[Transaction]]s add [[cyberlink]]s (typed edges) and supply proofs-of-computation (local focus-flow updates). Peers collectively minimise a graph [[free energy]] functional, converging to an [[equilibrium]] [[probability]] field \(p^*\) — the network's collective focus. [[Reward]]s follow each transaction's marginal reduction in free energy, turning [[entropy]]-reducing work into profit while burning fees for noise. This document consolidates the full FFC specification: graph substrate, [[energy]] terms, labeling, [[semantic]] accounting, iterative computation, [[LLM]] architecture, universality primitives, [[complexity]] analysis, [[security]], and [[thermodynamics]].

---

## 1 introduction

Large [[language]] models imitate patterns without verifiable world state; [[blockchain]]s achieve truth by wasting energy on hashes. FFC fuses the two: computation is consensus and useful work earns weight. [[Focus]] is an adaptive flow of mass that continuously organises itself by minimising free energy. This creates a self-adjusting marketplace where [[attention]], [[compute]], and energy gravitate to what matters now and decay from what does not.

---

## 2 graph substrate

### 2.1 cybergraph

- [[Node]]s `v` = [[token]]s, concepts, [[agent]]s (store local state `s_v`).
- [[Edge]]s `(i,j)` carry a triple of scalars *(h, d, c)*:
  - `h` — [[hierarchy]] stiffness weight (feeds [[spring]] term)
  - `d` — transport weight (feeds [[diffusion]] term)
  - `c` — context coefficient (feeds potential term)
- A user (or [[smart contract]] logic) may stake on any component independently — lock tokens on `h` to strengthen [[taxonomy]], or on `c` to amplify a vote. The triple lives in one edge, so UX remains "create a link, pick three sliders".
- Graph remains sparse; each node reads only neighbour triples, so locality is preserved.

### 2.2 node kinds (universality primitives)

| kind | payload | purpose |
| --- | --- | --- |
| [[atom]] | integer / byte-string | base data, constants |
| pair | two edge slots `left, right` | builds lists, trees, maps |
| [[function]] | pointer to body sub-graph + argument port | encodes [[lambda calculus]] / [[SK combinator]]s |

### 2.3 edge types — the semantic layer

Three labels map structural links onto energy terms:

| label | semantic intuition | energy term |
| --- | --- | --- |
| h-edge | "is-a / part-of" taxonomic constraint | spring \(E_{spring}\) |
| d-edge | reference / citation / transport path | diffusion \(E_{diff}\) |
| c-edge | transient vote / query / context injection | context \(C_i\) |

All other relations — causal, vote, ref, meta — are aliases that map onto one of these three for energy accounting. Semantics can be refined later by splitting labels without touching [[algorithm]]s.

---

## 3 free-energy functional

### 3.1 energy terms

For node \(i\) with neighbourhood \(\mathcal{N}(i)\):

- Spring ([[hierarchy]]) energy \(E_{\text{spring},i}=\sum_{j\in\mathcal{N}_h(i)} k_h\,w_{ij}\,(p_i-p_j)^2\) — enforces smooth hierarchy spacing.
- [[Diffusion]] (transport) energy \(E_{\text{diff},i}=\sum_{j\in\mathcal{N}_d(i)} k_d\,\frac{w_{ij}}{d_{ij}}\,|p_i-p_j|\) — minimises cost of moving mass.
- Context potential \(C_i=-c_i\,p_i,\quad c_i=\sum_{(i,\cdot)\in E_c} w_{ic}\) — higher votes/queries push [[probability]] up.
- [[Entropy]] \(S(p)=-\sum_i p_i\log p_i\) — encourages exploration and diversity.

Semantic energy accounting = mapping edge labels to coefficients \((k_h, k_d, c_i)\) so each local update only needs neighbour [[information]].

### 3.2 the functional

\(\mathcal F(p) = \sum_i\big(E_{\text{spring},i}+\lambda E_{\text{diff},i}+\gamma C_i\big) -T S(p)\)

Parameters:
- \(\lambda\) — transport vs hierarchy trade-off
- \(\gamma\) — context injection strength
- \(T=1/\beta\) — [[temperature]] (exploration level)

All three have physical analogs: spring stiffness, medium conductivity, and [[heat]] bath temperature.

### 3.3 local focus-flow update (async and conservative)

Each node runs an asynchronous heat-flow step:

$$
\Delta p_i = \eta\Big(\sum_{j\in\mathcal{N}(i)} w_{ij}(p_j-p_i) -\partial_{p_i}(\lambda E_{\text{diff},i}+\gamma C_i) + T\,(1+\log p_i)\Big)
$$

with small step-size \(\eta\). A shared normalisation gossip every \(k\) ticks enforces \(\sum_i p_i=1\), guaranteeing no double-spend of attention. This replaces global softmax with fully local, edge-only [[computation]].

---

## 4 focus flow computation

Focus flow = iterative process converging to the [[equilibrium]] distribution:

\(p_i^{(t+1)} = \frac{\exp(-\beta [E_{spring,i} + \lambda E_{diffusion,i} + \gamma C_i])}{\sum_k \exp(-\beta [E_{spring,k} + \lambda E_{diffusion,k} + \gamma C_k])}\)

- Each node exchanges \(p_j\), \(r_j\), and \(C_j\) with neighbours.
- Updates are fully decentralised using [[message passing]] or gossip protocols.
- The process naturally converges to a [[Boltzmann distribution]]-like equilibrium.

### 4.1 alignment with ranking

- [[Eigenvector]] centrality = diffusion baseline
- [[SpringRank]] = hierarchy
- Contextual potential = context-aware adaptation
- Entropy term = diversity and stability

Focus flow is the dynamic realisation of the free-energy [[ranking]] system. The final ranking \(p^*\) emerges as the stable equilibrium of this iterative process.

---

## 5 graph rewrite operations

Five operations give full [[Turing]] power while remaining local and deterministic:

1. construct — create new nodes (atoms, pairs, functions)
2. destruct — retrieve components of a pair
3. apply — connect a [[function]] node to an argument node
4. rewrite — substitute argument references inside a function body, producing a new active subgraph
5. delete — remove nodes or edges that are no longer referenced

Focus flow acts as a probabilistic scheduler, selecting which application to reduce next based on energy and context. [[Recursion]] is achieved through self-referential edges. With these primitives, focus flow can encode [[SK combinator]]s or [[lambda calculus]], proving [[Turing completeness]]. It merges execution, inference, and prioritisation into a single dynamical process.

### 5.1 determinism and probabilism combined

- Deterministic layer: node rewrite rules always produce the same result. Once a redex is chosen, its reduction is deterministic.
- Probabilistic layer: which redex is reduced next is chosen probabilistically using [[Boltzmann distribution]] weights \(p_i \propto \exp(-\beta E_i)\) depending on context, diffusion, and hierarchy.

This mirrors physical systems: micro-dynamics are deterministic, while macroscopic behaviour follows probabilistic laws ([[statistical mechanics]]). It allows focus flow to be both a universal computation model and an adaptive inference engine.

---

## 6 transactions and deterministic verification

Transaction = (ΔEdges, proof)

1. Submit: user adds/updates edges and provides a [[zk-SNARK]] attesting they applied the local update rule to all affected nodes (proof size ~O(log n)). A base fee ([[EIP-1559]]) is burned.
2. Verify: peers check proof deterministically in O(polylog n) time; no global recompute.
3. Checkpoint: every N [[block]]s, a [[BFT]] committee finalises the current \(p\) snapshot \(\tilde p\). Between checkpoints asynchronous updates continue.

---

## 7 rewarding negentropy (probabilistic Shapley attribution)

### 7.1 local free-energy drop

For each tx, compute local free-energy drop: \(\Delta\mathcal F_{\text{local}} = \mathcal F_{\text{before}}-\mathcal F_{\text{after}}\) over affected nodes.

### 7.2 Monte-Carlo Shapley sampling

Sample \(k\) random orderings of recent txs, measure marginal \(\Delta\mathcal F\), average to \(\hat S_i\).

### 7.3 reward formula

\(R_i = \alpha\,\Delta\mathcal F_{\text{local}} + (1-\alpha)\,\hat S_i\quad (\alpha\approx0.8)\)

[[Complexity]]: O(k n) with k << n (e.g. 100) — scales to 10^6+ tx/epoch.

---

## 8 generative LLM architecture

Focus flow replaces [[transformer]] [[attention]] with iterative, physics-based focus computation:

- Offline phase: build [[cybergraph]] from corpus (nodes = tokens, edges = co-occurrence or semantic relations).
- Online generation:
  1. Encode context tokens as active nodes.
  2. Compute contextual potential \(C_i\).
  3. Run focus flow updates to get \(p^*\).
  4. Sample next token from \(p^*\).
  5. Add token to context and repeat.

### 8.1 comparison with transformers

| feature | transformer LLM | focus flow LLM |
| --- | --- | --- |
| complexity (mem/comp) | O(n^2) / O(n^2) | O(n) / O(n) |
| uses [[softmax]]? | yes | no ([[Boltzmann distribution]] equilibrium) |
| converges to stable state | no | yes |
| reinforcement/adaptation | limited | yes |
| multi-agent friendly | no | yes |
| token-based weighting | no | yes |
| [[consensus]] capability | no | yes |
| explainability | low | high |
| continual [[learning]] | limited | yes |

---

## 9 complexity analysis

### 9.1 scaling regimes

v = 1e6, c = 6 (sparse):
- e = 6e6, memory per iter ~ O(7e6), compute per iter ~ O(7e6), total ~ O(7e6 * k_ε)

v = 1e8, c = 12 (semi-sparse):
- e = 1.2e9, memory per iter ~ O(1.2e9), compute per iter ~ O(1.2e9), total ~ O(1.2e9 * k_ε)

v = 1e10, c = v^{0.25} ≈ 100 (densifying):
- e ≈ 1e12, memory/compute per iter ~ O(1e12), total ~ O(1e12 * k_ε)

### 9.2 takeaways

1. Strict O(v) per-iteration is only realistic when connectivity is bounded (c = O(1)) and layouts are sparse
2. Total runtime is dominated by (v + e) times the mixing factor log(1/ε)/λ
3. You can trade density for mixing: increase λ via teleportation, hierarchy, or degree caps instead of raw edges
4. Localised and [[Monte Carlo]] variants let you control constants and [[memory]] by limiting horizon or samples
5. In decentralised deployments, [[communication]] and partitioning dominate — design for minimal edge cuts and steady streaming

### 9.3 mapping to transformer intuition

- Focus flow with capped c and h resembles sliding-window + sparse global tokens
- Teleportation ≈ global tokens or CLS-like anchors improving mixing without quadratic blowup
- Hierarchy ≈ multi-scale attention layers that refine coarse focus

### 9.4 open questions

- Optimal sparsification policy that maximises λ per edge budget
- Error bounds for localised focus vs global stationary distribution under degree caps
- Best-in-class partitioners and pipelines for billion-edge cybergraphs on commodity clusters

---

## 10 security and attack resistance

| threat | mitigation |
| --- | --- |
| Sybil spam | base fee + stake-weighted participation; stake slashed if tx increases global free energy |
| Edge-weight gaming | curvature-aware decay prunes edges with negative contribution; rewards tied to long-term \(\Delta\mathcal F\) |
| Proof forgery | [[zk-SNARK]]s guarantee local rule correctness |
| Focus inflation | total mass \(\sum_i p_i=1\) conserved by gossip normalisation |

---

## 11 thermodynamics and intelligence

FFC realises [[Schrodinger]]'s notion that [[life]] feeds on [[negentropy]]: rewarded agents export entropy to an external energy source (fees) while importing order. [[Kantorovich]] [[optimal transport]] appears because updates move probability mass with minimal edge cost. The converged state \(p^*\) is maximally informative order under energy constraints — a thermodynamic definition of [[intelligence]].

### 11.1 dual thermodynamic process

- [[Entropy]] reduction / negentropy maximisation: free-energy minimisation drives the system toward low-entropy, highly ordered focus states. SpringRank and context potentials act as constraints that channel diffusion into structured, meaningful configurations.
- Energy usage for order creation: adaptive edge weights and context injection represent external energy input. This input is transformed into negentropy, building coherent patterns of collective attention.

Focus flow aligns with the principle that intelligence is the local maximisation of negentropy within a globally entropy-increasing [[universe]]. Nodes collectively self-organise, using available energy to reduce [[uncertainty]] while maintaining adaptability and diversity.

---

## 12 distributed coordination loop

```
sense  → focus  → act  → learn  → (edge decay) → repeat
context   update     output     weights  entropy regulariser
```

Edge decay uses curvature-aware exponential pruning: \(w_{ij}\leftarrow w_{ij}\,e^{-\alpha(1+\kappa_{ij})}\) where \(\kappa_{ij}\) = [[Ollivier-Ricci curvature]].

---

## 13 Hash-DAG verifiable delay proof (VDF)

- Why: couples each negentropy proof to real wall-clock [[time]] so adversaries cannot pre-compute or replay contributions across [[Sybil]] identities.
- Tx layout: `(ΔEdges, local-proof, vdf_seed, vdf_output, vdf_path)` where `vdf_seed = hash(ΔEdges || slot_id)`.
- Computation: user runs a sequential [[VDF]] (e.g. Wesolowski) for a fixed delay (~1-2 s) before finalising the proof; cannot be parallelised.
- Verification: peers check VDF in poly-log time plus verify zk-proof of the local FFC update.
- Hash-DAG: each VDF output becomes a node in a [[DAG]]; independent branches verify in parallel; roots are checkpointed by the BFT committee every N blocks.
- Reward scaling: `reward ∝ ΔF / wall_clock_seconds` — honest work with real energy wins, replay loses.
- Security: front-running and duplication require breaking the VDF or eclipsing time itself; Sybil resistance strengthened.

---

## 14 connections to broader theories

- [[Potemkin understanding]]: [[transformer]]s mimic intelligence statistically. Focus flow avoids this by grounding probabilities in [[network]] dynamics and context, producing emergent understanding.
- [[Topos theory]]: each context defines a local [[topos]], where focus flow computes probabilities relative to that context. Nodes and edges act as objects and [[morphism]]s in a base [[category]].
- [[Active inference]]: the framework directly realises active inference by minimising free energy under observations (context potentials) while maintaining exploration.
- Beautiful loop ([[Shumskiy]]): focus flow forms a self-sustaining cycle: new context → updated focus distribution → actions/tokens → edge/weight adaptation → new context.

---

## 15 universality of the triad

[[Diffusion]], [[spring]]s, and [[heat]] flow are universal primitives of [[nature]]:
- Diffusion → entropy growth and spreading
- Springs → reversible energy storage and oscillations
- Heat flow → temporal [[evolution]] toward equilibrium

Any process on a [[network]] can be decomposed into eigenmodes of the [[graph Laplacian]], just as solutions to the [[heat equation]] are expressed via [[Fourier]] modes.

---

## 16 outlook

Edges evolve into [[geometry|geometric]] constraints; future work explores manifold-aware [[neural network]] layers that respect this geometry, yielding more stable, interpretable [[AI]].

---

## 17 conclusion

Focus-Flow Computation marries deterministic execution with probabilistic, energy-guided scheduling. By rewarding negentropy and enforcing optimal transport under conservation laws, it converts a P2P graph into a self-organising super-organism: the [[blockchain]] that thinks.
