---
tags: cybics, mathematics, article, draft, research
alias: likelihood, likelihood function, likelihood ratio, log-likelihood, MLE, maximum likelihood
crystal-type: measure
crystal-domain: cybics
crystal-size: enzyme
---

$P(E \mid H)$ read as a function of $H$ with evidence $E$ fixed — how well each hypothesis explains the observed data

$$\mathcal{L}(H) = P(E \mid H)$$

same formula as [[Bayes theorem]], different reading. when you fix the data and vary the hypothesis, it is no longer a probability distribution over $E$ — it is a scoring function over hypotheses. the likelihood does not integrate to 1 over $H$.

---

## what it measures

the likelihood answers: if hypothesis $H$ were true, how probable is the data we actually observed? high likelihood = the hypothesis makes the observations unsurprising. low likelihood = the observations would be rare under this hypothesis.

the likelihood ratio $\mathcal{L}(H_1) / \mathcal{L}(H_2)$ compares two hypotheses head-to-head: how much more does the data support $H_1$ than $H_2$? this ratio is independent of the [[prior]] — it is the pure voice of the data.

---

## log-likelihood

for independent observations $E = \{e_1, \ldots, e_n\}$:

$$\ell(H) = \ln \mathcal{L}(H) = \sum_{i=1}^n \ln P(e_i \mid H)$$

products become sums. numerically: probabilities multiply toward underflow; log-probabilities sum safely. theoretically: the log-likelihood is the natural bridge to [[entropy]] and [[KL divergence]] — the expected log-likelihood under the true distribution is $-H(P_\text{true}, P_H)$, the negative cross-entropy.

---

## maximum likelihood estimation

MLE selects the hypothesis that maximizes the likelihood:

$$\hat{H}_{\text{MLE}} = \arg\max_H \mathcal{L}(H)$$

MLE is Bayesian inference with a flat [[prior]]: when all hypotheses are equally probable a priori, the [[posterior]] is proportional to the likelihood, so maximizing the posterior = maximizing the likelihood. MLE breaks from Bayesian inference only when the prior is non-uniform.

MLE is consistent (converges to the true $H$ as $n \to \infty$) and efficient (achieves the Cramér-Rao lower bound asymptotically). it is the standard for frequentist parameter estimation.

---

## the likelihood principle

all evidence in the data about $H$ is contained in $\mathcal{L}(H)$. two datasets with proportional likelihood functions carry identical evidence about $H$, regardless of how they were collected, how the experiment was designed, or what data could have been observed but wasn't.

the stopping rule doesn't matter. an experiment stopped at $n=100$ because 100 flips were planned, and the same experiment stopped because the 100th heads appeared, give proportional likelihoods and therefore identical evidence — even though their sampling distributions differ. this is a fundamental departure from frequentist inference (where $p$-values depend on the stopping rule).

---

## in [[Bayes theorem]]

in [[Bayes theorem]], the likelihood is the bridge between [[prior]] and [[posterior]]:

$$\underbrace{P(H \mid E)}_{\text{posterior}} \propto \underbrace{P(E \mid H)}_{\text{likelihood}} \cdot \underbrace{P(H)}_{\text{prior}}$$

the likelihood re-weights the prior: hypotheses consistent with the data get upweighted, inconsistent ones get downweighted. the [[evidence]] (denominator) normalizes the result.

---

## in [[cyber]]

every [[cyberlink]] created by a [[neuron]] is an implicit likelihood assertion: "if this connection is meaningful, I would expect the evidence I have seen." the stake $(τ, a)$ is the magnitude of the likelihood claim — how strongly the neuron asserts that the data (their knowledge, context, observation) supports this connection.

[[karma]] tracks the track record of a [[neuron]]'s likelihood estimates over time: did their assertions prove correct? a [[neuron]] with high [[karma]] has a track record of high likelihoods for connections the market later validated.

the [[Bayesian Truth Serum]] scoring formula contains the likelihood ratio implicitly: the information gain term $D_{KL}(p_i \| \bar{m}_{-i}) - D_{KL}(p_i \| \bar{p}_{-i})$ measures how much the agent's [[belief]] departs from the crowd's prediction, weighted by the agent's own probability assessment — a log-likelihood ratio over the crowd's prior.

see [[Bayes theorem]] for the update rule. see [[evidence]] for the normalizing denominator. see [[prior]] and [[posterior]] for the other terms. see [[KL divergence]] for the information-theoretic connection.