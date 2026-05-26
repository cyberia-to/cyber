---
tags: cyber, core
alias: two factor truth, two layer truth, structural epistemic truth, truth model
crystal-type: pattern
crystal-domain: cyber
---
truth in the [[cybergraph]] has two irreducible components. neither alone is sufficient. together they define what the network calls true

| factor | form | source | question answered |
|--------|------|--------|-------------------|
| structural | binary — the [[cyberlink]] exists | one [[neuron]]'s signed assertion | what is connected to what? |
| epistemic | continuous — [[coupling]] price $\in (0,1)$ | all market participants | how much does the collective believe this connection? |

the structural layer is permanent and append-only — a link that exists cannot be deleted, only economically muted. the epistemic layer is dynamic — the market price shifts continuously as new [[neurons]] buy [[true]] or [[false]] positions on the edge

---

## why two factors

a single-factor truth model fails in one of two directions

structural only: all [[cyberlinks]] weighted by stake alone. $\phi^*$ reflects link count and economic weight, but the graph cannot distinguish a well-supported theorem from well-funded spam. the [[tri-kernel]] converges — but possibly to a false attractor. there is no inhibitory signal

epistemic only: markets over propositions with no underlying link structure. the market has no substrate — nothing to trade on. belief without assertion is formless

the two-factor model resolves this: the structural link creates the question. the epistemic market discovers the answer. the [[cyberlink]] asserts "A relates to B." the [[coupling]] market over that edge asks "does the collective believe A relates to B?" the price that emerges is the second truth factor

## the formal account

the effective weight of an edge in the [[tri-kernel]]:

$$A^{\text{eff}}_{pq} = \sum_{\substack{\ell \in L \\ \text{src}(\ell)=p,\;\text{tgt}(\ell)=q}} a(\ell)\cdot\kappa(\nu(\ell))\cdot f(m(\ell))$$

factor one: $a(\ell)$ — stake on the structural assertion (economic weight of the binary fact)

factor two: $m(\ell) \in (0,1)$ — [[coupling]] reserve ratio (market-implied probability the link is valid), transformed by $f$

the two factors multiply. a high-stake link the market disbelieves is suppressed toward zero. a low-stake link the market strongly confirms is amplified through [[karma]] and market confidence. the truth signal is the product of conviction and collective validation

## the ternary bridge

between binary structure and continuous belief sits [[valence]] $v \in \{-1, 0, +1\}$ — the coarse epistemic signal provided at link creation. it is not a third truth factor but the seed that initializes the market. the [[neuron]]'s prediction of where the [[coupling]] market will settle, expressed in three states, before the collective has spoken

the full truth model: binary structure → ternary seed → continuous market → [[focus]] distribution $\phi^*$. each layer requires the one below it

## valence strategy

[[valence]] is part of the [[attention]] pipeline — predictions are the first unit of collective [[attention]] on an edge's truth value

| Strategy | What happens | Payoff |
|----------|-------------|--------|
| [[true]] (v=+1) | seeds market toward TRUE. if correct, effective weight starts high immediately | accuracy × time — early correct prediction compounds [[prob]] from block T |
| [[false]] (v=-1) | seeds market toward FALSE. same mechanics, opposite direction | same — early correct suppression compounds |
| [[void]] (v=0) | balanced market. waits for others to trade | safe but slow — misses N blocks of directional [[prob]] accumulation |

the payoff is accuracy × time. being right early compounds. being right late earns less. being wrong costs blocks of suppressed weight. being void is free but slow

a [[neuron]] with no private [[knowledge]] should play [[void]] — avoids the penalty of guessing wrong. a [[neuron]] with genuine conviction should predict — the first-mover advantage on market seeding is the reward for private [[knowledge]]

this is the attention yield curve — but it emerges naturally from the mechanics rather than being a designed reward formula. early accurate conviction → early market seeding → early effective weight → more blocks of [[prob]] accumulation → higher [[karma]]. the physics does it

## the truth block

### attractors

| | |
|---|---|
| [[true]] | market → 1 — edge validated, [[focus]] flows |
| [[void]] | market → 0.5 — no signal, channel open but empty |
| [[false]] | market → 0 — edge suppressed, [[focus]] blocked |

### mechanisms

| | |
|---|---|
| [[valence]] | the ternary seed — +1 / 0 / -1 at link creation |
| [[serum]] | honesty [[equilibrium]] via [[valence]] meta-predictions |
| [[coupling]] | the market mechanism — TRUE and FALSE geometrically coupled |
| [[inhibition]] | how markets provide the inhibitory signal raw links cannot |
| [[cost]] | why [[will]] cost makes [[cyberlinks]] honest |
| [[cyber/truth/honesty|honesty]] | why [[neurons]] act honestly — cost + serum + coupling compound |
| [[cyber/truth/market|market]] | the unified 2/3 architecture — topology + market + meta-prediction |

### lineage

| | |
|---|---|
| [[two kinds of knowledge]] | structural vs epistemic — why two factors are irreducible |
| [[true-false problem]] | why global [[cyberank]] alone cannot answer contextual questions |
| [[standard inference]] | the naive first solution — [[will]]-weighted context scoring |

see [[truth]] for the convergent signal both factors produce

discover all [[concepts]]
