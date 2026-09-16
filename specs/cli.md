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

The implemented baseline describes the current unpublished 0.8.0 source,
including the neuron-convergence launcher and migration revision. Sections
labelled target specify later interfaces. MUST, SHOULD and MAY express
implementation requirements; profile acceptance and release evidence remain separate.

## responsibility

`cyber` owns node configuration, storage lifecycle, network participation,
job scheduling and acceptance of results into network state. It composes
soft3 libraries. Joy provides warrior capabilities through a selected backend;
workers are running instances, following the
[soft3 execution model](../../soft3/specs/execution-model.md).
The named robot attaches neurons with explicit keys, networks and device access.
Vault owns private custody; neuron executes durable progs under those subjects.
GraphSession retains multi-neuron state without a process signer. These roles
follow [cyb architecture](../../cyb/specs/architecture.md).

## implemented baseline

```text
cyber [--home PATH] init [--bind IP:PORT] [--moniker NAME]
cyber [--home PATH] node
cyber [--home PATH] status [--json]
cyber [--home PATH] config
cyber [--home PATH] cyb [--live]
cyber [--home PATH] storage import-legacy
cyber [--home PATH] auth enable [--import-legacy]
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
| node | hold home/database writer locks, recover native BBG state, serve until termination | no command result |
| status | bounded GET configured /status; validate document | cybermark, or JSON with --json |
| config | load and validate config without contacting the node | effective TOML |
| cyb | describe configuration without opening graph or contacting endpoint | cyber/connection/v2 JSON |
| cyb --live | add bounded validated /capabilities observation; graph remains unopened | cyber/connection/v2 JSON |
| storage import-legacy | offline strict import of retained log into the shared BBG owner | cyber/legacy-import/1 JSON |
| auth enable | offline permanent activation of signed native publication; optional strict import first | cyber/authentication/1 JSON |

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

Only loopback IP addresses and nonzero ports are accepted. Monikers contain
1–256 bytes, non-whitespace content and no control characters. Config reads
accept at most 16 KiB of a regular UTF-8 file. `init` creates
parent directories and refuses to overwrite config. Other files already
in the home remain intact. A fresh home starts independent local state.

Current `status --json` emits the cybermark field map: `height` is a JSON
number; other values, including counters and `catching-up`, remain strings.
This shape stays unchanged. Descriptor `cyber/connection/v2` explicitly
supersedes v1: `network_label` is presentation, `network` is a native ID or
null, and `cyb_commands` is a list. Old v1's label-valued `network`,
unsigned `submit` and singular `cyb_command` must not be treated as v2.

Offline descriptors use `observation: configuration-only` and null
`network`, `profile`, `submit`, `native_capabilities` and
`capabilities.authenticated_writes`. The `endpoints` map is a supported
route catalogue, independently of live activation. `cyb --live` uses
`observation: endpoint-capabilities` and retains validated endpoint metadata.
Only a compatible signed profile supplies `submit: /v3/action` and a
`net pin` command. Both forms state `receipt_meaning: endpoint-acceptance`
and `consensus_finality: false`.

Status and live capabilities use a five-second HTTP timeout, a 64-KiB body
limit and zero redirects. Non-200, malformed, oversized or inconsistent
responses fail with nonzero exit. Neither command opens NativeNode, creates
keys or changes a profile.

Offline import/auth and the running product hold `node.lock` and
`auth-upgrade.lock`; BBG additionally fences its writer. Operators stop the
node and all older writers before migration. Import verifies the complete
bounded retained source and returns its hash, event/signal counts, height and
root. Authentication optionally imports first, then promotes the persistent
reader generation and retires incompatible genesis-reader paths. The upgrade
is monotonic, preserves native network identity and creates no subject key.
Its JSON reports profile/network/authentication and the optional import result.

These current reports are distinct from the target generic envelope below.

The sibling `true-cyber` CLI now provides a signed native client over the shared
GraphSession/Registry/Host: explicit neuron attachment, `sync`, `link`, `relay`,
`receipt`, and legacy inspection/export. Its `sync` mirrors bounded endpoint
observations and reports no consensus verification. It does not start this
product's node. See [the native client contract](../../true-cyber/specs/native-client.md).

`soft3 node` remains a developer entry point to the same native coordinator.
BBG fences the database writer; each home has one active state owner. Offline
activation and import preserve exact legacy source and native network genesis.
Product wrappers reuse the shared owner's migration boundary.

`cy neuron` manages robot attachments; the standalone `neuron` executable manages
durable runtime/prog operations. `cy task` composes soma tasks over that runtime.
Command names do not create another authority layer. A task, prog or invocation
ID names retained work under a neuron and has no key.

## target command surface

| command | effect | availability |
|---|---|---|
| init, node, status, config, cyb, storage import-legacy, auth enable | preserve documented baseline semantics | implemented |
| doctor | report configuration, storage, dependency and readiness checks | planned |
| worker status | query backend capabilities and active/queued work from running node | planned |
| worker start --backend joy | enable scheduling in the running node | planned |
| worker stop | disable new dispatch and request cancellation while retaining attempted/unknown outcomes | planned |

Worker commands control the existing node through an authenticated local
control channel. They MUST NOT open its state independently or silently
start a second node. The control-channel transport/authentication is a
separate implementation prerequisite. The signed-native HTTP profile authorizes
its declared graph actions; worker administration requires its own explicit
grant and command contract. Standalone nox program execution belongs to `joy run`;
neuron owns durable prog execution through supported workers.

In the `cyber` product, `sync` is reserved for verified peer replication. Its
availability requires checkpoint, history, coverage and finality policies.
An observation-mirror command in another client must report that narrower scope.
No command may report verified synchronization after a mere HTTP probe.

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

Baseline tests are in `cyber/tests/node.rs` and the current launcher/migration
process tests. Target worker/drain/network cases retain their own release gates.
Changes to command semantics are made here first, then in code/help/tests
and [[specs/cyb-node]]. Additive JSON fields are allowed within a schema;
consumers ignore unknown result fields. Changed meanings/types require a
new schema. Requests/configurations reject unknown fields to expose typos.

discover all [[concepts]]
