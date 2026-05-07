---
tags: cybics, article, draft, research
alias: prediction markets, prediction market, information markets, decision markets
crystal-type: pattern
crystal-domain: cybics
crystal-size: bridge
---

markets where participants trade shares in future outcomes — and where prices become the aggregate probability estimate of those outcomes

the core mechanism: agents who believe an event will occur buy YES shares; agents who believe it will not buy NO. the market price of YES at any moment reflects the collective's implied probability for the event. if the event resolves, correct positions are paid; incorrect positions lose. calibrated forecasters profit. miscalibrated ones lose stake.

---

## why markets aggregate information

markets outperform polls and committees in forecasting for one reason: skin in the game. every position carries financial risk proportional to confidence. this creates a systematic filter:

- agents with genuine private knowledge profit from exploiting it
- agents without genuine knowledge lose when they speculate
- over time, capital flows toward the well-calibrated and away from the noise-makers

the price at any moment aggregates all private information held by all participants, weighted by their stake and track record. this is the core insight of the [[wisdom of the crowds]] under economic incentives — errors cancel not just statistically but economically.

---

## failure modes

prediction markets inherit the failure modes of [[wisdom of the crowds]] when beliefs are correlated:

thin market problem. most edges in a [[knowledge]] graph will have 0–5 participants. with few traders, the market may not aggregate much. this is why LMSR and [[inversely coupled bonding surface|ICBS]] are designed to function on thin markets — even one trader produces a meaningful price.

oracle problem. traditional prediction markets require an external oracle to resolve outcomes. "did the stock close above $100?" has a clear answer. "is this [[cyberlink]] true?" does not — there is no external ground truth. perpetual markets without resolution require different mechanisms: usage signals, liquidity dampers, periodic rebalancing.

herding and cascades. if positions are visible, traders may copy observed behavior rather than exploiting private information. ZKP on individual positions (showing only the aggregate price) eliminates herding by design — agents can only observe the price, not who holds what.

manipulation. coordinated actors can move prices by taking large positions. in [[inversely coupled bonding surface|ICBS]], this costs stake proportional to price movement and paradoxically sharpens the market's accuracy — attack = liquidity injection. in LMSR, the loss bound $b \cdot \ln(2)$ per market limits the market maker's maximum subsidy.

---

## from prediction markets to [[Bayesian Truth Serum]]

prediction markets solve the information aggregation problem for events with observable outcomes. [[Bayesian Truth Serum]] (Prelec, 2004) solves it for events with no observable outcome — beliefs themselves.

BTS rewards agents whose beliefs are more popular than they predicted they would be. this extracts private knowledge without requiring resolution. the mechanism: if you genuinely know something the crowd doesn't, you will underestimate how many others share that knowledge. BTS pays for exactly this gap.

the two mechanisms are complementary:

| | prediction markets | [[Bayesian Truth Serum]] |
|---|---|---|
| what is scored | position vs resolved outcome | belief vs crowd's predicted belief |
| oracle required | yes (external resolution) | no (crowd itself is the signal) |
| application | events with clear outcomes | beliefs, opinions, subjective judgments |
| mechanism | [[proper scoring rules]] via settlement | [[proper scoring rules]] via peer prediction |
| in [[cyber]] | [[inversely coupled bonding surface]] | valence $v$ in [[cyberlink]] |

---

## LMSR: the canonical automated market maker

Hanson's Logarithmic Market Scoring Rule (LMSR) is the standard market maker for thin prediction markets. cost function: $C(q) = b \cdot \ln(\sum_i e^{q_i/b})$ where $q_i$ is shares outstanding for outcome $i$ and $b$ is the liquidity parameter.

properties: no external LPs needed, price = probability directly ($p_i = e^{q_i/b}/\sum_j e^{q_j/b}$), loss bounded at $b \cdot \ln(n)$ for $n$ outcomes, functions on thin markets.

limitation: prices are bounded to [0,1]. early conviction is not specially rewarded — arriving first earns the same relative return as arriving late.

---

## [[inversely coupled bonding surface]]: the [[veritas]] market

[[inversely coupled bonding surface|ICBS]] (Williams & Buterin, 2020) is the market mechanism adopted in [[veritas]] and the [[cyberlink market protocol]]. cost function: $C(s_{YES}, s_{NO}) = \lambda\sqrt{s_{YES}^2 + s_{NO}^2}$.

the key departure from LMSR: prices range from 0 to $\lambda$ (not [0,1]), and trading volume grows TVL automatically. early correct positions earn arbitrarily large returns relative to late consensus-following. this aligns incentives toward surfacing private knowledge early — the mechanism directly rewards the contrarian who knew before the crowd did.

the settlement factors $f_{YES} = x/q$, $f_{NO} = (1-x)/(1-q)$ are inverse probability weights — the log-score [[proper scoring rules|proper scoring rule]] instantiated as a market mechanism.

---

## prediction markets in [[cyber]]

the [[cyberlink market protocol]] makes every [[cyberlink]] simultaneously a structural assertion and a prediction market on its own truth. one atomic act creates:

- the [[knowledge]] edge (binary structural layer)
- the market on that edge's validity (continuous epistemic layer)
- the BTS meta-prediction via valence $v$ (ternary prediction layer)

[[market inhibition]] describes how market prices enter the [[tri-kernel]] as effective edge weights: $w_\text{eff}(e) = \text{stake}(e) \times \text{trust}(\nu_e) \times f(\text{ICBS price}(e))$. edges the market disbelieves are suppressed toward zero. this transforms the [[cybergraph]] from an excitation-only associative network into a full discriminative system.

see [[inversely coupled bonding surface]] for the market mechanism. see [[Bayesian Truth Serum]] for the peer prediction layer. see [[proper scoring rules]] for the theoretical foundation. see [[wisdom of the crowds]] for the aggregation background. see [[market inhibition]] for how prices reshape [[focus]].