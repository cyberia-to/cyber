---
tags: cybics, mathematics, article, draft, research
alias: posterior, posterior probability, posterior distribution, posterior belief
crystal-type: measure
crystal-domain: cybics
crystal-size: enzyme
---

the [[belief]] an agent holds after observing evidence — the output of [[Bayes theorem]]

$$P(H \mid E) = \frac{P(E \mid H) \cdot P(H)}{P(E)}$$

---

## what the posterior encodes

the posterior $P(H \mid E)$ is the optimal belief given both the [[prior]] $P(H)$ and the evidence $E$. it compresses all information: once you have the posterior, you no longer need to store the raw evidence. the posterior is a sufficient statistic for future inference.

two components drive the update:

the likelihood $P(E \mid H)$ is the evidence's vote: how much more probable is the evidence under H than under alternative hypotheses? large likelihood relative to alternatives → large posterior shift.

the prior $P(H)$ is the background belief. a very improbable hypothesis requires overwhelming evidence to reach high posterior — this is the formal basis for Hume's maxim that extraordinary claims require extraordinary evidence.

---

## sequential update

Bayesian updating is a Markov chain: each [[posterior]] becomes the [[prior]] for the next observation:

$$P(H \mid E_1) \xrightarrow{E_2} P(H \mid E_1, E_2)$$

when observations are conditionally independent given H, the order of updates doesn't matter. the joint posterior after $n$ observations is the same regardless of the sequence.

this sequential structure is computationally important: you don't need to store all past evidence — just the current posterior. each update is $O(|\mathcal{H}|)$ in the hypothesis space, not in the size of the data.

---

## posterior as the target of epistemics

the posterior is what any epistemically rational agent should believe, given their [[prior]] and the evidence available. it is the answer to "what should I believe now?" — not "what is the absolute truth?" (which may be unknowable).

all of Bayesian decision theory flows from the posterior: optimal decisions under uncertainty are those that maximize expected utility under the posterior. rational [[belief]], rational action, and rational inference all converge at the posterior distribution.

---

## posterior concentration

as evidence accumulates, the posterior concentrates around the true hypothesis (under regularity conditions — the Bernstein-von Mises theorem). the rate of concentration is governed by the [[KL divergence]] between the data-generating distribution and the model:

$$D_{KL}(P_\text{true} \| P_\theta) \to 0 \quad \text{as } n \to \infty$$

this means all agents with any non-zero prior on the truth will eventually agree — Bayesian learning is self-correcting. the [[prior]] matters less and less as evidence grows.

---

## in [[cyber]]

φ* — the [[focus]] distribution computed by the [[tri-kernel]] — is the posterior over [[particle]] relevance given all [[cyberlinks]] ever submitted to the [[cybergraph]]. each [[cyberlink]] is evidence. φ* is the posterior that integrates all evidence from all [[neurons]], weighted by [[karma]] (the prior on their reliability) and by ICBS market prices (the collective belief about each edge's validity).

the [[approximation quality metric]] $\varepsilon(G,c) = D_{KL}(\phi^*_c \| q^*_c)$ measures how much the compiled [[transformer]] deviates from the exact posterior. the [[collective focus theorem]] proves that φ* is the unique posterior that the [[tri-kernel]] converges to from any initial [[prior]] under ergodicity.

[[cyberank]] is the marginal posterior probability of a [[particle]]'s relevance. [[syntropy]] is the total information gain — the total shift in the posterior from its initial uninformative state.

the [[cyberlink market protocol]]'s ICBS price for each edge is the collective posterior on that edge's validity: $q = r_{YES}/(r_{YES} + r_{NO})$, continuously updated as participants submit evidence (trades).

see [[Bayes theorem]] for the update rule. see [[prior]] for the starting distribution. see [[belief]] for the subjective probability interpretation. see [[focus flow computation]] for how φ* is computed.