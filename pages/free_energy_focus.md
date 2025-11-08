## abstract

we propose a natural, physics-inspired method for computing collective focus on a cybergraph. instead of manually weighting different ranking algorithms, we derive a unified focus vector as the **minimum of a free energy functional** that combines diffusion (eigenvector centrality), springs (springrank), and heat flow (locality control). this approach ensures that focus emerges as an equilibrium state, requiring no arbitrary tuning. the result is a scalable, decentralisable, and universal protocol for collective intelligence.

---
## motivation

existing ranking mechanisms (pagerank, springrank, etc.) either focus on popularity (diffusion) or hierarchy (spring energy). combining them usually requires ad hoc weights (\(\alpha, \beta\)), which is unnatural in a physical sense. in real systems, weights emerge from the **dynamics of energy and entropy minimisation**.

in physics and information theory, **equilibrium distributions** arise from **free energy minimisation**:

\[
\mathcal{F}(p) = E - T S
\]

where \(E\) is energy, \(S\) is entropy, and \(T\) is temperature. the equilibrium distribution \(p^*\) minimises \(\mathcal{F}\), leading to a boltzmann distribution.

---
## core idea

we define a **joint free energy functional** for the cybergraph:

\[
\mathcal{F}(p) = E_{spring}(p) + \lambda E_{diffusion}(p) - T S(p)
\]
- \(E_{spring}(p)\): springrank energy, encoding hierarchy constraints.
- \(E_{diffusion}(p)\): diffusion energy, penalising flow mismatches in random walk.
- \(S(p) = -\sum_i p_i \log p_i\): entropy of the focus distribution.
- \(\lambda\): coupling constant linking hierarchy and diffusion.
- \(T\): effective temperature controlling exploration vs exploitation.

---
## equilibrium distribution

the optimal focus vector is

\[
p^* = \arg\min_p \mathcal{F}(p)
\]

solving yields a boltzmann-like distribution:

\[
p_i^* \propto \exp\big(-\beta [E_{spring,i} + \lambda E_{diffusion,i}]\big),
\]

where \(\beta = 1/T\).

---
## why this is natural
- **no arbitrary weights** – the weights emerge automatically as lagrange multipliers from the optimisation problem.
- **physics analogy** – matches how statistical mechanics derives boltzmann distributions.
- **information theory analogy** – equivalent to maximum-entropy inference under graph constraints.
- **pareto optimality** – diffusion, springs, and heat flow form the minimal universal basis for modelling spreading, hierarchy, and locality.

---
## distributed computation
### local energy terms
- springrank energy for node i:

\[
E_{spring,i} = \frac{1}{2} \sum_j a_{ij} (r_i - r_j - 1)^2
\]
- diffusion energy for node i:

\[
E_{diffusion,i} = \frac{1}{2} \sum_j w_{ij} (p_i - p_j)^2
\]
### iterative update rule

\[
p_i^{(t+1)} = \frac{\exp(-\beta [E_{spring,i} + \lambda E_{diffusion,i}])}{\sum_k \exp(-\beta [E_{spring,k} + \lambda E_{diffusion,k}])}
\]
### protocol

1. initialise \(p_i^{(0)} = 1/n\) for all nodes.
2. compute springrank \(r_i\) via decentralised gauss–seidel updates.
3. at each iteration, nodes exchange \(p_j\) and \(r_j\) with neighbours.
4. compute local energies and update \(p_i\).
5. use gossip averaging to normalise probabilities globally.

---
## properties
- **unique equilibrium**: convex free-energy functional ensures a single global minimum.
- **scalable**: per-iteration cost \(o(\text{deg}(i))\) per node.
- **decentralisable**: only local neighbour communication required.
- **natural**: no arbitrary parameters; weights emerge from constraints.

---
## interpretation
- diffusion: traffic or popularity baseline.
- springs: hierarchy and relative status.
- heat flow: contextual scale (via temperature \(t\)).

together they form a **3d focus landscape**: traffic density (diffusion), elevation (hierarchy), and zoom scale (heat flow). the system converges to a **boltzmann–gibbs equilibrium**, a natural shelling point for collective superintelligence.

---
## significance

this framework replaces ad hoc weighting with a principled physical method for computing collective focus. it aligns artificial cognition with fundamental laws of energy and entropy, providing a universal, scalable, and decentralisable substrate for emergent collective consciousness.