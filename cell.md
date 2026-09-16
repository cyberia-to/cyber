---
tags: cyber, core
alias: cells, cell ladder, cyber cell, ledger-cell, knowledge-cell, runtime-cell, building-cell
crystal-type: entity
crystal-domain: cyber
status: compatibility
---
# cell — historical terminology

The cell ladder grouped four holders of state under one name. The accepted
[domain model](specs/domain-ladder.md) gives each responsibility a precise owner:

| Historical term | Current role | Preserved responsibility |
|---|---|---|
| runtime-cell | [[neuron]] executing progs | Program state, continuations, work history, resource limits and recovery |
| building-cell | [[aos/apps|service]] and its progs | Shared service state, player governance, admission and delivery |
| ledger-cell | [[research/oikos|token book and issuer]] | Complete home ledger, issuance rules, conditional settlement and conservation |
| knowledge-cell | [[shard]] | Graph partition, availability, validation, boundary proofs and split/merge lifecycle |

Neuron is the protocol subject. The named robot attaches neurons; programs,
services, books and shards have data and routing identities under their own
contracts. A separate neuron is introduced when independent authority or
attribution is needed. State, an address and a lifecycle alone provide no such
authority.

The old client mode `cell` is now the partial node mode described in
[[specs/node-modes|node modes]]. Cyb's former extension organ is
[[cyb/parts/prog|prog]]. Cybergraph retains history, BBG persists it and
[[cyb/parts/log|log]] renders it. Tape provides framing.

This page preserves historical links and terminology. Original signed records,
legacy codec names and migration provenance retain their bytes. Biological
cells, virtual-machine pairs, storage cells and table cells retain their ordinary
meanings. [[spectral cell division]] retains its historical proposal title and
describes graph shard division.

See [[hierarchy]] for graph aggregation and [[3c]] for domain read/write/trade
contracts. The shared transport carries each role's explicit evidence and
authority requirements.

discover all [[concepts]]
