---
icon: 🔗
tags: cyber, core
alias: cyberlink, cyberlinks, unit of knowledge, simple interactions, expert opinions, essential learning ability, cyberlinking, primitive learning acts
crystal-type: relation
crystal-domain: cyber
crystal-size: bridge
stake: 9929687381912652
---
the atomic unit of [[knowledge]]. a [[neuron]] binds two [[particles]] with a signed, staked, timestamped assertion — every cyberlink is simultaneously a [[learning]] act and an economic commitment

cheap talk produces noise. costly links produce [[knowledge]]

---

## the seven fields

$$\ell = (\nu,\; p,\; q,\; \tau,\; a,\; v,\; t)$$

| field | name | question answered |
|-------|------|-------------------|
| $\nu$ | author | who asserts this? |
| $p$ | from | which particle is the source? |
| $q$ | to | which particle is the target? |
| $\tau$ | token | in what denomination? |
| $a$ | amount | how much conviction? |
| $v$ | [[valence]] | what is the epistemic prediction? |
| $t$ | at | at which block? |

three layers in one atomic record:

- structural ($\nu$, $p$, $q$) — the assertion: author connects from→to. binary. it either exists or not
- epistemic ($v$) — the [[valence]]: author's prediction of where the [[inversely coupled bonding surface|ICBS]] market on this edge will settle. ternary: $\{-1, 0, +1\}$
- economic ($\tau$, $a$) — the conviction: the denomination and amount of what the author commits. together they determine the link's weight in [[effective adjacency]] and its yield under [[focus]] redistribution

conviction = ($\tau$, $a$): the pair that turns an assertion into a bet. denomination selects the [[token]], amount declares the stake. a link with zero conviction is structurally identical to a link with maximum conviction — the structural layer is binary. the conviction layer prices it

## the multiset property

the [[cybergraph]] is append-only. $t$ (block height) distinguishes every record: the same author linking from→to at block $t_1$ and again at block $t_2 > t_1$ produces two separate entries in $L$.

this matters in three ways:

- reinforcement — a neuron can increase conviction on a link by submitting again with higher $a$. [[effective adjacency]] sums all matching records: $A^{\text{eff}}_{pq} = \sum_\ell a(\ell) \cdot \kappa(\nu(\ell)) \cdot f(m(\ell))$
- valence update — a neuron can change its epistemic prediction after seeing market movement. the new $v$ is a new [[Bayesian Truth Serum|BTS]] input at the new block
- multi-denomination — the same structural link can be staked in different [[tokens]], expressing conviction across the token graph simultaneously

the structural triple $(\nu, p, q)$ is the identity of a relation. $(\tau, a, v, t)$ are the attributes of a specific assertion of that relation. multiple assertions of the same relation form the link's history

## the first link

the first cyberlink any [[particle]] receives is always a [[name]], turning the raw hash into a [[file]]. further links weave it into the [[cybergraph]]. the accumulated graph of all cyberlinks IS [[knowledge]]

see [[cybergraph]] for the formal definition including all six axioms. see [[valence]] for the ternary epistemic field. see [[Bayesian Truth Serum]] for the scoring that uses $v$. see [[effective adjacency]] for how conviction weights enter the [[tri-kernel]]. see [[cybergraph/cyberlink/hyperlink]] for comparison with hyperlinks

discover all [[concepts]]
