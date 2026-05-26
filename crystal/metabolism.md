---
tags: cyber, article, draft, research
alias: metabolism, metabolic signals, metabolic health, metabolic oracle, cap syntropy happiness
crystal-type: pattern
crystal-domain: cyber
crystal-size: bridge
---

the three signals that measure whether the [[cybergraph]] is alive — cap, [[syntropy]], [[happiness]] — and how they compound into a single health function the protocol optimizes

metabolism, in the biological sense, is the total chemical activity that sustains life: energy intake, waste removal, internal order maintenance, response to the environment. a living system without a metabolism is a crystal — static, ordered, unable to respond. the [[cybergraph]] has an equivalent: a set of measurable signals that distinguish growth from decay and feed back into the protocol's own parameter adaptation.

---

## the three signals

### cap: external validation

the total market capitalization of [[$CYB]] denominated in a reference unit (BTC, USD, energy equivalent).

cap reflects what the external world thinks the network produces. it integrates everything the protocol cannot observe internally: competing systems, regulatory shifts, speculative flows, actual utility demand. a rising cap means the environment rewards the network. a falling cap means the environment is penalizing it — or is indifferent.

cap as metabolic signal:
- rising cap → the environment values the network's output → parameters are working
- falling cap → the environment penalizes or ignores the network → recalibration needed
- cap relative to comparable protocols → comparative fitness signal

the critical property: cap cannot be gamed from inside the protocol. it originates outside the system boundary. any attempt to inflate it internally (token buybacks, artificial price supports) shows up immediately in the divergence between cap and the other two signals — the metabolic composite becomes incoherent, which the protocol detects and penalizes in its reward function.

### syntropy: internal order

$$J(\phi^*) = \log|V| + \sum_j \phi^*_j \log \phi^*_j$$

the information-theoretic structure of the [[focus]] distribution φ*. high syntropy means φ* is concentrated on a structured set of [[particles]] — the network has organized its [[attention]] into coherent [[knowledge]]. low syntropy means φ* is diffuse — the graph is noisy, unfocused, or spammed.

syntropy is computed every block from the current focus distribution. it requires no external input, no oracle, no participant vote. it is the graph's own objective measure of organizational quality.

syntropy as metabolic signal:
- rising syntropy → [[cyberlinks]] are creating structure → [[neurons]] contribute meaningful [[knowledge]]
- falling syntropy → noise outpaces signal → quality of the knowledge base is degrading
- syntropy growth rate → velocity of knowledge organization, independent of raw size

the failure mode: syntropy can be gamed by concentration. a cartel focusing all φ* on a small set of controlled [[particles]] produces high syntropy without genuine knowledge diversity. this is why syntropy alone is insufficient — it must be checked by cap (would a concentrated cartel actually raise external value?) and [[happiness]] (would participants served only cartel content report satisfaction?).

### happiness: subjective verification

a stake-weighted survey. each [[neuron]] privately submits a number from 0 (hell) to 100 (nirvana). the [[vimputer]] weights submissions by [[token]] stake to resist [[sybil attacks]] and outputs a global index.

happiness measures what cap and syntropy structurally cannot:
- cap reflects speculator expectations, not user experience
- syntropy measures information structure, not whether that structure serves participants
- happiness is direct self-report of whether the system is working for the people inside it

happiness as metabolic signal:
- high happiness → participants find the system useful, fair, and responsive
- low happiness → something is wrong that the other metrics cannot see
- happiness diverging upward from cap → internal utility exceeds external recognition (undervalued)
- happiness diverging downward from cap → speculative decoupling from real utility (overvalued)
- happiness diverging from syntropy → structure exists but does not serve the population

the failure mode: happiness is self-reported and stake-weighted, not independently verified. a wealthy cartel could report uniformly high happiness while the broader population suffers. the check: a cartel maximizing happiness signal would need to either improve real utility (which improves all three signals) or suppress non-cartel voices (which would reduce neuron diversity and eventually appear in syntropy and cap).

---

## the compound signal

no single metabolic factor is sufficient. together they compound:

$$M(t) = \text{cap}(t)^{w_c} \cdot J(t)^{w_s} \cdot H_{\text{happy}}(t)^{w_h}$$

where $w_c + w_s + w_h = 1$ are the metabolic weights and the geometric mean ensures that collapse in any single signal drags the entire composite down.

the metabolic derivative:

$$\dot{M}(t) = w_c \frac{\dot{\text{cap}}}{\text{cap}} + w_s \frac{\dot{J}}{J} + w_h \frac{\dot{H}_{\text{happy}}}{H_{\text{happy}}}$$

this is the growth rate of metabolic health — the primary reward signal for [[parametrization]] learning.

the metabolic weights $w_c, w_s, w_h$ are themselves governed, not learned. they encode the value judgment of what "health" means — how much to weight external validation vs internal order vs participant satisfaction. this is a normative choice that the protocol cannot make autonomously without circular reasoning. governance sets the weights; the RL agent optimizes within them.

---

## the metabolic oracle

a dedicated computation running alongside the [[tri-kernel]]:

```
every epoch:
  1. compute J(φ*) from current focus distribution
  2. read cap from on-chain oracle (IBC price feed or DEX TWAP)
  3. aggregate happiness from neuron submissions (stake-weighted median)
  4. compute M(t) = cap^w_c · J^w_s · H_happy^w_h
  5. compute ΔM = M(t) - M(t-1)
  6. feed ΔM to the parameter agent as reward
```

the oracle is deterministic: given the same graph state and oracle prices, every node computes the same M(t). this is required for [[consensus]] — the parameter agent must produce identical Δθ across the network.

---

## what metabolism is not

metabolism is not governance. it is not a vote on what the protocol should do. it is a measurement of how the protocol is performing — the equivalent of a patient's vital signs, not a prescription. the RL agent acts on the signal; it does not interpret it normatively.

metabolism is not a surveillance mechanism. happiness is submitted privately. the aggregate index is public; individual submissions are not. the protocol learns the population's health without learning which individual is unhappy.

metabolism is not sufficient for safety. a system optimizing M(t) could in principle find configurations that game all three signals simultaneously. the [[parametrization]] safety constraints — κ < 1 always, conservation, monotonicity, bounded change — are hard invariants that the metabolic optimizer cannot override.

see [[parametrization]] for how the RL agent uses ΔM. see [[syntropy]] for the information-theoretic formulation. see [[happiness]] for the stake-weighted survey mechanism. see [[functions of superintelligence]] for how metabolism integrates with the other autonomous capabilities.