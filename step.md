---
tags: cyber, core
alias: steps
crystal-type: entity
crystal-domain: cyber
---
# step

the signer's own clock. each [[neuron]] as **author** numbers its [[signal|signals]] `0, 1, 2…` and chains them with `prev`. a missing number is a missing signal; two signals with the same step are equivocation.

a step is not the network's time. that is a [[block]]. a step belongs to one signer; a block belongs to a [[network]].

[[oikos|oikos]] is the household: each signing [[neuron]] has a **home network** — its own book and graph. steps tick on that home chain. publishing into another network does not give you that network's clock; it still gives you steps on yours.

the same word in older pages sometimes meant "consensus tick". in the protocol now: **step = the neuron's chain**, **block = the network's tick**.

discover all [[concepts]]
