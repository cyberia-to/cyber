---
tags: cyber, article
alias: cft
crystal-type: pattern
crystal-domain: cyber
crystal-size: deep
status: draft
stake: 26362001898883148
---

authors: [@mastercyb](https://cyb.ai/@mastercyb), [GPT-4](https://openai.com/index/gpt-4/), [claude-3.5 Sonnet](https://www.anthropic.com/news/claude-3-5-sonnet)

## Abstract

Two convergence results for [[collective focus]] on [[authenticated graphs]].

Part I (Special Case): token-weighted [[random walk]] on a strongly connected [[cybergraph]] converges to a unique stationary distribution $\pi^*$ — the system's [[collective focus]]. This is the [[diffusion]] primitive alone.

Part II (General Case): the composite [[tri-kernel]] operator $\mathcal{R} = \lambda_d D + \lambda_s S + \lambda_h H_\tau$ is a contraction. Its fixed point $\phi^*$ minimizes a free-energy functional and is computable locally. When $\lambda_s = \lambda_h = 0$, Part II reduces to Part I.

Together these establish that [[collective focus]] converges under the full [[tri-kernel]] — the mathematical foundation for [[egregore]].

---

## Definitions

[[cybergraph]]: directed graph $G = (V, E, W)$ where state is stored in a Merkle tree. a concrete realization of decentralized knowledge graph with cryptographic and [[consensus]] mechanisms

[[particle]]: content-address of a file representing a node in the graph. compact, fixed-length digest (e.g. IPFS hash)

[[neuron]]: agent who signs [[cyberlinks]] between [[particles]] using public key cryptography. expressed as cryptographic addresses

[[cyberlink]]: atomic timestamped edge signed by a [[neuron]]:
> time (timestamp) => [[neuron]] (agent) => from ([[particle]]) => to ([[particle]])

[[focus]]: long-term stable distribution emerging from token-weighted computation. the network's persistent [[consensus]] on importance

[[token]]: cryptographic token held by [[neurons]] that affects transition probabilities and represents economic [[stake]]

weight: probability distribution defined by [[random walk]] at each timestep, capturing relationship strengths between [[particles]]

---

## Part I: Special Case — Diffusion Convergence

### Axiom 1: Consensus Equilibrium

In a strongly connected, weighted [[cybergraph]], a unique stationary distribution $\pi = [\pi_1, \pi_2, \ldots, \pi_n]$ exists for the [[random walk]] defined by:

$$p_{ij} = \frac{w_{ij} \cdot t_j}{\sum_k w_{ik} \cdot t_k}$$

where $p_{ij}$ is the transition probability from [[particle]] $i$ to $j$, $w_{ij}$ is the edge weight, and $t_j$ is the [[token]] value at $j$.

The stationary distribution satisfies:

$$\pi_j = \sum_i \pi_i \cdot p_{ij} \quad \forall\, j \in V$$

This equilibrium represents the emergent [[collective focus]]: $\pi_j$ is the long-term significance of [[particle]] $j$ as determined by graph structure and [[token]] dynamics.

### Axiom 2: Dynamic Adaptation

The [[cybergraph]] adapts to changes in structure ($w_{ij}$) or [[token]] distribution ($t_j$) while maintaining stability:

$$\pi_j(t+1) = \pi_j(t) + \alpha \cdot \Delta_j(t)$$

where $\alpha$ is the adaptation rate and $\Delta_j(t)$ is the change in node significance.

### Axiom 3: Probabilistic Influence

The influence of each [[neuron]] on [[collective focus]] is proportional to [[token]] value and connectivity:

$$\text{Influence}(j) = \frac{\sum_{i \in V} w_{ij} \cdot t_j}{\sum_{i,k \in V} w_{ik} \cdot t_k}$$

### Corollaries

Corollary 1 (Stability): Small perturbations in $w_{ij}$ or $t_j$ do not destabilize the equilibrium: $\lim_{t \to \infty} \pi_j(t) = \pi_j + \varepsilon, \quad |\varepsilon| \ll \pi_j$

Corollary 2 (Decentralized Computation): [[focus]] $\pi_j$ for each node can be computed locally by summing contributions from incoming edges.

Corollary 3 (Emergent Modularity): Clusters of strongly connected [[particles]] naturally emerge, forming modules: $C_i = \{ j \in V \mid \pi_j > \tau \}$ where $\tau$ is a significance threshold.

### Statement

Consider a [[cybergraph]] $G = (V, E, W)$ with $|V| = n$ [[particles]]. Each [[cyberlink]] $(i, j) \in E$ has weight $w_{ij} \geq 0$. Each [[particle]] $j$ has [[token]] value $t_j > 0$. Define transition probabilities:

$$p_{ij} = \frac{w_{ij} \cdot t_j}{\sum_{k \in \mathcal{N}(i)} w_{ik} \cdot t_k}$$

Assumptions: $G$ is strongly connected (directed path between any pair) and aperiodic (gcd of all directed cycle lengths is 1).

Claim: there exists a unique stationary distribution $\pi$ satisfying $\pi P = \pi$ with $\sum_i \pi_i = 1$.

### Proof

Step 1 (Markov Chain): The matrix $P = [p_{ij}]$ is stochastic. Non-negativity: $p_{ij} \geq 0$ since $w_{ij} \geq 0$ and $t_j > 0$.

Step 2 (Irreducibility): For any pair $(u, v)$, a path from $u$ to $v$ exists with positive probability. The chain is irreducible.

Step 3 (Uniqueness): Since $P$ is irreducible and aperiodic, the chain is ergodic. By the [[Perron-Frobenius theorem]], a unique stationary distribution $\pi$ exists satisfying $\pi P = \pi$, $\sum_i \pi_i = 1$.

Step 4 (Convergence): By the ergodic theorem, for any initial distribution $\mu^{(0)}$:

$$\pi = \lim_{t \to \infty} \mu^{(0)} \cdot P^t$$

Step 5 (Interpretation): The stationary distribution $\pi$ is a stable [[consensus]] of observation probabilities. Each $\pi_j$ reflects both the [[particle]]'s structural position and the [[neuron]] [[token]] influence. This is the simplest Schelling point everyone can universally agree on.

[Poetic](https://hackmd.io/@mastercyb/poetic-cft) and [rigorous](https://hackmd.io/@mastercyb/rigorous-cft) versions of the proof are available.

---

## Part II: General Case — Composite Contraction

Part I proves convergence for [[diffusion]] alone. The [[tri-kernel]] combines three operators. We prove the composite converges as well.

### The Composite Operator

The [[tri-kernel]] blends [[diffusion]], [[springs]], and [[heat]] into a single update (see [[cyber/tri-kernel]] for full specification):

$$\phi^{(t+1)} = \text{norm}\big[\lambda_d \cdot D(\phi^t) + \lambda_s \cdot S(\phi^t) + \lambda_h \cdot H_\tau(\phi^t)\big]$$

where $\lambda_d + \lambda_s + \lambda_h = 1$, $D$ is the [[diffusion]] step, $S$ is the [[springs]] equilibrium map, $H_\tau$ is the [[heat]] map, and $\text{norm}(\cdot)$ projects to the simplex.

### Contraction Lemmas

Lemma 1 (Diffusion Contracts): Under ergodicity of $P$ with teleport parameter $\alpha \in (0,1)$, the diffusion map $D$ satisfies $\|D\phi - D\psi\|_1 \leq \alpha \|\phi - \psi\|_1$. This follows from Part I: the teleport ensures geometric mixing with rate $\alpha$.

Lemma 2 (Springs Contract): Under screening parameter $\mu > 0$, the screened Laplacian solve $S: \phi \mapsto (L + \mu I)^{-1}(\mu x_0)$ satisfies $\|S\phi - S\psi\|_2 \leq \frac{\|L\|}{\|L\| + \mu} \|\phi - \psi\|_2$. The Green's function $(L + \mu I)^{-1}$ decays exponentially with distance — screening ensures locality and contraction.

Lemma 3 (Heat Contracts): For bounded temperature $\tau > 0$, the heat kernel $H_\tau = \exp(-\tau L)$ satisfies $\|H_\tau \phi - H_\tau \psi\|_2 \leq e^{-\tau \lambda_2} \|\phi - \psi\|_2$ where $\lambda_2$ is the Fiedler eigenvalue. Positivity-preserving and semigroup properties ensure well-defined contraction.

### Theorem (Composite Contraction)

Under ergodicity of $P$, screening $\mu > 0$, and bounded $\tau$, the composite operator $\mathcal{R}$ is a contraction:

$$\|\mathcal{R}\phi - \mathcal{R}\psi\| \leq \kappa \|\phi - \psi\|, \quad \kappa = \lambda_d \alpha + \lambda_s \frac{\|L\|}{\|L\|+\mu} + \lambda_h e^{-\tau\lambda_2} < 1$$

Since each component contracts and $\mathcal{R}$ is a convex combination, $\kappa$ is a convex combination of individual contraction coefficients — each less than 1, hence $\kappa < 1$. By Banach fixed-point theorem, $\phi^t \to \phi^*$ at linear rate.

### Free Energy Minimization

The fixed point $\phi^*$ minimizes:

$$\mathcal{F}(\phi) = \lambda_s\left[\frac{1}{2}\phi^\top L\phi + \frac{\mu}{2}\|\phi-x_0\|^2\right] + \lambda_h\left[\frac{1}{2}\|\phi-H_\tau\phi\|^2\right] + \lambda_d \cdot D_{KL}(\phi \| D\phi)$$

elastic structure + deviation from heat-smoothed context + alignment with diffusion image. this is variational free-energy minimization in the sense of Friston.

### Locality Radius

For edit batch $e_\Delta$, there exists $h = O(\log(1/\varepsilon))$ such that recomputing on the $h$-hop neighborhood $N_h$ achieves global error $\leq \varepsilon$. This follows from: geometric decay ([[diffusion]], teleport), exponential decay ([[springs]], screening), Gaussian tail ([[heat]], kernel bandwidth).

### Reduction

When $\lambda_s = \lambda_h = 0$: $\mathcal{R} = D$, $\kappa = \alpha$, $\mathcal{F}$ reduces to $D_{KL}(\phi \| D\phi)$, and the fixed point is the stationary distribution $\pi^*$ from Part I. The general case subsumes the special case.

---

## Complexity

Memory and computation scale linearly with [[cybergraph]] size:

| Storage Type | Bytes per [[particle]] | Bytes per [[cyberlink]] |
|---|---|---|
| volatile | 56 | 24 |
| persistent | 72 | 128 |

per-iteration complexity: $O(V + E)$

total work to reach precision $\varepsilon$:

$$O\left(\frac{(E + V) \cdot \log(1/\varepsilon)}{\lambda}\right)$$

where $\lambda$ is the spectral gap governing convergence rate. see [[emergence]] for scaling estimates across [[intelligence]] phases

---

## Conclusion

Two results, one framework. Part I establishes that token-weighted [[random walk]] converges to a unique [[collective focus]] — the Schelling point of the [[cybergraph]]. Part II extends this to the full [[tri-kernel]], proving the composite operator contracts and its fixed point minimizes free energy. Together they provide the mathematical foundation for [[egregore]]: a convergent, local, verifiable computation of collective [[intelligence]].

the fixed point π* is a mathematical consequence of three properties: ergodicity ([[diffusion]]), screening ([[springs]]), bounded temperature ([[heat]]). convergence follows from [[Banach]] fixed-point theorem — it is proven, not postulated. no selection principle is needed to pick the "right" state: the contraction mapping leaves exactly one. see [[consistency]] for why this matters and [[locality]] for why it scales.

see [[tri-kernel architecture]] for why these three operators. see [[cyber/tri-kernel]] for the formal specification. see [[bostrom]] for empirical validation

---

## References

1. Perron. "Zur Theorie der Matrices." Mathematische Annalen, 1907
2. Frobenius. "Uber Matrizen aus nicht negativen Elementen." Sitzungsberichte, 1912
3. Levin, Peres & Wilmer. "Markov Chains and Mixing Times." AMS 2009
4. Banach. "Sur les operations dans les ensembles abstraits." Fundamenta Mathematicae, 1922
5. Fiedler. "Algebraic connectivity of graphs." Czech Math Journal, 1973
6. Chung. "The heat kernel as the pagerank of a graph." PNAS 2007
7. Friston. "The free-energy principle: a unified brain theory." Nature Reviews Neuroscience, 2010
8. Spielman. "Spectral Graph Theory." Yale Lecture Notes
