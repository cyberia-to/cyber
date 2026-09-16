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

This is the target network money contract. Current native HTTP receipts and
GraphSession projections establish local observations and durable acceptance.
Library proof/fold support or the development `cy finalize` command alone does
not establish a deployed network's clocks A, B or C. A product must declare its
verified profile and evidence before assigning the grades below.

---

## 1. objects

| object | definition | owner repo |
|---|---|---|
| Coin | fungible class + balances | [[tok]] TSP-1 |
| Card | unique token object / holder reference under its ledger profile | [[tok]] TSP-2 |
| neuron | sole signing subject; native NeuronId or domain-qualified foreign reference | neuron-id/model, [[cyb]] attachments |
| neuron Card | ledger's representation of a subject and holdings | [[tok]], [[cyb/robot]] |
| prog / invocation | retained executable work and continuation under a neuron | neuron |
| cyberlink | edge + economic weight | [[cybergraph]] |
| Intent | atomic PLUMB op list | [[tok]] PLUMB |
| signal | signed cyberlink batch (+ proof when sealed) | [[cybergraph]], [[foculus]] |
| nullifier | spend uniqueness tag | [[foculus]], [[security]] |
| BBG_root | polynomial state commitment | [[bbg]] |
| tip | trusted (root, height, folding_acc) | [[foculus]] structural-sync, [[zheng]] |

Sigma presents Coin balances and Card bonds under their subject, network and
ledger profile. Protocol state owns these values. A Card, prog, book or shard
ID grants no signing authority by itself; [domain roles](domain-ladder.md)
identify each role's actual control policy. A named robot can attach several
neurons with different networks, keys and devices. Public observations and
private custody remain separately scoped.

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
| 4 | clock C (or verified full/partial history equivalent) for tip | tip trusted; openings money-grade |

rules:

- R1: MUST NOT mark received or allow respend below grade 2  
- R2: on light mode, grade 2 REQUIRES grade 4 (tip from fold)  
- R3: MUST NOT treat unauthenticated peer JSON as balance or receive  
- R4: sense NOTIFY for money MUST fire only on grade ≥ 2 credits (or grade ≥ 3 for settle-only mints)
- R5: every observation MUST retain subject, network, commitment/proof profile
  and source; late results cannot replace another selected subject's view
- R6: an endpoint's accepted receipt MAY be displayed as pending, with its
  acceptance scope explicit; grade 2–4 require the corresponding network evidence

---

## 4. operations

### 4.1 balance

query: balances of neuron Card $N$ at tip $T = (\texttt{BBG\_root}, h)$.

| mode | method |
|---|---|
| full / partial with complete apply | verified state after applying all relevant signals ≤ $h$ with authenticated coverage |
| light / partial open path | Lens open coins (or private note commitments) at key for $N$ against $\texttt{BBG\_root}$ |

response MUST include: `(token_id, amount, tip_height, proof)` where proof verifies against tip root (or empty proof only if mode is full node serving itself).

The envelope also binds subject, network, root and proof/coverage profile.
Aggregate GraphSession balances spanning several observations cannot silently
become a per-network verified balance. Private balances follow the selected
note/witness contract; a public projection cannot reconstruct missing secrets.

acceptance:

- opening verifies against tip OR local full apply matches root  
- amount conservation holds for known mints/burns (PLUMB laws)

### 4.2 send

Input binds `(from_neuron, network, to_holder, token, amount, optional memo/particle)`
and a stable request identity. Admission captures the attachment revision,
current grant, exact operation and selected proof/executor profile. A prog
supplies its prog/invocation context without introducing another signer.

steps (normative order):

1. tip ready: grade 4 on light; full/partial tip verified and current
2. select inputs / notes owned by `from_neuron` with witnesses at tip  
3. build Intent: one or more PLUMB `pay` ops (change outputs allowed)  
4. prove with [[zheng]]: auth, conservation, fresh nullifiers, well-formed links  
5. seal to signal; gossip via [[radio]] / [[foculus]]  
6. local UI → grade 1 on network-valid $\sigma$  
7. on clock A final at tip → grade 2; update sigma; optional payer sense echo  

rejection: any peer MUST drop signals failing $\sigma$ or nullifier already in $N$.

Before dispatch, check the captured subject/network and current grant again.
Selection changes preserve the prepared action. Revocation blocks new dispatch;
retained requests, notes, reservations and unknown outcomes remain available
for read-only reconciliation. Identical retries use the original complete bytes
only under the endpoint's declared idempotency contract. A changed subject,
destination or payment is a new explicitly authorized action.

Private note preparation and pending-spend evidence must be durable before
publication. A lost reply cannot release the same input for a second spend.
Recovery binds observed inclusion/nullifier/change to the original request,
subject and network before updating spendability.

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

intent particle for notifications: `intent/notify` (see cyb-core). The event
envelope MUST bind `reason` to signal/link ID and retain subject/network plus
the evidence behind its grade. Same-address text on a foreign network is a
separate domain-qualified reference until its profile proves the association.

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
