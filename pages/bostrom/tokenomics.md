---
tags: bostrom, cybernomics, article
alias: bostrom tokenomics, bostrom token model
crystal-type: article
crystal-domain: economics
---
# Bostrom Tokenomics

[[bostrom]] launched November 5, 2021 at block 0. Node software: [[go-cyber]] v7.0.1. Live mainnet.

## The Four Tokens

[[bostrom]] separates four economic functions that most [[blockchains]] compress into a single [[token]]:

| Token | Role | Issuance |
|---|---|---|
| [[$BOOT]] | [[bostrom/security]] and [[governance]] | [[inflation]] (~1.09% annually) |
| [[$H]] | liquid representation of [[bostrom/staking]] | [[mint]] 1:1 on [[$BOOT]] [[bostrom/staking]], [[burn]] 1:1 on unstaking |
| [[$V]] | write access to the [[knowledge graph]] | [[burn]] of [[$H]] via [[mint]] |
| [[$A]] | [[relevance machine]] focus influence | [[burn]] of [[$H]] via [[mint]] |

Every token derives from the one above it. [[$H]] requires staked [[$BOOT]]. [[$V]] and [[$A]] require burned [[$H]]. Every unit of network resource has a provable, on-chain opportunity cost denominated in committed stake.

## The Learning Loop

The [[knowledge graph]] learns through economic commitments. Every token operation is part of a cycle that makes the graph more valuable over time.

### The cycle

1. [[neuron]] stakes [[$BOOT]] → receives [[$H]] → burns [[$H]] → receives [[$V]] or [[$A]]
2. [[neuron]] spends [[$V]] to create [[cyberlinks]] — each link is a [[costly signal]], an economic commitment that two [[particles]] are related
3. [[diffusion]] computes [[focus]] distribution across all [[particles]] on GPU, weighted by [[$A]] balances
4. [[cyberank]] measures particle quality — emerges from the graph structure without external votes

The result: a [[knowledge graph]] where the quality of search improves with every [[cyberlink]]. The more [[neurons]] link, the better [[cyberank]] gets, the more valuable [[$V]] and [[$A]] become.

## Energy mint

A [[neuron]] burns [[$H]] through [[mint]] to create [[$V]] or [[$A]]. The [[$H]] is sent to the [x/resources](https://github.com/cyberia-to/go-cyber/blob/main/x/resources/keeper/keeper.go) module, burned immediately and permanently. [[$V]] or [[$A]] are created in return and delivered to the [[neuron]] in the same block.

### The Price

The cost to [[mint]] 1 unit of [[$V]] or [[$A]] in [[$H]]:

```
price = baseAmount / supplyDecay
```

`baseAmount` is fixed per token (1B H for [[$V]], 100M H for [[$A]]). `supplyDecay` falls with every [[mint]] ever made. The price can only go up.

### Supply Decay

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

![mint price chart](https://jade-gentle-pony-196.mypinata.cloud/ipfs/QmUGrVHDSReH6AHi54xkz9JAD1LGtAs4zeTH5dm1sL9zfY)

The [[$A]] half-life (32B) is 8x larger than [[$V]] (4B). [[$V]] gets expensive 8x faster — writing to the graph ([[$V]]) is scarcer than influencing focus ([[$A]]).

[[$A]] is not burned — it remains in the [[neuron]] account and continuously weights their [[cyberlinks]] in the [[relevance machine]] via diffusion.

No oracle, [[governance]] vote, or external trigger required. [[scarcity]] increases automatically and continuously as the network is used.

### Input Parameters

| Parameter | [[$V]] | [[$A]] |
|---|---|---|
| baseAmount | 1,000,000,000 H | 100,000,000 H |
| supply half-life | 4,000,000,000 | 32,000,000,000 |
| minimum [[mint]] threshold | 1,000 milli-units | 1,000 milli-units |

## Fees

### Live

- [[burn fee on moving A and V]] — 2% burn on every [[$V]] and [[$A]] transfer. Speculators pay a tax that permanently reduces supply.
- [[collect fee on moving A and V]] — 1% fee on [[$V]] and [[$A]] transfers directed into reward pools for [[staking on particles]] and [[staking on cyberlinks]].
- [x/liquidity](https://github.com/cyberia-to/go-cyber/tree/main/x/liquidity) — 0.3% swap fee (retained in pool reserves), 40M [[$BOOT]] pool creation fee (community pool).

### Coming next

Approved by [[governance]], shipping in the next upgrade:

- [[burn gas in H]] — all transaction gas fees paid in [[$H]] instead of [[$BOOT]]
- [[fixed fee on H burn]] — 2% fee on every [[$H]] [[burn]] operation

After this upgrade [[$H]] becomes the sole token that gets burned across the protocol.

### On the roadmap

Under design, planned for future upgrades:

- [[eternal cyberlinks]] — [[burn]] [[$V]] to permanently boost the weight of a [[cyberlink]]
- [[eternal particles]] — [[burn]] [[$A]] to permanently boost the weight of a [[particle]]

## Energy Grid

The grid module allows [[$V]] and [[$A]] to be routed to [[cosmwasm]] programs via energy routes. A [[neuron]] or contract can create a route that continuously directs their resource allocation to [[autonomous progs]]. Programs that receive routed [[$V]] can themselves create [[cyberlinks]] — enabling self-sustaining, autonomous [[knowledge graph]] expansion.

[[cosmwasm]] contract execution fees return 80% directly to the program creator (hardcoded in the dmn module). Programs earning [[$BOOT]] execution fees create a direct incentive for operators to reinvest those rewards back into the [[bostrom/staking]] → [[$H]] → [[$V]]/[[$A]] chain.

## Token Flow

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
  │                  cyberlinks │                 │ focus weight
  │                  (burn V)   │                 │
  │                            ▼                 ▼
  │                     knowledge graph ◄── Diffusion (GPU)
  │                            │
  │                        cyberank
  │                            │
  └── 80% exec fees ◄── Autonomous Programs ◄── Energy Routes (V/A)
```

1. acquire [[$BOOT]] — via secondary market, [[bostrom/staking]] rewards, or airdrop
2. [[bostrom/staking]] of [[$BOOT]] → receive [[$H]] 1:1
3. [[mint]]: [[burn]] [[$H]] → receive [[$V]] and/or [[$A]] — quantity determined by supply decay curve
4. spend [[$V]] to write [[cyberlinks]] — price adjusts dynamically with block utilisation
5. hold [[$A]] to weight [[cyberlinks]] in GPU-computed [[diffusion]]
6. [[diffusion]] computes [[focus]] → [[cyberank]] emerges from graph structure
7. route [[$V]]/[[$A]] via the grid to power [[autonomous progs]] → programs earn 80% of [[$BOOT]] execution fees → reinvest into step 1

## Why Tokens Grow

### Supply decay is irreversible

Every [[mint]] makes the next [[mint]] more expensive. Every [[cyberlink]] burns [[$V]] and adds to the cumulative supply floor. There are no resets, no rebases, no governance overrides. The cost curve is monotonically rising and embedded in the protocol math.

### The graph gets more valuable

As [[neurons]] create [[cyberlinks]], the [[knowledge graph]] accumulates structured, provable knowledge. A more valuable graph attracts more [[neurons]], who create more [[cyberlinks]], which demands more [[$V]] and [[$A]]. Demand grows while supply gets scarcer.

### Writing is scarcer than reading

[[$V]] half-life (4B) is 8x smaller than [[$A]] half-life (32B). Writing to the graph — permanent, irreversible, [[consensus]]-verified — gets expensive 8x faster than influencing focus. The protocol values creation over attention.

### Speculation feeds the machine

The 2% [[burn fee on moving A and V]] ensures that every speculative transfer of [[$V]] or [[$A]] permanently destroys supply. Trading activity directly increases [[scarcity]] for all participants.

### Everything costs stake

[[$V]] and [[$A]] can only be created by the [[burn]] of [[$H]], which requires [[bostrom/staking]] [[$BOOT]]. You cannot interact with the [[knowledge graph]] without locking value into network [[bostrom/security]]. Spam is economically impossible.

## Source References

- [x/resources](https://github.com/cyberia-to/go-cyber/blob/main/x/resources/keeper/keeper.go) — [[mint]] logic, halving, supply decay curve, maxPeriod
- [x/bandwidth](https://github.com/cyberia-to/go-cyber/blob/main/x/bandwidth/types/params.go) — [[bandwidth]] pricing and [[$V]] burn parameters
- [x/cyberbank](https://github.com/cyberia-to/go-cyber/tree/main/x/cyberbank) — [[$H]] [[mint]] on [[bostrom/staking]], [[burn]] on undelegation
- [x/rank](https://github.com/cyberia-to/go-cyber/tree/main/x/rank) — token-weighted diffusion (GPU/CUDA)
- [x/grid](https://github.com/cyberia-to/go-cyber/tree/main/x/grid) — [[$V]]/[[$A]] energy routing to [[autonomous progs]]
- [x/dmn](https://github.com/cyberia-to/go-cyber/tree/main/x/dmn) — 80% execution fee return mechanic
- [x/graph](https://github.com/cyberia-to/go-cyber/tree/main/x/graph) — [[cyberlink]] creation, [[$V]] and [[$A]] tracking
- [x/liquidity](https://github.com/cyberia-to/go-cyber/tree/main/x/liquidity) — [[automated market maker]], liquidity pools, farming rewards

[[bostrom]] mainnet — [[go-cyber]] v7.0.1 — February 2026
