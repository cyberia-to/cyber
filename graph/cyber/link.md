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

## naming the fields

each of the seven fields answers a fundamental question. [[who]] — $\nu$, the author. [[what]] — $p$ and $q$, the source and target particles. [[when]] — $t$, the block height. denomination ($\tau$) and amount ($a$) form [[conviction]] — the pair that prices the assertion. [[valence]] ($v$) is the epistemic prediction. see [[cybergraph]] for the full formal definition, type signatures, and six axioms

## conviction as UTXO

conviction is not a label attached to a link — it is a [[UTXO]]. creating a cyberlink is a transaction: the author moves $a$ tokens of denomination $\tau$ from a wallet UTXO to a new output bound to the cyberlink record. funds always move from one object to another. you cannot stake what you do not own.

the conviction output can itself be spent:

- transfer: spend the conviction UTXO to a new owner. the structural record stays in $L$; beneficial ownership moves. this is how the card's transferability operates at the protocol level
- withdraw: spend the conviction UTXO back to the author's wallet. the economic position closes. the structural record remains

the non-fungibility of the card (unique 7-tuple) and the fungibility of the token (transferable UTXO) coexist: the assertion is non-fungible, the economic position is a standard UTXO output

## CRUD in the graph

the append-only graph expresses all four operations through cyberlinks:

| operation | cyberlink action | what changes |
|-----------|-----------------|--------------|
| create | first record for structural triple $(\nu, p, q)$ | relation enters $L$ |
| read | query $\pi^*$ at any block — no link required | nothing |
| update | new record: spend old conviction UTXO, submit new $(\tau, a, v, t)$ | economic position + epistemic signal |
| delete | spend conviction UTXO + submit $v = -1$ | position closed, signal negated |

three dimensions vary independently across successive records for the same structural triple:

| structural | epistemic ($v$) | economic ($a$) | reading |
|-----------|-----------------|----------------|---------|
| exists | $+1$ | high | funded affirmation |
| exists | $-1$ | high | funded short |
| exists | $-1$ | zero | logical retraction |
| exists | $0$ | zero | bare assertion |

delete in the graph is not erasure. the record $(\nu, p, q, t_{\text{first}})$ remains in $L$ permanently. what closes is the economic position; what updates is the epistemic signal. the structural fact that an assertion was made at block $t$ is immutable. accumulated cyberlink history IS the semantic content of a relation

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

the protocol accepts any cyberlink as the first to a particle — there is no enforcement of what that first link must be. by convention, a [[name]] link is typically the first: it binds the raw hash to a human-readable identifier, making the particle discoverable. unnamed particles are hard to find and rarely linked further. naming emerges from practical necessity, not protocol enforcement. further links weave the particle into the [[cybergraph]]. the accumulated graph of all cyberlinks IS [[knowledge]]

see [[cybergraph]] for the formal definition including all six axioms. see [[valence]] for the ternary epistemic field. see [[Bayesian Truth Serum]] for the scoring that uses $v$. see [[effective adjacency]] for how conviction weights enter the [[tri-kernel]]. see [[UTXO]] for the transaction model underlying conviction. see [[eternal cyberlinks]] for the permanent-premium variant. see [[knowledge economy]] for the full epistemic asset taxonomy

discover all [[concepts]]
