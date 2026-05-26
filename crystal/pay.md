---
alias: transfer, send
tags: cyber, core
crystal-type: process
crystal-domain: cyber
crystal-size: enzyme
---
move [[coins]] between [[neurons]]. atomic — both balances update in one [[step]], or neither does. the simplest [[signal]] on the [[cybergraph]]

every [[neuron]] holds a balance of [[coins]] denominated in native [[tokens]]. a pay [[signal]] transfers an exact amount from sender to receiver within a single [[step]], guaranteeing atomicity through [[consensus]] execution.

the [[tru]] validates three conditions before applying a pay: the sender possesses sufficient balance, the [[signature]] matches the sending [[neuron]], and the [[gas]] fee covers execution cost. failure on any condition reverts the entire operation.

pay carries zero side effects on the [[cybergraph]] structure. it changes balances, consumes [[gas]], and produces a receipt — nothing else. this purity makes pay the foundation on which all other [[signals]] build.

batched pays allow a single [[neuron]] to distribute [[coins]] to many recipients within one [[step]]. multi-send operations fan out from a single [[signature]], reducing overhead for reward distribution and [[delegation]] flows.

pay is the entry point for every economic loop in the [[cyber]] protocol. before a [[neuron]] can [[lock]], [[mint]], [[burn]], or create [[cyberlinks]], it must first receive [[coins]] through a pay.

the irreversibility of pay follows from [[finality]]. once the [[step]] commits, the transfer becomes part of the permanent [[state]] and the balances reflect the new reality across all [[validators]].

discover all [[concepts]]
