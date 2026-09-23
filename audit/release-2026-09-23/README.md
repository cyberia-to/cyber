---
title: Node release automation qualification
tags: cyber, audit, release, soft3
crystal-type: entity
crystal-domain: cyber
date: 2026-09-23
---
# Node release automation qualification

[PR 108](https://github.com/cyberia-to/cyber/pull/108) adds tagged binary
publication with a mandatory inventory of the 15 soft3 component repositories.
This receipt covers the initial three-platform binary pipeline and release
assembly rehearsal. Candidate scheduling and owner promotion follow the
[release train](../../../cyberia/dev.md#release-train).

[GitHub Actions run 35854257803](https://github.com/cyberia-to/cyber/actions/runs/35854257803)
passed native Linux x86_64, Linux ARM64 and macOS ARM64 builds. Every platform
passed 1 unit test, 6 debug process tests and 6 process tests against its release
executable. Linux x86_64 additionally passed five source-guard negative checks
and nine release-guard negative checks. The checked source was PR merge commit
`28e2fc4e3a4950d05067e87e7befda644aa06341`; reviewed branch head was
`dc94964fe9199ebb058dc94c27bf6f014c874207`.

All three CI artifacts were downloaded and passed `node-release.nu collect`:
product identity, matching inventories/source locks, all expected targets and
archive contents. The downloaded macOS archive passed `shasum -a 256 -c
SHA256SUMS`, and its executable ran `--version` and `--help`. The collector
prepared release assets and Markdown notes containing the component table.
GitHub publication runs only on a version-tag push; this rehearsal created no
tag or public release.

The initial CI attempt failed because assembling dependencies below the product
workspace changed Cargo's workspace discovery for Neuron. Moving source
assembly to the runner's independent temporary directory resolved it. Component
pins and source code remained unchanged. Existing vendored Fjall warnings remain.

Evidence: [gate output excerpt](checks.txt), [structured results](release-validation.json),
[exact inventory](soft3-dependencies.json), [assembled asset hashes](SHA256SUMS).
The workflow retains downloadable archives for 14 days; a tagged GitHub Release
provides persistent assets. The complete train's macOS Intel, graph/candidate
gates and cross-product promotion remain outside this initial binary workflow.
