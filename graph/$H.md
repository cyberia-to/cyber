---
tags: cybernomics
alias: hydrogen
crystal-type: entity
crystal-domain: economics
stake: 15505983061356966
---
[[token]] of [[fuel]] in [[bostrom]]

denom: `hydrogen` (codebase: `scyb`)

## Role

[[$H]] is the liquid [[staking]] derivative of [[$BOOT]] and the primary [[token]] of the [[bostrom]] network. [[neurons]] hold, display, and transact with [[$H]]. The network's total value is expressed as the sigma of all [[$H]] in circulation.

## Issuance

```
delegate 1000 BOOT  →  mint 1000 H
undelegate 1000 BOOT  →  burn 1000 H
```

| | |
|---|---|
| Total supply | ~297T |
| % of [[$BOOT]] staked | 62% |

## Uses

1. [[mint]] input — [[burn]] [[$H]] to [[mint]] [[$V]] or [[$A]]
2. [[bostrom/liquidity]] — traded on the built-in [[automated market maker]] ([x/liquidity](https://github.com/cyberia-to/go-cyber/tree/main/x/liquidity) module), deposited into liquidity pools, or used in any [[cosmwasm]] contract

[[$H]] does not earn [[staking]] rewards itself. The underlying staked [[$BOOT]] continues to earn rewards. [[$H]] is the spendable proof that [[$BOOT]] is at stake.

[[bostrom/tokenomics]]
