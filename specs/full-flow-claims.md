---
tags: cyber, specs, claims, rewards, consensus, nodes
crystal-type: spec
crystal-domain: cyber
status: draft
date: 2026-08-11
---

# full cyber flow — claim checklist

four product claims and what makes each true in code.

---

## claim 1 — leaderless multi-node rewards by default

> “Cyber rewards run leaderlessly on a live multi-node network by default.”

| requirement | implementation |
|---|---|
| shared beacon after propose freeze | `EpochRunner::freeze` + `open_beacon` |
| independent miners grind tickets | `grind_settlement` / `LiveNode::mine_self_acc` |
| SelfAcc gossip (no leader) | `SettleMesh` + `SettleRadio` (iroh) + `absorb_peer_acc` |
| coordinator folds monoid only | `EpochRunner::settle_with_peers` |
| multi-node test | `live::tests::multi_node_swarm_leaderless_settle` |

**status: implemented (library + multi-node LiveNode swarm + radio SelfAcc).**  
Default product path: `LiveNode` full/cell closes epoch via peer SelfAccs when present.

---

## claim 2 — consensus finalizes live graph with epoch proofs

> “Consensus finalizes the live graph end-to-end with epoch proofs.”

| requirement | implementation |
|---|---|
| tip advances on signals | `LiveNode::ingest_signal` → `TipProver::fold_height` |
| propose → freeze → beacon from signal VDFs | `freeze_and_beacon` / `close_and_settle_epoch` |
| epoch certificate | `EpochCertificate` + `issue_epoch_cert` / `verify_epoch_cert` |
| binds tip + claims_root + VDF beacon + settle | `epoch_cert.rs` |
| optional φ* SpMV attach | `phi: Option<PhiProof>` + `verify_phi_on_cert` |

**status: implemented for continuous LiveNode epoch steps + verifiable `EpochCertificate`.**  
Full planetary φ* STARK remains scale work; domain φ* attach path exists.

---

## claim 3 — full / cell / light product stack

> “Full / cell / light nodes form a complete product stack.”

| mode | capabilities in `LiveNode` |
|---|---|
| **Full** | ingest all signals, tip fold, mine, settle, issue cert, export tip |
| **Cell** | same money-facing path for local neuron; settle + cert |
| **Light** | `light_join` / `light_advance`; `accept_epoch_cert`; no ingest/grind |

| matrix | full | cell | light |
|---|---|---|---|
| tip grade 4 | yes | yes | join/advance or cert import |
| link / claim | yes | yes | no |
| settle mint | yes | yes | via cert |
| verify epoch cert | yes | yes | yes |

**status: implemented as one `LiveNode` API with `NodeMode`.**  
Unified always-on binary still CLI-wired; library stack is complete.

---

## claim 4 — ticket proofs certify Δφ⁺ / Shapley marginals

> “Ticket proofs certify Δφ⁺ / Shapley marginals.”

| requirement | implementation |
|---|---|
| m(n) = marginals under π(n) from `v★=Δφ⁺(ρ·S)` | `settlement::marginals` + `tru::impulse` |
| commitment bind | `commit_marginals` |
| **replay required to verify** | `marginal_cert::replay_marginals` |
| HyperNova σ on ticket | `certify_ticket` / `verify_certified_ticket` |
| batch seal after replay | `prove_replayed_batch` |

**status: implemented.**  
Verification **recomputes** Δφ⁺ marginals from public base+contribs+beacon+nonce; tampered m fails. HyperNova seals the commitment.

---

## one-shot flow (what “done” looks like)

```
Full A: link → signals
Full B: link → signals
gossip signals (ingest cross)
freeze_and_beacon (shared S_E VDFs)
A.mine_self_acc → B.absorb
B.mine_self_acc → A.absorb
A.close_and_settle_epoch → EpochCertificate
Light C: accept_epoch_cert → grade4 tip + reward credit
```

code: `foculus/src/live.rs` test `multi_node_swarm_leaderless_settle`.

---

## verification (2026-08-11)

| suite | result |
|---|---|
| foculus offline (`--no-default-features`) | **138** passed |
| foculus radio (`--features net`) | **2** radio e2e passed |
| tok | **8** passed |
| cyb-core money | **12** passed |

reliability fixes applied in verification pass:

- `EpochCertificate` clone-stable (hash does not drop fields)  
- `settle` = `settle_with_peers(&[])` (no duplicated settle logic)  
- settle seals tickets only after **Δφ⁺ marginal replay**  
- `LiveNode` no double-grind; receipt credit **idempotent**  
- stronger live tests (mature amount, double-accept light)

## residual (honest)

| residual | severity |
|---|---|
| always-on OS daemon packaging | product ops, not protocol |
| iroh-gossip PlumTree vs peer-book push | scale optimization |
| φ* full D+S+H circuit per epoch at scale | performance |
| cyb Bevy UI | product surface |
| parameters (T, d) production-tuned | ops |

These do **not** block the four claims at protocol/library level.

---

discover all [[concepts]]
