---
tags: mathematics, cyber, core
alias: Perron-Frobenius, Perron root, Perron vector, Perron eigenvector, Frobenius theorem
crystal-type: pattern
crystal-domain: mathematics
crystal-size: bridge
---
every non-negative primitive matrix has a unique positive eigenvector. this is the theorem that guarantees [[focus]] converges

named after [[Oskar Perron]] (1907, positive matrices) and Georg Frobenius (1912, extension to non-negative matrices)

---

## the theorem

three forms, each extending the previous.

### positive matrices (Perron, 1907)

let $A$ be a real $n \times n$ matrix with all entries strictly positive ($A_{ij} > 0$). then:

1. $A$ has a unique real eigenvalue $\rho(A)$ equal to its spectral radius — the Perron root
2. $\rho(A) > |\lambda|$ for every other eigenvalue $\lambda \neq \rho(A)$
3. the corresponding eigenvector $\mathbf{v}$ has all positive entries — the Perron vector
4. $\mathbf{v}$ is unique up to scaling

### non-negative primitive matrices (Frobenius, 1912)

a non-negative matrix $A \geq 0$ is primitive if it is irreducible and aperiodic — equivalently, $A^k > 0$ for some $k$. the same conclusions hold: unique Perron root dominating all others, unique positive Perron vector.

### stochastic chains (the case that matters)

a row-stochastic matrix $P$ (all entries $\geq 0$, each row sums to 1) that is irreducible and aperiodic satisfies:

$$\exists!\; \phi^* \in \Delta^{n-1} : \phi^* P = \phi^*, \quad \phi^*_i > 0 \;\forall\, i$$

$\phi^*$ is the unique stationary distribution. from any starting distribution $\mu^{(0)}$:

$$\left\|\mu^{(0)} P^t - \phi^*\right\|_1 \leq C \cdot (1 - \lambda)^t \to 0$$

where $\lambda$ is the spectral gap $1 - |\lambda_2|$ and $\lambda_2$ is the second-largest eigenvalue by modulus.

---

## why primitive = irreducible + aperiodic

irreducibility: from any state $i$, every state $j$ is reachable with positive probability in finitely many steps. on a directed graph, this is strong connectivity.

aperiodicity: the chain is not periodic — there is no $d \geq 2$ such that the chain always returns to a state in multiples of $d$ steps. a single self-loop is sufficient for aperiodicity. the teleport term $\alpha$ in the diffusion operator guarantees this.

without irreducibility: $\phi^*$ may not be unique (absorbing states create multiple stationary distributions). without aperiodicity: $P^t$ oscillates rather than converges.

---

## application to the [[cybergraph]]

the [[tri-kernel]] composite operator $\mathcal{R}$ acts on probability distributions over [[particles]]. it converges to $\phi^*$ because its induced transition matrix satisfies both conditions.

irreducibility: the graph is strongly connected (any particle reachable from any other through the link structure). the teleport term in the [[diffusion]] operator $\mathcal{D}$ provides a global path between all particle pairs with probability $\alpha > 0$.

aperiodicity: the teleport also breaks periodicity — any state has a positive self-transition probability $\alpha/|P|$.

therefore by Perron-Frobenius:

| property | formal guarantee |
|----------|-----------------|
| [[focus]] $\phi^*$ exists | unique stationary distribution |
| all $\phi^*_p > 0$ | every particle has positive attention |
| $\phi^*$ is independent of initial distribution | any $\phi^{(0)} \to \phi^*$ |
| convergence rate is geometric | $\|\phi^{(t)} - \phi^*\|_1 \leq C(1-\lambda_2)^t$ |

the rate is controlled by the [[spectral gap]] $\lambda_2 = 1 - |\lambda_2(P)|$. larger gap = faster convergence. the gap is bounded below by the [[Cheeger constant]] (graph connectivity) and above by $1$ (trivial convergence in one step). see [[spectral gap]] for the graph-theoretic analysis.

---

## in [[PageRank]] and [[cyberank]]

[[Larry Page]] and [[Sergey Brin]] implicitly invoked Perron-Frobenius in PageRank (1998): the recursive definition "the importance of a page is the sum of importances of pages linking to it" is an eigenvalue equation $\phi^* = \phi^* P$. Perron-Frobenius guarantees this has a unique positive solution when $P$ is irreducible and aperiodic — which the damping factor (teleport parameter) ensures.

[[cyberank]] is PageRank generalized: instead of a flat importance score, the stationary distribution $\phi^*$ over particles weighted by neuron stakes and ICBS market prices. the convergence guarantee is the same theorem, applied to a richer transition matrix.

---

## the Banach fixed-point connection

Perron-Frobenius guarantees existence and uniqueness. the [[collective focus theorem]] (CFT) proves convergence via the Banach fixed-point theorem: the composite operator $\mathcal{R}$ is a contraction with coefficient $\kappa < 1$, so $\mathcal{R}^t \phi \to \phi^*$ for any $\phi$. the two approaches are complementary — Perron-Frobenius via spectral theory, CFT via metric contraction. the spectral gap $\lambda_2$ is the Banach contraction coefficient $\kappa$ viewed from the eigenvalue side.

see [[collective focus theorem]] for the contraction proof. see [[spectral gap]] for the rate analysis. see [[Oskar Perron]] for biography. see [[cyber/focus]] for the engineering implementation.

discover all [[concepts]]
