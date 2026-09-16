---
title: native neuron launcher migration
tags: cyber, audit, neuron, soft3
crystal-type: audit
crystal-domain: cyber
date: 2026-09-13
---
# Native neuron launcher migration

P12 updates the `cyber` product launcher to the migrated soft3 native storage and
publication profile. Product version remains 0.8.0, unpublished. The tested Rust
baseline is Homebrew rustc/cargo 1.95.0 on aarch64 macOS. Cargo.lock resolves the
current sibling components, including BBG durability and neuron identity/model.

## Operator paths

`cyber init` exclusively creates config.toml; an existing legacy home can receive
that configuration while retaining its genesis, log and custody files. The
normal `node` command requires an explicit import when a legacy log exists.

With the node stopped:

```nu
cyber --home /path/to/node storage import-legacy
cyber --home /path/to/node auth enable
```

The combined command is `cyber --home /path/to/node auth enable --import-legacy`.
It uses the same owner APIs as `soft3 auth enable [--import-legacy]`.
`storage import-legacy` calls soft3's strict native importer; it preserves exact
log frames, authors, order, source digest and legacy network bytes. Exact retries
resume or confirm the same source. Authentication monotonically promotes the
BBG reader generation and retires the original genesis file into
`genesis.json/source`; its retirement manifest blocks old flat-file executables.
Source retention and the imported database remain required for reopening.

Node lifetime and both offline commands hold `node.lock` and
`auth-upgrade.lock`. The latter also excludes the soft3 authentication command;
the underlying BBG store retains its own exclusion. Activation is explicit,
permanent and creates no neuron key or robot attachment. Before activation, the
declared local unsigned profile retains its existing behavior. After activation,
the old v1/v2 mutation routes return 401 and the native signed profile is active.

`storage import-legacy` emits `cyber/legacy-import/1` with event/signal counts,
height, root, source digest and `source_retained=true`. `auth enable` emits
`cyber/authentication/1` with native network ID, signed profile, authentication
state, optional import report and `consensus_finality=false`.

## Connection descriptor

`cyber cyb` emits `cyber/connection/v2` from configuration only. It reads neither
BBG nor genesis and creates neither. `network_label` is the configured friendly
name; `network`, `profile`, `submit`, `native_capabilities` and
`capabilities.authenticated_writes` are null until explicitly observed.
`observation` is `configuration-only`.

`cyber cyb --live` reads and validates `/capabilities`; `observation` becomes
`endpoint-capabilities`. A supported observation carries its actual 64-hex
network ID, profile and authentication state. Only the signed profile produces
`submit=/v3/action` and a `net pin spacepussy-test NETWORK` command after the
`net set spacepussy-test URL` transport command. The unsigned profile keeps
`submit=null`. Pinning and custody remain explicit operator actions.

The `endpoints` object is a route catalog: capabilities `/capabilities`, action
`/v3/action`, receipt `/v3/receipt/{subject}/{request}`, history `/v3/history`.
Its presence alone does not report route activation. `native_capabilities`
retains the validated endpoint observation. Receipts mean endpoint acceptance;
the descriptor declares `consensus_finality=false` and no consensus/peer-sync/
Joy-worker support. Endpoint metadata is an observation from the configured
transport, not a proof of the local home or network finality.

Status and capabilities reads require HTTP 200, forbid redirects, use a five
second timeout, and accept at most 64 KiB. Config reads accept at most 16 KiB and
monikers at most 256 bytes. Invalid/foreign/inconsistent capabilities,
unavailable endpoints, oversized replies and malformed inputs fail with a
nonzero exit status, with no fallback activation or unsigned publication.

## Verification

`cargo test --locked --all-targets` passed: one unit test, six process integration
tests, no failures or ignored tests. The readiness example compiled as an
all-target with zero tests; it is not counted as behavioral evidence.

`cargo build --release --locked` passed. The same six process scenarios also
passed with `CYBER_TEST_BINARY` set to the resulting release executable. Its
SHA-256 at verification was
`9c8310a7db5661c44fb033e207fc90b184384dd1a8e75023fbe3e0b52999e572`.
The dependency build reported three existing fjall warnings; the launcher
checks reported no failures. All test-owned node processes were stopped.

| Scenario | Evidence |
|---|---|
| Fresh config-only descriptor; no database/genesis creation; explicit live observation of unsigned profile | `cyb_wire_write_restart_and_exclusive_home` |
| Preserved legacy unsigned write/restart and exclusive product home | same process test |
| Explicit auth; no custody/config replacement; old routes 401; signed Signal bytes including valence; exact retry, conflict and wrong network | `explicit_authentication_descriptor_signed_receipt_and_restart` |
| Receipt and full history survive process restart; repeated activation retains network/genesis | same process test |
| Separate and combined legacy import paths; exact source bytes, two authors, SELF_NETWORK and durable counts preserved; changed source refused | `explicit_legacy_import_and_combined_auth_upgrade_preserve_sources` |
| Truncated source and invalid operator commands leave source/generation intact | `malformed_legacy_and_invalid_commands_leave_the_source_and_generation_intact` |
| Foreign metadata, redirects and oversized observations fail without state creation | `live_descriptor_refuses_foreign_metadata_redirects_and_oversized_observations` |
| Config validation/exclusive creation and complete status document | configuration process test and status unit test |

Full logs: [compile](../../soft3/audit/neuron-cell/implementation-baseline/p12-cyber-launcher-check.log),
[tests](../../soft3/audit/neuron-cell/implementation-baseline/p12-cyber-launcher-tests.log),
[release build](../../soft3/audit/neuron-cell/implementation-baseline/p12-cyber-launcher-release.log),
[release process scenarios](../../soft3/audit/neuron-cell/implementation-baseline/p12-cyber-launcher-release-tests.log).

These checks exercise real local HTTP servers and executable processes on the
native profile. They establish durable local acceptance and migration behavior;
network consensus, deployment, remote adversarial transport and full Hermes
provider parity remain their separately declared profiles and evidence.
