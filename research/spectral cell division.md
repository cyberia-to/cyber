---
tags: cyber, research, article
crystal-type: pattern
crystal-domain: cyber
status: proposal
alias: spectral cell division, laplacian cell division, cell division
---
# spectral cell division

Status: research proposal. The historical title preserves the biological
metaphor and earlier links. The object being divided is a [[shard]]: a region of
graph state under a validation and availability contract. Its open protocol
questions must be resolved before automatic division can become a consensus rule.

## the mechanism

When a shard approaches its declared capacity, spectral analysis can propose a
split boundary. Under a chosen weighting and normalization, the [[Laplacian]]
describes connectivity; a small second eigenvalue λ₂ can indicate weakly joined
communities. The Fiedler vector assigns each particle a coordinate. A sign cut
or threshold sweep proposes a partition, evaluated against edge-cut cost,
balance, data locality and capacity constraints. The spectral relaxation needs
an explicit cut objective and acceptance rule; its sign cut has no general
guarantee of being the exact minimum cut.

The proposed handoff replaces one serving region with two authenticated regions.
Particles, links, routing coverage and spending-state obligations acquire explicit
new owners at the partition layer; boundary [[focus]] state is exchanged with
coverage proofs. A spending commitment or pending cross-region operation retains
its protocol identity and conservation rules. Any [[mutator set]] or nullifier
representation must define how witnesses and uniqueness survive the boundary.

Repeated partitioning could produce the [[hierarchy]] of shards → zones → domains,
with the [[heat]] kernel suggesting resolution scales. Neuron keys and prog
identities remain independent of these partitions. Moving a prog's retained
state requires its runtime migration and writer-fencing contract in addition to
graph placement; spectral analysis supplies no permission to dispatch work.

## why it is not ready

open questions that keep this a proposal:

1. Atomic handoff. Which checkpoint closes the old region, who serves queries
   during migration, and what proves complete, disjoint new coverage? The protocol
   must preserve availability, spend uniqueness, open conditions and historical
   proofs while preventing both old and new writers from accepting conflicting work.
2. Validator economics. Who serves each new shard immediately, what stake and
   authority policy applies, and how does finality remain valid during reassignment?
   Graph partitions alone do not determine how validator stake should move.
3. Trigger and adversarial cost. Which capacity and spectral thresholds authorize
   division, who pays for measurement, and how are forced split/merge cycles or
   manipulated boundaries bounded? An eigengap supplies evidence for a decision.
4. Merging. Two under-used regions may benefit from reunion. The reverse handoff
   needs its own coverage, witness, routing and finality rules, including pending
   operations that refer to either former region.
5. Scale evidence. [[bostrom]] supplies an unpartitioned bootloader reference.
   A decision threshold requires a reproducible workload and measurements of
   capacity, boundary traffic and proof cost. A particle-count estimate alone
   cannot establish that division is necessary or economically beneficial.

## relation to oikos

[[research/oikos|oikos]] describes a token's complete home book under its issuer's
rules. Book creation registers an economic domain; shard division changes graph
coverage and serving responsibility. The operations have separate authorization
and proof requirements. A book can retain its name and full obligations through
storage repartitioning. This proposal owns graph partitioning; the
[domain model](../specs/domain-ladder.md) defines its relationship to neurons,
progs, books and services. The old [[cell|cell ladder]] remains a historical link.

discover all [[concepts]]
