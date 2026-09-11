---
tags: cyber, specs, soft3, cyb, joy, architecture
crystal-type: spec
crystal-domain: cyber
status: implementation
alias: node product, cyber binary
---

# node product

CLI contract: [[specs/cli]]. Computation job contract: [[specs/worker]].
Joy's developer CLI is specified in `joy/specs/cli.md` in the sibling repo.

The shared architecture is defined by [soft3's warrior/worker/network
foundation](../../soft3/specs/warriors.md). A warrior implements a VM/OS
family; workers instantiate it for an open-ended set of compatible networks.

Cyber is the network product assembled from [[soft3]]. This repository owns
the executable entry point, configuration, product contracts, release
artifacts, and the explanatory graph. Component repositories own algorithms.

## composition

```text
cyb — personal cell, keys, local graph, UI
  │ HTTP: status, links, balances, explorer
  ▼
cyber — node process, configuration, lifecycle, capabilities
  │ Rust library calls
  ▼
soft3 — stack assembly and developer interfaces
  ├─ cybergraph — signal application
  ├─ bbg / lens — authenticated state and openings
  ├─ foculus / tru — finality and graph computation libraries
  └─ nox / zheng / hemera / strata — execution, proofs, primitives

joy — warrior implementation for the nox target family
  └─ workers — embedded, isolated or remote instances
       └─ selected backend — nox execution + zheng proving/verification
```

The bottom branch describes a component boundary to integrate. The current
`cyber node` process calls `soft3::node::run`; it has no joy worker loop.
The presence of a proof library in the dependency graph does not establish
proof verification on every accepted transaction.

## ownership and overlap

| part | owns | relationship to the node |
|---|---|---|
| cyber | product binary, config, graph, contracts, distribution | main entry point |
| soft3 | reusable stack assembly, SDKs, developer tooling | engine dependency |
| cybergraph | graph transitions and signal chains | one implementation reused by node and cell |
| cyb | personal state, secrets, interface, client lifecycle | consumes node HTTP; keeps its local cell |
| warrior | reusable VM/OS target implementation | supports many compatible network instances |
| joy | nox warrior integrating Trident, nox and Zheng | planned embedded node library; standalone developer CLI |
| worker | running instance of warrior capabilities | selected backend, placement and resource budget |
| warriors | catalogue of proving/mining implementations | discovery and documentation |
| cybernode | server deployment and operations | hosts selected products and bootloader chains |
| true-cyber sibling | earlier standalone cell CLI | migration source for sync/link commands |

There are two kinds of overlap. Sharing cybergraph between a node and a
personal cell is intentional: they apply the same transitions to different
state scopes. Independent CLI configuration, log replay code, network
defaults, and release ownership need consolidation around this product.

The soft3 `stack` feature currently re-exports a published cyb crate. It is
disabled in the product binary. Runtime assembly should point from cyber to
components; cyb consumes the node contract. The GUI belongs in its own
process and release, with keys remaining in its personal cell.

Soft3 owns foundational composition contracts and developer interfaces.
This product uses its existing assembly library and leaves component
algorithms in their owners. A later extraction of its HTTP host must preserve
[[specs/cyb-node]]. The product design embeds Joy in the cyber binary;
separate worker processes remain an optional placement choice.

## current executable

`cyber init` creates `config.toml` once; `cyber config` displays it.
`cyber node` opens the graph, replays its log, and serves the existing
soft3 HTTP surface. An OS file lock excludes a second cyber process using
the same home and releases when the process exits. The older soft3 server
does not participate in this lock; give each independently run server a
separate home.

`cyber status` validates the returned cybermark document and reads the
current state. `cyber cyb` emits a versioned JSON connection descriptor.
`--home` overrides `CYBER_HOME`, which overrides
`~/.cyber/spacepussy-test`. An explicit config is required to start.

Only loopback bindings are accepted by this entry point. The current bridge
creates unsigned signals on behalf of a supplied neuron. Its local height
advances once per accepted signal; distributed finality requires additional
integration. A new home begins independent local state, even though the
engine's chain label is `spacepussy-test`.

## delivery boundary

The first deliverable is a working local node binary for cyb development:
configuration → process → HTTP writes → state → restart → same state root.
Tests exercise this path against the real soft3 engine, including the
cybermark status and JSON receipt shapes used by cyb. GUI execution is a
separate validation step.

Before a public node release, complete these work packages in order:

1. Durable acceptance: propagate journal errors, sync before success,
   reject corrupt/truncated replay, and test crash recovery.
2. Authenticated admission: preserve signed native signals end to end,
   verify proof-bound rewards, enforce resource limits and idempotent retry.
3. Network lifecycle: join/checkpoint verification, peer replication,
   competing histories, finality, upgrades, and readiness tied to these states.
4. Worker integration: root-bound job inputs, joy execution/proof results,
   verifier-owned acceptance, cancellation and bounded resource budgets.
5. Distribution: immutable component revisions, compatible primitive/wire
   versions, clean-checkout builds, CI artifacts, and platform release tests.

Existing engine risks are concrete: `append_frame` ignores I/O errors;
`open_store` skips rejected signals; the JSON bridge creates `proof: None`;
checkpoint rewards derive from labelled edges and amount. Loopback enables
local development while these paths are hardened.

Current source builds resolve sibling path dependencies, including local
component changes. Cargo.lock pins registry packages, while `dist/build.json`
records local repository revisions and dirty status. Rebuilding the same
published version alone cannot reproduce this development artifact.

The declared [[specs/node-modes|full, cell and light modes]] remain protocol
targets. The delivered runtime reports `local-chaosnet` and its actual
capabilities until those mode requirements are wired end to end.

discover all [[concepts]]
