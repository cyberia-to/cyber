---
tags: bostrom, cybernomics, article
alias: bostrom tokenomics, bostrom token model
crystal-type: article
crystal-domain: economics
---
# Bostrom Tokenomics

[[bostrom]] launched November 5, 2021 at block 0. Node software: [[go-cyber]] v7.0.1. Live mainnet.

## Overview

[[bostrom]] is a [[cosmos-sdk]] [[blockchain]] and the production deployment of the [[cyber]] [[knowledge graph]] protocol. It is the [[bootloader]] of [[superintelligence]] — the live network where the foundational mechanics of a provable, decentralized intelligence layer are built, tested, and proven in production.

The core primitive is a provable [[knowledge graph]] — a permissionless, on-chain structure where any [[neuron]] can create permanent, content-addressed semantic [[cyberlinks]] between [[particles]], and the [[relevance machine|relevance]] of every link is computed transparently on-chain using token-weighted algorithms running on GPU.

[[bostrom]] separates four economic functions that most [[blockchains]] compress into a single [[token]]:

| Function | Token | Symbol |
|---|---|---|
| network [[bostrom/infrastructure/security|security]] and [[governance]] | [[$BOOT]] | `BOOT` |
| liquid representation of [[staking]] | [[$H]] | `H` |
| write access to the [[knowledge graph]] | [[$V]] | `V` |
| influence over graph [[relevance machine|relevance]] ranking | [[$A]] | `A` |

Every token derives from the one above it. [[$H]] requires staked [[$BOOT]]. [[$V]] and [[$A]] require burned [[$H]]. Every unit of network resource has a provable, on-chain opportunity cost denominated in committed stake.

## Token: BOOT

denom: `boot`

[[$BOOT]] is the base layer. It does not grant direct access to network services — its role is to secure [[bostrom/consensus|consensus]], enable [[governance]], and anchor the value of everything built on top.

### Staking

[[$BOOT]] holders [[delegation|delegate]] to [[hero|heroes]] via standard [[cosmos-sdk]] DPoS. [[hero|Heroes]] and delegators earn [[inflation]]-based [[$BOOT]] rewards. The [[bostrom]] staking module is a custom fork of the [[cosmos-sdk]] staking module.

Delegating [[$BOOT]] simultaneously [[mint|mints]] [[$H]] into the delegator's account at 1:1. Undelegating [[burn|burns]] the corresponding [[$H]]. This is handled natively in the cyberbank module via [[delegation]] hooks — no separate liquid [[staking]] protocol is needed.

### Governance

On-chain [[basic governance|governance]] uses [[$BOOT]]-weighted voting across four proposal types:

- ideas — non-binding signals and directional proposals
- upgrades — binary software upgrade proposals
- parameters — all protocol parameters are adjustable by [[governance]] within validated bounds
- fund — disbursements from the community pool

### Smart Contract Execution Fees

[[cosmwasm]] smart contracts on [[bostrom]] pay execution fees in [[$BOOT]]. The fee distribution is split 80/20: 80% returns directly to the program creator, 20% goes to [[hero|heroes]] and the community pool through standard distribution. This is hardcoded in the dmn module and creates a native revenue model for [[autonomous progs]].

## Token: HYDROGEN

denom: `hydrogen` (referred to as `scyb` throughout the codebase — the original name, short for staked CYB)

[[$H]] is the liquid [[staking]] derivative of [[$BOOT]] and the primary [[token]] of the [[bostrom]] network. While [[$BOOT]] is the base [[bostrom/infrastructure/security|security]] layer, [[$H]] is what [[neurons]] actually hold, display, and transact with. The network's total value is expressed as the sigma of all [[$H]] in circulation, making [[$H]] the canonical unit of account for the ecosystem.

[[$H]] is issued solely through [[$BOOT]] [[delegation]]; destroyed solely through [[$BOOT]] undelegation.

```
delegate 1000 BOOT  →  mint 1000 H
undelegate 1000 BOOT  →  burn 1000 H
```

[[$H]] has two uses:

1. investmint input — [[burn|burned]] to [[mint]] [[$V]] or [[$A]]
2. [[cyber/liquidity|liquidity]] — deposited into farm contracts, traded on the built-in [[automated market maker]], or used in any [[cosmwasm]] contract

[[$H]] does not earn [[staking]] rewards itself. The underlying staked [[$BOOT]] continues to earn rewards for the delegator. [[$H]] is the spendable, transferable proof that the corresponding [[$BOOT]] is at stake.

## Tokens: VOLT and AMPERE

denoms: `millivolt`, `milliampere`

[[$V]] and [[$A]] are the operational [[tokens]] of the [[knowledge graph]]. They are [[mint|minted]] exclusively through investmint — there is no [[inflation]], no faucet, and no other issuance path. Every [[$V]] and [[$A]] in existence was produced by [[burn|burning]] [[$H]].

[[$V]] (VOLT) is [[bandwidth]]. Creating a [[cyberlink]] costs [[$V]] proportional to the current dynamic [[bandwidth price]]. A [[cyberlink]] is a permanent, content-addressed, directed edge in the on-chain [[knowledge graph]] connecting two [[ipfs]] CIDs.

[[$A]] (AMPERE) is rank weight. The GPU-executed token-weighted [[pagerank]] and graph-entropy algorithms weight each [[neuron]]'s [[cyberlinks]] proportionally to their [[$A]] balance. More [[$A]] means greater influence over what the graph surfaces as relevant.

## The Investmint Mechanism

Investmint is how [[$H]] becomes [[$V]] or [[$A]]. A [[neuron]] sends a quantity of [[$H]] to the resources module — the [[$H]] is [[burn|burned]] immediately and permanently — and [[$V]] or [[$A]] are [[mint|minted]] in return. There is no lock, no vesting cliff, no retrieval. The [[$H]] is gone; the resources are yours.

The amount [[mint|minted]] depends on three factors: base rate, halving, and supply decay.

### Base Rate

```
V (millivolt):    return = (H / 1_000_000_000) × (maxPeriod / 2_592_000) × halving × 1000
A (milliampere):  return = (H / 100_000_000)  × (maxPeriod / 2_592_000) × halving × 1000
```

where:
- `H` = [[$H]] [[burn|burned]] (base unit)
- `maxPeriod` = protocol-determined period at current block height (seconds)
- `2_592_000` = 30 days in seconds (base investmint period)

The [[$A]] base amount is 10x lower than [[$V]]. The same [[$H]] [[burn|burned]] at the same block height yields 10x more [[$A]] than [[$V]]. [[bandwidth]] ([[$V]]) is scarcer than computation weight ([[$A]]).

### Halving Schedule

Both [[$V]] and [[$A]] issuance follow a halving schedule. Every 9,000,000 blocks the effective [[mint]] rate halves. Halvings are shifted 6,000,000 blocks from [[bostrom/genesis|genesis]] and do not begin until block 15,000,000.

```
halving = max(0.01,  0.5 ^ floor((blockHeight − 6_000_000) / 9_000_000))
```

The floor is 1%: the halving factor never drops below 0.01 regardless of block height. Investmint never becomes completely worthless.

### Supply Decay

Beyond block-based halving, every investmint call applies a continuous penalty based on total cumulative outstanding supply:

```
factor = 0.5 ^ (totalSupply / halfLife)

halfLife(V) = 4,000,000,000
halfLife(A) = 32,000,000,000
```

The [[$A]] half-life (32B) is 8x larger than [[$V]]'s (4B). [[$A]] can accumulate 8x more before hitting the same penalty.

| Supply / halfLife | Decay factor |
|---|---|
| 0 | 1.000 (no penalty) |
| 0.5 | 0.707 |
| 1.0 | 0.500 |
| 2.0 | 0.250 |
| 3.0 | 0.125 |

No oracle, [[governance]] vote, or external trigger required. [[scarcity]] increases automatically as usage grows.

### Combined Mint Formula

```
final_return = base_return × halving × supply_decay_factor
```

If `final_return < 1000` (minimum threshold), the transaction is rejected. This prevents dust investmints.

### Investmint Parameters

| Parameter | Value |
|---|---|
| Base investmint amount (V) | 1,000,000,000 H |
| Base investmint amount (A) | 100,000,000 H |
| Base investmint period | 2,592,000 s (30 days) |
| Halving period (V and A) | 9,000,000 blocks |
| Halvings begin at block | 15,000,000 |
| Halving floor | 1% (0.01x) |
| Max investmint slots per address | 8 ([[governance]] max: 16) |
| V supply half-life | 4,000,000,000 |
| A supply half-life | 32,000,000,000 |
| Minimum [[mint]] threshold | 1,000 (milli-units) |

## Bandwidth Model

The [[bandwidth]] module governs [[$V]] consumption and throughput pricing.

Each [[neuron]] has a [[$V]] [[neuron bandwidth|bandwidth meter]]. The meter fills as the [[neuron]] holds [[$V]] and depletes as they create [[cyberlinks]]. The cost per [[cyberlink]] is not fixed — it adjusts dynamically based on current network utilisation via [[bandwidth price|adaptive pricing]]:

- when load is below target (10% of max block [[bandwidth]]): price falls, encouraging usage
- when load is above target: price rises, dampening demand without a mempool auction

The price adjusts every 5 blocks. A [[neuron]]'s depleted meter recovers to its maximum over 100 blocks.

| Parameter | Default |
|---|---|
| Recovery period | 100 blocks |
| Price adjustment period | 5 blocks |
| Base price | 0.25 V per unit |
| Target network load | 10% of max block [[bandwidth]] |
| Max block [[bandwidth]] | 10,000 units per block |

### Energy Grid

The grid module allows [[$V]] and [[$A]] to be routed to [[cosmwasm]] programs via energy routes. A [[neuron]] or contract can create a route that continuously directs their resource allocation to an [[autonomous progs|autonomous program]]. Programs that receive routed [[$V]] can themselves create [[cyberlinks]] — enabling self-sustaining, autonomous [[knowledge graph]] expansion. Programs earning [[$BOOT]] execution fees create a direct incentive for operators to reinvest those rewards back into the [[staking]] → [[$H]] → [[$V]]/[[$A]] chain.

## End-to-End Token Flow

```
                    delegate
BOOT ─────────────────────────────────► H
  ▲                                     │
  │ staking rewards                     │ Investmint (burn)
  │                                     │
  │                            ┌────────┴────────┐
  │                            ▼                 ▼
  │                            V                 A
  │                            │                 │
  │                            │ cyberlinks       │ graph rank weight
  │                            ▼                 ▼
  │                     knowledge graph    PageRank / Graph-Entropy
  │
  └── 80% execution fees ◄── Autonomous Programs ◄── Energy Routes (V/A)
```

1. acquire [[$BOOT]] — via secondary market, [[staking]] rewards, or airdrop
2. [[delegation|delegate]] [[$BOOT]] → receive [[$H]] 1:1 — [[$BOOT]] earns [[staking]] rewards; [[$H]] is the liquid representation
3. investmint [[$H]] → [[$H]] is [[burn|burned]] → receive [[$V]] and/or [[$A]] — quantity determined by halving factor (block height) x supply decay factor (cumulative supply)
4. spend [[$V]] to write [[cyberlinks]] — permanent, content-addressed entries in the [[knowledge graph]]; price adjusts dynamically with block utilisation
5. hold [[$A]] to weight [[cyberlinks]] in GPU-computed [[pagerank]] — more [[$A]] = greater influence over graph [[relevance machine|relevance]]
6. route [[$V]]/[[$A]] via the grid to power [[autonomous progs]] → programs earn 80% of [[$BOOT]] execution fees → reinvest back into step 1
7. deposit into farm contracts → receive liquid position [[tokens]] → earn additional [[$BOOT]]/[[$H]] rewards across configurable block schedules

## Economic Properties

### Everything costs stake

Because [[$V]] and [[$A]] can only be [[mint|minted]] by [[burn|burning]] [[$H]] — which itself requires [[staking]] [[$BOOT]] — every unit of network resource has an explicit, on-chain opportunity cost denominated in committed stake. You cannot spam the [[knowledge graph]] without locking value into the network's [[bostrom/infrastructure/security|security]].

### Two independent deflationary forces

[[$V]] and [[$A]] face halvings (discrete, block-triggered) and supply decay (continuous, usage-triggered). Neither force alone would be sufficient: halvings without decay would allow early bulk [[mint|minting]]; decay without halvings would have no schedule anchor. Together they create a monotonically rising marginal cost for network resources without relying on hard supply caps. This is [[scarcity]] driven by [[supply and demand]].

### Scarcity is proportional to impact

[[$V]] is scarcer than [[$A]] (lower base amount, smaller half-life). Writing to the graph ([[$V]]) is a final irreversible action that permanently expands the [[knowledge graph]] and costs [[consensus]] resources to process. Influencing rank ([[$A]]) is a continuous, reweightable state that costs GPU cycles. The 10x price difference and 8x half-life difference encode this distinction directly in the protocol.

### 80% execution fee return

[[cosmwasm]] smart contracts on [[bostrom]] return 80% of their execution fees directly to the program creator. This is hardcoded in the dmn module and creates a native revenue model for on-chain [[autonomous progs]] — a program that provides value to the network earns [[$BOOT]] proportional to how often it is called, with no intermediary taking the majority of the fee.

## Source References

All mechanics derived from:

- [x/resources](https://github.com/cyberia-to/go-cyber/blob/main/x/resources/keeper/keeper.go) — investmint logic, halving, exponential decay, max period
- [x/bandwidth](https://github.com/cyberia-to/go-cyber/blob/main/x/bandwidth/types/params.go) — [[bandwidth]] pricing and recovery parameters
- [x/cyberbank](https://github.com/cyberia-to/go-cyber/tree/main/x/cyberbank) — [[$H]] [[mint|minting]] on [[delegation]], [[burn|burning]] on undelegation
- [x/rank](https://github.com/cyberia-to/go-cyber/tree/main/x/rank) — token-weighted [[pagerank]] and graph-entropy (GPU/CUDA)
- [x/grid](https://github.com/cyberia-to/go-cyber/tree/main/x/grid) — [[$V]]/[[$A]] energy routing to [[autonomous progs]]
- [x/dmn](https://github.com/cyberia-to/go-cyber/tree/main/x/dmn) — 80% execution fee return mechanic
- [x/graph](https://github.com/cyberia-to/go-cyber/tree/main/x/graph) — [[cyberlink]] creation, burned [[$V]] and [[$A]] tracking

[[bostrom]] mainnet — [[go-cyber]] v7.0.1 — February 2026
