---
tags: cyber, core
alias: blocks, height
crystal-type: entity
crystal-domain: cyber
---
# block

one tick of a [[network]]. the header (~232 bytes) is what every node needs; it commits to the [[BBG]] root. [[sealing]] is the block height at which a [[signal]] becomes final.

a block is shared time. it is not a [[neuron]]'s own [[step]] — that is the signer's chain. many signers can land in one block; one signer can have many steps before any of them seals.

[[research/oikos|oikos]] implies each neuron has a home [[network]]. that home still produces **blocks** (shared ticks of *that* graph). the signer's **steps** remain private to the key. one neuron, one home network; the two clocks stay distinct.

[[epoch]] is a pack of blocks, not a block.

discover all [[concepts]]
