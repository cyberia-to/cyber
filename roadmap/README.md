---
title: cyber delivery roadmap
tags: cyber, roadmap, soft3, cyb, joy
crystal-type: plan
crystal-domain: cyber
status: active
alias: cyber roadmap, cyber delivery plan
---

# cyber delivery roadmap

The contract is [cyber/launch](../launch.md): phase 1 is the
bootloader reborn, bostrom and pussy on soft3 on 2026-11-05, with settlement
mining, fold, foculus consensus and privacy as non-negotiable cores and user
programmability deferred to the canary. This roadmap orders the engineering
work under that contract. Owner adopted the order A → C → D for phase 1 on
2026-09-18; B follows in phase 2.

| order | milestone | phase | deliverable |
|---|---|---|---|
| A | [local node](a-local-node.md) | 1 | cyb submits an operation, receives a durable receipt, and recovers the same result after retry or restart; BBG P0 storage gates close |
| C | [network](c-network.md) | 1 | independent nodes on independent machines converge, finalize by φ*, settle and fold rewards, and hold the three privacy invariants |
| D | content | 1 | every referenced particle resolves in cyb; media and PDF render; link → files → links navigation |
| B | [computation](b-computation.md) | 2 | one cyber binary builds, runs, proves and verifies user programs through shared Trident/Joy libraries; the first canary upgrade |
| throughout | [distribution](distribution.md) | 1 | reproducible binaries, configuration, explanatory graph and executable release checks; the friday release train |
| alongside | [cryptographic readiness](cryptographic-readiness.md) | 1 | finalized primitive/proof profiles and implementation evidence required by each claimed release |

The critical path is C: the pieces exist as tested libraries in foculus, zheng,
tru and mudra and have never run together as a network. A unblocks C's
durability; D runs beside them on its own conveyor. The first open dependency
in A remains [BBG P0: durable storage](../../bbg/roadmap/storage-reliability.md).

## architecture already selected

- Cyber owns the product, configuration, lifecycle and composition.
- Cybergraph owns signal validation, graph transitions and publication.
- BBG owns storage backends and authenticated state. Extend its existing RAM,
  fjall SSD and redb storage paths; use its existing application transactions
  where their semantics fit. Durable node state belongs behind Cybergraph/BBG.
- Foculus owns consensus, the beacon, settlement and fold; tru owns φ* and the
  single value oracle `impulse`; tok owns conservation and the mint; mudra owns
  identity and privacy.
- Soft3 owns shared composition contracts and developer interfaces.
- Joy is a reusable warrior library with a standalone developer CLI. In phase 1
  the chain runs only the protocol's own programs; user programs return in B.
- VM, OS, proof profile, network instance and executor are distinct selections.
  A new compatible network is configuration. Finite resource limits are explicit.
- Workers produce results; the node verifies expected statements and owns
  admission, finality and reward decisions.
- Verifiable private retrieval is a standard read-service capability, tracked
  in [C2.1](c-network.md#c21-verifiable-private-retrieval).

The [execution model](../../soft3/specs/execution-model.md) owns these invariants.
Product contracts are indexed in [specs](../specs/README.md).

## work order

Close each dependency before relying on it. Freeze identity and version
boundaries early; the core specs freeze on 2026-10-09 and features on
2026-10-23 per the launch calendar. Each work package names its owner,
prerequisites and observable exit criteria. Update the relevant specification
before changing an interface. Implement in the owning component and reuse it
from cyber. Keep changes reviewable as atomic commits.

Record observations, exact source revisions, command results and remaining
failures in the owning repository's `audit/`. The launch page tracks gates and
the property registry; this roadmap tracks work and dependencies. An unchecked
gate stays open until its executable evidence exists.

The existing [node audit](../audit/node-readiness.md) supplies initial failures;
the [BBG persistence audit](../../bbg/audit/persistence.md) supplies storage findings.

discover all [[concepts]]
