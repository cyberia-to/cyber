---
tags: cyber, specs, cyb, node, api
crystal-type: spec
crystal-domain: cyber
status: implementation
alias: cyb node connection
---

# cyb node connection

The host executable is `cyber`; its local HTTP adapter comes from soft3.
Cyb retains a named robot, explicit neuron attachments, private custody,
durable progs and a multi-neuron GraphSession in its own home.
[Architecture](../../cyb/specs/architecture.md) defines these roles.
The node process and the client's graph have no implicit signing subject.

[[specs/cli]] owns the product command surface; [[specs/worker]] owns the
target computation/proving backend. The current authenticated local profile
is [signed native](../../soft3/specs/signed-native-adapter.md).
Endpoint acceptance and mirrored history are local observations; network
finality requires [[specs/node-modes]] and its supported proof profile.

## run and bind an endpoint

With sibling source checkouts:

```sh
cargo build --release --locked
./target/release/cyber init --bind 127.0.0.1:7780 --moniker local
./target/release/cyber node
```

The product exposes `status --json`, offline `cyb` and live `cyb --live`.
A fresh unpromoted home uses the explicit unsigned development profile.
With the node stopped, activate the signed profile for that exact home:

```text
cyber --home /absolute/node/home auth enable
```

An existing legacy log requires `storage import-legacy` before activation,
or the explicit `auth enable --import-legacy` combination. The shared soft3
developer command remains available with the same owner. Import and activation
retain original source bytes; authentication activation retires incompatible
genesis-reader paths. The database writer lock must be available; an old running
writer must be stopped before retirement. Reopen the same home with the
product's `node` command after successful activation.

Cyb's commander configures an endpoint and separately pins its identity:

```text
net add local http://127.0.0.1:7780
net probe local
net pin local NETWORK_HEX
```

For an existing name, use `net set local URL`. Probe output supplies the
advertised network and profile for inspection; the user/configuration pins the
expected 32-byte network explicitly. The profile must match
`neuron/signed-native/1`. Names and URLs without pins remain read-only.
Action routing uses its retained subject/network/profile, independently of
endpoint list ordering. A changed URL must serve the same pinned network.
A fresh genesis produces an independent network even when its display label
is also `spacepussy-test`.

A current controlled neuron attachment for that network is required to submit.
Watch-only attachments, a successful probe and UI selection alone grant no
signing permission. Switching selection preserves pending operations' original
subject, binding revision and destination.

## launcher contract v2

`cyber --home PATH cyb` emits `cyber/connection/v2` with
`observation: configuration-only`, absolute home, `network_label`, RPC,
mode and supported endpoint catalogue. `network`, `profile`, `submit`,
`native_capabilities` and `capabilities.authenticated_writes` are null.
It reads configuration without opening the database or contacting the node.

`cyber --home PATH cyb --live` observes `/capabilities` and validates its
schema, network ID, profile, action kinds, limits and idempotency/receipt
metadata. It emits `observation: endpoint-capabilities`, retains the exact
`native_capabilities` object and exposes its network/profile. Only a signed
profile supplies `submit: /v3/action` and adds `net pin` to
`cyb_commands`. A compatible unsigned observation keeps submit null.

The `endpoints` catalogue always contains capabilities/action/receipt/history
routes; this catalogue states adapter support independently of activation.
Both descriptors report `receipt_meaning: endpoint-acceptance` and
`consensus_finality: false`. Live requests have a five-second timeout,
64-KiB maximum and zero redirects; invalid/unavailable observations fail
nonzero. They grant no custody or current action authority.

V2 supersedes the old label-valued `network`, singular `cyb_command` and
unsigned `submit` fields of v1. Consumers select the schema explicitly and
preserve expected network/profile pins before sending. A `net set` command
only changes the configured URL.

A launcher starts `node` as a foreground child, checks `/health` and
validates `/status`. A descriptor reports configuration, health reports
liveness, and readiness/finality have their own verification requirements.
Startup errors exit nonzero on stderr. The product and offline import/auth hold the same home and authentication-upgrade
locks; BBG also fences the database writer. Bounded graceful drain remains a target
lifecycle requirement.

## HTTP profiles

| route | current behavior / consumer |
|---|---|
| GET /health | Text liveness, unavailable after ambiguous storage failure |
| GET /status | Cybermark local state and height for network views / diagnostics |
| GET /capabilities | Live schema, network, profile, kinds and bounds |
| POST /v3/action | Exact SignedAction; native/signal preserves the complete Signal; native/pay delegates sequence assignment to the endpoint |
| GET /v3/receipt/SUBJECT/REQUEST | Correlated prepared/accepted receipt for the retained action |
| GET /v3/history?after=POSITION&limit=N | Bounded complete native observations for client mirroring |
| GET /root, /stats, /blocks, /block/H | Local state and explorer views |
| GET /log, /v2/history | Explicit legacy/versioned diagnostic export formats |
| POST /v1/link, /v1/pay, /v1/frame, /v2/frame | Legacy unsigned ingress; rejected after authenticated-profile activation |

All routes reuse the same soft3 adapter and Cybergraph native coordinator.
BBG atomically retains accepted operations and receipts. Client GraphSession
mirroring preserves complete Signals and provenance, advances a network-scoped
cursor only after a complete projection and keeps local credits as observations.
Legacy byte offsets cannot seed the new operation cursor.

Signed retries bind exact subject, network, request, context and payload.
Lost replies retain unknown outcomes for receipt lookup; a new authorized
attempt uses the same bytes only under the profile's idempotency contract.
Revocation blocks dispatch while preserving read-only reconciliation.
A signed HTTP receipt establishes this endpoint's acceptance, with consensus
and economic finality remaining separate states.

## verification

Current source-level evidence for signed submission, wrong-network rejection,
multiple subjects/networks, legacy retirement, exact retries and actual client
processes is indexed in
[convergence implementation](../../soft3/audit/neuron-cell/implementation.md).
The product's `tests/node.rs` covers its launcher/config baseline. Release
claims require the exact artifact and the applicable lifecycle, storage and
network gates, independently of library-level conformance.

discover all [[concepts]]
