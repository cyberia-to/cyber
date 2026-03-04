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

[[KL divergence]]: the [[approximation quality metric]] in [[focus flow computation]] is $\varepsilon(G,c) = D_{KL}(\pi^*_c \| q^*_c)$ — the same divergence measure that veritas uses for scoring. the cybergraph optimizes the same quantity at the structural level (reducing the gap between the compiled [[transformer]] and the exact [[focus]] distribution) that veritas optimizes at the epistemic level (reducing the gap between individual beliefs and collective truth).

temporal extension: veritas's living truth — beliefs that evolve without resolution — is structurally identical to the [[focus]] distribution π* in [[cyber]]. π* never "resolves." it continuously converges from the current graph state. every new [[cyberlink]] shifts π* incrementally. truth in [[cyber]] IS the same kind of object: not a final answer but a continuously updated convergent signal.

trust weight: veritas weights agents by both stake and trust (track record of information contribution). [[cyber]]'s current model weights only by stake. the veritas trust metric — accumulated BTS score history — is the missing component that would make [[karma]] a full epistemic weight, not just an economic one.

---

## comparison: veritas vs LMSR

| | veritas | LMSR |
|---|---|---|
| output | full probability distribution | point price ∈ (0,1) |
| scoring | KL divergence against mean | net position at resolution |
| resolution | none (continuous update) | oracle-dependent |
| measures | information contribution (bits) | prediction accuracy |
| attack response | noise penalized, stake redistributed | liquidity injected, price sharpened |
| second-order | yes (meta-predictions) | no (layer 3 in [[cyberlink market protocol]]) |

veritas is epistemically richer. LMSR is simpler to implement. for the [[cybergraph]], veritas is the correct target for the epistemic layer — LMSR is a tractable approximation.

---

## the key claim

without an epistemic layer, the [[cybergraph]] is excitation-only: it accumulates structural connections but cannot deactivate misleading ones. with veritas-style scoring, the [[cybergraph]] gains the inhibitory signal described in [[market inhibition]] — but grounded in information theory rather than market price alone.

a [[cyberlink]]'s effective weight in the [[tri-kernel]] would be:

$$w_\text{eff}(e) = \text{stake}(e) \times \text{trust}(\nu_e) \times f(\text{veritas score of } e)$$

where $f$ maps accumulated BTS scores to a weight multiplier. links from high-trust [[neurons]] whose previous signals proved informative carry more weight. links from noise producers are down-weighted automatically.

truth is emerging — from the interaction of structural [[knowledge]] ([[cyberlinks]]) and epistemic [[knowledge]] (veritas scores). neither alone is sufficient.

see [[Bayesian Truth Serum]] for the mathematical foundation. see [[two kinds of knowledge]] for the structural/epistemic split. see [[market inhibition]] for why the epistemic layer is necessary. see [[cyberlink market protocol]] for the LMSR approximation. see [[wisdom of the crowds]] for the aggregation background. see [[syntropy]] for the information-theoretic signal.
