- **GFlowNet**: a proposal engine that *samples edits* (small graph changes) in proportion to how good they look.
- **Focus‑Flow**: a physics‑style process that keeps a live **attention field** \(π\) over the graph (what the network cares about now).

**Why marry them**
- Let **π steer what to try next**; let accepted edits **reshape π**. That closes the loop so exploration stays useful and the network keeps learning.

**The loop (5 steps)**
1) Snapshot the current focus \(π_t\).  
2) GFlowNet proposes a batch of edits (add link, up‑weight, attach evidence…).  
3) Score each edit with a *fast local* focus‑gain estimate **Δπ̂** (no global recompute).  
4) Pass budget/guard checks → commit the best subset.  
5) Recompute focus **π_{t+1}**, train GFlowNet on what worked, repeat.

**What’s rewarded**
- Edits that **increase order/information** (lower free energy / raise useful focus) get paid; noise burns fees. Incentives match the global objective.

**Guardrails (so it doesn’t go off the rails)**
- Quotas per topic, fees, and rate limits at hot nodes.
- Proofs/audits for suspicious Δπ̂.
- Costs in the reward: storage/compute/network all counted.
- Rollback window + revert metrics keep reliability in check.

**Glossary (one‑liners)**
- **π**: the network’s current attention allocation over nodes.
- **Δπ̂**: quick estimate of how much an edit would shift π locally.
- **Edit/Diff**: a small set of graph changes (e.g., add_link u→v with weight/tag).
- **R(x)**: the reward used by GFlowNet when proposing edits.
- **SubTB**: a training trick (sub‑trajectory balance) that spreads credit over long edit sequences.
- **Cyberlink**: a signed edge; the atomic “fact” we add.

**Tiny worked example (concrete)**
1) Question spikes interest in *Cat*.  
2) π raises near **Cat**; GFlowNet proposes: (Cat→Animal [h‑edge]), (Cat→Wikipedia‑Cat [d‑edge]).  
3) Δπ̂ says both improve coverage with low cost; budgets OK.  
4) Commit; Focus‑Flow diffuses attention to Animal + sources.  
5) π_{t+1} stabilises with better hierarchy & references.  
6) GFlowNet is trained on this success pattern; next time it proposes similar high‑yield motifs.

—
gflownet × focus-flow — key insights (chapters)

chapter 1 — overview and thesis
- gflownet is a proposal engine that samples structured edits with probability proportional to a reward r(x)
- focus-flow computes a global attention field π over the network graph from ongoing activity
- we marry them by letting π shape r(x), while each accepted edit updates the graph and shifts π

chapter 2 — where they align
- both turn unnormalized scores into stochastic processes with good stationary behavior
- both help approximate intractable sums: gflownet via learned flows and balance losses; focus-flow via fast diffusion/consensus on the graph
- both admit forward/backward views: gflownet uses p_f/p_b over constructive dags; focus-flow has diffusion and global correction phases

chapter 3 — where they differ
- objective: gflownet learns to generate; focus-flow computes to rank
- substrate: gflownet reasons over trajectories of edits; focus-flow maintains a global stationary distribution over existing nodes/edges
- agency: gflownet is agentic and exploratory; focus-flow is deterministic aggregation with economic constraints

chapter 4 — coupling pattern (one line)
- r(x) = exp(β·φ(x, π_t) + u_task(x) − cost(x) + novelty(x))
- retrain gflownet against a lagged snapshot π_t while the network updates to π_{t+1}

chapter 5 — edit language (action/state/reward)
- actions: add_link(u→v, w, tag) · upweight(u→v, Δw) · spawn_node(z) · attach_evidence(v, blob_id)
- state: a partial subgraph (the pending diff) plus local context features
- terminal: a validated batch of edits ready to commit
- reward: r(x) = exp(β₁·Δπ̂(x) + β₂·u_task(x) − β₃·cost(x) + β₄·novelty(x))
- Δπ̂(x): fast local estimate of focus lift from applying diff x (learned surrogate or incremental rank)

chapter 6 — training tactics
- use sub-trajectory balance to propagate credit over long edit sequences
- stabilize with a lagged focus prior π̂_t (target network) to avoid chasing a moving field
- temper the policy (entropy or temperature τ) for diversity vs exploitation
- mix on-policy sampling with replay of high-Δπ̂ trajectories for sample efficiency

chapter 7 — closed-loop algorithm (sketch)
- snapshot π_t and local graph view
- gflownet proposes k candidate diffs x₁…x_k
- compute Δπ̂(x_i) via incremental rank or a learned surrogate
- filter by budgets/guards; select a feasible subset s ⊆ {x_i}
- commit s → graph_t+1; run focus-flow to compute π_{t+1}
- train gflownet on accepted/rejected trajectories with subtb; update lagged π̂ on schedule
- repeat continuously per bucket/lane

chapter 8 — safety and economics (guards)
- quotas: per-bucket caps, rate limits, and fee weights to prevent spam at hot nodes
- fast checks: existence of referenced blobs, schema/acl validation before inclusion
- fraud proofs: require rank-delta proofs or audits on suspicious Δπ̂
- cost shaping: include compute/storage/network costs directly in cost(x)

chapter 9 — metrics that actually move the needle
- diversity: entropy of sampled edit types and coverage of node/edge classes
- impact: realized Δπ over time per accepted edit and per unit cost
- stability: spectral gap / mixing time of the focus kernel before vs after edits
- efficiency: edits per joule and per dollar; gpu occupancy and tail latency
- reliability: revert rate, failed-proof rate, and mean time to consistency across shards

chapter 10 — minimal pseudocode
- training loop: for each epoch → snapshot π_t → for b in 1..batches: sample τ ~ p_f(·|π_t); score r(τ); backprop subtb → every m steps update target π̂
- deployment loop: at wall-clock ticks, take top-k by r(τ) under budgets; commit; recompute π; log realized Δπ and training targets

chapter 11 — design notes
- make Δπ̂ differentiable: a small graph net predicts rank deltas from local motifs so reward remains smooth
- sparsify action space: constrain add_link to whitelisted motifs (triadic closure, co-citation, functional dependencies) for tractability
- multi-scale: run a gflownet per bucket; aggregate compressed Δ-ranks to a hub and feed back as priors
- asynchrony: prefer eventual consistency with bounded staleness windows over global barriers

chapter 12 — prototype plan
- single-bucket sandbox with 10⁶ edges, incremental pagerank, and a tiny gflownet over add_link/upweight
- offline evaluator that replays diffs and measures realized Δπ and stability metrics
- ablations: fixed prior vs focus-shaped r(x); tb vs subtb; with/without lagged π̂

chapter 13 — open questions
- how to bound global regret when π drifts quickly under heavy edit throughput
- how to prevent collusive edits that game Δπ̂ while hurting downstream utility
- what economic signals best stabilize exploration at scale without collapse to popular hubs

chapter 14 — one-sentence takeaway
- let focus-flow set the beat; let gflownet compose moves that follow the beat yet expand the score, and train on the delta between the two