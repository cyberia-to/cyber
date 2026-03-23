---
tags: cyber, core, cybernomics
crystal-type: process
crystal-domain: cyber
---

the suppressive signal in the [[cybergraph]] where [[ICBS]] markets reduce the effective weight of disbelieved [[cyberlinks]]. analogous to inhibitory synapses in biological neural networks

## mechanism

in a standard graph, every [[cyberlink]] contributes positively to [[attention]] flow. market inhibition introduces a negative channel: when a prediction market on a link's validity resolves against the link, the market outcome scales down its weight

$$w_{\text{eff}}(\ell) \;=\; w(\ell) \cdot \bigl(1 - \alpha \cdot m(\ell)\bigr)$$

where $w(\ell)$ is the original stake-weighted strength, $m(\ell) \in [0, 1]$ is the market's disbelief signal, and $\alpha$ is the inhibition coefficient

## computational equivalence

excitation alone produces a directed weighted graph. adding inhibition makes the [[cybergraph]] computationally equivalent to a neural network:

| biological | cyber |
|---|---|
| excitatory synapse | staked [[cyberlink]] with positive weight |
| inhibitory synapse | market-suppressed [[cyberlink]] |
| neurotransmitter balance | stake vs. disbelief ratio |

the [[tri-kernel]] processes both signals simultaneously: [[diffusion]] spreads excitation, while market inhibition dampens unreliable paths

## economic dynamics

inhibition carries a cost. a [[neuron]] that inhibits a link must stake into the [[ICBS]] market against it. if the link turns out to be valid, the inhibitor loses stake. this symmetry ensures that both belief and disbelief are costly — cheap talk in either direction is eliminated

see [[cyberlinks]], [[cybergraph]], [[ICBS]], [[tri-kernel]], [[attention]], [[tru]]
