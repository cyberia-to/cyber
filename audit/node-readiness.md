---
title: node readiness audit
tags: cyber, node, audit, soft3, joy
crystal-type: report
crystal-domain: cyber
status: blocked
alias: cyber binary readiness, node acceptance audit
---

# node readiness audit

The 2026-09-11 audit confirms a functioning local HTTP node and finds
blocking gaps in reliability, admission and the agreed execution model.
The executable remains a local development artifact. Product readiness
requires the failures below to be resolved.

## artifact and evidence

Audited artifact: `dist/cyber`, version `cyber 0.8.0`, macOS arm64.
SHA-256: `2fdfba0fb25ddbfc4cce5c80d6331850b98992146a4fa22764503a7b0bb9b217`.
The JSON report records that checksum and rejects executable changes during
the audit. Sibling path dependencies remain mutable; a package version alone
does not identify this build. Source provenance is recorded in
[the captured build manifest](node-readiness-build.json).

The [audit runner](../examples/readiness.rs) invokes the real binary and HTTP
API in disposable homes with temporary loopback ports. It kills and waits for
its child processes; no existing node, personal cyb store or public endpoint
is used. [Raw observations](node-readiness.json) include responses,
state roots, heights, exit codes and capability declarations.

```sh
cargo run --locked --example readiness -- dist/cyber audit/node-readiness.json
```

The runner requires `shasum`. Exit 0 means the selected gates passed; exit 1
means an unmet requirement; exit 2 means the audit could not complete.
Passing this selected set alone would not establish full production readiness.

## observed results

| requirement | observation | verdict |
|---|---|---|
| embed Joy in cyber | connection descriptor reports joy_worker=false; Cargo/source contain no Joy integration | missing |
| manage workers | cyber worker status exits 2, unknown subcommand | missing |
| configure another compatible network | changing the network field to audit-network-alpha exits 1; full instance descriptors are absent | missing |
| ordinary HTTP write and replay | accepted link reaches height 1; restart preserves height and root | passed |
| refuse writes when journal append fails | directory at log path causes append failure; node still returns HTTP 200, ok=true, height 1; restart returns height 0 | failed |
| reject truncated replay | remove the final byte from a one-signal journal; node reports healthy at height 0 after restart | failed |
| retry a relayed operation without duplication | two identical JSON submissions yield heights 1 and 2 | failed |
| require evidence before reward issuance | unsigned, unproved zheng → pussy link with amount 1000000 credits that amount | failed |

Seven selected requirements remain unmet. The first three are missing
architecture capabilities; the last four are reproduced behavior failures.
The retry probe models a client retry after a lost response. The current
bridge provides no stable identity to distinguish that retry from a second
intentional link; this must be solved in the signed submission contract.

The journal fault uses a directory at the append path after healthy startup,
so it works independently of Unix permission overrides. This demonstrates a
real I/O error, false acknowledgement and lost state; it is not a simulated
power-loss test. The truncation probe removes one byte from a valid journal.

## cause and ownership

`soft3/crate/src/node.rs::append_frame` ignores open/write errors.
`Node::link` mutates memory and the handler finalizes regardless of durable
append success. `open_store` treats unreadable storage as empty and skips
undecodable/rejected data instead of refusing unsafe recovery.

The JSON bridge creates unsigned signals with `proof: None`. Reward economics
derive from the labelled edge and amount; the reproduced path requires no
verified proof. Every JSON submission is assigned a fresh signal position,
so repeated request bodies are independently applied.

`cyber/src/main.rs` currently exposes init/node/status/config/cyb and accepts
only the literal network spacepussy-test. It calls soft3's server directly.
Embedding a shared graph engine is implemented; embedding Joy, worker
scheduling and general network-instance binding are future work.

## cyb and Joy validation boundaries

The existing node integration tests exercise the same cybermark height/root
fields and JSON link receipt shape used by cyb. Its network and relay code
were read during this review. GUI startup and an actual cyb session were not
run; HTTP shape compatibility is the extent of the runtime integration check.

Cyb currently relays individual link bodies and stores a cursor by neuron
and signal step, rather than network plus stable submission identity. Endpoint
changes, multi-link signals and interrupted retries require further testing
and a shared admission protocol. HTTP compatibility alone cannot guarantee
lossless synchronization of the personal cell with the network.

Eight selected Joy public execution tests passed:

```sh
cargo test --locked -p cyber-joy --test execution_claim -p joy-rs --test public_execution
```

They cover native/proof interfaces, requested statement matching, tampering
and unsupported state/secret requests. These are component tests. They do
not establish that cyber calls Joy, that rewards verify proofs, or that the
full legacy state-proof acceptance suite passes. Joy build/deploy commands,
stateful proofs and remote/GPU worker integration were not implemented here.

## checks completed and next gates

`nu scripts/release.nu` rebuilt the executable, passed three existing tests
and repeated both integration tests against the release binary. Clippy for
all cyber targets passed; cybergraph emitted an existing dead-code warning.
During the check, a sibling cybergraph change added a cyber-nox dependency;
Cargo.lock was refreshed to restore locked builds. This illustrates why
immutable dependency snapshots remain a delivery requirement.

Close the acceptance gaps in this order:

1. Journal error propagation, durable acknowledgement and fail-closed replay,
   with explicit recovery and crash-consistency tests.
2. Authenticated native submission, verified rewards and stable retry identity;
   wire cyb's relay to preserve signal atomicity and network-scoped cursors.
3. Embed Joy behind the [[specs/worker|worker contract]] and add actual
   management, cancellation and resource-limit enforcement.
4. Replace the single-name configuration with compatible VM/OS/proof/network
   descriptors, preserving identity across endpoint changes.
5. Complete peer sync/finality, reproducible source releases and full cyb
   integration checks, then run the remaining execution-model gates.

The [soft3 execution model](../../soft3/specs/execution-model.md) defines the
target architecture. [[specs/cli]], [[specs/cyb-node]] and [[specs/node-product]]
describe the product interface and its implementation boundary.

discover all [[concepts]]
