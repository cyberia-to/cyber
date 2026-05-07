---
tags: cybics, mathematics, article, draft, research
alias: LMSR, Logarithmic Market Scoring Rule, Hanson market scoring rule, log market scoring rule
crystal-type: pattern
crystal-domain: cybics
crystal-size: enzyme
---

a market mechanism for [[prediction markets]] invented by Robin Hanson (2003) — the canonical automated market maker for thin markets

cost function: $C(\mathbf{q}) = b \cdot \ln\!\left(\sum_i e^{q_i/b}\right)$

where $q_i$ is shares outstanding for outcome $i$ and $b$ is the liquidity parameter

---

## prices and probabilities

prices emerge as derivatives of the cost function:

$$p_i = \frac{\partial C}{\partial q_i} = \frac{e^{q_i/b}}{\sum_j e^{q_j/b}}$$

this is the softmax function. LMSR price = softmax of log-odds. for a binary market (TRUE/FALSE):

$$p_{YES} = \frac{e^{q_{YES}/b}}{e^{q_{YES}/b} + e^{q_{NO}/b}} = \sigma\!\left(\frac{q_{YES} - q_{NO}}{b}\right)$$

where $\sigma$ is the logistic sigmoid. price is directly interpretable as probability: $p_{YES} = 0.73$ means the market estimates 73% probability for YES.

---

## the softmax connection

the LMSR price formula is the softmax. the softmax appears in:

- LMSR prediction markets (price as implied probability)
- transformer attention weights (query-key alignment → attention distribution)
- multinomial logistic regression (class probabilities)
- Boltzmann distributions in statistical mechanics (energy → probability)

all four are the same mathematical object: exponentiated linear scores normalized to sum to 1. this is why [[prediction markets]] and transformer [[attention]] are structurally isomorphic — both aggregate information by computing softmax over a set of evidence vectors. the [[cybergraph]]'s [[tri-kernel]] is belief propagation over the same softmax-weighted graph.

---

## the market maker guarantee

the market maker (protocol or designated agent) is always willing to trade at the current price. when a trader buys $\Delta q_i$ shares of outcome $i$, the cost is:

$$\text{cost} = C(\mathbf{q} + \Delta\mathbf{e}_i) - C(\mathbf{q}) = b \cdot \ln\!\left(\frac{\sum_j e^{q_j/b} + \Delta_i}{...}\right)$$

more precisely, the incremental cost is the change in the log-sum-exp. the market maker absorbs all trades, so the market is always liquid regardless of participant count.

---

## bounded loss

the market maker's maximum loss is bounded:

$$\text{max loss} = b \cdot \ln(n)$$

for a binary market: $b \cdot \ln(2) \approx 0.693b$. this is a known cost before deployment — the market maker commits at most $b \cdot \ln(n)$ in subsidy for the information the market aggregates. this is why LMSR is used in knowledge graphs where individual edges may attract very few traders: even with one trader, the loss is bounded.

---

## key properties

thin market designed. LMSR functions correctly even with a single trader. parameter $b$ controls the sensitivity: low $b$ → prices move quickly (high information value per trade, but noisier), high $b$ → prices move slowly (smoother signal, higher subsidy).

price = probability directly. no reserve ratio needed. $p_{YES}$ is the market's estimate of $P(\text{YES})$ — a direct probability.

no external LPs needed. the market maker is the protocol. no liquidity provider needs to be compensated or recruited.

loss bounded. maximum market maker exposure is $b \cdot \ln(n)$, known in advance. this enables pre-funded market contracts.

---

## limitations vs [[inversely coupled bonding surface|ICBS]]

| | LMSR | [[inversely coupled bonding surface|ICBS]] |
|---|---|---|
| price bounds | [0, 1] | [0, λ] |
| liquidity | capped at $b \cdot \ln(n)$ | self-scaling (trading grows TVL) |
| early conviction | not specially rewarded | rewarded (prices can approach λ) |
| probability encoding | direct price | reserve ratio $q = r_{YES}/(r_{YES} + r_{NO})$ |
| inverse coupling | independent YES/NO | buying YES suppresses NO |
| loss bound | $b \cdot \ln(n)$ — known in advance | unbounded for market maker |

LMSR is better when: a hard loss cap is required, or when probability needs to be read directly without computing reserve ratios.

ICBS is better when: early signal discovery matters, self-scaling liquidity is needed, or the epistemic opposition between TRUE and FALSE should be geometrically enforced (the circle invariant).

[[veritas]] and the [[cyberlink market protocol]] adopted ICBS over LMSR for the self-scaling and early-conviction properties — the most important edges in a [[knowledge]] graph are the most contested ones, and they need to attract the most liquidity automatically.

---

## the scoring rule foundation

LMSR is a [[proper scoring rules|proper scoring rule]] expressed as a market. a forecaster who "trades" with the LMSR market maker is equivalent to reporting a probability to a log scoring rule. the market maker implements the scoring rule implicitly: profitable trades correspond to reports closer to the true probability, losing trades to reports further away.

this is why market prices aggregate information efficiently: every trader is implicitly submitting a probability report to a proper scoring rule, and the aggregate price is the market's best estimate given all reports received.

see [[prediction markets]] for the broader context. see [[inversely coupled bonding surface]] for the adopted alternative. see [[proper scoring rules]] for the theoretical foundation. see [[Bayes theorem]] for why market prices are Bayesian posteriors.