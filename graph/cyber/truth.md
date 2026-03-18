---
tags: cyber, core
alias: two factor truth, two layer truth, structural epistemic truth, truth model
crystal-type: pattern
crystal-domain: cyber
stake: 13572769588772200
---
truth in the [[cybergraph]] has two irreducible components. neither alone is sufficient. together they define what the network calls true

| factor | form | source | question answered |
|--------|------|--------|-------------------|
| structural | binary — the [[cyberlink]] exists | one [[neuron]]'s signed assertion | what is connected to what? |
| epistemic | continuous — ICBS price $\in (0,1)$ | all market participants | how much does the collective believe this connection? |

the structural layer is permanent and append-only — a link that exists cannot be deleted, only economically muted. the epistemic layer is dynamic — the market price shifts continuously as new [[neurons]] buy TRUE or FALSE positions on the edge.

---

## why two factors

a single-factor truth model fails in one of two directions.

structural only: all [[cyberlinks]] weighted by stake alone. $\pi^*$ reflects link count and economic weight, but the graph cannot distinguish a well-supported theorem from well-funded spam. the [[tri-kernel]] converges — but possibly to a false attractor. there is no inhibitory signal.

epistemic only: markets over propositions with no underlying link structure. the market has no substrate — nothing to trade on. belief without assertion is formless.

the two-factor model resolves this: the structural link creates the question. the epistemic market discovers the answer. the [[cyberlink]] asserts "A relates to B." the [[inversely coupled bonding surface|ICBS]] market over that edge asks "does the collective believe A relates to B?" the price that emerges is the second truth factor.

## the formal account

the effective weight of an edge in the [[tri-kernel]]:

$$A^{\text{eff}}_{pq} = \sum_{\substack{\ell \in L \\ \text{src}(\ell)=p,\;\text{tgt}(\ell)=q}} a(\ell)\cdot\kappa(\nu(\ell))\cdot f(m(\ell))$$

factor one: $a(\ell)$ — stake on the structural assertion (economic weight of the binary fact)

factor two: $m(\ell) \in (0,1)$ — ICBS reserve ratio (market-implied probability the link is valid), transformed by $f$

the two factors multiply. a high-stake link the market disbelieves is suppressed toward zero. a low-stake link the market strongly confirms is amplified through [[karma]] and market confidence. the truth signal is the product of conviction and collective validation.

## the ternary bridge

between binary structure and continuous belief sits [[valence]] $v \in \{-1, 0, +1\}$ — the coarse epistemic signal provided at link creation. it is not a third truth factor but the seed that initializes the market. the neuron's prediction of where the ICBS market will settle, expressed in three states, before the collective has spoken.

the full truth model: binary structure → ternary seed → continuous market → [[focus]] distribution $\pi^*$. each layer requires the one below it.

## the truth block

| Page | Role |
|------|------|
| [[true-false problem]] | why global [[cyberank]] alone cannot answer contextual questions |
| [[standard inference]] | the naive first solution — [[will]]-weighted context scoring |
| [[costly signal]] | why [[will]] cost makes [[cyberlinks]] honest |
| [[bayesian truth serum]] | honesty [[equilibrium]] via [[valence]] meta-predictions |
| [[inversely coupled bonding surface]] | the market mechanism — TRUE and FALSE geometrically coupled |
| [[market inhibition]] | how markets provide the inhibitory signal raw links cannot |
| [[cyberlink market protocol]] | the unified 2|3 architecture — topology + market + meta-prediction |

see [[two kinds of knowledge]] for the full structural/epistemic analysis. see [[valence]] for the ternary seed. see [[truth]] for the convergent signal both factors produce
