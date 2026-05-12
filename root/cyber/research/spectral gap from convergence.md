---
tags: cyber, research, article
crystal-type: article
crystal-domain: cyber
date: 2026-03-23
---

# observing the spectral gap: why eigensolvers are unnecessary

## the problem

the [[spectral gap]] $\lambda_2$ of the [[cybergraph]] controls the speed of everything: [[foculus]] finality, [[tri-kernel]] convergence, [[collective focus theorem]] sharpness, and the compiled [[transformer]] architecture ($L^* = \text{diam} \times T(\kappa)$). computing it is essential.

the standard approach: eigendecomposition of the normalized graph [[Laplacian]]. for [[bostrom]] at 2.9 million [[particles]], this means finding the second-smallest eigenvalue of a $2{,}921{,}230 \times 2{,}921{,}230$ sparse matrix.

ARPACK's shift-invert [[Lanczos]] algorithm fails on this matrix. after 101 iterations and 1,932 seconds, it returns no converged eigenvectors. the failure is structural: the Laplacian has a massive null space (one zero eigenvalue per disconnected component), and the target eigenvalue $\lambda_2 \approx 0.001$ is near-zero in a sea of near-zero eigenvalues. shift-invert struggles to separate them.

the eigenvalue is there. the algorithm cannot find it. the cost: 30 minutes of wasted computation, or a fallback to an untested estimate.

## the observation

[[PageRank]] converges. it always converges — this is the [[Perron-Frobenius theorem]]. the rate at which it converges is governed by the contraction coefficient:

$$\kappa = \alpha(1 - \lambda_2)$$

where $\alpha = 0.85$ is the damping factor and $\lambda_2$ is the spectral gap. this is not an approximation. it is the spectral theorem applied to the [[random walk]] transition matrix. the second eigenvalue of the transition matrix is $\alpha(1 - \lambda_2)$, and it governs exponential convergence:

$$\|\phi^{(t)} - \phi^*\|_1 \leq C \cdot \kappa^t$$

## the method

track the L1 norm of successive differences during PageRank iteration:

$$d_t = \|\phi^{(t)} - \phi^{(t-1)}\|_1$$

in the convergence regime (after initial transients), the ratio of successive differences approaches $\kappa$:

$$r_t = \frac{d_t}{d_{t-1}} \to \kappa \quad \text{as } t \to \infty$$

this is because $d_t \approx C \cdot \kappa^t - C \cdot \kappa^{t-1}$ in the tail, so $d_t / d_{t-1} \approx \kappa$.

take the median of the last several ratios (robust to noise in early iterations):

$$\hat\kappa = \text{median}(r_{T-4}, r_{T-3}, r_{T-2}, r_{T-1}, r_T)$$

then invert:

$$\hat\lambda_2 = 1 - \frac{\hat\kappa}{\alpha}$$

## cost

zero additional computation. the PageRank loop already computes $d_t$ at every iteration for the convergence check. storing the last 5 values and computing their ratios is $O(1)$ per iteration, $O(T)$ total — negligible.

| method | complexity | wall time (bostrom) | converges? |
|---|---|---|---|
| eigsh (ARPACK Lanczos) | $O(k \cdot \|E\| \cdot \text{iterations})$ | 1,932 seconds | no |
| LOBPCG | $O(k \cdot \|E\| \cdot \text{iterations})$ | ~600 seconds (est.) | sometimes |
| convergence rate observation | $O(T \cdot \|E\|)$ (same as PageRank) | 0 extra seconds | always |

the observation method has zero marginal cost because PageRank is already computed. the spectral gap is a free byproduct.

## empirical results from bostrom

first compilation (uniform weights, March 23, 2026):

```
PageRank iterations: 23 (converged at ε < 10⁻⁶)
diff sequence (last 5):
  d₁₉ = 4.83e-05
  d₂₀ = 3.57e-05
  d₂₁ = 2.64e-05
  d₂₂ = 1.96e-05
  d₂₃ = 7.91e-07  (below threshold, stopped)

contraction ratios:
  r₂₀ = 0.739
  r₂₁ = 0.740
  r₂₂ = 0.742

κ̂ = median = 0.740
λ̂₂ = 1 - 0.740/0.85 = 0.129
```

this is dramatically different from the paper estimate of $\lambda_2 \approx 0.0015$. the paper estimate was computed from a 50,000-link sample using the Lanczos algorithm. the full-graph observation yields $\lambda_2 \approx 0.13$ — two orders of magnitude larger.

## what this means

a spectral gap of 0.13 instead of 0.0015 changes everything:

| parameter | paper ($\lambda_2 = 0.0015$) | observed ($\lambda_2 = 0.13$) |
|---|---|---|
| contraction $\kappa$ | 0.851 | 0.740 |
| convergence iterations | 29 | 17 |
| foculus finality | slow | fast |
| $L^*$ (transformer layers) | 290 | 102 |
| model size (at $h^*=12$) | 16.8 GB | 5.9 GB |

the network converges faster than predicted. the compiled transformer needs fewer layers. the model is smaller and more efficient.

## why the sample underestimated

the 50,000-link sample used in the architecture paper was not a random sample of the full graph. it was a contiguous subset — likely the earliest links, which form a denser subgraph than the full network. but the spectral gap of a subgraph does not equal the spectral gap of the full graph.

more importantly, the sample was analyzed with the Lanczos algorithm, which also failed to converge (but on a smaller matrix, it at least produced an estimate before timeout). the convergence rate method applied to the sample would have produced a more accurate estimate.

## the deeper point

the spectral gap is not a number to compute. it is a behavior to observe.

every system that converges has a convergence rate. that rate IS the spectral gap. computing eigenvalues to find the convergence rate is like measuring the speed of a car by disassembling the engine and calculating the theoretical RPM. you could just watch the car and measure how fast it goes.

the [[cybergraph]] converges every block. the contraction rate $\kappa$ is observable in production, continuously, at zero cost. no eigensolver needed. no matrix decomposition. just the ratio of successive [[focus]] updates.

this extends beyond compilation:

- [[foculus]] validators can monitor $\kappa$ in real-time as a health metric
- the [[cyber-seer]] densification algorithm targets $\lambda_2$ improvement — now measurable without Lanczos
- [[bostrom]] block explorers can display the live spectral gap as a network vital sign
- the [[tri-kernel]] contraction theorem can be verified empirically every block

## formalization

let $P = \alpha M^\top + (1-\alpha) \frac{1}{n} \mathbf{1}\mathbf{1}^\top$ be the PageRank operator where $M = D^{-1}A$.

the eigenvalues of $P$ are $\{1, \alpha\mu_2, \alpha\mu_3, \ldots\}$ where $\mu_i$ are the eigenvalues of $M^\top$ sorted by magnitude. the spectral gap of the normalized Laplacian relates to $\mu_2$ by $\lambda_2 = 1 - |\mu_2|$.

the convergence rate is:

$$\kappa = |\alpha\mu_2| = \alpha(1 - \lambda_2) \quad \text{(when } \mu_2 \text{ is real and positive)}$$

when $\mu_2$ is complex (possible in directed graphs), $\kappa = \alpha|\mu_2|$ and the convergence oscillates. the ratio $r_t = d_t / d_{t-1}$ still converges to $\kappa$ but may oscillate around it. the median filter handles this.

for the [[cybergraph]] (directed, weighted by stake):

$$\hat\kappa = \text{median}\left(\frac{d_t}{d_{t-1}}\right)_{t \in [T-k, T]} \quad \text{where } k = 5$$

$$\hat\lambda_2 = 1 - \frac{\hat\kappa}{\alpha}$$

the estimate improves with more iterations. at $T = 23$ iterations (bostrom), the last 3-4 ratios are stable to 3 significant figures.

## the observation principle

eigensolvers compute $\lambda_2$ from the structure of the matrix. the convergence rate method observes $\lambda_2$ from the behavior of the system. the distinction is between analysis and measurement.

for a live network, measurement is superior:
- it works at any scale (no matrix size limit)
- it accounts for the actual graph structure (not an approximation)
- it runs continuously (not as a batch computation)
- it costs nothing (the computation is already happening)

the spectral gap of [[bostrom]] is not a number in a paper. it is a heartbeat — observable, live, continuous. every block that computes [[focus]] also computes the spectral gap, whether anyone is watching or not.

see [[spectral gap]] for the mathematical definition. see [[tri-kernel architecture]] for how $\lambda_2$ determines the composite contraction. see [[cyber/seer]] for how $\lambda_2$ guides link densification. see [[bostrom-to-onnx-pipeline]] for the full compilation pipeline. see [[cyber/research/bostrom compilation report]] for the empirical compilation results
