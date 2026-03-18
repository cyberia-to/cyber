---
tags: cyber, article, draft, research
alias: stake dynamics, cyberlink lifecycle, stake mobility, dynamic stake, link weight dynamics
crystal-type: pattern
crystal-domain: cyber
crystal-size: enzyme
---

how staked [[tokens]] on a [[cyberlink]] interact with token mobility — the mechanics of link weight as a live signal rather than a snapshot

the core question: if a [[neuron]] creates a [[cyberlink]] with staked tokens and later moves those tokens, does the link weight update automatically, or must the [[neuron]] resubmit a proof?

---

## the design choice

two approaches:

locked stake. tokens staked on a [[cyberlink]] are locked for the link's lifetime. moving them requires closing the link. the proof of stake is a one-time authenticated commitment. high-conviction but inflexible — a neuron cannot reallocate capital without destroying their links.

dynamic stake. link weight is computed from the [[neuron]]'s current balance at each block. no locking required. moving tokens automatically adjusts link weight. the link persists; its weight floats with current conviction.

the simplest solution is dynamic stake. it requires no lock mechanism, no resubmission, no special proof — just a balance query at weight-computation time.

---

## the dynamic stake formula

$$A_{pq}(\ell) = \text{rate}(\tau(\ell)) \cdot \text{balance}(\nu(\ell), \tau(\ell), t)$$

where:
- $\tau(\ell)$ is the token denomination specified in the [[cyberlink]] record
- $\nu(\ell)$ is the signing [[neuron]]
- $\text{balance}(\nu, \tau, t)$ is the [[neuron]]'s current unlocked balance at block $t$
- $\text{rate}(\tau)$ converts the denomination to a common unit

weight is recomputed each block. no proof required for weight changes. the authenticated [[cyberlink]] record is immutable; only the effective weight floats.

---

## what this means for neurons

sustained influence requires sustained capital. a [[neuron]] who creates a link and moves all their tokens sees the link's effective weight drop to zero. the link exists in the authenticated record — it retains its historical provenance — but contributes nothing to π* until the [[neuron]] restakes.

this is the correct incentive structure: influence in the [[cybergraph]] should reflect ongoing conviction, not past commitments. a link created five years ago by a [[neuron]] who no longer holds tokens should not continue to shape focus.

capital reallocation is cheap. a [[neuron]] can create many links and redistribute stake across them freely. the [[tri-kernel]] sees the current allocation at every block. portfolio management of [[cyberlinks]] is as natural as portfolio management of assets.

---

## the blocking proof option

a [[neuron]] who wants to signal permanent, immovable conviction has an optional mechanism: a blocking proof that prevents spending the staked tokens.

technically: the [[neuron]] creates a time-locked UTXO or a commitment that provably cannot be spent for a defined duration. the lock is verifiable by the [[tru]] — no trust required. the link weight for locked tokens persists regardless of the [[neuron]]'s other balance movements.

use case: foundational claims where the [[neuron]] wants to signal long-term epistemic commitment — "I am so certain this link is correct that I am willing to lock capital against it for N years." this is costly signaling in the game-theoretic sense: the cost is the opportunity cost of locked capital.

this is not in the base protocol. it is an optional extension. the base protocol is simpler: all stake is dynamic. blocking proofs can be introduced as a higher-order feature once the base mechanics are proven stable.

---

## market positions vs link stake

the [[cyberlink]] record encodes two distinct economic facts:

link stake $(τ, a)$: the [[neuron]]'s structural assertion — "this connection is meaningful and I back it with capital." this is the first-order [[belief]] input to [[Bayesian Truth Serum]].

[[inversely coupled bonding surface|ICBS]] market position: the [[neuron]]'s (and all other participants') continuous trading on the edge's validity. this is the collective epistemic layer.

these are independent. a [[neuron]] can create a link with high stake but sell their ICBS YES position — they assert the connection structurally but bet against collective validation. the two signals measure different things.

under dynamic stake, moving tokens affects link weight (structural layer). buying/selling ICBS positions affects market price (epistemic layer). the two layers are decoupled in their update mechanics.

---

## implications for [[forgetting]]

dynamic stake is one of the three [[forgetting]] mechanisms. when a [[neuron]] reallocates capital away from old links, those links naturally lose weight. the graph forgets proportional to the [[neuron]]'s evolving conviction.

this is not the same as market forgetting (price → 0) or archival sweep (hot → cold). stake decay is the individual [[neuron]]'s forgetting. market forgetting is collective epistemic assessment. archival is the system's space management. all three operate independently.

the combination: a link that a [[neuron]] has abandoned (stake → 0) AND the market has suppressed (price → 0) AND that has attracted no traffic (cyberank → 0) for N epochs — that link is a strong candidate for archival. each mechanism contributes evidence that the link is no longer serving active inference.

---

## the cyberlink lifecycle

```
creation → active (hot tier, weighted by current stake × market price × karma)
         ↕ stake floats with neuron's balance
         ↕ market price floats with ICBS trades
         ↓
deactivated (stake ≈ 0 OR price ≈ 0 for N epochs)
         ↓
archived (cold tier, permanent record, excluded from tri-kernel)
         ↑ reactivation possible (restake + market activity resumes)
```

the link record is immutable at every stage. what changes is its inclusion in active computation.

see [[forgetting]] for the full forgetting mechanism. see [[cybergraph]] for the formal record definition. see [[cyberlink market protocol]] for the market layer. see [[Bayesian Truth Serum]] for the scoring implications of stake as first-order belief.
