---
title: cyber distribution
tags: cyber, roadmap, release, configuration
crystal-type: plan
crystal-domain: cyber
status: planned
---

# distribution across A, B and C

Owner: Cyber product, with versioned components supplied by their repositories.
Apply these gates to each delivered capability set.

## reproducible source and binary artifacts

- [ ] Pin the complete component source closure, lockfiles, toolchain, feature
  selection and generated resources. Record dirty inputs explicitly during
  development; release from reproducible immutable inputs.
- [ ] Build from a clean checkout without undeclared sibling resources.
- [ ] Produce CI binaries, checksums, source provenance and capability manifests
  for each supported platform. Check the installed artifact in a fresh process.
- [ ] Define upgrade/rollback compatibility for configuration, databases, graph
  codecs, proof artifacts and network profiles before overwriting a user home.

## configuration and explanatory graph

- [ ] Provide the simple configurator through the versioned configuration API:
  select local/network mode, storage placement and supported workers/resources.
- [ ] Validate the whole configuration before atomic publication; show actionable
  diagnostics and expose actual capabilities through doctor and the launcher.
- [ ] Keep explanatory graph pages linked to the corresponding contracts and
  audit evidence. Define how the binary/release discovers the shipped graph.
- [ ] Connect cyb installation, startup, shutdown, status and supported operations
  to the same product interface used by the terminal.

## acceptance evidence

- [ ] Run positive scenarios plus disk errors, interrupted writes, corrupt replay,
  invalid authorization/proofs, wrong networks and retry/cancellation failures.
- [ ] Keep logs, exact revisions and artifact hashes in the owning `audit/`.
- [ ] Describe the delivered A/B/C scope and each unsupported capability in release
  notes. Preserve failing acceptance requirements as visible work.

Exit: a fresh machine can reproduce the documented installation and supported
scenario using the released artifacts and configuration instructions.

discover all [[concepts]]
