---
title: Cyber releases
tags: cyber, spec, release, soft3
crystal-type: spec
crystal-domain: cyber
---
# Cyber releases

GitHub Releases in `cyberia-to/cyber` distributes the product executable.
The Node workflow tests relevant pull requests and changes to `master`.
A `vVERSION` tag selects a release; VERSION must equal `Cargo.toml`'s package
version. The current channel publishes development prereleases. A manual
workflow run builds downloadable CI artifacts.

## mandatory soft3 inventory

Every published release must include `soft3-dependencies.md` and
`soft3-dependencies.json`. The Markdown table is also included in the GitHub
release description. Each component entry identifies its repository, full
commit SHA, package names, versions and manifest paths. Vendored third-party
packages have a separate classification.

The inventory covers the default node's build and test source closure. It is
generated from the [verified source lock](node-sources.md). Presence in this
inventory describes build inputs; the [product contract](node-product.md)
defines available runtime capabilities.

Each platform's `build.json` binds its binary checksum to the inventory checksum,
Cargo lock, compiler identity and actual compilation target. All platform
inventories and source locks must agree with the tagged product commit.
Missing, altered or mismatched inventories prevent publication.

## build and publication

The initial platform set is Linux x86_64, Linux ARM64 and macOS ARM64. Native
GitHub runners independently fetch the locked component commits, use the Rust
version in `rust-toolchain.toml`, run formatting/unit/process checks, build the
release executable and run process acceptance against that executable.

Each `cyber-vVERSION-TARGET.tar.gz` contains `cyber`, `build.json`, `SHA256SUMS`,
both dependency inventories, `sources.lock.json`, `Cargo.lock`, the toolchain
file and the license. The release also exposes inventories, locks, per-target
build provenance and archive checksums as separate assets.

Publication waits for every platform. The collector checks the complete
platform set, source identities, inventory agreement and archive contents.
Assets are uploaded to a draft, which becomes public after successful upload.
Failed creation/upload retains the draft for inspection; retry by removing only
that incomplete draft and rerunning the failed job. Existing published releases
retain their original assets. Tags and component pins select immutable sources.

## maintainer cycle

1. Update reviewed component pins through `node-sources.nu capture` and commit
   the source lock together with any Cargo lock/version changes.
2. Merge after the Node workflow passes on all three platforms.
3. The owner tags the chosen commit `vVERSION` and pushes the tag. The Node workflow builds
   and publishes the prerelease with its mandatory soft3 inventory.
4. Download the target archive, verify the attached checksums, unpack it and run
   `./cyber --help`. `./cyber init` creates a new local home.

Database compatibility with a concrete predecessor remains available through
the local `release.nu --previous-binary` qualification. The initial GitHub cycle
records no cross-release database compatibility result. Signed OS distribution
and further platforms have their own delivery gates.
