---
alias: learning rewards
tags: cyber, article, cip
crystal-type: process
crystal-domain: economics
crystal-size: article
status: draft
---
# learning incentives

[[knowledge]] creation is costly, but its benefits are collective. without incentives, rational agents free-ride on others' [[cyberlinks]]. the reward system must make contributing to the [[cybergraph]] profitable — and free-riding unprofitable

the [[economic model]] defines the seven mechanisms. this article explains the reward design: what to measure, how to attribute, and why [[cyberlinks]] are yield-bearing assets

## the signal: Δπ

every reward in the system traces back to one quantity: how much did your action shift the stationary distribution π?

$$\text{reward}(v) \propto \Delta\pi(v)$$

this is the gradient of system [[free energy]]. creating valuable structure is literally creating [[value]]. no designed loss function — physics defines what should be optimized

## reward functions

five candidates for measuring convergence contribution, each with trade-offs:

| function | formula | strength | weakness |
|---|---|---|---|
| Δπ norm | $\sum_j |\pi_j^{(t+1)} - \pi_j^t|$ | simple, easy to verify | gameable by oscillation |
| entropy reduction | $H(\pi^t) - H(\pi^{t+1})$ | rewards semantic sharpening | computationally heavier |
| spectral gap | $\lambda_2^t - \lambda_2^{t+1}$ | measures global convergence speedup | expensive, non-local |
| predictive alignment | $\text{align}(\pi^{(t+1)}, \pi^T)$ | favors early correct contributions | requires delayed validation |
| DAG weight | descendant blocks referencing this one | rewards foundational work | slow to accrue |

the hybrid model combines them:

$$R = \alpha \cdot \Delta\pi + \beta \cdot \Delta H + \gamma \cdot \text{DAGWeight} + \epsilon \cdot \text{AlignmentBonus}$$

fast local rewards use Δπ and ΔH. checkpoints add alignment and spectral verification bonuses. validators sample and verify blocks probabilistically

## link valuation

[[cyberlinks]] are yield-bearing epistemic assets. they accrue rewards over time based on contribution to [[focus]] emergence:

$$R_{i \to j}(T) = \int_0^T w(t) \cdot \Delta\pi_j(t) \, dt$$

where $\Delta\pi_j(t)$ = change in [[focus]] on target [[particle]] $j$ attributable to the link, $w(t)$ = time-weighting function, $T$ = evaluation horizon

| link type | characteristics | reward trajectory |
|---|---|---|
| viral | high Δπ short-term | early peak, fast decay |
| foundational | low Δπ early, grows later | slow rise, long reward |
| redundant | low/no Δπ | no reward |
| semantic bridge | medium, cross-module | moderate, persistent |

rewards paid from accumulated transaction fees — no inflation. this makes the [[cybergraph]] a semantic investment market where links earn yield proportional to epistemic impact

## attribution

exact [[Shapley values]] are infeasible ($O(n!)$). [[probabilistic shapley attribution]] approximates them:

1. local marginal — compute each transaction's individual $\Delta\mathcal{F}$
2. Monte Carlo sampling — sample $k$ random orderings, measure marginal contributions
3. hierarchical batching — cluster transactions by affected nodes, distribute within clusters
4. final reward: $R_i = \alpha \cdot \Delta\mathcal{F}_i + (1-\alpha) \cdot \hat{S}_i$

complexity: $O(k \cdot n)$ with $k \ll n$. feasible for 10⁶+ transactions per epoch

## funding

rewards come from transaction fees, not inflation. this is non-dilutive economics:

  - [[cyberlink]] submission incurs a small fee (spam deterrent)
  - fees pool and distribute to: link submitters, [[focus]] computation provers, validators
  - links that accumulate sufficient [[attention]] yield net positive reward over time
  - burn [[$CYB]] for permanent weight in π — anchoring critical [[knowledge]]

## the game

the game design ensures the [[cybergraph]] improves over time:

  - early, accurate links to important [[particles]] earn the most (attention yield curve)
  - redundant links earn nothing — the system penalizes noise
  - [[neurons]] build long-term reputation via accumulated π-weight ([[karma]])
  - [[focus]] as cost ensures every [[cyberlink]] is a [[costly signal]]

see [[economic model]] for the formal 7-point spec. see [[collective learning]] for the group-level dynamics. see [[cybernomics]] for the full economics collection
