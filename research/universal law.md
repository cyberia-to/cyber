---
tags: cyber, research, article
crystal-type: entity
crystal-domain: cyber
---
# exponential optimality under constraint

## the principle

when a finite budget of identical quanta must be distributed to minimise a global cost, the optimal solution follows an exponential law with base $e$.

this is not a theorem about a specific system. it is a variational principle that appears whenever:
1. a resource is finite (conservation)
2. a cost must be minimised (optimality)
3. allocation is across distinguishable states (differentiation)

the principle links three classical traditions:
- **least-action paths** in [[mechanics]] — the path that minimises action has exponential weight in the [[path integral]]
- **maximum-[[entropy]]** distributions in statistical physics and [[information theory]] — the distribution that maximises entropy under energy constraint is $\exp(-\beta E)$
- **[[collective focus theorem]]** in cognitive and social systems — attention across ranked items decays as $\exp(-\alpha k)$

all three are special cases of the same variational template: extremise a functional under a sum constraint, get an exponential.

## derivation

let $n$ denote the amount of information to encode and $b$ the radix. the total code cost:

$$c(b, n) = b \times \log_b n$$

has a stationary point at $b = e$, giving the most economical radix. the derivative:

$$\frac{\partial c}{\partial b} = \log_b n - \frac{1}{\ln b} = 0 \implies b = e$$

the result generalises: whenever a constraint has the form "sum of resources is fixed," introducing a Lagrange multiplier $\lambda$ and extremising yields weights proportional to $\exp(-\lambda x)$ — an exponential distribution whose natural base is $e$.

the Lagrangian:

$$\mathcal{L} = \sum_i f(x_i) + \lambda \left( \sum_i x_i - C \right)$$

extremising over $\{x_i\}$ with $f(x) = x \ln x$ (entropy) or $f(x) = x^2$ (energy) or $f(x) = x \cdot \text{cost}(x)$ (attention) always produces:

$$x_i^* \propto \exp(-\lambda \cdot g(i))$$

where $g(i)$ encodes the cost structure. the exponential is the ONLY function that satisfies the maximum entropy principle under linear constraints (Jaynes, 1957).

## where it appears in [[cyber]]

the universal law is not an abstraction that floats above the system. it appears at every layer — from hash function design to consensus convergence to light client sampling.

### [[focus]] and [[tri-kernel]] convergence

the [[tri-kernel]] computes the stationary distribution $\phi^*$ of the [[cybergraph]]. this distribution determines where collective attention goes. the convergence follows exponential mixing:

$$\|\phi^*_t - \phi^*\|_1 \leq C \cdot \lambda_2^t$$

where $\lambda_2$ is the second-largest eigenvalue (the [[spectral gap from convergence|spectral gap]]). convergence speed is exponential in time. the universal law predicts this: the optimal path to the fixed point (least action) has exponential weight.

the resulting $\phi^*$ itself follows a power law (Zipf) — which is log-linear, the discrete shadow of the exponential on a rank axis. see [[collective focus theorem]].

### [[temporal decay]]

axon weights decay exponentially:

$$w_{\text{eff}}(t) = A_{pq} \cdot \alpha^{t - t_{\text{last}}}$$

this IS the universal law applied to attention over time. given finite focus, the optimal allocation to past events decays exponentially — recent events carry exponentially more weight than old ones. the decay rate $\alpha$ is the system's "temperature" for temporal attention.

### [[pi-weighted replication]]

storage replication factor is proportional to $\phi^*$ (cyberank). particles follow a power law:

```
top-100 particle:    φ* ~ 10⁻²    → R = 1000 replicas
median particle:     φ* ~ 10⁻⁶    → R = 10 replicas
tail particle:       φ* ~ 10⁻¹²   → R = 3 replicas (minimum)
```

given finite storage across the network, the exponential allocation minimises total retrieval cost. storing everything equally wastes resources on content nobody queries. storing proportionally to $\phi^*$ is the maximum-entropy allocation under the storage budget constraint.

### [[gravity commitment]]

mass-weighted polynomial encoding: high-$\phi^*$ particles occupy low-degree polynomial terms (cheap to verify), low-$\phi^*$ particles occupy high-degree terms (expensive).

```
hot layer (top 2⁸):     8 rounds, ~2 KiB proof, ~50 μs
warm layer (next 2¹²):   12 rounds, ~5 KiB proof, ~200 μs
cold layer (remaining):  20 rounds, ~12 KiB proof, ~500 μs
```

the verification cost is exponential in layer depth. the allocation of data to layers follows the exponential principle: the optimal layering minimises expected verification cost under the constraint that all data must be accessible.

### [[DAS|data availability sampling]]

confidence grows exponentially with sample count:

$$P(\text{available}) \geq 1 - (1/2)^k$$

at $k = 20$: 99.9999% confidence. this is the information-theoretic consequence of the universal law — each independent sample provides one bit of evidence, and evidence compounds exponentially. the optimal number of samples balances linear sampling cost against exponential confidence gain.

### [[sumcheck]] bookkeeping

the [[sumcheck]] prover maintains a table that halves each round:

```
round 1:  2^k entries
round 2:  2^{k-1} entries
...
round k:  1 entry
total:    2^{k+1} - 1 = O(N)
```

the geometric series $\sum 2^{-i}$ converges to 2 — the same exponential convergence that makes the prover O(N). the halving IS the exponential principle applied to computational resources: each round does exponentially less work than the previous, and the total is bounded by twice the first round.

### [[HyperNova]] folding

each fold compresses two CCS instances into one with ~30 field operations. after $N$ folds:

- information compressed: $N$ instances → 1 accumulator
- computational cost: $30N$ field operations
- verification: one decider ($\sim$12K constraints)

the compression ratio is exponential: $N$ instances in $O(N)$ work, verified in $O(1)$. the universal law appears as the gap between linear cost and exponential compression — the same gap that makes entropy encoding efficient.

### [[Hemera]] algebraic degree

the $x^{-1}$ S-box achieves algebraic degree $2^{1046}$ over 24 rounds. each round multiplies the algebraic degree — exponential growth in algebraic complexity with linear cost in rounds. the security margin grows exponentially while the constraint count grows linearly. this is the universal law applied to cryptographic hardness: the optimal S-box maximises algebraic degree (security) per constraint (cost).

### [[VDF]] rate limiting

an adversary flooding $N$ signals pays $N \times T_{\min}$ sequential wall-clock time. the honest network processes one signal per $T_{\min}$. the adversary's cost grows linearly but cannot be parallelised. the damage potential of $N$ fake signals grows at most linearly (one state corruption per signal), while detection probability grows exponentially with the number of honest observers checking hash chains.

### tree branching factor

binary trees ($b = 2$) are standard. ternary trees ($b = 3$) are closer to the optimal base $e \approx 2.718$. the universal law predicts: cost per lookup $= b \times \log_b n$ is minimised at $b = e$. in practice:

```
b = 2:  cost = 2 × log₂ n = 2 × 1.443 × ln n = 2.886 × ln n
b = 3:  cost = 3 × log₃ n = 3 × 0.910 × ln n = 2.731 × ln n  ← closest integer
b = 4:  cost = 4 × log₄ n = 4 × 0.721 × ln n = 2.885 × ln n
```

ternary is 5.4% more efficient than binary or quaternary. this appears in Verkle tree branching factor selection — wider branching trades depth for node cost, with the optimum near 3.

## the unified Lagrangian

the open question from the original formulation: can a unified Lagrangian encompass action, entropy, and attention within one functional?

proposal:

$$\mathcal{L}[\rho] = \int \left[ \rho(x) \ln \rho(x) + \lambda_1 \rho(x) V(x) + \lambda_2 |\nabla \rho(x)|^2 \right] dx$$

where:
- $\rho(x) \ln \rho(x)$ = entropy (information cost)
- $\rho(x) V(x)$ = potential energy (physical cost)
- $|\nabla \rho(x)|^2$ = gradient penalty (smoothness / attention cost)

extremising yields:

$$\rho^*(x) \propto \exp\left(-\lambda_1 V(x) - \lambda_2 \nabla^2 \ln \rho^*(x)\right)$$

in the limit $\lambda_2 \to 0$: Boltzmann distribution (physics).
in the limit $V \to \text{rank}$: Zipf/exponential focus (attention).
in the limit $V \to \text{Hamiltonian}$: Feynman path weight (mechanics).

the three traditions are not analogies. they are the same functional with different cost functions plugged into $V(x)$.

## applications beyond [[cyber]]

- **physics**: Boltzmann factors $\exp(-\beta E)$ weight microstates in canonical ensembles
- **coding**: radix-3 positional notation approximates base-$e$ efficiency with integers
- **[[compression]]**: Huffman codes converge to exponential length distributions under uniform symbol cost
- **cognition**: softmax routing in neural networks implements exponential [[focus]] with learnable [[temperature]]
- **computation**: qutrit logic approaches base-$e$ density more closely than qubit logic
- **interfaces**: systems respecting exponential focus allocation reduce cognitive overload
- **data structures**: B-tree branching factors near 3 minimise lookup depth per unit node cost

## conclusion

exponential optimality under constraint is the thread connecting:
- why [[Hemera]]'s S-box degree grows as $2^{1046}$ (security per round)
- why [[tri-kernel]] $\phi^*$ converges exponentially (mixing time)
- why [[DAS]] gives $1 - 2^{-k}$ confidence (evidence per sample)
- why [[sumcheck]] is O(N) (halving per round)
- why [[temporal decay]] is $\alpha^t$ (relevance over time)
- why [[gravity commitment]] layers by $\phi^*$ (verification per importance)
- why replication follows $\phi^*$ (storage per attention)
- why Verkle branching near 3 is optimal (cost per lookup)

one principle. not separate stories but chapters of one law: efficient differentiation when resources are bounded, the optimal allocation is always exponential.

see [[collective focus theorem]] for the attention case, [[spectral gap from convergence]] for the convergence rate, [[tri-kernel architecture]] for the computation, [[structural-sync]] for the system these laws govern, [[cyber/research/zheng vs starks|zheng]] for the proof system where sumcheck and folding embody the principle
