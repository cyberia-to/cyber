---
tags: cybics, article, draft, research
alias: veritas, Veritas, decentralized truth discovery, living truth, truth emergence
crystal-type: entity
crystal-domain: cybics
crystal-size: bridge
---

a protocol for continuous collective truth discovery, scaling [[Bayesian Truth Serum]] into a persistent epistemic system

source: [veritas.computer](https://veritas.computer)

---

## what veritas is

a primitive that surfaces collective [[intelligence]] as social consensus. not by polling or by expert authority — by principled [[social epistemology]] using the structure of belief itself.

veritas excels where no institution can arbitrate truth: legal interpretations, artistic judgments, moral arguments, cultural relevance, and intersubjective domains where no definitive answer exists. unlike prediction markets, it does not require resolution — it models how collective understanding evolves continuously.

the tagline is precise: "truth is emerging." not announced. not polled. not voted. emerging — as a convergent process.

---

## the problem with polling

democracy's "one person, one vote" treats all opinions as equal. but [[knowledge]] is not democratic: sometimes the majority is wrong, crowds follow trends, information is unevenly distributed. a popular vote is unfiltered crowd wisdom — correlated errors compound rather than cancel.

the question is not who has the most votes but who has genuine private [[knowledge]] that the aggregate is missing. [[Bayesian Truth Serum]] (Prelec, 2004) proved that the answer can be extracted mathematically: reward insight, not consensus.

---

## what veritas builds

veritas extends [[Bayesian Truth Serum]] across three dimensions:

continuous extension. participants submit full probability distributions over any number of options, not point estimates. this preserves honest uncertainty and captures how entire belief structures shift in coordinated patterns. it distinguishes reducible epistemic uncertainty (shrinks as evidence accumulates) from irreducible aleatory uncertainty (fundamental randomness in the world).

temporal extension. beliefs persist, evolve asynchronously, and are continuously updated without resolution. the system maintains a memory of its existing state and rewards those who push collective understanding forward. this is living truth — not a snapshot, not a market settlement, but a continuously converging distribution over what the collective knows.

economic extension. agents stake capital alongside their beliefs. stake is not just skin in the game — it scales the weight of an agent's contribution and is redistributed from noise producers to signal producers in proportion to their scores.

---

## the scoring formula

for agent $i$:

$$s_i = \underbrace{D_{KL}(p_i \,\|\, \bar{m}_{-i}) - D_{KL}(p_i \,\|\, \bar{p}_{-i})}_{\text{information gain}} - \underbrace{D_{KL}(\bar{p}_{-i} \,\|\, m_i)}_{\text{prediction accuracy}}$$

where $p_i$ is the agent's belief, $m_i$ is their prediction of others' aggregate beliefs, $\bar{p}_{-i}$ is the geometric mean of others' beliefs, and $\bar{m}_{-i}$ is the geometric mean of others' predictions.

negative scores indicate noise. stake flows from noise producers to signal producers in proportion to scores — a zero-sum redistribution whose magnitude scales with actual epistemic progress (reduction in collective uncertainty).

veritas does not tokenize shares in outcomes. it measures how many bits of [[information]] or noise each agent added to the collective picture and redistributes accordingly.

---

## truth emergence

learning occurs when collective uncertainty decreases — when the [[KL divergence]] between the prior distribution and the updated one shrinks. this is the signal that the system has incorporated new [[information]].

the mechanism is resistant to adversarial attack: attacking the system (submitting noise) is punished by negative scores. gaining disproportionate influence requires continuously contributing genuine signal. influence must be earned and renewed, not purchased once. the system naturally evolves into a meritocracy of insight rather than a plutocracy of stake.

---

## connections to [[cyber]]

veritas and [[cyber]] are solving adjacent parts of the same problem. their mathematical foundations converge.

[[two kinds of knowledge]]: veritas is an implementation of the epistemic layer — the layer that evaluates structural [[knowledge]] ([[cyberlinks]]) rather than creating it. veritas asks "what does the collective believe about this connection?" — exactly the question that [[two kinds of knowledge]] identifies as missing from raw [[cyberlink]] data.

[[syntropy]]: the veritas score for an agent is syntropy at the individual level — the amount by which one agent's contribution reduced collective uncertainty. aggregate veritas scores across all agents = the system's total syntropy gain in that epoch. [[karma]] in [[cyber]] is the accumulated syntropy contribution per [[neuron]] over time.

[[KL divergence]]: the [[approximation quality metric]] in [[focus flow computation]] is $\varepsilon(G,c) = D_{KL}(\phi^*_c \| q^*_c)$ — the same divergence measure that veritas uses for scoring. the cybergraph optimizes the same quantity at the structural level (reducing the gap between the compiled [[transformer]] and the exact [[focus]] distribution) that veritas optimizes at the epistemic level (reducing the gap between individual beliefs and collective truth).

temporal extension: veritas's living truth — beliefs that evolve without resolution — is structurally identical to the [[focus]] distribution φ* in [[cyber]]. φ* never "resolves." it continuously converges from the current graph state. every new [[cyberlink]] shifts φ* incrementally. truth in [[cyber]] IS the same kind of object: not a final answer but a continuously updated convergent signal.

trust weight: veritas weights agents by both stake and trust (track record of information contribution). [[cyber]]'s current model weights only by stake. the veritas trust metric — accumulated BTS score history — is the missing component that would make [[karma]] a full epistemic weight, not just an economic one.

---

## the market mechanism: ICBS

veritas uses the [[inversely coupled bonding surface]] (ICBS) as its market substrate — not LMSR. the distinction matters.

ICBS cost function: $C(s_{YES}, s_{NO}) = \lambda\sqrt{s_{YES}^2 + s_{NO}^2}$. iso-cost curves are circles in the $(s_{YES}, s_{NO})$ plane. buying YES directly suppresses NO's price:

$$\frac{\partial p_{YES}}{\partial s_{NO}} = -\lambda \cdot \frac{s_{YES} \cdot s_{NO}}{(s_{YES}^2 + s_{NO}^2)^{3/2}} < 0$$

this inverse coupling is the geometric encoding of opposition between beliefs. TRUE and FALSE are not independent assets — they compete on a circle.

key properties of ICBS over LMSR:

- self-scaling liquidity: trading volume grows TVL automatically. no external LPs, no fixed subsidy parameter. the [[cybergraph]]'s most-contested edges become the most liquid
- early conviction rewarded: prices range from 0 to λ (not bounded to [0,1]). early correct linking earns arbitrarily large returns relative to late consensus-following
- probability encoding via reserve ratio: $q = r_{YES}/(r_{YES} + r_{NO})$ — not the direct price
- on-manifold invariant: TVL always equals the cost function, ensuring solvency without external capital

the settlement factors $f_{YES} = x/q$ and $f_{NO} = (1-x)/(1-q)$ are inverse probability weights — the same mathematical structure that appears in importance sampling and in the [[Bayesian Truth Serum]] scoring formula. both are instances of proper scoring rules applied to belief elicitation.

---

## the full stack

veritas is a three-layer system:

| layer | mechanism | what it does |
|---|---|---|
| market | [[inversely coupled bonding surface]] | prices beliefs, couples TRUE/FALSE, self-scales liquidity |
| scoring | [[Bayesian Truth Serum]] | measures information contribution, rewards private knowledge surfaced |
| trust | accumulated BTS score history | weights agents by epistemic track record, not just stake |

ICBS handles the economic layer. BTS handles the epistemic layer. trust accumulation handles the reputation layer. each layer is necessary; none subsumes the others.

---

## the key claim

without an epistemic layer, the [[cybergraph]] is excitation-only: it accumulates structural connections but cannot deactivate misleading ones. with veritas-style scoring, the [[cybergraph]] gains the inhibitory signal described in [[market inhibition]] — grounded in information theory and geometrically enforced by ICBS inverse coupling.

a [[cyberlink]]'s effective weight in the [[tri-kernel]]:

$$w_\text{eff}(e) = \text{stake}(e) \times \text{trust}(\nu_e) \times f(\text{ICBS price}(e))$$

where ICBS price encodes collective belief about the link, and trust encodes the [[neuron]]'s accumulated BTS score history. links from high-trust [[neurons]] on high-confidence edges carry maximum weight. links from noise producers on contested edges are suppressed.

truth is emerging — from the interaction of structural [[knowledge]] ([[cyberlinks]]) and epistemic [[knowledge]] (ICBS prices + BTS scores). neither alone is sufficient.

see [[Bayesian Truth Serum]] for the scoring foundation. see [[inversely coupled bonding surface]] for the market mechanism. see [[two kinds of knowledge]] for the structural/epistemic split. see [[market inhibition]] for why the epistemic layer is necessary. see [[wisdom of the crowds]] for the aggregation background. see [[syntropy]] for the information-theoretic signal.