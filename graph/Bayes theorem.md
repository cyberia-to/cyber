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

## likelihood

$P(E \mid H)$ evaluated as a function of $H$ with $E$ fixed is the likelihood function:

$$\mathcal{L}(H) = P(E \mid H)$$

same formula, different reading. when you fix the data and vary the hypothesis, it is no longer a probability distribution over $E$ — it is a function over $H$ that measures how well each hypothesis explains the observed evidence. the likelihood does not integrate to 1 over $H$.

the likelihood is the evidence's vote: hypotheses that make the observed data more probable receive higher likelihood. the ratio $\mathcal{L}(H_1) / \mathcal{L}(H_2)$ is the likelihood ratio — how much more the data supports $H_1$ over $H_2$, independent of the [[prior]].

log-likelihood: $\ell(H) = \ln P(E \mid H)$. for independent observations $E = \{e_1, \ldots, e_n\}$, the log-likelihood sums rather than multiplies:

$$\ell(H) = \sum_{i=1}^n \ln P(e_i \mid H)$$

this makes computation tractable and converts the product of small probabilities into a sum that doesn't underflow.

maximum likelihood estimation (MLE) selects the hypothesis that maximizes the likelihood:

$$\hat{H}_{\text{MLE}} = \arg\max_H \mathcal{L}(H) = \arg\max_H P(E \mid H)$$

MLE is Bayesian inference with a flat [[prior]]: when all hypotheses are equally probable a priori, the [[posterior]] is proportional to the likelihood, so the maximum posterior = maximum likelihood.

the likelihood principle: all evidence about $H$ contained in the data is captured by $\mathcal{L}(H)$. two datasets with proportional likelihoods carry identical evidence about $H$, regardless of how they were generated.

---

## evidence

$P(E)$ — the marginal probability of the observed evidence under all hypotheses:

$$P(E) = \sum_h P(E \mid H=h) \cdot P(H=h) = \int P(E \mid H) \cdot P(H)\, dH$$

three roles evidence plays:

normalizing constant. $P(E)$ ensures the [[posterior]] sums to 1 over all hypotheses. it is the total probability mass of observing $E$ across the entire hypothesis space. without it, the Bayes formula gives an unnormalized score proportional to the posterior but not a probability.

model evidence. for model comparison, $P(E \mid \mathcal{M})$ — the probability of the data under model $\mathcal{M}$ — is the measure of how well the model fits. two models $\mathcal{M}_1$ and $\mathcal{M}_2$ are compared by the Bayes factor:

$$\text{BF} = \frac{P(E \mid \mathcal{M}_1)}{P(E \mid \mathcal{M}_2)}$$

the Bayes factor is the ratio of evidences — how much more the data supports one model than another, after integrating out all hypotheses. it automatically penalizes model complexity (Occam's razor emerges from the math: overly complex models spread probability thinly over many hypotheses, reducing marginal likelihood).

computational bottleneck. computing $P(E) = \int P(E \mid H) P(H)\, dH$ requires integrating over the entire hypothesis space. this integral is analytically tractable only for conjugate priors. for everything else it requires approximate methods: MCMC (samples from the posterior), variational inference (approximates the posterior with a tractable family), or importance sampling (estimates the integral by reweighting samples from the [[prior]]).

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
