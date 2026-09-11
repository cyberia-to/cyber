---
tags: cyber, specs, cli, node, cyb
crystal-type: spec
crystal-domain: cyber
status: draft
alias: cyber cli contract
---

# cyber CLI

Product owner: cyber. Implementation: `cyber/src/main.rs`. This document
specifies the public executable boundary; [[specs/worker]] owns computation
jobs and [[specs/cyb-node]] owns the existing cyb HTTP connection.

The baseline below describes source version 0.8.0. Sections labelled target
are requirements for the next interface revision, rather than available
commands. MUST, SHOULD and MAY express implementation requirements.

## responsibility

`cyber` owns node configuration, storage lifecycle, network participation,
job scheduling and acceptance of results into network state. It composes
soft3 libraries. Joy provides warrior capabilities through a selected backend;
workers are running instances, following the
[soft3 execution model](../../soft3/specs/execution-model.md).
User keys and private personal state remain with the cyb cell.

## implemented baseline

```text
cyber [--home PATH] init [--bind IP:PORT] [--moniker NAME]
cyber [--home PATH] node
cyber [--home PATH] status [--json]
cyber [--home PATH] config
cyber [--home PATH] cyb
cyber --help
cyber --version
```

`start` is an alias for `node`; `--home` is global and may follow a command.
The home is resolved as flag → `CYBER_HOME` →
`$HOME/.cyber/spacepussy-test`. Relative paths resolve from the caller's
working directory. Missing commands and invalid argument syntax use Clap
exit code 2; help/version succeed with 0; application errors use 1.

| command | behavior | stdout |
|---|---|---|
| init | validate and exclusively create config; preserve existing files | config path |
| node | hold home lock, replay log, serve until process termination | no command result |
| status | GET configured /status with five-second timeout; validate document | cybermark, or JSON with --json |
| config | load and validate config without contacting the node | effective TOML |
| cyb | describe configuration without contacting the node | cyber/connection/v1 JSON |

All commands except init/help/version require valid `config.toml`.
Errors and server diagnostics go to stderr. Status success establishes a
valid local status response, while the descriptor reports configuration.
Neither command establishes distributed finality.

Configuration v1 contains exactly these fields; unknown fields fail:

```toml
version = 1
network = "spacepussy-test"
bind = "127.0.0.1:7780"
moniker = "cyber-local"
```

Only loopback IP addresses and nonzero ports are accepted. Monikers must
contain non-whitespace content and no control characters. `init` creates
parent directories and refuses to overwrite config. Other files already
in the home remain intact. A fresh home starts independent local state.

Current `status --json` emits the cybermark field map: `height` is a JSON
number; other values, including counters and `catching-up`, remain strings.
This existing shape and `cyber/connection/v1` MUST remain compatible until
explicitly superseded. They are distinct from the target envelope below.

The earlier sibling `true-cyber` CLI's `sync` and `link` commands have not
been migrated. `soft3 node` remains a developer entry point to the shared
engine. Its home lock is independent; a home must have one engine owner.

## target command surface

| command | effect | availability |
|---|---|---|
| init, node, status, config, cyb | preserve baseline semantics | implemented |
| doctor | report configuration, storage, dependency and readiness checks | planned |
| worker status | query backend capabilities and active/queued work from running node | planned |
| worker start --backend joy | enable scheduling in the running node | planned |
| worker stop | disable new dispatch and cancel active jobs | planned |

Worker commands control the existing node through an authenticated local
control channel. They MUST NOT open its state independently or silently
start a second node. The control-channel transport/authentication is a
separate implementation prerequisite; the unsigned chaosnet HTTP bridge
does not satisfy it. Standalone program execution belongs to `joy run`.

`sync` is reserved for verified peer replication. Until checkpoint,
history and finality policies are implemented it MUST remain unavailable.
No command may report sync completion after a mere HTTP probe.

## target machine interface

Add opt-in `--format json-v1` for finite commands. Human-readable output
remains the default. Existing `status --json` and `cyb` output retain their
baseline shapes; incompatible combinations of output flags fail with 2.
Long-running `node` uses stderr logs and its API for readiness; it MUST
reject `--format json-v1` until an event-stream contract exists.

Exactly one UTF-8 JSON object plus newline is written to stdout:

```json
{"schema":"cyber/cli/v1","command":"worker status","ok":true,"result":{"enabled":false,"jobs":[]},"error":null}
```

On application failure, `ok` is false, `result` is null and `error` contains
`code` (stable snake_case), `message` and `retryable` (boolean). Diagnostics
remain on stderr. Syntax errors detected before machine-mode selection may
use stderr only. Successful commands MUST have a non-null result object.

In the new envelope, u64 values use canonical decimal strings to preserve
precision for cyb/JavaScript. Booleans remain booleans; missing optional
values are null, never fabricated zeros. Heights, roots and capabilities
must describe the same observation. A root carries its encoding/profile.
Secrets and witness contents MUST NOT appear in output or diagnostics.

Target application exits: 0 success; 1 operation failed; 2 usage/config
validation failed; 3 requested capability unsupported; 4 service unavailable
or resource busy; 5 verification or admission rejected. Cancellation uses
130 when handled; an OS-killed child retains its platform termination
status. Consumers MUST accept unknown nonzero exits as failure. Baseline
codes remain documented above until this target ships.

## target lifecycle and configuration

Startup validates configuration, acquires the home lock, validates/replays
durable state, binds listeners, then announces readiness. Startup failure
MUST release resources and MUST NOT publish readiness. Corrupt journals
fail closed with a recovery diagnostic; repair requires an explicit action.

Readiness MUST distinguish `starting`, `local_ready`, `syncing`,
`network_ready`, `draining` and `failed`. `network_ready` requires the
configured network's verification policy. Worker readiness is an additional
capability, independent of HTTP liveness. Current `/health` is liveness only.

SIGINT/SIGTERM initiate a bounded drain: reject new writes/jobs, cancel
workers, flush acknowledged state, release listeners and lock. Forced exit
must preserve the recovery contract for acknowledged writes. The current
server does not yet implement this drain protocol.

Future config mutation MUST validate the whole proposed config and replace
it atomically. Changes to home, chain identity or commitment profile require
explicit migration; a network label alone cannot establish compatibility.
Any worker configuration extension requires config version 2, with an
explicit v1 migration. Worker scheduling defaults to disabled.

## conformance and ownership

Before claiming target conformance:

- exercise init preservation, malformed configuration and competing homes;
- run the same write/restart/root test against the released executable;
- check exact JSON types, error envelopes and exits for every finite command;
- distinguish offline node, live local node and verified network readiness;
- prove worker start/stop never creates a second state owner;
- reject unsupported proof/state modes without downgrading;
- exercise cancellation, disk failure and corrupt replay without false success.

Baseline tests are in `cyber/tests/node.rs`; the target cases remain work.
Changes to command semantics are made here first, then in code/help/tests
and [[specs/cyb-node]]. Additive JSON fields are allowed within a schema;
consumers ignore unknown result fields. Changed meanings/types require a
new schema. Requests/configurations reject unknown fields to expose typos.

discover all [[concepts]]
