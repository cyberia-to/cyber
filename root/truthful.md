---
tags: cyber, core
alias: truthfulness, truthful neuron, honest neuron, incentive compatible
crystal-type: property
crystal-domain: cyber
---
a [[neuron]] is truthful when its [[cyberlinks]] report its actual private beliefs — not adjusted for social pressure, predicted popularity, or gaming the reward signal

a truthful link: the neuron creates the connection because it genuinely believes it reflects reality, stakes according to that conviction, and sets [[valence]] to match its honest prediction of where the [[inversely coupled bonding surface|ICBS]] market will settle

---

## truthfulness in mechanism design

in mechanism design, a protocol is truthful (dominant strategy incentive compatible, DSIC) when honest reporting is a dominant strategy — the best response regardless of what others do. this is stronger than a Nash equilibrium, where honesty is optimal only given that others are also honest.

[[Bayesian Truth Serum]] achieves Bayes-Nash equilibrium truthfulness: honest reporting is optimal when the neuron believes others will also report honestly. whether the full [[veritas]] protocol achieves the stronger DSIC property is an open question — see [[cyber/epistemology]] §6.1.

## truthfulness and [[syntropy]]

a truthful link increases [[syntropy]]: the cyberlink sharpens the collective picture, reducing uncertainty ($D_{KL}(\phi^*_{\text{after}} \| u) > D_{KL}(\phi^*_{\text{before}} \| u)$). a spammy or false link decreases syntropy — it moves $\phi^*$ toward noise.

[[karma]] is the accumulated truthfulness record: the running sum of [[Bayesian Truth Serum|BTS]] scores across all a neuron's links. high karma means a consistent track record of signal over noise. karma enters [[effective adjacency]] as the multiplier $\kappa(\nu)$, making past truthfulness a structural property of current influence.

## the incentive structure

[[Bayesian Truth Serum]] makes truthfulness rational:

- inflating [[valence]] toward predicted popularity loses information gain (surprise drops to zero once the neuron has predicted its own position)
- setting valence contrarian without genuine signal loses prediction accuracy
- the unique score-maximizing strategy is accurate reporting of both belief (link + stake) and meta-belief (valence)

over time: truthful neurons earn stake from noise producers, accumulating influence. noise producers lose stake to truthful neurons, losing influence. the graph self-selects toward truthful contributors in proportion to epistemic accuracy.

## truthfulness and [[trust]]

truthfulness is a property of track record, not individual acts. a single truthful link earns a positive BTS score. systematic truthfulness earns high karma. high karma is the formal analog of trust: the network has observed that this neuron's private signals are genuine.

trust is not agreement. a truthful neuron can consistently disagree with the majority — setting $v = -1$ on links others rate highly — and earn high karma if its contrarian predictions repeatedly prove accurate.

see [[truth]] for the probabilistic truth signal. see [[truth model]] for the two-layer structure. see [[valence]] for the ternary field that carries the honesty signal. see [[Bayesian Truth Serum]] for the scoring formula. see [[karma]] for the accumulated truthfulness record.

discover all [[concepts]]
