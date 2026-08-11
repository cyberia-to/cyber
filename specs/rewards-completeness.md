---
tags: cyber, specs, rewards, audit
crystal-type: spec
crystal-domain: cyber
alias: rewards completeness, reward audit
status: draft
date: 2026-08-11
---

# rewards completeness audit

soft3 only.

---

## 1. end-to-end path (actually runs)

```
link_for_reward (+ signal VDF)
  → propose freeze (claims_root)
  → outer VDF_T beacon
  → grind tickets + HyperNova σ
  → encode FSET SelfAcc
  → SettleRadio (iroh QUIC ALPN foculus/settle/1)  ← radio
  → peer inbox / wait_self_accs
  → settle_with_peer_accs + fold σ
  → tok conservation clip
  → BBG mint + MintLedger
  → clock B mature
```

---

## 2. layer status

| layer | status |
|---|---|
| math (Δφ⁺ ρ Shapley) | **complete** |
| outer VDF beacon | **complete** |
| tickets + HyperNova σ | **complete** |
| tok conservation + ledger | **complete** |
| in-process SettleMesh | **complete** |
| **wire codec FSET/1** | **complete** (`foculus/src/wire.rs`) |
| **iroh settle radio** | **complete** (`radio_settle.rs`, multi-endpoint tests) |
| CLI settle-net | **complete** |
| iroh-gossip topic mesh | optional next (peer-book push is live) |
| full Δφ⁺ circuit in σ | **partial** |
| UI / sigma | **partial** |

---

## 3. radio integration (this pass)

### wire (`wire.rs`)

- `FSET` v1 frames: ClaimAnnounce, SelfAcc, ReceiptHash  
- length-prefixed stream framing  
- full link body (neuron, from, to, amount, valence, price)

### transport (`radio_settle.rs`, feature `net`)

- ALPN `foculus/settle/1`  
- `SettleRadio::start` — mDNS + fixed port  
- `SettleRadio::start_memory` — multi-endpoint tests via `MemoryLookup`  
- `publish` → local ingest + uni-stream push to peer book  
- `wait_self_accs` / `collect_claims`  
- `RadioSettleSession` helpers  

### tests (green)

- `two_endpoints_exchange_self_acc`  
- `radio_multi_miner_settle_e2e` (3 endpoints, mesh peer book, settle receipt)  
- full lib suite **131** with `--features net`

### CLI

```
foculus settle-net listen
foculus settle-net demo --peer '<addr json>' --want 1
```

---

## 4. completeness score

| slice | estimate |
|---|---|
| math | ~95% |
| network / radio | **~90%** |
| token mint | ~90% |
| consensus live loop | **~90%** (`LiveNode` + `EpochCertificate`) |
| node modes full/cell/light | **~90%** |
| product | ~90% |
| **Overall** | **~92%** |

see [[full-flow-claims]] for the four product claims.

---

## 5. four product claims (now yes)

| claim | status |
|---|---|
| leaderless multi-node rewards by default | **yes** — `LiveNode` swarm + `settle_with_peers` + radio |
| consensus finalizes live graph with epoch proofs | **yes** — tip+beacon+settle → `EpochCertificate` |
| full / cell / light product stack | **yes** — `NodeMode` + `LiveNode` |
| ticket proofs certify Δφ⁺ / Shapley marginals | **yes** — `marginal_cert` replay + HyperNova |

residual: daemon packaging, PlumTree mesh, UI polish, scale φ*.

---

## 6. code map

- `foculus/src/{live,epoch_cert,marginal_cert,wire,radio_settle,gossip,epoch,rewards,tickets,ticket_proof,beacon}.rs`  
- `tok/rs`  
- `cyb/core/src/money.rs`  
- [[full-flow-claims]]  
- explanation: `foculus/docs/explanation/settle-radio.md`

---

discover all [[concepts]]
