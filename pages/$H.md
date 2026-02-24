---
tags: cybernomics
alias: hydrogen
crystal-type: entity
crystal-domain: economics
---
[[token]] of [[fuel]] in [[bostrom]]

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


[[bostrom/tokenomics]]
