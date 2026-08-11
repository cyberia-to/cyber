---
tags: cyber, specs, light client, money, soft3, zheng, bbg
crystal-type: spec
crystal-domain: cyber
alias: light money, light client money, fold tip money
status: draft
---

# light money

normative integration of the light client tip path with the [[specs/money-loop\|money loop]]. implements "how a thin device is sure it sent and received" with light client **in scope**.

protocol join detail: [[structural sync]] § light client protocol.  
header narrative: [[cyber/light\|light client]].  
folding: [[zheng]] accumulator / recursion.  
clocks: [[latency targets]], [[specs/money-loop\|money-loop]] §2.

---

## 1. thesis

on a light client, money certainty is:

$$\text{sure(send/receive)} \equiv \text{grade 4 (tip)} \land \text{grade 2 (finality at tip)} \land \text{valid opening/proof}$$

genesis fold is not optional polish. it is how grade 4 is obtained without replaying history. without grade 4, openings are not money-grade.

---

## 2. tip object

```
Tip = {
  height:          u64
  bbg_root:        Commitment      // BBG_root
  folding_acc:     Accumulator     // HyperNova / universal acc
  header_hash:     Hash            // optional link to header spine
}
```

grade 4 when:

1. `decide(folding_acc)` verifies, binding history to `bbg_root` at `height`, OR  
2. implementation-defined stronger full verify (full node)  

steady state: each new block/header updates `folding_acc` with O(1) fold ops and advances `bbg_root`. MUST NOT require re-decide from genesis every tip.

---

## 3. join algorithm (normative)

```
light_join(peer):
  1. ck ← peer.checkpoint()
       // (bbg_root, folding_acc, height) ~232–240 B class
  2. π ← peer.decider_proof(ck.folding_acc)   // or embedded
  3. assert verify_decide(π, ck.bbg_root, ck.height)
  4. tip ← Tip from ck
  5. grade4 ← true
  6. for each namespace in interest_set:
         open_and_verify(namespace, tip)
  7. return tip
```

failure modes:

| failure | action |
|---|---|
| decide fails | discard peer checkpoint; try other peers; do not set grade 4 |
| open fails | do not update that balance; tip may still be grade 4 |
| no peers | remain grade 4-false; money UI disabled |

acceptance: verify compute ≪ 1 s; join latency dominated by download RTT.

---

## 4. balance on light

```
balance(N, tip):
  assert tip.grade4
  (amounts, open_π) ← peer.open_coins(N, tip.bbg_root)
  assert Lens.verify(tip.bbg_root, key(N), amounts, open_π)
  return amounts
```

MUST bind open to current tip root. MUST refresh or invalidate on TipAdvanced.

private note model (when mutator / private box enabled): open commitments and local secrets; same tip binding.

---

## 5. receive on light

```
on_credit_hint(hint):  // push or poll
  assert tip.grade4
  (credit, open_π, finality_π) ← resolve(hint)
  assert Lens.verify(tip.bbg_root, …, open_π)
  assert finality_valid(credit.signal_id, tip, finality_π)  // clock A evidence
  apply_local_note_if_mine(credit)
  emit TransferIn | RewardCredited
  sigma_refresh()
```

rules:

- R1: push hints are untrusted until open + finality verify  
- R2: finality evidence MUST be checkable without full tri-kernel (provable consensus / completeness proofs — see [[provable consensus]], [[vec]])  
- R3: NOTIFY only after R1–R2 pass  

if finality proof format for thin clients is not yet available in code, implementation MUST still specify the interface and fail closed (no grade 2) rather than trust peer "it's final".

---

## 6. send on light

```
send(from, to, token, amount):
  assert tip.grade4
  witnesses ← peer.open_membership(inputs, tip.bbg_root)
  assert all witnesses verify
  intent ← PLUMB.pay(...)
  σ ← zheng.prove(intent, secrets, witnesses)
  signal ← seal(intent, σ)
  broadcast(signal)
  await finality_valid(signal, tip', finality_π)  // tip may advance
  assert grade 2
  update local notes (nullify inputs, store outputs)
```

rules:

- R4: witnesses MUST verify at the tip used in the proof  
- R5: if tip moves between prove and include, re-prove or use protocol-stable witness rules (implementation MUST document)  
- R6: respend of change outputs only after grade 2  

---

## 7. multi-payee and settle on light

pay legs: same as receive (open + finality at tip).

attribution mints (clock B): after settle commits mint into state at tip height $h_B$, open balance at tip ≥ $h_B$ + depth $d$ policy as in parameters. light does not run settlement lottery; it only verifies post-state openings and any settle receipt proofs the network publishes.

---

## 8. tip follow

```
on_header(h):
  assert links(prev_tip, h)
  folding_acc' ← fold(folding_acc, h.block_proof_instance)
  // optional: periodic decide snapshot for audit
  tip ← update(h, folding_acc')
  invalidate openings older than tip unless still valid under new root
  refresh watched namespaces
```

MUST keep fold current for money-grade operation. lagging tip → pending only, not grade 2 against stale root for new claims.

---

## 9. storage budget (target)

| item | size class |
|---|---|
| tip + acc | ~0.2–1 KB |
| keys / note secrets | O(owned notes) |
| cached openings | O(watched keys) |
| optional header window | product choice |

full header chain download without fold is non-goals for money path (fold replaces it).

---

## 10. interfaces (minimum)

```
trait LightTip {
  fn join(peers) -> Result<Tip>
  fn advance(header_or_block_acc) -> Result<Tip>
  fn grade4(&self) -> bool
}

trait MoneyOpenings {
  fn open_balance(neuron, tip) -> Result<(Balances, Proof)>
  fn open_note_witness(note_id, tip) -> Result<Witness>
  fn verify_finality(signal_id, tip, proof) -> bool
}

trait MoneySubmit {
  fn prove_and_send(intent, secrets, witnesses) -> Result<SignalId>
}
```

owners: tip fold → foculus + zheng; openings → bbg + cybergraph RPC; prove → zheng + tok; UI → cyb.

---

## 11. conformance tests

- [ ] join from empty disk with only checkpoint + decide + open balance  
- [ ] reject wrong acc (decide fails) → no grade 4  
- [ ] reject valid tip but forged balance open  
- [ ] send → wait finality proof → grade 2 → respend change  
- [ ] receive push without proof → no NOTIFY  
- [ ] receive with open + finality → NOTIFY + balance  
- [ ] multi-payee: two opens both verify  
- [ ] tip advance invalidates stale opens or re-verifies  

---

## 12. dependency on component specs

| concern | normative home |
|---|---|
| fold / decide | [[zheng]] accumulator, recursion |
| checkpoint fields | [[structural sync]], [[cyber/light\|light]] |
| Lens open | [[bbg]], lens |
| finality thin proof | [[provable consensus]], [[vec]] |
| pay validity | [[tok]] PLUMB + zheng circuits |
| events | [[specs/money-loop\|money-loop]] |

if thin finality proof is incomplete in code, track as blocker for light grade 2; do not substitute trust.

---

see [[specs/node-modes\|node-modes]], [[specs/component-ownership\|component-ownership]], [[latency targets]].

---

discover all [[concepts]]
