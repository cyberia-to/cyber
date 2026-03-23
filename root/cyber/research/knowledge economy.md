---
tags: cyber, article, draft, research
alias: knowledge economy, epistemic economy, cyber knowledge economy, knowledge markets
crystal-type: pattern
crystal-domain: cyber
crystal-size: bridge
diffusion: 0.0009385683757468901
springs: 0.0023643389005248033
heat: 0.0018928491061059652
focus: 0.0015571556792521248
gravity: 2
density: 2.94
---

the mechanisms that make contributing to the [[cybergraph]] more profitable than free-riding — and that make epistemic accuracy the unit of wealth

---

## epistemic assets

the [[cybergraph]] creates a new category of financial asset. an epistemic asset is a claim on the [[knowledge]] economy's flow. unlike financial assets (claims on future cash flows) or utility tokens (access rights to service capacity), epistemic assets yield returns proportional to the [[information]] contributed to collective [[intelligence]].

four asset classes:

[[cyberlinks]] are yield-bearing [[knowledge]] claims. every [[cyberlink]] accrues rewards over time as a function of the [[focus]] shift it generates:

$$R_{i \to j}(T) = \int_0^T w(t) \cdot \Delta\pi_j(t) \, dt$$

where $\Delta\pi_j(t)$ is the change in [[focus]] on target [[particle]] $j$ attributable to the link and $w(t)$ is the time-weighting function. four reward trajectories: viral (high $\Delta\pi$ early, fast decay), foundational (low early, grows as graph builds around it), confirming (shared reward via [[Shapley]] attribution), semantic bridge (moderate, persistent, cross-module).

[[eternal particles]] are positions burned into permanence. burning [[$CYB]] permanently anchors a [[particle]]'s $\pi$-weight — the particle cannot be archived or deprioritized below the burn-weighted floor. the graph's long-term assertions: the claims whose importance the market cannot undo.

[[eternal cyberlinks]] are edges burned into permanence. the link cannot be forgotten by stake dynamics or [[inversely coupled bonding surface|ICBS]] market collapse. the graph's highest-conviction structural commitment.

[[inversely coupled bonding surface|ICBS]] market positions are YES/NO bets on the epistemic market attached to every [[cyberlink]]. position value grows as the market converges. early conviction rewards are unbounded — prices range from $0$ to $\lambda$. capital flows from incorrect beliefs to correct ones.

[[karma]] is the accumulated [[Bayesian Truth Serum|BTS]] score history of a [[neuron]]. not tradeable but structurally determinant: karma weights every future link the neuron creates in the [[tri-kernel]] effective adjacency. epistemic capital — the form of wealth that can only be earned by being right before the crowd.

---

## the focus reward

every reward traces back to one quantity: how much did your action shift the [[tri-kernel]] fixed point $\pi^*$?

$$\text{reward}(v) \propto \Delta\pi(v)$$

$\Delta\pi$ is the gradient of the system's [[free energy]]. creating valuable structure literally creates [[value]]. no designed loss function — the physics of [[convergence]] defines what deserves to be optimized.

the hybrid reward function:

$$R = \alpha \cdot \Delta\pi + \beta \cdot \Delta J + \gamma \cdot \text{DAGWeight} + \epsilon \cdot \text{AlignmentBonus}$$

new [[$CYB]] is minted only when $\Delta\pi > 0$. the protocol's inflation is literally evidence of [[knowledge]] creation — there is no emission without demonstrated contribution to collective [[focus]].

---

## attribution

multiple [[neurons]] contribute [[cyberlinks]] in the same epoch. the total $\Delta\pi$ shift is a joint outcome. the [[Shapley value]] distributes fair credit: each agent's reward equals their average marginal contribution across all possible orderings. exact computation is $O(n!)$. the approximation:

$$R_i = \alpha \cdot \Delta\mathcal{F}_i + (1-\alpha) \cdot \hat{S}_i$$

complexity: $O(k \cdot n)$ with $k \ll n$, feasible for $10^6+$ transactions per epoch.

---

## epistemic markets

every [[cyberlink]] carries a perpetual prediction market on its own [[truth]]. one atomic act — creating a link — simultaneously asserts structural [[knowledge]] and opens an epistemic market on it.

the market mechanism is [[inversely coupled bonding surface|ICBS]]:

$$C(s_{YES}, s_{NO}) = \lambda \sqrt{s_{YES}^2 + s_{NO}^2}$$

buying YES directly suppresses NO's price — TRUE and FALSE are geometrically coupled on a circle, the market analog of inhibitory weights in the [[tri-kernel]]. the effective adjacency weight:

$$A^{\text{eff}}_{pq} = \sum_\ell \text{stake}(\ell) \times \text{karma}(\nu(\ell)) \times f(\text{ICBS price}(\ell))$$

the 2|3 architecture: each [[cyberlink]] carries topology (binary: edge exists), market (continuous: ICBS price), and meta-prediction (ternary: valence $v \in \{-1, 0, +1\}$). this produces a two-dimensional epistemic signal: price encodes magnitude, meta-score encodes collective confidence.

---

## honest signaling

the [[cybergraph]] achieves honest markets through [[Bayesian Truth Serum]] (Prelec, 2004). the valence field in every [[cyberlink]] is the BTS meta-prediction — no separate submission needed. honesty is a Bayes-Nash equilibrium: no [[neuron]] can improve their expected score by misreporting belief or meta-belief. [[karma]] compounds the trust multiplier: consistently right before the crowd → high karma → more adjacency weight per link → more reward per contribution → more resources to stake on the next correct insight.

---

## the GFP flywheel

the optimal mining hardware and the optimal proving hardware are the same chip. the [[Goldilocks field processor]] exercises four primitives (fma, ntt, p2r, lut) for both PoUW mining and real workloads ([[stark]] proving, focus computation, neural inference). mining rewards bootstrap chip development. chips accelerate proving. proving serves users. users pay fees. fees replace emission. no stranded assets.

---

## the evolutionary loop

contribute accurately → $\Delta\pi$ reward → accumulate [[$CYB]] → stake on more links → accumulate [[karma]] → links carry more adjacency weight → earlier $\Delta\pi$ attribution → more [[$CYB]] per contribution

the burn layer: burn on high-conviction [[particles]] → [[eternal particles|eternal weight]] → long-term yield floor → reduces risk premium for foundational contributions

the result: the unit of wealth is provably epistemic accuracy. the only sustainable path to large [[$CYB]] balances, high [[karma]], and consistent ICBS returns is being right about what matters before the crowd recognizes it.

see [[cyber/tokenomics]] for the monetary plumbing (emission, policy, hardware). see [[learning incentives]] for the detailed reward function specification. see [[inversely coupled bonding surface]] for the ICBS market mechanism. see [[Bayesian Truth Serum]] for the scoring layer. see [[karma]] for the trust multiplier dynamics. see [[functions of superintelligence]] for how the autonomous neuron participates in the same economy.