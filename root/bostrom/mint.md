---
tags: bostrom, cybernomics
alias: bostrom/mint, bostrom mint, energy mint
crystal-type: entity
crystal-domain: economics
stake: 19639691414667944
diffusion: 0.00016282257632298797
springs: 0.0012905364265434053
heat: 0.0009400500705659263
focus: 0.00065658223023772
gravity: 2
density: 3.22
---
# Mint

- Back to [[bostrom tokenomics]]

A [[neuron]] burns [[$H]] through [[mint]] to create [[$V]] or [[$A]]. The [[$H]] is sent to the [x/resources](https://github.com/cyberia-to/go-cyber/blob/main/x/resources/keeper/keeper.go) module, burned immediately and permanently. [[$V]] or [[$A]] are created in return and delivered to the [[neuron]] in the same block.

## The Price

The cost to [[mint]] 1 unit of [[$V]] or [[$A]] in [[$H]]:

```
price = baseAmount / supplyDecay
```

`baseAmount` is fixed per token (1B H for [[$V]], 100M H for [[$A]]). `supplyDecay` falls with every [[mint]] ever made. The price can only go up.

## Supply Decay

Every [[mint]] call computes a decay factor from the total cumulative supply of the resource (including burned units):

```
supplyDecay = 0.5 ^ (totalSupply / halfLife)

halfLife(V) = 4,000,000,000
halfLife(A) = 32,000,000,000
```

Each unit of [[$V]] or [[$A]] ever minted — including [[$V]] burned by [[cyberlinks]] — permanently raises the cumulative supply floor and reduces the output of every subsequent [[mint]].

| totalSupply / halfLife | supplyDecay | cost multiplier |
|---|---|---|
| 0 | 1.000 | 1x |
| 0.5 | 0.707 | 1.4x |
| 1.0 | 0.500 | 2x |
| 2.0 | 0.250 | 4x |
| 3.0 | 0.125 | 8x |

![mint price chart](https://jade-gentle-pony-196.mypinata.cloud/ipfs/QmUGrVHDSReH6AHi54xkz9JAD1LGtAs4zeTH5dm1sL9zfY)

The [[$A]] half-life (32B) is 8x larger than [[$V]] (4B). [[$V]] gets expensive 8x faster — writing to the graph ([[$V]]) is scarcer than influencing focus ([[$A]]).

[[$A]] is not burned — it remains in the [[neuron]] account and continuously weights their [[cyberlinks]] in the [[relevance machine]] via [[diffusion]].

No oracle, [[governance]] vote, or external trigger required. [[scarcity]] increases automatically and continuously as the network is used.

## Input Parameters

| Parameter | [[$V]] | [[$A]] |
|---|---|---|
| baseAmount | 1,000,000,000 H | 100,000,000 H |
| supply half-life | 4,000,000,000 | 32,000,000,000 |
| minimum [[mint]] threshold | 1,000 milli-units | 1,000 milli-units |

## Source

- [x/resources](https://github.com/cyberia-to/go-cyber/blob/main/x/resources/keeper/keeper.go) — [[mint]] logic, halving, supply decay curve, maxPeriod