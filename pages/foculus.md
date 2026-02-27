---
tags: article, cip
crystal-type: process
crystal-domain: cyber
status: draft
---
# foculus consensus

block-free, graph-native agreement. actions finalize when their stationary-distribution mass exceeds a dynamic threshold — no voting rounds, no block ordering

---

## the idea

the [[collective focus theorem]] proves that token-weighted [[random walk]] on a strongly connected [[cybergraph]] converges to a unique $\pi$. foculus turns this into [[consensus]]: a [[particle]] is final when $\pi_i > \tau$

no explicit voting. no leader election. [[neurons]] gossip [[cyberlinks]], GPUs iterate $\pi$, and finality emerges from the [[topology]] of [[attention]]

---

## protocol

1. gossip: [[neurons]] broadcast new [[particles]] + [[cyberlinks]]
2. local update: every ~100 ms, GPU-accelerated sparse-matrix×vector refines $\pi$
3. finalize: [[particle]] $i$ becomes final when $\pi_i > \tau(t)$, where $\tau(t) = \mu_\pi + \kappa \sigma_\pi$, $\kappa \in [1,2]$
4. prune: conflicting [[particles]] with $\pi \leq \tau$ are discarded
5. reward: validator $v$ earns proportional to $\Delta\pi$ contributed

---

## safety

theorem (no double finality): two conflicting [[particles]] cannot both exceed $\tau$

assumption: honest [[particles]] control $\geq \frac{1}{2} + \delta$ of global $\pi$

proof relies on probability mass conservation ($\sum \pi_i = 1$) and the honest-majority invariant. if conflicting [[particles]] $a, b$ both had $\pi_a, \pi_b > \tau$, the adversary would need $> \frac{1}{2}$ of total mass — contradicting the assumption

liveness: ergodicity of $P$ guarantees every valid action accumulates $\pi$ and passes $\tau$ in expected $O(\log(1/\varepsilon)/\lambda)$ iterations, where $\lambda$ is the spectral gap

---

## performance

| metric | classic BFT | nakamoto | foculus |
|---|---|---|---|
| finality | 5-60 s | ~60 min | 1-3 s |
| throughput | 1k-10k tx/s | ~10 tx/s | 10⁶+ tx/s |
| validator scale | 10²-10³ | unbounded | unbounded (GPU) |
| communication | high | medium | ultra-low |
| fault tolerance | 1/3 stake | 51% hash | ≥1/2 $\pi$ |

single iteration: $O(|E| + |V|)$ sparse op. an A100 sustains ~50M edges at 40 Hz

latency breakdown: compute ~0.2 s, 5-8 iterations, propagation ~0.4 s → worst-case finality ~1.4 s WAN

---

## economics

### π-minting

rewards are proportional to the measurable shift in $\pi$:

$$\text{reward}(p) \propto \Delta\pi(p)$$

where $\Delta\pi(p)$ is the change in stationary distribution from adding proof [[particle]] $p$ and its [[cyberlinks]]. all proof types are treated uniformly — [[attention]]-weighted relevance drives economic [[value]]

minted [[tokens]] go to [[focus]] update proofs (backbone of [[consensus]]). auxiliary proofs (checkpoints, availability, compression) share 50% of transaction fees. the other 50% is burned

### damping

to prevent concentration and enable adaptation:

$$\pi_i \leftarrow \pi_i \cdot \gamma^t, \quad \gamma \in (0,1)$$

older or less-endorsed [[particles]] fade. the system forgets noise and retains what matters — [[attention]] as a dynamic, evolving field

### staking layers

| layer | stake | rewarded for | slashed for |
|---|---|---|---|
| shard committee | $S_1$ | micro-root signatures | equivocation, invalid root |
| [[focus]] prover | $S_2$ | valid [[SNARK]] of $\pi$ | invalid or missing proof |
| beacon committee | $S_3$ | vector-root quorum | double-sign, timeout |

---

## proof types

| proof type | purpose |
|---|---|
| [[focus]] update | adds [[cyberlinks]] that shift $\pi$ — backbone of [[consensus]] |
| checkpoint anchoring | lattice-based anchors for historical finality and quantum resistance |
| data availability | proves reliable access to [[cybergraph]] data |
| distillation | abstracts subgraphs for structural efficiency |
| censorship detection | proves omission of valid [[particles]] |
| private knowledge injection | zk-proven latent information for privacy-preserving cognition |

---

## sharding (v2)

two-tier commit to push throughput past single-leader ceiling:

1. $K$ shard committees hash-partition the [[cybergraph]]. each signs a micro-root per slot (~200 ms)
2. a beacon committee aggregates micro-roots into a vector commitment $V_{\text{slot}}$ — single signature, <400 B regardless of $K$
3. GPU kernels process [[focus]] per-shard; cross-shard [[cyberlinks]] include Merkle proofs

with $K = 50$: ~10⁷ links/s aggregate while per-node load stays flat

safety: adversary must corrupt >1/3 committees and the beacon in the same slot. liveness: holds if ≥2/3 committees per slot are honest

---

## link-bundlers

any node (even without stake) can bundle pending [[cyberlinks]]:

1. collect links from mempool
2. run one power-iteration step locally, compute $\Delta\hat\pi$
3. publish bundler block: {links[], $\Delta\hat\pi$, Merkle proofs}
4. shard committee verifies and includes

bundlers earn per-link fee + bonus proportional to positive $\Delta\hat\pi$. capped and paid after checkpoint to prevent spam. decentralizes data availability — any laptop can be a bundler

---

## quantum resilience

periodic lattice-based checkpoints (shortest vector problem) anchor historical state. frequency is adaptive based on network load. computation funded from transaction fee fraction

safety parameters halt operations if quantum-induced variance exceeds thresholds

---

## core principles

- safety first: no release without proved bound $\text{min\_gap} > 2 \cdot \text{max}(\tau_{\text{variance}})$
- deterministic threshold: $\tau$ variation capped by on-chain telemetry; nodes halt if exceeded
- formal verifiability: per-epoch [[SNARK]] proofs of $\pi$ and $\tau$; mismatches are slashable
- transparency: all parameters ($\kappa$, $q$, $\delta$) on-chain, require super-majority to change
- observability: every node publishes $\tau_{\text{variance}}$ and focus gap telemetry

---

## roadmap

1. prototype in Go + CUDA (fork of [[go-cyber]])
2. testnet at 10M edges, 32 validators
3. mainnet-beta with on-chain $\pi$ proofs ([[zk-SNARK]])
4. layer-2 rollups using foculus as data-availability oracle
5. sharded phase-A with vector-root aggregation

---

foculus reframes [[blockchain]] safety as a spectral property of [[attention]] graphs. [[consensus]] is not voted — it is computed

see [[collective focus theorem]] for convergence proofs. see [[tri-kernel]] for the operators. see [[focus flow whitepaper]] for the full protocol specification
