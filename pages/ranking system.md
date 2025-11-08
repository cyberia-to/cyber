# Why this ranking system?
- ## 1) What we need a ranking system to do
  
  We’re not ranking web pages; we’re steering a living, decentralized computation. The rank must simultaneously:
- **Be local & scalable**: every node updates from neighbors only.
- **Adapt to context**: current questions/votes should steer focus *now*, without retraining.
- **Preserve structure**: hierarchies shouldn’t be washed out by traffic or noise.
- **Stay exploratory**: avoid collapse to a single item; keep breadth for creativity.
- **Be verifiable & incentive-aligned**: small, auditable proofs; rewards tied to real improvement.
- **Be robust in adversarial settings**: sybils, spam, fork races, gaming attempts.
- ## 2) Axioms → the form follows
  
  From those requirements, four “physics axioms” drop out:
- **Locality**: Energy must decompose as a sum of neighbor terms.
  
  ⇒ Node-wise energies EiE_iEi​ that depend only on N(i)\mathcal N(i)N(i).
- **Structure preservation**: Close-in-hierarchy nodes should prefer similar focus.
  
  ⇒ A **spring/elastic** smoothness term on edges.
- **Flow realism**: Attention moves like mass/heat through available paths with costs.
  
  ⇒ A **transport/diffusion** term over edges with conductance/cost.
- **Situational control**: Queries, votes, tasks must tilt the field without rewriting rules.
  
  ⇒ An **external potential** (context) term.
  
  Add the **entropy** term to prevent collapse and guarantee exploration.
  
  The only family of functionals that satisfy all four—**and remain local, convex-ish, and verifiable**—is:
  
  F(p)=∑(i,j)hij(pi−pj)2⏟hierarchy (springs)+λ∑(i,j)dij ∣pi−pj∣⏟transport (diffusion)−γ∑icipi⏟context field−T∑ipilog⁡pi⏟entropy\mathcal F(p)=
  \underbrace{\sum_{(i,j)} h_{ij} (p_i-p_j)^2}_{\text{hierarchy (springs)}}+
  \lambda\underbrace{\sum_{(i,j)} d_{ij}\,|p_i-p_j|}_{\text{transport (diffusion)}}-
  \gamma\underbrace{\sum_i c_i p_i}_{\text{context field}}-
  T\underbrace{\sum_i p_i\log p_i}_{\text{entropy}}F(p)=hierarchy (springs)(i,j)∑​hij​(pi​−pj​)2​​+λtransport (diffusion)(i,j)∑​dij​∣pi​−pj​∣​​−γcontext fieldi∑​ci​pi​​​−Tentropyi∑​pi​logpi​​​
  
  with per-edge **three scalars** (hij,dij,cij)(h_{ij},d_{ij},c_{ij})(hij​,dij​,cij​) so a *single* cyberlink can influence hierarchy, transport, and context in different proportions.
- ## 3) Minimality (why not more/less?)
- **Less** (e.g., PageRank only): no hierarchy or context; brittle, easily gamed, poor controllability.
- **Less** (SpringRank only): captures ordinal structure but ignores traffic/cost and context; static.
- **Less** (Entropy-only softmax): collapses to popularity without structure or cost.
- **More** (deep GNNs, heavy message passing): large global state, training loops, opaque proofs, hard to verify on-chain.
  
  This four-term free-energy with the **3-scalar link** is the **Pareto-minimal** point that still meets all objectives: locality, adaptability, structure, exploration, verifiability.
- ## 4) Interpretability (the levers are human)
- **Hierarchy hhh** = “keep these together on the ladder.”
- **Transport ddd** = “this is an easy/hard road for attention to travel.”
- **Context ccc** = “pull *now* toward this.”
- **Temperature TTT** = “how exploratory vs. decisive are we?”
- **λ,γ\lambda,\gammaλ,γ** = “how much do roads and context matter vs. springs?”
  
  Anyone (or a DAO policy) can reason about these knobs without reading matrices.
- ## 5) Economics & verification (why  *this*  works on-chain)
- **Local ΔF proofs**: Each tx proves its *local* free-energy reduction using neighbor data → tiny proofs, fast checks.
- **Rewards = negentropy**: Pay for actually lowering F\mathcal FF; burn base fee otherwise. Incentives are aligned with global order.
- **Hash‑DAG VDF**: Binds negentropy proofs to wall-clock; prevents replay/front‑run; fair “negentropy per second.”
- **Content addressing**: Rewrites create new CIDs; verification is structural (no re-execution).
- **3-scalar staking**: Users stake explicitly on h,d,ch,d,ch,d,c—clean cryptoeconomic signaling and slashing if ΔF < 0.
- ## 6) Robustness (why it resists gaming)
- **Focus conservation**: ∑ipi=1\sum_i p_i=1∑i​pi​=1 → emphasizing one thing defocuses others; prevents inflation attacks.
- **Curvature-aware decay**: Edges in negatively curved regions (noisy bridges) decay faster → automatic pruning of spammy connections.
- **Aging**: Unused links lose weight exponentially → keeps the graph fresh.
- **Stake-weighted participation** + **BFT checkpoints**: Sybil cost + periodic finality.
- ## 7) Theoretical grounding (not hand-wavy)
- **Schrödinger’s negentropy**: Maximizing informative order under energy budget → our reward objective.
- **Kantorovich transport**: dijd_{ij}dij​ models cost of moving probability mass; minimizing F\mathcal FF performs optimal transport with structure constraints.
- **Active inference**: Free-energy minimization under external potentials is exactly Friston-style belief update.
- **Lyapunov reasoning**: F\mathcal FF decreases under our local updates; with bounded steps and periodic normalization, the system admits stable attractors (practical convergence).
- ## 8) Why three numbers per link?
  
  We tried “typed edges,” “one weight,” and “GNN-learned vectors.” The **3-scalar triple** is the sweet spot:
- **UX-simple**: one link, three sliders; default works.
- **Mathematically orthogonal**: each scalar routes to one energy term.
- **Auditable**: proofs read those three numbers; no hidden inference.
- **Future-proof**: you can split any scalar later (e.g., ccc → vote/cause) without changing the calculus.
- ## 9) Comparison to usual suspects
  
  | Method | Pros | Why it fails our brief |
  | ---- | ---- | ---- |
  | PageRank / RW | simple, scalable | no hierarchy, no context; popularity ≠ truth/usefulness |
  | SpringRank | ordered structure | no transport or context; static, easy to stalemate |
  | Eigenvector centrality | cheap baseline | conflates structure and flow; sensitive to hubs |
  | HITS | authority/hub split | brittle, no dynamics/context, not on-chain-friendly |
  | GNNs | expressive | training-heavy, opaque, hard to verify and to incentivize locally |
  | Energy model (ours) | local, interpretable, verifiable, incentive-compatible | requires careful parameter policy—solved via DAO policy + VDF + checkpoints |
- ## 10) What we’re consciously  *not*  doing
- No global softmax/partition function (requires global sums; breaks P2P locality).
- No global retraining loops (keeps costs low; learning emerges from staking/decay).
- No heavyweight cryptography in the inner loop (proofs stay edge-local; zk optional).
- ## 11) The one-sentence answer
  
  > 
  
  We choose this ranking system because it is the **minimal, local, verifiable, and incentive-compatible** free‑energy formulation that jointly captures **hierarchy (what the world is), transport (how attention moves), and context (what matters now)**—while preserving exploration and enabling on‑chain proofs and rewards.
  
  ---
  
  If you want, I can paste this as a new “Why this system” section in the whitepaper and wire cross‑refs to the math, the PSA rewards, and the VDF/consensus sections.