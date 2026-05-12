---
tags: article, cyber, cip
crystal-type: pattern
crystal-domain: cyber
crystal-size: deep
status: draft
---
# Tri-Kernel Specification

formal definition of the three local operators whose fixed point is [[cyberank]]. part of the [[cyber/core]] specification

---

## 1. The Three Primitives

### 1.1 Primitive M: Markov/Diffusion

The transition matrix M = D⁻¹A (or column-stochastic P = AD⁻¹) governs probability flow:

$$\phi^{(t+1)} = \alpha P^\top \phi^{(t)} + (1-\alpha)u$$

where α ∈ (0,1) is the teleport parameter and u is a prior (often uniform or stake-weighted).

Properties: Row-stochastic, preserves probability mass, powers remain local. Under ergodicity (strong connectivity + aperiodicity), converges to unique stationary distribution φ*.

Answers: *"Where does probability flow?"*

### 1.2 Primitive L: Laplacian/Springs

The graph Laplacian L = D - A (or normalized ℒ = I - D⁻¹/²AD⁻¹/²) encodes structural constraints:

$$(L + \mu I)x^* = \mu x_0$$

where μ > 0 is the screening/stiffness parameter and x₀ is a reference state.

Properties: Positive semi-definite, null space = constant vectors. The screened Green's function (L+μI)⁻¹ has exponential decay, ensuring locality.

Answers: *"What satisfies structural constraints?"*

### 1.3 Primitive H: Heat Kernel

The heat kernel H_τ = exp(-τL) provides multi-scale smoothing:

$$\frac{\partial H}{\partial \tau} = -LH, \quad H_0 = I$$

where τ ≥ 0 is the temperature/time parameter.

Properties: Positivity-preserving, semigroup (H_{τ₁}H_{τ₂} = H_{τ₁+τ₂}). Admits Chebyshev polynomial approximation for locality.

Answers: *"What does the graph look like at scale τ?"*

---

## 2. The Composite Operator

The tri-kernel blends the three primitives into a single update:

$$\phi^{(t+1)} = \text{norm}\big[\lambda_d \cdot D(\phi^t) + \lambda_s \cdot S(\phi^t) + \lambda_h \cdot H_\tau(\phi^t)\big]$$

where λ_d + λ_s + λ_h = 1, D is the [[diffusion]] step, S is the [[springs]] equilibrium map, H_τ is the [[heat]] map, and norm(·) projects to the simplex.

### 2.1 The Free Energy Functional

The fixed point of the composite operator minimizes:

$$\mathcal{F}(\phi) = \lambda_s\left[\frac{1}{2}\phi^\top L\phi + \frac{\mu}{2}\|\phi-x_0\|^2\right] + \lambda_h\left[\frac{1}{2}\|\phi-H_\tau\phi\|^2\right] + \lambda_d \cdot D_{KL}(\phi \| D\phi)$$

This is a free-energy functional: the first term is elastic structure, the second penalizes deviation from heat-smoothed context, the third aligns φ with its [[diffusion]] image.

### 2.2 Convergence and Locality

Theorem (Composite Contraction): Under ergodicity of P, screening μ > 0, and bounded τ, the composite operator ℛ is a contraction with coefficient κ < 1. Hence φ^t → φ* linearly. See [[collective focus theorem]] Part II for the proof.

Theorem (Locality Radius): For edit batch e_Δ, there exists h = O(log(1/ε)) such that recomputing only on N_h (the h-hop neighborhood) achieves global error ≤ ε.

This follows from: geometric decay for [[diffusion]] (teleport), exponential decay for [[springs]] (screening), Gaussian tail for [[heat]] (kernel bandwidth).

### 2.3 Compute-Verify Symmetry

Because all operations are local and memoizable:

$$t_{verify} / t_{compute} \to c \approx 1$$

Light clients can verify [[focus]] updates by checking boundary flows and authenticated neighborhood commitments, with constant-factor overhead relative to computation.

---

## 3. Completeness

### 3.1 Completeness Conjecture

Conjecture (Weak Completeness): Any h-local linear operator T can be written as T = p(M) + q(L) for polynomials p, q of degree ≤ h.

Conjecture (Strong Completeness): Any eventually-local operator that is equivariant, continuous, and convergent can be expressed as T = α·f(M) + β·g(L) + γ·H_τ for spectral functions f, g and scale τ.

### 3.2 Lemmas Toward Proof

Lemma 1: Any 1-local linear operator is a linear combination of {I, A, D}.

Lemma 2: Any k-local linear operator is a polynomial of degree ≤ k in {A, D}.

Lemma 3: Polynomials in {A, D} can be rewritten as polynomials in {M, L}.

Theorem (Linear Local Completeness): Every k-local linear operator on a graph is a polynomial of degree ≤ k in M and L.

The heat kernel H_τ = exp(-τL) is required for multi-scale analysis—it is the unique generator of resolution-dependent queries. Together {M, L, H_τ} span the space of meaningful local graph computations.

---

## 4. Implementation

### 4.1 Two-Timescale Architecture

The correct implementation separates timescales:

- Structure (slow, amortized): [[springs]] precompute effective distances, modify [[diffusion]] tensor D
- [[Focus]] flow (fast, local): [[diffusion]] + [[heat]] operate on fixed structure, converge to equilibrium

[[Springs]] compute *where nodes are*; ranking computes *how [[attention]] flows*. Different questions, different timescales.

### 4.2 Algorithm Sketch

Per epoch on neighborhood N_h:

1. Detect affected neighborhood around edit batch e_Δ
2. Pull boundary conditions: cached φ, boundary flows, Laplacian blocks
3. Apply local [[diffusion]] (fixed-point iteration with boundary injection)
4. Apply local [[heat]] (Chebyshev K-term filter)
5. Normalize and splice back into global φ
6. Emit attention_root and locality report for verification

Complexity: O(|N_h| · c) per kernel for average degree c.

### 4.3 Telemetry

Monitor per epoch:
- Entropy H(φ*), negentropy J(φ*)
- Spectral gap estimate
- ℓ₁ drift ‖φ*^t - φ*^(t-1)‖
- Locality radius h, nodes touched
- Compute vs verify wall-time

Safety policies: degree caps, spectral sparsification, novelty floor, auto-rollback to [[diffusion]]-only on threshold breach.

---

## References

1. Brin & Page. "The anatomy of a large-scale hypertextual web search engine." WWW 1998
2. Zhu et al. "Semi-supervised learning using Gaussian fields and harmonic functions." ICML 2003
3. Chung. "The heat kernel as the pagerank of a graph." PNAS 2007
4. Fiedler. "Algebraic connectivity of graphs." Czech Math Journal 1973
5. Spielman. "Spectral Graph Theory." Yale Lecture Notes
6. Levin, Peres & Wilmer. "Markov Chains and Mixing Times." AMS 2009

see [[tri-kernel architecture]] for the explanatory whitepaper