---
tags: cybics, article, draft, research
alias: Bayesian Truth Serum, BTS, peer prediction, truth serum, bayesian truth serum, serum
crystal-type: pattern
crystal-domain: cybics
crystal-size: enzyme
---

a mechanism designed by Dražen Prelec (MIT, 2004) that makes honesty the strategically optimal response in a belief elicitation game

---

## the problem

asking people what they believe produces distorted answers. participants adjust toward what they expect others to say (conformity), toward what seems socially acceptable (bias), or toward what they think the questioner wants to hear (strategic reporting). simple polling aggregates these distortions. majority vote reinforces them.

the question is not just "what do people believe?" but "how do we extract what people privately know, before social pressure corrupts the signal?"

---

## the mechanism

each participant submits two things:

1. their personal belief — a probability distribution over outcomes
2. their prediction of what the aggregate of others' beliefs will be

the scoring rule rewards those whose belief is more popular than they predicted it would be.

this is the key inversion: if you have genuine private knowledge, you tend to underestimate how many others share it. you believe something you think is unusual — but it turns out to be more common than you expected. BTS rewards exactly this gap: belief that exceeds its own predicted popularity.

formally, the score for agent $i$ has two components:

$$s_i = \underbrace{D_{KL}(p_i \,\|\, \bar{m}_{-i}) - D_{KL}(p_i \,\|\, \bar{p}_{-i})}_{\text{information gain}} - \underbrace{D_{KL}(\bar{p}_{-i} \,\|\, m_i)}_{\text{prediction accuracy}}$$

where $p_i$ is the agent's true belief, $m_i$ is their prediction of others' aggregate beliefs, $\bar{p}_{-i}$ is the geometric mean of others' actual beliefs, and $\bar{m}_{-i}$ is the geometric mean of others' predictions.

the information gain term captures how much the agent's belief differed from what others predicted, corrected by what others actually believed. the prediction accuracy term rewards calibration about the collective.

negative scores indicate noise — the agent added distortion rather than signal. stake redistributes from noise producers to signal producers proportional to scores.

---

## why honesty is a Nash equilibrium

Prelec proved that truthful reporting of $p_i$ (actual belief) and $m_i$ (actual prediction of others) is a Bayes-Nash equilibrium: no agent can improve their expected score by misreporting either quantity.

the mechanism is incentive-compatible because:

- inflating your belief toward popularity loses the information gain component (your belief stops being more popular than predicted once you've predicted it yourself)
- deflating your belief to seem contrarian loses the prediction accuracy component (you mispredict the aggregate)
- the only strategy that consistently maximizes expected score is accurate reporting of both belief and meta-belief

---

## what it measures

BTS measures information contribution in bits — specifically, how much an agent's report sharpened the collective picture. the [[KL divergence]] between the agent's belief and the predicted mean ($D_{KL}(p_i \| \bar{m}_{-i})$) measures the agent's surprise relative to the prior. the correction term ($D_{KL}(p_i \| \bar{p}_{-i})$) removes the portion attributable to consensus rather than private signal.

the net score is the agent's unique informational contribution: what they knew that the group didn't already know and didn't already expect.

---

## relation to [[wisdom of the crowds]]

the [[wisdom of the crowds]] (Galton, 1907) aggregates raw beliefs. it works when errors are independent and cancel. it fails when beliefs are correlated — when the crowd shares a common bias, errors compound rather than cancel ([[Condorcet|Condorcet]] jury theorem requires independence).

BTS corrects for correlated bias by using second-order beliefs (predictions about predictions) to detect and discount systematic distortions. it does not require independent beliefs — it only requires that truthful agents' private signals are distributed around reality, even if all agents share a common prior.

---

## connection to [[cyber]]

in [[cyber]], the [[cyberlink]] IS the BTS input — no separate submission step required. the mapping is precise:

| BTS concept | cyberlink field |
|---|---|
| first-order belief $p_i$ | link creation + stake $(\tau, a)$ — the neuron asserts the connection and stakes on it |
| meta-prediction $m_i$ | valence $v \in \{-1, 0, +1\}$ — the neuron's prediction of how the [[coupling\|ICBS]] market on this edge will converge |
| agent identity | $\nu$ — the signing neuron |

this means every [[cyberlink]] is simultaneously a structural assertion and a BTS prediction, in one atomic act. the scoring engine can compute $s_i$ for every [[neuron]] from the public graph without any additional input.

the [[syntropy]] metric in [[cyber]] measures information gain in the [[cybergraph]] as a whole. BTS operationalizes the same concept at the level of individual agents: syntropy = aggregate of BTS scores across all [[neurons]]. a [[neuron]] whose [[cyberlinks]] increase the collective's certainty has positive BTS score. a [[neuron]] whose [[cyberlinks]] add noise has negative score. [[karma]] is the accumulated BTS score history — the trust multiplier in the effective adjacency weight.

the [[approximation quality metric]] in [[focus flow computation]] uses $D_{KL}(\phi^*_c \| q^*_c)$ — the same divergence measure — to quantify how much the compiled [[transformer]] deviates from the exact [[focus]] distribution. the same mathematical object measures epistemic quality at three scales: individual [[neuron]] (BTS score), compiled model (approximation gap), and collective knowledge state (φ* convergence).

see [[veritas]] for the full continuous temporal extension of BTS into a living protocol. see [[cybergraph]] for the formal definition including the valence field. see [[wisdom of the crowds]] for the aggregation foundation. see [[cyber/epistemology]] for how honest linking becomes incentive-compatible under the full protocol.