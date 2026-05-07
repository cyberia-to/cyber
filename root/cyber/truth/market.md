---
tags: cyber, cip, draft, research
alias: cyberlink market protocol, self evaluating knowledge graph, two dimensional epistemic signal
crystal-type: process
crystal-domain: cyber
crystal-size: deep
authors: mastercyb
---

a self-evaluating [[knowledge]] graph with two-dimensional epistemic signal

*mastercyb · Cyber Valley · 2026*

---

## principle

creating a link in the [[knowledge]] graph = creating a market on the truth of that link. one atomic action produces both [[knowledge]] and its verification mechanism. all individual actions are private (ZKP). only aggregates are public.

---

## three layers in one act

### layer 1: topology (binary)

an agent creates [[cyberlink]] A→B and deposits stake. the stake becomes the initial LMSR liquidity for a market on that edge. creating an edge costs money → spam is expensive → the graph self-cleans.

- public: edge exists
- private: who created it

### layer 2: market (continuous)

each edge carries a prediction market with two outcome tokens: TRUE and FALSE. agents buy positions, moving the price. price of TRUE ∈ (0,1) = implied probability that the link is true/useful.

the market mechanism is the [[coupling]] (ICBS): $C(s_{YES}, s_{NO}) = \lambda\sqrt{s_{YES}^2 + s_{NO}^2}$. ICBS was adopted over LMSR because: self-scaling liquidity (trading volume grows TVL automatically), early conviction rewarded (prices range 0 to λ, not [0,1]), inverse coupling (buying YES directly suppresses NO's price — TRUE and FALSE are geometrically opposed on a circle). no external LPs needed. the protocol is the market maker.

the market is perpetual — no oracle resolution. periodic liquidity transfer from the winning token to the losing one acts as a damper: prevents the market from freezing into dogma, always preserves liquidity for challenge. usage signal ([[cyberank]], traffic through the edge) serves as a soft oracle: if the edge is actively traversed, the TRUE price receives a weak upward nudge.

- public: TRUE/FALSE price, volume
- private: who holds what position, position sizes

### layer 3: meta-prediction (ternary)

simultaneously with their market position, each agent makes a staked prediction: where will the market converge?

- +1: market will converge to TRUE
- −1: market will converge to FALSE
- 0: market will not resolve

this is a paid prediction about collective [[knowledge]] — peer prediction, falsifiable by the market. wrong prediction → lose stake.

the mechanism is based on Bayesian Truth Serum (Prelec, 2004) and the Surprisingly Popular Algorithm. the question is not "is A→B true?" but "will the market converge to TRUE?" — a second-order belief about collective [[knowledge]], not a first-order belief about the world.

- public: aggregated meta-score
- private: individual predictions

---

## two-dimensional epistemic signal

the divergence between market price (first-order) and meta-score (second-order) is a measure of epistemic confidence:

price and meta align — the market is self-confident. strong signal.

TRUE price high, meta lower — people bet on TRUE more than they expect others to. private [[knowledge]] in the market. signal may be stronger than it appears. contrarians with conviction — they know something others don't yet.

TRUE price high, meta higher — people bet on TRUE less than they expect the market to. herding behavior, momentum. signal may be weaker than it appears.

meta-score near zero — participants don't know where the market will converge. genuine uncertainty.

two numbers: magnitude (price) and confidence (meta). one-dimensional price → two-dimensional signal.

---

## public aggregates

for each edge in the [[cybergraph]], an external observer sees three numbers:

| aggregate | what it says | source |
|---|---|---|
| edge existence | someone paid for this question | layer 1 (binary) |
| TRUE price | market consensus | layer 2 (continuous) |
| meta-score | market's confidence in itself | layer 3 (ternary) |

from these, the system derives:

- rank — from price and topology (modified [[cyberank]])
- confidence — from divergence between price and meta-score
- signal quality — from volume and [[neuron]] count

everything else is behind ZKP. who created, who bet, how much, which direction — private.

---

## why full privacy

the brain's [[neurons]] don't know which neighbor sent a signal. a synapse receives neurotransmitter — excitatory or inhibitory — but doesn't know "this is from neuron #47291." it knows only the aggregate: total membrane potential. if threshold is exceeded — spike. if not — silence.

in [[mycelium]]: a hypha "senses" a concentration gradient. more sugar on the right — flow goes right. the hypha doesn't know "this is from oak #3." it knows the aggregate.

privacy is an architectural principle of the computational system. the brain is private not to protect [[neurons]]. it is private because aggregated signal is more informative than individual signal for the task of computation. disclosing individual signals would add noise, not signal.

without privacy, the market is vulnerable: I see TRUE is winning 80/20 and bet TRUE not because I believe it but because of momentum. herding. the market loses informativeness.

with ZKP: you see the price (aggregate) but not positions. you don't know if one whale holds 80% TRUE or a thousand small agents. you are forced to bet based on your actual belief, not based on observing others. pure signal.

---

## properties

spam resistance. each edge costs stake. junk edges attract no traders → price falls to 0 → rank = 0 → invisibility. spam self-destructs economically.

antifragility. attacking an edge (betting on FALSE) = liquidity injection. the stronger the attack, the more liquid the market, the more accurate the price. junk edges aren't worth attacking. important edges get attacked and emerge stronger. Lindy effect.

meritocratic [[knowledge]] economy. agents whose bets and meta-predictions prove correct earn returns. good epistemologists get richer. bad ones get poorer. reputation from first principles: not voting on reputation but P&L.

no vote buying. there are no votes — nothing to buy. only market positions, private behind ZKP. buying a position = a bet with risk, not corruption. even "vote buying" in this context means paying to move the price of TRUE — but if the market disagrees, you lose. advertising with skin in the game.

no social pressure. aggregates are visible but not attributed. you cannot say "smart money is betting TRUE." you cannot copy a whale's strategy. you cannot build social proof. clean signal.

self-referential graph. each edge is simultaneously [[knowledge]] and a market on that [[knowledge]]. the graph trades itself. a [[cyberlink]] simultaneously transmits a signal and evaluates its own usefulness through the market mechanism. a connection that works — strengthens. a useless one — withers.

---

## the 2|3 architecture

binary → ternary → continuous. three levels, from discrete to dense:

```
topology     [2]  edge exists / doesn't              binary
meta         [3]  converge+ / uncertain / converge−   ternary
market       [∞]  price ∈ (0,1)                      continuous
```

the same architecture as DNA (4 bases → 3-position codons → 20 amino acids → ∞ proteins), [[neurons]] (spike/no spike → excitation/modulation/inhibition → continuous potential), [[mycelium]] (connection yes/no → give/hold/receive → continuous flow). see [[two three paradox]] and [[binary topology ternary economics]].

only aggregates are public — like the membrane potential on the outside of a neuron: one summary signal from thousands of private inputs.

---

## ICBS specifics

the [[coupling]] (Williams & Buterin, 2020) is the market mechanism. cost function: $C(s_{YES}, s_{NO}) = \lambda\sqrt{s_{YES}^2 + s_{NO}^2}$.

no external LPs needed. the protocol is the market maker. self-scaling: trading volume automatically grows TVL, so the most-contested edges become the most liquid. probability is encoded in the reserve ratio: $q = r_{YES}/(r_{YES} + r_{NO})$.

works on thin markets. even with one trader, the market produces a meaningful price. parameter λ (set at deployment by the initial deposit) controls the market's scale without bounding its information range.

early conviction rewarded. prices range from 0 to λ — not bounded to [0,1]. a [[neuron]] who links something the market later validates strongly earns arbitrarily large returns relative to late consensus-following. this directly incentivizes surfacing private [[knowledge]] early.

probability encoding. TRUE(A→B) reserve ratio = 0.73 means "the market estimates the probability of the link's utility at 73%." this plugs directly into ranking and the [[tri-kernel]].

bootstrapping liquidity. options: (a) link creator pays — creating [[knowledge]] costs money, spam becomes expensive; (b) protocol subsidizes — [[bostrom]] mints [[tokens]] for initial liquidity, inflation = price of collective [[knowledge]]; (c) hybrid — creator pays part, protocol supplements based on creator's [[karma]]. trusted agents get more subsidy. mycelial analogy: the fungus more readily extends hyphae from large healthy trees.

---

## perpetual market dynamics

no oracle resolves the market. instead:

liquidity transfer. periodically, a fraction of liquidity transfers from the winning side to the losing side. this ensures the losing side always has enough liquidity for a challenger to enter cheaply. anti-echo-chamber mechanism built into the economics. analogous to how [[mycelium]] maintains even unprofitable hyphae — you never know when a weak connection will become critical.

usage as soft oracle. [[cyberank]] (traffic, citations, traversals through the edge) provides a weak signal. high-rank edges get a small TRUE nudge. this is not resolution — a nudge. like [[mycelium]]: if resource actually flows through a hypha, the hypha thickens.

feedback loop. rank influences visibility → visibility influences usage → usage influences TRUE price → price influences rank. positive feedback with damping (liquidity transfer = damper). the same as in [[mycelium]]: more resource through a hypha → hypha thickens → more resource through hypha.

---

## open questions

- transfer parameters: speed, frequency, and dependency on volume for the liquidity transfer mechanism
- bonding curve: standard LMSR or modification for perpetual markets without resolution
- meta-prediction pricing: how stake and payoff are determined for layer 3; resolution criteria for meta-predictions
- bootstrapping: protocol subsidy vs full creator payment vs hybrid; optimal b parameter per edge
- convergence dynamics: what transfer parameters give stable convergence vs oscillation vs divergence; connection to e ≈ 2.718
- rank-price interaction: feedback loop dynamics, stability conditions, preventing circular reinforcement

see [[coupling]] for the market mechanism. see [[serum]] for the meta-prediction scoring. see [[proper scoring rules]] for the theoretical foundation. see [[cyber/epistemology]] for threat model and epistemic correctness. see [[foculus]] for the consensus mechanism that interacts with market finality.

---

2ᵐ ≠ 3ⁿ — and in this gap lives [[intelligence]]