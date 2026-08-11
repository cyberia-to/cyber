---
tags: cyber, specs, money, soft3, tok, foculus, cyb
crystal-type: spec
crystal-domain: cyber
alias: money loop, balance send receive, reward after link
status: draft
---

# money loop

normative integration spec for the cyber network money product:

1. show balance  
2. send  
3. receive (with notification)  
4. reward after link (multi-payee)

implements the composition of [[tok]] (PLUMB), [[foculus]] (finality), [[bbg]] (state), [[zheng]] (proofs), and [[cyb]] (sigma + sense). soft3 only.

non-normative latency narrative: [[latency targets]].  
node modes: [[specs/node-modes\|node-modes]].  
light path: [[specs/light-money\|light-money]].

---

## 1. objects

| object | definition | owner repo |
|---|---|---|
| Coin | fungible class + balances | [[tok]] TSP-1 |
| Card | unique holder (neuron, asset, …) | [[tok]] TSP-2 |
| neuron Card | identity + Sigma holdings | [[tok]], [[cyb/robot]] |
| cyberlink | edge + economic weight | [[cybergraph]] |
| Intent | atomic PLUMB op list | [[tok]] PLUMB |
| signal | signed cyberlink batch (+ proof when sealed) | [[cybergraph]], [[foculus]] |
| nullifier | spend uniqueness tag | [[foculus]], [[security]] |
| BBG_root | polynomial state commitment | [[bbg]] |
| tip | trusted (root, height, folding_acc) | [[foculus]] structural-sync, [[zheng]] |

Sigma (product term) = the set of Coin balances and Card bonds held by a neuron Card. not a separate ledger type.

---

## 2. clocks (normative names)

implementations MUST use these names in APIs and UI state machines so products do not collapse distinct times into one spinner.

| clock | name | condition | money meaning |
|---|---|---|---|
| A | transfer finality | particle final: $\phi^*_i > \tau$ in domain with completeness; nullifiers committed | pay outputs spendable |
| B | attribution settlement | epoch settle + mint + reorg depth $d$ | contribution reward spendable |
| C | history trust | `decide(folding_acc)` valid for tip (or full-node equivalent) | tip openings are money-grade |

pure send/receive uses A (+ C on light). attribution rewards use B. see [[latency targets]] for target durations; see [[foculus parameters]] for knobs.

---

## 3. certainty grades

| grade | condition | allowed UI |
|---|---|---|
| 0 | local author only | draft / local submit |
| 1 | $\sigma$ accepted by peers; not final | pending |
| 2 | clock A at trusted tip | sent / received / spendable |
| 3 | clock B complete for that credit | earned reward spendable |
| 4 | clock C (or full/cell history equivalent) for tip | tip trusted; openings money-grade |

rules:

- R1: MUST NOT mark received or allow respend below grade 2  
- R2: on light mode, grade 2 REQUIRES grade 4 (tip from fold)  
- R3: MUST NOT treat unauthenticated peer JSON as balance or receive  
- R4: sense NOTIFY for money MUST fire only on grade ≥ 2 credits (or grade ≥ 3 for settle-only mints)

---

## 4. operations

### 4.1 balance

query: balances of neuron Card $N$ at tip $T = (\texttt{BBG\_root}, h)$.

| mode | method |
|---|---|
| full / cell with apply | local state after apply of all signals ≤ $h$ touching $N$ |
| light / cell open path | Lens open coins (or private note commitments) at key for $N$ against $\texttt{BBG\_root}$ |

response MUST include: `(token_id, amount, tip_height, proof)` where proof verifies against tip root (or empty proof only if mode is full node serving itself).

acceptance:

- opening verifies against tip OR local full apply matches root  
- amount conservation holds for known mints/burns (PLUMB laws)

### 4.2 send

input: `(from_neuron, to_holder, token, amount, optional memo/particle)`.

steps (normative order):

1. tip ready: grade 4 on light; full/cell tip current  
2. select inputs / notes owned by `from_neuron` with witnesses at tip  
3. build Intent: one or more PLUMB `pay` ops (change outputs allowed)  
4. prove with [[zheng]]: auth, conservation, fresh nullifiers, well-formed links  
5. seal to signal; gossip via [[radio]] / [[foculus]]  
6. local UI → grade 1 on network-valid $\sigma$  
7. on clock A final at tip → grade 2; update sigma; optional payer sense echo  

rejection: any peer MUST drop signals failing $\sigma$ or nullifier already in $N$.

### 4.3 receive

no separate receive transaction.

1. watch tip for finalized signals whose payee set includes local neuron (or open balance/note deltas)  
2. verify credit against tip (apply or Lens)  
3. on grade 2: update sigma; emit sense event `TransferIn`  
4. new outputs become spendable for subsequent send  

acceptance: receiver can respend only after grade 2.

### 4.4 reward after link (multi-payee)

a link Intent MAY include any of:

| leg | type | clock | payees |
|---|---|---|---|
| structural cyberlink | edge + optional stake weight | A | — (structure) |
| pay to counterparty | PLUMB pay | A | other neuron/card |
| pay to self / stake position | PLUMB pay/lock as designed | A | linker |
| attribution mint | settle pipeline | B | any payee set from reward spec |

implementations MUST support multi-payee: one Intent / one signal MAY credit more than one holder. MUST NOT hardcode a single exclusive policy (linker-only XOR owner-only).

events:

```
RewardCredited { to, amount, token, reason: signal_id | link_id, clock: A|B }
TransferIn     { to, from, amount, token, reason: signal_id }
TransferOut    { from, to, amount, token, reason: signal_id }
```

sense:

- every payee gets NOTIFY on their credit at the grade required by the leg  
- payer MAY get outbox echo (optional, product)  

---

## 5. state machine (client)

```
          compose
             │
             ▼
         [grade 0]
             │ broadcast
             ▼
         [grade 1 pending]
             │
        ┌────┴────┐
        │ final A │
        ▼         │
    [grade 2]     │ conflict lost → pruned / failed
        │
        │ if settle mint for me
        ▼
    [grade 3] after B
```

tip side-channel (light):

```
empty disk → download acc → decide → [grade 4]
grade 4 ──fold──► grade 4' (new height)
```

---

## 6. event bus (cyb)

minimum events the robot MUST expose to sense/sigma:

| event | payload | when |
|---|---|---|
| TipAdvanced | root, height, grade4 | fold or full apply |
| BalanceUpdated | neuron, balances[], tip | after open/apply |
| TransferOut | … | grade 2 pay from me |
| TransferIn | … | grade 2 pay to me |
| RewardCredited | … | grade 2 (pay leg) or grade 3 (settle) |
| FinalityFailed | signal_id, reason | pruned / conflict lost |

intent particle for notifications: `intent/notify` (see cyb-core). payload MUST bind `reason` to signal/link id.

---

## 7. latency acceptance (Earth hub typical)

normative *acceptance bands* for product tests (not security parameters):

| action | accept if |
|---|---|
| cold light join verify | decide completes after download; verify ≪ 1 s compute |
| send → grade 2 | p50 ≤ 5 s hub domain under honest majority; document p95 |
| receive NOTIFY after final | ≤ 1 RTT + open verify after local tip sees final |
| settle reward grade 3 | within configured epoch×$d$ (see parameters); UI separate from send |

sparse domains and conflict races MAY exceed hub bands; UI MUST not promise a single global SLA.

---

## 8. security requirements

| property | requirement |
|---|---|
| double spend | same nullifier cannot finalize twice (foculus) |
| conservation | PLUMB laws unprovable if broken (zheng rejects) |
| tip integrity (light) | openings MUST verify against grade-4 tip |
| privacy | private notes: openings do not reveal unlinkable secrets; public box pays are explicit |
| no foreign chain | this loop does not depend on external L1 schedules |

---

## 9. non-goals

- investmint / grid energy product surfaces  
- AMM / DEX  
- full tri-kernel on light client  
- social recovery / multisig (compose later via PLUMB hooks)

---

## 10. conformance checklist

- [ ] balance query returns tip-bound proof or full local apply  
- [ ] send produces zheng-valid signal; invalid never accepted  
- [ ] receive only at grade 2; sense NOTIFY only then  
- [ ] multi-payee Intent credits both linker and counterparty when both present  
- [ ] light mode refuses money-grade opens without grade 4  
- [ ] UI labels distinguish clock A vs B  
- [ ] no dependency on non-soft3 chains  

---

see [[specs/node-modes\|node-modes]], [[specs/light-money\|light-money]], [[specs/component-ownership\|component-ownership]], [[latency targets]], [[tok]], [[foculus protocol]], [[structural sync]].

---

discover all [[concepts]]
