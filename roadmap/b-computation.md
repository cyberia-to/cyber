---
title: cyber computation delivery
tags: cyber, roadmap, joy, trident, worker
crystal-type: plan
crystal-domain: cyber
status: planned
---

# B — computation in the cyber binary

Deliver standalone program development and node-managed computation through
shared Trident/Joy libraries. Prerequisite: the A storage/admission foundation
for persistent node jobs. Stateless compiler integration can be developed sooner.

## B0: complete the product interface

Owners: Cyber, Joy and Trident.

- [ ] Reconcile the one-binary product with the current Cyber CLI text directing
  standalone execution to Joy. Specify the program command namespace before
  implementation; proposed surface: cyber program build/run/prove/verify/deploy.
- [ ] Share implementation with the standalone Joy CLI. Keep node/worker lifecycle
  independent of stateless program execution and node configuration.
- [ ] Version command results, canonical inputs, errors, artifact publication and
  output overwrite rules. Distinguish execution, proof verification and admission.

Exit: [Cyber CLI](../specs/cli.md), [Joy CLI](../../joy/specs/cli.md) and the
[worker contract](../specs/worker.md) define a consistent user/API surface.

## B1: build and execute

Owners: Trident compiler, Joy adapter, Cyber product. Prerequisite: B0.

- [ ] Add Joy build through Trident's shared library and expose it from Cyber.
- [ ] Resolve target packages, imports, profile precedence and artifact identity
  consistently. Bundle compiler/SDK resources for execution outside a checkout.
- [ ] Execute supported nox programs with explicit inputs and reduction budgets.

Exit: standalone build/run works with an empty node home; equivalent inputs
through Trident, Joy and Cyber produce matching program semantics and artifacts.

## B2: prove and verify

Owners: Joy/Zheng/Lens; Cyber retains expected job statements.
Prerequisites: B1 and the selected cryptographic profile.

- [ ] Integrate the explicit public execution profile first. Bind exact program,
  public inputs/output, primitive versions and supported budgets.
- [ ] Keep disclosure, succinctness and supported instructions in capabilities.
  State and secret requests require separately implemented proof profiles.
- [ ] Reject altered statements, malformed artifacts, legacy trace substitution
  and valid proofs for another requested computation.

Exit: a fresh Cyber process independently verifies the artifact against requested
expectations; verification success has its documented computational meaning.

## B3: managed workers

Owners: Cyber scheduling; Joy execution adapter. Prerequisites: A1–A3, B2.

- [ ] Persist immutable jobs, attempts, results and acceptance through the shared
  graph/storage boundary. Enforce one acceptance despite retries and restarts.
- [ ] Implement authenticated local control and worker status/start/stop.
- [ ] Enforce advertised concurrency, memory, time and artifact limits; implement
  acknowledged cancellation and discard late results.
- [ ] Support embedded execution where its limits are enforceable. Use an isolated
  worker mode of the same product where stronger isolation is required.

Exit: direct Joy and managed-worker results agree; worker crash, cancellation,
budget exhaustion and retry preserve node state and acceptance records.

## B4: artifact handoff and delivery

- [ ] Define the immutable deployment-plan API, signer reference and receipt types
  together with C's network admission contract. Keep build/proof and submission
  as separate explicit operations.
- [ ] Ship the supported B command surface through the distribution checks.

Exit: one installed Cyber binary handles the advertised local program lifecycle.
Live deployment follows C's verified network path.

discover all [[concepts]]
