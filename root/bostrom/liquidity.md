---
tags: module, bostrom
crystal-type: entity
crystal-domain: cyber
diffusion: 0.00015292113142548427
springs: 0.00017683885706268337
heat: 0.00019318079487391345
focus: 0.00016814838180633477
gravity: 3
density: 1.88
---
# Liquidity Module

[[bostrom]] uses a fork of the [[cosmos-sdk]] [tendermint/liquidity](https://github.com/tendermint/liquidity) module (v1.6.0-forced). it implements a hybrid AMM that combines an orderbook with constant-product pools. Orders are collected into batches and executed at a single uniform swap price per batch (Equivalent Swap Price Model). ESPM prevents front-running and reduces arbitrage compared to instant-execution AMMs.

## Key Properties

- pool type: constant product (x * y = k), only StandardLiquidityPool (type 1) supported — exactly two reserve coins
- swap matching: fractional matching within batch, executed at single clearing price (ESPM)
- fees: swap fee rate 0.3% (half reserved upfront, half deducted after execution), pool creation fee 40M boot → community pool, withdrawal fee rate 0%
- batch interval: every block (UnitBatchHeight = 1)
- max order amount: 10% of pool reserves per swap (MaxOrderAmountRatio)
- circuit breaker: emergency switch disables create/deposit/swap, withdrawals remain enabled

## Operations

| Operation | Description |
|---|---|
| create-pool | create pool with two coin denoms. pays PoolCreationFee to community pool. mints InitPoolCoinMintAmount pool coins |
| deposit | coins escrowed until batch execution. mint amount = min(supply × coinA/reserveA, supply × coinB/reserveB). remainder refunded |
| withdraw | pool coins escrowed, burned at batch execution. uses InputOutputCoins — bypasses [[cyberbank]] 2% energy burn. depleted pool reactivates on new deposit |
| swap | offer coin + half swap fee escrowed. minimum offer 100 units. order expires at end of current batch (CancelOrderLifeSpan = 0) |

## Pool Denomination

- PoolName: `{denomA}/{denomB}/{poolTypeId}` with denoms sorted alphabetically
- ReserveAccount: `crypto.AddressHash(PoolName)`
- PoolCoinDenom: `pool` + uppercase hex of `sha256(PoolName)`

## Interaction with Other Modules

- [[tokenfactory]] tokens can be used in pools
- IBC tokens (via [[teleport]]) can be pooled
- [[cyberbank]] 2% energy burn on [[$V]]/[[$A]] applies to deposits (regular SendCoins), withdrawals bypass it (InputOutputCoins)
- WASM bindings deprecated — legacy `ParseCustom`/`QueryCustom` still function, `Parse()`/`Query()` return nil

## Known Issues

see [[bostrom/liquidity roadmap]] for confirmed bugs and fix strategy:
0. 18-decimal ERC-style tokens malfunction
1. withdrawal stuck on certain pools
2. swap price display inconsistencies
3. missing price/volume indexing

## Interfaces

- [[warp]] — pool management UI (create, deposit, withdraw)
- [[teleport]] — send, bridge, swap
- [[nebula]] — token listing and discovery
- [[hub]] — tokens registry, states, bridges

## Specification

full module spec: [go-cyber/x/liquidity/spec/](https://github.com/cybercongress/go-cyber/tree/master/x/liquidity/spec)

## Related

- [[bostrom/liquidity roadmap]]
- [[aqua style dex]] — draft BIP for future replacement
- [[liquidity subsidy]] — draft CIP for senate-funded subsidies
- [[market making]] — Osmosis pools