---
title: Cyber releases
tags: cyber, spec, release, soft3
crystal-type: spec
crystal-domain: cyber
---

# Cyber releases

Cyber consumes the [soft3 release contract](https://github.com/cyberia-to/soft3/blob/main/specs/releases.md).
`release/soft3.toml` pins its version and full source revision; the Release train
workflow uses the same revision. Soft3 owns stack qualification and dependency
pins. Cyber adds executable acceptance and protocol graph compilation.

Friday 12:00 UTC, or manual dispatch, captures default-branch origin inputs and
creates `candidate-YYYYMMDD.N` as a GitHub draft prerelease. A rehearsal uses
`cut=false`. Native runners target macOS and Linux, ARM64 and x64. Failed gates
retain available artifacts and red receipts. The owner decides promotion and
pushes version tags. Automation creates drafts only.

## evidence

Every candidate includes `SHA256SUMS`, `sources.json`, `candidate.json`,
`release-validation.json`, `soft3-dependencies.json` and `soft3-dependencies.md`.
The release description includes the dependency inventory. Platform archives
include the executable when built, exact source/package inventories and command
logs. Missing binaries, failed package resolution, drifted pins and blocked gates
remain explicit. Collection checks all platform receipts and their checksums.

The inventory covers the common source set plus resolved package closure.
Manifest declarations are labelled separately when resolution fails. The product
contract in [[specs/node-product]] defines runtime capabilities.

## gates

Soft3 component and conformance gates must pass. Cyber additionally runs:

- `cargo test --locked`;
- `nu scripts/release.nu --locked-sources`, including process acceptance;
- locked optica builder compilation and `optica build` on the protocol graph;
- matching `release/soft3.toml`, unchanged source inputs and complete inventories.

The Node workflow remains development CI for the [node source lock](node-sources.md).
Its feature-branch inputs must converge with default branches before the common
release train can turn green. Its artifacts alone do not satisfy the common
candidate contract.

From candidate cut until owner verdict, default branches of soft3, cyber and cyb
are frozen. Candidate fixes and receipts go on `release/<date>`, with receipts in
`audit/release-<date>/` and a work-log entry in [[launch]]. Version and pin changes
follow the coordinated bump PR rule in [[cyberia/dev]].
