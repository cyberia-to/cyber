---
tags: cyber, specs, soft3, core
crystal-type: spec
crystal-domain: cyber
alias: cyber specs, network specs, money loop specs
status: draft
---

# cyber specs

integration specifications for the [[cyber]] network: the product contracts that span soft3 components. component repos own *how* a mechanism works. this directory owns *what the network requires* when those mechanisms compose into money, light clients, and neuron UX.

## why here (not tok, not foculus alone, not cyb alone)

| home | role | wrong for |
|---|---|---|
| [[foculus]]/specs | consensus, structural sync, finality clocks | end-to-end money product; cyb event shapes |
| [[tok]] | coin/card natures, PLUMB ops, conservation | tip trust, light join, sense notifications |
| [[zheng]] | prove/verify/fold machinery | balance UX, multi-payee product rules |
| [[cyb]] | robot UI, sense, sigma surfaces | protocol-normative wire and finality |
| [[cyber]]/specs | **network product contracts** | re-deriving φ* math or PLUMB field layouts |

cyber is the root protocol graph and the name of the network. cross-cutting "balance + send/receive + reward-after-link + light client tip trust" is a network property. implementers read these specs, then open the linked component specs for algorithms and constants.

explanation (non-normative clocks narrative): [[latency targets]] in foculus docs.  
normative constants: [[foculus parameters]].  
value ops: [[tok]] / PLUMB.  
join protocol detail: [[structural sync]] light client section.

## documents

| doc | status | what it specifies |
|---|---|---|
| [[specs/money-loop\|money-loop]] | draft | balance, send, receive, multi-payee reward-after-link, events, certainty grades |
| [[specs/node-modes\|node-modes]] | draft | full node, cell, light client — storage, duties, what each must implement |
| [[specs/light-money\|light-money]] | draft | light path for tip trust + money (fold, openings, send/receive on thin devices) |
| [[specs/component-ownership\|component-ownership]] | draft | ownership matrix, required interfaces, acceptance tests |
| [[specs/rewards-completeness\|rewards-completeness]] | draft | audit of reward pipeline: math vs protocol vs mint vs product |

## scope boundaries

in scope:

- soft3 / cyber protocol only (no foreign chain schedules)
- light client as first-class tip path for money
- multi-payee rewards (linker and counterparty in one Intent model)
- clocks A (transfer finality), B (attribution settle), C (history fold)

out of scope for these docs:

- re-specifying tri-kernel math (→ [[tru]], [[foculus]])
- PLUMB field encodings (→ [[tok]])
- UI layout pixels (→ [[cyb]])
- interplanetary parameter tables (→ [[interplanetary]], parameters)

## reading order for implementers

1. [[specs/component-ownership\|component-ownership]] — who builds what  
2. [[specs/node-modes\|node-modes]] — which mode you ship  
3. [[specs/money-loop\|money-loop]] — events and state machine  
4. [[specs/light-money\|light-money]] — if you ship thin tip trust  
5. component specs linked from each section  

## implementation map (code)

| contract | code |
|---|---|
| tip / clock C | `foculus/src/tip.rs` — `Tip`, `TipProver`, fold per height |
| thin finality | `foculus/src/finality_evidence.rs` (nullifier-bound) |
| pay σ | `foculus/src/pay_proof.rs` — prove_pay / verify_pay |
| money loop | `cyb/core/src/money.rs` — proofs, private notes, settle |
| sense | `cyb/core/src/sense.rs` — `money_to_sense` |
| box_moves | `foculus` Signal + cybergraph bridge |
| CLI (WP7) | `cy fund/balance/send/events/sense/finalize` |

all WP0–WP7 library + CLI: see [[specs/component-ownership\|component-ownership]] status.

run tests:

```
cd ~/cyber/foculus && cargo test --lib --no-default-features
cd ~/cyber/cyb && cargo test -p cyb-core --lib
cd ~/cyber/cyb && cargo build -p cy
```

---

discover all [[concepts]]
