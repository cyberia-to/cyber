---
tags: cyber
crystal-type: measure
crystal-domain: cybics
alias: mixing time
---

the difference between the two largest eigenvalues of a transition matrix or graph [[Laplacian]] — the single number that controls how fast a system reaches [[equilibrium]]

$$\lambda = 1 - |\lambda_2|$$

where $\lambda_2$ is the second-largest eigenvalue of the transition matrix $P$. $\lambda = 0$ means the system never mixes. $\lambda = 1$ means instant convergence. everything in between is governed by exponential decay:

$$\|\phi^{(t)} - \phi^*\| \leq C \cdot (1-\lambda)^t$$

## why it matters for cyber

the spectral gap is the heartbeat of the [[cybergraph]]. it determines:

- [[foculus]] finality speed — expected time to finality is $O(\log(1/\varepsilon)/\lambda)$ iterations. larger gap = faster [[consensus]]
- [[tri-kernel]] convergence rate — the composite contraction coefficient $\kappa < 1$ depends directly on the spectral gap of each operator
- [[learning incentives]] — spectral gap improvement $\lambda_2^t - \lambda_2^{t+1}$ is one of five candidate reward functions. linking that tightens the gap accelerates the entire system
- [[emergence]] thresholds — phase transitions in [[collective]] [[intelligence]] depend on $\lambda$ crossing critical values. sparse graphs have small gaps (slow mixing). dense, well-connected [[cybergraphs]] have large gaps (fast convergence)
- bootstrapping — a cold network has few [[cyberlinks]] and small spectral gap. finality may be slow until the [[cybergraph]] reaches sufficient density
- partition recovery — when two halves reconnect after a partition, $\lambda$ determines how quickly $\phi^*$ reconverges

## the math

for a [[random walk]] on a graph with transition matrix $P$:

the eigenvalues of $P$ satisfy $1 = \lambda_1 \geq |\lambda_2| \geq \ldots \geq |\lambda_n|$

the spectral gap $\lambda = 1 - |\lambda_2|$ controls mixing time:

$$t_{\text{mix}}(\varepsilon) = O\left(\frac{\log(n/\varepsilon)}{\lambda}\right)$$

for the [[tri-kernel]] composite operator $\mathcal{R} = \lambda_d D + \lambda_s S + \lambda_h H_\tau$:

- [[diffusion]] gap: determined by graph connectivity and teleport parameter $\alpha$
- [[springs]] gap: determined by screening parameter $\mu$ in $(L + \mu I)^{-1}$. larger $\mu$ = faster decay = larger effective gap
- [[heat kernel]] gap: determined by temperature $\tau$. the kernel $H_\tau = \exp(-\tau L)$ damps all modes except the leading eigenvector at rate $\exp(-\tau \lambda_{\text{Laplacian}})$

the composite gap: $\kappa = \lambda_d(1-\lambda_D) + \lambda_s(1-\lambda_S) + \lambda_h(1-\lambda_H) < 1$

see [[collective focus theorem]] Part II for the contraction proof

## what makes the gap large

- high connectivity — more edges = more paths for probability to flow = faster mixing
- small diameter — short distances between any two [[particles]]
- low degree variance — balanced graphs mix faster than hub-dominated ones
- teleport — the damping factor $\alpha$ in [[diffusion]] guarantees a minimum gap of $(1-\alpha)$, even for poorly connected graphs

## what shrinks the gap

- bottlenecks — a narrow cut between two dense clusters forces probability through a chokepoint
- partitions — disconnected components have $\lambda = 0$
- star topology — a single hub creates slow mixing (all paths go through one node)
- cold start — few [[cyberlinks]] means sparse graph means tiny gap

## related concepts

[[convergence]] — the process the spectral gap controls

[[equilibrium]] — the destination: $\phi^* = \phi^* P$

[[Laplacian]] — the graph operator whose eigenvalues define the gap. $L = D - A$, and the Fiedler eigenvalue $\lambda_2(L)$ is the algebraic connectivity

[[Perron-Frobenius theorem]] — guarantees existence and uniqueness of $\phi^*$ for irreducible aperiodic chains

[[entropy]] — the spectral gap bounds entropy production rate: $dH/dt \leq -\lambda \cdot H$

## in the literature

- Fiedler (1973): algebraic connectivity $\lambda_2(L)$ as graph connectivity measure
- Levin, Peres & Wilmer (2009): Markov chains and mixing times — the standard reference
- Spielman: spectral graph theory lecture notes
- Chung (2007): heat kernel as PageRank — spectral gap connects [[diffusion]] and [[heat]]

see [[collective focus theorem]] for convergence proofs. see [[foculus]] for consensus timing. see [[cyber/crystal]] for spectral gap validation targets