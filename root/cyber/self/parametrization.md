---
tags: cyber, article, cip
crystal-type: pattern
crystal-domain: cyber
alias: parameter optimization, parameter reality, consensus parameter optimization, metabolic optimization
stake: 28558835390456748
diffusion: 0.00012985896128921983
springs: 0.0014685071937250377
heat: 0.0010491739810932852
focus: 0.0007153164349807992
gravity: 3
density: 2.09
---
# parametrization

## 1. the credibility gap

the [[cyber/whitepaper]] claims: "no parameters. only physics." this refers to the [[tri-kernel]] blend weights λ_d, λ_s, λ_h — which emerge as Lagrange multipliers from the [[free energy]] functional, the same way [[thermodynamics]] derives the Boltzmann distribution.

the claim is precisely correct for λ_d, λ_s, λ_h. it is silent about everything else.

the protocol contains at least twelve tunables that are parameters in every meaningful sense:

| parameter | controls | current specification |
|---|---|---|
| α | teleport probability in [[diffusion]] | (0, 1), unspecified |
| μ | screening strength in [[springs]] | > 0, unspecified |
| τ | temperature in [[heat kernel]] | ≥ 0, unspecified |
| κ | adaptive threshold scaling in [[foculus]] | [1, 2], self-regulating |
| γ | damping rate for temporal decay | (0, 1), unspecified |
| α_R | Shapley vs. marginal blend in [[learning incentives]] | [0, 1], unspecified |
| β_R, γ_R, ε_R | reward function coefficients (Δπ, ΔJ, DAG, alignment) | unspecified |
| E(t) | emission curve in [[cyber/tokenomics]] | PID-controlled |
| F | fee distribution | unspecified |

the blend weights λ_d, λ_s, λ_h are genuinely emergent — this is a real result, not rhetoric. but α, μ, and τ are free parameters that determine what each kernel computes before the variational optimization blends them. the screening strength μ determines how rigid the springs are. the temperature τ determines how much heat smoothing occurs. the teleport α determines how much random exploration diffusion performs. these are design choices, not physics.

the honest statement: the architecture is parameter-sparse. twelve tunables govern a system that replaces millions of weights in transformer architectures. the blend is physics. the individual kernel parameters are engineering. the question is how to set them.

## 2. three metabolic signals

every living system has metabolic indicators — measurable quantities that reflect health, growth, and homeostasis. the [[cybergraph]] has three:

### 2.1 cap: external validation

the total economic value of the network relative to external forces. measured as the fully diluted market capitalization of [[$CYB]] denominated in a reference unit (BTC, USD, energy equivalent).

cap reflects the external world's assessment of the network's utility. a rising cap means the network produces something the environment values — [[knowledge]], computation, coordination. a falling cap means the network is failing its environment.

this is the harshest signal. it integrates all external information: competing protocols, regulatory changes, macroeconomic shifts, actual usage. it cannot be gamed internally because it originates outside the system boundary.

cap as a metabolic signal:
- high cap / rising → the environment rewards the network → parameters are working
- low cap / falling → the environment penalizes the network → parameters need adjustment
- cap relative to competitors → comparative fitness signal

### 2.2 syntropy: internal order

[[syntropy]] (negentropy) J(π) = log|V| - H(π) measures the information-theoretic structure of the [[focus]] distribution π. high syntropy means π is concentrated on a structured set of [[particles]] — the network has organized its [[attention]] into coherent [[knowledge]]. low syntropy means π is diffuse — the network is noisy, unfocused, or spammed.

syntropy is computed every block. it is the objective, graph-intrinsic measure of organizational quality:

$$J(\pi) = \log|V| + \sum_j \pi_j \log \pi_j$$

syntropy as a metabolic signal:
- rising syntropy → [[cyberlinks]] are creating structure → [[neurons]] are contributing meaningful [[knowledge]]
- falling syntropy → noise outpaces structure → the graph is being degraded
- syntropy growth rate → velocity of [[knowledge]] organization

syntropy can be gamed by concentration — a cartel focusing all π on a few [[particles]] would produce high syntropy without genuine [[knowledge]]. this is why syntropy alone is insufficient. it must compound with cap (external validation) and [[happiness]] (subjective verification).

### 2.3 happiness: subjective verification

[[happiness]] is a stake-weighted survey: each [[neuron]] privately submits a number from 0 (hell) to 100 (nirvana). the [[vimputer]] weights submissions by [[token]] stake to resist [[sybil attacks]] and outputs a global index.

happiness reflects what cap and syntropy cannot: the subjective experience of participants. a network can have high cap (speculators love it) and high syntropy (bots create structure) while actual [[neurons]] are miserable — censored, manipulated, or unable to find what they need.

happiness as a metabolic signal:
- high happiness → participants find the system useful, fair, and responsive
- low happiness → something is wrong that metrics cannot capture
- happiness diverging from cap → speculation decoupled from utility
- happiness diverging from syntropy → structure exists but does not serve users

## 3. the compound signal

no single metabolic factor is sufficient. cap without syntropy rewards hype. syntropy without cap rewards internal coherence disconnected from reality. happiness without cap or syntropy rewards self-deception.

the three compound into a single metabolic health function:

$$M(t) = \text{cap}(t)^{w_c} \cdot J(t)^{w_s} \cdot H_{\text{happy}}(t)^{w_h}$$

where $w_c + w_s + w_h = 1$ are the metabolic weights, and the geometric mean ensures that collapse in any single signal drags the entire composite down. a network with zero happiness scores zero health regardless of cap or syntropy.

the metabolic derivative:

$$\dot{M}(t) = w_c \frac{\dot{\text{cap}}}{\text{cap}} + w_s \frac{\dot{J}}{J} + w_h \frac{\dot{H}_{\text{happy}}}{H_{\text{happy}}}$$

this is the growth rate of metabolic health — the signal that parameter optimization maximizes.

## 4. reinforcement learning on parameters

### 4.1 the optimization problem

the protocol is a parameterized dynamical system. the state evolves under the [[tri-kernel]] with parameters θ = (α, μ, τ, κ, γ, α_R, ...). the metabolic health M(t) is the long-horizon reward.

this is a reinforcement learning problem:
- state: the current [[cybergraph]] topology, [[focus]] distribution π, and metabolic history
- action: adjust parameter vector θ
- reward: ΔM over an evaluation window
- policy: a mapping from metabolic state to parameter adjustment

### 4.2 why RL and not fixed optimization

the parameter landscape is non-stationary. the optimal α depends on graph density, which changes as [[neurons]] add [[cyberlinks]]. the optimal τ depends on the spectral properties of the [[cybergraph]], which shift as the network grows. the optimal κ depends on adversarial pressure, which varies over time.

static optimization finds a fixed point for a frozen system. reinforcement learning continuously adapts to a living one.

the environment is partially observable: the protocol cannot see external market conditions, cannot predict regulatory changes, cannot measure user intent directly. RL handles partial observability through temporal credit assignment — adjusting parameters based on delayed metabolic consequences.

### 4.3 the parameter hierarchy

parameters operate at different timescales and carry different risks:

| tier | parameters | adjustment frequency | risk of change |
|---|---|---|---|
| epoch-level | κ (foculus threshold scaling) | every epoch | low — self-regulating by design |
| seasonal | α, τ (exploration/smoothing) | every 10³-10⁴ blocks | medium — affects convergence rate |
| structural | μ (screening strength) | every 10⁵+ blocks | high — affects fixed point location |
| economic | reward coefficients (α_R, β_R, γ_R) | governance cycles | high — affects incentive equilibrium |
| permanent | [[Hemera]] hash parameters | never | irreversible |

the RL agent operates differently at each tier. fast parameters use online learning with short evaluation windows. slow parameters use batched evaluation with long lookback. permanent parameters are outside the optimization loop.

### 4.4 the search space

for the [[tri-kernel]] parameters (α, μ, τ), the search is constrained by the [[collective focus theorem]]: any valid (α, μ, τ) must maintain κ < 1 for contraction. this defines a feasible region:

$$\kappa(\theta) = \lambda_d \alpha + \lambda_s \frac{\|L\|}{\|L\| + \mu} + \lambda_h e^{-\tau \lambda_2} < 1$$

the RL agent searches within this region. configurations that violate κ < 1 are rejected — the protocol's mathematical invariants are hard constraints, not suggestions.

within the feasible region, the landscape has structure:
- high α → more exploration, slower convergence, higher syntropy diversity
- high μ → stiffer springs, faster convergence to structural consensus, lower adaptability
- high τ → more heat smoothing, broader context integration, risk of oversmoothing

the optimal balance depends on the current state of the [[cybergraph]] — which is exactly what RL can learn.

### 4.5 safety constraints

parameter optimization must respect safety invariants:

- conservation: Σ π_i = 1 at every step, regardless of parameters
- convergence: κ < 1 always — no parameter adjustment may break the contraction guarantee
- monotonicity: finalized [[particles]] stay final — parameter changes cannot retroactively invalidate [[consensus]]
- bounded change: |Δθ| < ε per adjustment step — no discontinuous parameter jumps

violations of any constraint are blocked at the protocol level. the RL agent proposes; the invariant checker disposes.

## 5. implementation architecture

### 5.1 the metabolic oracle

a dedicated computation, running alongside the [[tri-kernel]], that tracks the three metabolic signals:

```
every epoch:
  1. compute syntropy J(π) from current focus distribution
  2. read cap from on-chain oracle (IBC price feed or DEX TWAP)
  3. aggregate happiness from neuron submissions (stake-weighted)
  4. compute M(t) = cap^w_c · J^w_s · H_happy^w_h
  5. compute ΔM = M(t) - M(t-1)
  6. feed ΔM to the parameter agent
```

### 5.2 the parameter agent

a bounded computation that proposes parameter adjustments:

```
every evaluation window (10³ blocks):
  1. observe: metabolic history [M(t-W), ..., M(t)]
  2. observe: current parameters θ
  3. observe: graph statistics (density, spectral gap, active neurons)
  4. propose: Δθ within safety bounds
  5. verify: κ(θ + Δθ) < 1
  6. apply: θ ← θ + Δθ
```

the agent itself is deterministic — given the same metabolic history and graph state, it produces the same parameter adjustment. this is essential for [[consensus]]: every [[neuron]] must compute the same Δθ.

### 5.3 what is learned vs. what is fixed

learned by the parameter agent:
- α, τ: adapted to current graph topology and spectral properties
- κ bounds: adapted to observed variance patterns
- reward blend coefficients: adapted to observed incentive outcomes

fixed by protocol design:
- λ_d, λ_s, λ_h: emergent from [[free energy]] minimization — the "no parameters, only physics" claim holds here
- conservation laws: structural invariant, unmodifiable
- [[Hemera]] hash parameters: permanent genesis commitment
- safety constraints: κ < 1, bounded change, monotonicity

governed (not learned):
- μ (screening strength): too consequential for autonomous adjustment — governance proposal required
- metabolic weights w_c, w_s, w_h: define what "health" means — a value judgment, not an optimization target

## 6. the honest claim, revised

the original claim: "no parameters. only physics."

the revised claim: the [[tri-kernel]] blend weights λ_d, λ_s, λ_h emerge from physics via [[free energy]] minimization — this is proven. the kernel parameters α, μ, τ are engineering choices — this is acknowledged. the protocol resolves this through metabolic reinforcement learning: three compounding signals (cap, [[syntropy]], [[happiness]]) provide the reward function for continuous parameter adaptation. the chain learns its own configuration by optimizing for external validation, internal order, and participant satisfaction simultaneously.

twelve tunables. three metabolic signals. one optimization loop. the physics determines the architecture. the metabolism determines the parameters.

---

see [[tri-kernel]] for the three operators, [[foculus]] for the adaptive threshold, [[free energy]] for the variational foundation, [[syntropy]] for the information-theoretic signal, [[happiness]] for the subjective signal, [[cyber/rewards]] for the incentive mechanism, [[collective focus theorem]] for the convergence guarantee, [[epistemic correctness]] for the gap between convergent attention and truth