---
title: Cyber releases
tags: cyber, spec, release, soft3
crystal-type: spec
crystal-domain: cyber
---

# Cyber releases

Cyber selects one concrete [soft3 build](https://github.com/cyberia-to/soft3/blob/main/specs/releases.md)
through `release/soft3.toml`: build name, GitHub release ID and SHA256SUMS digest,
plus the expected version and source revision. The authenticated build fixes all
stack component revisions and supplies their original qualification verdict.

A product cut captures Cyber's own default-branch source, downloads and verifies
the selected soft3 evidence, and checks out that exact source assembly. Cyber's
Cargo path dependencies resolve inside it. The shared engine is pinned separately
in `.github/workflows/release-train.yml`; changing that tool leaves the selected
stack build intact. External packages remain governed by Cargo.lock; stack-owned
crates must resolve from the selected assembly.

Cyber runs locked Cargo tests, formatting, the release build, process acceptance
against that binary, and optica/protocol graph compilation. Soft3's recorded stack
qualification is inherited. A RED stack keeps Cyber RED even when Cyber's own
gates pass. The standalone development `sources.lock.json` remains development
provenance; the release train takes its authority from the selected soft3 build.

Every candidate carries `SHA256SUMS`, `sources.json`, `candidate.json`,
`release-validation.json`, `soft3-dependencies.json`, `soft3-dependencies.md`,
`soft3-build.json` and `soft3-build.tar.gz`. The last two retain the build pin and
original upstream evidence. Source or checksum substitution fails closed. All
native platforms must supply matching receipts and a binary for a green verdict.

Friday 12:00 UTC or manual dispatch creates `candidate-YYYYMMDD.N` as a draft
prerelease; `cut=false` records a rehearsal. Public soft3 releases require no
cross-repository secret. Reading a soft3 draft requires `SOFT3_READ_TOKEN` with
read access to that repository. Existing candidate assets remain unchanged.

From cut until owner verdict, default branches remain frozen. Candidate fixes
and receipts go to `release/<date>` and `audit/release-<date>/`. The owner merges
build/version bumps, promotes candidates and pushes version tags. See
[[specs/node-product]], [[specs/cyb-node]] and [[cyberia/dev]].
