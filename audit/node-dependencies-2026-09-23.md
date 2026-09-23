---
title: Committed node dependency closure
tags: cyber, audit, dependencies, release
crystal-type: entity
crystal-domain: cyber
date: 2026-09-23
---
# Committed node dependency closure

The default Cyber node builds and passes process acceptance from an independent
checkout fetched entirely from GitHub. No source files are copied from dirty
sibling working trees. The source lock covers 15 dependency repositories and
38 local packages, including Cyber itself in the package set. This closes the
node source-availability blocker from the earlier
[launch review](launch-review-2026-09-23.md); it does not close the complete
phase-1 fleet release, database-upgrade or distributed-finality gates.

## Published components

| component | pinned commit | reviewed scope |
|---|---|---|
| Lens | `a5f57486` | versioned commitments and canonical bounded openings; [PR 12](https://github.com/cyberia-to/lens/pull/12) |
| Nox | `1f2d9ef0` | dependencies and guarded jets; [PR 15](https://github.com/cyberia-to/nox/pull/15) |
| Zheng | `3760de03` | manifests/tests/example, no production proof algorithm changes; [PR 32](https://github.com/cyberia-to/zheng/pull/32) |
| Neuron | `7d214e02` | runtime-free identity/model/actions; [PR 7](https://github.com/cyberia-to/neuron/pull/7) |
| BBG | `1467b68c` | storage coordination, transfers, generations and root-bound public queries; [PR 22](https://github.com/cyberia-to/bbg/pull/22) |

Full hashes and the other component revisions are in
[`sources.lock.json`](../sources.lock.json). These five candidates are published
as branches with draft integration PRs; they are not all merged into default
branches. Exact-commit checkout does not depend on their merge order. Existing
owner working trees, including unfinished Neuron runtime convergence and private
Zheng execution research, were preserved.

Soft3 is pinned at `26850f00`. Its standalone workspace still has an optional
cyb facade manifest dependency; the default Cyber node closure and Soft3 node
tests require neither cyb nor Honeycrisp. This lock covers the node profile,
not every component's independent all-feature workspace.

## Executed acceptance

| check | result |
|---|---|
| checkout from recorded GitHub origins into an empty directory | all 16 repositories fetched; 15 dependency revisions and 38 packages verified |
| `nu scripts/release.nu --locked-sources` | format, debug tests, release build, release process tests and final clean-source gate passed |
| Cyber debug tests | 1 unit + 6 process tests passed |
| release-binary process tests | 6 passed: write/restart, exclusive home, signed receipts/auth activation, live descriptor rejection and explicit legacy import |
| source guard in disposable Git checkouts | clean acceptance plus five negative checks passed: dirty files, wrong commit, changed Cargo lock, omitted repository and changed package set |
| Soft3 node-profile tests | 82 library + 11 CLI tests passed |
| BBG all features, reviewed candidate | 165 tests passed, zero ignored |
| Neuron existing workspace / model all features | 22 / 10 tests passed |
| Lens all features | 128 tests + 1 doctest passed |
| Nox default / all features | 169 / 175 tests passed |
| Zheng default / release with serde | 170 / 176 tests passed; 91,530 wire mutation attempts, zero ignored or filtered |

Component review evidence lives in each repository's `audit/`. Supplemental
`cargo test --locked -p cybergraph` from Cyber was refused by Cargo because its
dev dependencies are outside this workspace; it is not counted as coverage.
Cybergraph was compiled and exercised by node process tests and Neuron's
existing workspace tests.

## Host artifact and remaining boundaries

- Product source: `87f8359e1ab64e14e23d5ba434089c2313e93ad1`.
- Version: `cyber 0.8.0`, macOS arm64, Homebrew Rust 1.95.0.
- Artifact SHA-256: `5cb3b61388d2232738a56187f01e7846af59a582a1939971cd2958af936e3181`.
- Cargo lock SHA-256: `0858f99daca13c4d224c180a5972f1e9e14501fddacf179a74c49f4688896e9f`.
- Retained checkout: `/Users/master/cyber/.node-builds/20260923`.
- Binary, checksum and provenance: `cyber/dist/` inside that checkout.
- `build.json` records `sources_locked: true`; every source record is clean.

Three existing vendored Fjall warnings remain. This is source-pinned host
acceptance, not bit-identical output across toolchains or a published production
release. Linux runtime/artifact qualification, CI distribution, overload/drain
and power-loss qualification remain separate.

BBG dimension encoding version 2 changes roots/offsets while native persisted
metadata still uses version 1. Migration from older committed native stores is
unverified; fresh-store/restart tests do not establish transparent upgrades.
Preserve old stores and qualify this boundary before deployment.

The [source-lock contract](../specs/node-sources.md) supplies repeatable checkout,
release and negative-test commands. Development builds can still record dirty
provenance; they do not pass the source-locked release gate.
