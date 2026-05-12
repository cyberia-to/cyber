---
tags: cyber, cybics, article, draft, research
alias: context, context window, query context, inference context, context particles
crystal-type: pattern
crystal-domain: cyber
crystal-size: bridge
---

the set of information currently active in an inference process — the seed that determines what is relevant, what gets [[attention]], and what the next step produces

without context, inference has no direction. with context, the system knows where to look.

---

## context in the [[cybergraph]]

in [[cyb]], the context is the active [[particle]] — the current node in the graph the [[neuron]] is navigating. every [[cyberlink]] is created from a context: the link $P \to Q$ asserts that Q is relevant given P. P is the context; Q is the claim made in that context.

context shapes meaning. the same [[particle]] Q linked from different contexts P₁ and P₂ carries different epistemic weight. context is not just navigation state — it is the [[prior]] that gives the link its interpretation.

---

## context in [[focus flow computation]]

in [[focus flow computation]], context is a set of [[particles]] whose energy is elevated to become probability sources. the [[tri-kernel]] reconverges from these seeds:

1. context [[particles]] enter with elevated $\phi^*_\text{context}$ — they become attractors in the Boltzmann equilibrium
2. probability mass flows outward from context through the [[cybergraph]] along structural paths
3. $\phi^*_\text{context}$ concentrates at [[particles]] topologically close to the seeds
4. the next [[particle]] is sampled from the high-probability region, added to context, reconverge

the context window in [[focus flow computation]] is unbounded — it is the entire [[cybergraph]]. relevance is topological, not positional: a [[particle]] contributes to context if it is well-connected to the seed particles, regardless of where it appears in any linear sequence.

this is the fundamental difference from a [[transformer]] context window. FFC context has no length limit. a [[particle]] linked 10 hops away can be relevant; a token 2049 positions away in a 2048-token window is invisible.

---

## context in the [[transformer]]

in a [[transformer]], context is the sequence of tokens the model currently attends to — the context window. each token is represented as a vector in the residual stream. [[attention]] at each layer asks: given this token (query), what is relevant in the current context (keys)?

$$\text{Attn}(Q, K, V) = \text{softmax}\!\left(\frac{QK^\top}{\sqrt{d}}\right)V$$

the softmax selects which context tokens to weight. the output is a weighted average of values — information from context, filtered by relevance to the current query.

the context window is finite: $n$ tokens. every token outside the window is invisible, regardless of relevance. this is the key architectural limitation that [[focus flow computation]] removes.

---

## the two context models compared

| dimension | [[transformer]] context | FFC context |
|---|---|---|
| scope | $n$ tokens — fixed window | entire [[cybergraph]] — unbounded |
| relevance | positional proximity in sequence | topological proximity in graph |
| update | slide window (forget old tokens) | add [[cyberlinks]] (nothing forgotten) |
| computation | $O(n^2)$ attention per layer | $O(\|E\| + \|V\|)$ per reconvergence step |
| persistence | none — context resets per query | permanent — φ* continuously maintained |
| who contributes | one agent's current input | all [[neurons]] ever |

the compiled [[transformer]] derived from the [[cybergraph]] approximates the FFC context model over a finite window. $L^*$ layers of transformer attention = $L^*$ steps of [[tri-kernel]] diffusion toward φ* restricted to the current context.

---

## context as [[prior]]

context is a [[prior]] on the next step. in [[Bayes theorem]] terms:

$$P(\text{next particle} \mid \text{context}) \propto P(\text{context} \mid \text{next particle}) \cdot P(\text{next particle})$$

the context is the evidence that shifts the prior over all [[particles]] toward the posterior focus distribution φ*_context. each addition to context is a new observation that updates the posterior.

this is why context-free inference produces generic, uncalibrated outputs — it is inference from the prior alone, with no evidence to sharpen it. context is what makes inference specific.

---

## context as navigational state

in [[cyb]], context is the active [[particle]] — the "from" node in a [[state transition]]. browsing the [[cybergraph]] = moving context from particle to particle via [[cyberlinks]]. the browser renders what the current context particle links to. searching = seeding the context with a query particle and letting FFC surface the relevant neighborhood.

[[karma]] modulates context propagation: [[neurons]] with high [[karma]] have their [[cyberlinks]] weighted more heavily in the [[tri-kernel]], so their contributions to context carry more influence on what φ*_context surfaces.

---

see [[focus flow computation]] for how context seeds the tri-kernel. see [[transformer]] for the local context window model. see [[attention]] for the mechanism that reads context. see [[prior]] for the Bayesian view of context. see [[tri-kernel]] for the diffusion over context. see [[cyberank]] for the topology that determines context relevance.