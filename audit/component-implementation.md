---
title: component implementation report
tags: cyber, audit, soft3, architecture
crystal-type: report
crystal-domain: cyber
alias: component implementation status, money work package status
---

# component implementation report

Implementation notes extracted from the component ownership specification.
The statuses and test commands below preserve the earlier component report;
this relocation adds no new verification of those claims. Ownership and
change control are defined in [[specs/component-ownership]]. Executable node
readiness is recorded separately in [[audit/node-readiness]].

## work packages — status

### WP0 — contracts freeze — **done**

- owner: cyber (this specs tree)
- deliverable: money-loop, node-modes, light-money, ownership

### WP1 — tip + fold (clock C) — **done (library)**

- owner: zheng + foculus
- code: `foculus/src/tip.rs` — `Tip`, `TipProver`, `fold_height`, `join_checkpoint`, `advance_fold`, `seal_tip`
- each height binds `(height, root)` into HyperNova acc; light join = decide + verify

### WP2 — openings (balance) — **done (public box)**

- owner: bbg + cyb money
- code: `MoneyWallet::open_balance` → `prove_balances` + `verify_query`
- bbg re-exports `balance_key`, `NeuronRecord`
- private-note openings remain future work

### WP3 — pay Intent / send — **done (cell path)**

- owner: tok shape via cyberlinks + nullifiers
- code: `MoneyWallet::pay` / `send`; `Signal.box_moves` → bbg nullifier gate
- multi-payee Intent; double-spend rejected
- zheng σ on every pay still optional (local apply path)

### WP4 — thin finality evidence — **done (local + certified binding)**

- owner: foculus
- code: `foculus/src/finality_evidence.rs` — `FinalityEvidence::{issue_local, issue_certified, verify}`
- light verifies binding against grade-4 tip without tri-kernel
- full φ* circuit evidence remains future (provable-consensus)

### WP5 — multi-payee + sense events — **done (library)**

- owner: cyb-core
- code: `MoneyEvent` bus + `sense::money_to_sense` → `SenseNotify` with `NOTIFY` intent

### WP6 — attribution settle (clock B) — **done (mint + depth)**

- owner: cyb money + foculus height
- code: `mint_settle_reward`, `settle_depth`, `mature_settles`, `finalize_block`
- full Shapley settle lottery remains foculus settlement; wallet consumes mint results

### WP7 — cyb product wiring — **done (CLI)**

- owner: cy
- code: `cyb/cli` — `fund`, `balance`, `send`, `events`, `sense`, `finalize`
- Bevy sigma/sense screens remain open

---

## code map

| package | path |
|---|---|
| tip / fold | `foculus/src/tip.rs` |
| finality evidence | `foculus/src/finality_evidence.rs` |
| money wallet | `cyb/core/src/money.rs` |
| sense bridge | `cyb/core/src/sense.rs` |
| box_moves wire | `foculus/src/chain.rs`, `frames.rs`, cybergraph bridge |
| CLI | `cyb/cli/src/main.rs` |

### tests

```
cd ~/cyber/foculus && cargo test --lib --no-default-features
cd ~/cyber/cyb && cargo test -p cyb-core --lib
cd ~/cyber/cyb && cargo build -p cy
```

---

## remaining (next depth)

| item | status | notes |
|---|---|---|
| zheng σ required on every pay | **landed** | `foculus/pay_proof` + `MoneyWallet::require_pay_proof` |
| finality binds nullifiers | **landed** | `FinalityEvidence` v1 |
| private notes + nullifier spend | **landed (wallet)** | `mint_private_note` / `spend_private_note`; full AOCL/SWBF later |
| domain finality gate | **landed** | `FinalityEvidence::issue_from_domain` wraps `finalizes()` |
| Bevy sigma UI | **landed** | `cyb/shell` world `Sigma` · Cmd+4 · `cyb://sigma` |
| tip fold block+leaves | **landed** | `TipProver::fold_block(height, root, leaves_hash)` |
| full φ* SpMV circuit in zheng | **landed (domain)** | `zheng/rs/src/phi` — SpMV CCS + prove_phi_star; planetary scale = same code larger n |
| full AOCL/SWBF mutator set | open | wallet private notes first cut |

---

discover all [[concepts]]
