---
tags: cyber, core
crystal-type: entity
crystal-domain: cyber
alias: validator, validation, validator set
---
[[neurons]] that propose and confirm [[steps]] in the [[cyber]] protocol. validators stake [[coins]] as collateral, run the [[tru]] computation engine, and collectively produce the append-only sequence of finalized [[state]] transitions

a validator's duties: collect pending [[signals]] from the mempool, order them into a candidate [[step]], execute all [[state]] transitions (including [[cyberank]] recomputation), produce a [[stark]] proof of valid transition, and broadcast the proposed step to peers. the step achieves [[finality]] when a supermajority of staked [[coins]] across the validator set signs the commit

the validator set is permissionless — any [[neuron]] that meets the minimum [[lock]] threshold can join. delegation allows neurons with smaller holdings to stake through an existing validator, sharing rewards proportionally. the total staked amount determines the validator's weight in [[consensus]]

validators earn rewards for honest participation: a share of newly [[minted]] [[coins]] and transaction fees collected from [[signals]] processed in each [[step]]. the reward rate scales with the fraction of total [[supply]] that is staked — higher participation yields lower per-unit rewards, balancing [[security]] against [[liquidity]]

dishonest behavior triggers slashing. a validator that signs conflicting steps at the same height (equivocation), goes offline for extended periods (downtime), or produces invalid [[state]] transitions loses a fraction of staked [[coins]] through forced [[burn]]. slashing makes attacks economically destructive for the attacker

the validator set collectively maintains the [[cybergraph]]'s integrity. every [[cyberlink]], every [[pay]] transfer, every [[token]] operation passes through validator consensus before achieving [[finality]]

discover all [[concepts]]
