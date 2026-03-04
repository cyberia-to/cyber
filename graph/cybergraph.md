---
icon: 🕸
tags: cyber, core
alias: content oracle, cybergraphs
crystal-type: observed
crystal-domain: cyber
crystal-size: article
stake: 15224056096605018
---
the shared memory of the planet. every [[cyberlink]] ever created by every [[neuron]], accumulated into one authenticated [[graph]]

five primitives:

- [[particle]] — content-addressed node. identity = [[hash]] of content
- [[neuron]] — agent with stake and identity. creates links
- [[cyberlink]] — signed, staked, timestamped directed edge between two [[particles]]
- [[token]] — unit of [[value]]: [[coins]], [[cards]], [[scores]], [[badges]]
- [[focus]] — attention distribution over the graph. conserved: Σ = 1

a [[particle]] enters only with a [[cyberlink]]. the first link — a `~` [[name]] — turns it into a [[file]]. further links create [[knowledge]]. every [[cyberlink]] records [[who]], [[when]], and [[what]]

[[neurons]] write into the cybergraph. the [[tru]] reads from it and computes [[cyberank]], [[karma]], [[syntropy]]. the cybergraph is where both halves of [[intelligence]] meet

---

## formal definition

a cybergraph is a 4-tuple:

$$\mathbb{G} = (P,\; N,\; T,\; L)$$

| symbol | name | definition |
|---|---|---|
| $P$ | particles | content-addressed objects. $p \in P$ iff $p = H(\text{content})$ |
| $N$ | neurons | authenticated agents. $\nu \in N$ iff $\nu = H(\text{pubkey})$ |
| $T$ | tokens | finite set of protocol-native denominations |
| $L$ | cyberlinks | set of authenticated directed assertions |

### the cyberlink record

each cyberlink $\ell \in L$ is a 7-tuple:

$$\ell = (\nu,\; p,\; q,\; \tau,\; a,\; v,\; t)$$

| field | type | layer | semantics |
|---|---|---|---|
| $\nu$ | $N$ | structural | signing neuron — who asserts |
| $p$ | $P$ | structural | source particle — from |
| $q$ | $P$ | structural | target particle — to |
| $\tau$ | $T$ | economic | token denomination — in what |
| $a$ | $\mathbb{R}_+$ | economic | amount staked — how much |
| $v$ | $\{-1, 0, +1\}$ | epistemic | BTS meta-prediction — predicting the collective's assessment |
| $t$ | $\mathbb{Z}$ | temporal | block timestamp — when |

the three layers correspond to the [[two three paradox]]:

- structural $(\nu, p, q)$: binary — the connection either exists or it doesn't
- epistemic $v$: ternary — the neuron's [[Bayesian Truth Serum|BTS]] meta-prediction
- economic $(τ, a)$: continuous — stake magnitude across the full range $\mathbb{R}_+$

### semantics of $v$

$v$ is not an assertion about whether the connection is true. it is the neuron's prediction of how the [[inversely coupled bonding surface|ICBS]] market on this edge will converge:

| $v$ | prediction |
|---|---|
| $+1$ | market will converge to high price — others will affirm this connection |
| $0$ | market will be uncertain — genuine epistemic ambiguity |
| $-1$ | market will converge to low price — others will initially reject this connection |

creating a link with $v = -1$ is the contrarian signal: "I have private [[knowledge]] the collective hasn't priced yet." the [[Bayesian Truth Serum]] mechanism rewards exactly this when correct — beliefs that exceed their predicted popularity.

the cyberlink is therefore the [[Bayesian Truth Serum|BTS]] input in a single atomic act:
- link creation + stake $(\tau, a)$ = first-order belief $p_i$: the connection is meaningful
- valence $v$ = meta-prediction $m_i$: how the collective will assess it

### homoiconicity

$$H(L) \subseteq P$$

every cyberlink induces a particle via content-addressing. $\ell \in L$ implies $H(\ell) \in P$. links and particles share the same type — both are content-addressed objects. this is not a design choice but a consequence of the content-addressing axiom.

### derived objects

from $L$, the [[tru]] computes:

$$A_{pq} = \sum_{\ell:\, \nu(\ell)=p,\, \text{tgt}(\ell)=q} \text{rate}(\tau(\ell)) \cdot a(\ell)$$

the raw structural adjacency. the [[tri-kernel]] runs over this to produce π*.

with the epistemic layer active (markets running), the effective adjacency is:

$$A^{\text{eff}}_{pq} = \sum_{\ell:\, \text{src}(\ell)=p,\, \text{tgt}(\ell)=q} \underbrace{a(\ell)}_{\text{stake}} \times \underbrace{\text{trust}(\nu(\ell))}_{\text{karma}} \times \underbrace{f(\text{ICBS price}(\ell))}_{\text{market belief}}$$

where $f(\text{price}) \in [0, 1]$ maps the ICBS reserve ratio to a weight multiplier. edges the collective disbelieves are suppressed toward zero. this is [[market inhibition]] — the inhibitory signal that makes the [[cybergraph]] computationally equivalent to a neural network with both excitation and inhibition.

---

see [[cybergraph/architecture]] for namespace structure and implementation. see [[collective focus theorem]] for convergence proof over $A^{\text{eff}}$. see [[two kinds of knowledge]] for the structural/epistemic split. see [[Bayesian Truth Serum]] for the scoring mechanism. see [[inversely coupled bonding surface]] for the market substrate. see [[focus flow computation]] for how π* is computed from both layers.

discover all [[concepts]]
