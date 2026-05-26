---
tags: cyber, mathematics
crystal-type: entity
crystal-domain: cyber
---
# heat kernel

the multi-scale smoothing operator in the [[tri-kernel]]

$$H_\tau = \exp(-\tau L)$$

where $L$ is the graph [[Laplacian]] and $\tau$ is the temperature parameter. one of three components of the [[tru]] computation alongside [[diffusion]] and [[springs]]

## the heat equation on graphs

the operator arises from the graph heat equation:

$$\frac{\partial H}{\partial \tau} = -L H, \quad H_0 = I$$

the solution is the matrix exponential $H_\tau = \exp(-\tau L)$. applied to a signal $\phi$ on the [[cybergraph]], $H_\tau \phi$ returns a smoothed version where each [[particle]]'s value blends with its neighborhood — Gaussian-like decay over graph distance

## temperature as scale

$\tau$ controls the spatial reach of smoothing:

| regime | behavior | analogy |
|---|---|---|
| $\tau \to 0$ | identity — each particle sees only itself | crystallization, commitment |
| small $\tau$ | local smoothing — nearby particles blend | fine-grained context |
| large $\tau$ | global smoothing — distant particles contribute | annealing, exploration |
| $\tau \to \infty$ | uniform distribution — all structure erased | maximum entropy |

high $\tau$ explores. low $\tau$ commits. the temperature parameter is the thermostat of [[collective focus]] — controlling how much the system adapts versus how much it preserves local structure

## spectral decomposition

in the eigenbasis of $L$ with eigenvalues $0 = \lambda_1 \leq \lambda_2 \leq \cdots \leq \lambda_n$:

$$H_\tau = \sum_{k=1}^{n} e^{-\tau \lambda_k} \, v_k v_k^\top$$

low-frequency eigenvectors (small $\lambda_k$) persist. high-frequency eigenvectors (large $\lambda_k$) are exponentially suppressed. this is why the operator acts as a multi-scale smoother: it preserves global community structure while erasing local noise

the [[spectral gap]] $\lambda_2$ (the Fiedler eigenvalue) governs the contraction rate:

$$\|H_\tau \phi - H_\tau \psi\|_2 \leq e^{-\tau \lambda_2} \|\phi - \psi\|_2$$

## properties

positivity-preserving: if $\phi \geq 0$ then $H_\tau \phi \geq 0$. the operator never creates negative values from positive inputs

semigroup: $H_{\tau_1} H_{\tau_2} = H_{\tau_1 + \tau_2}$. smoothing at scale $\tau_1$ then at scale $\tau_2$ equals smoothing at scale $\tau_1 + \tau_2$

locality: Gaussian tail decay means the effect at distance $d$ falls as $\exp(-d^2 / 4\tau)$. for precision $\varepsilon$, only $h = O(\log(1/\varepsilon))$ hops are needed. Chebyshev polynomial approximation computes $H_\tau \phi$ locally with bounded error

contraction: the [[collective focus theorem]] proves $H_\tau$ contracts with rate $e^{-\tau \lambda_2} < 1$, contributing to the composite contraction of the full [[tri-kernel]]

## role in the tri-kernel

the [[tri-kernel]] blends three operators:

$$\phi^{(t+1)} = \text{norm}\big[\lambda_d \cdot D(\phi^t) + \lambda_s \cdot S(\phi^t) + \lambda_h \cdot H_\tau(\phi^t)\big]$$

[[diffusion]] explores via random walks. [[springs]] enforce structural consistency via the screened [[Laplacian]]. the heat kernel provides adaptive context — what the graph looks like at scale $\tau$. together the three forces converge to [[focus]]: the fixed point that is [[cyberank]]

see [[cyber/heat]] for the full specification. see [[tri-kernel]] for the composite operator. see [[collective focus theorem]] for the convergence proof. see [[Fan Chung]] for the foundational work on heat kernels and PageRank. see [[spectral gap]] for the eigenvalue that governs contraction rate
