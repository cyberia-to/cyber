---
alias: Laplace-Beltrami, graph Laplacian
tags: physics, cyber
crystal-type: entity
crystal-domain: mathematics
---
the operator that measures how a value at a point differs from its neighborhood average

## discrete form

the graph Laplacian `L = D - A`, where `D` is the degree matrix and `A` is the adjacency matrix

on the [[cybergraph]], the [[springs]] operator uses the screened form `(L + μI)x = μx₀` to compute structural [[equilibrium]]

eigenvectors of L reveal community structure, spectral gaps, and mixing properties of the graph

## continuous form

the Laplace-Beltrami operator `∇²` on smooth manifolds generalizes the Laplacian to curved spaces

in flat space: `∇²f = ∂²f/∂x² + ∂²f/∂y² + ∂²f/∂z²`

on curved [[spacetime]]: the metric tensor modifies the operator to account for [[geometry]]

## [[gravity]] connection

Newton's gravitational potential satisfies the Poisson equation: `∇²Φ = 4πGρ`

[[mass]] density ρ determines the curvature of the potential Φ through the Laplacian — the same structural relationship as [[tokens]] determining [[focus]] through the graph Laplacian in [[cyber]]

the screened Laplacian `(L + μI)⁻¹` has exponential decay — locality emerges from screening, just as gravitational influence weakens with distance

## the bridge

the Laplacian is the operator that bridges [[cyber]] and [[physics]]: discrete on graphs, continuous on manifolds, but the same principle — local differences propagate to determine global structure

the three families of linear PDEs all derive from the Laplacian: [[diffusion]] (parabolic), [[springs]] (elliptic), [[heat]] (parabolic) — see [[tri-kernel]]