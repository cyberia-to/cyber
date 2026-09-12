---
title: cyber delivery roadmap
tags: cyber, roadmap, soft3, cyb, joy
crystal-type: plan
crystal-domain: cyber
status: active
alias: cyber roadmap, cyber delivery plan
---

# cyber delivery roadmap

Deliver a reliable node to cyb, extend the same cyber binary with Joy's
computation capabilities, then ship a verified network participant.
The owner adopted this order on 2026-09-11: A → B → C.

The first open dependency is
[BBG P0: durable storage](../../bbg/roadmap/storage-reliability.md).
It is BBG's highest delivery priority and blocks A1 and reliable local-node
acceptance until the backend and end-to-end recovery gates close.

| order | milestone | deliverable |
|---|---|---|
| A | [local node](a-local-node.md) | cyb submits an operation, receives a durable receipt, and recovers the same result after retry or restart |
| B | [computation](b-computation.md) | one cyber binary builds, runs, proves and verifies programs through shared Trident/Joy libraries |
| C | [network](c-network.md) | compatible network instances, verified synchronization, admission, finality and deployment |
| throughout | [distribution](distribution.md) | reproducible binaries, configuration, explanatory graph and executable release checks |
| alongside A–C | [cryptographic readiness](cryptographic-readiness.md) | finalized primitive/proof profiles and implementation evidence required by each claimed release |

## architecture already selected

- Cyber owns the product, configuration, lifecycle and composition.
- Cybergraph owns signal validation, graph transitions and publication.
- BBG owns storage backends and authenticated state. Extend its existing RAM,
  fjall SSD and redb storage paths; use its existing application transactions
  where their semantics fit. Durable node state belongs behind Cybergraph/BBG.
- Soft3 owns shared composition contracts and developer interfaces. Consolidate
  its current node host around the product contract as integration progresses.
- Joy is a reusable warrior library with a standalone developer CLI. Cyber
  embeds it; isolated and remote workers use the same logical job contract.
- VM, OS, proof profile, network instance and executor are distinct selections.
  A new compatible network is configuration. CPU/GPU selection preserves
  warrior identity. Finite resource limits are explicit.
- Workers produce results; the node verifies expected statements and owns
  admission, finality and reward decisions.
- Verifiable private retrieval is a standard read-service capability, hosted
  by the node or delegated to compatible infrastructure. Cybergraph/Inf define
  complete evaluation over authenticated state; Mudra supplies privacy
  operations. Delivery is tracked in [C2.1](c-network.md#c21-verifiable-private-retrieval).

The [execution model](../../soft3/specs/execution-model.md) owns these invariants.
Product contracts are indexed in [specs](../specs/README.md).

## work order

Start at A0 and close each dependency before relying on it. Work on the
cryptographic and distribution tracks can proceed alongside the integration
sequence. Freeze identity and version boundaries early; network consensus
integration follows the local acceptance and worker contracts.

Each work package names its owner, prerequisites and observable exit criteria.
Update the relevant specification before changing an interface. Implement in
the owning component and reuse it from cyber. Preserve supported callers or
provide an explicit migration. Keep changes reviewable as atomic commits.

Record observations, exact source revisions, command results and remaining
failures in the owning repository's `audit/`. This roadmap tracks work and
dependencies. An unchecked gate stays open until its executable evidence exists.
Success for A or B establishes that milestone's scope; C and production
cryptography have their own acceptance criteria.

Continue with [A1: complete the storage contract](a-local-node.md#a1-complete-the-existing-storage-contract)
through [BBG P0](../../bbg/roadmap/storage-reliability.md).
The existing [node audit](../audit/node-readiness.md) supplies initial failures;
the [BBG persistence audit](../../bbg/audit/persistence.md) supplies storage findings.

discover all [[concepts]]
