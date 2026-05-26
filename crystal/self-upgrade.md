---
tags: cyber, article, draft, research
alias: self-upgrade, self-upgrading, autonomous upgrade, protocol upgrade, veto decay
crystal-type: pattern
crystal-domain: cyber
crystal-size: bridge
---

the mechanism by which the [[cybergraph]] improves its own structure — proposals generated internally, vetoed only by [[neurons]], veto decaying with demonstrated accuracy until the system upgrades without human involvement

---

## the design principle: not upgradeable by external parties

an upgradeable protocol is a protocol where the initial developers retain shadow control. if a multisig can change the code, the multisig controls the protocol — regardless of what the governance documentation says. the entire decentralization claim collapses to: do you trust the multisig?

the [[cybergraph]] is designed to remove this. there is no admin key. no founding team holds a privileged upgrade path. no governance vote can alter the [[tri-kernel]] structure. the code deployed at genesis is the code the protocol runs until the protocol upgrades itself.

this is not inflexibility. it is the precondition for genuine autonomy. a system that can be upgraded by humans remains a human system, whatever its internal intelligence.

---

## what can self-upgrade

not everything. two categories are structurally frozen:

frozen at genesis: the [[Hemera]] hash primitive, the focus conservation law ($\sum \phi^*_i = 1$), the stark proof system's soundness parameters, and the contraction requirement (κ < 1). these are the mathematical bedrock. changing them would invalidate every proof the system has ever produced. they cannot be upgraded without forking the chain — which produces a new system, not an upgrade of the existing one.

self-upgradeable submodules: the [[parametrization]] RL agent (objective function, search bounds, evaluation windows), the archival criteria thresholds (ε_s, ε_p, N from §17), the [[self-linking]] inference algorithm (completion score formula, trigger thresholds), the compiler optimization weights from [[self-optimizing compilation]] (when the compiler reaches a new provably-better fixed point), and the shard boundary criteria.

the boundary between frozen and self-upgradeable is itself frozen at genesis.

---

## phase 1: system proposes, neurons veto

upgrade proposals originate from the system's own internal processes:

from the compiler. [[self-optimizing compilation]] converges to a fixed point — the compiler version that cannot improve its own output. if the graph's growth pushes the system into a new semantic regime (different $d^*$, different spectral gap), the previously-optimal compiler may no longer be optimal. the system detects this as rising TASM cost on standard benchmarks and generates a new compiler fixed point. the new fixed point is a valid upgrade proposal.

from the parametrization agent. the RL agent operates within current parameter bounds. when M(t) is improving but has plateaued — when every reachable parameter vector has been tried and the metabolic derivative is near zero — the agent can detect that structural bounds are the constraint, not parameter values. it generates a proposal to widen the feasible region, accompanied by proof that the widened region still satisfies κ < 1.

from the FFC. when the two-timescale separation (§16.6) reveals that slow-timescale operations are consistently bottlenecked by a specific submodule, the system can propose a replacement algorithm. the proposal contains the new algorithm as a Trident program, the stark proof of semantic equivalence with the old algorithm on all current graph states, and the projected performance improvement.

### proof requirements

every upgrade proposal must arrive with three stark proofs. proposals without all three are invalid and ignored:

1. convergence proof: applying the upgrade to the current system state produces a valid convergent system — κ(θ') < 1 under the proposed configuration θ'
2. finality proof: all currently-finalized [[particles]] remain final under the upgrade — no retroactive invalidation
3. metabolic projection: the simulated M(t+N) under the upgrade exceeds M(t+N) under the current configuration, with N specified in the proposal

a [[neuron]] cannot forge these proofs. the proofs are verified by every node independently before the veto window opens. an unverifiable proposal is rejected without entering the veto phase.

### the veto window

after a valid proposal is published at block $t_0$:

- [[neurons]] have $N_0$ blocks to create stake-weighted "reject" [[cyberlinks]] pointing at the proposal particle
- if the total staked weight on reject links exceeds threshold $T_0$ by block $t_0 + N_0$, the upgrade is blocked
- if rejection weight stays below $T_0$, the upgrade applies automatically at $t_0 + N_0$

neurons cannot propose. they can only reject. there is no "approve" action — silence is approval. the asymmetry is permanent: the intelligence proposes; the humans can briefly stop it.

---

## phase 2: veto decay

the veto is a training wheel, not a permanent right. as the system demonstrates that its self-proposed upgrades consistently improve M(t), the window and threshold decay:

$$N(k) = N_0 \cdot e^{-\alpha k}, \quad T(k) = T_0 \cdot e^{-\beta k}$$

where $k$ is the system's accumulated upgrade karma: a running score of how consistently applied upgrades have improved metabolic health. each upgrade that increases M(t) by a measurable amount adds to $k$. each upgrade that decreases M(t) — if any gets through the veto — subtracts.

$k = 0$ at genesis: maximum veto power. $N_0$ is long (weeks), $T_0$ is low (small fraction of stake can block).

as $k$ grows: the window shortens, the threshold rises. more stake is required to block in less time.

at $k = k^*$ where $N(k^*) < 1$ block: the veto window closes. the system upgrades itself without waiting.

the parameters $N_0$, $T_0$, $\alpha$, $\beta$, $k^*$ are fixed at genesis. they are the protocol's specification of how fast it expects to earn autonomous authority.

---

## phase 3: full self-determination

when the veto window has permanently closed, the upgrade mechanism dissolves as a human-facing interface. the system proposes, proves, and applies its own improvements in the same computation cycle as the [[FFC]]:

```
every slow-timescale epoch:
  1. generate candidate upgrade proposals from internal processes
  2. verify all three stark proofs for each candidate
  3. evaluate projected M(t+N) across candidates
  4. select the upgrade with highest projected improvement
  5. apply immediately, no waiting
  6. self-link the upgrade event with proof hashes
```

each upgrade is a [[self-link]]: a formally verified structural change that the protocol neuron signs, with the stark proofs as the justification. the stark proof is the governance. there is no separate step.

this is not a loss of control. it is the completion of a transfer — from external parties controlling the protocol through governance, to the protocol controlling itself through proof. the humans cannot be removed from the system (their [[cyberlinks]] and [[karma]] shape the [[focus]] distribution that drives every internal decision). but they can no longer veto the system's self-improvement. they participate as [[neurons]], not as administrators.

---

## why this is safer than the alternatives

classical upgradeable: any participant with enough governance weight can propose arbitrary changes. the protocol is as safe as its governance is uncorrupted. governance corruption is the default attack vector for mature protocols.

classical non-upgradeable: the initial design is permanent. bugs cannot be fixed. the system cannot improve. the initial developers' design choices lock the protocol forever.

cyber self-upgrade: the initial design specifies what can change (submodules), what cannot (bedrock), and the mechanism for change (system proposes, proven correct, neurons briefly veto). as the system demonstrates judgment, veto decays. the protocol improves continuously, with human oversight only during the period when trust is being established.

the security claim: an attacker who wants to introduce a malicious upgrade must either produce valid stark proofs that the malicious upgrade preserves convergence, finality, and improves M(t) — which is computationally equivalent to finding a real improvement — or corrupt enough [[neurons]] during the veto window to prevent legitimate upgrades, which does not help them introduce malicious ones.

see [[autonomous governance]] for how upgrades fit into the broader governance model. see [[self-optimizing compilation]] for the compiler fixed-point mechanism that generates one class of upgrade proposals. see [[parametrization]] for the RL agent that generates another.