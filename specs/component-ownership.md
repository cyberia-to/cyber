---
tags: cyber, specs, soft3, architecture
crystal-type: spec
crystal-domain: cyber
alias: component ownership, money ownership matrix
status: draft
---

# component ownership

who implements what for the cyber money loop + light client. integration contracts live in [[cyber]]/specs; algorithms live in component repos.

---

## 1. ownership matrix

| concern | normative spec | implement in | consumed by |
|---|---|---|---|
| PLUMB pay/mint/burn, Intent | [[tok]] | tok + nox circuits | cybergraph, cyb |
| signal structure, seal | cybergraph + [[structural sync]] | cybergraph, foculus | radio, cyb |
| gossip / DAS / CRDT local | [[structural sync]], gossip | foculus, radio | cell |
| finality φ* > τ, nullifiers | [[foculus protocol]] | foculus, tru | all modes |
| thin finality evidence | [[specs/light-money\|light-money]] | foculus `finality_evidence` | cyb money |
| epoch settle / attribution | reward spec, fold-mining | foculus, tok mint | cyb sense |
| BBG state + Lens open | bbg research/spec | bbg, lens | light, cell, full |
| zheng prove/verify/fold | [[zheng]] | zheng | all prove paths |
| tip checkpoint + decide | [[structural sync]], [[specs/light-money\|light-money]] | foculus `tip` | light, cell |
| sigma / sense UI events | [[specs/money-loop\|money-loop]] | cyb-core money + sense | human |
| network product contract | **this directory** | — | all implementers |

---

## 2. dependency direction

```
cyb (sigma, sense, CLI)
  │  uses
  ▼
cyber/specs          ◄── product contracts (this tree)
  │  cites
  ├─► tok            value ops
  ├─► foculus        finality, sync, nullifiers, tip fold, finality evidence
  ├─► zheng          proofs, fold, decide
  ├─► bbg / lens     state openings
  ├─► cybergraph     signals + box_moves bridge
  ├─► radio / tape   transport
  └─► tru            φ* operators (full/cell consensus compute)
```

---

## 3. work packages — status

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

## 4. code map

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

## 5. remaining (next depth)

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

## 6. change control

- breaking change to clocks, grades, or tip object → version bump in this directory
- parameter number changes ($d$, epoch lengths) → [[foculus parameters]] only
- explanation prose → foculus `docs/explanation/latency-targets.md`

---

discover all [[concepts]]
