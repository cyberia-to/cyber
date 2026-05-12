---
tags: cyber, cybics, article, draft, research
alias: forgetting, graph forgetting, synaptic pruning, selective forgetting, memory pruning
crystal-type: pattern
crystal-domain: cyber
crystal-size: bridge
---

the selective removal of weak connections from active computation while preserving the authenticated record — the cybergraph's equivalent of sleep-phase synaptic homeostasis

forgetting is essential. a system that remembers everything equally is a system that can extract nothing. signal requires noise suppression. memory requires forgetting.

---

## the biological model

during sleep, the brain executes synaptic homeostasis: synapses strengthened during waking activity are globally downscaled. weak synapses — those that were not repeatedly activated — are pruned. strong ones are reinforced. the result is a more efficient, lower-noise representation of what was learned.

the brain does not delete experience. it compresses it. the authenticated record of what happened is retained in the pattern of strengthened connections. the noise — the weakly-activated, one-off, low-signal synapses — is discarded. space is reclaimed. signal-to-noise ratio improves.

this is not pathological forgetting. it is structural maintenance. a brain that never pruned would saturate its synaptic capacity in hours. biological memory is capacity-limited and forgetting is the management mechanism.

---

## the cybergraph problem

the [[cybergraph]] is permanently append-only. every [[cyberlink]] ever created is structurally present in the authenticated record. there is no native expiration, no central authority to delete stale content, no automatic garbage collection.

at $10^{15}$ [[particles]] and $10^{10}$ [[neurons]], the graph grows without bound. the space complexity problem is real.

three distinct failure modes if forgetting is absent:

saturation. active computation (the [[tri-kernel]]) must eventually exclude some links. at planetary scale, no machine can hold all links in working memory simultaneously. the graph must have a hot tier (active) and cold tier (archived), and the hot tier must be bounded.

staleness noise. a [[cyberlink]] from five years ago asserting "X is the best Y" adds noise when X is no longer best. the market suppresses this if participants update their positions. but the market lags: low-traffic edges may stay at stale prices for years. uncorrected staleness degrades the signal quality of φ*.

attention dilution. as the graph grows, [[cyberank]] and [[focus]] distribution φ* are computed over an ever-growing graph. [[particles]] from years ago compete for focus with current signal. the effective resolution of attention decreases.

---

## what forgetting is — and what it is not

forgetting in the cybergraph means: removing a [[cyberlink]] from active [[tri-kernel]] computation. its authenticated record remains. it is queryable. it has provenance. it is simply excluded from the working set that shapes φ*.

forgetting is not:
- deleting content from the permanent record
- invalidating a [[neuron]]'s historical assertion
- removing a [[particle]] from the content-addressed store
- reversing the [[stark]] proof that authenticated the link

the permanent record and the active working set are separate concerns. the cybergraph never deletes. it selectively pays attention.

---

## three forgetting mechanisms

### market forgetting

the [[inversely coupled bonding surface|ICBS]] market is the most natural forgetting mechanism. a link whose market price converges to zero has near-zero effective weight in the [[tri-kernel]]:

$$w_\text{eff}(e) = \text{stake}(e) \times \text{trust}(\nu_e) \times f(\text{ICBS price}(e))$$

when $f(\text{price}) \to 0$, the link is computationally deactivated regardless of structural existence. this is the epistemic layer's forgetting mechanism: the market collectively decides what to stop attending to.

limitation: market forgetting requires active market participation. low-traffic, low-interest edges may never attract enough participation to suppress stale content. markets lag reality.

### conviction withdrawal

a [[cyberlink]]'s conviction is a [[UTXO]] — the [[neuron]] can spend it back to their wallet at any time. withdrawing conviction removes the economic weight from the link. the structural record stays in $L$ permanently, but without conviction it contributes nothing to the [[tri-kernel]]

a [[neuron]] who withdraws conviction from old links is forgetting — reallocating capital to new assertions. the graph forgets proportional to the [[neuron]]'s evolving conviction

see [[cyber/link]] for the conviction UTXO mechanics

### archival sweep

during the slow timescale of the [[focus flow computation]] two-timescale separation (~hours), the [[tru]] sweeps for links meeting archival criteria:

| criterion | threshold |
|---|---|
| stake | $< \epsilon_s$ for $N$ consecutive epochs |
| ICBS price | $< \epsilon_p$ for $N$ consecutive epochs |
| traversal traffic | zero [[cyberank]] flow for $N$ epochs |

links meeting all criteria move from hot (active computation) to cold (archived record). this is the sleep-phase compression pass.

archived links can be reactivated: the [[neuron]] restakes, or market activity resumes, or traffic returns. reactivation restores hot-tier status.

---

## temporal decay

staleness is a harder problem than spam. spam is cheap-to-create noise; the market suppresses it economically. staleness is high-quality signal that has aged past its relevance.

temporal decay addresses this: link weight decreases with age unless explicitly refreshed:

$$w(t, \ell) = \text{stake}(\ell) \cdot e^{-\lambda(t - t_\ell)}$$

the decay rate $\lambda$ should be per-domain. mathematics: $\lambda = 0$ (theorems don't expire). current events: $\lambda$ calibrated to domain half-life. technology: fast decay. history: slow decay.

this is design open space. the right $\lambda$ values require empirical calibration from live graph data.

---

## the two-tier architecture

| tier | contents | included in tri-kernel | retention |
|---|---|---|---|
| hot | links with meaningful stake, price, or traffic | yes | current epoch |
| cold | authenticated historical record | no | permanent |

the hot tier is the brain's active working memory. the cold tier is long-term storage. the [[tru]] manages the boundary between them.

---

## forgetting and [[knowledge completeness]]

forgetting creates a tension with [[knowledge completeness]]: the cybergraph aspires to preserve all [[knowledge]], but active forgetting removes links from the working set. the resolution: the authenticated record preserves the epistemic claim. forgetting removes it from active inference, not from the historical fact base.

a [[neuron]] researching historical context can access cold-tier links. the cybergraph's memory is complete; its current attention is selective. this is the correct architecture for both completeness and efficiency.

see [[stake dynamics]] for how stake mobility works without proof resubmission. see [[market inhibition]] for how market prices suppress links. see [[focus flow computation]] for the two-timescale separation. see [[knowledge completeness]] for the completeness/efficiency tension.