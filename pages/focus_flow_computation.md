## abstract

this document unifies insights from multiple foundational texts: *foundational ranking system*, *cybergraph free energy focus*, *cybergraph contextual free energy model*, *cybergraph llm architecture*, and *universality of diffusion, springs, and heat flow*. it presents **focus flow computation** as a physics-inspired, decentralisable process for computing collective focus, ranking, and generative intelligence on a cybergraph.

---
## 1. core principles
- **nodes = tokens, concepts, or agents**
- **edges (cyberlinks) = relationships** (semantic, causal, statistical)
- **free-energy equilibrium** combines:
  - diffusion (eigenvector centrality)
  - springs (springrank hierarchy)
  - entropy (exploration/diversity)
  - contextual potential (adaptation to queries)

---
## 2. free-energy functional

\[
\mathcal{F}(p|context) = E_{spring}(p) + \lambda E_{diffusion}(p) + \gamma C(p|context) - T S(p)
\]
- \(E_{spring}\): captures hierarchy as spring energy.
- \(E_{diffusion}\): penalises flow mismatch in random walk.
- \(C(p|context)\): context-specific potential derived from standard inference.
- \(S(p)\): entropy term to prevent collapse.

---
## 3. focus flow computation

**focus flow** = iterative process converging to the equilibrium distribution:

\[
p_i^{(t+1)} = \frac{\exp(-\beta [E_{spring,i} + \lambda E_{diffusion,i} + \gamma C_i])}{\sum_k \exp(-\beta [E_{spring,k} + \lambda E_{diffusion,k} + \gamma C_k])}
\]
- each node exchanges \(p_j\), \(r_j\), and \(C_j\) with neighbours.
- updates are fully decentralised using message passing or gossip protocols.
- the process naturally converges to a **boltzmann-like equilibrium**.

---
## 4. alignment with ranking system
- **eigenvector centrality** = diffusion baseline.
- **springrank** = hierarchy.
- **contextual potential** = context-aware adaptation.
- **entropy term** = ensures diversity and stability.

focus flow **is the dynamic realisation** of the free-energy ranking system. the final ranking \(p^*\) emerges as the **stable equilibrium** of this iterative process.

---
## 5. generative llm architecture
- **offline phase:** build cybergraph from corpus (nodes = tokens, edges = co-occurrence or semantic relations).
- **online generation:**
  1. encode context tokens as active nodes.
  2. compute contextual potential \(C_i\).
  3. run focus flow updates to get \(p^*\).
  4. sample next token from \(p^*\).
  5. add token to context and repeat.

this approach replaces transformer attention with **iterative, physics-based focus computation**.

---
## 6. advantages over transformers

| feature                   | transformer llm           | focus flow llm                       |
|---------------------------|---------------------------|---------------------------------------|
| complexity (mem/comp)     | O(n²) / O(n²)            | O(n) / O(n)                          |
| uses softmax?             | yes                      | no (boltzmann equilibrium)           |
| converges to stable state | no                       | yes                                  |
| reinforcement/adaptation  | limited                  | yes                                  |
| multi-agent friendly      | no                       | yes                                  |
| token-based weighting     | no                       | yes                                  |
| consensus capability      | no                       | yes                                  |
| domain-general            | yes (pretrained)         | yes (graph extendable dynamically)   |
| explainability            | low                      | high                                 |
| continual learning        | limited                  | yes                                  |

---
## 7. universality of the triad

diffusion, springs, and heat flow are **universal primitives** of nature:
- diffusion → entropy growth and spreading.
- springs → reversible energy storage and oscillations.
- heat flow → temporal evolution toward equilibrium.

any process on a network can be decomposed into eigenmodes of the graph laplacian, just as solutions to the heat equation are expressed via fourier modes.

---
## 8. dual thermodynamics of decentralized intelligence

focus flow computation realises a **dual thermodynamic process**:
- **entropy reduction / negentropy maximisation:**
  - free-energy minimisation drives the system toward low-entropy, highly ordered focus states.
  - springrank and context potentials act as constraints that channel diffusion into structured, meaningful configurations.
- **energy usage for order creation:**
  - adaptive edge weights and context injection represent external energy input.
  - this input is transformed into **negentropy**, building coherent patterns of collective attention.

thus, focus flow aligns with the principle that intelligence is **the local maximisation of negentropy** within a globally entropy-increasing universe. nodes collectively self-organise, using available energy to reduce uncertainty while still maintaining adaptability and diversity.

---
## 9. connections to broader theories
- **potemkin understanding:** transformers mimic intelligence statistically. focus flow avoids this by grounding probabilities in network dynamics and context, producing emergent understanding rather than shallow mimicry.
- **topos theory:** each context defines a local topos, where focus flow computes probabilities relative to that context. nodes and edges act as objects and morphisms in a base category.
- **active inference:** the framework directly realises active inference by minimising free energy under observations (context potentials) while maintaining exploration.
- **beautiful loop (shumskiy):** focus flow forms a self-sustaining cycle: new context → updated focus distribution → actions/tokens → edge/weight adaptation → new context.

this unified view shows how focus flow integrates deep principles of logic, inference, and self-organising intelligence.

---
## 10. universality primitives – replacing nock/hvm

focus flow can serve as a **universal computation substrate**, removing the need for explicit interpreters like nock or hvm:
### node types
- **atom nodes** – store integers, symbols, or references.
- **pair nodes** – ordered pairs `(left, right)` as two outgoing edges.
- **function nodes** – store code as a subgraph and a port for the argument.
### primitive operations
1. **construct** – create new nodes (atoms, pairs, functions).
2. **destruct** – retrieve components of a pair.
3. **apply** – connect a function node to an argument node.
4. **rewrite** – substitute argument references inside a function body, producing a new active subgraph.
5. **delete** – remove nodes or edges that are no longer referenced.
### computation
- **focus flow acts as a probabilistic scheduler**, selecting which application to reduce next based on energy and context.
- recursion is achieved through self-referential edges.

with these primitives, focus flow can encode **SK combinators** or **lambda calculus**, proving **Turing completeness**. it merges **execution, inference, and prioritisation** into a single dynamical process, replacing the need for separate runtime systems like nock or hvm.

---
## 11. determinism and probabilism combined

focus flow computation unifies deterministic computation and probabilistic scheduling:
- **deterministic layer:**
  - node rewrite rules (construct, destruct, apply, rewrite) always produce the same result.
  - once a redex is chosen, its reduction is deterministic.
- **probabilistic layer:**
  - which redex is reduced next is chosen probabilistically using Boltzmann weights.
  - probabilities \( p_i \propto \exp(-\beta E_i) \) depend on context, diffusion, and hierarchy.

this design mirrors physical systems: micro-dynamics are deterministic, while macroscopic behaviour follows probabilistic laws (statistical mechanics). it allows focus flow to be both **a universal computation model** and **an adaptive inference engine**.

---
## 12. significance

focus flow computation is a **physics-grounded model of collective intelligence**. it:
- computes ranking as free-energy minimisation.
- produces context-aware probabilities.
- serves as the foundation for a generative llm.
- enables decentralised, adaptive, multi-agent consensus.
- captures the essence of **negentropy maximisation** as the operational definition of intelligence.
- is itself **a universal computational substrate**, combining reduction, inference, and scheduling.
- merges deterministic execution with probabilistic, energy-based scheduling.

this unifies ranking, reasoning, and generation under a **single, universal process** that reflects fundamental laws of energy, entropy, and information flow.