---
tags: cybics, mathematics, article, draft, research
alias: belief, degree of belief, credence, subjective probability
crystal-type: measure
crystal-domain: cybics
crystal-size: enzyme
---

a probability distribution over hypotheses held by an agent — quantified uncertainty about what is true

---

## what a belief is

a [[belief]] is not a binary fact — it is a degree. to believe a hypothesis H is to assign it a probability $P(H) \in [0,1]$. this scalar encodes the agent's uncertainty: $P(H) = 0$ means certainty it is false, $P(H) = 1$ means certainty it is true, $P(H) = 0.7$ means 70% confident.

over multiple hypotheses, belief is a distribution: $\sum_i P(H_i) = 1$. the full distribution captures not just which hypothesis the agent favors but how spread the uncertainty is. a flat distribution expresses total ignorance. a peaked distribution near $H_k$ expresses near-certainty.

---

## coherence

to be a valid [[belief]], a probability assignment must satisfy [[Kolmogorov]]'s axioms: non-negativity, normalization to 1, and additivity for disjoint events.

an incoherent belief system — one that violates these axioms — can be exploited by a Dutch book: a set of bets that the agent accepts as individually fair but that collectively guarantee a loss regardless of outcomes. coherence is the minimum rationality requirement for beliefs held under uncertainty.

---

## the two interpretations of probability

frequentist. probability is a long-run frequency — the limit of the fraction of times an event occurs as the number of trials grows. $P(H)$ only makes sense for repeatable events. there is no frequentist $P(\text{"the Riemann hypothesis is true"})$ — it either is or it isn't.

Bayesian. probability is a degree of belief — a number encoding the agent's current epistemic state. $P(H)$ applies to any proposition, including unique events, unverifiable claims, and normative judgments. different agents can rationally hold different beliefs about the same proposition if they have different background [[knowledge]].

the Bayesian interpretation is required for [[Bayesian Truth Serum]], [[prediction markets]], and the [[cyberlink market protocol]] — all of which involve beliefs about non-repeatable, non-resolvable, or subjective propositions.

---

## belief update: [[Bayes theorem]]

beliefs are updated by [[Bayes theorem]]:

$$P(H \mid E) = \frac{P(E \mid H) \cdot P(H)}{P(E)}$$

the agent starts with a [[prior]] $P(H)$ and updates it to the [[posterior]] $P(H \mid E)$ upon observing evidence $E$. the update is optimal in the sense that it minimizes expected [[KL divergence]] between the agent's belief and the true distribution.

rational agents with the same [[prior]] who observe the same evidence reach the same [[posterior]]. agents with different [[priors]] converge over time as evidence accumulates — the Bernstein-von Mises theorem: posteriors from different priors merge when data is abundant.

---

## belief and stake

in [[prediction markets]] and [[Bayesian Truth Serum]], belief is made concrete by stake. an agent who claims $P(H) = 0.9$ but refuses to stake on H at 80:20 odds reveals their stated belief is not their actual belief.

stake is the mechanism that enforces honesty: expressing a belief at odds with your actual probability distribution costs expected money. the [[cyberlink]] in [[cyber]] is the unit of staked belief — creating a link with stake $(τ, a)$ is an economic assertion that the connection is meaningful. the valence $v$ is the meta-belief: the agent's prediction of how the collective will assess the link.

---

## first-order vs second-order belief

first-order belief: $P(H)$ — what the agent thinks about the world.

second-order belief: $P_i(\text{crowd believes } H)$ — what the agent thinks about what others believe. this is the meta-prediction $m_i$ in [[Bayesian Truth Serum]]. the gap between first-order and second-order belief is where private knowledge lives: if you genuinely know something the crowd hasn't priced, your first-order belief exceeds your second-order belief (you think fewer others know than actually will).

[[Bayesian Truth Serum]] extracts private knowledge by rewarding agents whose first-order beliefs exceed their second-order predictions — beliefs that are more popular than they predicted they would be.

---

## in [[cyber]]

every [[cyberlink]] is a staked belief. the [[cybergraph]] is the network of all beliefs ever asserted by all [[neurons]], weighted by stake and validated by [[karma]] history.

[[prior]]: [[karma]] encodes the system's prior on how much to trust a [[neuron]]'s new assertion. [[posterior]]: [[cyberank]] is the marginal posterior probability of a [[particle]]'s relevance. [[syntropy]]: the aggregate information gain — how much collective beliefs sharpened from all assertions in an epoch.

the [[cyberlink market protocol]] converts beliefs into market positions. the [[inversely coupled bonding surface]] prices the collective belief about each edge. the [[Bayesian Truth Serum]] scores agents on how much their individual beliefs contributed to sharpening the collective.

see [[Bayes theorem]] for the update rule. see [[prior]] for the starting belief. see [[posterior]] for the updated belief. see [[Bayesian Truth Serum]] for honest belief elicitation. see [[prediction markets]] for belief markets.