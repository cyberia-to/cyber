---
tags: cyberia, system, erp
crystal-type: pattern
crystal-domain: cyberia
alias: cyberia erp, ledger model
---
# cyberia system

minimal ERP. one foundation, one framework, six trait categories, one operational layer above.

---

## architecture

```
view                                                ← financial statements derived
─────────────────────────────────────────────────────
patterns (product · process · project · commitmentguard · ...)
─────────────────────────────────────────────────────
intent  ·  schedule  ·  workflow  ·  template       ← operational orchestration
─────────────────────────────────────────────────────
traits — what each token IS, CAN, MUST, KNOWS, SENSES, IS WITH
   skills · duties · senses · bonds · memory
─────────────────────────────────────────────────────
PLUMB                                                ← operation framework
   pay · lock · update · mint · burn  +  hooks
─────────────────────────────────────────────────────
nature                                               ← two token primitives
   TSP-1 (fungible coin)   ·   TSP-2 (non-fungible card)
```

bottom three layers are [[gold standard]] (PLUMB, TSP-1, TSP-2, [[token traits]]). top three are this ERP. the seam is well-defined: ERP composes trait declarations and operations into business semantics.

---

## one foundation: two token natures

every entity in the system is a token. there are no accounts, no assets, no separate registries. two natures cover everything:

| nature | conservation law | what it represents |
|--------|----------------|------------------|
| TSP-1 (Coin) | Σ balances = supply | divisible value: money, goods by unit, time-fungible items |
| TSP-2 (Card) | owner_count(id) = 1 | unique entity: person, DAO, place, slot, contract |

a person is a Card. a DAO is a Card. a banya at 14:00 is a Card. a kilogram of chicken is balance of a Coin held by a Card. IDR is a Coin. a building is a Card. a relationship to a builder is a Bond between two Cards.

the dichotomy account-vs-asset disappears. everything composes from these two.

---

## PLUMB — how tokens operate

every token operation is one of five:

| operation | what it does |
|-----------|------------|
| pay | transfer Coin balance from one Card to another |
| lock | constrain a token (install a Duty, set a floor, freeze) |
| update | change config (rotate authority, install/remove traits) |
| mint | create new token instance |
| burn | destroy token instance |

each operation has hook slots where traits install. a pay operation triggers `pay_hook` which may run logic from any installed trait. operations compose through proof — multiple operations across multiple tokens commit atomically.

PLUMB is the universal verb layer. the entire ERP reduces to sequences of these five operations.

---

## traits — what makes a token specific

every token carries a trait profile in five categories. four are declared, one accumulates.

**skills** — what the token CAN do (capabilities, outward, generate value)
- governance, lending, staking, vault, bridging, batch ops, burn-to-redeem, sword evaluator, liquidity

**duties** — what the token MUST do or CANNOT do (constraints, inward, build trust)
- compliance, soulbound, transfer limits, fee-on-transfer, kyc gate, controller gate, supply cap, timelock, commitmentguard

**senses** — what the token can PERCEIVE (external information channels)
- oracle pricing, proven price, cross-chain relay, event monitor, timestamp

**bonds** — who the token is WITH (typed relationships to other tokens)
- royalties, delegation, multisig, subscription, vesting, controller, contract position

**memory** — what the token KNOWS (accumulates from operations, not declared)
- the token's slice of the ledger plus all skill-state-trees it touches

each trait installs through a PLUMB hook. the category tells you the trait's intent — capability vs constraint vs perception vs connection — without scanning every hook individually.

---

## composition algebra

the categories compose differently:

| category | composition rule | example |
|----------|----------------|---------|
| skills | additive (free) | Liquidity + Lending + Governance — all simultaneously |
| duties | conjunctive (AND) | Compliance + TransferLimits — all must hold |
| senses | disjunctive (OR) | OraclePricing + ProvenPrice — either provides info |
| bonds | structural (per axis) | Royalties + Delegation — independent dimensions |

cross-category conflicts are visible at install time: Soulbound (Duty: never transfer) + Liquidity (Skill: enable swaps) is a contradiction. proof composition fails because both proofs cannot be simultaneously valid. the type system catches this before deployment.

---

## the accounting identity

for every token, its trait profile produces a balance sheet in real time:

```
skills + senses + bonds(receivable)  =  duties + bonds(payable) + nature
```

every receivable on one side is a payable on the other — the double-entry principle expressed at the trait level. memory accumulates retained earnings.

balance sheet, P&L, cash flow are derivable as views from the trait profile and ledger slice — no separate accounting layer needed.

---

## intent — proof in progress

an intent is a planned set of operations, not yet committed. it is the only mutable thing in the system.

```
Intent(id, owner, workflow, state, operations[], metadata)
```

operations inside an intent are pending until the workflow says otherwise. pending operations reserve their inputs (lock the slots, lock the balances). on a workflow transition, designated operations commit to the ledger and become immutable. on cancellation, pending operations drop, locks release.

a booking, a procurement, a maintenance ticket, a complaint, a hire — every business event is an intent.

---

## workflow — declared state machines

a workflow is a state machine attached to an intent type. transitions specify:

- source and target states
- which operations commit on this transition
- who is allowed to perform it (signature, multisig, vote threshold, automatic, time-based)
- optional script logic to run during transition

```
workflow: hospitality_booking
states: draft → reserved → paid → consumed
                     ↘ cancelled

transitions:
  draft → reserved:    requires_signature(guest)
  reserved → paid:     requires_payment(deposit)
  paid → consumed:     automatic at slot.start
  any → cancelled:     guest_or_staff_signature
```

authorization is a property of transitions. multiple proposal modules in a single DAO core map onto multiple workflow declarations.

---

## template — parameterized operation patterns

a template is a declared recipe of operations parameterized by inputs:

```
Template: nasi_goreng(portions: int)
operations:
  burn  rice    from kitchen   200g × portions
  burn  chicken from kitchen   100g × portions
  burn  egg     from kitchen     1  × portions
  mint  dish    to ready-tray    1  × portions
```

intents are constructed by invoking templates with arguments, or assembled manually. templates are reusable, intents are unique runtime instances.

---

## schedule — when intents are born

a schedule fires templates at declared moments:

- monthly payroll on day 25 → instantiate `payroll(employee, hours)` for each employee
- weekly maintenance on tuesday → instantiate `maintenance(asset)` for due-soon items
- end of stay at checkout time → instantiate `final_invoice(booking)`

schedules close the recurring loop. business events that recur live in declarations, not in external cron jobs.

---

## patterns — named compositions

universal business concepts assembled from primitives. they are the standard library — first-class concepts in the model, but technically composed from layers below.

`Product` — what is offered to a customer:

```
Product = token(s) + pricing(skill) + sale_template + marketing_metadata
```

a banya session, a glamping night, a nasi goreng plate are products. a product is a Card with a Skill that produces sale operations parameterized by price.

`Process` — a recurring or ad-hoc operation:

```
Process = workflow + (optional) schedule + (optional) templates
```

morning briefing, harvest cycle, payroll close are processes. each declares its workflow and trigger.

`Project` — a bounded effort with budget and team:

```
Project = Card(container) + Coin balance(budget) + Bonds(team) + sub_intents + workflow
```

burn.city, banya repair, booking app build are projects. the container Card holds the budget Coin balance. team relationships are Bonds. work items are sub-intents.

`CommitmentGuard` — assurance without escrow:

```
CommitmentGuard = Duty installed on a Card's pay_hook
  rejects any pay operation that drops balance below committed floor
  removable only by beneficiary's signature
```

build a house, hire a vendor, commit to a partnership without locking capital. the Card's balance stays usable for governance, lending, staking — only operations that violate the floor fail to produce a valid proof.

new patterns join over time (subscription, membership, partnership, campaign). the kernel stays constant — patterns live in declarations.

---

## view — derived projections

views never mutate. they read the ledger plus trait declarations and produce projections.

| view | shows | from |
|------|-------|------|
| balance | Coin holdings of a Card per token type | ledger |
| calendar | which Cards hold time-bound Cards in which windows | ledger + duties |
| inventory | Coin balances grouped by Coin class | ledger |
| flow | net Coin movement across a Card over a period | ledger |
| memory | full operational history of an entity | ledger filtered to entity |
| balance_sheet | skills + senses + bonds = duties + bonds + nature | traits + ledger |
| profit_loss | revenue from skills − expenses from duties | ledger filtered |
| cash_flow | operating + investing + financing flows | ledger filtered |
| kanban | intents grouped by workflow state | intents |
| graph | Cards and their Bonds | trait profiles |

every entity automatically produces its own financial statements — no separate accounting layer.

---

## conservation

every pay operation has exactly one source and one destination. for every quantity that leaves a Card, the same quantity enters another. mints and burns are explicit creation/destruction operations between designated source and sink Cards. the conservation law holds across the full graph by construction.

```
Σ holdings(coin) = mint_total(coin) − burn_total(coin) = supply(coin)
```

violation is impossible because the proof system rejects any operation that breaks it.

---

## what each layer adds

| layer | adds | example |
|-------|-----|--------|
| nature | foundational types | "this is a unique slot vs divisible currency" |
| PLUMB | atomic operations + hooks | "transfer Coin · install Duty · mint Card" |
| traits | classification of behavior | "this Card has Lending skill and Compliance duty" |
| workflow | state machines for processes | "booking goes draft → reserved → paid" |
| template | parameterized operation recipes | "1 nasi goreng = these 4 burns + 1 mint" |
| intent | mutable runtime instance | "intent #4521: book banya for guest X" |
| schedule | recurring intent generation | "every 25th, instantiate payroll" |
| patterns | named compositions of all the above | "this Card configured this way is a Product" |
| view | derived projections | "balance sheet · P&L · cash flow · calendar" |

the kernel is the bottom three layers. the ERP is the top six. domain knowledge lives in declarations. nothing is hardcoded.

---

## blockchain alignment

this model maps directly onto a proof-based settlement substrate ([[trident]] / [[gold standard]]):

- coin/card naturalizes to TSP-1 / TSP-2
- traits are PLUMB hooks classified by intent
- intent is a transaction-in-progress (proofs being assembled)
- workflow is the orchestration of proof composition
- conservation is enforced by circuit, not application code

what the ERP layer adds beyond settlement: business processes, schedules, patterns, projections — the operational orchestration that turns settlement primitives into a working business.

---

see [[cyberia/midao/operations]] for the human and process layer above this data layer.
