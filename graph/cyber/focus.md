---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: focus dynamics, CORE focus
---
# Focus Dynamics

[[focus]] is a single conserved quantity in [[CORE]] that serves three roles simultaneously. This unifies what other systems split into separate mechanisms—gas, stake, priority, reputation.

| Role | Mechanism |
|------|-----------|
| Attention | High-focus computations scheduled first |
| Fuel | Computation consumes focus |
| Consensus weight | Focus distribution = agreement signal |

The mathematical foundation is formalized in the [[cft]]. This page specifies the engineering: conservation laws, flow equation, and convergence properties as implemented in the [[CORE]] substrate.

## Conservation Laws

```
FOCUS CONSERVATION
──────────────────
Σᵢ focus(i) = 1   (always, enforced structurally)

Focus can:
  ✓ Flow between neurons (transfer)
  ✓ Be consumed (computation)
  ✓ Regenerate (from pool, proportional to balance)

Focus cannot:
  ✗ Be created from nothing
  ✗ Be destroyed (only redistributed)
  ✗ Exceed 1 in total


BALANCE CONSERVATION
────────────────────
Σᵢ balance(i) = B_total   (for non-minting transactions)

Enforced by polynomial commitment structure.
Invalid conservation → invalid state transition → rejected.


ENERGY CONSERVATION (Privacy Layer)
───────────────────────────────────
Σ(record values) = initial + minted - burned

Enforced by ZK circuit constraints.
```

## Focus Flow Equation

```
CONTINUOUS FORM (for analysis)
──────────────────────────────
∂f/∂t = -∇·(D∇f) + R - C

  f = focus distribution vector
  D = diffusion tensor (derived from weights × balances)
  R = regeneration rate
  C = consumption rate

DISCRETE FORM (for implementation)
──────────────────────────────────
f'ᵢ = Σⱼ Pᵢⱼ · fⱼ + rᵢ - cᵢ

  Pᵢⱼ = wᵢⱼ · bⱼ / Σₖ wₖⱼ · bₖ

  wᵢⱼ = edge weight from i to j
  bⱼ  = balance of neuron j
  rᵢ  = regeneration for neuron i
  cᵢ  = consumption by neuron i
```

## Convergence Theorem

Theorem: Focus dynamics converge to a unique stationary distribution π.

Proof:
The transition matrix P is:
- Stochastic (rows sum to 1)
- Irreducible (graph is strongly connected by assumption)
- Aperiodic (self-loops or odd cycles exist)

By Perron-Frobenius theorem, there exists a unique π where:
  πP = π
  Σᵢ πᵢ = 1
  πᵢ > 0 for all i

All initial distributions converge to π geometrically fast. ∎

Convergence rate: Determined by spectral gap λ = 1 - |λ₂|
  ‖f^(t) - π‖ ≤ C · (1-λ)^t

For the full probabilistic framework including axioms, proofs, and emergence theory, see [[cft]].
