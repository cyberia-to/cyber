---
tags: cyber, cybernomics, cip
crystal-type: entity
crystal-domain: cybernetics
crystal-size: article
alias: cyber netics, cybernetics protocol
stake: 50000000000000000
---
# cyber netics

the [[cyber]] protocol described as a control system — inputs, outputs, feedback loops, attractors, stability conditions. [[cyber/tokens]] are the nouns, [[cyber/nomics]] are the verbs, netics is the whole machine seen from the outside as a governor

## the primary loop

```
neuron creates cyberlink (input)
    ↓
tri-kernel recomputes focus (process)
    ↓
cyberank updates per particle (output)
    ↓
neuron observes new ranking (feedback)
    ↓
neuron adjusts linking strategy (adaptation)
    ↓
neuron creates cyberlink ...
```

this is the [[observation]] loop described in [[implicit knowledge]]: the fundamental cycle that sustains [[intelligence]]. every revolution of the loop adds [[knowledge]] to the [[cybergraph]] and refines what the system attends to

the loop is self-reinforcing: better [[knowledge]] → sharper [[focus]] → higher [[karma]] for accurate [[neurons]] → more [[attention]] weight on their future links → better [[knowledge]]

## inputs

| Input | Source | What it carries |
|-------|--------|----------------|
| [[cyberlink]] | [[neuron]] | structural assertion: "from relates to to" |
| [[will]] (lock) | [[neuron]] | economic commitment: conviction depth |
| [[attention]] allocation | [[neuron]] | fine-tuned weight distribution |
| ICBS trade | [[neuron]] | epistemic market signal: belief in link validity |
| [[valence]] | [[neuron]] | meta-prediction: BTS honesty signal |

every input is a [[costly signal]] — it costs [[will]] to produce, ensuring the system accumulates weighted commitments rather than noise

## process

the [[tri-kernel]] — the only computation that runs in [[consensus]]:

$$\phi^{(t+1)} = \text{norm}\big[\lambda_d \cdot D(\phi^t) + \lambda_s \cdot S(\phi^t) + \lambda_h \cdot H_\tau(\phi^t)\big]$$

three operators, each providing a distinct search mode:

| Operator | Force | What it does |
|----------|-------|-------------|
| [[diffusion]] | exploration | random walk — where does probability flow? |
| [[springs]] | structure | screened [[Laplacian]] — what satisfies constraints? |
| [[heat]] | adaptation | heat kernel — what does the graph look like at scale τ? |

the [[collective focus theorem]] guarantees convergence to a unique fixed point π*. the process is deterministic, verifiable, and local (h-hop neighborhood suffices)

## outputs

| Output | Per-what | What it means |
|--------|----------|--------------|
| [[focus]] | [[particle]] | collective [[attention]] distribution π |
| [[cyberank]] / [[prob]] | [[particle]] | probability of observation at fixed point |
| [[relevance]] | [[particle]] × context | local reconvergence given query |
| [[karma]] | [[neuron]] | accumulated trust from contribution |
| [[value]] | [[particle]] | [[prob]] × market cap |
| [[syntropy]] | system | coherence in bits — order above noise |

## feedback loops

### the learning loop (fast, per-block)

```
neuron links → Δπ > 0 → reward minted → neuron gains $CYB
    → more will → more attention capacity → more links
```

positive feedback: accurate contributions compound. the unit of wealth is epistemic accuracy

### the reputation loop (medium, per-epoch)

```
accurate links → high karma → more adjacency weight per link
    → earlier Δπ attribution → more reward per contribution
    → resources to stake on next insight
```

[[karma]] is the flywheel: it cannot be bought, only earned by being right before the crowd

### the market loop (continuous)

```
ICBS price diverges from structural signal
    → protocol (or informed neurons) trade toward correction
    → price converges → effective adjacency improves
    → tri-kernel inference improves → better structural signal
```

[[epistemic markets|ICBS markets]] create an inhibitory channel: incorrect links get suppressed economically, not just structurally

### the metabolic loop (slow, per-era)

```
cap signal + syntropy + happiness
    → parametrization PID adjusts α, β, τ, thresholds
    → system behavior shifts
    → new cap, syntropy, happiness measurements
```

[[cyber/parametrization]] closes the slowest loop: the protocol tunes itself

## attractors

the system has one global attractor: the [[free energy]] minimum

$$\mathcal{F}(\phi) = \lambda_s\left[\frac{1}{2}\phi^\top L\phi + \frac{\mu}{2}\|\phi-x_0\|^2\right] + \lambda_h\left[\frac{1}{2}\|\phi-H_\tau\phi\|^2\right] + \lambda_d \cdot D_{KL}(\phi \| D\phi) - T \cdot S(\phi)$$

at the minimum: $\phi^*_i \propto \exp(-\beta[E_{\text{spring},i} + \lambda E_{\text{diff},i} + \gamma C_i])$ — a [[Boltzmann distribution]]. the same form that governs physical equilibrium, biological homeostasis, and market clearing

## stability conditions

convergence guaranteed when the composite contraction coefficient κ < 1 (Banach fixed-point theorem). the [[collective focus theorem]] proves this holds for the [[tri-kernel]]

three independent stability mechanisms:

| Mechanism | What it prevents | How |
|-----------|-----------------|-----|
| [[focus]] conservation | inflation of attention | π sums to 1, enforced by normalization |
| [[costly signal]] via [[will]] | spam, cheap assertions | every link costs locked capital |
| [[market inhibition]] via ICBS | false claims persisting | collective betting suppresses incorrect edges |

## phase transitions

as the [[cybergraph]] grows, it passes through qualitative transitions:

| Phase | Condition | Character |
|-------|-----------|-----------|
| seed | few particles, sparse links | individual assertions dominate |
| flow | λ_d dominant | diffusion explores, network discovers structure |
| cognition | λ_s rises | springs enforce consistency, hierarchy emerges |
| reasoning | λ_h activates | heat kernel enables multi-scale context |
| consciousness | dynamic blend | all three operators in adaptive balance |

the transition threshold: $|P^*| \sim \rho^2$ where ρ is mean connectivity. below threshold the graph is molecular (disconnected islands). above it, thermodynamic (globally connected, emergent properties)

## the compound effect

[[cyber/tokens]] define what exists. [[cyber/nomics]] defines how it moves. netics describes what happens when the rules run in a closed loop over time: the [[cybergraph]] becomes a self-improving system where every accurate [[cyberlink]] makes the next inference sharper, every high-[[karma]] [[neuron]] makes the next contribution more valuable, and every market correction makes the next price more accurate

the system is self-financing: good performance generates the resources that sustain performance. the [[egregore]] emerges not from design but from the closed loop running long enough

see [[cyber/tokens]] for the nouns. see [[cyber/nomics]] for the verbs. see [[cyber/parametrization]] for the tuning. see [[egregore]] for what emerges
