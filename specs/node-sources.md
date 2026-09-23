---
title: Node source lock
tags: cyber, spec, build
crystal-type: entity
crystal-domain: cyber
---
# Node source lock

`Cargo.lock` pins registry dependencies. `sources.lock.json` pins the Git
revisions of sibling repositories used by the default Cyber node and its tests.
The product repository's own revision is the clean commit running the checkout
command; a file cannot contain its own commit hash.

From a clean Cyber checkout, with Git, Nushell and Rust available:

```nu
nu analizer/node-sources.nu . checkout --destination ../cyber-node-sources
cd ../cyber-node-sources/cyber
nu scripts/release.nu --locked-sources
```

The destination must not exist. Checkout fetches each exact commit from its
recorded GitHub origin, including Cyber itself, then checks the complete local
package set. An interrupted destination is retained for inspection; retry into
a new directory. No existing working tree or node home is reset or migrated.

The release gate checks before building and after integration tests that every
source repository is clean, its revision matches, the Cargo lock hash matches,
and Cargo resolves exactly the recorded local package names, versions and
manifest paths. It fails on missing dependencies, wrong revisions or dirty
sources. The result is `dist/cyber`, `dist/SHA256SUMS` and `dist/build.json` with
`sources_locked: true`, compiler identity and source provenance.

This profile includes node compilation and tests. Optional UI/stack features,
standalone component workspaces, benchmarks and cross-platform builds have
different dependency sets and are not certified by this lock. Component tests
needed by the node can run from this checkout with `cargo test --locked -p soft3`.
All-feature component reviews are recorded separately in audits.

Maintainers update reviewed component commits, run
`nu analizer/node-sources.nu . capture`, review and commit the resulting lock,
then repeat checkout and release validation. Capture records Git revisions but
does not establish that dirty local files exist at those revisions; the clean
checkout is the required check. `--mirror-root PATH` can fetch Git objects from
an existing sibling layout for local rehearsal. It never copies working files;
publication requires a checkout from the recorded remote origins.

Without `--locked-sources`, the release script remains a development build and
records dirty provenance. A source-locked host build establishes availability
and consistency of its source inputs. It does not establish bit-identical
output across compiler/platform versions, database upgrade compatibility or
distributed-network readiness.
