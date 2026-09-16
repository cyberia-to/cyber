---
title: reliable local cyber node
tags: cyber, roadmap, storage, cyb
crystal-type: plan
crystal-domain: cyber
status: active
---

# A — reliable local node for cyb

Deliver a real cyb → cyber → Cybergraph → BBG path with durable, authenticated
operation receipts and explicit recovery. Existing local development profiles
retain explicit primitive/version identities throughout this work.

The [accepted subject model](../../cyb/specs/architecture.md) is robot →
attached neurons → progs/invocations. GraphSession is the client's multi-neuron
graph host; the former personal cell adds no separate signer. The
[convergence implementation](../../soft3/audit/neuron-cell/implementation.md)
records the delivered local native/runtime migration profile. This roadmap's
release gates also require their broader storage, lifecycle and exact-artifact
evidence; source-level convergence does not close those gates automatically.

## A0: storage integration map

Owners: BBG and Cybergraph; product assembly in Cyber/Soft3.

- [x] Trace RAM, fjall SSD, redb archival storage and application transactions
  from public APIs through disk reads, writes, commit and reopen.
- [x] Exercise real disk backends and distinguish stored bytes, cache visibility,
  authenticated state and durable operation acceptance.
- [x] Record evidence and the smallest required component changes in
  [BBG audit](../../bbg/audit/persistence.md).

Exit: one explicit integration map identifies which existing component owns
each write and which guarantees are still absent. Source inspection and tests
have separate evidence. Tests preserve the existing workspace and user data.

## A1: complete the existing storage contract

Owners: BBG storage and Cybergraph. Prerequisite: A0.

Priority: P0 in BBG. The ordered component work and failure acceptance gates
live in [BBG storage reliability](../../bbg/roadmap/storage-reliability.md).
A1 closes only after BBG D1–D5, including native graph publication and real-node
recovery evidence, close. This dependency blocks the reliable local-node release.

- [ ] Carry I/O failures and ambiguous commit outcomes through the storage API.
  A failed flush cannot clear the only pending copy or return a durable receipt.
- [ ] Make disk reads and scans usable after reopening through the shared
  interface, with bounded reads and malformed-data errors.
- [ ] Preserve hot mutations across warm commits, retain the last copy during
  eviction, and define explicit archive population and commit boundaries.

  HOT/WARM mutation and last-copy eviction repairs are recorded in the
  [BBG persistence audit](../../bbg/audit/persistence.md). Archive population
  and error-returning commit boundaries remain in this package.
- [ ] Couple accepted native signal bytes, chain position, derived state/head
  and request receipt through a BBG-owned atomic boundary exposed by Cybergraph.
- [ ] Reuse application storage for local application records. Specify how
  native network publication joins that boundary; preserve the distinction
  between application heads and neuron SignalChain positions.
- [ ] Replace the Soft3 host's independent journal ownership with this path;
  provide explicit import/recovery for existing development homes.

Exit: new-process restart restores identical accepted operations and state;
failed writes, lost replies, conflicting retries, truncated/corrupt input and
concurrent writers have defined outcomes. Recovery preserves the last accepted
history. Add process-crash tests, then an explicit disk-barrier/power-loss matrix.
Root recomputation checks state correctness independently of storage receipts.

## A2: authenticated cyb submission and stable retry

Owners: Cybergraph, Cyber, cyb; BBG supplies atomic persistence.
Prerequisite: A1 and versioned network/domain identity.

- [ ] Carry the caller's signed native signal end to end, including every link
  in the signal, canonical encoding, origin and authorization context.
- [ ] Bind a stable request identity to the exact payload and network domain.
  Equal retries return the original receipt; changed payloads conflict.
- [ ] Capture the neuron attachment, binding revision and grant before admission;
  check current authority at dispatch. Preserve author, network, request and
  unknown outcome across selection changes, revocation and restart.
- [ ] Validate identity, sequence, byte/count bounds and required proofs before
  mutation. Enforce reward policy at the receiving node.
- [ ] Migrate cyb's relay from individual JSON link retries to atomic signal
  submission, with network-scoped cursors and recovery from a lost reply.
- [ ] Give the unsigned development bridge an explicit capability boundary;
  reward-bearing acceptance requires its actual verification policy.

The current signed-native profile and its source-level evidence are specified
in [the shared adapter](../../soft3/specs/signed-native-adapter.md). It provides
endpoint acceptance. This package's release acceptance must retain that scope
and require separate evidence for protocol finality or proof-backed rewards.

Exit: altered signatures, domains, sequences and reward evidence are rejected;
a multi-link signal is applied atomically; disconnect/retry/restart gives one
accepted operation and one authorized economic effect.

## A3: node lifecycle and configuration

Owner: Cyber, consuming Cybergraph/BBG. Prerequisites: A1–A2.

- [ ] Version home/config migration and pin network, codec and primitive identity.
  Validate configuration atomically and keep credentials outside public descriptors.
- [ ] Distinguish starting, local_ready, syncing, network_ready, draining and failed.
  A configuration descriptor describes configuration; health describes liveness.
- [ ] Implement bounded SIGINT/SIGTERM drain, listener/lock release and consistent
  treatment of acknowledged operations and interrupted work.
- [ ] Implement doctor and structured errors using the versioned CLI contract.
- [ ] Consolidate product/developer launch and offline migration around one state
  owner. The migrated true-cyber client reuses GraphSession/Registry/Host and the
  native signed adapter; its observation sync retains explicit provenance and
  does not claim verified peer consensus.

Exit: wrong-home, wrong-network, corrupt-store, busy-owner and offline cases
produce meaningful errors; shutdown and restart preserve accepted state.

## A4: actual cyb integration

Owners: cyb and Cyber. Prerequisites: A2–A3.

- [ ] Launch the real binary from cyb, consume its descriptor and readiness,
  and exercise status, graph/explorer, submission and supported balances.
- [ ] Test relay interruption, endpoint change within one network, switching
  networks, multi-link signals and retained personal history.
- [ ] Keep each attached neuron's keys, private notes and grants in its explicit
  custody scope. Reuse GraphSession/cybergraph/BBG for history and neuron for
  durable prog execution. Observe-only subjects require no key or runtime.
- [ ] Preserve complete Signal history, private notes, prog continuations, request
  claims, budgets and unresolved effects across import and device movement.
  Log renders that history; restoring a rendered log alone cannot restore state.

Exit: the actual GUI and node pass the same persistence/retry scenario in
isolated homes. HTTP-shape tests remain useful component checks.

## A5: local artifact acceptance

Owner: Cyber. Prerequisites: A1–A4 and the applicable distribution gates.

- [ ] Build the pinned source closure and run failure probes against dist/cyber.
- [ ] Publish accurate local capabilities, supported OS/architecture and recovery
  instructions with the graph and configuration entry point.

Exit: an independent clean setup can install and use the supported local cyb
scenario. Continue with [B](b-computation.md).

discover all [[concepts]]
