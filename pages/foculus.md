---
tags: article, cip
crystal-type: process
crystal-domain: cyber
status: draft
---
# foculus consensus

the [[collective focus theorem]] proves that token-weighted [[random walk]] on a strongly connected [[cybergraph]] converges to a unique $\pi$. foculus turns this into [[consensus]]: a [[particle]] is final when $\pi_i > \tau$. [[neurons]] gossip [[cyberlinks]], GPUs iterate $\pi$, and finality emerges from the [[topology]] of [[attention]] — no voting rounds, no leader election, no block ordering

## network model

leaderless. every [[neuron]] computes $\hat\pi$ independently from its local view of the [[cybergraph]]. there is no block proposer, no rotation schedule, no single point of serialization. convergence emerges from gossip, not from coordination

foculus operates in partial synchrony: messages arrive within an unknown but finite bound $\Delta$. during asynchronous periods (partitions), no new [[particles]] finalize — but no conflicting [[particles]] can finalize either, because local $\hat\pi$ cannot reach $\tau$ without sufficient global connectivity. safety holds always. liveness resumes when connectivity restores

## state

each [[neuron]] maintains:

  - the local [[cybergraph]] $G = (V, E)$ — [[particles]] as vertices, [[cyberlinks]] as weighted edges
  - the current estimate $\hat\pi$ — converging toward the true stationary distribution
  - the finality set $F$ — [[particles]] whose $\pi_i$ has crossed $\tau$

a [[particle]] is in one of three states: pending → final → pruned. transitions are irreversible

## conflict

two [[particles]] conflict when they consume the same resource: the same [[token]] spent twice, or contradictory [[cyberlinks]] from the same [[neuron]] in the same slot

conflict detection is local: each [[neuron]] tags conflicting [[particles]] upon receipt. conflicting [[particles]] compete for $\pi$ mass — the graph routes [[attention]] to one or the other, never both simultaneously

## fork choice

$\pi$ is the fork choice rule. when conflicts exist, the [[particle]] with higher $\pi_i$ is the canonical choice. this is not a vote — it is the outcome of the entire network's link structure converging through the [[tri-kernel]]

why this works: $\pi$ integrates all [[cyberlinks]] from all [[neurons]], weighted by [[token]] stake. manipulating $\pi$ requires controlling the topology of the [[cybergraph]] itself — which costs real [[tokens]]

## protocol

1. gossip — [[neurons]] broadcast new [[particles]] + [[cyberlinks]]
2. local update — every ~100 ms, GPU-accelerated sparse-matrix×vector refines $\hat\pi$
3. finalize — [[particle]] $i$ becomes final when $\hat\pi_i > \tau(t)$, where $\tau(t) = \mu_\pi + \kappa\sigma_\pi$, $\kappa \in [1,2]$
4. prune — conflicting [[particles]] with $\hat\pi \leq \tau$ are discarded
5. reward — validator $v$ earns proportional to $\Delta\pi$ contributed

## safety

theorem (no double finality): two conflicting [[particles]] cannot both exceed $\tau$

assumption: honest [[neurons]] control $\geq \frac{1}{2} + \delta$ of staked [[tokens]]

this bounds their share of $\pi$ from below: honest [[neurons]] create the majority of weighted [[cyberlinks]], so honest [[particles]] attract the majority of random-walk mass. the adversary's conflicting [[particle]] is starved of inbound links

proof: $\sum \pi_i = 1$. if conflicting [[particles]] $a, b$ both had $\pi_a, \pi_b > \tau$, the adversary would need $> \frac{1}{2}$ of total mass — contradicting the honest-majority bound

double spend prevention follows directly: a [[token]] transfer is a [[particle]]. two conflicting spends are conflicting [[particles]]. only one crosses $\tau$

## liveness

ergodicity of the transition matrix $P$ guarantees every valid [[particle]] accumulates $\pi$ mass over time

convergence rate depends on the spectral gap $\lambda$ of $P$: expected time to finality is $O(\log(1/\varepsilon)/\lambda)$ iterations. larger spectral gap means faster finality. dense, well-connected [[cybergraphs]] have larger gaps

during partitions: $\lambda$ drops for the disconnected subgraph, finality slows or halts. this is the correct behavior — the system refuses to finalize when it lacks global information

## sybil resistance

$\pi$ is weighted by staked [[tokens]], not by node count. creating 1000 [[neurons]] with zero stake produces zero $\pi$ influence. creating fake [[cyberlinks]] without stake backing produces negligible mass shifts

the cost of attacking $\pi$ is the cost of acquiring $> \frac{1}{2}$ of staked [[tokens]] — same economic security model as proof-of-stake, but the attack surface is the graph topology rather than a voting protocol

## finality

foculus provides deterministic finality: once $\pi_i > \tau$, the [[particle]] is final. no rollbacks, no probabilistic confirmation depth

the threshold $\tau(t) = \mu_\pi + \kappa\sigma_\pi$ adapts to the current distribution. when the network is decisive (low variance), $\tau$ is low and finality is fast. when the network is uncertain (high variance), $\tau$ rises and finality slows — the system self-regulates

## performance

| metric | classic BFT | nakamoto | foculus |
|---|---|---|---|
| leader | rotating proposer | miner (PoW lottery) | none |
| finality | 5-60 s | ~60 min | 1-3 s |
| throughput | 1k-10k tx/s | ~10 tx/s | ~10⁹ signals/s per GPU |
| validator scale | 10²-10³ | unbounded | unbounded |
| fault tolerance | 1/3 stake | 51% hash | 1/2 $\pi$ |

each iteration is a sparse matrix-vector multiply — embarrassingly parallel, no sequential bottleneck. single GPU (A100): ~50M edges at 40 Hz ≈ 2×10⁹ edge ops/s. with $K$ shards, throughput scales linearly

latency: compute ~0.2 s, 5-8 iterations, propagation ~0.4 s → worst-case finality ~1.4 s WAN

## economics

rewards proportional to the measurable shift in $\pi$:

$$\text{reward}(v) \propto \Delta\pi(v)$$

validators who add [[cyberlinks]] that meaningfully shift the stationary distribution earn more. this aligns incentives: the network rewards contributions to convergence, not mere participation

damping prevents concentration: $\pi_i \leftarrow \pi_i \cdot \gamma^t$, $\gamma \in (0,1)$. older or less-endorsed [[particles]] fade. the system forgets noise and retains what matters

## open questions

  - partition recovery: when two halves of the network reconnect, how quickly does $\pi$ reconverge? bounded by spectral gap, but practical latency under adversarial partitions is uncharacterized
  - threshold gaming: can an attacker oscillate $\sigma_\pi$ to manipulate $\tau$? the adaptive threshold needs formal bounds on adversarial variance injection
  - bootstrapping: a cold network has few [[cyberlinks]] and small spectral gap — finality may be slow until the [[cybergraph]] reaches sufficient density
  - MEV: ordering within a finality window is determined by $\pi$ dynamics, not by a sequencer — but extractable value from link ordering needs analysis

---

[[consensus]] is not voted — it is computed

see [[collective focus theorem]] for convergence proofs. see [[tri-kernel]] for the operators. see [[focus flow whitepaper]] for the full protocol specification
