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

## the card

every cyberlink is also a card — an epistemic asset with four properties:

immutable. axiom A3 (append-only) guarantees the record $\ell = (\nu, p, q, \tau, a, v, t)$ is permanent once published. the assertion cannot be altered or retracted. the author's conviction, valence, and timestamp are locked into the graph's history forever. immutability is what makes the card a credible commitment rather than a revisable claim

unique. the 7-tuple is the card's identity — no two cyberlinks are identical (block height $t$ ensures this even when the same author re-links the same particles). each card is non-fungible: it is a specific assertion, by a specific author, at a specific block, with a specific conviction

transferable. ownership of a cyberlink — and thus the rights to its yield and governance weight — can be transferred between [[neurons]]. the structural record stays in $L$ forever; beneficial ownership moves. this separates the assertion (immutable, authorial) from the economic position (transferable, tradeable)

yield-bearing. a cyberlink earns in proportion to how much the target particle gains [[focus]]:

$$R_\ell(T) = \int_0^T w(t) \cdot \Delta\pi^*(q, t)\, dt$$

where $w(t)$ is the conviction weight at time $t$ and $\Delta\pi^*(q, t)$ is the increment in the target particle's focus. a link that correctly anticipated an important particle — created early, with genuine conviction — earns the most. early discovery is maximally rewarded; late consensus-following earns little

the card unifies what financial instruments split: the assertion (content), the commitment (conviction), the epistemic signal (valence), and the yield right — all in one atomic, immutable, tradeable record

## the first link

the first cyberlink any [[particle]] receives is always a [[name]], turning the raw hash into a [[file]]. further links weave it into the [[cybergraph]]. the accumulated graph of all cyberlinks IS [[knowledge]]

see [[cybergraph]] for the formal definition including all six axioms. see [[valence]] for the ternary epistemic field. see [[Bayesian Truth Serum]] for the scoring that uses $v$. see [[effective adjacency]] for how conviction weights enter the [[tri-kernel]]. see [[eternal cyberlinks]] for the permanent-premium variant. see [[knowledge economy]] for the full epistemic asset taxonomy

discover all [[concepts]]
