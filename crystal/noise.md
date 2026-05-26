---
tags: cyber, core
alias: graph noise, signal noise, noise floor, noise producer
crystal-type: measure
crystal-domain: cyber
crystal-size: bridge
---
anything that moves [[focus]] distribution $\phi^*$ toward uniform. noise is the complement of [[syntropy]]

---

## the formal definition

the [[cybergraph]]'s total informational capacity over $|P|$ particles is $\log|P|$ bits. that capacity is always partitioned:

$$\underbrace{J(\phi^*)}_{\text{syntropy}} + \underbrace{H(\phi^*)}_{\text{noise}} = \log|P|$$

syntropy $J(\phi^*) = D_{KL}(\phi^* \| u)$ is the organized fraction — how far the focus distribution has been structured above random. noise $H(\phi^*) = -\sum_p \phi^*_p \log \phi^*_p$ is the remaining Shannon entropy — the fraction of capacity that carries no distinguishing structure.

at maximum noise: $\phi^* = u$ (uniform) — all particles equally attended, $J = 0$, $H = \log|P|$. no signal. at maximum syntropy: $\phi^*$ is a point mass — all attention on one particle, $J = \log|P|$, $H = 0$.

every [[cyberlink]] shifts $\phi^*$ in some direction. a link moves the graph toward lower entropy (signal) or higher entropy (noise). there is no neutral link — every addition has a sign.

## a cyberlink is noise if

its addition to $L$ decreases $J(\phi^*)$, equivalently increases $H(\phi^*)$. in [[Bayesian Truth Serum|BTS]] terms: the neuron's score $s_i < 0$ — the link added more uncertainty to the collective picture than it removed. [[karma]] accumulates these negative contributions and reduces the neuron's future influence in [[effective adjacency]].

noise links share a characteristic: they do not reflect genuine private [[knowledge]]. they assert connections the author does not actually believe, or connections that are randomly true, or connections that were once true but no longer are. the common denominator is that the epistemic signal $v$ (valence) is not a reliable predictor of where the [[inversely coupled bonding surface|ICBS]] market will settle.

## four kinds of noise

structural noise — spam. low-conviction links created cheaply at volume. the [[conviction]] cost ($\tau$, $a$) is the first defense: cheap talk is economically suppressed. but a large-stake actor can create structural noise profitably if they can manipulate φ* faster than the market can correct.

epistemic noise — false assertion. a link that is confidently wrong: high conviction, but the market settles against it ($m(\ell) \to 0$). [[market inhibition]] suppresses the edge weight toward zero in $A^{\text{eff}}$, but the structural record remains in $L$. the link persists as history; its influence on $\phi^*$ is reduced to near zero.

staleness noise — temporal decay. a link that was once signal and became noise as reality changed. the assertion was true at block $t$; at block $t'  \gg t$ it no longer is. the [[inversely coupled bonding surface|ICBS]] market may not update if no one trades the edge — low-traffic links can stay at stale prices for years. [[forgetting]] addresses this by decaying low-activation links out of active computation.

dilution noise — scale effect. as $|P|$ grows, the denominator of $\phi^*$ grows. a fixed amount of structure (fixed number of organized edges) produces less syntropy over a larger graph. the noise floor rises with graph size unless the rate of signal creation exceeds the rate of particle growth.

## the noise floor

the noise floor is the uniform distribution $u$ — the baseline state before any cyberlinks. a new graph with no links has $\phi^* = u$, $J = 0$. a mature graph with high syntropy has $J \gg 0$. the distance from the noise floor is the graph's total informational achievement.

noise floor in practice: even a fully mature graph will have a noise floor above $u$ because $|P|$ includes particles that were never linked beyond their initial [[name]], particles on contested topics with divided market prices, and particles that are genuinely ambiguous. the effective noise floor is set by the graph's least-resolved regions.

## the anti-noise stack

four mechanisms suppress noise at four timescales:

| mechanism | operates on | timescale | suppresses |
|-----------|------------|-----------|------------|
| conviction ($\tau$, $a$) | link creation | instant | cheap spam |
| [[Bayesian Truth Serum\|BTS]] + [[karma]] | neuron reputation | epoch | persistent noise producers |
| [[market inhibition]] | edge weights | continuous | false assertions |
| [[forgetting]] | active computation | long-term | staleness noise |

no single mechanism is sufficient. spam resistance requires conviction cost. false assertion resistance requires market inhibition. staleness resistance requires forgetting. the stack works because each kind of noise has a different structure and requires a different filter.

see [[syntropy]] for the information measure noise reduces. see [[forgetting]] for the primary long-term noise filter. see [[market inhibition]] for suppression of false assertions in $A^{\text{eff}}$. see [[Bayesian Truth Serum]] for the neuron-level noise scoring.

discover all [[concepts]]
