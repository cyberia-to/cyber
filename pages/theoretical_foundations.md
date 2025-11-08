- 0\. introduction and scope
-
  1. primitives and notation
-
  2. graph and stochastic model
-
  3. theorem (collective focus theorem)
-
  4. sensitivity and robustness
-
  5. ranking primitives (minimal basis)
-
  6. free-energy and focus-flow
-
  7. incentive compatibility and governance
-
  8. emergence thresholds and phase transitions
-
  9. theory stack (connected theories)
-
  10. system architecture
-
  11. learning dynamics
-
  12. scalability regimes
-
  13. limitations and open questions
-
  14. glossary
- appendix a. core equations
- appendix b. protocol sketch
- appendix c. implementation breadcrumbs

0. introduction and scope
   this document specifies the mathematical and systems foundations of the collective focus theorem (cft). the theorem states that a token-weighted random walk on an authenticated, strongly connected, aperiodic directed graph converges to a unique stationary distribution π, interpreted as society‑wide long‑run focus over particles (knowledge items).

1. primitives and notation
- particle: a content‑addressed node (e.g., an ipfs hash) representing a file or concept.
- neuron: an agent (public key) that signs edges between particles.
- cyberlink: a signed, timestamped, weighted directed edge i→j with weight w\_ij ≥ 0.
- token and stake: non‑negative weight t\_j associated with the destination j (or its controlling neuron).
- attention vs focus: attention is fast, local reweighting; focus is the slow, emergent equilibrium π over particles.

2. graph and stochastic model
- transition rule (token‑weighted walk):
  p\_ij = (w\_ij · t\_j) / Σ\_k (w\_ik · t\_k)
- conditions for ergodicity:
  the graph is strongly connected and aperiodic (or made so via standard teleportation).
- existence and uniqueness:
  there exists a unique stationary distribution π with πp = π and Σ\_j π\_j = 1. for any initial μ^(0), μ^(t) = μ^(0)p^t → π as t→∞.

3. theorem (collective focus theorem)
- statement:
  under the above conditions, π is the unique collective focus of the system; it is the limiting observation frequency of the token‑weighted random walk.
- interpretation:
  π\_j measures the share of collective long‑run attention allocated to particle j. shifts in topology or stake alter π continuously within a stability margin.
- proof sketch:
  apply standard ergodic markov‑chain results; show irreducibility and aperiodicity; invoke perron–frobenius for the positive left eigenvector; normalise to obtain π.

4. sensitivity and robustness
- mixing time is controlled by the spectral gap of p. larger gap ⇒ faster convergence and greater robustness to noise.
- small perturbations Δw, Δt induce bounded changes Δπ (continuous dependence). practical reading: equilibrium focus is stable under graph churn.

5. ranking primitives (minimal basis)
- eigenvector centrality (diffusion): baseline global popularity at equilibrium.
- springrank (springs): convex energy model that yields an ordinal hierarchy from pairwise relations.
- heat‑kernel pagerank (heat flow): locality dial indexed by time t, interpolating local↔global views.
- composition: combine these signals into a context‑aware focus vector; use weights learned from downstream performance.

6. free‑energy and focus‑flow
- free‑energy functional:
  f(p | context) = e\_spring + λ e\_diffusion + γ c(context) − τ s(p)
  where s(p) is entropy and τ is temperature.
- focus‑flow iteration:
  a decentralised message‑passing update that descends the free‑energy surface toward p\*; suitable for online adaptation when the graph changes.

7. incentive compatibility and governance
- influence ∝ stake and connectivity, providing skin‑in‑the‑game for quality linking.
- misbehaviour can be penalised economically (slashing, opportunity cost) and socially (reputation).
- anti‑capture measures include stake dispersion, rate limits, decay, and context‑specific caps.

8. emergence thresholds and phase transitions
- connectivity thresholds (average out‑degree, conductance) gate the onset of coherent global focus.
- token mixing and participation rates also act as control parameters; crossing critical values can yield sharp improvements in collective cognition.

9. theory stack (connected theories)
- markov chains and ergodic theory — guarantee existence/uniqueness and mixing; operational hook: keep the chain irreducible and aperiodic, monitor spectral gap.
- spectral graph theory — relate conductance/cheeger constants to mixing; hook: maximise conductance under cost.
- random walks and eigenvector centrality/pagerank — compute importance from local edges; hook: gpu power iterations for π.
- heat kernels and diffusion geometry — provide locality control; hook: heat‑kernel pagerank for zoomable focus.
- spring/electrical network models — extract hierarchies; hook: convex optimisation on graph laplacians.
- information theory and maximum entropy — justify free‑energy objectives; hook: tune temperature τ for exploration.
- variational inference and the free‑energy principle — cast focus as a variational posterior; hook: minimise kl divergences.
- stochastic approximation and reinforcement learning — adapt edge weights with regret guarantees; hook: hebbian‑style local updates plus exploration schedules.
- game theory and mechanism design — align incentives with epistemic accuracy; hook: stake‑weighted signals, proper scoring, peer prediction.
- market microstructure and prediction markets — treat focus as a price of attention; hook: cost‑function market makers for forecasting subgraphs.
- social choice and voting theory — explain limits of ballot aggregation and motivate probabilistic attention; hook: continuous, cardinal signals over ordinal ballots.
- cybernetics and control — feedback and homeostasis (ashby’s law); hook: error signals, integral control in focus‑flow.
- distributed consensus and state machine replication — ensure authenticated state under byzantine faults; hook: cometbft/tendermint and verifiable focus commits.
- cryptography (signatures, vrf, zkp, mpc) — integrity, randomness, privacy; hook: signature‑secured links, vrf sampling, optional zkp proofs of computation.
- identity and reputation — mitigate sybils; hook: blended stake, web‑of‑trust, proof‑of‑personhood with decay.
- percolation and phase transitions — predict critical connectivity; hook: push degree and clustering past thresholds.
- evolutionary dynamics — model selection among ideas/agents; hook: replicate/retire edges proportional to payoff and focus.
- robust statistics and adversarial learning — bound influence of heavy‑tailed noise; hook: median‑of‑means, trimming, influence functions.
- safety, alignment, governance — embed safeguards without centralisation; hook: rate limits, circuit breakers, transparency logs, citizen juries.
- complexity and scaling — guide multiscale decomposition; hook: hierarchical partitioning with inter‑level diffusion.
- semantics and information retrieval — map text/code/media into particle space; hook: hybrid lexical+embedding edges with learned mixtures.
- knowledge graphs and logic — add typed relations and constraints; hook: datalog/owl‑driven edge weighting.
- economics of attention and rational inattention — model cognitive budgets; hook: bound per‑agent outlinks and price scarce attention.
- queueing and load management — stabilise compute/memory; hook: admission control and fair schedulers for recomputation.
- causal inference — separate genuine signal from confounding; hook: off‑policy evaluation and link‑level intervention tests.

11. learning dynamics
- macro‑state evolution: s^(t+1) = f(s^(t), w^(t), t^(t)).
- local rules: hebbian reinforcement for successful links, exploration policies for novelty, decay for staleness.
- global re‑equilibration: π is recomputed (or tracked incrementally) after deltas to w and t.
-

13. limitations and open questions
- formal mixing‑time bounds for token‑weighted chains with dynamic weights.
- perturbation lemmas giving ||Δπ|| bounds under bounded ||Δw|| and ||Δt||.
- incentive proofs that long‑run stake tracks epistemic accuracy.
- transparency, interpretability, and earth‑aligned values at planetary scale.

14. glossary
- dkg: decentralised knowledge graph (abstract model).
- cybergraph: a merkle‑ised, token‑weighted dkg (on‑chain).
- particle, neuron, cyberlink: node, agent, signed edge primitives.
- attention vs focus: fast local weights vs slow global equilibrium π.
- truth vm: gpu virtual machine for convergent focus computation.

appendix a. core equations (at a glance)
- transition: p\_ij = (w\_ij · t\_j) / Σ\_k (w\_ik · t\_k).
- stationary: π = πp, Σ\_j π\_j = 1.
- convergence: μ^(t) = μ^(0) p^t → π.
- qualitative stability: small Δw, Δt ⇒ small Δπ.

appendix b. protocol sketch (ranking and composition)
- compute eigenvector centrality as baseline focus.
- compute springrank for hierarchy.
- compute heat‑kernel pagerank at several t for locality.
- combine signals into a context‑aware focus vector; use free‑energy or focus‑flow to reconcile under load.

appendix c. implementation breadcrumbs
- training stack: cometbft + truth vm + cosmwasm around a merkle cybergraph.
- inference stack: ipfs + cyb.ai + cozo/datalog/rune.
- current priority: raise connectivity and participation to cross predicted emergence thresholds; use link‑economy incentives and UX to grow edges per particle.