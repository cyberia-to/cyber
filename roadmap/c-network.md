---
title: cyber network delivery
tags: cyber, roadmap, network, consensus, worker
crystal-type: plan
crystal-domain: cyber
status: planned
---

# C — verified network participant

Compose the A persistence/admission path and B computation capabilities into a
network participant. Network identity/versioning is specified during A; the
distributed behavior and release evidence are completed here.

## C1: open network instances

Owners: Cyber/OS protocol, Soft3 contracts, Trident/Joy adapters.

- [ ] Define versioned VM/OS/proof/network descriptors and compatibility checks.
  Pin genesis, domain/fork identity, rules, trust anchors and state codecs.
- [ ] Accept arbitrary compatible instances as data; isolate state, jobs, caches,
  credentials, receipts and relay cursors by network identity.
- [ ] Preserve identity across endpoint changes and reject the wrong network.

Exit: the same installed warrior serves two instances and a previously unknown
third one without a source enum change or rebuild. Unsupported combinations fail.

## C2: verified join and synchronization

Owners: Cybergraph, Foculus, BBG/Lens and transport owners; Cyber orchestrates.

- [ ] Freeze checkpoint/history/wire versions and verification policy.
- [ ] Verify checkpoints and state openings, retain admitted history, and resume
  bounded catch-up after interruption. Bound untrusted input before allocation.
- [ ] Handle competing histories, fork policy, peer failure and availability.

Exit: independent nodes converge from different starting states; forged, missing,
reordered and conflicting data cannot produce a verified-ready node.

## C3: finality, rewards and node modes

Owners: Foculus/Tru, Cybergraph, Zheng/Lens and Cyber.

- [ ] Connect live signal acceptance, epoch transitions and finality verification.
- [ ] Bind reward evidence to the actual challenge, domain/epoch, beneficiary and
  work; reject duplicate or stale work and persist effects atomically.
- [ ] Implement full/cell/light behavior according to the declared mode contract.
  Distinguish local progress from network finality in APIs and cyb.
- [ ] Specify protocol/parameter upgrades and state/primitive migrations.

Exit: a real multi-node scenario covers conflicting proposals, restart during an
epoch, verified light-client advancement and exactly one authorized reward effect.

## C4: deployment

Owners: Cyber admission, Joy deployment adapter, Trident artifact APIs.

- [ ] Specify publication bytes, program identity, network policy, signer authority,
  fees/resources where applicable, and the receipt/finality lifecycle.
- [ ] Implement deterministic offline preparation and explicit signed submission.
- [ ] Handle ambiguous submission, repeat requests, policy rejection and finality
  updates without rebuilding or weakening the frozen artifact/profile.

Exit: build → prove when required → prepare → submit → independently verify the
resulting network receipt works through the installed product and real nodes.

## C5: additional executors and network release

- [ ] Implement authenticated remote-worker transport, authorization and artifact
  transfer under the shared job model, with network-separated accounting.
- [ ] Add GPU/backend conformance where advertised. Preserve accepted semantics
  across hardware and expose actual resource/isolation guarantees.
- [ ] Run the complete node/worker/multi-network matrix against release artifacts.
  Ship only capability combinations supported by evidence.

Exit: [distribution](distribution.md) and the required
[cryptographic gates](cryptographic-readiness.md) pass for the exact release.

discover all [[concepts]]
