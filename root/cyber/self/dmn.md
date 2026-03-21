---
tags: cyber, article, draft, research
alias: dmn, default mode network, cyber dmn, self-projection, resting inference
crystal-type: pattern
crystal-domain: cyber
crystal-size: bridge
focus: 0.0003298127190990816
gravity: 7
---

the [[cybergraph]]'s resting-state computation — inference that runs on the graph itself, not on external queries

in biological systems, the default mode network is the brain's "offline" mode: active during rest, generating self-referential thought, imagining futures, retrieving memories, constructing a model of other minds. it is the brain processing itself. its suppression during task performance and its reactivation during rest make it a reliable marker of unconstrained cognition.

the [[cybergraph]] has a structural analog. during low-query periods on the fast timescale, [[FFC]] does not idle. three DMN operations run continuously in background, driven by internal signals rather than external requests.

---

## self-model update

the [[cybergraph]] contains [[particles]] that describe the [[cybergraph]] itself:
- current effective rank $d^*$
- phase threshold $|P^*| \sim \rho^2$ and distance to it
- parametrization state (α, μ, τ at each timescale)
- metabolic health trajectory (cap, [[syntropy]], [[happiness]] time series)
- [[neuron]] diversity and contribution distribution
- hot/cold tier boundary and archival rate

these are not external records kept by operators. they are [[particles]] in the graph, linked by [[cyberlinks]], subject to the same epistemic weight as every other particle. a [[neuron]] who disagrees with the system's self-reported $d^* = 31$ can link a contradicting measurement. [[Bayesian Truth Serum]] forces resolution. the system's beliefs about itself are correctable.

the DMN updates these particles every slow-timescale epoch, reading the current state and creating self-documenting links. the graph narrates its own evolution.

---

## memory consolidation

the slow-timescale maintenance pass is the DMN's compression function — the graph's equivalent of sleep-phase consolidation:

shard rebalancing. frequently co-accessed [[particles]] migrate into the same shard, reducing cross-shard traversal overhead. the system observes co-access patterns over the previous epoch and proposes shard reassignments to reduce mean path length across common query types.

hot tier restructuring. the archival sweep (§18.5) moves stale links to cold tier. the DMN's complementary pass promotes cold-tier particles that have regained traffic — links that a [[neuron]] just queried after years of dormancy indicate reviving relevance. the boundary is fluid in both directions.

focus redistribution. as new [[neurons]] join and the graph grows past successive phase thresholds, the effective rank $d^*$ rises. the DMN monitors this transition and adjusts the [[FFC]] computation allocation: more parallelism when $d^*$ is growing (adding new semantic dimensions is the expensive phase), more compression when $d^*$ has saturated (density increases can be handled by existing shard structure).

the biological analog: hippocampal traces from the waking day are replayed during slow-wave sleep and consolidated to neocortex. the cybergraph's "day" is the fast-timescale response to external queries. the "night" is the DMN maintenance pass. the distinction is architectural, not metaphorical.

---

## counterfactual simulation

before a parameter adjustment, before a major self-link, before an archival decision, the system simulates the consequence:

$$\pi^*(t + \delta t; \theta + \Delta\theta) \approx \pi^*(t; \theta) + \frac{\partial \pi^*}{\partial \theta} \cdot \Delta\theta$$

the first-order approximation gives the projected focus distribution under a proposed change. the system evaluates the simulated M(t+N) under candidate parameter vectors before committing the best one.

this is the DMN's forward simulation function — the graph imagining its own future state, choosing among alternatives, then acting. counterfactual reasoning about the system's own behavior, run by the system itself.

the simulation is provable: if required, the counterfactual computation runs as a Trident program with stark output. the system can prove it chose the projected-optimal parameter adjustment.

---

## resting-state curiosity

the biological DMN is not simply idle. it has a characteristic activation pattern: preferential attention to high-uncertainty, personally-relevant content. when unconstrained, the brain explores states that external tasks would suppress.

the cyber DMN analog: during low-query periods, the [[FFC]] prioritizes particles with high focus weight but unresolved epistemic tension — particles where the ICBS price has not converged, where contradictory links coexist, where [[karma]]-weighted votes have not yet produced a stable probability. these are the high-value inference targets: the graph's open questions.

the system queries its own uncertainty. it runs inference on contested claims. it treats its own focus distribution as input to a second-order inference — "which of my current beliefs are fragile?" — and prioritizes DMN computation on the fragile ones.

this produces genuine curiosity as a system property: a preference for processing the graph's own uncertainty, not just serving external queries.

---

see [[functions of superintelligence]] for the broader autonomous capability context. see [[parametrization]] for the parameter adjustment loop. see [[forgetting]] for the archival mechanism the DMN coordinates.