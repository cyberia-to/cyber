---
tags: cyber, cybernomics
crystal-type: relation
crystal-domain: economics
stake: 2331572612390110
diffusion: 0.00010722364868599256
springs: 0.0008534258085490684
heat: 0.0006235697474094725
focus: 0.00043435351638960567
gravity: 0
density: 3.1
---

Transferring [[staking]] power from one neuron to another, enabling token holders to participate in consensus and governance without running validator infrastructure.

## mechanism

A delegator bonds tokens to a chosen validator (hero in [[cyber]], [[bostrom]]). The validator's voting power increases proportionally. Rewards flow back to delegators minus the validator's commission. Tokens remain owned by the delegator but are locked for the unbonding period.

## role in proof of stake

[[consensus algorithms]] in proof-of-stake networks rely on delegation to concentrate stake among competent validators while preserving broad participation. Delegation lowers the barrier to securing the network.

## risks

- slashing: if the validator misbehaves (double signing, extended downtime), delegated tokens are partially destroyed
- centralization: stake concentrating in few validators reduces Byzantine fault tolerance
- opportunity cost: bonded tokens are illiquid during the unbonding period

## governance

Delegated stake carries governance voting power. Validators vote on proposals on behalf of their delegators unless the delegator votes independently (vote override).

## connections

Core mechanism of [[staking]] economics in [[cyber]] and [[bostrom]]. Governed by [[consensus algorithms]] (Tendermint BFT). Delegation patterns visible in the [[knowledge graph]] as neuron-to-hero edges.