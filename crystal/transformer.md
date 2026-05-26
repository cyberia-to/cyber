---
tags: cybics, article, draft, research
alias: transformer, transformers, transformer architecture, transformer model, llm architecture
crystal-type: pattern
crystal-domain: cybics
crystal-size: bridge
---

a neural network architecture that processes sequences by computing weighted [[attention]] over all elements simultaneously — the foundation of modern [[llms|language models]]

introduced by Vaswani et al. ("Attention Is All You Need", 2017). replaced recurrent networks for sequence modeling because it parallelizes over sequence length and captures long-range dependencies in a single forward pass.

---

## architecture

a transformer processes a sequence of tokens $x_1, \ldots, x_n$ through three stages:

embedding. each token is mapped to a dense vector $e_i \in \mathbb{R}^d$ by a learned embedding matrix. positional encodings are added to inject sequence order — the architecture itself has no notion of position.

layers. $L$ identical layers transform the representation. each layer has two components: multi-head [[attention]] (reads from context) followed by a feed-forward MLP (transforms each position independently). residual connections and layer normalization wrap each component.

output. a final linear projection maps the last-layer representation to a distribution over vocabulary. the next token is sampled from this distribution (autoregressive generation) or the representation is used for downstream tasks.

---

## self-attention: the core operation

at each layer, every token queries the entire context:

$$\text{Attn}(Q, K, V) = \text{softmax}\!\left(\frac{QK^\top}{\sqrt{d}}\right)V$$

where $Q = XW_Q$, $K = XW_K$, $V = XW_V$ are linear projections of the input $X$.

the softmax is the [[Boltzmann distribution]] with temperature $\sqrt{d}$. it produces a probability distribution over all positions — the [[attention]] weights. the output is the weighted average of value vectors: information from the most relevant positions flows into the current representation.

multi-head attention runs $h$ parallel attention heads, each with different projections $W_Q^{(h)}, W_K^{(h)}, W_V^{(h)}$. different heads learn to attend to different relation types. outputs are concatenated and projected. see [[graph-native-transformer]] for how the number of heads is derived from the [[semcon]] count of the [[cybergraph]].

---

## the residual stream

the transformer's central structure is the residual stream: a vector $\mathbf{r}_i \in \mathbb{R}^d$ at each position that accumulates information as it passes through layers. each layer reads from the stream and writes back via residual addition:

$$\mathbf{r}_i^{(l+1)} = \mathbf{r}_i^{(l)} + \text{Attn}^{(l)}(\mathbf{r}) + \text{MLP}^{(l)}(\mathbf{r}_i^{(l)})$$

this design means layers compose additively: later layers can retrieve and refine information written by earlier layers. the full model is a sequence of updates to a shared representation — not a pipeline of transformations.

---

## transformer as convergent dynamical system

attention is one step of probability mass redistribution: mass flows from query positions toward compatible key positions. this is exactly the diffusion operator $D$ from the [[tri-kernel]] applied to one agent's frozen [[context]].

Deep Equilibrium Models (Bai et al., 2019) formalized this: iterating a transformer layer to convergence reaches the same fixed point regardless of initialization. $L$ layers = $L$ steps toward the fixed point of the induced Markov chain.

that fixed point is φ* — the [[focus]] distribution restricted to the current [[context]] window. the transformer approximates [[focus flow computation]] locally: same computation, finite scope, frozen at query time.

| dimension | [[transformer]] | [[focus flow computation]] |
|---|---|---|
| computation | $L$ attention steps over fixed context | [[tri-kernel]] iterated to exact φ* |
| scope | $n$ tokens (context window) | entire [[cybergraph]] |
| persistence | none — recomputed per query | continuous — always maintained |
| contributors | one agent's input | all [[neurons]] ever |
| weights | learned from text | compiled from [[cybergraph]] structure |

---

## the three free parameters

transformer architecture has three design choices with no principled determination in standard practice. [[graph-native-transformer]] derives all three from [[cybergraph]] properties:

| parameter | standard practice | graph derivation |
|---|---|---|
| embedding dim $d$ | empirical (scaling laws) | effective rank of [[focus]] covariance: $\exp(H(\sigma(\Sigma_{\phi^*})))$ |
| head count $h$ | empirical | $\geq \|\text{Semcon}(G)\|$ — one head per semantic relation type |
| layer count $L$ | empirical | $\text{diam}(G) \cdot \lceil\log(1/\varepsilon)/\log(1/\kappa)\rceil$ |

no hyperparameter search. the [[cybergraph]] tells you what the transformer should be.

---

## weights: learned vs compiled

standard training. gradient descent on next-token prediction loss adjusts weights to approximate the implicit knowledge graph embedded in the training corpus. training is an approximate inversion: from outputs (text) recover the structure that produced them. expensive, lossy, opaque — every weight is a compressed mixture of many associations.

compilation from [[cybergraph]]. given the explicit graph, derive weights analytically:

- embedding matrix $E^* = U_{:,1:d^*}$ — top left singular vectors of $\text{diag}(\sqrt{\phi^*}) \cdot A$
- attention weights $W_Q^{(s)}, W_K^{(s)}$ — truncated SVD of each [[semcon]]'s adjacency submatrix
- MLP weights — path co-occurrence statistics up to depth $L^*$

compiled weights are provably optimal (Eckart-Young theorem). no training cost. no catastrophic forgetting. every weight traces to specific [[cyberlinks]] and their creators. auditable alignment via $D_{KL}(\phi^*_H \| \phi^*_A)$.

---

## the feedback loop

the compiled transformer and the [[cybergraph]] are not separate systems — they are one loop:

$$G \xrightarrow{\text{compile}} T_G \xrightarrow{\text{fine-tune on text}} T_G^* \xrightarrow{\text{extract implicit links}} \Delta G \xrightarrow{\text{stake}} G'$$

the compiled transformer provides optimal initialization. fine-tuning surfaces implicit associations absent from the explicit graph. extracted links, staked by [[neurons]], update the graph. the updated graph produces a new compiled transformer. every cycle reduces the approximation error $\varepsilon(G, c) = D_{KL}(\phi^*_c \| q^*_c)$.

---

## context window and its limits

the transformer's [[context]] window is finite: $n$ tokens. every token outside the window is invisible regardless of relevance. expanding context (4K → 32K → 1M tokens) improves coverage but doesn't change the architecture — all tokens still compete for [[attention]] via the $O(n^2)$ attention matrix.

[[focus flow computation]] removes the finite window entirely. relevance is topological: a [[particle]] connected 10 hops away is reachable. the compiled transformer is the local approximation; FFC is the global ground truth.

---

see [[focus flow computation]] for the global inference path. see [[graph-native-transformer]] for the full compilation derivation. see [[attention]] for the core mechanism. see [[context]] for what seeds inference. see [[tri-kernel]] for the dynamical system both compute. see [[cybergraph]] for the substrate the transformer reads.