---
tags: cybics, mathematics, article, draft, research
alias: evidence, Bayesian evidence, marginal likelihood, model evidence, normalizing constant
crystal-type: measure
crystal-domain: cybics
crystal-size: enzyme
---

$P(E)$ — the total probability of observing the evidence across all hypotheses — the denominator in [[Bayes theorem]]

$$P(E) = \int P(E \mid H) \cdot P(H)\, dH$$

the marginal [[likelihood]]: the probability of the data after integrating out (marginalizing over) all possible hypotheses, weighted by the [[prior]].

---

## as normalizing constant

the denominator ensures the [[posterior]] is a valid probability distribution:

$$P(H \mid E) = \frac{P(E \mid H) \cdot P(H)}{P(E)}$$

without $P(E)$, the right side is proportional to the posterior but not normalized — it doesn't sum to 1 over $H$. $P(E)$ is the unique constant that makes it a probability distribution. for computation, many algorithms (MCMC, variational inference) work with the unnormalized numerator $P(E \mid H) \cdot P(H)$ and avoid computing $P(E)$ directly.

---

## as model evidence

for model comparison, $P(E \mid \mathcal{M})$ — the probability of the data under model $\mathcal{M}$ — measures how well the model fits. the Bayes factor compares two models directly:

$$\text{BF}_{12} = \frac{P(E \mid \mathcal{M}_1)}{P(E \mid \mathcal{M}_2)}$$

the Bayes factor is the update to the [[prior]] odds that the data provides. if $\text{BF}_{12} = 10$, the data is 10 times more probable under $\mathcal{M}_1$ than $\mathcal{M}_2$ — the posterior odds shift by a factor of 10 in favor of $\mathcal{M}_1$, regardless of the prior odds.

---

## Occam's razor from the math

the marginal likelihood automatically penalizes model complexity. a complex model spreads its prior probability over many hypotheses — it can fit the data well under many of them, but the average (the marginal likelihood integral) is lower than a simpler model that concentrates its prior on the data-generating region.

formally: $P(E \mid \mathcal{M}) = \mathbb{E}_{H \sim P(H|\mathcal{M})}[P(E \mid H)]$. a model that only fits the data well for a small region of hypothesis space will have high likelihood in that region but the integral over the whole prior is penalized by the small support. parsimony emerges from marginalization without any explicit complexity penalty.

---

## computational hardness

computing $P(E) = \int P(E \mid H) P(H)\, dH$ is analytically tractable only for conjugate prior-likelihood pairs. for everything else, approximation is necessary:

MCMC (Markov chain Monte Carlo). samples from the [[posterior]] $P(H \mid E) \propto P(E \mid H) P(H)$ without computing $P(E)$. the normalizing constant cancels in the acceptance ratio (Metropolis-Hastings). computationally expensive but asymptotically exact.

variational inference. approximates the [[posterior]] with a tractable family $q(H)$ by minimizing $D_{KL}(q \| P(H \mid E))$. this is equivalent to maximizing the ELBO (evidence lower bound): $\text{ELBO} = \mathbb{E}_q[\ln P(E \mid H)] - D_{KL}(q \| P(H))$. the ELBO is a lower bound on $\ln P(E)$.

importance sampling. estimates $P(E) = \mathbb{E}_{H \sim P(H)}[P(E \mid H)]$ by drawing samples from the [[prior]] and averaging their likelihoods. effective when the [[prior]] overlaps well with the likelihood. the same inverse-probability structure as [[proper scoring rules]] and [[inversely coupled bonding surface|ICBS]] settlement.

---

## in [[cyber]]

in the [[cybergraph]], the total evidence for a [[particle]]'s relevance is the marginal over all [[neurons]] who linked to it:

$$P(\text{particle } q \text{ is relevant}) \propto \sum_\nu \sum_{\ell: \text{src}=p,\, \text{tgt}=q} P(\text{link} \mid \nu) \cdot P(\nu)$$

where $P(\nu)$ is the [[prior]] on that [[neuron]] (their [[karma]]) and $P(\text{link} \mid \nu)$ is the [[likelihood]] that their link is informative. [[cyberank]] approximates this marginal: it integrates out individual [[neuron]] contributions into a single relevance score for each [[particle]].

the [[cyberlink market protocol]]'s ICBS reserve ratio $q = r_{YES}/(r_{YES} + r_{NO})$ is the collective evidence for an edge: the market has integrated the likelihoods asserted by all positions into a single posterior probability. it is the practical analog of $P(E)$ computed not by integration but by market aggregation.

see [[Bayes theorem]] for the full formula. see [[likelihood]] for the numerator term. see [[prior]] and [[posterior]] for the other distributions. see [[KL divergence]] for the information-theoretic measure of how much evidence shifts belief.