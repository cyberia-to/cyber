---
tags: cybernomics
alias: volt, millivolt, volts, millivolts
crystal-type: entity
crystal-domain: economics
---
[[token]] of [[will]] in [[bostrom]]

denom: `millivolt`

## Role

[[$V]] is [[bandwidth]]. Creating a [[cyberlink]] costs [[$V]] proportional to the current dynamic [[bandwidth price]]. Each [[cyberlink]] is a permanent, content-addressed, directed edge in the on-chain [[knowledge graph]] connecting two [[ipfs]] CIDs.

## Issuance

[[$V]] is created by the [[burn]] of [[$H]] via [[mint]]. Early [[$V]] was issued via the original investmint mechanism; all new issuance goes through [[mint]].

| | |
|---|---|
| Circulating supply | ~2.2B millivolt |
| baseAmount | 1,000,000,000 H |
| Supply half-life | 4,000,000,000 |

## Price curve

The cost to [[mint]] 1 V grows exponentially with cumulative supply. Price doubles every 4B millivolt ever minted (including burned).

![mint price curve](https://jade-gentle-pony-196.mypinata.cloud/ipfs/QmddzXtyds43F2wMNHBVmjeB3DZtbuJrtro8p9hjdT1fgM)

## Bandwidth Pricing

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

## Burn sinks

- [[cyberlinks]]: every link permanently burns [[$V]] at the current [[bandwidth price]]
- [[burn fee on moving A and V]]: 2% burn on every [[$V]] transfer
- [[eternal cyberlinks]] (roadmap): [[burn]] [[$V]] for permanent weight boost

Burned [[$V]] counts toward cumulative supply — every burn makes the next [[mint]] more expensive.

[[bostrom/tokenomics]]