---
tags: cyber, specs, soft3, partial node, light client, full node
crystal-type: spec
crystal-domain: cyber
alias: node modes, full partial light client
status: draft
---

# node modes

Full, partial and light describe participation in a graph network: retained
state, validation duties and the source of trusted tips. The former cell mode
is named partial. [Domain roles](domain-ladder.md) and
[cyb architecture](../../cyb/specs/architecture.md) define neuron as the subject,
prog as executable work and GraphSession as a multi-neuron local graph host.
Mode selection is independent of the number of attached neurons or progs.

This document specifies target network profiles. The current local native
adapter supplies durable endpoint acceptance and authenticated submission when
enabled; its local height, mirrored history and HTTP receipts alone do not
satisfy the finality or folded-tip requirements below. Implementations advertise
only profiles supported end to end. See [[specs/node-product]].

Money requirements: [[specs/money-loop]]; folded-tip detail:
[[specs/light-money]].

## 1. modes

| mode | retained state | tip trust | private wallet state | computation |
|---|---|---|---|---|
| full | State/history sufficient to validate the complete selected domain and serve openings | Own verified application under the network's consensus rules | For explicitly controlled subjects | Tri-kernel and block proofs as required by the selected validator role |
| partial | Selected graph slices, relevant notes and authenticated coverage | Verified slice application/completeness anchored to a trusted tip; may embed the light fold | Notes and witnesses for attached controlled subjects | Optional domain-local computation |
| light | Checkpoint, accumulator, headers/openings and local secrets as needed | Clock C: verified `decide` and folded tip updates | Local secrets; witnesses obtained through verified openings | Opening/send verification; full graph computation optional |

The target cyb default combines partial state with a verified light tip: local
notes and relevant graph updates, plus clock-C trust. Storage scales with the
chosen slice; pure light minimizes retained graph state. Observation-only
attachments require public references and verified observations. Control uses
explicit ward grants and vault custody; a mode label grants neither.

## 2. required capabilities

### 2.1 full node

A full implementation MUST:

- retain or recompute state sufficient to serve BBG openings and history;
- verify every accepted signal and required proof under its network profile;
- maintain nullifier consistency with foculus ordering and conflict rules;
- compute or verify φ* domain finality as required by its role;
- serve authenticated Lens openings and required completeness evidence;
- produce accumulator contributions when its consensus/settlement role requires them.

Archival pruning MAY remove data only while the declared opening, history and
accumulator availability contract remains satisfied. Local execution of all
received data establishes network trust only when join, coverage and consensus
verification also satisfy the selected profile.

### 2.2 partial node

A partial implementation MUST:

- retain the selected slice and identify its coverage relative to authenticated tips;
- retain owned notes and recovery state for subjects it controls;
- obtain grade-4 tip trust through fold or the specified continuous completeness
  and validated-history equivalent;
- apply or open every relevant credit, debit and spent-status change for its
  selected subjects, including any hidden-note discovery required by the profile;
- prepare/prove sends through current subject/network authority and verify peers'
  openings against the trusted tip;
- emit subject- and network-bound observations to sense/sigma;
- grant money-grade labels and respend only after the required finality grade.

Multi-device state synchronization SHOULD preserve notes, observations and
continuations with the supported merge policy. A CRDT merge transports data;
dispatch requires the current writer fence, binding and grant. A second device
cannot infer signing permission or an additional effect attempt from synchronized
state alone.

### 2.3 light client

A light implementation MUST:

- join clock C through an authenticated checkpoint and valid `decide(folding_acc)`;
- verify and fold each new tip update under the declared constant-work profile;
- verify balance, receive and spent-status openings against that tip;
- verify its own send proof before broadcast and finality evidence before respend;
- retain local custody and private recovery state according to its attachment mode;
- keep money-grade operations unavailable while grade-4 trust is missing.

Full history replay and full tri-kernel computation are optional additions to
this mode. Cached headers, openings and push hints are useful inputs whose
verification duties remain unchanged.

## 3. money loop by mode

| capability | full | partial | light |
|---|---|---|---|
| balance | Verified local state | Complete relevant apply or tip-bound opening | Tip-bound opening |
| send | Prove + gossip | Prove + gossip | Prove + gossip using verified witnesses |
| receive detection | Apply | Complete relevant apply / open | Open + verified event |
| receive notification | After required finality | After required finality | After verified opening and required finality |
| multi-payee reward | Validate all legs | Apply/open each relevant leg | Open each relevant leg |
| cold start | Verified join + replay/checkpoint | Verified slice + trusted tip | Authenticated accumulator + decide + required openings |
| grade 4 | Verified full-history/consensus equivalent | Fold or declared completeness/history equivalent | Fold |

## 4. promotion and demotion

| transition | preserved state and added duties |
|---|---|
| light → partial | Retain trusted tip and custody; add slice application, completeness and note storage |
| partial → full | Retain subjects/notes; add complete selected-domain state and full validation duties |
| full → partial/light | Retain custody, notes, unresolved effects and recovery evidence; establish the destination mode's trusted-tip path before relying on it |

Changing mode preserves NeuronId, prog/invocation identities, pending author and
network, nonce claims and budgets. Losing clock-C trust drops money views to
explicit unverified/read-only observations until trust is restored. Detachment,
device movement and mode changes preserve history and unknown outcomes.

## 5. conformance

- [ ] Light join verifies without downloading history, using accumulator, decide and opens.
- [ ] Partial send/receive verifies its embedded tip and complete relevant slice.
- [ ] Full nodes serve the openings and availability promised to thinner clients.
- [ ] All modes reject wrong-network proofs, stale/nullified spends and incomplete coverage.
- [ ] Promotion/demotion preserve notes, subject bindings and unresolved work across restart.
- [ ] Diagnostics report actual mode, proof profile, trusted-tip state and supported capabilities separately.

See [[specs/light-money]], [[structural sync]] and [[specs/domain-ladder]].

discover all [[concepts]]
