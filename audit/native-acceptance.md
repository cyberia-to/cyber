---
title: local native acceptance artifact
tags: cyber, audit, storage, binary
date: 2026-09-12
---
# local native acceptance artifact

The local node now commits graph changes, native history, economics, block
metadata and request receipts through one BBG Database transaction. HTTP
success follows durable commit. The old host-owned append/replay path is
removed. Existing homes require explicit strict import; source files remain.

## artifact and executed checks

`nu scripts/release.nu` completed successfully on macOS 26.4.1, Darwin 25.4.0,
aarch64, APFS on the internal SSD, using Rust 1.95.0. It ran formatting,
`cargo test --locked`, `cargo build --release --locked`, and all 11 tests in
`tests/node.rs` against the release executable via `CYBER_TEST_BINARY`.
The ordinary test run also passed its one unit test and 11 integration tests.

Artifact: `dist/cyber`, version `cyber 0.8.0`.

```text
SHA256 79881675117208d6741e9ec46d983b5f47a2dbb6400f544fa972232585d9c4a5
Cargo.lock SHA256 ca25bc3805b898b373f4507e7714aa3aeaeadbeb0f8923fd380a8d006c77b72c
```

`dist/SHA256SUMS` and `dist/build.json` accompany the executable. Build provenance
records Cyber `4d68741d`, Soft3 `4a36269`, Cybergraph `c9836ae`, BBG `568a0d8`,
Foculus `27e284c`, all other local dependencies, registry lock and dirty status.
Subsequent documentation commits do not change this binary identity.
Concurrent BBG proof/query/state/package and neuron identity work was preserved
and is included in the recorded working source closure. This is a tested local
development artifact; its provenance does not claim a clean reproducible checkout.

The real-process checks cover:

- dropped HTTP response, observed commit, SIGKILL and restart, followed by the
  exact original link/payment receipt and one economic effect;
- identical/conflicting IDs, reusable server-assigned IDs, and history cursors;
- native subsidy, payment, intent persistence and atomic whole-batch rejection;
- complete v2 network/envelope retention and refusal to silently export it in
  the older lossy wire format;
- historical frame bytes preserved through explicit import and restart;
- truncated/malformed logs, source mutation, changed/malformed genesis and
  corrupted persisted block root preventing readiness;
- exclusive home ownership and incomplete/ambiguous/oversized HTTP rejection.

The [Cybergraph checks](../../cybergraph/audit/native-acceptance.md) passed
20 library, 5 application, 10 native storage and 11 import tests. They include
real proof verification after disk recovery and staged-failure rollback.
[BBG composition tests](../../bbg/audit/native-record-composition.md) cover
Fjall/redb atomic record composition and actual redb write/sync/lost-ack faults.
Soft3's four unit tests pass. Strict Clippy passes for Cyber and Soft3;
component Clippy retains documented pre-existing warnings.

## scope and open gates

The supported profile is local RAM plus Fjall SSD with checked recovery and
explicit request IDs. The receipt identifies local durable acceptance.
Authentication, verified reward admission and distributed consensus remain
separate capabilities. Cyb's existing relay still needs stable persisted request
IDs and atomic signal/cursor handling before GUI retry acceptance can close.

Process termination and injected storage faults were exercised. Physical power
cuts, actual filesystem exhaustion, archive movement, other OS/architectures
and a real GUI launch were not exercised. Full Cybergraph stack tests currently
fail compiling obsolete Nox test APIs; the linked audit names those failures.
The passing node/targeted storage checks do not claim that broader suite passes.

The [roadmap](../roadmap/a-local-node.md) retains these open release gates.
