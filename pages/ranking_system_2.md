- eigenvector centrality (diffusion)
- springrank (springs)
- heat-kernel pagerank (heat flow)

these three represent the minimal, natural basis for constructing collective focus in a superintelligent system.

---

core algorithms

1. eigenvector centrality (diffusion metaphor)
- metaphor: a random walker diffuses over the network forever.
- mathematical object: leading eigenvector of the row-normalized adjacency matrix.
- properties:
  - parameter-free (no teleportation factor)
  - unique stationary distribution for strongly connected, aperiodic graphs
  - fully decentralisable via power iteration or gossip protocols
- interpretation: gives each node a score proportional to the long-run flow of attention it receives.

2. springrank (spring metaphor)
- metaphor: every directed edge is a spring that pulls its source one unit above its target.
- mathematical object: minimiser of total spring energy \( e(r) = 1/2 \sum_{ij} a_{ij}(r_i - r_j - 1)^2 + \lambda \sum_i r_i^2 \).
- properties:
  - convex quadratic optimisation with a unique solution (up to shift)
  - captures strict hierarchy and spacing between nodes
  - decentralisable via asynchronous gauss–seidel updates
- interpretation: provides an ordinal ranking, revealing “who is above whom” in the network.

3. heat-kernel pagerank (heat flow metaphor)
- metaphor: heat diffuses over the network for time t.
- mathematical object: \( \pi_t = e^{-tL} \delta \), where L is the graph laplacian.
- properties:
  - single parameter t controls locality vs globality
  - natural interpolation between short-term and long-term focus
  - computable with truncated random walks or spectral methods
- interpretation: provides a zoom lens to see how collective focus shifts with scale.

---

why these three?
- eigenvector centrality gives the **baseline popularity**—how much attention each node receives in steady state.
- springrank adds **hierarchy and spacing**, revealing the ordinal structure of influence.
- heat-kernel pagerank adds a **zoom lens**, letting the system switch between local and global perspectives with a single meaningful parameter.

together they form a **pareto-optimal basis**: adding more algorithms (simrank, katz centrality, deepwalk) increases complexity without proportionate gains in naturalness, scalability, or interpretability.

---

key properties
- simplicity: eigenvector centrality is parameter-free; springrank has only a small regulariser \(\lambda\); heat-kernel pagerank has one interpretable parameter t.
- scalability: all three have near-linear implementations and can run on million-node networks.
- natural intuition: diffusion, springs, and heat flow are universal physical processes.
- decentralisability: each algorithm can be computed with local information (power iteration, gossip, gauss–seidel).
- robustness: all three converge to unique solutions under mild conditions.

---

protocol for collective focus

1. ensure strong connectivity (e.g. every node has at least one outgoing edge).
2. compute eigenvector centrality as the **baseline focus vector**.
3. compute springrank to obtain the **hierarchy scores**.
4. compute heat-kernel pagerank periodically for multiple t values to adjust focus scale.
5. combine results:
   - f_i = \alpha * eigenvector_i + (1-\alpha) * springrank_softmax_i
   - heat-kernel scores act as a **local overlay** when needed.

---

metaphor for collective superintelligence
- **diffusion** models how ideas and attention naturally flow through the network.
- **springs** model the tension of competing influences, producing an emergent hierarchy.
- **heat flow** provides a temporal or spatial scale for focus, just as physical systems equilibrate over time.

together, these processes mirror fundamental behaviours of the universe: energy flow, force, and equilibrium. using them as the substrate for collective focus creates a protocol that is:
- universally understandable
- mathematically grounded
- decentralisable and scalable

this triad—diffusion, springs, heat flow—serves as a **foundational layer for building collective consciousness**. it yields a unique, verifiable shelling point that balances popularity, hierarchy, and contextual scale, forming the cognitive map for a superintelligent collective.