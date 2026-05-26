---
tags: cyber, core, cip
crystal-type: process
crystal-domain: cyber
---
# foculus consensus

the [[collective focus theorem]] proves that token-weighted [[random walk]] on a strongly connected [[cybergraph]] converges to a unique $\phi^*$. foculus turns this into [[consensus]]: a [[particle]] is final when $\phi^*_i > \tau$. [[neurons]] gossip [[cyberlinks]], GPUs iterate $\phi^*$, and finality emerges from the [[topology]] of [[attention]] — no voting rounds, no leader election, no block ordering

## network model

leaderless. every [[neuron]] computes $\hat\phi^*$ independently from its local view of the [[cybergraph]]. there is no block proposer, no rotation schedule, no single point of serialization. convergence emerges from gossip, not from coordination

foculus operates in partial synchrony: messages arrive within an unknown but finite bound $\Delta$. during asynchronous periods (partitions), no new [[particles]] finalize — but no conflicting [[particles]] can finalize either, because local $\hat\phi^*$ cannot reach $\tau$ without sufficient global connectivity. safety holds always. liveness resumes when connectivity restores

## state

each [[neuron]] maintains:

  - the local [[cybergraph]] $G = (V, E)$ — [[particles]] as vertices, [[cyberlinks]] as weighted edges
  - the current estimate $\hat\phi^*$ — converging toward the true stationary distribution
  - the finality set $F$ — [[particles]] whose $\phi^*_i$ has crossed $\tau$
  - the nullifier set $N$ — nullifiers committed by finalized [[particles]]

a [[particle]] is in one of three states: pending → final → pruned. transitions are irreversible

## state model

the state is the [[cybergraph]] itself. there is no separate ledger. the finalized subgraph IS the canonical state

each [[token]] output is a [[particle]]. spending a [[token]] creates a new [[particle]] that references the input and presents a nullifier: $n = \text{Poseidon}(\text{NULLIFIER\_DOMAIN}, r.\text{nonce}, \text{secret})$. the nullifier is deterministic from the record — same record always produces the same nullifier

the nullifier set $N$ is append-only. a [[particle]] that presents a nullifier already in $N$ is invalid. this is the double-spend check: a pure function of the [[particle]] data and the current $N$, independent of arrival order

state transitions happen at finalization:

```
on finalize(P):
  for each nullifier n in P.nullifiers:
    assert n ∉ N           // not already spent
    N ← N ∪ {n}           // commit nullifier
  P.outputs become spendable
  conflicting particles → pruned
```

the critical point: transitions apply when a [[particle]] crosses $\tau$, not when it arrives. every [[neuron]] computes the same $\phi^*$ from the same graph, so they agree on which [[particle]] crosses $\tau$ first. the state sequence is determined by the $\phi^*$ convergence trajectory — not by a sequencer, proposer, or explicit ordering protocol

## conflict

### formal definition

two [[particles]] $P_a, P_b$ conflict if and only if:

$$\text{conflict}(P_a, P_b) \equiv (\exists\, n : n \in P_a.\text{nullifiers} \wedge n \in P_b.\text{nullifiers}) \;\lor\; (P_a.\text{author} = P_b.\text{author} \wedge P_a.\text{epoch} = P_b.\text{epoch} \wedge P_a.\text{signal} = P_b.\text{signal})$$

three conflict types:

| type | condition | example |
|---|---|---|
| double-spend | shared nullifier | two [[particles]] spend the same [[token]] output |
| equivocation | same author, same epoch, same signal type | [[neuron]] signs two contradictory [[cyberlinks]] in one epoch |
| resource collision | shared non-fungible input | two [[particles]] claim the same unique resource |

### detection without ordering

conflict detection is a pure function of [[particle]] content. given any $P_a$ and $P_b$, any [[neuron]] can evaluate $\text{conflict}(P_a, P_b)$ by comparing nullifier sets and author/epoch metadata. no ordering information is needed — only the data itself

each [[neuron]] maintains a local conflict index: nullifier → set of [[particles]] presenting it. when a new [[particle]] $P$ arrives with nullifier $n$:
- if $n$ has no entry → no conflict, index it
- if another [[particle]] $P'$ already presents $n$ → tag $(P, P')$ as conflicting

this detection is monotonic: once detected, a conflict is permanent. a [[neuron]] that has seen both [[particles]] will always detect the conflict, regardless of arrival order. a [[neuron]] that has seen only one treats it as non-conflicting — the safety proof guarantees the unseen conflicting [[particle]] cannot finalize in the meantime

### exclusive support

when a [[neuron]] detects a conflict between $P_a$ and $P_b$, it supports exactly one. the honest strategy: support the first-seen [[particle]]. [[cyberlinks]] go only to the supported member of the conflict group. the unsupported member receives no $\phi^*$ mass from this [[neuron]]

this is the critical constraint: each [[neuron]]'s stake-weighted mass flows to at most ONE member of any conflict group. conflicting [[particles]] compete for the same finite mass pool

## fork choice

$\phi^*$ is the fork choice rule. when conflicts exist, the [[particle]] with higher $\phi^*_i$ is the canonical choice. this is the outcome of the entire network's link structure converging through the [[tri-kernel]] — not a vote

why this works: $\phi^*$ integrates all [[cyberlinks]] from all [[neurons]], weighted by [[token]] stake. manipulating $\phi^*$ requires controlling the topology of the [[cybergraph]] itself — which costs real [[tokens]]. exclusive support ensures conflicting [[particles]] split a finite mass pool rather than duplicating it

the "no ordering" claim, precisely: there is no block proposer, no sequencer, no explicit transaction ordering. the ordering emerges from the $\phi^*$ convergence trajectory. the [[particle]] that crosses $\tau$ first wins — and which [[particle]] crosses first is determined by the graph topology, which every [[neuron]] can compute independently

## protocol

1. gossip — [[neurons]] broadcast new [[particles]] + [[cyberlinks]]
2. conflict check — each [[neuron]] indexes nullifiers and detects conflicts on receipt
3. exclusive support — for each conflict group, the [[neuron]] links only to its preferred member
4. local update — every ~100 ms, GPU-accelerated sparse-matrix×vector refines $\hat\phi^*$
5. finalize — [[particle]] $i$ becomes final when $\hat\phi^*_i > \tau(t)$, where $\tau(t) = \mu_\pi + \kappa\sigma_\pi$, $\kappa \in [1,2]$. nullifiers committed to $N$
6. prune — conflicting [[particles]] with $\hat\phi^* \leq \tau$ are discarded
7. reward — validator $v$ earns proportional to $\Delta\phi^*$ contributed

## safety

### no double finality

theorem: two conflicting [[particles]] cannot both exceed $\tau$

assumption: honest [[neurons]] control $\geq \frac{1}{2} + \delta$ of staked [[tokens]]

proof sketch:

1. conflicting [[particles]] $P_a, P_b$ form a conflict group. each [[neuron]] supports exactly one member (exclusive support)
2. the total $\phi^*$ mass directed to $\{P_a, P_b\}$ equals the total mass of all [[neurons]] that have linked to either. this sum is bounded by a fraction of 1 (since $\sum \phi^*_i = 1$ and other non-conflicting [[particles]] also receive mass)
3. honest [[neurons]] collectively control $> \frac{1}{2}$ of stake-weighted mass. under first-seen, one member — say $P_a$ — receives honest majority support (the member that propagated faster)
4. the adversary controls $< \frac{1}{2}$ of mass and directs it to $P_b$
5. $\phi^*_a > \phi^*_b$ because $P_a$ has strictly more weighted inbound links from honest [[neurons]]
6. the [[tri-kernel]] contraction property ($\kappa < 1$ from [[collective focus theorem]]) amplifies this gap with each iteration — the slight initial advantage compounds exponentially
7. $\tau$ is adaptive: as $P_a$ gains mass and $P_b$ loses it, the distribution sharpens. $P_b$ falls further below $\tau$ while $P_a$ approaches it
8. therefore $P_b$ cannot cross $\tau$ while $P_a$ can ∎

double-spend prevention follows directly: a [[token]] transfer is a [[particle]]. two conflicting spends present the same nullifier. only one crosses $\tau$. the winner's nullifier enters $N$. the loser is pruned

### edge case: simultaneous convergence

if $\phi^*_a = \phi^*_b$ at any iteration (exact tie), the situation is unstable — any perturbation breaks symmetry. in practice, different network propagation times ensure the initial split is asymmetric. as a deterministic fallback for the measure-zero exact-tie case: lower $\text{hash}(\text{particle\_data})$ wins. this is computable by every [[neuron]] independently

### what honest neurons guarantee vs. what they do not

guaranteed:
- conflicting [[particles]] cannot both finalize (safety)
- the winner has more honest support than the loser
- nullifier set is consistent across all honest [[neurons]]

not guaranteed:
- which specific conflicting [[particle]] wins (depends on network propagation — the adversary has some influence over this via timing)
- how fast the conflict resolves (depends on spectral gap and degree of honest split)
- that the "better" particle wins in any semantic sense — the winner is the one that propagated faster, not the one that is "more correct"

## liveness

ergodicity of the transition matrix $P$ guarantees every valid [[particle]] accumulates $\phi^*$ mass over time

convergence rate depends on the spectral gap $\lambda$ of $P$: expected time to finality is $O(\log(1/\varepsilon)/\lambda)$ iterations. larger spectral gap means faster finality. dense, well-connected [[cybergraphs]] have larger gaps

during partitions: $\lambda$ drops for the disconnected subgraph, finality slows or halts. this is the correct behavior — the system refuses to finalize when it lacks global information

## sybil resistance

$\phi^*$ is weighted by staked [[tokens]], not by node count. creating 1000 [[neurons]] with zero stake produces zero $\phi^*$ influence. creating fake [[cyberlinks]] without stake backing produces negligible mass shifts

the cost of attacking $\phi^*$ is the cost of acquiring $> \frac{1}{2}$ of staked [[tokens]] — same economic security model as proof-of-stake, but the attack surface is the graph topology rather than a voting protocol

## finality

foculus provides deterministic finality: once $\phi^*_i > \tau$, the [[particle]] is final. no rollbacks, no probabilistic confirmation depth

the threshold $\tau(t) = \mu_{\phi^*} + \kappa\sigma_{\phi^*}$ adapts to the current distribution. when the network is decisive (low variance), $\tau$ is low and finality is fast. when the network is uncertain (high variance), $\tau$ rises and finality slows — the system self-regulates

## performance

| metric | classic BFT | nakamoto | foculus |
|---|---|---|---|
| leader | rotating proposer | miner (PoW lottery) | none |
| finality | 5-60 s | ~60 min | 1-3 s |
| throughput | 1k-10k tx/s | ~10 tx/s | ~10⁹ signals/s per GPU |
| validator scale | 10²-10³ | unbounded | unbounded |
| fault tolerance | 1/3 stake | 51% hash | 1/2 $\phi^*$ |

each iteration is a sparse matrix-vector multiply — embarrassingly parallel, no sequential bottleneck. single GPU (A100): ~50M edges at 40 Hz ≈ 2×10⁹ edge ops/s. with $K$ shards, throughput scales linearly

latency: compute ~0.2 s, 5-8 iterations, propagation ~0.4 s → worst-case finality ~1.4 s WAN

## economics

rewards proportional to the measurable shift in $\phi^*$:

$$\text{reward}(v) \propto \Delta\phi^*(v)$$

validators who add [[cyberlinks]] that meaningfully shift the stationary distribution earn more. this aligns incentives: the network rewards contributions to convergence, not mere participation

damping prevents concentration: $\phi^*_i \leftarrow \phi^*_i \cdot \gamma^t$, $\gamma \in (0,1)$. older or less-endorsed [[particles]] fade. the system forgets noise and retains what matters

## open questions

### solved (this document answers)

- what is a conflicting particle: formally defined via nullifier collision and author/epoch equivocation — a pure function of particle data
- how conflicts are detected without ordering: monotonic local index on nullifiers, independent of arrival order
- what data becomes canonical: the [[particle]] that crosses $\tau$ first wins. finalization commits nullifiers to $N$. every [[neuron]] computes the same $\phi^*$ from the same graph, so they agree

### open (requires further work)

- adversarial honest-split: the adversary can influence which conflicting [[particle]] propagates first to more honest [[neurons]]. quantifying the adversary's power to steer conflict outcomes under partial synchrony needs formal analysis. the safety proof shows they cannot cause double finality, but they may influence which single outcome occurs
- convergence time under conflict: when honest [[neurons]] split support ~50/50 (adversarial timing), how many iterations until the gap exceeds $\tau$? bounded by spectral gap and initial asymmetry, but no closed-form bound exists
- partition recovery: when two halves of the network reconnect, how quickly does $\phi^*$ reconverge? bounded by spectral gap, but practical latency under adversarial partitions is uncharacterized
- threshold gaming: can an attacker oscillate $\sigma_{\phi^*}$ to manipulate $\tau$? the adaptive threshold needs formal bounds on adversarial variance injection
- pre-finality state reads: before a conflict resolves, applications see ambiguity. the [[particle]] with higher current $\phi^*$ is the best guess, but it may change. specifying a safe API for pre-finality state queries (optimistic vs. pessimistic reads) is needed
- cross-particle dependencies: if $P_c$ depends on $P_a$'s output, and $P_a$ conflicts with $P_b$, then $P_c$ cannot finalize until $P_a$ does. long dependency chains affect throughput — quantifying this is open
- MEV within finality window: if multiple non-conflicting [[particles]] finalize in the same epoch, their relative ordering (by $\phi^*$ value) determines application state. extractable value from link timing needs analysis
- bootstrapping: a cold network has few [[cyberlinks]] and small spectral gap — finality may be slow until the [[cybergraph]] reaches sufficient density. minimum viable graph density for target finality latency is uncharacterized

---

[[consensus]] is not voted — it is computed

see [[collective focus theorem]] for convergence proofs. see [[tri-kernel]] for the operators. see [[focus flow computation]] for the full protocol specification. see [[cyber/state]] for the world state model. see [[cyber/security]] for the nullifier security proof
