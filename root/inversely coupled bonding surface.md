---
tags: cyber, core, cybernomics
alias: ICBS, bonding surface, epistemic market
crystal-type: entity
crystal-domain: cyber
---
# inversely coupled bonding surface

the epistemic market mechanism of [[cyber]]. a two-sided bonding curve where YES and NO are geometrically coupled — buying one directly suppresses the other

## the cost function

$$C(s_{YES},\, s_{NO}) = \lambda \sqrt{s_{YES}^2 + s_{NO}^2}$$

where $s_{YES}$ and $s_{NO}$ are token supplies and $\lambda$ is a scaling constant fixed at deployment. iso-cost curves are circles in the $(s_{YES}, s_{NO})$ plane. trading moves outward from the origin along the surface

## prices and coupling

prices emerge as partial derivatives of the cost function:

$$p_{YES} = \lambda \cdot \frac{s_{YES}}{\sqrt{s_{YES}^2 + s_{NO}^2}}, \quad p_{NO} = \lambda \cdot \frac{s_{NO}}{\sqrt{s_{YES}^2 + s_{NO}^2}}$$

the inverse coupling:

$$\frac{\partial p_{YES}}{\partial s_{NO}} = -\lambda \cdot \frac{s_{YES} \cdot s_{NO}}{(s_{YES}^2 + s_{NO}^2)^{3/2}} < 0$$

buying NO lowers YES. buying YES lowers NO. the two sides are genuine opposites on a shared geometric surface

## role in cyber

every [[cyber/link]] in the [[cybergraph]] can carry an ICBS market. the market price enters the [[tri-kernel]] as the effective edge weight:

$$w_{\text{eff}}(e) = \text{price}(e) \times \text{stake}(e)$$

at $p \to 1$: full [[focus]] flows through the edge. at $p \to 0$: the edge is deactivated. this is [[market inhibition]] — the mechanism by which collective epistemic assessment reshapes structural connectivity

self-scaling liquidity means trading volume automatically grows the market. the most-contested edges become the most liquid, yielding the most accurate prices. no external liquidity providers required

## properties

| property | description |
|---|---|
| self-scaling | TVL $= C(s_{YES}, s_{NO})$ grows with trading volume |
| solvency | total value locked always equals the cost function |
| early conviction | prices range from 0 to $\lambda$, rewarding early discovery |
| geometric simplicity | only square roots — tractable on-chain computation |
| inverse coupling | buying one side directly suppresses the other |

see [[cyber/truth/coupling]] for the full specification. see [[market inhibition]] for how ICBS provides inhibitory signals to the [[tri-kernel]]. see [[valence]] for the ternary epistemic field on [[cyberlinks]]. see [[Bayesian Truth Serum]] for the scoring layer. see [[cyber/nomics]] for the broader economic design
