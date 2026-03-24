---
alias: blocks
tags: page
crystal-type: entity
crystal-domain: cyber
stake: 20339016742818876
diffusion: 0.0002453315929373196
springs: 0.00005004416225416235
heat: 0.00012289797727471304
focus: 0.00016225864059985585
gravity: 3
density: 0
---

Steps are the discrete time units of the [[cyber]] protocol, equivalent to blocks in traditional blockchains. Each step represents one tick of the consensus clock during which the network processes transactions and advances [[state]].

During a single step, [[validators]] collect pending [[signals]] from [[neurons]], order them deterministically, and apply the [[state transition function]] to produce the next version of the [[cybergraph]]. The result is committed to consensus and becomes the canonical [[state]].

Step duration is governed by the consensus parameters of the [[bostrom]] chain. Faster steps increase throughput and reduce latency for [[signal]] inclusion, while slower steps allow more time for validator coordination across geographic distances.

Each step carries a monotonically increasing height number. This height serves as the universal clock reference for all protocol operations including [[staking]] reward distribution, governance voting periods, and [[cyberank]] recalculation.

The sequence of steps from genesis to the current height constitutes the complete, auditable history of the [[cyber]] network. Any observer can replay this sequence to independently verify the current [[state]] of the [[cybergraph]].

Step finality in [[bostrom]] is achieved through Tendermint BFT consensus, meaning once a step is committed, it cannot be reverted under normal network conditions.

discover all [[concepts]]