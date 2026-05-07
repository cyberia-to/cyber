---
tags: cybics, article, draft, research
alias: wisdom of the crowds, crowd wisdom, collective judgment
crystal-type: pattern
crystal-domain: cybics
crystal-size: enzyme
---

the aggregated judgment of many independent agents outperforms most individuals — and often the best expert

first articulated by [[Aristotle]]: the many, though individually inferior, can collectively surpass the few best

formalized by [[Condorcet]] in the jury theorem (1785): if each juror is independently more likely than not to be correct, the probability that the majority is correct approaches 1 as group size grows

modern revival: Surowiecki (2004) — conditions for wise crowds: diversity of opinion, independence, decentralization, aggregation mechanism

---

## when it works

crowd wisdom holds when individual errors are independent and approximately symmetric around the truth. if 1000 people estimate the weight of an ox (Galton, 1907), their personal biases and random errors cancel. the average converges to the true weight even though no individual is accurate.

the conditions:

- errors must be independent — no one's guess is influenced by others'
- errors must be approximately zero-mean — biases cancel across the crowd
- the aggregation mechanism must reach all agents equally

---

## when it fails

the [[Condorcet]] jury theorem requires independence. when that assumption breaks down, correlated errors compound rather than cancel.

three failure modes that systematically corrupt crowd signals:

conformity bias. agents adjust toward what they expect others to say, not toward what they privately believe. the aggregate reflects social equilibrium, not private information.

social desirability bias. agents report toward what seems acceptable — systematically distorted toward approval rather than truth.

herding. agents observe each other's answers and update toward visible consensus, amplifying any early signal regardless of its truth. information cascades (Bikhchandani, Hirshleifer, Welch, 1992): even rational agents rationally ignore private signals when public signals seem overwhelming.

in all three cases, the aggregate does not reflect what agents privately know. it reflects the common prior they share — the noise, not the signal.

---

## the correction: [[Bayesian Truth Serum]]

[[Bayesian Truth Serum]] (Prelec, 2004) extracts the private signal even when beliefs are correlated. the mechanism: ask agents two things simultaneously — their belief, and their prediction of the aggregate belief.

BTS does not require independent errors. it only requires that agents with genuine private knowledge tend to underestimate how common their insight is. if you know something unusual but true, you think fewer others know it than actually do. BTS rewards this gap: beliefs that exceed their own predicted popularity.

crowd wisdom + BTS: raw aggregation extracts the first-order signal (what most people believe). BTS extracts the second-order signal (who knows something the crowd hasn't priced yet). both are needed.

---

## in [[cyber]]

the [[tri-kernel]] is the aggregation mechanism. [[neurons]] provide diverse independent signals via [[cyberlinks]]. [[focus]] is the crowd's verdict.

raw [[focus]] is the first-order aggregate — crowd wisdom without correction for correlated errors. the [[cyberlink market protocol]] adds the correction: market prices weight each [[neuron]]'s contribution by collective epistemic assessment. [[Bayesian Truth Serum]] scoring via the valence $v$ field adds the second-order signal: whose links exceed their predicted reception?

[[karma]] accumulates the BTS history — who has consistently contributed signal vs noise. the effective adjacency $A^{\text{eff}}_{pq}$ weights contributions by karma, not just raw stake.

see [[Bayesian Truth Serum]] for the scoring mechanism. see [[prediction markets]] for the market layer. see [[cyberlink market protocol]] for the full protocol design. see [[egregore]] for the emergent collective intelligence.