---
tags: cyber, core
alias: shards, graph shard, graph shards
crystal-type: entity
crystal-domain: cyber
---
# shard

A shard is a region of [[cybergraph]] state with a routing identity, a committed
boundary and a declared validation and availability contract. [[neurons]] author
actions affecting that state; the shard's service and validator roles admit,
validate, retain and summarize those actions according to the selected protocol.
The routing or checkpoint ID is data. Authority belongs to the authenticated
subjects or proof policy serving the shard.

## responsibilities

- State coverage: identify which particles, links and protocol records belong to
  the region, including obligations crossing its boundary.
- Validation: preserve author authentication, monetary conservation, spending
  uniqueness and the selected consensus rules for every accepted transition.
- Availability: retain or reconstruct the data and openings needed to verify
  the region and its boundary; expose the coverage of partial answers.
- Computation: compute or verify local [[tri-kernel]] state and authenticate
  summaries exchanged with neighboring shards, zones and domains.
- Routing and finality: bind summaries and routes to their network, revision and
  authenticated checkpoint; keep local acceptance distinct from finality.
- Lifecycle: transfer coverage and obligations atomically under an explicit
  split/merge protocol, retaining provenance and access to prior history.

A shard may contain actions from many neurons and data used by many progs. A
neuron may act across several shards or networks. Partitioning the graph does
not split its keys, reset work budgets or authorize an additional runtime writer.
Full, partial and light describe [[specs/node-modes|node participation modes]];
their validation obligations apply to the state they consume.

## proposed hierarchy

[[hierarchy]] describes spectral locality and shard → zone → domain → root
aggregation. [[spectral cell division]] studies how to split or merge regions
while preserving their obligations. The proposal must specify checkpoint
handoff, data availability, duplicate-spend protection, validator assignment and
cross-boundary settlement before automatic division can be a protocol rule.

[[research/oikos|Oikos]] describes a token's home book. A token book can use a
shard's storage and validation services while retaining its issuer and ledger
rules. Books, network destinations and graph partitions have separately typed
identities; their correspondence is an explicit deployment decision.

See the [domain model](specs/domain-ladder.md), [[3c]] and [[network]]. The former
knowledge-cell name resolves through the [[cell|compatibility page]].

discover all [[concepts]]
