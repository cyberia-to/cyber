---
tags: cybics, mathematics, article, draft, research
alias: KL divergence, Kullback-Leibler divergence, relative entropy, information gain
crystal-type: measure
crystal-domain: cybics
crystal-size: enzyme
---

a measure of how much one probability distribution differs from another — the information lost when distribution Q is used to approximate the true distribution P

$$D_{KL}(P \| Q) = \sum_x P(x) \log \frac{P(x)}{Q(x)}$$

for continuous distributions: $D_{KL}(P \| Q) = \int p(x) \log \frac{p(x)}{q(x)}\, dx$

---

## what it measures

$D_{KL}(P \| Q)$ answers: if the true distribution is P but you encode using Q, how many extra bits per symbol do you pay?

three properties govern its behavior:

non-negativity: $D_{KL}(P \| Q) \geq 0$, with equality iff $P = Q$ almost everywhere. the [[Claude Shannon|Shannon]]-Gibbs inequality: you always pay extra when your model is wrong.

asymmetry: $D_{KL}(P \| Q) \neq D_{KL}(Q \| P)$ in general. the direction matters. $D_{KL}(P \| Q)$ is large wherever P is large and Q is small — you underestimated the density that matters. $D_{KL}(Q \| P)$ is large wherever Q assigns mass that P does not.

additivity: for independent sources, $D_{KL}(P_1 \times P_2 \| Q_1 \times Q_2) = D_{KL}(P_1 \| Q_1) + D_{KL}(P_2 \| Q_2)$.

---

## relation to [[entropy]]

$D_{KL}(P \| Q) = H(P, Q) - H(P)$

where $H(P, Q)$ is cross-entropy and $H(P)$ is Shannon [[entropy]]. the KL divergence is the excess entropy from using the wrong model.

mutual information is a symmetric KL divergence:

$$I(X;Y) = D_{KL}(P(X,Y) \| P(X)P(Y))$$

how far the joint distribution is from the product of marginals — how much knowing X tells you about Y.

---

## in [[Bayesian Truth Serum]]

the BTS scoring formula decomposes into three KL terms:

$$s_i = \underbrace{D_{KL}(p_i \,\|\, \bar{m}_{-i}) - D_{KL}(p_i \,\|\, \bar{p}_{-i})}_{\text{information gain}} - \underbrace{D_{KL}(\bar{p}_{-i} \,\|\, m_i)}_{\text{prediction accuracy}}$$

the information gain term measures how much the agent's belief deviated from what others predicted, corrected by what others actually believed. a positive net score means the agent reduced collective uncertainty — they added information. a negative score means they added noise.

---

## in [[focus flow computation]]

the approximation quality metric $\varepsilon(G,c) = D_{KL}(\phi^*_c \| q^*_c)$ measures how much the compiled [[transformer]] deviates from the exact [[focus]] distribution. the same measure quantifies epistemic quality at three scales:

| scale | formula | what it measures |
|---|---|---|
| individual [[neuron]] | BTS score $s_i$ | one agent's information contribution |
| compiled model | $D_{KL}(\phi^*_c \| q^*_c)$ | approximation gap vs exact focus |
| collective state | $D_{KL}(\phi^*_\text{prior} \| \phi^*_\text{updated})$ | how much the graph learned |

---

## in [[veritas]]

learning in [[veritas]] occurs when collective uncertainty decreases — when the KL divergence between the prior distribution and the updated distribution shrinks. this is the signal that new [[information]] has been incorporated into the collective. stake flows from agents who increased divergence (noise) to agents who decreased it (signal).

---

## as the backbone of [[proper scoring rules]]

every strictly proper scoring rule is equivalent to a Bregman divergence, and the log-score proper scoring rule is equivalent to KL divergence. this means:

- [[Bayesian Truth Serum]] (peer prediction without oracle) — KL-based
- [[inversely coupled bonding surface]] settlement ($f_{YES} = x/q$) — log-score structure
- importance sampling weights — same inverse probability structure

all three are instances of the same information-theoretic object. see [[proper scoring rules]] for the unifying framework.

---

see [[Bayesian Truth Serum]] for the peer prediction application. see [[veritas]] for the truth-discovery protocol. see [[proper scoring rules]] for the broader scoring rule family. see [[entropy]] for the foundational measure. see [[focus flow computation]] for the approximation quality metric.