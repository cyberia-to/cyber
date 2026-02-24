---
tags: bip
crystal-type: process
crystal-domain: cyber
status: draft
---
replace current [[warp]] implementation with aqua-style decentralized exchange

aqua-style: concentrated liquidity AMM with order book semantics, inspired by [neutron dex](https://docs.neutron.org/neutron/modules/dex/messages)

steps

- release liquidity from current [[warp]] implementation
- deploy aqua-style dex as [[cosmwasm]] [[prog]]
- redesign [[teleport]] swap routing to use the new dex
- network upgrade with compulsory migration
