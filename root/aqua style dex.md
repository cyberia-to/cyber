---
tags: bip
crystal-type: process
crystal-domain: cyber
status: draft
---
replace native [[cosmos-sdk]] [liquidity module](https://github.com/tendermint/liquidity) with [aqua](https://github.com/1inch/aqua)-style decentralized exchange

## current state

[[warp]] uses the tendermint liquidity module: classic constant-product AMM with liquidity locked in pools. capital is fragmented, strategies are rigid, tokens leave the wallet

## aqua approach

aqua is a registry-based shared liquidity layer. key differences:

- tokens stay in the LP wallet. aqua tracks virtual balances via allowance records
- one approval enables participation in unlimited strategies
- strategies are immutable once shipped. parameter updates happen through dock-and-ship without token transfers
- pull/push interface: apps pull maker tokens and push taker tokens during swap execution

this decouples token custody from strategy management. capital efficiency without custody risk

## migration

- deploy aqua-style dex as [[cosmwasm]] [[prog]]
- redesign [[teleport]] swap routing to use the new dex
- release liquidity from current [[warp]] implementation
- network upgrade with compulsory migration