---
title: launch PR review and local node acceptance
tags: cyber, audit, launch, node, storage
crystal-type: report
crystal-domain: cyber
date: 2026-09-23
status: local-validation-passed-release-blocked
---

# launch PR review and local node acceptance

Nine reviewed PRs were merged. The local Cyber release artifact passes the
process acceptance suite. A reproducible release remains blocked by the
uncommitted dependency set and the pending coordinated storage/native adapter
integration. The initial inventory contained 290 open PRs; this review selected
the node ingress, startup and storage paths. It does not certify that entire queue.

## merged changes

| PR | reviewed head | result |
|---|---|---|
| [soft3 #5](https://github.com/cyberia-to/soft3/pull/5) | `8eb1c8c` | request identity validation coverage |
| [soft3 #6](https://github.com/cyberia-to/soft3/pull/6) | `d2d411c` | query parameter validation coverage |
| [soft3 #10](https://github.com/cyberia-to/soft3/pull/10) | `3409c3c` | malformed HTTP, duplicate headers and request size coverage |
| [soft3 #11](https://github.com/cyberia-to/soft3/pull/11) | `55760fc` | legacy-home startup guard coverage |
| [soft3 #14](https://github.com/cyberia-to/soft3/pull/14) | `5362a17` | missing CLI argument values return an error instead of panicking |
| [soft3 #15](https://github.com/cyberia-to/soft3/pull/15) | `9243eb2` | route dispatch coverage; fixed temporary database cleanup before merging |
| [soft3 #16](https://github.com/cyberia-to/soft3/pull/16) | `043cf2f` | submission and receipt coverage; fixed temporary database cleanup before merging |
| [bbg #15](https://github.com/cyberia-to/bbg/pull/15) | `b63ca50` | reject fetched content whose hash differs from the requested particle |
| [bbg #20](https://github.com/cyberia-to/bbg/pull/20) | `7695814` | reject impossible signal collection counts before allocating memory |

BBG #20 was reopened after concurrent sweep triage closed it. Its title now
describes the fix without an invented registry number. BBG #16 is the superseded,
closed allocation patch. No launch property is closed by these merges alone.

The changes also reached the active feature branches. Soft3 integration commits
are `461124c` and `26850f0`. BBG `58c9b8d` preserves the new hash check and moves
its tests into the durable storage branch's separate test module. That commit
was pushed independently of the owner's pre-existing unpublished `bd0875b`
dimension-commitment change. Existing unrelated dirty work was retained.

## changes held after review

| PR | concrete reason |
|---|---|
| [soft3 #7](https://github.com/cyberia-to/soft3/pull/7) | permits other genesis chains while `Node::open` and status still select spacepussy-test; runtime identity must follow genesis consistently |
| [soft3 #8](https://github.com/cyberia-to/soft3/pull/8) | genesis installation bypasses home locking and the retired-genesis lifecycle; needs concurrent-startup and retired-home checks |
| [bbg #12](https://github.com/cyberia-to/bbg/pull/12) | dependency requirements name versions available only in dirty sibling manifests; isolated committed sources fail to resolve Lens 0.2; also ignores a failing proof integration test |
| [bbg #21](https://github.com/cyberia-to/bbg/pull/21) | saturating balance credit silently discards value; a self-transfer at u64::MAX loses one unit; overflow must reject before mutation |
| [soft3 #3](https://github.com/cyberia-to/soft3/pull/3) | 401-file signed-adapter/convergence change was labelled as a two-word documentation edit; retitled, scope corrected and returned to draft for coordinated review |

Specific findings were posted on the respective PRs. BBG's proof test
`look_proof_verifies_against_state_root` also fails with
`UnsupportedRecursiveOpening` on committed Zheng 0.3.3. A version bump alone
does not fix that proof path.

## executed validation

| scope | command / evidence | result |
|---|---|---|
| BBG #15, committed sibling snapshots | library suite, with lock refresh to committed Zheng 0.3.3 | 54 passed, including 12 tiered-store tests |
| BBG #20 + #15, committed sibling snapshots | CLI suite with the independent tape-to-tade manifest/path correction and lock refresh | 7 passed |
| BBG active durable-storage integration | `cargo test --manifest-path rs/Cargo.toml --locked --lib storage::tiered`; CLI suite | 12 + 7 passed |
| Soft3 #5/#6/#10/#11/#14 combined | `cargo test`; `cargo check --tests --locked` | 47 library + 11 CLI passed |
| Soft3 #15/#16 combined on main | same checks | 76 library + 11 CLI passed |
| Soft3 active signed-adapter integration | `cargo test --manifest-path crate/Cargo.toml --locked` | 82 library + 11 CLI passed |
| Cyber artifact | `nu scripts/release.nu` | formatting, debug suite, release build and six process tests against the release binary passed |

The process tests use disposable homes and loopback endpoints. They cover
exclusive ownership, accepted state across process termination/restart,
signed submission and retry receipts, unsigned-write rejection after explicit
authentication activation, explicit legacy import, invalid/truncated source
preservation, and descriptor rejection of redirects/foreign metadata/oversized
responses. No existing user's home or running network was used.

Local artifact: `dist/cyber`, version `cyber 0.8.0`, macOS arm64. Its checksum
and source records are generated in `dist/SHA256SUMS` and `dist/build.json`.
Tested artifact SHA-256:
`bd07e60724099ef96cd94e9cca82b6f73538752803cc222768e922af2fa78e86`.
The build records dirty sibling sources. Existing vendored Fjall warnings remain.
These checks do not establish power-loss survival, a soak-test result, verified
peer consensus or mainnet readiness.

## next node gates, in order

1. Assemble a committed, reproducible dependency set. Current Lens/Nox/Zheng
   manifests declare 0.2/0.3/0.4 while their committed versions are
   0.1.3/0.2.0/0.3.3. Neuron's native model and BBG coordination also contain
   uncommitted source used by the artifact. Review and integrate BBG #9,
   Cybergraph #3, Foculus #5 and the correctly scoped Soft3 #3 against that set.
2. Close the lifecycle gaps: `soft3::node::run` still spawns one unbounded thread
   per connection, logs accept failures in a loop, and has no bounded shutdown
   drain. Add overload and SIGTERM acceptance scenarios for the actual binary.
3. Complete the remaining storage fault matrix: disk-full/flush failures,
   ambiguous commit recovery and the filesystem/power-loss boundary. Keep
   original receipts and independently recomputed state roots in each scenario.
4. Wire and test multiple nodes through Radio/Foculus: catch-up, partition,
   reconnection and state-root convergence. HTTP endpoint acceptance currently
   supplies only the local half of a stable network node.

The maintained delivery sequence remains [A — local node](../roadmap/a-local-node.md)
and the canonical [launch registry](../launch.md).
