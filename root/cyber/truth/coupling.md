---
tags: cybics, article, draft, research
alias: inversely coupled bonding surface, ICBS, Euclidean norm ICBS, bonding surface, coupling
crystal-type: pattern
crystal-domain: cybics
crystal-size: enzyme
---

a market mechanism for prediction markets where the two sides of a bet are geometrically coupled — buying one directly suppresses the other

proposed by Nick Williams and Vitalik Buterin, [Ethereum Research, 2020](https://ethresear.ch/t/better-curation-via-inversely-coupled-bonding-surfaces/7613)

---

## the core idea

standard prediction markets bound prices to [0,1] because shares settle to fixed payouts of $0 or $1. ICBS presents an alternative: settlement rebalances reserves rather than paying fixed amounts. prices are not bounded to [0,1]. instead, the ratio of reserves encodes the market's probability forecast.

in traditional bonding curves, buying token A doesn't affect token B's price. ICBS couples them: buying YES pushes NO's price down, and vice versa. this creates genuine opposition between beliefs rather than independent liquidity pools.

---

## the cost function

$$C(s_{YES}, s_{NO}) = \lambda \sqrt{s_{YES}^2 + s_{NO}^2}$$

where $s_{YES}$ and $s_{NO}$ are token supplies and $\lambda$ is a fixed scaling constant set at deployment.

geometrically: this is the Euclidean distance from the origin in the $(s_{YES}, s_{NO})$ plane. iso-cost curves are circles — every point at distance $r$ from the origin costs $\lambda \cdot r$. trading moves outward from the origin.

$\lambda$ is fixed at deployment by the initial deposit $D$:

$$\lambda = \frac{D}{\sqrt{s_{YES}^2 + s_{NO}^2}}$$

for a 50/50 split at initial price $1, a \$100 deposit creates $s_{YES} = s_{NO} = 50$ tokens, giving $\lambda = 100/\sqrt{50^2 + 50^2} \approx 1.414$. markets of different sizes have identical percentage-based price dynamics — enabling cross-market comparison.

---

## prices and inverse coupling

prices emerge as partial derivatives of the cost function:

$$p_{YES} = \frac{\partial C}{\partial s_{YES}} = \lambda \cdot \frac{s_{YES}}{\sqrt{s_{YES}^2 + s_{NO}^2}}$$

$$p_{NO} = \frac{\partial C}{\partial s_{NO}} = \lambda \cdot \frac{s_{NO}}{\sqrt{s_{YES}^2 + s_{NO}^2}}$$

each token's price increases with its own supply but is suppressed by the opposing side:

$$\frac{\partial p_{YES}}{\partial s_{NO}} = -\lambda \cdot \frac{s_{YES} \cdot s_{NO}}{(s_{YES}^2 + s_{NO}^2)^{3/2}} < 0$$

buying NO directly lowers YES's price. this is the inverse coupling that gives ICBS its name — and that makes it the correct market structure for an epistemic system where TRUE and FALSE are genuine opposites, not independent assets.

---

## the invariant: TVL = cost function

virtual reserves are defined as $r = s \cdot p$:

$$r_{YES} = \lambda \cdot \frac{s_{YES}^2}{\sqrt{s_{YES}^2 + s_{NO}^2}}, \quad r_{NO} = \lambda \cdot \frac{s_{NO}^2}{\sqrt{s_{YES}^2 + s_{NO}^2}}$$

total value locked:

$$TVL = r_{YES} + r_{NO} = \lambda\sqrt{s_{YES}^2 + s_{NO}^2} = C(s_{YES}, s_{NO})$$

TVL always equals the cost function — the on-manifold property. this ensures solvency: total claimable value always matches what the vault holds. reserves can rebalance at settlement without minting or burning tokens.

the market's current probability forecast:

$$q = \frac{r_{YES}}{r_{YES} + r_{NO}}$$

---

## settlement

at resolution, actual outcome $x \in \{0, 1\}$ determines settlement factors:

$$f_{YES} = \frac{x}{q}, \quad f_{NO} = \frac{1-x}{1-q}$$

if the event happens ($x = 1$): YES holders gain ($f_{YES} > 1$), NO holders lose ($f_{NO} < 1$). if it doesn't ($x = 0$): NO holders gain, YES holders lose. reserves rebalance directly:

$$r'_{YES} = r_{YES} \cdot f_{YES}, \quad r'_{NO} = r_{NO} \cdot f_{NO}$$

total vault balance is preserved: $r'_{YES} + r'_{NO} = r_{YES} + r_{NO}$. capital flows from incorrect predictions to correct ones without external capital injection.

settlement uses square-root scaling of the supply parameter $\sigma$ (converting display to virtual supply). scaling $\sigma$ by $\sqrt{f}$ makes virtual supplies scale by $\sqrt{f}$, making reserves (proportional to supply$^2$ via the norm) scale by exactly $f$.

---

## key properties

self-scaling liquidity. buying moves supply further from the origin. TVL $= \lambda\sqrt{s_{YES}^2 + s_{NO}^2}$, so trading volume automatically grows liquidity. no external LPs needed. markets bootstrap from minimal deposits and scale organically. this differs fundamentally from [[LMSR]], where the subsidy parameter $b$ caps liquidity.

early conviction rewards. prices range from 0 to $\lambda$:

$$\lim_{s_{YES} \to \infty,\, s_{NO} \text{ fixed}} p_{YES} = \lambda$$

early traders who buy near zero can see prices approach $\lambda$, yielding arbitrarily large returns. unlike LMSR's fixed [0,1] bounds, ICBS rewards early conviction rather than just tracking consensus. this aligns incentives toward surfacing private knowledge early.

geometric simplicity. only square roots — no fractional powers, no exponentials. the mechanism is computationally tractable and the geometry is intuitive.

---

## ICBS vs LMSR

| | ICBS | LMSR |
|---|---|---|
| price bounds | [0, λ] | [0, 1] |
| liquidity | self-scaling (trading grows TVL) | capped by subsidy parameter b |
| external LPs | none needed | none needed |
| settlement | reserve rebalancing | fixed $0/$1 payouts |
| early conviction | rewarded (prices can approach λ) | not specially rewarded |
| probability encoding | ratio of reserves | direct price |
| loss bound | none (market maker takes risk) | b·ln(2) per market |

---

## connection to [[cyber]]

the inverse coupling property is the market analog of [[inhibition]]: buying FALSE directly suppresses the effective weight of YES in the market, exactly as negative weights suppress activations in neural networks. the geometry makes this explicit — the two sides move on a circle, so amplifying one necessarily suppresses the other.

the self-scaling liquidity property solves the bootstrapping problem for the [[cybergraph]]: every [[cyberlink]] that attracts market activity automatically deepens its own liquidity. the most-contested edges (the epistemically important ones) become the most liquid, yielding the most accurate prices. this is the Lindy effect on the market structure.

the settlement factors $f_{YES} = x/q$ and $f_{NO} = (1-x)/(1-q)$ are inverse probability weights — the same mathematical structure that appears in importance sampling, in the [[serum]] scoring formula, and in the KL divergence terms that measure information gain. this is not coincidental: all three are instances of proper scoring rules applied to belief elicitation.

the on-manifold property (TVL = cost function) ensures the market remains solvent as [[cyberlinks]] accumulate, without requiring external capital injection. the [[cybergraph]] itself is the liquidity — structural [[knowledge]] ([[cyberlinks]]) bootstraps epistemic [[knowledge]] (market prices).

see [[veritas]] for how ICBS fits into the full truth-discovery protocol. see [[inhibition]] for the connection to inhibitory weights in the [[tri-kernel]]. see [[serum]] for the scoring layer that sits above the market mechanism. see [[market]] for the broader design.