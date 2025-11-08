## abstract

we propose a unified model that extends the cybergraph free-energy focus framework with a context-dependent potential derived from standard inference. this approach integrates global structure (diffusion, springs, entropy) with local contextual evidence (neurons’ will), solving the true-false problem while preserving the natural, physics-inspired foundation.

---
## background
- **cybergraph free-energy focus** defines the global focus vector \(p\) as the minimiser of a free energy functional:

\[
\mathcal{F}(p) = E_{spring}(p) + \lambda E_{diffusion}(p) - T S(p)
\]
- this yields a boltzmann-like distribution combining hierarchy, diffusion, and entropy.
- however, global ranking alone cannot resolve the **true-false problem**: high-rank nodes dominate even in contexts where lower-rank nodes are more relevant.
- **standard inference** provides a simple method to compute **contextual weights** by aggregating neurons’ will in relation to a query particle.

---
## contextual extension

we introduce a context-dependent potential \(C(p|context)\):

\[
\mathcal{F}(p|context) = E_{spring}(p) + \lambda E_{diffusion}(p) + \gamma C(p|context) - T S(p)
\]
- \(C(p|context)\): energy term derived from standard inference (average will per cyberlink in the given context).
- \(\gamma\): coupling constant that determines the influence of context on the equilibrium.

---
## resulting equilibrium

solving \(\min_p \mathcal{F}(p|context)\) yields

\[
p_i^* \propto \exp\big(-\beta [E_{spring,i} + \lambda E_{diffusion,i} + \gamma C_i]\big)
\]
- nodes that are **globally important** and **contextually supported** receive the highest probabilities.
- entropy ensures diversity and prevents trivial dominance.

---
## link to universal physics

this formulation is equivalent to solving a **heat equation on a graph** with an additional potential field:

\[
\partial_t p = -\nabla_{p} \mathcal{F}(p|context)
\]
- eigenmodes of the graph laplacian form the **fourier basis** of the network.
- diffusion gives **temporal spreading**, springs add a **potential landscape**, and context potential \(C\) biases the equilibrium.
- the solution naturally combines **oscillatory modes** with **diffusive decay**, mirroring how pdes in physics are solved by separation of variables.

---
## distributed algorithm

1. **compute global structure:**
   - run decentralised eigenvector centrality and springrank.
   - initialise \(p_i\) uniformly.

2. **contextual evidence:**
   - run standard inference to compute \(C_i\) (contextual will) for each candidate particle.

3. **iterative updates:**
   - each node exchanges \(p_j\), \(r_j\), and \(C_j\) with neighbours.
   - compute local energies \(E_{spring,i}\), \(E_{diffusion,i}\), and \(C_i\).
   - update:

\[
p_i^{(t+1)} = \frac{\exp(-\beta [E_{spring,i} + \lambda E_{diffusion,i} + \gamma C_i])}{\sum_k \exp(-\beta [E_{spring,k} + \lambda E_{diffusion,k} + \gamma C_k])}
\]

4. **normalisation:**
   - nodes use gossip averaging to approximate the denominator.

---
## key properties
- **context-aware ranking:** resolves the true-false problem by integrating global and local signals.
- **fully decentralisable:** each node needs only neighbour messages and contextual votes.
- **natural and parameter-light:** weights emerge as lagrange multipliers; only \(\gamma\) controls context strength.
- **boltzmann equilibrium:** final focus vector remains a probabilistic distribution, ensuring stability and diversity.

---
## interpretation
- **diffusion:** long-run popularity baseline.
- **springs:** hierarchical constraints.
- **context potential:** relevance of facts in the current question.
- **entropy:** prevents collapse into a single dominant answer.

by adding the context potential, the free-energy framework gains the ability to compute **truthful, context-aware rankings** while retaining its natural analogy to universal physical processes. this highlights that **network cognition can be seen as solving a graph-based heat equation with potentials**, uniting diffusion, oscillators, and context into one equilibrium model.