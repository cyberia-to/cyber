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

The core primitive is a provable [[knowledge graph]] — a permissionless, on-chain structure where any [[neuron]] can create permanent, content-addressed semantic [[cyberlinks]] between [[particles]], and the [[relevance machine]] computes the relevance of every link transparently on-chain using token-weighted algorithms running on GPU.

[[bostrom]] separates four economic functions that most [[blockchains]] compress into a single [[token]]:

| Token | Function |
|---|---|
| [[$BOOT]] | network [[bostrom/security]] and [[governance]] |
| [[$H]] | liquid representation of [[staking]] |
| [[$V]] | write access to the [[knowledge graph]] |
| [[$A]] | [[relevance machine]] focus influence |

Every token derives from the one above it. [[$H]] requires staked [[$BOOT]]. [[$V]] and [[$A]] require burned [[$H]]. Every unit of network resource has a provable, on-chain opportunity cost denominated in committed stake.

## Token: BOOT

denom: `boot`

[[$BOOT]] is the base layer. It does not grant direct access to network services — its role is to secure [[bostrom/consensus]], enable [[governance]], and anchor the value of everything built on top.

### Supply

Total [[$BOOT]] supply: ~480 trillion (480T). [[inflation]] mints new [[$BOOT]] each block and distributes it to [[heroes]] and delegators proportionally to their stake.

| Parameter | Value |
|---|---|
| Current [[inflation]] rate | ~1.09% annually |
| Minimum [[inflation]] | 1.09% |
| Maximum [[inflation]] | 5.46% |
| Target bonded ratio | 25.49% |

When the bonded ratio is below target, [[inflation]] increases toward the maximum to incentivize [[staking]]. When above target, it decreases toward the minimum. All parameters are adjustable by [[governance]].

### Staking

[[$BOOT]] holders delegate to [[heroes]] via standard [[cosmos-sdk]] DPoS. [[heroes]] and delegators earn [[inflation]]-based [[$BOOT]] rewards through [[delegation rewards]]. The [[bostrom]] staking module is a custom fork of the [[cosmos-sdk]] staking module.

| Parameter | Value |
|---|---|
| Max [[heroes]] (validators) | 92 |
| Unbonding period | 8 days (691,200 s) |

[[delegation]] of [[$BOOT]] simultaneously creates [[$H]] in the delegator's account at 1:1. Undelegation destroys the corresponding [[$H]]. This is handled natively in the cyberbank module via [[delegation]] hooks — no separate liquid [[staking]] protocol is needed.

### Governance

On-chain [[basic governance]] uses [[$BOOT]]-weighted voting across four proposal types:

- ideas — non-binding signals and directional proposals
- upgrades — binary software upgrade proposals
- parameters — all protocol parameters are adjustable by [[governance]] within validated bounds
- fund — disbursements from the community pool

### Fees

All transactions on [[bostrom]] pay fees in [[$BOOT]]. Standard transaction fees are distributed to [[heroes]] and delegators through the [[cosmos-sdk]] distribution module (10% community tax, remainder to validators proportional to stake).

[[cosmwasm]] contract execution fees follow a different path: 80% returns directly to the program creator, 20% goes to [[heroes]] and the community pool. This split is hardcoded in the dmn module and creates a native revenue model for [[autonomous progs]].

Two accepted [[cip]] proposals will transition fee economics to [[$H]]:

- [[burn gas in H]] — transaction fees paid in [[$H]] instead of [[$BOOT]], creating direct demand for [[$H]] and linking network usage to staked value
- [[fixed fee on H burn]] — 2% fee on every [[$H]] [[burn]] operation, extracting value from [[staking]] loan usage

## Token: HYDROGEN

denom: `hydrogen` (referred to as `scyb` throughout the codebase — the original name, short for staked CYB)

[[$H]] is the liquid [[staking]] derivative of [[$BOOT]] and the primary [[token]] of the [[bostrom]] network. While [[$BOOT]] is the base [[bostrom/security]] layer, [[$H]] is what [[neurons]] actually hold, display, and transact with. The network's total value is expressed as the sigma of all [[$H]] in circulation, making [[$H]] the canonical unit of account for the ecosystem.

[[$H]] is issued solely through [[$BOOT]] [[delegation]]; destroyed solely through [[$BOOT]] undelegation.

```
delegate 1000 BOOT  →  mint 1000 H
undelegate 1000 BOOT  →  burn 1000 H
```

[[$H]] has two uses:

1. [[mint]] input — [[burn]] to [[mint]] [[$V]] or [[$A]]
2. [[cyber/liquidity]] — traded on the built-in [[automated market maker]] ([x/liquidity](https://github.com/cyberia-to/go-cyber/tree/main/x/liquidity) module), deposited into liquidity pools to earn farming rewards, or used in any [[cosmwasm]] contract

[[$H]] does not earn [[staking]] rewards itself. The underlying staked [[$BOOT]] continues to earn rewards for the delegator. [[$H]] is the spendable, transferable proof that the corresponding [[$BOOT]] is at stake.

## Tokens: VOLT and AMPERE

denoms: `millivolt`, `milliampere`

[[$V]] and [[$A]] are the operational [[tokens]] of the [[knowledge graph]]. There is no [[inflation]], no faucet, and no other current issuance path. Early [[$V]] and [[$A]] were issued via the original investmint mechanism; all new issuance goes through [[mint]] — the [[burn]] of [[$H]].

[[$V]] (VOLT) is [[bandwidth]]. Creating a [[cyberlink]] costs [[$V]] proportional to the current dynamic [[bandwidth price]]. A [[cyberlink]] is a permanent, content-addressed, directed edge in the on-chain [[knowledge graph]] connecting two [[ipfs]] CIDs.

[[$A]] is focus. The GPU-computed diffusion weight each [[neuron]] [[cyberlinks]] proportionally to their [[$A]] balance. More [[$A]] means greater influence over what the graph surfaces as important.

## Mint

[[mint]] is how [[$H]] becomes [[$V]] or [[$A]]. A [[neuron]] sends [[$H]] to the [x/resources](https://github.com/cyberia-to/go-cyber/blob/main/x/resources/keeper/keeper.go) module in a single transaction. The [[$H]] undergoes [[burn]] immediately and permanently. [[$V]] or [[$A]] are created in return and delivered to the [[neuron]] in the same block.

### The Formula

```
finalMint = (H / baseAmount) × (maxPeriod / basePeriod) × halving × 1000 × supplyDecay
```

Three independent factors shape the output:

| Factor | Source | Direction |
|---|---|---|
| `cycles = maxPeriod / basePeriod` | chain maturity (block height) | grows over time |
| `halving` | chain maturity (block height) | shrinks over time |
| `supplyDecay` | cumulative minted supply | shrinks with usage |

### Cycles and Halving — the balancer

Both `cycles` and `halving` are derived from the same parameter: `halvingPeriod` (9,000,000 blocks for both [[$V]] and [[$A]]).

`cycles` grows exponentially:

```
maxPeriod = 2^(blockHeight / halvingPeriod) × halvingPeriod × 6
cycles = maxPeriod / basePeriod
```

`halving` shrinks exponentially (activates at block 15,000,000, retroactive to block 6,000,000):

```
halving = 0.5 ^ ((blockHeight - 6,000,000) / halvingPeriod)
floor: 0.01
```

Because both derive from `2^(blockHeight / halvingPeriod)` — one as growth, one as decay — they cancel each other out. The product `cycles × halving` remains approximately constant over time:

| Block | cycles | halving | cycles × halving |
|---|---|---|---|
| 9M | 41.7 | 1.0 | 41.7 |
| 15M | 66.0 | 0.5 | 33.0 |
| 18M | 83.3 | 0.25 | 20.8 |
| 22.8M (current) | 120.4 | 0.274 | 33.0 |
| 27M | 166.7 | 0.125 | 20.8 |

Halving is the balancer — without it, `cycles` would grow unbounded and [[mint]] would produce more resources over time. With it, the block-height component stays flat, and the real price driver is supply decay alone.

### Supply Decay — the price driver

Every [[mint]] call computes a decay factor from the total cumulative supply of the resource (including burned units):

```
supplyDecay = 0.5 ^ (totalSupply / halfLife)

halfLife(V) = 4,000,000,000
halfLife(A) = 32,000,000,000
```

This is the only factor that monotonically increases the cost of [[mint]] over time. Each unit of [[$V]] or [[$A]] ever minted — including [[$V]] burned by [[cyberlinks]] — permanently raises the cumulative supply floor and reduces the output of every subsequent [[mint]].

| totalSupply / halfLife | supplyDecay | cost multiplier |
|---|---|---|
| 0 | 1.000 | 1x |
| 0.5 | 0.707 | 1.4x |
| 1.0 | 0.500 | 2x |
| 2.0 | 0.250 | 4x |
| 3.0 | 0.125 | 8x |

The [[$A]] half-life (32B) is 8x larger than [[$V]] (4B). [[$V]] gets expensive 8x faster — writing to the graph ([[$V]]) is scarcer than influencing focus ([[$A]]).

[[$A]] is not burned — it remains in the [[neuron]] account and continuously weights their [[cyberlinks]] in the [[relevance machine]] via diffusion.

No oracle, [[governance]] vote, or external trigger required. [[scarcity]] increases automatically and continuously as the network is used.

### Input Parameters

| Parameter | [[$V]] | [[$A]] |
|---|---|---|
| baseAmount | 1,000,000,000 H | 100,000,000 H |
| basePeriod | 2,592,000 s (30 days) | 2,592,000 s (30 days) |
| halvingPeriod | 9,000,000 blocks | 9,000,000 blocks |
| supply half-life | 4,000,000,000 | 32,000,000,000 |
| halving floor | 0.01 | 0.01 |
| minimum [[mint]] threshold | 1,000 milli-units | 1,000 milli-units |

## Bandwidth Model

The [[bandwidth]] module governs [[$V]] consumption and throughput pricing.

Creating a [[cyberlink]] permanently burns [[$V]] from the [[neuron]] account. The amount burned per [[cyberlink]] is the current [[bandwidth price]], which adjusts dynamically based on network utilisation:

- when load is below target (10% of max block [[bandwidth]]): price falls, encouraging usage
- when load is above target: price rises, dampening demand without a mempool auction

The price adjusts every 5 blocks. Burned [[$V]] is gone permanently — it counts toward total cumulative supply in the [[mint]] decay curve, increasing [[scarcity]] for all future minters.

| Parameter | Default |
|---|---|
| Price adjustment period | 5 blocks |
| Base price | 0.25 V per [[cyberlink]] |
| Target network load | 10% of max block [[bandwidth]] |
| Max block [[bandwidth]] | 10,000 [[cyberlinks]] per block |

### Energy Grid

The grid module allows [[$V]] and [[$A]] to be routed to [[cosmwasm]] programs via energy routes. A [[neuron]] or contract can create a route that continuously directs their resource allocation to [[autonomous progs]]. Programs that receive routed [[$V]] can themselves create [[cyberlinks]] — enabling self-sustaining, autonomous [[knowledge graph]] expansion. Programs earning [[$BOOT]] execution fees create a direct incentive for operators to reinvest those rewards back into the [[staking]] → [[$H]] → [[$V]]/[[$A]] chain.

## End-to-End Token Flow

```
                    delegate
BOOT ─────────────────────────────────► H
  ▲                                     │
  │ staking rewards                     │ mint (burn H)
  │                                     │
  │                            ┌────────┴────────┐
  │                            ▼                 ▼
  │                            V                 A
  │                            │                 │
  │                            │ cyberlinks       │ focus weight
  │                            ▼                 ▼
  │                     knowledge graph    Diffusion
  │
  └── 80% execution fees ◄── Autonomous Programs ◄── Energy Routes (V/A)
```

1. acquire [[$BOOT]] — via secondary market, [[staking]] rewards, or airdrop
2. [[delegation]] of [[$BOOT]] → receive [[$H]] 1:1 — [[$BOOT]] earns [[staking]] rewards; [[$H]] is the liquid representation
3. [[mint]] [[$H]] → [[$H]] undergoes [[burn]] → receive [[$V]] and/or [[$A]] — quantity determined by supply decay curve (cumulative supply)
4. spend [[$V]] to write [[cyberlinks]] — permanent, content-addressed entries in the [[knowledge graph]]; price adjusts dynamically with block utilisation
5. hold [[$A]] to weight [[cyberlinks]] in GPU-computed diffusion — more [[$A]] = greater influence over what the graph surfaces as important
6. route [[$V]]/[[$A]] via the grid to power [[autonomous progs]] → programs earn 80% of [[$BOOT]] execution fees → reinvest back into step 1
7. deposit [[$H]] into [x/liquidity](https://github.com/cyberia-to/go-cyber/tree/main/x/liquidity) pools → receive pool [[tokens]] → earn farming rewards across configurable block schedules

## Economic Properties

### Everything costs stake

Because [[$V]] and [[$A]] can only be created by the [[burn]] of [[$H]] — which itself requires [[staking]] [[$BOOT]] — every unit of network resource has an explicit, on-chain opportunity cost denominated in committed stake. You cannot spam the [[knowledge graph]] without locking value into the network [[bostrom/security]].

### Continuous deflationary pressure

[[$V]] and [[$A]] issuance follows an exponential supply decay curve — every unit ever minted (including burned units) raises the cumulative supply floor and reduces the output of every subsequent [[mint]]. This creates a monotonically rising marginal cost for network resources without relying on hard supply caps. [[scarcity]] emerges from usage itself.

### Scarcity is proportional to impact

[[$V]] is scarcer than [[$A]] (lower base amount, smaller half-life). Writing to the graph ([[$V]]) is a final irreversible action that permanently expands the [[knowledge graph]] and costs [[consensus]] resources to process. Influencing focus ([[$A]]) is a continuous, reweightable state that costs GPU cycles. The 10x price difference and 8x half-life difference encode this distinction directly in the protocol.

### 80% execution fee return

[[cosmwasm]] smart contracts on [[bostrom]] return 80% of their execution fees directly to the program creator. This is hardcoded in the dmn module and creates a native revenue model for on-chain [[autonomous progs]] — a program that provides value to the network earns [[$BOOT]] proportional to how often it is called, with no intermediary taking the majority of the fee.

## Source References

All mechanics derived from:

- [x/resources](https://github.com/cyberia-to/go-cyber/blob/main/x/resources/keeper/keeper.go) — [[mint]] logic, halving, supply decay curve, maxPeriod
- [x/bandwidth](https://github.com/cyberia-to/go-cyber/blob/main/x/bandwidth/types/params.go) — [[bandwidth]] pricing and [[$V]] burn parameters
- [x/cyberbank](https://github.com/cyberia-to/go-cyber/tree/main/x/cyberbank) — [[$H]] [[mint]] on [[delegation]], [[burn]] on undelegation
- [x/rank](https://github.com/cyberia-to/go-cyber/tree/main/x/rank) — token-weighted diffusion (GPU/CUDA)
- [x/grid](https://github.com/cyberia-to/go-cyber/tree/main/x/grid) — [[$V]]/[[$A]] energy routing to [[autonomous progs]]
- [x/dmn](https://github.com/cyberia-to/go-cyber/tree/main/x/dmn) — 80% execution fee return mechanic
- [x/graph](https://github.com/cyberia-to/go-cyber/tree/main/x/graph) — [[cyberlink]] creation, [[$V]] and [[$A]] tracking
- [x/liquidity](https://github.com/cyberia-to/go-cyber/tree/main/x/liquidity) — [[automated market maker]], liquidity pools, farming rewards

[[bostrom]] mainnet — [[go-cyber]] v7.0.1 — February 2026
