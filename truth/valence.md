---
tags: cyber, core
alias: valences, epistemic valence, link valence, v field, ternary signal, valence
crystal-type: entity
crystal-domain: cyber
crystal-size: bridge
---
the ternary epistemic field of a [[cyberlink]]. $v \in \{-1,\, 0,\, +1\}$

| value | name | meaning |
|-------|------|---------|
| $+1$ | affirmative | the neuron affirms the link and predicts the [[inversely coupled bonding surface\|ICBS]] market on this edge will converge toward TRUE |
| $\phantom{+}0$ | uncertain | the neuron has no confident prediction; the link exists but the epistemic signal is withheld |
| $-1$ | negating | the neuron affirms the link exists and predicts the market will converge toward FALSE |

valence is the ternary layer sitting between binary topology and continuous [[inversely coupled bonding surface|ICBS]] price discovery. it is the coarse human-readable quantization of belief: the three-state summary before the market produces a continuous probability.

## what valence is

a [[cyberlink]] always creates a structural fact — two [[particles]] are connected. that connection is binary: it exists or it does not. valence is not about whether to create the link. it is the neuron's meta-prediction, provided at the moment of link creation, about where the market on that edge will eventually settle.

$v$ is the input to [[Bayesian Truth Serum]] scoring. it is $m_i$ in the BTS formula — the neuron's prediction of what the collective will come to believe, before the collective has spoken. BTS rewards $v$ when the neuron's prediction proves accurate relative to outcomes and the predictions of others who had worse private [[knowledge]].

this means $v = -1$ is not contradiction. a neuron can create a link (asserting structural connection) while predicting the link will be judged false by the network. this is rational when the neuron knows something the market has not yet priced, or when the neuron deliberately adds anti-[[knowledge]] to the graph for others to refute. [[Bayesian Truth Serum]] rewards this exactly when correct.

## the three states in the cybergraph

valence maps directly onto the [[binary topology ternary economics]] architecture observed in mycorrhizal networks, neural synapses, and markets:

| domain | +1 | 0 | -1 |
|--------|----|---|----|
| [[cybergraph]] | affirm: market → TRUE | uncertain: no prediction | negate: market → FALSE |
| neurobiology | excitatory synapse | neuromodulation | inhibitory synapse |
| [[mycelium]] | give resources | maintain channel | receive / take |
| market | buy TRUE | hold | buy FALSE |

the zero state carries information even when it carries no directional belief. a neutral-valence link holds the structural channel open — the connection exists, the topic has been raised — without forcing an epistemic commitment. signaling molecules flow through zero-valence links in the same way neuromodulators flow through neutral synapses.

## valence in the formal record

each $\ell \in L$ is a 5-tuple:

$$\ell = (p,\; q,\; \tau,\; a,\; v) \;\in\; P \times P \times \mathcal{T} \times \mathbb{R}_+ \times \{-1,0,+1\}$$

$v$ is at position five. it is fixed at link creation — immutable once signed into the append-only record. authorship $\nu$ and block height $t$ belong to the containing [[signal]]. the [[inversely coupled bonding surface|ICBS]] market price $m(\ell) \in (0,1)$ that emerges afterward is the continuous refinement of what $v$ anticipated as a coarse signal.

## effect on the graph computation

valence seeds the market that weights edges in effective adjacency:

$$A^{\text{eff}}_{pq} = \sum_{\substack{\ell \in L \\ \text{src}(\ell)=p,\;\text{tgt}(\ell)=q}} a(\ell)\cdot\kappa(\nu(\ell))\cdot f(m(\ell))$$

where $m(\ell)$ is the ICBS reserve ratio (market-implied probability the edge is valid) and $f: [0,1] \to [0,1]$ maps market price to a weight multiplier. edges the collective disbelieves converge toward $m \approx 0$, so $f(m) \approx 0$ — they are suppressed in the [[tri-kernel]] computation without being deleted from the structural record. this is [[market inhibition]]: the graph-theoretic analog of inhibitory synaptic transmission.

valence is what makes the [[cybergraph]] computationally equivalent to a neural network with both excitation and inhibition. without negative valence, $A^{\text{eff}}$ is purely excitatory — the graph can only reinforce, never suppress. with $v = -1$ positions and market consensus, false or misleading links are dynamically downweighted while structurally persisting in the provenance record.

## connection to [[syntropy]]

the aggregate of valence-seeded market prices, once resolved toward collective consensus, raises [[syntropy]] when predictors are accurate ($J(\phi^*) = D_{KL}(\phi^* \| u)$ increases when $\phi^*$ sharpens around true structure) and lowers it when markets remain uncertain or divided. a neuron whose $v$ predictions proved correct contributed positive BTS score $s_i$ — that neuron increased the graph's organizational quality.

see [[Bayesian Truth Serum]] for the BTS scoring formula that uses $v$ as input. see [[inversely coupled bonding surface]] for the market that converts valence seeds into continuous prices. see [[market inhibition]] for the suppression mechanism. see [[two three paradox]] for why 3 is irreducible to 2. see [[two kinds of knowledge]] for the structural / epistemic split the valence field bridges.

discover all [[concepts]]
