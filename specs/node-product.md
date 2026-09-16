---
tags: cyber, specs, soft3, cyb, joy, architecture
crystal-type: spec
crystal-domain: cyber
status: implementation
alias: node product, cyber binary
---

# node product

Cyber is the network product assembled from [[soft3]]. This repository owns
the executable entry point, configuration, product contracts, release artifacts
and explanatory graph. Component repositories own reusable algorithms.
[[specs/cli]] owns commands and [[specs/cyb-node]] owns client connections.

[Cyb architecture](../../cyb/specs/architecture.md) defines neuron as the
subject and prog as its durable work. [Domain roles](domain-ladder.md) separate
subjects from graph sessions, node participation modes, services, books and
shards. The [soft3 execution model](../../soft3/specs/execution-model.md)
defines warriors as VM/OS families and workers as running instances.

## composition

```text
cyb — named robot, attached neurons, private custody, UI
  ├─ soma → neuron execution → compatible workers
  ├─ GraphSession → cybergraph / BBG
  └─ explicitly pinned native HTTP client
       ↓
cyber — process, configuration, lifecycle, advertised capabilities
  ↓
soft3 — assembly and shared HTTP adapter
  ├─ cybergraph — native acceptance, Signal chains, history/recovery
  ├─ bbg / lens — transactional state and authenticated openings
  ├─ foculus / tru — ordering/finality and graph computation
  └─ nox / zheng / hemera / strata — execution, proofs and primitives

joy — warrior for the nox family
  └─ selected embedded, isolated or remote worker placement
```

Current `cyber node` calls `soft3::node::run` and shares its native
coordinator. Joy integration into this product remains a separate job-control
work package. Neuron's existing local Rune worker and soma's local model/tool
driver already have their own declared profiles. A dependency on a proof
library alone cannot establish verification at an ingress boundary.

## ownership

| part | owns / relationship |
|---|---|
| cyber | Product entry point, configuration, process lifecycle, contracts and release assembly |
| soft3 | Reusable stack assembly, adapters, SDKs and developer interfaces |
| cybergraph | One graph transition/history implementation reused by network hosts and multi-neuron GraphSession |
| bbg / lens | Durable transactions, application namespaces and supported authenticated state views |
| cyb | Named robot composition, explicit attachments, private custody, UI and client lifecycle |
| neuron | Subject model, durable progs, continuations, resource accounting, current-authority execution and recovery |
| soma | Task/context/tool/delegation/schedule orchestration over neuron execution |
| warrior / worker | Reusable target capability / running bounded placement |
| joy | Nox warrior integrating Trident, nox and Zheng; product scheduling integration remains a target |
| cybernode | Server deployment and operations for selected products and bootloader chains |
| true-cyber sibling | Current headless native client, sharing GraphSession/Registry/Host and signed-native retry/migration |

The node and a personal graph apply shared cybergraph transitions at their
declared state scopes. Each uses explicit database ownership. A GraphSession
may retain several authors; its database, process, address or home creates no
additional signing identity. The robot's attached neurons hold the relevant
subject authority and execute progs with data IDs.

The optional soft3 `stack` feature points to the local cyb facade and stays
disabled in this product. Foundational identity users can depend only on
neuron-id or the selected bounded model/crypto profile. GUI and inference
belong to applications that use them.

## current executable and storage

`cyber init` exclusively creates config, `config` displays it, `node`
holds its home lock and opens the shared native coordinator. BBG owns the
database writer fence. Independently launched hosts must respect that same
database ownership; a process-local configuration lock alone is insufficient.

The current database is `HOME/bbg`. Accepted operations, receipts and derived
state recover through Cybergraph/BBG. An existing flat `log` needs explicit
import before normal startup. `cyber storage import-legacy` imports through the shared
owner; `cyber auth enable [--import-legacy]` permanently activates signed
HTTP publication. Both operate offline under the same locks as `node` and
interoperate with `soft3 auth enable --home PATH [--import-legacy]`. They
preserve source/genesis bytes; authentication promotes the persistent reader
generation without generating a subject key. [[specs/cli]] defines their reports.

`status` validates cybermark and reports local state. `cyb` emits the v2
configuration-only connection descriptor with null live network/profile fields.
`cyb --live` validates bounded endpoint capabilities and reports signed submission
only when active. The endpoint catalogue alone makes no readiness claim. `--home` overrides `CYBER_HOME`,
then `~/.cyber/spacepussy-test`. Explicit product config remains required,
and this entry point accepts loopback listeners.

A fresh home has the unsigned local development profile until explicit
activation. An authenticated home advertises `neuron/signed-native/1`,
requires a verified subject-bound SignedAction for HTTP mutation and rejects
old unsigned ingress. Its network is H(canonical genesis bytes), independently
of the display label. The descriptor labels that observation as endpoint acceptance and retains
`consensus_finality: false`.

## supported local acceptance

The current adapter journals the exact verified action before native acceptance.
The native coordinator atomically retains accepted operations and request
receipts; exact retries return the original result, changed payloads conflict,
and ambiguous storage outcomes stop further acceptance until validated reopen.
Complete native Signals preserve authors, network, chain position and proofs.
The signed path validates its configured proof/economic rules before mutation.

Cyb and true-cyber use the same explicit subject/network binding contract,
complete-Signal outbox and bounded observation mirror. Current grants are
checked at dispatch; old attempted operations remain reconcilable after revoke.
Private notes, task continuations and history retain their owning graph/custody
contracts. Runtime commit index, SignalChain step, native operation position and
network finality remain distinct.

[Native adapter](../../soft3/specs/native-node.md) and
[signed profile](../../soft3/specs/signed-native-adapter.md) own exact wire,
limits, generation guards and recovery. Source-level checks are indexed in
[convergence evidence](../../soft3/audit/neuron-cell/implementation.md).

## delivery and remaining network gates

Local supported behavior covers explicit custody, durable acceptance/restart,
signed submission, exact retry, subject/network isolation and strict legacy
migration. Declared profiles and their test evidence determine the claim.
The broader reliable-node and network product roadmap still requires:

1. Complete storage failure/power-loss and artifact recovery gates for the
   selected platform and backend.
2. Complete the versioned product lifecycle and structured diagnostics, including
   bounded drain. Preserve the delivered live/configuration descriptor distinction
   and shared offline migration/authentication behavior.
3. Verified network join, coverage, peer replication, conflict policy, finality
   and upgrades, with readiness tied to those verified states.
4. Product worker control, authenticated placement, proof-bound job acceptance,
   cancellation and advertised resource/isolation guarantees.
5. Reproducible source closure, compatible wire/primitives and platform releases.

The [[specs/node-modes|full, partial and light modes]] remain target network
profiles until these duties are composed end to end. Current local height,
endpoint acceptance and history mirroring do not establish folded-tip trust
or distributed monetary finality.

Source builds use sibling path dependencies. Cargo.lock pins registry packages;
release provenance must additionally record component revisions and dirty source
state. A package version by itself does not reproduce a development artifact.

discover all [[concepts]]
