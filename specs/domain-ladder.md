---
title: subjects, programs and graph domains
tags: cyber, soft3, neuron, architecture
crystal-type: spec
crystal-domain: cyber
status: accepted
---
# Subjects, programs and graph domains

The old cell ladder grouped several responsibilities by their shared need for
state, addressing and lifecycle. Those properties do not imply one kind of
signing subject. [Cyb architecture](../../cyb/specs/architecture.md) defines one
subject: neuron. The named robot attaches neurons, each able to execute progs.

| Role | State / identity | Authority and preserved obligations |
|---|---|---|
| Neuron | Protocol subject with native/domain-qualified ID | Key/proof policy, attribution, linking, protocol resources |
| Prog / invocation | Installed code/state and retained work IDs | Acts under captured neuron/network/grant; cancellation, continuation, budget and recovery |
| GraphSession / node | Retained multi-neuron graph and service placement | Validates/serves/commits according to its profile; no signer by being a process |
| Partial node | Selected graph slice, notes and authenticated tip | Apply/open relevant changes, verify coverage and tip proofs, retain private state |
| Service / building | Application data, API and progs | Player/collective governance; independent neuron only when needed for authority |
| Book / issuer | Token's complete obligations under its named home ledger | Issuance/spending rules, conditions, expiry, receipts and monetary conservation |
| Shard | Region of graph state and its routing/checkpoint identity | Validation, availability, boundary proofs, shard-local computation and split/merge protocol |
| Zone / domain / root | Hierarchical graph aggregation and routing | Authenticated summaries, coverage and selected consensus duties |

One service can use several progs and subjects; one subject can serve several
programs/devices/networks. A book or shard may have an authority set, but its data
ID is not silently converted into a native NeuronId. A deployment boundary, NFT,
database, checkpoint or numeric subnet slot is not sufficient proof of custody.

Graph sharding preserves the proposed spectral split/merge and locality duties.
It does not split a neuron key, mint subjects for every partition or reset task
budgets. Token books preserve issuer authority and conditional settlement without
making every program its own ledger. Services preserve governance, admission and
delivery guarantees without turning each screen/building into a signer.

Full/partial/light are node participation modes, independent of the number of
neurons or installed progs. All retain the selected protocol's validity, nullifier,
coverage and finality obligations; partial does not mean weaker correctness.
Actual supported evidence/worker/storage profiles must be declared separately.

3C remains the read/write/trade application contract between domains. Endpoints
carry typed subject/domain/network references, exact operation and evidence.
Radio transports and tape frames; neither proves remote authority or consensus.
Local execution commit, signed Signal and finalized network fact are distinct.

Original signed/wire history and foreign networks remain unchanged. Old names
survive only as compatibility links or immutable provenance. Biological cells,
VM pairs, storage/table cells and upstream terminology retain their meanings.
