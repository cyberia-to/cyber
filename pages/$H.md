---
tags: cybernomics
alias: hydrogen
crystal-type: entity
crystal-domain: economics
---
[[token]] of [[fuel]] in [[bostrom]]

denom: `hydrogen` (referred to as `scyb` throughout the codebase — the original name, short for staked CYB)

[[hydrogen]] is the liquid [[staking]] derivative of [[$BOOT]] and the primary [[token]] of the [[bostrom]] network. While [[$BOOT]] is the base [[bostrom/infrastructure/security]] layer, [[hydrogen]] is what [[neurons]] actually hold, display, and transact with. The network's total value is expressed as the sigma of all [[hydrogen]] in circulation, making [[hydrogen]] the canonical unit of account for the ecosystem.

[[hydrogen]] is issued solely through [[$BOOT]] [[delegation]]; destroyed solely through [[$BOOT]] undelegation.

```
delegate 1000 BOOT  →  mint 1000 H
undelegate 1000 BOOT  →  burn 1000 H
```

[[hydrogen]] has two uses:

1. [[mint]] input — [[burn]] to [[mint]] [[volt]] or [[amper]]
2. [[cyber/liquidity]] — deposited into farm contracts, traded on the built-in [[automated market maker]], or used in any [[cosmwasm]] contract

[[hydrogen]] does not earn [[staking]] rewards itself. The underlying staked [[$BOOT]] continues to earn rewards for the delegator. [[hydrogen]] is the spendable, transferable proof that the corresponding [[$BOOT]] is at stake.

[[bostrom/tokenomics]]
