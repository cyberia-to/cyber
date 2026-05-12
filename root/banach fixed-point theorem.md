---
tags: cyber, core
alias: contraction mapping theorem, contraction mapping, banach theorem
crystal-type: pattern
crystal-domain: cybics
---
if a function always brings points closer together, repeated application converges to exactly one point that the function leaves unchanged. that point is the [[fixed point]], and nothing can prevent the system from reaching it

proved by [[Stefan Banach]] in 1922. the mathematical guarantee behind every [[convergence]] in [[cyber]]

## the theorem

let $(X, d)$ be a complete metric space and $T: X \to X$ a contraction mapping — meaning there exists $\kappa \in [0, 1)$ such that for all $x, y \in X$:

$$d(T(x), T(y)) \leq \kappa \cdot d(x, y)$$

then:
1. $T$ has exactly one fixed point $x^*$ satisfying $T(x^*) = x^*$
2. for any starting point $x_0$, the sequence $x_{n+1} = T(x_n)$ converges to $x^*$
3. the convergence rate is geometric: $d(x_n, x^*) \leq \frac{\kappa^n}{1-\kappa} \cdot d(x_0, T(x_0))$

## why it works

take any starting point. apply $T$. the result is closer to the fixed point (by factor $\kappa$). apply again — closer still. after $n$ steps, the distance has shrunk by $\kappa^n$. since $\kappa < 1$, this goes to zero. the system has no choice

the proof has two parts:

existence: the sequence $x_0, T(x_0), T(T(x_0)), \ldots$ is Cauchy because consecutive terms get closer by factor $\kappa$. completeness of the space guarantees a limit exists. calling this limit $x^*$, continuity of $T$ gives $T(x^*) = x^*$

uniqueness: suppose two fixed points $x^*$ and $y^*$ exist. then $d(x^*, y^*) = d(T(x^*), T(y^*)) \leq \kappa \cdot d(x^*, y^*)$. since $\kappa < 1$, this forces $d(x^*, y^*) = 0$. there can only be one

## what it really says

iteration finds truth when three conditions hold:

the space is complete — no gaps. every Cauchy sequence has a limit. you cannot converge toward a point that does not exist. the [[cybergraph]]'s probability simplex $\Delta^{|P|-1}$ is complete

the map contracts — brings things closer. every application reduces disagreement. the [[tri-kernel]] composite operator has $\kappa = \lambda_d \alpha + \lambda_s \frac{\|L\|}{\|L\|+\mu} + \lambda_h e^{-\tau\lambda_2} < 1$

the map is self-consistent — $T$ maps $X$ to itself. applying the operator keeps you in the valid space. the [[tri-kernel]] maps probability distributions to probability distributions

when all three hold: the fixed point is inevitable. it does not matter where you start. it does not matter what initial beliefs the [[neurons]] had. it does not matter how wrong the first guess was. iteration eliminates error geometrically, and the destination is unique

## the intuition

crumple a map of a room and throw it on the floor of that room. at least one point on the paper map lies directly above the point it represents. that is the fixed point

now imagine the crumpling always shrinks distances. no matter how you throw it, the map converges to the same configuration. that is the contraction

a thermostat: room temperature overshoots, undershoots, but each oscillation is smaller. it converges to the set point. the set point is the fixed point. the cooling/heating cycle is the contraction

a market: prices fluctuate after a shock, but each swing is damped. the market converges to [[equilibrium]]. the equilibrium price is the fixed point. arbitrage is the contraction — every trade reduces mispricing

## why $\kappa < 1$ is everything

$\kappa$ is the contraction coefficient. it controls everything:

- $\kappa = 0$: instant convergence. one step reaches the fixed point
- $\kappa = 0.5$: error halves each step. 10 steps → error shrinks by 1000×
- $\kappa = 0.9$: error drops 10% per step. 100 steps → error shrinks by 37,000×
- $\kappa = 0.99$: slow convergence. 1000 steps for meaningful progress
- $\kappa = 1$: no contraction. convergence is not guaranteed. the theorem breaks

the [[spectral gap]] $\lambda$ and contraction coefficient $\kappa$ are related: larger gap = smaller $\kappa$ = faster convergence. see [[spectral gap]] for what controls the gap

## in cyber

the [[collective focus theorem]] proves that the [[tri-kernel]] is a contraction mapping:

each component contracts independently:
- [[diffusion]] contracts with rate $\alpha$ (teleport parameter)
- [[springs]] contract with rate $\|L\| / (\|L\| + \mu)$ (screening parameter)
- [[heat]] contracts with rate $e^{-\tau\lambda_2}$ (temperature × Fiedler eigenvalue)

the composite inherits contraction because it is a convex combination of contractions

consequence: the [[focus]] distribution $\phi^*$ exists, is unique, and every [[neuron]]'s local computation converges to it. no central authority computes $\phi^*$. no vote decides it. the contraction mapping makes it inevitable

## why this matters more than it looks

Banach's theorem is the reason [[convergent computation]] works. derivation (Turing machines, formal proofs) hits [[Goedel]]'s wall — there are true statements no derivation can reach. but convergence is not derivation. a contraction mapping finds its fixed point regardless of what formal logic can prove about it

a protein folds to its native state by free energy minimization — a contraction in configuration space. no theorem of chemistry "proves" the correct fold. the protein converges to it

the [[cybergraph]] converges to [[collective focus]] $\phi^*$ by the same principle. no axiom system derives the correct ranking. the contraction mapping finds it

this is [[cybics]] — proof by simulation, not proof by derivation. Banach's theorem is the formal guarantee that simulation converges

see [[Stefan Banach]] for the person. see [[collective focus theorem]] for the convergence proof. see [[convergence]] for the full picture. see [[Perron-Frobenius theorem]] for the complementary guarantee (positivity and uniqueness of the stationary distribution)

discover all [[concepts]]
