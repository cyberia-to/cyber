---
tags: cyber, cybernomics, draft, research
crystal-type: entity
crystal-domain: cyber
alias: token traits
---

# Token Traits

## A Complete Vocabulary for Token Identity in Provable Systems

Version: 0.1-draft | Date: March 3, 2026

---

## Abstract

Every [[token]] system flattens the rich structure of economic entities into a single mechanism — "smart contracts" (Ethereum), "programs" (Solana), "skills" (Gold Standard v0.1). This flattening hides distinctions that matter: the difference between what an entity CAN do and what it MUST do, between what it knows and what it perceives, between what it is and who it's connected to.

This paper introduces six traits for tokens on provable blockchains. The traits emerge from the intersection of two lenses — double-entry accounting (balance sheet, P&L, cash flow) and personality theory (what makes an agent an agent). The resulting vocabulary is not decorative. It is structural: a token's traits determine its operational capabilities, its trust profile, its composability surface, and its economic behavior. The vocabulary is designed so that when tokens interact — trade, lend, govern, contract — the interactions produce the same statements (balance sheet, income, cash flow) that every company and every person already expects from their economic life.

**Depends on:** [Gold Standard](https://github.com/cyberia-to/trident/blob/master/docs/explanation/gold-standard.md) ([[PLUMB]], TSP-1, TSP-2), [Skill Library](https://github.com/cyberia-to/trident/blob/master/docs/explanation/skill-library.md)

---

## 1. The Problem With "Skills"

The Gold Standard defines a skill as "a composable package of hooks, optional state, and config that teaches a token a new behavior." Twenty-three skills are specified: Liquidity, [[Governance]], Lending, Compliance, Soulbound, [[Oracle]] Pricing, Delegation, and so on.

But look at what Compliance actually does. It restricts who can send or receive via [[merklezation|Merkle]] inclusion proofs. It does not teach the token a new behavior. It removes a behavior. A token with Compliance is LESS free than one without. Calling it a "skill" is like calling a prison sentence a "career opportunity."

And look at Oracle Pricing (COMPASS). It doesn't enable the token to DO anything new in terms of on-chain actions. It enables the token to PERCEIVE something — market prices. A blind person who gains sight hasn't learned a skill. They've gained a sense.

And Delegation creates a relationship between an owner and a delegate. Royalties create a permanent link between a creator and every future holder. These are not capabilities of the token itself — they are connections between tokens.

One word cannot carry all this meaning. When everything is a "skill," you lose the ability to ask precise questions:

- "What can this token do?" (its capabilities)
- "What constraints bind this token?" (its obligations)
- "What data can this token access?" (its perception)
- "What is this token connected to?" (its relationships)
- "What has this token experienced?" (its history)

These are different questions requiring different answers. A vocabulary that collapses them into one word forces you to scan all 23 "skills" and mentally re-classify each time. The vocabulary should do this work for you.

---

## 2. Two Lenses, One Structure

### 2.1 The Accounting Lens

Every entity that participates in economic life — a person, a company, a trust, a DAO — has its existence fully described by three financial statements:

**Balance Sheet** captures what IS, at a single point in time. Assets (what you have and what you can do), liabilities (what you owe and what constrains you), and equity (your fundamental nature — assets minus liabilities). The balance sheet always balances: Assets = Liabilities + Equity.

**Profit & Loss** captures what CHANGED over a period. Revenue (value that flowed in from exercising capabilities) minus expenses (value that flowed out from honoring obligations) equals net income (did the entity grow or shrink). P&L is the dynamic counterpart to the static balance sheet.

**Cash Flow** captures what MOVED in practice. Operating cash flow (from core activity), investing cash flow (from acquiring or disposing assets), financing cash flow (from raising or repaying capital). Cash flow reveals what the balance sheet and P&L obscure — an entity can be profitable on paper but illiquid in practice.

These three statements are not arbitrary conventions. They are the minimal complete description of an economic entity's state, change, and movement. Every regulatory regime, every auditing standard, every investor framework reduces to reading these three.

### 2.2 The Personality Lens

What makes an agent an agent — not just a passive container but an entity that acts in the world? Across philosophy, psychology, and common sense, six aspects recur:

**Character** — who you fundamentally are. Your nature, your type, the thing that persists when everything else changes. A kind person is still kind when they lose their job, forget their experiences, or move to a new city.

**Abilities** — what you can do. Learned, developed, exercised. A musician can play instruments. A trader can read markets. Abilities generate value when applied.

**Obligations** — what you must do or cannot violate. Promises made, debts incurred, laws accepted, vows taken. Obligations constrain freedom but build trust — a person who keeps their promises is more valuable as a counterparty than one who is unconstrained.

**Experience** — what you know from your history. Accumulated state, track record, memories. An experienced doctor is more trusted than a new graduate. The experience is not a capability (both can prescribe) but a credibility signal.

**Perception** — what you can sense about the world. Your inputs, your data channels, your awareness. A person with access to market data makes better decisions than one without, even if their analytical abilities are identical.

**Relationships** — who you are connected to. Family, partners, counterparties, dependents. Relationships define your social existence and your exposure — to opportunity and to risk.

### 2.3 The Intersection

When you overlay these two lenses, they align:

| Accounting | Personality | Direction | Question answered |
|---|---|---|---|
| Equity | Character | Intrinsic | What IS it? |
| Assets | Abilities | Outward | What CAN it do? |
| Liabilities | Obligations | Inward | What MUST it do? |
| Retained Earnings | Experience | Backward | What does it KNOW? |
| Revenue Streams | Perception | Inward | What can it SENSE? |
| Counterparties | Relationships | Between | Who is it WITH? |

This alignment is not forced. It reflects a deeper truth: accounting and personality theory are both trying to describe the same thing — the complete state of an agent in the world. One uses numbers, the other uses words. The structure is the same.

---

## 3. Six Token Traits

A token on a provable blockchain is an economic agent. It has identity, state, capabilities, constraints, inputs, and connections. Its traits are the vocabulary for describing all of these.

### 3.1 Nature — What You ARE

**Accounting analog:** Equity
**Personality analog:** Character
**Direction:** Intrinsic

Nature is the token's fundamental type — the conservation law it enforces. A TSP-1 Coin enforces `sum(balances) = supply`. A TSP-2 Card enforces `owner_count(id) = 1`. Nature does not change when you add or remove other traits. A Coin with Lending is still a Coin. A Card with Governance is still a Card.

Nature is already fully defined by the Gold Standard: two conservation laws, two standards, 10-field PLUMB leaves, five operations (Pay, Lock, Update, Mint, Burn). Nature is not an attachable trait — it is the foundation on which all other traits are installed.

Nature answers: "What kind of entity is this?" Divisible value (Coin) or unique object (Card). This determines which operations are possible, which conservation law applies, and which other traits can be attached.

**What Nature produces in statements:**

- Balance Sheet → Equity section. The token type and its conservation law.
- P&L → Not directly. Nature doesn't generate income; it defines what income means.
- Cash Flow → Not directly. Nature defines what "cash" is for this entity.

### 3.2 Skills — What You CAN Do

**Accounting analog:** Assets
**Personality analog:** Abilities
**Direction:** Outward (capability)

A skill is a composable package of hooks and optional state that enables a new behavior. Skills are positive capabilities — adding a skill means the token can do something it couldn't before. Skills generate value: swap fees from Liquidity, interest from Lending, rewards from [[Staking]], votes from Governance.

Skills are the asset side of the token's balance sheet. A token with more skills is like a company with more productive assets — more capable, more revenue-generating, more composable.

**The skills (from the existing 23):**

| Skill | What it enables | Revenue model |
|---|---|---|
| Liquidity (TIDE) | Provide and take liquidity in swaps | Strategy fees from takers |
| Governance | Vote on proposals using token balance | Influence over protocol parameters |
| Lending | Lend tokens or borrow against collateral | Interest income |
| Staking | Lock tokens to earn rewards | Staking rewards |
| Vault | Deposit assets, receive shares at exchange rate | Yield from underlying strategy |
| Bridging | Move tokens across chains via proof relay | Access to multi-chain markets |
| Batch Operations | Bundle multiple operations in one proof | Gas/proof cost savings |
| Burn-to-Redeem | Destroy one asset to claim another | Physical goods, crafting, migration |
| Sword Evaluator (new) | Evaluate financial contract ASTs | Derivative settlement |

**What Skills produce in statements:**

- Balance Sheet → Asset section. Each skill is an intangible asset of the token entity.
- P&L → Revenue line. Fees, interest, rewards earned from exercising skills.
- Cash Flow → Operating section. Inflows from core skill activity.

### 3.3 Duties — What You MUST Do

**Accounting analog:** Liabilities
**Personality analog:** Obligations
**Direction:** Inward (constraint)

A duty is a composable package of hooks that constrains behavior. Duties are negative capabilities — adding a duty means the token CANNOT do something it could before, or MUST do something it didn't have to. Duties restrict proof generation: any transaction that would violate a duty fails to produce a valid proof.

Duties are the liability side of the token's balance sheet. But critically, duties that constrain you build trust in others. A person who vows to keep a balance above a threshold is more trustworthy to a builder than one who is free to spend everything. The constraint is your liability; the trust it creates is your counterparty's asset. This is the accounting identity in action: your liabilities are someone else's assets.

**The duties (from the existing 23 + new):**

| Duty | What it constrains | Trust it creates |
|---|---|---|
| Compliance | Only transfer to/from approved parties | Regulatory assurance |
| Soulbound | Never transfer at all | Permanent identity binding |
| Transfer Limits | Cap amounts per transaction or period | Protection against theft/manipulation |
| Fee-on-Transfer | Deduct percentage to treasury on every move | Sustainable protocol funding |
| KYC Gate | Require verified credential to mint/receive | Legal compliance |
| Controller Gate | Require program proof to move tokens | Program-controlled accounts |
| Supply Cap | Never mint beyond ceiling | Scarcity guarantee |
| Timelock | Wait before config changes take effect | Governance safety |
| CommitmentGuard (new) | Keep balance above committed floor | Counterparty payment assurance |

**What Duties produce in statements:**

- Balance Sheet → Liability section. Each duty is an obligation of the token entity.
- P&L → Expense line. Fees paid (Fee-on-Transfer), opportunity cost of constraints.
- Cash Flow → Financing section. Outflows from honoring obligations.

### 3.4 Memory — What You KNOW

**Accounting analog:** Retained Earnings
**Personality analog:** Experience
**Direction:** Backward (accumulated)

Memory is the token's accumulated state from its operational history. In PLUMB terms, memory lives in skill state trees — Merkle trees that persist across operations and record what has happened. Memory is not a hook that fires on operations; it is the residue that operations leave behind.

In accounting, retained earnings are the sum of all past net income not distributed to owners. They represent what the entity has accumulated from its entire operating history. Memory serves the same function for tokens — it is the verifiable, content-addressed record of everything the token has done.

Memory makes the difference between a new token and a seasoned one. Two tokens with identical skills and duties but different memories have different trust profiles. A token that has processed 10,000 swaps with zero failed settlements is more trustworthy than a freshly minted one, even if they are structurally identical. Memory IS reputation.

**What constitutes Memory:**

| Memory source | What it records | Trust signal |
|---|---|---|
| Liquidity allocation tree | Swap history, strategy performance | Market-making reliability |
| Governance proposal tree | Voting history, proposal outcomes | Governance maturity |
| Oracle attestation tree | Price feed history, uptime | Data reliability |
| Settlement receipts | Contract outcomes, payout accuracy | Financial trustworthiness |
| Compliance audit trail | Transfer approvals, rejection history | Regulatory compliance track record |
| Merkle root history | State transitions over time | Overall operational integrity |

**What Memory produces in statements:**

- Balance Sheet → Retained Earnings / Accumulated Experience section.
- P&L → Not directly, but memory determines how P&L items are valued (a token with proven history commands premium pricing).
- Cash Flow → Not directly, but memory quality affects access to capital (trusted tokens attract more liquidity).

### 3.5 Senses — What You Can PERCEIVE

**Accounting analog:** Revenue Streams (input channels)
**Personality analog:** Perception
**Direction:** Inward (information)

A sense is a connection to external data that the token can consume in its operations. Without Oracle Pricing, a token is blind to market prices — it literally cannot evaluate a Sword contract that references ETH/USD. Without a cross-chain proof relay, a token is deaf to events on other chains. Senses determine what the token can know about the world outside its own state tree.

In accounting, revenue streams are the channels through which value enters the entity. For tokens, senses are the channels through which information enters. Revenue follows information — you can only charge for swaps at market price if you can perceive the market price. You can only liquidate undercollateralized positions if you can perceive the collateral ratio. Senses are the prerequisite for every skill that depends on external state.

**What constitutes Senses:**

| Sense | What it perceives | What it enables |
|---|---|---|
| Oracle Pricing (COMPASS) | Market prices via stark-proven aggregation | Lending health factors, Sword evaluation, stablecoin pegs |
| Proven Price (native) | Fee-weighted TWAP from on-chain swaps | Self-pricing without external oracles |
| Cross-chain Relay | State proofs from other blockchains | Bridging, cross-chain collateral |
| Event Monitor | On-chain events (liquidations, governance votes) | Reactive strategies, automated settlement |
| Timestamp | Current block time from consensus | Timelock expiry, vesting schedules, contract maturity |

A key property of senses on provable blockchains: every perceived value comes with a proof. Oracle Pricing data is a [[stark]] proof of correct aggregation. Proven Price is a byproduct of proven swap execution. Cross-chain state comes with inclusion proofs. There is no "trust the oracle" — there is "verify the proof of what the oracle claims."

**What Senses produce in statements:**

- Balance Sheet → Not directly, but senses determine the valuation of assets (a token that can perceive prices can mark-to-market).
- P&L → Revenue enablement. Senses don't generate revenue, but skills that require senses do.
- Cash Flow → Operating section (indirectly). Data feed costs, oracle fees.

### 3.6 Bonds — Who You Are WITH

**Accounting analog:** Counterparties
**Personality analog:** Relationships
**Direction:** Between (connection)

A bond is a persistent, verifiable relationship between two tokens. Not a transaction (which is a one-time event) but a structural link that affects ongoing operations. The `controller` field pointing from a Coin to a Card is a bond. The `royalty` field linking an NFT to its creator is a bond. The delegation chain from owner to delegate is a bond.

In accounting, your counterparty ledger defines your exposure — receivables, payables, credit lines, guarantees. In personality, relationships define your social existence. For tokens, bonds define the web of permissions, obligations, and dependencies that connect entities.

Bonds are bidirectional in effect: a Delegation bond gives the delegate spending power (their asset) while creating exposure for the owner (their liability). A Royalties bond gives the creator recurring income (their asset) while imposing a cost on every transfer (the holder's duty). Every bond shows up as an asset on one side and a liability on the other — the double-entry principle, expressed as token relationships.

**What constitutes Bonds:**

| Bond | Between | Effect |
|---|---|---|
| Royalties | Creator ↔ Asset | Creator receives percentage on every transfer |
| Delegation | Owner ↔ Delegate | Delegate can spend within bounds |
| Multisig | N Signers ↔ Controlled Account | Threshold approval required |
| Subscription | Payer ↔ Recipient | Recurring authorized payments |
| Vesting | Beneficiary ↔ Token | Time-locked progressive release |
| Controller | Coin ↔ Controlling Program/Card | Program must co-authorize movement |
| Contract Position | Contract Card ↔ Position Holder | Settlement rights |

**What Bonds produce in statements:**

- Balance Sheet → Counterparty section. Both asset-side (receivables from bonds) and liability-side (payables from bonds).
- P&L → Revenue (royalty income, delegation fees) and Expenses (subscription payments, royalty costs).
- Cash Flow → All three sections. Operating (subscription income/expense), investing (position acquisition/disposal), financing (delegation relationships).

---

## 4. The Accounting Identity

In double-entry accounting, the fundamental equation is:

```
Assets = Liabilities + Equity
```

For tokens:

```
Skills + Senses + Bonds(receivable) = Duties + Bonds(payable) + Nature
```

This is not metaphor. It is structurally enforced. Consider a token entity's complete state:

**Left side (what the entity HAS):**
- Skills: Liquidity, Lending, Governance = revenue-generating capabilities
- Senses: Oracle Pricing, Proven Price = information channels
- Bonds (receivable): Royalties owed TO this entity, Delegation rights FROM others

**Right side (what the entity OWES or IS):**
- Duties: Compliance, Transfer Limits, CommitmentGuard = constraints and obligations
- Bonds (payable): Royalties owed BY this entity, Subscriptions payable
- Nature: TSP-1 or TSP-2 conservation law = fundamental equity

The identity balances because every capability has a corresponding constraint somewhere in the system. If you add Lending (skill/asset), your counterparty gains a Debt (duty/liability). If you add Delegation (bond/receivable on the delegate's side), the owner gains Exposure (bond/payable on the owner's side). Adding and removing traits always preserves the accounting identity across the system.

Memory (retained earnings) accumulates on the left side over time, growing equity. The P&L captures the change per period:

```
Revenue (from Skills) - Expenses (from Duties) = Net Income → Memory (accumulated)
```

### 4.1 Financial Statements From Token Traits

The practical consequence: by reading a token's traits, you can reconstruct its financial statements without any additional infrastructure.

**Balance Sheet** (at any block height):

```
ASSETS                          LIABILITIES + EQUITY
─────────────────────────────   ─────────────────────────────
Skills:                         Duties:
  Liquidity (TIDE)     ✓         Compliance           ✓
  Lending              ✓         Transfer Limits      ✓
  Governance           ✓         CommitmentGuard      ✓

Senses:                         Bonds (payable):
  Oracle Pricing       ✓         Royalties (5% out)   ✓
  Proven Price         ✓         Subscription         ✓

Bonds (receivable):             Nature:
  Delegation rights    ✓         TSP-1 (Coin)
  Royalties (2% in)   ✓         supply = 1,000,000

Memory:                         
  10,247 swaps processed
  99.8% settlement success
  18 months operational
─────────────────────────────   ─────────────────────────────
```

**P&L** (per epoch/period):

```
REVENUE                              EXPENSES
────────────────────────────────     ────────────────────────────────
Liquidity fees earned:    +450 NPT   Fee-on-Transfer paid:     -120 NPT
Lending interest:         +200 NPT   Royalties paid:            -85 NPT
Staking rewards:          +100 NPT   Subscription payments:     -50 NPT
                                     Oracle feed costs:         -10 NPT
────────────────────────────────     ────────────────────────────────
Total Revenue:            +750 NPT   Total Expenses:           -265 NPT

                    NET INCOME:      +485 NPT → Memory
```

**Cash Flow** (per epoch/period):

```
OPERATING:
  Swap fees received:           +450 NPT
  Interest received:            +200 NPT
  Fees paid:                    -120 NPT
  Royalties paid:                -85 NPT
  Net Operating:                +445 NPT

INVESTING:
  Sword contract positions acquired:  -10,000 NPT
  Sword contract settled:              +12,000 NPT
  Net Investing:                        +2,000 NPT

FINANCING:
  Staking rewards:              +100 NPT
  Subscription payments:         -50 NPT
  Net Financing:                 +50 NPT

NET CASH FLOW:                +2,495 NPT
```

This is not hypothetical. Every line item corresponds to a provable on-chain event — a stark proof of a specific operation. The financial statements are free byproducts of the proof system. No separate accounting layer needed. The token's traits determine which line items exist. The token's operations fill them in.

### 4.2 Why This Matters

If tokens automatically produce balance sheets, P&L, and cash flow statements, then:

**For DAOs:** Treasury management becomes native. A DAO's TSP-2 Card (its identity) has Skills (Governance, Lending), Duties (Compliance, Timelock), Senses (Oracle Pricing), Bonds (Delegation to working groups), and Memory (proposal history). Its financial statements are derived directly from its traits and operations. No Gnosis Safe spreadsheets. No manual reconciliation.

**For Regulators:** Compliance reporting becomes provable. A regulated token's Duties (Compliance, KYC Gate, Transfer Limits) are on-chain and machine-readable. Its Memory contains the complete audit trail. The regulator can verify the token's financial statements by checking proofs — not by trusting self-reported data.

**For Individuals:** Personal finance becomes automatic. Your identity Card's Bonds (contract positions, delegations, subscriptions) define your exposure. Your Coins' Skills (Lending, Staking, Liquidity) define your income sources. Your Duties (CommitmentGuards) define your obligations. Your financial picture is always current, always provable, always complete.

**For Protocols:** Composability becomes typed. When two tokens interact, the trait vocabulary tells you exactly what will happen: which Skills are exercised, which Duties must be honored, which Bonds are created, which Senses are consulted, what Memory is recorded. No surprises. No "read all 23 skills and figure out which matter."

---

## 5. Implementation

### 5.1 Trait Architecture in PLUMB

Token traits are implemented through the existing PLUMB hook system. The vocabulary change is organizational, not architectural — no new on-chain primitives are needed. What changes is how traits are classified, discovered, and reasoned about.

```
┌──────────────────────────────────────────────────────────┐
│  NATURE                                                   │
│  TSP-1 (Coin) or TSP-2 (Card) — conservation law          │
├──────────────────────────────────────────────────────────┤
│  TRAITS                                                   │
│                                                           │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐                    │
│  │ Skills  │ │ Duties  │ │ Senses  │                    │
│  │ (can do)│ │(must do)│ │(see)    │                    │
│  └────┬────┘ └────┬────┘ └────┬────┘                    │
│       │           │           │                          │
│       └─────┬─────┘           │                          │
│             │                 │                          │
│  ┌──────────v─────────────────v──────────┐               │
│  │         PLUMB Hook Slots              │               │
│  │  pay_hook · lock_hook · update_hook   │               │
│  │  mint_hook · burn_hook                │               │
│  └───────────────────────────────────────┘               │
│                                                           │
│  ┌─────────┐ ┌─────────┐                                │
│  │ Memory  │ │ Bonds   │                                │
│  │ (knows) │ │(linked) │                                │
│  └─────────┘ └─────────┘                                │
│  State trees    Leaf fields                               │
│  & history      & cross-token                             │
│                 references                                │
├──────────────────────────────────────────────────────────┤
│  PLUMB FRAMEWORK                                          │
│  Leaf format · Config · Auth · 5 Operations               │
└──────────────────────────────────────────────────────────┘
```

All trait types install through the same PLUMB hook system. The distinction between Skills, Duties, Senses, Bonds, and Memory is semantic — it describes the INTENT and EFFECT of the hook, not a different mechanism. A Compliance hook and a Liquidity hook both install in `pay_hook`. But they are opposites: one restricts, one enables.

### 5.2 Trait Metadata

Each trait carries metadata that allows classification:

```
struct TraitMeta {
    name: Digest,           // content-addressed name
    category: Field,        // 0=Skill, 1=Duty, 2=Sense, 3=Bond, 4=Memory
    hooks: [Field; 5],      // which PLUMB hooks it installs (bitmask per operation)
    requires: [Digest; 4],  // other traits this depends on
    state_tree: bool,       // whether it maintains a persistent state tree
    description: Digest,    // content-addressed human-readable docs
}
```

Category classification enables:

- Wallets displaying "Skills: 3 | Duties: 2 | Senses: 1 | Bonds: 4"
- Queries: "show me all my Duties" or "what Senses does this token have?"
- Trust scoring: ratio of Skills to Duties, Memory depth, Bond network
- Automated financial statement generation from trait categories

### 5.3 Trait Discovery

A wallet or explorer reading a token's config can reconstruct its complete trait profile:

1. Read the config hash → extract 5 hook digests
2. For each non-empty hook, resolve the content-addressed trait definition
3. Read `TraitMeta.category` to classify
4. Read Bonds from leaf fields (`controller`, `locked_by`, `royalty`, `delegation`)
5. Read Memory from state tree roots referenced in the config
6. Assemble the balance sheet / P&L / cash flow view

This is the equivalent of reading a company's annual report — but provable, real-time, and automatically structured.

### 5.4 Trait Composition Rules

Not all trait combinations are valid. The vocabulary makes conflicts visible:

**Skills compose freely.** Liquidity + Lending + Governance = a token that can swap, lend, and vote. No conflicts possible — capabilities are additive.

**Duties compose with AND.** Compliance + Transfer Limits = must transfer to approved parties AND within amount caps. Both constraints apply. The proof must satisfy ALL duties simultaneously.

**Senses compose with OR.** Oracle Pricing + Proven Price = the token can perceive prices from external oracles OR from on-chain swap data. Multiple senses provide redundancy and optionality.

**Bonds compose structurally.** Royalties + Delegation = each operates on its own axis. Royalties affect `pay` hooks. Delegation affects `auth`. No conflict unless two bonds claim authority over the same operation.

**Cross-category conflicts are visible.** Soulbound (Duty: never transfer) + Liquidity (Skill: enable swaps) = contradictory. stark proof composition fails — both proofs cannot be simultaneously valid. The trait vocabulary makes this conflict visible at configuration time, not at runtime. You can see "wait, I'm installing a Skill that requires transfers, but I already have a Duty that forbids transfers" before deployment.

---

## 6. The CommitmentGuard Pattern

The trait vocabulary emerged from a specific design challenge: how does a homebuyer assure a builder that construction payment is guaranteed, without locking capital in escrow?

### 6.1 The Problem

Alice hires Bob to build a house. Cost: 100,000 NPT. Construction takes 6 months. Bob invests labor and materials progressively. Bob needs assurance that Alice's money will be there at each milestone.

Traditional solution: escrow. Alice deposits 100,000 NPT into a third-party account. The money is frozen for 6 months. Alice earns nothing on it. She can't use it for governance, lending, or any other purpose. Dead capital.

### 6.2 The Solution

Alice installs a Duty on her TSP-1 account:

```
CommitmentGuard {
    category: Duty,
    floor: 100,000 NPT,
    beneficiary: bob_builder_card_id,
    expiry: activation + 6 months,
    hooks: [pay_hook],  // constrains outgoing payments
}
```

The `pay_hook` rejects any proof where Alice's post-payment balance falls below 100,000 NPT. Alice can still:

- Vote in governance with these tokens (Governance Skill reads balance, doesn't move it)
- Earn staking rewards on these tokens (Staking Skill adds to balance)
- Receive payments, increasing her free balance above the 100,000 floor
- Spend any amount ABOVE 100,000 freely

Alice's total balance is 150,000 NPT. She can spend up to 50,000 freely. The bottom 100,000 is committed — not locked, committed. The difference matters: the tokens are in Alice's account, earning, voting, composable. But no valid proof exists that would move her balance below the floor.

### 6.3 Why This Is Stronger Than Escrow

| Property | Escrow | CommitmentGuard |
|---|---|---|
| Builder assurance | Money is in third party's hands | No valid proof exists to spend below floor |
| Alice's capital efficiency | Zero — money is frozen | Full — money earns, votes, composes |
| Cryptographic guarantee | Trust the escrow agent | Mathematical — proof generation fails |
| Composability | None | Full — same balance backs multiple purposes |
| Removal protection | Escrow agent controls | Requires Bob's co-signature (Multisig on config) |
| Cost | Escrow agent fees | Zero — it's a hook, not a service |

### 6.4 How the Vocabulary Helped

This pattern is invisible when everything is a "skill." CommitmentGuard is not a capability — it's a constraint. Classifying it as a Duty makes its purpose clear: it appears on the liability side of Alice's balance sheet. Bob can query "what Duties does Alice carry?" and see the commitment. Alice's wallet shows "Duties: CommitmentGuard (100,000 NPT for Bob, expires Oct 2026)." The vocabulary does the work.

---

## 7. Complete Trait Catalog

Reclassification of all existing Gold Standard hooks plus new additions:

### 7.1 Skills (9)

| # | Trait | What it enables |
|---|---|---|
| S1 | Liquidity (TIDE) | Provide/take liquidity in token swaps |
| S2 | Governance | Vote on proposals using token balance |
| S3 | Lending | Lend tokens or borrow against collateral |
| S4 | Staking | Lock tokens to earn protocol rewards |
| S5 | Vault | Deposit assets, receive shares at exchange rate |
| S6 | Bridging | Move tokens across chains via proof relay |
| S7 | Batch Operations | Bundle multiple operations in one proof |
| S8 | Burn-to-Redeem | Destroy one asset to claim another |
| S9 | Sword Evaluator | Evaluate financial contract ASTs to payout tables |

### 7.2 Duties (9)

| # | Trait | What it constrains |
|---|---|---|
| D1 | Compliance | Only transfer to/from approved parties |
| D2 | Soulbound | Never transfer |
| D3 | Transfer Limits | Cap amounts per transaction or period |
| D4 | Fee-on-Transfer | Deduct percentage to treasury on every move |
| D5 | KYC Gate | Require verified credential to mint/receive |
| D6 | Controller Gate | Require program proof to move tokens |
| D7 | Supply Cap | Never mint beyond ceiling |
| D8 | Timelock | Wait before config changes take effect |
| D9 | CommitmentGuard | Keep balance above committed floor |

### 7.3 Senses (3)

| # | Trait | What it perceives |
|---|---|---|
| P1 | Oracle Pricing (COMPASS) | Market prices via stark-proven aggregation |
| P2 | Proven Price | Fee-weighted TWAP from on-chain swaps |
| P3 | Cross-chain Relay | State proofs from other blockchains |

### 7.4 Bonds (5)

| # | Trait | Between whom |
|---|---|---|
| B1 | Royalties | Creator ↔ Asset (permanent revenue share) |
| B2 | Delegation | Owner ↔ Delegate (bounded spending authority) |
| B3 | Multisig | N Signers ↔ Controlled entity (threshold approval) |
| B4 | Subscription | Payer ↔ Recipient (recurring authorized payments) |
| B5 | Vesting | Beneficiary ↔ Token pool (time-locked release) |

### 7.5 Memory (implicit)

Memory is not installed as a discrete trait. It accumulates automatically from all operations. Every Skill, Duty, Sense, and Bond that maintains a state tree contributes to the token's Memory. The Memory IS the set of state trees plus the Merkle root history.

What is discrete is the **depth** and **queryability** of Memory:

- A token with Liquidity (TIDE) accumulates swap history automatically.
- A token with Governance accumulates proposal/vote history automatically.
- A token with no stateful traits has Memory limited to its basic operation log.

Memory grows over time. It cannot be deleted (content-addressed, on-chain). It can be archived (old state roots remain valid but off-chain storage may prune intermediate states).

---

## 8. Recipes: Trait Combinations for Common Entities

### 8.1 Simple Currency

```
Nature:  TSP-1 (Coin)
Skills:  Liquidity
Duties:  Supply Cap
Senses:  Proven Price
Bonds:   —
```

A basic tradeable token with fixed supply and market pricing.

### 8.2 Regulated Security Token

```
Nature:  TSP-1 (Coin)
Skills:  Liquidity, Governance, Lending
Duties:  Compliance, KYC Gate, Transfer Limits, Supply Cap
Senses:  Oracle Pricing, Proven Price
Bonds:   Multisig (board of directors)
```

A fully compliant security that can trade, vote, and be used as collateral — but only between verified parties within regulatory limits.

### 8.3 Artist's NFT Collection

```
Nature:  TSP-2 (Card)
Skills:  Burn-to-Redeem (physical item claim)
Duties:  —
Senses:  —
Bonds:   Royalties (artist receives 5% on every transfer)
```

Minimal setup. The NFT trades freely, the artist earns on every sale.

### 8.4 DAO Treasury

```
Nature:  TSP-2 (Card) — the DAO identity
Skills:  Governance, Lending, Vault, Staking
Duties:  Timelock, Compliance
Senses:  Oracle Pricing, Proven Price
Bonds:   Multisig (core team), Delegation (working groups), Subscription (recurring grants)
```

A fully autonomous economic entity. The Card IS the DAO. Its Skills define what the treasury can do. Its Duties protect against rash decisions. Its Bonds define the organizational structure. Its Memory IS the institutional knowledge.

### 8.5 Sword Derivative Contract

```
Nature:  TSP-2 (Card) — the contract itself
Skills:  Sword Evaluator
Duties:  Timelock (maturity enforcement)
Senses:  Oracle Pricing (for obs() evaluation)
Bonds:   Contract Position (linked to position holders' Coins)
```

The contract is a Card. Evaluation is a Skill. Maturity is a Duty. Price perception is a Sense. The counterparty relationship is a Bond. Every trait category participates.

### 8.6 Person (Identity)

```
Nature:  TSP-2 (Card) — unique, non-fungible
Skills:  Governance (vote), Lending (borrow)
Duties:  CommitmentGuard (obligations to counterparties)
Senses:  Oracle Pricing (market awareness)
Bonds:   Delegation (authorized agents), Subscription (recurring payments),
         Vesting (employment compensation), Contract Position (derivative exposure)
```

A person IS a token. Their capabilities (Skills), obligations (Duties), data access (Senses), relationships (Bonds), and reputation (Memory) form a complete identity — one that produces the same financial statements any person or company expects from their economic life.

---

## 9. Design Principles

### 9.1 Orthogonality

Each trait category is independent. You can add a Skill without changing Duties. You can add a Duty without affecting Senses. You can form Bonds without installing Skills. The categories are orthogonal dimensions of the token's identity, not layers that depend on each other.

Exception: some traits have cross-category dependencies. The Lending Skill requires the Oracle Pricing Sense (to check health factors). The Sword Evaluator Skill requires Oracle Pricing Sense (for `obs()` evaluation) and Timelock Duty (for maturity enforcement). Dependencies are declared in `TraitMeta.requires`.

### 9.2 Symmetry

Every bond shows up twice — as a receivable on one side and a payable on the other. Every duty that constrains one party creates trust for another. Every skill that enables one operation requires a corresponding duty somewhere in the system to keep it safe. The trait vocabulary makes the double-entry nature of economic relationships visible.

### 9.3 Completeness

The six traits (Nature, Skills, Duties, Memory, Senses, Bonds) are claimed to be complete — there is no seventh category needed. The argument:

- Nature covers the intrinsic (what you ARE).
- Skills cover outward capability (what you CAN DO).
- Duties cover inward constraint (what you MUST DO).
- Memory covers the past (what you KNOW).
- Senses cover external input (what you PERCEIVE).
- Bonds cover relationships (who you're WITH).

These six exhaust the possible relationships between an entity and the world: being, acting, constraining, remembering, perceiving, connecting. Any proposed seventh category either reduces to one of these or is a compound (Vesting = Bond + Duty). The claim is that any new token behavior will classify cleanly into one of the six, or will be a composition of traits from multiple categories.

### 9.4 Parsimony

The vocabulary adds zero new on-chain primitives. PLUMB hooks, state trees, leaf fields, and proof composition are unchanged. The six trait categories are a lens for understanding what already exists — a classification system that makes the existing 23+ hooks legible as a coherent model of economic agency, rather than a flat list of features.

---

## 10. Open Questions

1. **Should trait category be enforced on-chain?** Currently proposed as metadata. Could be a constraint: a hook self-declares as Skill or Duty, and the system prevents contradictory installations. Or should it remain purely informational?

2. **Memory pricing.** Memory grows forever (state trees only accumulate). Should there be state rent for Memory, or is the proof cost of deeper Merkle trees sufficient economic pressure?

3. **Trait standards.** Should there be formal interfaces per category? E.g., every Duty must export `constraint_description()`, every Sense must export `data_schema()`. Or is the informal convention sufficient?

4. **Composite traits.** Vesting is both a Bond (relationship with beneficiary) and a Duty (time-locked constraint). Should it be classified as one or the other, or should composite classification be supported?

5. **Negative Bonds.** Current bonds are cooperative (Delegation, Royalties, Multisig). Are there adversarial bonds? A lien, a lawsuit, a blacklist entry? How do these map to the vocabulary?

6. **Trait market.** If traits are content-addressed code deployed to Atlas, can there be a marketplace for traits? "Install this CommitmentGuard variant optimized for construction escrow." Economic dynamics of trait creation and adoption.

---

## See Also

- [Gold Standard](https://github.com/cyberia-to/trident/blob/master/docs/explanation/gold-standard.md) — PLUMB framework, TSP-1, TSP-2, proven price
- [Skill Library](https://github.com/cyberia-to/trident/blob/master/docs/explanation/skill-library.md) — Detailed specifications of all 23 hooks
- [Sword](./SWORD.md) — Financial contract DSL as contracts-as-tokens