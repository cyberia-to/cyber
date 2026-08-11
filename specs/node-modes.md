---
tags: cyber, specs, soft3, cell, light client, full node
crystal-type: spec
crystal-domain: cyber
alias: node modes, full node cell light client
status: draft
---

# node modes

three modes of participation in the cyber network. all are in scope. they share roots, nullifiers, and finality rules; they differ in storage and how tip trust is obtained.

money loop requirements: [[specs/money-loop\|money-loop]].  
light money detail: [[specs/light-money\|light-money]].

---

## 1. modes

```
full node                 cell (cyb)                    light client
hold / replay             apply my                      headers + fold
all signals               namespaces                    decide(acc)
tri-kernel locally        local notes + sigma           openings only
size: unbounded           size: O(my slice)             size: ~constant
```

| mode | tip trust | private notes | run φ* | produce block proofs | typical device |
|---|---|---|---|---|---|
| full | own apply of history | yes | yes | yes (validators) | server / desktop |
| cell | apply my slice + completeness; MAY embed light fold | yes | optional / domain-local | no | phone / laptop cyb |
| light | clock C: decide + fold | secrets only; witnesses via open | no | no | thin phone / embed |

production cyb default: **cell that embeds light tip** (grade 4 via fold, plus local note store and apply of signals that touch the owner). pure light is the thinnest extreme; pure full is the fattest.

---

## 2. required capabilities by mode

### 2.1 full node

MUST:

- store or recompute state sufficient to serve BBG openings and history  
- verify all signal $\sigma$ it accepts  
- maintain nullifier set consistency with foculus rules  
- compute or verify φ* domain finality as required by its role  
- serve Lens openings and completeness proofs to peers  
- produce folding accumulator contributions if participating in consensus/settlement  

MAY:

- prune archival data if openings and acc still served  

### 2.2 cell (cyb)

MUST:

- hold neuron secrets and owned notes  
- submit sends (prove Intents)  
- obtain grade-4 tip (fold preferred; continuous completeness minimum)  
- apply or open all signals that credit/debit local neuron  
- emit money-loop events to sense/sigma  
- verify peer openings against tip  

MUST NOT:

- mark money final without grade 2 at trusted tip  

SHOULD:

- multi-device local sync (CRDT) for same identity (structural-sync local merge)  

### 2.3 light client

MUST:

- implement clock C join: checkpoint → `decide(folding_acc)` → tip  
- fold each new tip update O(1)  
- verify Lens openings against tip for balance and receive  
- verify own send proofs before broadcast; verify finality evidence for own spends at tip  
- refuse money-grade state if grade 4 missing  

MUST NOT:

- re-execute full history  
- run full tri-kernel as a requirement of money  
- accept balance/receive without tip-bound proof  

MAY:

- cache headers and openings  
- subscribe to push hints (still MUST verify)  

---

## 3. matrix: money loop × mode

| capability | full | cell | light |
|---|---|---|---|
| balance | local | apply or open | open |
| send | prove + gossip | prove + gossip | prove + gossip (witnesses from peers) |
| receive detect | apply | apply / open | open + event |
| receive NOTIFY | yes | yes | yes after verify |
| multi-payee reward | yes | yes | yes after open |
| cold start | sync/replay | sync slice + fold | **fold only** |
| grade 4 | implicit | fold or completeness | **fold** |

---

## 4. promotion and demotion

| transition | rule |
|---|---|
| light → cell | add private note store + apply pipeline; keep fold tip |
| cell → full | add full graph / validator duties; not required for money UX |
| full → light | drop local graph; retain keys; re-join via decide |

losing grade 4 (cannot fold, no completeness) → money UI MUST drop to non-money-grade (read-only warning) until tip restored.

---

## 5. conformance

- [ ] light join works offline-of-history: only acc + decide + opens  
- [ ] cell send/receive works with embedded fold tip  
- [ ] full node can serve the openings light needs  
- [ ] mode flag visible in diagnostics (for support, not for trust)  

---

see [[specs/light-money\|light-money]], [[structural sync]], [[cyber/light\|light client]], [[cell]].

---

discover all [[concepts]]
