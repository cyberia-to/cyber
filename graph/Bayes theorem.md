---
tags: cybics, mathematics, article, draft, research
alias: Bayes theorem, Bayes' theorem, Bayes rule, Bayesian inference, Bayes formula
crystal-type: pattern
crystal-domain: cybics
crystal-size: bridge
---

the rule for updating beliefs in light of evidence — how probability flows from what you assumed (prior) to what you now conclude (posterior) after observing data

$$P(H \mid E) = \frac{P(E \mid H) \cdot P(H)}{P(E)}$$

---

## the four terms

| term | name | meaning |
|---|---|---|
| $P(H \mid E)$ | [[posterior]] | probability of hypothesis H after seeing evidence E |
| $P(E \mid H)$ | likelihood | probability of seeing E if H were true |
| $P(H)$ | [[prior]] | probability of H before seeing E |
| $P(E)$ | evidence | total probability of E under all hypotheses — a normalizing constant |

the key inversion: you usually know $P(E \mid H)$ (how likely the evidence given the hypothesis) but you want $P(H \mid E)$ (how likely the hypothesis given the evidence). Bayes theorem bridges the two directions.

---

## the update loop

today's [[posterior]] is tomorrow's [[prior]]. Bayes theorem is not a one-shot formula — it is a protocol for continuous belief revision:

$$P(H \mid E_1, E_2) = \frac{P(E_2 \mid H) \cdot P(H \mid E_1)}{P(E_2 \mid E_1)}$$

each observation shifts the distribution. the order of updates doesn't matter when observations are conditionally independent given H. the posterior after two updates equals the result of applying both updates in sequence in either order.

this sequential structure makes Bayes theorem the natural language for learning: each piece of evidence is a message that sharpens the distribution. accumulating messages converges toward the truth at the maximum rate consistent with the information received.

---

## likelihood and the direction of inference

$P(E \mid H)$ is a model of the world: "if H is true, what evidence would we expect?" this is the direction of generative models — from hypothesis to prediction.

$P(H \mid E)$ is the direction of inference: from observation to hypothesis. science runs in the inference direction; experiments run in the generative direction. Bayes theorem is the bridge between them.

---

## the denominator

$P(E) = \sum_h P(E \mid H=h) \cdot P(H=h)$ — the sum over all hypotheses of the probability of the evidence under each. for continuous hypotheses: $P(E) = \int P(E \mid H=h) \cdot P(H=h)\, dh$.

the denominator is the hardest part computationally — summing over all hypotheses is intractable for large spaces. most of Bayesian computation (MCMC, variational inference, expectation-propagation) is algorithms for approximating or avoiding the denominator.

---

## frequentist vs Bayesian

frequentist probability: $P(E)$ is a long-run frequency — the probability that event E would occur over many repetitions of the same experiment. $P(H)$ makes no sense in frequentist terms because the hypothesis is either true or false — not a frequency.

Bayesian probability: $P(H)$ is a [[belief]] — a degree of certainty held by an agent. it encodes what the agent knows, not an objective feature of the world. two agents with different priors will reach different posteriors from the same evidence. over time, with enough evidence, posteriors converge regardless of prior (Bernstein-von Mises theorem).

---

## connection to [[KL divergence]]

the Bayesian update minimizes [[KL divergence]] between the posterior and the true data-generating distribution. the log-likelihood $\log P(E \mid H)$ is the information the evidence provides about H. the posterior is the distribution closest to the prior that correctly accounts for that information.

learning = reduction in $D_{KL}(\text{posterior} \| \text{true distribution})$. this is the same objective that [[veritas]] and [[Bayesian Truth Serum]] optimize: moving the collective [[belief]] closer to the ground truth.

---

## in [[cyber]]

every [[cyberlink]] is a Bayesian observation. creating E→Q is evidence that Q is relevant in the context of E. the [[tri-kernel]] accumulates these observations and computes π* — the posterior over which [[particles]] deserve [[focus]] given all evidence ever submitted to the [[cybergraph]].

[[karma]] is the [[prior]] on a [[neuron]]'s reliability — before seeing their new link, the system has a prior on how much weight to assign it. [[cyberank]] is the current marginal posterior probability of a [[particle]]'s relevance. [[syntropy]] measures information gain — how much each new [[cyberlink]] shifts the posterior.

the [[Bayesian Truth Serum]] mechanism is a proper implementation of Bayes theorem applied to [[belief]] elicitation: the scoring formula computes how much each agent's report updated the collective posterior versus how much was already implied by others' priors.

see [[prior]] for the starting distribution. see [[posterior]] for the updated distribution. see [[Bayesian network]] for the graphical model. see [[belief]] for the subjective probability interpretation. see [[KL divergence]] for the information-theoretic measure.
