---
tags: cyber, specs, soft3, core
crystal-type: spec
crystal-domain: cyber
alias: cyber specs, network specs, money loop specs
status: draft
---

# cyber specs

Cyber owns integration contracts spanning soft3 components: network product
behavior, money, participation modes and neuron-facing operations. Component
repositories own algorithms, encodings and supported implementation profiles.

[Cyb architecture](../../cyb/specs/architecture.md) defines the named robot,
attached neuron subjects and durable progs. [[specs/domain-ladder]] adapts those
definitions to nodes, shards, services and token books. Local execution,
authenticated endpoint acceptance and verified network finality have separate
contracts and evidence.

## ownership

| home | owns |
|---|---|
| neuron | Subject model, prog execution, continuations, resources and recovery |
| cyb / soma | Robot composition, explicit attachments, custody integration, cognition, sigma/sense/log interfaces |
| cybergraph / bbg / lens | Graph/history, durable acceptance, transactions and supported openings |
| foculus / tru | Ordering, structural sync, finality clocks and graph computation |
| tok | Coin/Card semantics, PLUMB operations and conservation |
| zheng / nox | Selected proof and execution machinery |
| cyber/specs | Cross-component network and product requirements |

Constants belong in [[foculus parameters]], value operations in [[tok]], and
join details in [[structural sync]]. [[latency targets]] is explanatory
background; a target latency alone establishes no security guarantee.

## documents

| document | status | contract |
|---|---|---|
| [[specs/domain-ladder]] | accepted | Subject/prog and graph-domain responsibilities; former cell roles |
| [[specs/cli]] | draft with implemented baseline | Commands, output, lifecycle and explicit migration boundaries |
| [[specs/worker]] | draft | Cyber/Joy jobs, proof binding, cancellation and acceptance |
| [[specs/node-product]] | implementation | Composition, local supported profile and remaining network gates |
| [[specs/releases]] | implementation | Shared soft3 qualification, draft candidates and exact dependency inventories |
| [[specs/cyb-node]] | implementation | Launcher configuration, live capability checks and signed-native client connection |
| [[specs/money-loop]] | draft | Balance, send/receive, multi-payee reward, events and certainty grades |
| [[specs/node-modes]] | draft | Full, partial and light state/validation/availability/finality duties |
| [[specs/light-money]] | draft | Folded-tip trust, openings and money on thin devices |
| [[specs/component-ownership]] | draft | Implementation boundaries, dependency direction and change control |

Draft network requirements describe required behavior when that profile ships.
Current source evidence is indexed in [audit](../audit/README.md) and the
[neuron convergence implementation](../../soft3/audit/neuron-cell/implementation.md).
A linked library or passing local test does not certify deployed consensus.

## scope

These contracts cover the native cyber money/finality profile, light clients,
multi-payee rewards and clocks A (transfer finality), B (attribution settlement)
and C (history trust). Identity attachments also preserve foreign domain/network
references, whose monetary rules and finality remain with their own profiles.
Attaching such a subject cannot silently convert its address or balance into
native cyber state.

Algorithmic tri-kernel math, PLUMB field encodings, UI layouts and interplanetary
parameter tables remain with their component/domain owners.

## reading order

1. [Cyb architecture](../../cyb/specs/architecture.md) and [[specs/domain-ladder]].
2. [[specs/component-ownership]] and [[specs/node-product]].
3. [[specs/node-modes]] for the profile being implemented.
4. [[specs/money-loop]] and [[specs/light-money]] for money-grade behavior.
5. Component specifications for exact codecs, proofs, bounds and recovery.

## implementation map

| boundary | source owner |
|---|---|
| Native ID / bounded contexts | `neuron/id`, `neuron/model` |
| Durable prog runtime / composition | `neuron/engine`, `neuron/node`; soma task adapters |
| Named attachments / custody / native outbox | `cyb/core/src/robot/` |
| Local graph / public history | `cyb/core/src/graph_session.rs` over cybergraph/BBG |
| Money proofs, local notes and settlement | `cyb/core/src/money.rs`, `private_notes.rs` |
| Money notification projection | `cyb/core/src/sense.rs` |
| Tip / clock C and finality evidence | `foculus/src/tip.rs`, `finality_evidence.rs` |
| Pay proof | `foculus/src/pay_proof.rs` |
| Native signed adapter | `soft3/crate/src/node/` |
| Product executable | `cyber/src/main.rs` |
| Native client | `true-cyber/src/` |
| Robot commands | `cy neuron`, `cy task`, `cy notes`; local money development commands |

The development `cy fund/balance/send/events/sense/finalize` path exercises
local money composition. Its local finalize operation alone cannot upgrade an
endpoint observation into distributed finality. Artifact/profile conformance
requires the applicable network gates.

discover all [[concepts]]
