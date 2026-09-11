---
tags: cyber, specs, joy, warrior, execution
crystal-type: spec
crystal-domain: cyber
status: draft
alias: cyber worker contract
---

# worker contract

Owner: cyber. Backend CLI specification: `joy/specs/cli.md` in the sibling
joy repository. This is a proposed logical contract v1; neither a deployed
worker service nor a frozen network wire encoding is claimed.

## authority boundary

Cyber owns scheduling, trusted state, admission policy and reward decisions.
Joy executes a supplied program and produces or verifies a computation
artifact. A warrior is the role running a backend such as Joy.

Worker results are untrusted inputs to cyber. Cyber MUST verify artifacts
against its own saved job expectations before accepting results. A worker's
`verified: true`, reported root or elapsed time supplies no authority to
advance state or mint rewards. Worker processes receive no node signing key
and no mutable access to the authoritative graph store.

## first supported profile

The first integration targets public stateless execution:

- target VM `nox`, proof kind `zheng-nox-public-execution-v1`;
- compiled ProgramBundle, public inputs, bounded reductions;
- execute, prove or verify as explicitly requested;
- complete witness disclosure and linear verification cost;
- state access, secret proof inputs and network rewards disabled.

This profile reuses Joy's current execution certificates. Native execution
is a distinct result kind and MUST NOT satisfy a request for a certificate.
Legacy trace-statement inspection MUST NOT satisfy execution verification.
Unsupported instructions or proof features fail explicitly.

## backend capabilities

Before dispatch, obtain backend/version, supported contract versions, VM,
proof kinds, operations and enforceable limits. The public profile advertises
`state_reads=false`, `secret_proofs=false`, `zero_knowledge=false` and
`succinct=false`. Report cancellation and hard memory/time limits only if
the adapter actually enforces them. Capability negotiation MUST use exact
profile identifiers, without falling back to a weaker proof kind.

The initial implementation MAY call `joy-rs` through a Rust adapter. Existing
Trident Runner/Prover/Verifier traits provide execution primitives; the
adapter must add job identity, typed errors, limits and cancellation.
An external-process adapter MAY follow. A remote worker protocol additionally
requires authenticated transport, authorization and artifact transfer rules.

## job model

All jobs are immutable after admission. These are logical fields, not a
JSON layout to hash. A job has:

| field | requirement |
|---|---|
| contract | exact version `cyber/worker/v1` |
| job_id | node-assigned unique opaque identifier |
| operation | execute, prove or verify |
| profile | exact VM, primitive and proof-format versions |
| program | frozen ProgramBundle bytes plus profile-defined commitment |
| public_input | ordered canonical field values |
| expected_output | optional ordered vector; mandatory when required by admission policy |
| state_context | null for the first profile |
| limits | reductions, wall time in milliseconds, memory and artifact byte limits |
| artifact | input proof for verify; absent for execute/prove |
| witness | absent for the first profile |

The bundle's source path, display name and arbitrary metadata are not its
program identity. The proof profile defines canonical program identity and
encoding; cyber checks the executed identity against the frozen bundle.
Raw CLI sources must be compiled before node dispatch. Artifact paths are
transport-local references, never authoritative identities.

Field values must be canonical for the pinned field, with no silent modular
reduction. JSON adapters encode u64 quantities as decimal strings. Admission
validates lengths and byte limits before allocations or compilation.
An unenforceable requested limit yields `unsupported_limit` before execution.
Wall-clock limits protect resources; they do not enter deterministic state.

The contract does not freeze hash parameters, root length or signed-job
serialization. These are identified by the admitted profile and must be
pinned before interoperability or reward-bearing jobs are enabled.

## state and cryptographic binding

Future stateful profiles must specify network/genesis identity, height,
commitment profile and root, plus authenticated openings for every read.
Those values must be bound into the verified statement with the exact
program and inputs. Output writes are proposed effects validated and
committed by cyber against its state-transition policy.

Placing a root in job metadata, in an unconstrained public input, or in a
worker response does not authenticate reads from that root. The initial
profile requires `state_context=null`; it cannot claim stateful execution.
If the trusted root changes during execution, cyber applies an explicit
staleness policy. It must never silently substitute the new root.

Job IDs identify dispatches. A future reward profile must additionally bind
the challenge, epoch/domain and beneficiary as its protocol requires, and
reject reused work. Merely echoing a new job ID around an old certificate
cannot establish fresh or reward-eligible work.

## lifecycle and retry

```text
queued → running → produced → verified → accepted
   │         │         │          └─ rejected
   └─ cancelled       └─ rejected
             ├─ failed
             └─ cancelled
```

`produced` means the worker returned a result. `verified` means cyber's
verifier checked it. `accepted` means node policy accepted that verified
result for its stated purpose; this can be a local computation with no
network effect. Rejected/failed/cancelled are terminal for an attempt.

Retries retain the job ID and immutable payload and increment an attempt
counter. Reusing an ID with a different payload is an error. At most one
attempt may produce an accepted result. Late results after cancellation or
acceptance are discarded. Persistent acceptance records, coupled atomically
to any state effect, are required before reward-bearing jobs are possible.
Transport retry alone offers no exactly-once guarantee.

Cancellation has an acknowledgement and bounded completion deadline. After
the deadline an isolated worker may be killed. In-process mode is eligible
only when it can meet requested cancellation/resource bounds; otherwise
dispatch must use isolation or reject the request. Partial artifacts remain
unpublished. Node restart requeues only jobs whose retry policy permits it.

## result and error model

A result carries contract, job ID, attempt, profile, program identity,
result kind, ordered public output, reductions and optional proof artifact.
The result kind is one of `execution`, `execution_certificate`, or
`verification`. A verification result identifies the checked artifact and
exact checked expectations. Telemetry such as wall time and memory is
separate from authenticated computation coordinates.

Artifacts must be complete and within declared size limits before publication.
File publication uses a temporary file and atomic rename; path-based adapters
must prevent path traversal and symlink access outside their job sandbox.
Witnesses travel through explicit in-memory or private file/pipe channels;
they must never enter argv, logs, public artifacts or diagnostics in a future
secret-capable profile. The present public profile refuses secrets entirely.

Errors contain a stable code, safe message and retryable flag. Required
codes include `invalid_job`, `unsupported_profile`, `unsupported_limit`,
`unsupported_state`, `unsupported_secret`, `budget_exhausted`,
`deadline_exceeded`, `memory_exhausted`, `artifact_too_large`,
`execution_failed`, `proof_rejected`, `claim_mismatch`, `stale_state`,
`cancelled`, `io_error` and `internal_error`. A retryable failure never
authorizes weaker verification. Backend panic/crash is a failed attempt.

## acceptance sequence

1. Resolve the saved job and active attempt; reject unknown/terminal jobs.
2. Validate result profile, type, identity, encoding and resource bounds.
3. Verify the artifact using node-selected verification code and parameters.
4. Compare authenticated program, inputs, outputs and budget with saved
   expectations. Self-contained verification alone is insufficient.
5. For future stateful/reward profiles, validate state binding, freshness,
   beneficiary and duplicate-work rules.
6. Record acceptance and any authorized state effect atomically.

Execute-only jobs return local computational results. They cannot cross
steps 3–6 as execution certificates. Cyber retains the acceptance decision
even when Joy provides the verification implementation.

## conformance gates

- Match direct Joy execution with the adapter on the same frozen bundle.
- Reject tampered program, public input/output, proof and profile.
- Reject valid certificates for another saved job's expected computation.
- Reject state/secret requests and legacy statements in the public profile.
- Exercise cancellation, worker crash, oversize artifacts and exhausted limits.
- Retry/restart without duplicate acceptance; discard late results.
- Show verification separately from network acceptance and rewards.
- Before stateful admission, demonstrate wrong-root and forged-opening failure.

CLI management lives in [[specs/cli]]. Joy's CLI is a developer interface;
cyber MUST NOT scrape its human-readable PASS messages as a worker protocol.

discover all [[concepts]]
