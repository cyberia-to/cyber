---
title: cyber cryptographic readiness
tags: cyber, roadmap, hemera, eidos, zheng, lens
crystal-type: plan
crystal-domain: cyber
status: planned
---

# cryptographic readiness

This track retains the earlier Hemera/Eidos objectives and supplies exact
profiles to the A–C integration work. Local development can exercise versioned
experimental profiles; production acceptance requires the evidence below.

## H1: finalize Hemera's primitive

Owner: Hemera.

- [ ] Resolve the exact mixed x^7/inverse RF=8/RP=16 candidate against explicit
  collision, preimage, second-preimage and transcript-use requirements.
- [ ] Validate attack candidates against the real implementation; distinguish
  complete-profile results from local S-box properties and reduced models.
- [ ] If a requirement fails, carry a justified replacement through matched
  cost/security evaluation and production parameter selection.
- [ ] Freeze field, matrices, constants, rounds, domain separation, output/encoding
  and streaming behavior with versioned vectors and migration rules.

Evidence starts in [Hemera research](../../hemera/research/inverse-sbox-assessment.md).
The earlier local witness and degree findings retain their stated scope.

## H2: structural commitments and the .cyb adapter

Owner: Hemera; format semantics supplied by the .cyb owner.

- [ ] Finalize the permutation/sponge → universal structural commitments → format
  adapter layering and its canonical blob/sequence/record encodings.
- [ ] Specify semantic section/chunk extraction, order, names, offsets and parser
  rejection behavior. Bind structure and domain unambiguously.
- [ ] Validate inclusion/opening rules, streaming/chunk equivalence and identity
  stability; version any change to existing particle/tree interpretation.

Contract: [structural commitments](../../hemera/specs/structural-commitments.md).

## E1: harden Eidos and refine the implementation

Owners: Eidos kernel/checker; Hemera theorem and implementation models.

- [ ] Validate inductive environments, normalization/resource bounds and the
  closed-term checking boundary; reject unsupported claims explicitly.
- [ ] Establish universal field and machine-word arithmetic obligations, including
  noncanonical inputs, overflow and reduce128, in checked proof terms.
- [ ] Prove inversion-chain, encoding, padding, permutation, streaming and tree
  refinements against the actual implementation and its pinned sources.
- [ ] Maintain negative controls and source/model conformance. Report cryptanalytic
  assumptions separately from functional refinement and checker trust.

Evidence: [Hemera proof coverage](../../hemera/audit/formal-proofs.md);
interface: [strict checking](../../eidos/specs/strict-checking.md).

## P1: network-usable execution profiles

Owners: Zheng/Lens/Nox, Joy adapter and Cyber admission.

- [ ] Pin supported statements, encodings, verifier parameters and disclosure.
- [ ] Complete authenticated state reads/transitions and any advertised secret,
  hiding or succinct proof mode; connect them to node-owned trusted context.
- [ ] Cover program/input/output/state binding, forged openings, malformed sizes,
  replay/freshness and expected-job matching with adversarial tests.
- [ ] Version source compilation semantics and its validation separately from
  machine execution proofs. Migrate all root/digest/codec consumers together.

Exit: each production operation selects a fully specified, implemented and
validated profile; the exact primitive and implementation evidence supports
the security and disclosure claims made for that operation.

discover all [[concepts]]
