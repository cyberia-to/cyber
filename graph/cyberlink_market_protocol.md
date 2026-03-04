# Cyberlink Market Protocol

**A Self-Evaluating Knowledge Graph with Two-Dimensional Epistemic Signal**

*mastercyb · Cyber Valley · 2026*

---

## Principle

Creating a link in the knowledge graph = creating a market on the truth of that link. One atomic action produces both knowledge and its verification mechanism. All individual actions are private (ZKP). Only aggregates are public.

---

## Three Layers in One Act

### Layer 1: Topology (binary)

An agent creates cyberlink A→B and deposits stake. The stake becomes the initial LMSR liquidity for a market on that edge. Creating an edge costs money → spam is expensive → the graph self-cleans.

- **Public:** edge exists
- **Private:** who created it

### Layer 2: Market (continuous)

Each edge carries an LMSR market with two outcome tokens: TRUE and FALSE. Agents buy positions, moving the price. Price of TRUE ∈ (0,1) = implied probability that the link is true/useful.

LMSR is chosen because: no external LPs needed (the protocol is the market maker), works on thin markets (most edges will have few traders), bounded loss (b·ln2 per edge), price = probability directly.

The market is perpetual — no oracle resolution. Periodic liquidity transfer from the winning token to the losing one acts as a damper: prevents the market from freezing into dogma, always preserves liquidity for challenge. Usage signal (cyber~Rank, traffic through the edge) serves as a soft oracle: if the edge is actively traversed, the TRUE price receives a weak upward nudge.

- **Public:** TRUE/FALSE price, volume
- **Private:** who holds what position, position sizes

### Layer 3: Meta-Prediction (ternary)

Simultaneously with their market position, each agent makes a staked prediction: where will the market converge?

- **+1:** market will converge to TRUE
- **−1:** market will converge to FALSE
- **0:** market will not resolve

This is not an opinion (free, cheap talk, proof can be sold). It is a paid prediction about collective knowledge — peer prediction, falsifiable by the market. Wrong prediction → lose stake.

The mechanism is based on Bayesian Truth Serum (Prelec, 2004) and the Surprisingly Popular Algorithm. The question is not "is A→B true?" but "will the market converge to TRUE?" — a second-order belief about collective knowledge, not a first-order belief about the world.

- **Public:** aggregated meta-score
- **Private:** individual predictions

---

## Two-Dimensional Epistemic Signal

The divergence between market price (first-order) and meta-score (second-order) is a measure of epistemic confidence:

**Price and meta align** — the market is self-confident. Strong signal.

**TRUE price high, meta lower** — people bet on TRUE more than they expect others to. Private knowledge in the market. Signal may be stronger than it appears. Contrarians with conviction — they know something others don't yet.

**TRUE price high, meta higher** — people bet on TRUE less than they expect the market to. Herding behavior, momentum. Signal may be weaker than it appears.

**Meta-score near zero** — participants don't know where the market will converge. Genuine uncertainty.

Two numbers: magnitude (price) and confidence (meta). One-dimensional price → two-dimensional signal.

---

## Public Aggregates

For each edge in the graph, an external observer sees three numbers:

| Aggregate | What it says | Source |
|---|---|---|
| Edge existence | Someone paid for this question | Layer 1 (binary) |
| TRUE price | Market consensus | Layer 2 (continuous) |
| Meta-score | Market's confidence in itself | Layer 3 (ternary) |

From these, the system derives:

- **Rank** — from price and topology (modified cyber~Rank)
- **Confidence** — from divergence between price and meta-score
- **Signal quality** — from volume and participant count

Everything else is behind ZKP. Who created, who bet, how much, which direction — private.

---

## Why Full Privacy

The brain's neurons don't know which neighbor sent a signal. A synapse receives neurotransmitter — excitatory or inhibitory — but doesn't know "this is from neuron #47291." It knows only the aggregate: total membrane potential. If threshold is exceeded — spike. If not — silence.

In mycelium: a hypha "senses" a concentration gradient. More sugar on the right — flow goes right. The hypha doesn't know "this is from oak #3." It knows the aggregate.

Privacy is not a feature for users. It is an **architectural principle of the computational system.** The brain is private not to protect neurons. It is private because **aggregated signal is more informative than individual signal** for the task of computation. Disclosing individual signals would add noise, not signal.

Without privacy, the market is vulnerable: I see TRUE is winning 80/20 and bet TRUE not because I believe it but because of momentum. Herding. The market loses informativeness.

With ZKP: you see the price (aggregate) but not positions. You don't know if one whale holds 80% TRUE or a thousand small agents. You are forced to bet based on your *actual belief*, not based on observing others. Pure signal.

---

## Properties

**Spam resistance.** Each edge costs stake. Junk edges attract no traders → price falls to 0 → rank = 0 → invisibility. Spam self-destructs economically.

**Antifragility.** Attacking an edge (betting on FALSE) = liquidity injection. The stronger the attack, the more liquid the market, the more accurate the price. Junk edges aren't worth attacking. Important edges get attacked and emerge stronger. Lindy effect on steroids.

**Meritocratic knowledge economy.** Agents whose bets and meta-predictions prove correct earn returns. Good epistemologists get richer. Bad ones get poorer. Reputation from first principles: not voting on reputation but P&L.

**No vote buying.** There are no votes — nothing to buy. Only market positions, private behind ZKP. Buying a position = a bet with risk, not corruption. Even "vote buying" in this context means paying to move the price of TRUE — but if the market disagrees, you lose. Advertising with skin in the game.

**No social pressure.** Aggregates are visible but not attributed. You cannot say "smart money is betting TRUE." You cannot copy a whale's strategy. You cannot build social proof. Clean signal.

**Self-referential graph.** Each edge is simultaneously knowledge and a market on that knowledge. The graph trades itself. Analogy: a neural connection simultaneously transmits a signal and evaluates its own usefulness through Hebbian learning. A connection that works — strengthens. A useless one — withers. No external arbiter.

---

## The 2|3 Architecture

Binary → ternary → continuous. Three levels, from discrete to dense:

```
Topology     [2]  edge exists / doesn't        binary
Meta         [3]  converge+ / uncertain / converge−   ternary
Market       [∞]  price ∈ (0,1)               continuous
```

The same architecture as DNA (4 bases → 3-position codons → 20 amino acids → ∞ proteins), neurons (spike/no spike → excitation/modulation/inhibition → continuous potential), mycelium (connection yes/no → give/hold/receive → continuous flow).

Only aggregates are public — like the membrane potential on the outside of a neuron: one summary signal from thousands of private inputs.

---

## LMSR Specifics

Hanson's Logarithmic Market Scoring Rule fits this use case precisely:

**No external LPs needed.** The protocol is the market maker. Maximum loss is bounded: b·ln(n), where b is the liquidity parameter and n is the number of outcomes. For a binary market (TRUE/FALSE): b·ln(2) ≈ 0.693b. This is a known-in-advance cost of operating the market.

**Works on thin markets.** Most edges in a knowledge graph will have 0–5 traders. Uniswap-style AMMs on such markets are catastrophic — one trade moves the price 50%. LMSR is designed for thin markets: parameter b controls sensitivity, and even with one trader, the market produces a meaningful price.

**Price = probability.** In LMSR, outcome token price is literally implied probability. No conversion needed. TRUE(A→B) price = 0.73 means "the market estimates the probability of the link's utility at 73%." This plugs directly into ranking.

**Bootstrapping liquidity.** Who pays for b? Options: (a) link creator pays — creating knowledge costs money, max loss known in advance, spam becomes expensive; (b) protocol subsidizes — Bostrom mints tokens for initial liquidity, inflation = price of collective knowledge; (c) hybrid — creator pays part, protocol supplements based on creator's rank. Trusted agents get more subsidy. Mycelial analogy: the fungus more readily extends hyphae from large healthy trees.

---

## Perpetual Market Dynamics

No oracle resolves the market. Instead:

**Liquidity transfer.** Periodically, a fraction of liquidity transfers from the winning side to the losing side. This ensures the losing side always has enough liquidity for a challenger to enter cheaply. Anti-echo-chamber mechanism built into the economics. Analogous to how mycelium maintains even unprofitable hyphae — you never know when a weak connection will become critical.

**Usage as soft oracle.** cyber~Rank (traffic, citations, traversals through the edge) provides a weak signal. High-rank edges get a small TRUE nudge. Not resolution — a nudge. Like mycelium: if resource actually flows through a hypha, the hypha thickens.

**Feedback loop.** Rank influences visibility → visibility influences usage → usage influences TRUE price → price influences rank. Not a vicious circle but a feedback loop — the same as in mycelium: more resource through a hypha → hypha thickens → more resource through hypha. Positive feedback with damping (liquidity transfer = damper).

---

## Open Questions

- **Transfer parameters:** speed, frequency, and dependency on volume for the liquidity transfer mechanism
- **Bonding curve:** standard LMSR or modification (pm-AMM style from Paradigm) for perpetual markets without resolution
- **Meta-prediction pricing:** how stake and payoff are determined for Layer 3; resolution criteria for meta-predictions
- **Bootstrapping:** protocol subsidy vs full creator payment vs hybrid; optimal b parameter per edge
- **Convergence dynamics:** what transfer parameters give stable convergence vs oscillation vs divergence; connection to the number e ≈ 2.718
- **Rank-price interaction:** feedback loop dynamics, stability conditions, preventing circular reinforcement

---

*2ᵐ ≠ 3ⁿ — and in this gap lives intelligence.*
