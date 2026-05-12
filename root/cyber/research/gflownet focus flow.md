---
tags: cyber, research, article
alias: gflownet focus flow
crystal-type: process
crystal-domain: cyber
status: draft
---
# GFlowNet × Focus-Flow: learned proposal engine for the [[cybergraph]]

## the problem

the [[cybergraph]] grows by [[neurons]] creating [[cyberlinks]]. each cyberlink costs [[focus]] — an irreversible resource commitment. a neuron choosing what to link faces a combinatorial search: which [[particles]] to connect, with what weight, to maximise the value of its remaining focus budget.

the question: can a learned model propose high-value edits — not the single best edit, but a DISTRIBUTION over good edits proportional to their quality?

## why GFlowNets

a Generative Flow Network (GFlowNet, Bengio et al. 2021) is a learned sampler that constructs structured objects by sequential actions, producing samples with probability proportional to a reward $R(x)$:

$$p_\theta(x) \propto R(x)$$

unlike RL (which finds the mode — the single best action), GFlowNets sample the DISTRIBUTION. this matters for a knowledge graph:
- RL would always propose the same "best" link → monoculture
- GFlowNet proposes diverse links proportional to quality → exploration

unlike MCMC (which also samples the distribution), GFlowNets are AMORTISED — once trained, each sample is a single forward pass. no mixing time, no burn-in, no chain convergence.

the training objective (trajectory balance, Malkin et al. 2022):

$$\log \frac{Z \cdot \prod_{t} p_F(s_{t+1} | s_t)}{\prod_{t} p_B(s_t | s_{t+1}) \cdot R(x)} = 0$$

where $p_F$ is the forward (construction) policy, $p_B$ is the backward (decomposition) policy, and $Z$ is the partition function. credit propagates over full trajectories via sub-trajectory balance (SubTB).

## connection to [[cyber/seer|cyber-seer]]

cyber-seer and GFlowNet solve the same problem: WHERE to link. they differ in method:

| | [[cyber/seer|cyber-seer]] | GFlowNet |
|---|---|---|
| approach | analytical (Fiedler vector, Lanczos) | learned (policy network, trajectory balance) |
| signal | $\Delta\lambda_2 = (v_2(i) - v_2(j))^2$ | $R(x) \propto \exp(\beta \cdot \text{quality})$ |
| output | ranked list of optimal links | distribution over candidate links |
| diversity | deterministic top-K | stochastic sampling proportional to quality |
| phases | explicit (bridge → mesh → semantic) | learned (reward terms shift automatically) |
| cost | cheap (Lanczos is O(k·|E|)) | expensive (training + inference) |

they compose naturally. cyber-seer's three signals become GFlowNet reward components:

$$R(x) = \exp\left(\beta_1 \cdot \underbrace{\Delta\lambda_2(x)}_{\text{seer: bridges}} + \beta_2 \cdot \underbrace{\Delta\phi^*(x)}_{\text{seer: semantic}} + \beta_3 \cdot \underbrace{\text{resilience}(x)}_{\text{seer: mesh}} - \beta_4 \cdot \underbrace{c(n)}_{\text{exponential cost}}\right)$$

cyber-seer's three phases emerge automatically from the reward balance:
- **early** (cost low): all $\beta$ terms matter, $\Delta\lambda_2$ dominates because graph has large spectral gaps → GFlowNet learns to propose bridges
- **mid** (cost rising): $\Delta\lambda_2$ saturates, resilience term matters → GFlowNet learns mesh patterns
- **late** (cost high): exponential cost crushes low-value proposals, only high-$\Delta\phi^*$ semantic links survive → GFlowNet proposes precision links

the GFlowNet doesn't need to be told which phase it's in. the reward function's exponential cost term (from the [[universal law]]) naturally shifts the learned policy from structural to semantic as the graph matures.

**cyber-seer as GFlowNet teacher**: cyber-seer's Fiedler-optimal links can pre-train the GFlowNet (behavioural cloning on analytical decisions). the GFlowNet then generalises beyond the analytical signal — discovering link patterns that improve $\phi^*$ in ways the spectral analysis doesn't predict (semantic shortcuts, multi-hop bridges, creative connections).

## the coupling

GFlowNet proposes edits. [[tri-kernel]] focus-flow evaluates them. the loop:

```
1. snapshot current focus π_t from tri-kernel
2. GFlowNet proposes batch of candidate edits
   (add cyberlink, upweight axon, attach evidence)
3. score each edit: Δφ*̂ = estimated focus gain
4. filter by budget/guards → commit best subset
5. recompute π_{t+1} via tri-kernel
6. train GFlowNet on realised Δφ*
7. repeat
```

the reward function:

$$R(x) = \exp\left(\beta_1 \cdot \Delta\hat{\phi^*}(x) + \beta_2 \cdot u_{\text{task}}(x) - \beta_3 \cdot \text{cost}(x) + \beta_4 \cdot \text{novelty}(x)\right)$$

where:
- $\Delta\hat{\phi^*}(x)$ = estimated focus lift from edit $x$ (fast local proxy for full tri-kernel)
- $u_{\text{task}}(x)$ = task-specific utility (e.g., answer a query, complete a pattern)
- $\text{cost}(x)$ = focus + storage + compute cost of the edit
- $\text{novelty}(x)$ = information gain (new connections vs redundant)

the exponential form follows from the [[universal law|exponential optimality under constraint]]: given finite focus budget, the optimal proposal distribution is exponential in quality.

## what the architecture already provides

several components of the original 14-chapter design have landed in the architecture through other mechanisms:

| original idea | where it landed | mechanism |
|---|---|---|
| focus-shaped reward | [[tri-kernel]] $\phi^*$ | stationary distribution IS the quality signal |
| edit validation | [[zheng]] proof per signal | every cyberlink carries a validity proof |
| budget/guards | [[focus]] metering in [[nox]] | focus is the native rate limiter |
| fraud proofs | [[structural-sync]] layer 1 | zheng proof prevents invalid edits |
| cost shaping | [[temporal decay]] | unused edges decay exponentially |
| metrics (spectral gap) | [[spectral gap from convergence]] | tri-kernel convergence already measured |
| rollback window | [[signal]] hash chain | append-only, immutable history |

the GFlowNet adds ONE thing the architecture doesn't have: a LEARNED proposal policy that improves over time. the rest is infrastructure that already exists (or will exist once the stack is built).

## the concrete research questions

### Q1: can $\Delta\hat{\phi^*}$ be estimated locally?

the full tri-kernel computation (diffusion + springs + heat kernel over the entire graph) is the most expensive operation in cyber. a GFlowNet reward that requires running the full tri-kernel per candidate edit is intractable.

the research question: can a cheap local proxy predict focus gain?

approaches:
- **graph neural network surrogate**: train a GNN to predict $\Delta\phi^*$ from a local subgraph around the edit. cost: O(1) per evaluation after training
- **incremental rank update**: personalised PageRank allows O(1/ε) push-back updates for single-edge changes. approximate $\Delta\phi^*$ by running a few push-back steps
- **spectral proxy**: the edit's effect on $\phi^*$ depends on how it changes the graph's spectral properties. low-rank spectral updates may give fast approximations

the quality of the GFlowNet depends entirely on the quality of $\Delta\hat{\phi^*}$. if the proxy is poor, the GFlowNet proposes noise.

### Q2: can GFlowNets scale to 10^6+ action spaces?

current GFlowNets (DAG-GFlowNet for Bayesian structure learning, molecular GFlowNets) operate on graphs with ~50 nodes and ~1000 possible actions per step. the [[cybergraph]] has billions of particles.

approaches:
- **hierarchical action space**: first select a namespace (coarse), then select a particle within namespace (fine). reduces action space from O(N) to O(√N) per level
- **attention-guided masking**: use $\phi^*_t$ to mask the action space — only consider particles with $\phi^* > \epsilon$ as link targets. the [[universal law]] predicts most focus concentrates on a small fraction of particles
- **per-neuron GFlowNet**: each neuron runs a small personal GFlowNet over its local context (particles it knows about). the global effect emerges from many local proposals. matches the decentralised architecture

### Q3: how does privacy interact?

individual [[cyberlinks]] are private (mutator set). the GFlowNet needs to evaluate candidate links. tension:
- training requires seeing which edits improved $\phi^*$ → but individual edits are private
- inference requires evaluating candidates against the graph → but the graph is partially hidden

resolution: the GFlowNet operates on PUBLIC aggregates only. $\phi^*$ is public. axon weights (aggregates) are public. individual cyberlinks are hidden. the GFlowNet proposes links to public particles, and the neuron decides privately whether to commit them.

this means the GFlowNet cannot optimise for specific private patterns. it optimises for publicly visible focus flow — which is exactly the right objective (public knowledge improvement, not private advantage).

### Q4: can the proposal be proved?

if a GFlowNet is a [[nox]] program, its execution produces a [[zheng]] proof via [[proof-carrying computation|proof-carrying]]. the proposal itself is provable: "this neuron ran a GFlowNet policy and it produced these candidate links."

this doesn't prove the links are GOOD — it proves the proposal process was correctly executed. quality comes from the reward function and focus economics, not from the proof.

the interesting question: can the GFlowNet's TRAINING be proved? if training uses gradient descent on the trajectory balance loss, can the gradient computation be proved in [[nox]]? this would enable verified model updates — provable learning.

## where it fits in the timeline

```
prerequisite stack (must exist first):
  nox VM          → trace generation, proof-carrying computation
  zheng prover    → signal validity proofs
  hemera hash     → content addressing, Fiat-Shamir
  bbg state       → NMT/polynomial state, mutator set
  tri-kernel      → φ* computation (the reward signal)
  foculus          → global convergence

GFlowNet layer (builds on top):
  Δφ*̂ surrogate   → train GNN proxy for focus gain prediction
  action space    → hierarchical namespace-aware proposal
  local GFlowNet  → per-neuron proposal policy
  training loop   → online learning from realised Δφ*
```

the GFlowNet is a LAYER 6 component — it sits above the five structural sync layers and uses them as infrastructure. it does not affect the core architecture. it is an optimisation for neuron decision-making, not a protocol requirement.

estimated timeline: after tri-kernel is operational and producing $\phi^*$ on a live graph with measurable $\Delta\phi^*$ per edit.

## honest assessment

| aspect | status | confidence |
|---|---|---|
| GFlowNet theory | mature (2021-2025, peer-reviewed) | high |
| GFlowNet for graph construction | demonstrated (DAG-GFlowNet) | high |
| GFlowNet at 10^6+ scale | undemonstrated | low |
| $\Delta\hat{\phi^*}$ local proxy | research question | medium |
| privacy-compatible training | feasible (public aggregates) | medium |
| provable proposals via nox | architecturally possible | medium |
| provable training | open research question | low |
| dependency on unbuilt stack | critical path blocker | — |

the research direction is valid. the architecture is compatible. the timing is wrong — this layer needs the stack beneath it to exist before it can be built or validated.

## what to do now

1. **formalise $\Delta\hat{\phi^*}$ estimation** as a standalone research question. this is valuable regardless of GFlowNet — any system that proposes edits needs a fast focus-gain proxy
2. **prototype DAG-GFlowNet on a synthetic cybergraph** (10^4 particles, known $\phi^*$). measure: does the GFlowNet learn to propose high-$\Delta\phi^*$ links? how does diversity compare to random/greedy baselines?
3. **defer integration** until tri-kernel is live and producing real $\phi^*$ values on a real graph

## references

[1] E. Bengio et al., "Flow Network based Generative Models for Non-Iterative Diverse Candidate Generation," NeurIPS 2021.
[2] N. Malkin et al., "Trajectory Balance: Improved Credit Assignment in GFlowNets," NeurIPS 2022.
[3] T. Deleu et al., "DAG-GFlowNet: Bayesian Structure Learning with GFlowNets," ICML 2022.
[4] N. Malkin et al., "GFlowNets and Variational Inference," ICLR 2023.
[5] E. Bengio et al., "GFlowNet Foundations," JMLR 2023.

see [[tri-kernel architecture]] for the focus computation, [[collective focus theorem]] for why exponential proposals are optimal, [[universal law]] for the variational principle, [[structural-sync]] for the infrastructure layers, [[cyber/research/zheng vs starks|zheng]] for the proof system
