---
tags: cyber, specs, soft3, architecture
crystal-type: spec
crystal-domain: cyber
alias: component ownership, money ownership matrix
status: draft
---

# component ownership

Cyber owns cross-component network product contracts. Component repositories own
their algorithms and reusable boundaries. [Cyb architecture](../../cyb/specs/architecture.md)
defines one subject, neuron, with durable progs; [domain roles](domain-ladder.md)
separate that subject from nodes, shards, services and books.

[[specs/node-product]] describes the executable composition and supported local
profile; [[specs/cyb-node]] describes client connections. The
[convergence evidence](../../soft3/audit/neuron-cell/implementation.md) records
implemented identity, runtime and migration behavior. Network release status
remains in [node readiness](../audit/node-readiness.md) and the network roadmap.

## 1. ownership matrix

| concern | contract / implementation owner | consumers |
|---|---|---|
| Native NeuronId bytes | neuron-id; H(compressed pubkey) in the supported mudra profile | Foundational libraries, identity-only SDKs, graph and applications |
| Foreign subject / network / attachment context | neuron-model; cyb robot registry composes named attachments | Robot, CLI, soma, native adapters |
| Prog state, invocations, continuations, reservations, unknown effects | neuron-engine; neuron-node composes graph/authority/worker ports | Soma, services, host CLIs |
| Goals, task context, tools, delegation, schedules, learning proposals | soma over neuron execution | Robot and applications |
| Current grants / revocation and secret operations | ward / vault; mudra and proof profiles authenticate | Subject-bound admission and dispatch |
| Signal application, multi-neuron GraphSession, history and native acceptance | cybergraph | Cyb, node, true-cyber and adapters |
| Durable state, transactional application records and openings | bbg / lens | GraphSession, full/partial/light implementations |
| PLUMB pay/mint/burn and Intent laws | tok + supported nox/zheng circuits | Cybergraph, foculus, sigma |
| Ordering, nullifiers, finality φ* > τ, epoch attribution | foculus / tru with tok mint semantics | All declared network modes |
| Proofs, accumulator fold, decide | zheng and foculus tip/finality-evidence adapters | Full/partial/light verification paths |
| Transport / framing / replicated data | radio / tade; structural-sync protocols | Node and client adapters |
| VM/OS family and running bounded executor | Warriors / workers under the soft3 execution model | Neuron and node job orchestration |
| Configuration / cognition / visualization / devices | Cyb soul / soma / avatar / body | Named robot composition |
| Identity and holdings workflows / notifications / history rendering | Cyb sigma / sense / log | Human and program interfaces |
| Process configuration, network readiness and releases | cyber over soft3 assembly | Local and network deployments |

Runtime budgets reserve and account resources; protocol balances, stake, focus
and karma keep their protocol owners. A prog has an addressable data ID and
executes under a neuron; installation creates no second signing subject. A
GraphSession retains many authors without a process signer. Log renders graph
history, while tade supplies frames and BBG supplies durability.

## 2. dependency direction

Foundational BBG/tok/transport clients may import the dependency-free neuron-id
contract. Identity consumers may add the bounded model and selected crypto
profile. Engine, node, VM, GUI and inference dependencies belong to hosts that
actually execute or compose those capabilities.

Cyber and cyb compose the same cybergraph implementation. Soft3 assembles
components; its optional local `stack` facade remains an explicit feature.
Neuron owns durable execution semantics and delegates bounded work to compatible
workers. Soma orchestrates cognition through that execution contract. Inf's
optional neuron query adapter reads retained projections without gaining grant
or dispatch authority. Radio endpoint keys and tade display labels do not grant
subject access.

## 3. profile and change control

An ownership row names the responsible component, rather than certifying a
released integration. Local runtime commit, authenticated endpoint acceptance,
network ordering and verified finality have separate evidence and readiness.

- Changes to clocks, grades, tip objects or authority contexts require versioned contracts.
- Parameter changes belong in [[foculus parameters]].
- Domain/wire migrations preserve original bytes and explicit provenance.
- New node modes retain the same validation, nullifier, coverage and finality obligations.

discover all [[concepts]]
