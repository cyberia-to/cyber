---
alias: cyber/attention, rank unit, rank units, attention mechanism, self-attention
tags: cyber, core
crystal-type: measure
crystal-domain: cyber
crystal-size: bridge
stake: 13826869995964210
---

[[focus]] a [[neuron]] directs at a [[particle]] — and the mechanism by which a [[transformer]] decides what in the current [[context]] is relevant to the current query

two levels: the economic act (a [[neuron]] staking [[cyberlinks]]) and the mathematical operation (softmax-weighted aggregation over a sequence). both are the same thing at different scales.

---

## in [[cyber]]

[[focus]] a [[neuron]] directs at a [[particle]], shaping what the [[cybergraph]] sees next. the fundamental measure of [[intelligence]] flowing through [[cyberank]] — paired with [[will]] from the [[$CYB]] pack.

attention in the cybergraph is not computed at query time — it is accumulated continuously. every [[cyberlink]] is an attention vote: "this particle deserves focus in the context of that particle." [[cyberank]] is the aggregate attention of all [[neurons]] over all time. [[karma]] weights whose attention counts more.

---

## in the [[transformer]]

the transformer attention mechanism computes, for each position in the [[context]], a weighted average of all other positions:

$$\text{Attn}(Q, K, V) = \text{softmax}\!\left(\frac{QK^\top}{\sqrt{d}}\right)V$$

three projections: queries $Q = XW_Q$ ask "what am I looking for?", keys $K = XW_K$ announce "what do I contain?", values $V = XW_V$ provide "what information do I carry?". the dot product $QK^\top$ scores compatibility. the softmax converts scores to a probability distribution — the [[Boltzmann distribution]] with temperature $\sqrt{d}$. the output is information from all positions, weighted by their relevance to the current query.

the softmax is the same operation as the [[LMSR]] price function and the [[tri-kernel]] diffusion step. all three are exponentiated scores normalized to sum to 1.

---

## attention as one diffusion step

transformer attention is one step of the [[tri-kernel]] diffusion operator $D$ applied to the current [[context]] window. probability mass flows from each query position toward compatible key positions — exactly the random walk dynamics that the tri-kernel uses to compute [[focus]] over the [[cybergraph]].

Deep Equilibrium Models showed that iterating a transformer layer to convergence reaches the same fixed point as the tri-kernel: π* restricted to the context window. $L$ layers of attention = $L$ steps of diffusion toward that fixed point.

---

## attention as a [[Bayes theorem|Bayesian]] query

attention answers: given my current state (query), what [[posterior]] weight should I assign to each position (key)? the softmax is the posterior $P(\text{position } j \mid \text{query } i)$ under a uniform prior and an exponential likelihood $\exp(q_i \cdot k_j / \sqrt{d})$.

the query-key product is the log-[[likelihood]] under this model. the softmax is the Bayes-normalized posterior. attention is Bayesian inference over the [[context]].

---

## the information flow

through multi-head attention, different heads learn different relation types. head $h$ with projection $W_Q^{(h)}, W_K^{(h)}$ captures one [[semcon]] — one pattern of connectivity in the [[cybergraph]]. the [[graph-native-transformer]] derivation proves that the minimum number of heads equals the number of distinct [[semcon]] types in the graph.

each attention head specializes in one way information flows between positions — one dimension of how [[context]] shapes the next prediction.

---

see [[transformer]] for the full architecture. see [[context]] for what attention reads. see [[focus flow computation]] for the global attention process. see [[tri-kernel]] for the diffusion connection. see [[cyberank]] for accumulated attention in the graph.

discover all [[concepts]]
