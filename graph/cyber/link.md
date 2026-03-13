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

## the nine fields

$$\ell \;=\; (\nu,\; p,\; q,\; \tau,\; a,\; v,\; t,\; \pi_\Delta,\; \sigma) \;\in\; N \times P \times P \times \mathcal{T} \times \mathbb{R}_{+} \times \{-1,\,0,\,+1\} \times \mathbb{Z}_{\geq 0} \times (P \times \mathbb{F}_p)^* \times \Pi$

| field | name | type | layer | semantics | question |
|-------|------|------|-------|-----------|----------|
| $\nu$ | [[subject]] | $N$ | structural | signing [[neuron]] | [[who]] asserts this? |
| $p$ | from | $P$ | structural | source [[particle]] | [[what]] is the source? |
| $q$ | to | $P$ | structural | target [[particle]] | [[what]] is the target? |
| $\tau$ | token | $\mathcal{T}$ | economic | token denomination | in what denomination? |
| $a$ | amount | $\mathbb{R}_+$ | economic | stake amount | how much conviction? |
| $v$ | [[valence]] | $\{-1,0,+1\}$ | epistemic | [[Bayesian Truth Serum\|BTS]] meta-prediction | what is the epistemic prediction? |
| $t$ | at | $\mathbb{Z}_{\geq 0}$ | temporal | block height | [[when]]? |
| $\pi_\Delta$ | focus delta | $(P \times \mathbb{F}_p)^*$ | computational | sparse [[focus]] update | how does this shift [[focus]]? |
| $\sigma$ | proof | $\Pi$ | verification | [[STARK]] proof of correct $\pi_\Delta$ | why should anyone believe it? |

four layers in one atomic record. structural $(\nu, p, q)$ is binary — the connection either exists or it doesn't. epistemic $v$ is ternary — the neuron's prediction of how the [[inversely coupled bonding surface|ICBS]] market on this edge will converge. economic $(\tau, a)$ is continuous over $\mathbb{R}_+$. computational $(\pi_\Delta, \sigma)$ carries a provably correct local update to the global [[focus]] distribution. see [[two three paradox]] for why this layering is not arbitrary

conviction = ($\tau$, $a$): the pair that turns an assertion into a bet. denomination selects the [[token]], amount declares the stake. a link with zero conviction is structurally identical to a link with maximum conviction — the structural layer is binary. the conviction layer prices it

focus delta = ($\pi_\Delta$, $\sigma$): the pair that turns a cyberlink into a distributed computation step. $\pi_\Delta$ is a sparse vector of (particle_id, $\Delta\pi$) pairs — the neuron's locally computed shift to the [[focus]] distribution $\pi^*$ caused by adding this link to the [[cybergraph]]. $\sigma$ is a [[STARK]] proof that $\pi_\Delta$ was correctly computed against the [[BBG]] root in the current header. any verifier checks $\sigma$ in $O(\log n)$ without recomputing the [[tri-kernel]]. if valid and $\|\pi_\Delta\| > 0$, the neuron may self-mint [[$CYB]] proportional to the proven shift. the [[locality theorem]] bounds the support of $\pi_\Delta$ to $O(\log(1/\varepsilon))$ hops — most entries are zero, so the sparse representation is compact. see §6.9 and §14.2 of the [[cyber/whitepaper]] for the full specification

the [[cybergraph]] is append-only. $t$ (block height) distinguishes every record: the same author linking from→to at block $t_1$ and again at block $t_2 > t_1$ produces two separate entries in $L$. this enables reinforcement (higher $a$ on a new record), valence updates (new $v$ at a new block), and multi-denomination staking (same structural link in different [[tokens]])

## conviction as UTXO

conviction is not a label attached to a link — it is a [[UTXO]]. creating a cyberlink is a transaction: the author moves $a$ tokens of denomination $\tau$ from a wallet UTXO to a new output bound to the cyberlink record. funds always move from one object to another. you cannot stake what you do not own.

the conviction output can itself be spent:

- transfer: spend the conviction UTXO to a new owner. the structural record stays in $L$; beneficial ownership moves. this is how the card's transferability operates at the protocol level
- withdraw: spend the conviction UTXO back to the author's wallet. the economic position closes. the structural record remains

the non-fungibility of the card (unique 9-tuple) and the fungibility of the token (transferable UTXO) coexist: the assertion and its proven focus contribution are non-fungible, the economic position is a standard UTXO output

## CRUD in the graph

the append-only graph expresses all four operations through cyberlinks:

| operation | cyberlink action | what changes |
|-----------|-----------------|--------------|
| create | first record for structural triple $(\nu, p, q)$ | relation enters $L$ |
| read | query $\pi^*$ at any block — no link required | nothing |
| update | new record with new $(\tau, a, v, t)$ for the same triple | any mutable dimension |
| delete | withdraw conviction UTXO + new record with $v = -1$ | economic position closed, epistemic signal negated |

the three mutable dimensions — epistemic ($v$), economic ($a$), and temporal ($t$) — vary independently. every combination is meaningful:

| $v$ | $a$ | reading |
|-----|-----|---------|
| $+1$ | high | funded affirmation — bet the market confirms |
| $+1$ | zero | unfunded affirmation — structural + epistemic signal, no economic exposure |
| $0$ | high | funded agnostic — stake without prediction |
| $0$ | zero | bare assertion — structural fact only |
| $-1$ | high | funded short — bet the market rejects |
| $-1$ | zero | logical retraction — epistemic negation, no economic exposure |

$v = -1$ does not mean the structural link is absent. the connection $p \to q$ is permanent (A3). $v = -1$ is the [[subject]]'s prediction that the [[inversely coupled bonding surface|ICBS]] market on this edge will converge to FALSE — a funded short when $a > 0$, a pure retraction when $a = 0$

delete in the graph is never erasure. the record $(\nu, p, q, t_{\text{first}})$ stays in $L$ permanently. economic close and epistemic retraction are separable operations — a subject can withdraw conviction while keeping $v = +1$, or submit $v = -1$ while maintaining stake. the full semantic delete is both together

## the card

every cyberlink is also a card — an epistemic asset with four properties:

immutable. axiom A3 (append-only) guarantees the record $\ell = (\nu, p, q, \tau, a, v, t, \pi_\Delta, \sigma)$ is permanent once published. the assertion cannot be altered or retracted. the author's conviction, valence, and timestamp are locked into the graph's history forever. immutability is what makes the card a credible commitment rather than a revisable claim

unique. the 9-tuple is the card's identity — no two cyberlinks are identical (block height $t$ ensures this even when the same author re-links the same particles). each card is non-fungible: it is a specific assertion, by a specific author, at a specific block, with a specific conviction

transferable. ownership of a cyberlink — and thus the rights to its yield and governance weight — can be transferred between [[neurons]]. the structural record stays in $L$ forever; beneficial ownership moves. this separates the assertion (immutable, authorial) from the economic position (transferable, tradeable)

yield-bearing. a cyberlink earns in proportion to how much the target particle gains [[focus]]:

$$R_\ell(T) = \int_0^T w(t) \cdot \Delta\pi^*(q, t)\, dt$$

where $w(t)$ is the conviction weight at time $t$ and $\Delta\pi^*(q, t)$ is the increment in the target particle's focus. a link that correctly anticipated an important particle — created early, with genuine conviction — earns the most. early discovery is maximally rewarded; late consensus-following earns little

the card unifies what financial instruments split: the assertion (content), the commitment (conviction), the epistemic signal (valence), and the yield right — all in one atomic, immutable, tradeable record

## the first link

the protocol accepts any cyberlink as the first to a particle — there is no enforcement of what that first link must be. by convention, a [[name]] link is typically the first: it binds the raw hash to a human-readable identifier, making the particle discoverable. unnamed particles are hard to find and rarely linked further. naming emerges from practical necessity, not protocol enforcement. further links weave the particle into the [[cybergraph]]. the accumulated graph of all cyberlinks IS [[knowledge]]

see [[cybergraph]] for the formal definition including all six axioms. see [[valence]] for the ternary epistemic field. see [[Bayesian Truth Serum]] for the scoring that uses $v$. see [[effective adjacency]] for how conviction weights enter the [[tri-kernel]]. see [[UTXO]] for the transaction model underlying conviction. see [[eternal cyberlinks]] for the permanent-premium variant. see [[knowledge economy]] for the full epistemic asset taxonomy

discover all [[concepts]]
