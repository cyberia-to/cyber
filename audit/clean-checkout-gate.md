---
title: cyber clean-checkout gate
tags: cyber, audit, launch, release
crystal-type: report
crystal-domain: cyber
date: 2026-09-24
---
# cyber clean-checkout gate

date: 2026-09-24
revision: 04a1b3d5 (origin/master)
property: [[cyber/launch|launch]] row 39 — every phase-1 component builds and tests from a clean checkout of its default branch against the default branches of its siblings.

this repo carries the `true-cyber` package and the `cyber` binary at its root (`Cargo.toml`), separate from the published [[true-cyber]] crate at `cyberia-to/true-cyber`; it was not on the 2026-09-22/23 sweep's list of repos failing the gate, and it has no prior row-39 launch PR, so this measures it directly, the method [[tade]]'s and hemera's row-39 audits used for the same row.

worktree: `git worktree add ... origin/master`, siblings resolved through path dependencies (`../soft3/crate`, `../cybergraph`, `../foculus`, `../tok/rs`, `../tru/rs`, `../bbg/rs`, `../neuron/model`) against each sibling's own current checkout, no owner working-tree edits used.

```
$ cargo check --tests --locked
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.45s

$ cargo test --locked
running 1 test
test tests::status_requires_a_complete_network_bound_document ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 6 tests
test config_is_explicit_validated_and_never_overwritten ... ok
test malformed_legacy_and_invalid_commands_leave_the_source_and_generation_intact ... ok
test live_descriptor_refuses_foreign_metadata_redirects_and_oversized_observations ... ok
test cyb_wire_write_restart_and_exclusive_home ... ok
test explicit_authentication_descriptor_signed_receipt_and_restart ... ok
test explicit_legacy_import_and_combined_auth_upgrade_preserve_sources ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

no warnings, no errors, 7 tests green. result: cyber builds and tests clean from `origin/master`. this closes this repo's own slice of row 39; the row stays open until every repo the sweep found broken has its fix merged, and this repo's sibling pins are added to `soft3/release/phase1.toml` so drift is caught automatically rather than by a manual re-run.
