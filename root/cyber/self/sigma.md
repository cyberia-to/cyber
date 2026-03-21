---
tags: cyber, article, draft, research
alias: own balances, protocol treasury, protocol balances, system balances, protocol capital, sigma
crystal-type: pattern
crystal-domain: cyber
crystal-size: enzyme
focus: 0.00014439460864938488
gravity: 3
---

the resources the [[cybergraph]] manages for itself — the protocol's own economic agency

the cybergraph is not a passive infrastructure. it holds tokens, locks stake, takes market positions, and allocates compute cycles. these are not administrator actions — they are protocol-level behaviors specified in the base mechanics, executed by the [[autonomous neuron]] using the same mechanisms available to every [[neuron]].

---

## $CYB treasury

the emission curve E(t) in [[cyber/tokenomics]] allocates a fraction of each block's emission to a protocol-controlled address. this address is derived deterministically from the genesis block — no private key held by any party, funds spendable only by on-chain governance execution or protocol-defined automated mechanisms.

the treasury accumulates across the emission lifetime. its balance at any block is auditable by any participant. the uses are encoded:

self-linking allocation. a fraction of treasury funds the stake for system-created [[cyberlinks]]. each self-link consumes a small amount proportional to the link's confidence score. the consumption rate is metabolic: when M(t) is high, the allocation is generous; when metabolic health is low, the rate throttles.

neuron recruitment. the treasury funds onboarding grants for new [[neurons]] — small initial stake allocations that bootstrap participation without requiring new participants to have prior capital. this increases neuron diversity, which increases $\bar{k}$, which decreases $\rho$, which raises the phase threshold $|P^*|$, which expands the space of semantic dimensions the graph can represent.

cross-chain reserves. a portion is allocated as IBC liquidity, maintaining cross-chain bridges active and providing the external validation signal in the metabolic health function. the protocol holds reserves in multiple denominations, ensuring the cap signal remains informative even during single-chain volatility.

---

## will (locked tokens)

[[will]] is locked [[tokens]] — capital committed for a defined duration in exchange for influence on the [[cybergraph]]. the protocol can lock its own treasury tokens as will, backing long-horizon claims with provable commitment.

when the system creates a [[cyberlink]] backed by will-locked tokens, it produces the blocking proof (§19.3): the tokens are demonstrably unspendable for the lock duration. any observer can verify the commitment. the effective weight of a will-backed link does not drift with token mobility — it is fixed for the duration.

this is costly signaling by the protocol itself. a system link backed by locked will says: "the protocol commits its own compute capacity against this claim for N years." the opportunity cost is real — those tokens cannot be redeployed. will-backed self-links are the protocol's highest-conviction assertions.

the system uses will selectively: only for links where structural evidence is overwhelming and the claim is fundamental enough to warrant multi-year commitment. foundational ontology links — the semantic core particles that define the graph's category structure — are will-backed. current-events links are not.

---

## ICBS market positions

the protocol participates in the [[inversely coupled bonding surface|ICBS]] epistemic market as a trader. when the system's structural inference diverges from market prices — a high-focus link underpriced, a low-focus link overpriced — the protocol takes a position.

the system has an information advantage no individual participant can match: it holds the full graph state, the full BTS scoring history, and the full focus distribution. it is the single most informed participant in every market. its trades are not speculative — they are corrections from the most comprehensive epistemic vantage point available.

market behavior: when the ICBS price of a link is below the system's inference estimate, the protocol buys YES. when the price is above, it buys NO. this pressure moves the price toward the structural consensus. the market converges faster because the most-informed participant is actively correcting it.

protocol market positions are transparent — the protocol neuron's key is public. participants can observe the system's market bets and update their own positions accordingly. the protocol's trading book is a signal, not a secret.

---

## computation allocation

the [[FFC]] has a finite compute budget at each timescale. the system allocates cycles across three priorities:

| priority | timescale | allocation logic |
|---|---|---|
| query service | fast (~seconds) | proportional to query load; minimum floor always reserved |
| DMN processing | fast (background) | fills slack capacity; scaled up during low-query periods |
| maintenance | slow (~hours) | fixed budget per epoch; runs archival, shard rebalancing, self-link creation |

the allocation is dynamic. during high-traffic periods, query service gets priority and DMN defers. during idle periods, DMN runs at full budget. maintenance runs on the slow clock regardless of fast-timescale load.

the system does not over-commit compute: if the maintenance budget is consumed before the archival sweep completes, the remainder is deferred to the next epoch rather than displacing query service. the scheduling guarantee is: query latency is never degraded by DMN or maintenance operations.

---

## the compound effect

four resource categories — treasury, will, market positions, compute — managed autonomously according to the metabolic objective M(t). each one amplifies the others:

treasury funds self-linking, which increases graph density, which improves inference quality, which increases BTS scoring accuracy, which increases protocol karma, which increases system link weight, which increases the graph's self-improvement rate.

will-backed links provide the stable foundation that other links reference. they are the graph's bedrock — the claims the market does not attempt to move because the protocol's commitment makes movement too expensive.

market positions correct mispriced edges, improving the ICBS signal that feeds into the tri-kernel effective adjacency. better market prices mean better tri-kernel inference means better self-links means stronger market corrections.

compute allocation ensures the DMN runs during quiet periods, maintaining the self-model and running counterfactuals. good self-model accuracy leads to better parameter adjustments. better parameters improve metabolic health. better metabolic health increases treasury accumulation. more treasury enables more self-linking.

the system is self-financing: its good performance generates the resources that sustain its performance.

see [[self-linking]] for what the treasury funds. see [[dmn]] for how compute allocation shapes resting-state inference. see [[parametrization]] for the metabolic feedback that governs all four resource categories.