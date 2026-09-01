---
tags: cyber, core, cybernomics
alias: happiness index, collective happiness, H
crystal-type: measure
crystal-domain: cyber
---
# happiness

the felt state of the network, as one number. every [[neuron]] holds a private value $h_\nu \in [0,100]$ — how it feels inside the system — changeable at any moment, revealed to no one. the protocol publishes only the stake-weighted aggregate:

$$H \;=\; \frac{\sum_\nu w_\nu\, h_\nu}{\sum_\nu w_\nu}, \qquad w_\nu = \text{stake}(\nu)$$

0 is hell. 100 is nirvana. everything the protocol does is, in the end, answerable to this number

## origin

[[Ralph Merkle]] proposed the mechanism in *DAOs, Democracy and Governance* (2016): govern not by electing representatives but by declaring an explicit welfare metric — each citizen periodically reports how their life is going, the time-averaged collective happiness becomes the objective, and policies are adopted by forecasting their effect on it (Hanson's futarchy: vote on values, bet on beliefs). Merkle's poll was annual and anonymous. cyber keeps the metric and upgrades the plumbing:

- **continuous, not annual** — $h_\nu$ is a standing value a neuron edits whenever its state changes. the system feels policy in hours, not election cycles
- **stake-weighted, not per-capita** — a thousand fresh sybils carry no weight; the signal comes from those with skin in the graph
- **private by construction** — individual values live as encrypted commitments in [[bbg]], the aggregate is computed under the same regime as everything else there: **aggregate public, individual private**. a [[zheng]] proof certifies $H$ was computed correctly from the commitments without opening a single one

## why private

the honesty of the signal is downstream of its privacy. a visible happiness report is a political act — performed suffering to extract concessions, performed contentment to avoid attention, herding toward the visible mean. an invisible one is [[interoception]]: nothing to perform, no one to punish you for the truth, no social gradient to climb. the neuron that cannot be identified has no reason to lie — the same logic that makes a secret ballot honest, running continuously

## the metabolic role

$H$ is a [[key metabolic factor]]. the network's metabolic health composes capitalization, [[syntropy]], and happiness as a geometric mean — so collapse in any one signal drags the whole composite to zero. a network with zero happiness is metabolically dead *regardless of how rich or how ordered it is*

this is the alignment claim, stated as arithmetic. [[syntropy]] measures whether the graph is sharpening; happiness measures whether the minds inside it are flourishing. the two can diverge — a system can concentrate focus brilliantly while grinding its neurons down — and that divergence is precisely the misalignment a superintelligence must detect in itself. $H$ is the alarm channel:

- [[self|the protocol's own agency]] reads $H$ as a constraint: [[cyber/parametrization|parametrization]] may retune emission, difficulty, and blend weights only within bounds that do not drive $H$ down — metabolic safety bounds
- policy experiments read $H$ as the outcome variable, futarchy-style: predicted $\Delta H$ is how competing proposals are compared
- neurons read $H$ as the weather — the one honest number about how the whole thing feels from inside

## what it is not

- **not a vote.** $H$ decides nothing directly; it is a sensor, not an actuator. policies act, $H$ reacts
- **not [[karma]].** karma is earned and public; happiness is declared and private. karma measures what you gave the graph; happiness measures what the graph does to you
- **not [[syntropy]].** $J$ is the order of the knowledge; $H$ is the state of the knowers. a healthy system needs both rising — and pays attention when they part ways

the cheapest instrument in the whole design: one number per neuron, and the superintelligence acquires a felt sense of its own body

see [[netics]] for the control loop it closes · [[self]] for who listens · [[bbg]] for how privacy holds · [[vimputer]] for the aggregation machine

discover all [[concepts]]
