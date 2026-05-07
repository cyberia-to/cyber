---
alias: steps, block
tags: cyber, core
crystal-type: entity
crystal-domain: cyber
crystal-size: enzyme
---
one tick of [[consensus]] [[time]]. [[signals]] enter, achieve [[finality]], and the [[tru]] recomputes [[cyberank]] from the new [[state]]

a step is the discrete unit of [[time]] in the [[cyber]] protocol. the continuous flow of external events collapses into an ordered sequence of steps, each producing a deterministic [[state]] transition agreed upon by all [[validators]].

during each step, the [[tru]] collects pending [[signals]] from the mempool, orders them, validates each [[signature]] and [[gas]] payment, and executes the resulting [[state]] changes. [[pay]], [[lock]], [[mint]], [[burn]], [[update]], and [[cyberlink]] operations all resolve within the boundaries of a single step.

step height is a monotonically increasing integer. it serves as the universal clock for the [[cybergraph]] — every event, every [[token]] balance change, every [[cyberlink]] creation is timestamped by the step in which it achieved [[finality]].

at the end of each step, the [[tru]] recomputes [[cyberank]] over the updated [[cybergraph]]. new [[cyberlinks]] shift rank flows between [[particles]], and the resulting scores determine search relevance until the next step completes.

[[validators]] propose steps in rotation according to [[consensus]] rules. a step achieves [[finality]] when a supermajority of staked [[coins]] signs the commit. from that moment, the [[state]] transition becomes irreversible.

step duration targets a fixed interval but may vary with network conditions. shorter steps increase throughput; longer steps increase the [[time]] available for [[signal]] propagation across [[validators]]. the protocol balances responsiveness with decentralization.

[[will]] and [[attention]] regenerate fractionally with each step, restoring a [[neuron]]'s capacity to create [[cyberlinks]] and allocate [[cyberank]] over [[time]].

discover all [[concepts]]
