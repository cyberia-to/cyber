---
tags: cybics, mathematics, article, draft, research
alias: prior, prior probability, prior distribution, prior belief
crystal-type: measure
crystal-domain: cybics
crystal-size: enzyme
---

the [[belief]] an agent holds before observing evidence — the starting distribution in [[Bayes theorem]]

$$P(H) \quad \text{(before evidence } E \text{)}$$

---

## what a prior encodes

a prior is not ignorance — it is everything the agent knows before the current observation. it encodes background [[knowledge]], theoretical constraints, past experience, and assumptions about the structure of the problem.

two agents with different priors will update differently from the same evidence. this is not irrational: they are starting from different epistemic positions. over enough evidence, their [[posteriors]] will converge (Bernstein-von Mises theorem), but the speed of convergence depends on how far the priors are from the truth.

---

## types of prior

uninformative (flat) prior. assigns equal probability to all hypotheses — maximum [[entropy]] prior, Laplace's principle of indifference. expresses: "I have no reason to favor any hypothesis." problematic because "uniform" depends on the parameterization — a flat prior over $\theta$ is not flat over $\theta^2$.

Jeffreys prior. invariant under reparameterization: $p(\theta) \propto \sqrt{I(\theta)}$ where $I(\theta)$ is the Fisher information. the canonical uninformative prior. expresses genuine ignorance rather than arbitrary flatness.

informative prior. encodes domain knowledge, physical constraints, or theoretical structure. a prior that $P(\text{coin is fair}) = 0.99$ reflects manufacturing knowledge, not wishful thinking.

conjugate prior. chosen so that the [[posterior]] stays in the same distributional family as the prior. the Beta distribution is conjugate to the Binomial; the Gaussian is self-conjugate. conjugate priors make Bayesian updates analytically tractable.

---

## the prior as accumulated experience

in sequential Bayesian learning, today's [[posterior]] is tomorrow's prior. this means the prior at any moment is a compressed summary of all previous evidence:

$$P(H \mid E_1, \ldots, E_{n-1}) \xrightarrow{\text{becomes}} P_n(H)$$

the prior is not arbitrary — it is earned. an agent who has processed much evidence has an informative prior grounded in that experience. an agent who has processed none has a diffuse prior expressing genuine ignorance.

---

## in [[cyber]]

[[karma]] is the prior on [[neuron]] reliability. before seeing a [[neuron]]'s new [[cyberlink]], the system has a prior on how much weight to assign it:

$$\text{prior on neuron quality} = \kappa(\nu) = \text{accumulated BTS score history}$$

a [[neuron]] with high [[karma]] has a strong informative prior in its favor. a new [[neuron]] has a diffuse prior — the system waits for evidence before trusting heavily.

the [[tri-kernel]]'s initial state before any [[cyberlinks]] exist is the maximum-[[entropy]] prior over [[particles]] — uniform [[focus]] distribution $\phi^{(0)} = \mathbf{1}/|P|$. each [[cyberlink]] is evidence that updates this distribution toward φ*.

the [[cyberlink market protocol]]'s initial ICBS deposit at 50/50 — equal reserves in YES and NO — is the uninformative prior on each edge: genuine uncertainty about whether the link will be validated.

see [[Bayes theorem]] for the update rule. see [[posterior]] for the updated distribution. see [[belief]] for the subjective probability interpretation. see [[karma]] for the network-level prior on neuron quality.