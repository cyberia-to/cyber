---
alias: learning incentives, learning rewards
tags: cyber, article, cip
crystal-type: process
crystal-domain: economics
crystal-size: article
status: draft
stake: 66218419658672376
diffusion: 0.0013820152299540785
springs: 0.0011163276004283616
heat: 0.001223107287953102
focus: 0.0012705273526962056
gravity: 26
density: 3.89
---
# learning incentives

one mechanism within [[cyber/tokenomics]]: how [[$CYB]] is minted, burned, and locked to reward [[knowledge]] creation in the [[cybergraph]]

[[knowledge]] creation is costly, but its benefits are [[collective]]. without incentives, rational agents free-ride on others' [[cyberlinks]]. this mechanism makes contributing profitable — and free-riding unprofitable

## the signal: Δπ

every reward traces back to one quantity: how much did your action shift the [[tri-kernel]] fixed point π?

$$\text{reward}(v) \propto \Delta\pi(v)$$

π is the stationary distribution of the composite operator $\mathcal{R} = \lambda_d D + \lambda_s S + \lambda_h H_\tau$ — [[diffusion]] explores, [[springs]] enforce structure, [[heat kernel]] adapts. the [[collective focus theorem]] proves π exists, is unique, and is computable locally

Δπ is the gradient of system [[free energy]]. creating valuable structure is literally creating [[value]]. no designed loss function — physics defines what should be optimized

## reward functions

five candidates for measuring convergence contribution, each with trade-offs:

| function | formula | strength | weakness |
|---|---|---|---|
| Δπ norm | $\sum_j \|\pi_j^{(t+1)} - \pi_j^t\|$ | simple, easy to verify | gameable by oscillation |
| [[syntropy]] growth | $H(\pi^t) - H(\pi^{t+1})$ | rewards semantic sharpening | computationally heavier |
| spectral gap | $\lambda_2^t - \lambda_2^{t+1}$ | measures global convergence speedup | expensive, non-local |
| predictive alignment | $\text{align}(\pi^{(t+1)}, \pi^T)$ | favors early correct contributions | requires delayed validation |
| DAG weight | descendant blocks referencing this one | rewards foundational work | slow to accrue |

the hybrid model combines them:

$$R = \alpha \cdot \Delta\pi + \beta \cdot \Delta J + \gamma \cdot \text{DAGWeight} + \epsilon \cdot \text{AlignmentBonus}$$

where $\Delta J = H(\pi^t) - H(\pi^{t+1})$ is [[syntropy]] growth. fast local rewards use Δπ and ΔJ. checkpoints add alignment and spectral verification bonuses. validators sample and verify blocks probabilistically

## link valuation

[[cyberlinks]] are yield-bearing epistemic assets. they accrue rewards over time based on contribution to [[focus]] emergence:

$$R_{i \to j}(T) = \int_0^T w(t) \cdot \Delta\pi_j(t) \, dt$$

where $\Delta\pi_j(t)$ = change in [[focus]] on target [[particle]] $j$ attributable to the link, $w(t)$ = time-weighting function, $T$ = evaluation horizon

| link type | characteristics | reward trajectory |
|---|---|---|
| viral | high Δπ short-term | early peak, fast decay |
| foundational | low Δπ early, grows later | slow rise, long reward |
| confirming | low individual Δπ, strengthens axon weight | shared reward via attribution |
| semantic bridge | medium, cross-module | moderate, persistent |

## attribution

multiple [[neurons]] contribute [[cyberlinks]] in the same epoch. the total Δπ shift is a joint outcome — how to divide credit fairly?

the [[Shapley value]] answers: each agent's reward equals their average marginal contribution across all possible orderings. in this system, the coalition's total value is the [[free energy]] reduction $\Delta\mathcal{F}$, and each agent's marginal contribution is how much π shifts when their [[cyberlinks]] are added to the graph. Shapley distributes the total Δπ reward proportionally to each [[neuron]]'s causal impact

exact computation is infeasible ($O(n!)$). [[probabilistic shapley attribution]] approximates:

1. local marginal — compute each transaction's individual $\Delta\mathcal{F}$ (add link, measure π shift)
2. Monte Carlo sampling — sample $k$ random orderings of the epoch's transactions, measure marginal contributions in each ordering
3. hierarchical batching — cluster transactions by affected neighborhood, distribute within clusters
4. final reward: $R_i = \alpha \cdot \Delta\mathcal{F}_i + (1-\alpha) \cdot \hat{S}_i$

where $\Delta\mathcal{F}_i$ is the fast local estimate and $\hat{S}_i$ is the sampled Shapley approximation. $\alpha$ balances speed (local marginal) against fairness (Shapley)

complexity: $O(k \cdot n)$ with $k \ll n$. feasible for 10⁶+ transactions per epoch

## self-minting

rewards are not computed centrally. each [[neuron]] proves their own contribution and claims their own reward.

every [[cyber/signal]] carries a $\pi_\Delta$ — the neuron's locally computed focus shift for a batch of [[cyberlinks]]. this $\pi_\Delta$ is proven correct by a single [[stark]] proof referencing a specific $\text{bbg\_root}$. the proof is the reward claim:

1. [[neuron]] creates [[cyber/signal]] with one or more [[cyberlinks]], $\pi_\Delta$, and [[stark]] proof
2. proof demonstrates: applying these links to the graph at $\text{bbg\_root}_t$ shifts π by $\pi_\Delta$
3. any verifier checks the proof against the header — O(log n), no recomputation
4. if valid and Δπ > 0, the neuron mints [[$CYB]] proportional to the proven shift

no aggregator decides the reward. the proof IS the mining. a [[neuron]] on a phone: buy a header, query neighborhood state, create [[cyberlinks]], prove Δπ, bundle into a [[cyber/signal]], mint tokens

conservation: total minting per epoch is bounded by the actual global Δπ, verifiable from consecutive headers. if the sum of individual claims exceeds the actual shift (overlapping neighborhoods), all claims are scaled proportionally

see §6.9 and §14.2 of the whitepaper for the full specification

## the three token operations

- mint: [[neurons]] prove Δπ via [[stark]] and self-mint [[$CYB]] proportional to their contribution
- burn: [[neurons]] destroy [[$CYB]] for permanent π-weight on [[particles]] ([[eternal particles]]) or [[cyberlinks]] ([[eternal cyberlinks]])
- lock: [[neurons]] stake [[$CYB]] on [[particles]] or [[cyberlinks]], earning from fee pools proportional to [[attention]] attracted

## the game

the game design ensures the [[cybergraph]] improves over time:

- early, accurate links to important [[particles]] earn the most ([[attention]] yield curve)
- confirming links strengthen [[axon]] weight — repeated signals build consensus, not noise
- [[neurons]] build long-term reputation via accumulated π-weight ([[karma]])
- [[focus]] as cost ensures every [[cyberlink]] is a [[costly signal]]

see [[cyber/tokenomics]] for the system-level economics (monetary policy, allocation curve, GFP flywheel). see [[collective learning]] for the group-level dynamics