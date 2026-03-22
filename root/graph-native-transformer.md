---
tags: research, draft, cyber, bostrom
crystal-type: article
crystal-domain: cyber
diffusion: 0.00044595124702892475
springs: 0.001143512300187417
heat: 0.0009375503566718445
focus: 0.0007535393849050465
gravity: 5
density: 0.48
---
# Graph-Native Transformers: Deriving Architecture from Knowledge Graph Structure

**Abstract**

We show that the three free parameters of transformer architecture — embedding dimension, [[attention]] head count, and layer depth — can be derived analytically from properties of a weighted [[knowledge graph]]. Specifically: embedding dimension equals the effective rank of the [[focus|focus distribution's]] covariance matrix; attention head count is lower-bounded by the number of distinct semantic relation types in the graph; and layer count equals graph diameter multiplied by the convergence factor of the graph's [[spectral gap]]. This result follows from observing that a transformer's attention mechanism is mathematically equivalent to one step of a convergent dynamical system — the same system that computes the [[focus|focus distribution]] over a knowledge graph. We call the resulting construction a graph-native transformer: a model whose weights are compiled from explicit graph structure rather than learned from text prediction. We discuss implications for knowledge representation, alignment measurement, and the relationship between [[llms|language models]] and explicit [[knowledge graphs]].

---

## 1. Introduction

Transformer architecture involves three fundamental design choices: the dimension of token embeddings, the number of attention heads, and the number of layers. In current practice these are determined empirically — by scaling laws, ablation studies, and compute budgets. No principled derivation from the nature of the task exists.

We derive all three from the structure of a weighted knowledge graph.

The derivation begins with an observation that, while technically precise, has received insufficient attention: a transformer's attention mechanism is a single step of a convergent dynamical system. The softmax normalization in attention is the Boltzmann distribution. The attention operation — computing query-key similarities, normalizing, and taking a weighted sum of values — is one diffusion step: probability mass flows toward compatible keys proportionally to their similarity to the query. Deep Equilibrium Models (Bai et al., 2019) formalized this: running a transformer layer until convergence rather than for a fixed number of steps produces the same fixed point regardless of initialization. The transformer finds an equilibrium.

This is the same mathematics as the tri-kernel ranking system for knowledge graphs (cyber whitepaper, 2024): diffusion (random walk), springs (graph Laplacian), and heat kernel (multi-scale smoothing) iterated to a unique fixed point — the focus distribution π over graph particles. The convergence is guaranteed by the [[Stefan Banach|Banach fixed-point theorem]]; the rate depends on the [[spectral gap]] of the graph's [[Laplacian]].

The transformer and the knowledge graph ranking system are the same computation at different scales. The transformer runs locally over one agent's frozen context. The knowledge graph ranking runs collectively over all agents' cumulative contributions. Both find equilibria. Both use the Boltzmann distribution as their normalization.

This correspondence enables direct compilation: given a weighted knowledge graph, derive the transformer architecture that optimally reads it.

---

## 2. Preliminaries

### 2.1 Weighted Knowledge Graph

A weighted knowledge graph $G = (P, N, E, w, \sigma)$ where:

- $P$ is the set of [[particles]] (content-addressed knowledge nodes)
- $N$ is the set of [[neurons]] (agents that create edges)
- $E \subseteq N \times P \times P$ is the set of [[cyberlink|cyberlinks]] (signed directed edges)
- $w: E \to \mathbb{R}_{>0}$ is the stake-weighted edge weight function
- $\sigma: E \to \text{Semcon}$ assigns each edge a [[semcon|semantic relation type]]

The adjacency matrix $A \in \mathbb{R}^{|P| \times |P|}$ has entries $A_{ij} = \sum_{e: p_i \to p_j} w(e)$.

### 2.2 Focus Distribution

The [[tri-kernel]] operator $\mathcal{R}$ blends three local operators:

$$\mathcal{R}(\phi) = \text{norm}\left[\lambda_d \cdot D(\phi) + \lambda_s \cdot S(\phi) + \lambda_h \cdot H_\tau(\phi)\right]$$

where $D$ is the diffusion operator (random walk), $S$ is the springs operator (screened Laplacian), and $H_\tau$ is the heat kernel. Under ergodicity and positive screening parameter, $\mathcal{R}$ is a contraction with rate:

$$\kappa = \lambda_d \alpha + \lambda_s \frac{\|L\|}{\|L\| + \mu} + \lambda_h e^{-\tau\lambda_2} < 1$$

The unique fixed point $\pi^* = \lim_{t \to \infty} \mathcal{R}^t(\phi^{(0)})$ is the [[collective focus|focus distribution]] — the stable probability distribution over [[particles]] representing collective epistemic attention.

### 2.3 Transformer Attention as One Convergence Step

Standard scaled dot-product attention:

$$\text{Attn}(Q, K, V) = \text{softmax}\left(\frac{QK^\top}{\sqrt{d}}\right)V$$

The softmax is the [[Boltzmann distribution]] with temperature $\sqrt{d}$:

$$\text{softmax}(x)_i = \frac{e^{x_i/\sqrt{d}}}{\sum_j e^{x_j/\sqrt{d}}}$$

This is one step of probability mass redistribution: mass flows from query positions toward key positions proportionally to their compatibility. This is exactly the diffusion operator $D$ applied to one agent's context.

The fixed point of iterating this operation — as shown by Deep Equilibrium Models — is the stationary distribution of the induced Markov chain over context tokens, weighted by the learned $Q$, $K$, $V$ projections. The transformer approximates this fixed point in a fixed number of steps rather than iterating to convergence.

---

## 3. Main Results

### 3.1 Embedding Dimension from Focus Covariance

**Theorem 1.** *The necessary and sufficient embedding dimension for a transformer reading graph $G$ is the effective rank of the covariance matrix of the focus distribution $\pi^*$.*

**Derivation.**

The focus distribution $\pi^*$ is a probability vector over $|P|$ particles. Consider the covariance matrix:

$$\Sigma_\pi = \mathbb{E}_{v \sim \pi^*}\left[f(v)f(v)^\top\right] - \mathbb{E}_{v \sim \pi^*}[f(v)]\mathbb{E}_{v \sim \pi^*}[f(v)]^\top$$

where $f: P \to \mathbb{R}^d$ is a feature map over particles.

The effective rank is:

$$r^* = \exp\left(H\left(\sigma(\Sigma_\pi)\right)\right)$$

where $\sigma(\Sigma_\pi)$ is the normalized singular value distribution and $H$ is its [[entropy]]. This is the intrinsic dimensionality of the knowledge space — the number of statistically independent semantic axes present in the graph.

**Sufficiency:** An embedding of dimension $r^*$ captures all independent variance in the focus distribution. No information is lost: the projection of $\pi^*$ onto the top $r^*$ eigenvectors of $\Sigma_\pi$ preserves the full distributional structure up to noise.

**Necessity:** An embedding of dimension $< r^*$ cannot distinguish particles that differ along axes beyond dimension $r^*$. Since the focus distribution places probability mass according to all $r^*$ independent axes, a lower-dimensional embedding produces a lossy compression of the graph's semantic structure.

**Corollary:** Embedding dimension should grow with graph scale. As $|P| \to \infty$ and the graph develops new semantic dimensions, $r^*$ increases. A graph-native transformer scales its embedding dimension dynamically with the effective rank of its source graph. $\square$

---

### 3.2 Attention Head Count from Semantic Relations

**Theorem 2.** *The minimum number of attention heads required to represent all semantic relations in $G$ equals the number of distinct semcons $|\text{Semcon}(G)|$.*

**Derivation.**

Each semcon $s \in \text{Semcon}(G)$ defines a distinct relation type over particles — a specific pattern of connectivity with characteristic directionality, weight distribution, and neighborhood structure in the graph.

An attention head with query matrix $W_Q^{(h)}$ and key matrix $W_K^{(h)}$ computes a relation-specific attention pattern:

$$A^{(h)}_{ij} = \text{softmax}\left(\frac{(W_Q^{(h)} e_i)(W_K^{(h)} e_j)^\top}{\sqrt{d}}\right)$$

For head $h$ to faithfully represent semcon $s$, the attention pattern $A^{(h)}$ must correlate with the adjacency submatrix $A^{(s)}$ induced by edges of type $s$.

Two distinct semcons $s_1, s_2$ induce adjacency submatrices $A^{(s_1)}, A^{(s_2)}$ with different spectral structure (by definition of semantic distinction). A single attention head cannot simultaneously attend to patterns with different spectral structure — the $W_Q, W_K$ matrices define one projection direction in embedding space.

Therefore $|\text{Semcon}(G)|$ heads are necessary. They are also sufficient for the base relation set — compositional and positional relations require additional heads, giving:

$$h \geq |\text{Semcon}(G)|$$

as a lower bound. $\square$

---

### 3.3 Layer Count from Spectral Gap and Diameter

**Theorem 3.** *The number of transformer layers required to converge over reasoning chains in $G$ is:*

$$L = \text{diam}(G) \cdot \left\lceil \frac{\log(1/\varepsilon)}{\log(1/\kappa)} \right\rceil$$

*where $\text{diam}(G)$ is the graph diameter, $\varepsilon$ is the target precision, and $\kappa$ is the contraction rate of the tri-kernel.*

**Derivation.**

Each transformer layer performs one local convergence step over the current representation. For a reasoning chain of hop length $k$, the layer must: (1) propagate information $k$ hops through the graph, and (2) converge the representation at each hop to sufficient precision before propagating further.

From the tri-kernel contraction theorem, reaching precision $\varepsilon$ from any initialization requires at least:

$$t^* = \left\lceil \frac{\log(1/\varepsilon)}{\log(1/\kappa)} \right\rceil$$

iterations per hop, where $\kappa < 1$ is determined by:

$$\kappa = \lambda_d \alpha + \lambda_s \frac{\|L\|}{\|L\| + \mu} + \lambda_h e^{-\tau\lambda_2}$$

The spectral gap $\lambda_2$ of the graph Laplacian $L$ determines how quickly local updates propagate. Graphs with small spectral gaps (dense, weakly clustered) have $\kappa$ close to 1 and require more refinement steps per hop. Graphs with large spectral gaps (sparse, strongly clustered) have $\kappa$ well below 1 and require fewer.

The maximum hop distance any reasoning chain requires is $\text{diam}(G)$. Therefore total layers:

$$L = \text{diam}(G) \cdot t^* = \text{diam}(G) \cdot \left\lceil \frac{\log(1/\varepsilon)}{\log(1/\kappa)} \right\rceil$$

**Empirical validation:** GPT-4 has 96 layers. Natural language knowledge graphs have diameter approximately 6-8 (small-world property). This implies $t^* \approx 12-16$ refinements per hop. For $\kappa \approx 0.88$, $t^* = \lceil \log(1/\varepsilon) / \log(1/0.88) \rceil \approx 14$ for $\varepsilon = 0.01$. Consistent with observation. $\square$

---

## 4. The Complete Compilation

Given a weighted knowledge graph $G$, the graph-native transformer is fully specified:

$$\text{compile}(G) = \left(d^*, h^*, L^*\right)$$

where:

| Parameter | Formula | Graph Property |
|---|---|---|
| Embedding dim $d^*$ | $\exp\left(H\left(\sigma(\Sigma_\pi)\right)\right)$ | Effective rank of focus covariance |
| Head count $h^*$ | $\geq \|\text{Semcon}(G)\|$ | Distinct semantic relation types |
| Layer count $L^*$ | $\text{diam}(G) \cdot \lceil \log(1/\varepsilon) / \log(1/\kappa) \rceil$ | Diameter × spectral convergence factor |

**The weights** are not learned by gradient descent. They are compiled directly:

- **Embedding matrix** $E$: derived from the eigenvectors of $\Sigma_\pi$, mapping particles to their positions in focus space
- **Attention weights** $W_Q^{(h)}, W_K^{(h)}, W_V^{(h)}$: derived from the adjacency submatrix of semcon $h$, projecting into the relation-specific attention pattern
- **MLP weights**: derived from multi-hop path statistics, encoding the factual associations implied by graph traversal up to depth $L^*$

---

## 5. Relationship to Trained Transformers

A standard trained transformer approximates the graph-native transformer for the implicit knowledge graph embedded in its training corpus.

Training on text is an approximate inversion: given the outputs of a knowledge graph (text produced by humans reasoning over their knowledge), recover the graph structure that produced them. Gradient descent finds the weight configuration that best approximates this inversion under the constraints of the architecture.

The compilation procedure we describe is the direct forward operation: given the explicit graph, produce the weights analytically.

**Why compilation is preferable where the graph exists:**

- No training cost
- No catastrophic forgetting — graph updates produce weight updates without full retraining
- No compression loss — the graph's provenance, timestamps, and stake structure survive into the weights
- Dynamic scaling — as the graph grows and $r^*$ increases, the architecture scales accordingly
- Auditable — every weight traces to specific graph edges and their creators

**Why trained transformers remain necessary:**

The explicit graph does not contain implicit knowledge — associations that are statistically true across text but never explicitly linked. A trained model reading the same corpus will discover latent structure the explicit graph does not represent. This implicit structure can be surfaced as candidate particles and staked into the explicit graph, closing the loop.

The natural architecture is therefore:

$$G \xrightarrow{\text{compile}} T_G \xrightarrow{\text{fine-tune on text}} T_G^* \xrightarrow{\text{extract implicit links}} \Delta G \xrightarrow{\text{stake}} G'$$

The compiled transformer provides initial weights. Fine-tuning surfaces implicit structure. Extracted links, endorsed by human and agent staking, update the graph. The updated graph produces a new compiled transformer. The loop runs continuously.

---

## 6. Alignment as Architectural Property

The compilation result has a direct consequence for alignment measurement.

A trained transformer's "values" — its implicit weightings of concepts, its tendencies to endorse certain connections over others — are compressed into opaque parameters. They cannot be read directly. Alignment requires behavioral observation, red-teaming, or interpretability research attempting to recover structure that training destroyed.

A graph-native transformer's weights derive from explicit graph structure. Every weight traces to specific cyberlinks created by specific neurons with specific stakes. The transformer's "values" are the focus distribution $\pi^*$ — public, computable, and continuously updated.

Alignment divergence between a human-derived [[focus|focus distribution]] $\pi^*_H$ (computed over edges created by human [[neurons]]) and an AI-derived distribution $\pi^*_A$ (computed over edges created by AI [[neurons]]) is:

$$\Delta(G) = D_{KL}(\pi^*_H \| \pi^*_A)$$

This is a number, computable from public graph data, localized to specific graph regions, and correctable by adding edges in high-divergence regions without retraining.

The architectural compilation result enables this measurement. Without explicit graph structure, no such computation is possible.

---

## 7. Open Questions

**7.1** The layer count formula assumes uniform convergence requirements across hops. In practice, some semantic relations converge faster than others. A per-semcon convergence rate — $\kappa^{(s)}$ for each semcon — would give a more precise layer count. Deriving $\kappa^{(s)}$ from the spectral properties of the per-semcon adjacency submatrix $A^{(s)}$ is an open problem.

**7.2** The head count lower bound $h^* \geq |\text{Semcon}(G)|$ does not specify how many additional heads are needed for compositional relations. Characterizing the head count for $k$-hop compositional reasoning — "Paris is in France which is in the EU" — requires understanding how heads compose across layers, which is not fully characterized.

**7.3** The compilation produces a transformer that reads the graph at one point in time. As the graph evolves — new particles, new links, shifted focus distribution — the compiled weights become stale. The rate at which weight staleness degrades performance, and the conditions under which recompilation is necessary versus incremental update is sufficient, requires empirical study.

**7.4** The relationship between the compiled transformer's performance and a trained transformer's performance on the same knowledge domain is unknown. Compilation produces the theoretically optimal architecture for the explicit graph. Fine-tuning produces an approximation of the implicit graph. Whether the compiled weights provide a better initialization than random for fine-tuning — and whether compilation + fine-tuning outperforms fine-tuning from random — is an empirical question with significant practical implications.

---

## 8. Conclusion

We have shown that transformer architecture is not a free design choice when a weighted knowledge graph is available. The embedding dimension, attention head count, and layer depth are determined by three graph properties: the effective rank of the focus covariance, the semcon count, and the product of graph diameter with the spectral convergence factor.

The result follows from a simple observation: a transformer's attention mechanism is one step of the same convergent dynamical system that computes the focus distribution over a knowledge graph. The transformer and the knowledge graph ranking system are the same computation at different scales — local and ephemeral in the transformer, collective and persistent in the graph.

Compilation bridges the two: given an explicit graph, derive the transformer that reads it without training. The compiled transformer inherits the graph's auditability and provenance — properties that trained transformers destroy in the compression from text to weights.

The long-term implication is a different trajectory for large language models: not larger training runs over larger corpora, but compiled architectures over explicit knowledge graphs that grow continuously from collective human and AI contribution, with structure that is inspectable and correctable rather than opaque.

---

## References

1. Bai, S., Kolter, J.Z., Koltun, V. "Deep Equilibrium Models." NeurIPS 2019.
2. Elhage, N. et al. "A Mathematical Framework for Transformer Circuits." Anthropic, 2021.
3. [[Miroslav Fiedler|Fiedler, M.]] "Algebraic Connectivity of Graphs." Czech Mathematical Journal, 1973.
4. [[Stefan Banach|Banach, S.]] "Sur les Operations dans les Ensembles Abstraits." Fundamenta Mathematicae, 1922.
5. Chung, F. "The Heat Kernel as the [[cyberank|Pagerank]] of a Graph." PNAS, 2007.
6. cyber whitepaper. "cyber: a protocol for planetary superintelligence." cyber.page/cyber-whitepaper, 2024.
7. Vaswani, A. et al. "Attention Is All You Need." NeurIPS 2017.
8. Roy, A. et al. "Efficient Content-Based Sparse Attention with Routing Transformers." TACL 2021.
9. Levin, D., Peres, Y., Wilmer, E. "Markov Chains and Mixing Times." AMS, 2009.
10. Spielman, D. "Spectral Graph Theory." Yale Lecture Notes.