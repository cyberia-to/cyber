---
tags: cyber, research, article, cip
crystal-type: pattern
crystal-domain: cyber
status: proposal
date: 2026-09-08
alias: oikos, one token one chain, sovereign mints, oikos architecture
---
# oikos — one token, one chain

Status: proposal. The home-book economic model and its settlement protocols
require explicit protocol acceptance.

Oikos proposes one home book for one token: the complete ledger of its issuer's
obligations. One non-fungible name roots the book; nested sub-names form its chart
of accounts. Οἶκος is the household: each book accounts for value at home.

The [domain model](../specs/domain-ladder.md) separates that book from its issuer,
its execution and its physical storage. An issuer's authenticated neuron or
explicit collective authority governs issuance. Progs execute the book's rules;
node services validate and serve them; [[shards]] partition graph state. One
issuer may govern several books, and one book may use several progs or services.
The book name supplies a ledger identity. Its governance policy determines who
can act; a name, program ID or shard root alone supplies no signing authority.

Two rules define the proposed economics:

1. Each home book accounts for exactly one token. Issuing a second token creates
   a separately named book with complete and separately verifiable obligations.
2. Every balance change settles in that token's home book. Cross-book trading
   specifies conditional reassignment at home against evidence from the other
   book, with explicit authority, freshness, expiry and conservation rules.

Applied to [[cyber]], the proposal gives [[cyber/$CYB|$CYB]] a home book rooted by
the root name. Routing and registration link other books' names and state roots
into the [[cybergraph]]. These rules define a target economic profile. Existing
foreign networks retain their actual token, address and contract rules; adapting
an interface preserves their original wire data and identifies their profiles.

## required foundations

The proposal depends on four foundations:

1. Affordable verification. Conditions require a supported verifier for the
   counterparty's consensus and state. Recursive [[zheng]] proofs can compress
   verification work; their trusted starting point, validator assumptions,
   finality rules and availability obligations remain part of the profile.
2. Domain finality. [[foculus]] must specify when a book can settle locally and
   when cross-domain dependencies require waiting. A partition retains pending
   obligations and expires conditions according to their rules.
3. Complete books. Issuance, burning, spending, locked obligations, fees and
   outstanding conditions must reconcile under each book's conservation law.
   Multiple programs or storage partitions cannot fragment that accounting.
4. Authenticated registration. A name/root link gives discoverability and an
   anchor under its own [[cyb/parts/state|state]] tier. Verification of the
   book's validity still requires its authority, execution and finality evidence.

## intended benefits and limits

- Custody stays at home: the protocol settles conditional balances instead of
  holding transported copies. Issuer policy, verifier and condition-code risks
  still require their own treatment.
- Books have local fee and governance rules. Cross-book matching still needs
  explicit ordering and incentive analysis, including free options and MEV.
- A complete named book makes proof coverage and accounting easier to state.
  Proof size, client verification cost and growth limits require measurement;
  [[cyb/parts/sigma|sigma]] displays the evidence actually available for each book.
- A [[tokens/plumb|plumb]] factory can create a named book, its authority policy
  and its program as one authorized operation. Registration alone cannot prove
  valid issuance or fund the new ledger.

## the two settlement protocols

These protocols must be specified before oikos is law:

1. Trade without a free option. In a two-party conditional swap the second
   committer can gain an option. Candidate epoch-batched matching binds both legs
   to the same [[foculus]] beacon and gives every condition an explicit deadline.
   Order commitments may be hosted in a routing domain; balances settle at home.
   Delivery evidence must bind the exact leg before commitment. Activation,
   rollback, timeout, replay protection and concurrent claims need formal rules.
2. Freshness. A condition specifies event E, source book and network, evidence
   tier at least τ, authenticated checkpoint and maximum age t. The
   [[cyb/parts/state|state]] query coordinates become part of the condition
   language. A halted counterparty eventually ceases to satisfy freshness;
   expiry releases or resolves obligations under the named home-book policy.

## integration obligations

| Owner | Required contract |
|---|---|
| [[3c]] | Typed read/write/trade operations; exact evidence, authority, delivery and settlement meanings |
| [[network]] · [[communication]] | Authenticated book/endpoint routes and freshness economics; delivery bound to the exact message |
| [[tokens/plumb|plumb]] | Authorized book creation with issuer policy, program and complete initial obligations |
| [[cyber/$CYB|$CYB]] · [[litepaper]] · [[whitepaper]] | Consistent home-book economics, with the proposal's settlement assumptions explicit |
| [[cyb/parts/state|state]] | Evidence tier, network, root, coverage and age in every condition |
| [[cyb/parts/sigma|sigma]] · [[cyb/parts/vault|vault]] | Portfolio observations retain their tier; spending uses the captured issuer/owner binding and scoped custody |
| [[aos/teleport|teleport]] · [[aos/hub|hub]] | Condition composition and explicit foreign adapters with their actual trust and wire profiles |
| [[neurons]] and progs | Issuer/owner authority separate from program work IDs; reservations and unknown effects survive recovery |
| [[foculus]] | Epoch matching, domain finality, partition and timeout rules |
| [[zheng]] · [[bbg]] | Verifier profiles, authenticated book state and availability; conservation across shard handoff |
| [[cyberia/protocol/century-index|century index]] | Explicit book, issuer policy and network if CX adopts this economic profile |

## relation to graph partitioning

[[spectral cell division]] studies how graph regions divide. Creating a book is
an issuer-authorized economic operation; splitting a [[shard]] is a protocol
operation on state coverage and serving responsibility. They may happen
independently. A partition change must retain the book's obligations, spend
uniqueness, pending conditions and authenticated history. The historical
[[cell|ledger-cell]] name remains a compatibility reference.

discover all [[concepts]]
