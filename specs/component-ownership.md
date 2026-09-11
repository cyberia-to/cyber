---
tags: cyber, specs, soft3, architecture
crystal-type: spec
crystal-domain: cyber
alias: component ownership, money ownership matrix
status: draft
---

# component ownership

who implements what for the cyber money loop + light client. integration contracts live in [[cyber]]/specs; algorithms live in component repos.

The executable composition and current runtime capability boundary live in
[[specs/node-product]]. Local cyb integration is specified in
[[specs/cyb-node]]. The [component implementation report](../audit/component-implementation.md)
records work-package status, code locations and prior checks. The
[node readiness audit](../audit/node-readiness.md) records executable evidence.

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

## 3. change control

- breaking change to clocks, grades, or tip object → version bump in this directory
- parameter number changes ($d$, epoch lengths) → [[foculus parameters]] only
- explanation prose → foculus `docs/explanation/latency-targets.md`

---

discover all [[concepts]]
