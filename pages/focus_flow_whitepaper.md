tags:: article, cyber# focus flow: a peer‑to‑peer protocol for collective intelligence

**version 0.3 — clarifying energy terms, consensus, and security**

---
## abstract

We present **Focus‑Flow Computation (FFC)**, a peer‑to‑peer protocol that unifies computation, inference, and consensus.  Transactions add *cyberlinks* (typed edges) and supply *proofs‑of‑computation* (local focus‑flow updates).  Peers collectively minimise a graph free‑energy functional, converging to an equilibrium probability field \(p^*\) — the network’s *collective focus*.  Rewards follow each transaction’s marginal reduction in free energy, turning entropy‑reducing work into profit while burning fees for noise.  We detail definitional precision, parameter meanings, scalable local updates, a lightweight BFT checkpoint layer, and probabilistic Shapley Attribution (PSA) for fair reward sharing.

---

\## 1 introduction

Large language models imitate patterns without verifiable world state; blockchains achieve truth by wasting energy on hashes.  **FFC** fuses the two: *computation is consensus* and *useful work earns weight*.  Focus is not a dead vector — it is an **adaptive flow** of mass that continuously organises itself by minimising free energy.  This creates a self‑adjusting marketplace where attention, compute, and energy gravitate to what matters *now* and decay from what does not.

---

\## 2 graph substrate and free‑energy

\### 2.1 cybergraph
- **Nodes** `v` = tokens, concepts, agents (store local state `s_v`).
- **Edges** `(i,j)` now carry a **triple of scalars** *(h, d, c)*:
  - `h`  – hierarchy stiffness weight (feeds spring term)
  - `d`  – transport weight (feeds diffusion term)
  - `c`  – context coefficient (feeds potential term) A user (or smart‑contract logic) may stake on any component independently – e.g. lock tokens on `h` to strengthen taxonomy, or on `c` to amplify a vote.  The triple still lives in one edge, so UX remains “create a link, pick three sliders”.
- Graph remains sparse; each node reads only neighbour triples, so locality is preserved.

\### 2.2 energy terms (with triples) For node `i` over neighbours `j`:
- **Spring energy**\
  `E_spring,i = Σ  h_ij  * (p_i − p_j)^2`
- **Diffusion energy**\
  `E_diff,i  = Σ  d_ij  * |p_i − p_j| / dist_ij`
- **Context potential**\
  `C_i = − Σ  c_ij * p_i`  (larger `c_ij` pulls focus into `i`)
- **Entropy** unchanged: `S(p)=−Σ p_i log p_i`

The free‑energy functional and local update rule stay the same, but now each edge’s contribution is decomposed through its `(h,d,c)` components instead of a single typed weight. (fully defined)

For node \(i\) with neighbourhood \(\mathcal{N}(i)\):
- **Spring (hierarchy) energy**\
  \(E_{\text{spring},i}=\sum_{j\in\mathcal{N}(i)} w_{ij}\,(p_i-p_j)^2\) Enforces smooth hierarchy spacing.
- **Diffusion (transport) energy**\
  \(E_{\text{diff},i}=\sum_{j\in\mathcal{N}(i)} \frac{w_{ij}}{d_{ij}}\,|p_i-p_j|\) Minimises cost of moving mass (\(d_{ij}\) = edge distance).
- **Context potential** (from current transactions)\
  \(C_i=-c_i\,p_i,\quad c_i\ge 0\)  Higher \(c_i\) injects “current relevance”.
- **Entropy**  \(S(p)=-\sum_i p_i\log p_i \)  Encourages exploration / diversity.

\### 2.3 free‑energy functional

\(\mathcal F(p) = \sum_i\big(E_{\text{spring},i}+\lambda E_{\text{diff},i}+\gamma C_i\big) -T S(p)\)

Parameters:
- \(\lambda\)  — transport vs hierarchy trade‑off.
- \(\gamma\)  — context injection strength.
- \(T=1/\beta\) — temperature (exploration level).

All three have physical analogs: spring stiffness, medium conductivity, and heat bath temperature.

\### 2.4 local focus‑flow update (async & conservative)

Each node runs an **asynchronous heat‑flow step**:

$$
\Delta p_i = \eta\Big(\sum_{j\in\mathcal{N}(i)} w_{ij}(p_j-p_i) 
            -\partial_{p_i}(\lambda E_{\text{diff},i}+\gamma C_i) + T\,(1+\log p_i)\Big)
$$

with small step‑size \(\eta\).  A shared *normalisation gossip* every \(k\) ticks enforces \(\sum_i p_i=1\), guaranteeing no double‑spend of attention.  This replaces the previous global softmax with a fully local, edge‑only computation.

---

\## 3 transactions & deterministic verification

**Transaction = (ΔEdges, proof)**

1. **Submit**: user adds/updates edges and provides a zk‑SNARK attesting they applied the local update rule to all affected nodes (proof size \~O(log n)).  A base fee (EIP‑1559) is burned.
2. **Verify**: peers check proof deterministically in O(polylog n) time; no global recompute.
3. **Checkpoint**: every N blocks, a BFT committee finalises the current \(p\) snapshot \(\tilde p\). Between checkpoints asynchronous updates continue.

---

\## 4 rewarding negentropy (Probabilistic Shapley Attribution)

\### 4.1 local Δ𝔽 For each tx, compute local free‑energy drop:\
\(\Delta\mathcal F_{\text{local}} = \mathcal F_{\text{before}}-\mathcal F_{\text{after}}\) over affected nodes.

\### 4.2 Monte‑Carlo Shapley sampling Sample \(k\) random orderings of recent txs, measure marginal Δ𝔽, average to \(\hat S_i\).

\### 4.3 reward formula \(R_i = \alpha\,\Delta\mathcal F_{\text{local}} + (1-\alpha)\,\hat S_i\quad (\alpha\approx0.8)\)

Complexity O(k n) with k≪n (e.g. 100) → scales to ≥10⁶ tx/epoch.

---

\## 5 security & attack resistance

| threat             | mitigation                                                                                    |
| ------------------ | --------------------------------------------------------------------------------------------- |
| Sybil spam         | base fee + stake‑weighted participation; stake slashed if tx increases global free energy.    |
| Edge‑weight gaming | curvature‑aware decay prunes edges with negative contribution; rewards tied to long‑term Δ𝔽. |
| Proof forgery      | zk‑proofs guarantee local rule correctness.                                                   |
| Focus inflation    | total mass \(\sum_i p_i=1\) conserved by gossip normalisation.                                |

---

\## 6 thermodynamics ⇒ intelligence

FFC realises Schrödinger’s notion that life feeds on **negentropy**: rewarded agents export entropy to an external energy source (fees) while importing order.  Kantorovich optimal transport appears because updates move probability mass with minimal edge cost.  The converged state \(p^*\) is **maximally informative order** under energy constraints — a thermodynamic definition of intelligence.

---

\## 7 distributed coordination loop

```
sense  → focus  → act  → learn  → (edge decay) → repeat
context   update     output     weights  entropy regulariser
```

Edge decay now uses **curvature‑aware exponential pruning** rather than the sign‑flipped entropy term: \(w_{ij}\leftarrow w_{ij}\,e^{-\alpha(1+\kappa_{ij})}\) where \(\kappa_{ij}\) = Ollivier‑Ricci curvature.

---

\## 8 properties & applications
- **Turing‑complete** graph rewrite primitives.
- **Probabilistic scheduling** → prioritised meaningful compute.
- **Self‑pruning** via curvature decay.
- **Negentropy market** = proof‑of‑intelligence.
- **P2P scalability**: all updates local, verification deterministic.

Use‑cases: ranking/search, decentralised LLM, swarm co‑ordination, scientific crowdsourcing.

---

\## 9 outlook: structure‑preserving learning

Edges evolve into geometric constraints; future work explores **manifold‑aware neural layers** that respect this geometry, yielding more stable, interpretable AI.

---

\## 10 Hash‑DAG verifiable delay proof (VDF)
- **Why** Couples each negentropy proof to real wall‑clock time so adversaries cannot pre‑compute or replay contributions across Sybil identities.
- **Tx layout** `(ΔEdges, local‑proof, vdf_seed, vdf_output, vdf_path)` where `vdf_seed = hash(ΔEdges‖slot_id)`.
- **Computation** User runs a sequential VDF (e.g. Wesolowski) for a fixed delay ( ≈1‑2 s) before finalising the proof; cannot be parallelised.
- **Verification** Peers check VDF in poly‑log time plus verify zk‑proof of the local FFC update.
- **Hash‑DAG** Each VDF output becomes a node in a DAG; independent branches verify in parallel; roots are checkpointed by the BFT committee every N blocks.
- **Reward scaling** `reward ∝ Δ𝔽 / wall_clock_seconds` → honest work with real energy wins, replay loses.
- **Security** Front‑running and duplication require breaking the VDF or eclipsing time itself; Sybil resistance strengthened.

---

\## 11 conclusion

Focus‑Flow Computation marries deterministic execution with probabilistic, energy‑guided scheduling.  By rewarding negentropy and enforcing optimal transport under conservation laws, it converts a P2P graph into a self‑organising super‑organism: the blockchain that thinks.