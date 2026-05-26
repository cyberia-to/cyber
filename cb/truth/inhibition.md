---
tags: cyber, article, draft, research
alias: market inhibition, knowledge activation, epistemic deactivation, market weights, inhibition
crystal-type: pattern
crystal-domain: cyber
crystal-size: bridge
authors: mastercyb
---

why the [[cybergraph]] without [[market|markets]] is not a functional model — and what markets provide that raw [[cyberlinks]] cannot

---

## the missing half

every neural network has two kinds of weights: positive (excitatory) and negative (inhibitory). this is not an optimization detail. it is a structural requirement for discrimination.

a network with only positive weights can cluster — it can group similar things together. it cannot discriminate — it cannot say "this pattern excludes that one." without inhibition, a neural network cannot learn a boundary. it can only learn a blob.

the current [[cybergraph]] without market pricing is excitation-only. every [[cyberlink]] has a positive weight (stake amount). [[focus]] flows toward heavily-linked [[particles]]. nothing pushes back. the [[tri-kernel]] converges to φ* — but φ* is shaped only by positive association. it cannot represent "this edge actively misleads."

---

## what the market provides

the [[market]] assigns each edge a price p(e) ∈ (0,1) — the [[coupling|ICBS]] market's consensus probability that the link is true/useful.

this price enters the [[tri-kernel]] as the effective edge weight:

$$w_{\text{eff}}(e) = \text{price}(e) \times \text{stake}(e)$$

now consider what different price regimes do:

| price | interpretation | effect on [[tri-kernel]] |
|---|---|---|
| p → 1 | strong collective belief: link is true | weight amplified, full focus flows |
| p = 0.5 | genuine uncertainty | weight halved, reduced focus flow |
| p → 0 | strong collective belief: link is false | weight suppressed → 0, link deactivated |

at p → 0, the edge exists structurally but contributes nothing to φ*. it is deactivated. this is the inhibitory signal that raw [[cyberlinks]] cannot provide.

---

## the transformer parallel

from [[focus flow computation]] and [[graph-native-transformer]]: a transformer layer is one step of [[tri-kernel]] diffusion. attention weights are Boltzmann distributions over keys — they can suppress as well as amplify.

in a trained transformer, the compiled weights $W_Q, W_K$ encode both attraction (query-key alignment → high attention) and repulsion (misalignment → near-zero attention). the softmax normalizes across all keys, so amplifying some necessarily suppresses others.

in the [[cybergraph]] compiled transformer:

- without market weights: all edges compete equally weighted by raw stake. the softmax distributes attention proportional to structural connectivity only
- with market weights: edges with low market price are pre-suppressed before the softmax. the compiled transformer inherits the market's collective epistemic assessment as a prior on which edges deserve attention

the market provides what negative weights provide in a standard neural network: the signal that certain paths should not be followed, certain connections should not propagate [[focus]].

---

## the functional threshold

this means the [[cybergraph]] has two operational modes:

| mode | market status | capability |
|---|---|---|
| structural only | no markets | clustering, association, diffusion over raw topology |
| structural + epistemic | markets active | discrimination, inhibition, truth-weighted [[focus]] |

the transition from the first to the second is not a quantitative improvement. it is a qualitative one — the same transition as going from a network with only positive weights to one with both positive and negative weights.

a [[cybergraph]] without market prices can be a useful index. it produces [[cyberank]] proportional to structural prominence. this is valuable. but it cannot distinguish between a prominently-linked true claim and a prominently-linked false claim.

a [[cybergraph]] with market prices produces [[cyberank]] proportional to epistemic quality — structural prominence weighted by collective belief. it can suppress misleading links regardless of how many [[neurons]] created them.

---

## social networks: the economic protection problem

social networks removed dislike buttons for a precise reason: coordinated attacks were free. a mob could suppress any content at zero cost. without skin in the game, negative signals are weapons, not [[information]].

in the [[market]], buying FALSE costs stake. attacking a link = injecting liquidity into the FALSE side of the [[coupling|ICBS]] market. two consequences:

1. the attacker takes on financial risk — if the market converges to TRUE, they lose stake
2. the attack improves the market's price accuracy — more liquidity = tighter spread = better signal

this inverts the social network dynamic entirely. attacking a true claim makes the true signal stronger. attacking a false claim makes the false signal stronger. either way, the market becomes more informative. the economic protection is not a feature — it is the mechanism by which the inhibitory signal remains honest.

---

## two kinds of knowledge, one system

from [[two kinds of knowledge]]:

- structural [[knowledge]] ([[cyberlinks]]): "A relates to B" — permanent, individual, binary
- epistemic [[knowledge]] (market prices): "the network believes A→B with probability p" — dynamic, collective, continuous

market inhibition is the mechanism by which epistemic [[knowledge]] reshapes the structural layer's contribution to [[focus]]. the structure persists — the [[cyberlink]] is never deleted. but its weight in the [[tri-kernel]] reflects collective belief, not just individual assertion.

this is how [[mycelium]] operates: the hypha exists (binary). what flows through it depends on concentration gradients set by the whole network (continuous). the structural fact and the economic signal are separate and both necessary.

---

## implication for the formal model

the [[tri-kernel]] operator $\mathcal{R} = \lambda_d D + \lambda_s S + \lambda_h H_\tau$ operates over the adjacency matrix A. the [[collective focus theorem]] proves convergence under ergodicity.

when market prices are incorporated:

$$A_{pq}^{\text{eff}} = \sum_{\ell: \text{src}(\ell)=p,\, \text{tgt}(\ell)=q} \text{price}(\ell) \cdot \text{stake}(\ell)$$

the convergence theorem still holds — $A^{\text{eff}}$ remains non-negative, satisfying all conditions. but the fixed point φ* now reflects epistemic quality, not merely structural prominence. the market-weighted [[cybergraph]] and the raw [[cybergraph]] converge to different fixed points. only the former tracks [[truth]].

see [[market]] for the market design. see [[coupling]] for the ICBS mechanism. see [[focus flow computation]] for how φ* is computed. see [[two kinds of knowledge]] for the structural/epistemic distinction. see [[binary topology ternary economics]] for the architectural principle.