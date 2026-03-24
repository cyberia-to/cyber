---
tags: cyber, cip
alias: zheng, 証, proof system, cyber stark, cyber starks, Whirlaway implementation
crystal-type: entity
crystal-domain: cyber
subgraph: true
repo: ../zheng
exclude: ".claude/**, target/**, CLAUDE.md"
stake: 38544821775428340
diffusion: 0.0006607731092721277
springs: 0.0001060245233229204
heat: 0.0003001583874043206
focus: 0.00042222558911381646
gravity: 17
density: 0
---
Zheng is the [[proof system]] of the [[cyber]] protocol. It is a [[STARK]]-based proving engine built on the Whirlaway implementation, providing computational integrity guarantees for all state transitions in the [[cybergraph]].

When a [[neuron]] submits a [[cyberlink]] or any other state-changing operation, zheng generates a succinct proof that the transition follows protocol rules. [[Validators]] verify these proofs instead of re-executing every computation, dramatically reducing the cost of consensus.

The system relies on [[hemera]] as its algebraic hash, ensuring that all commitments within proofs are computed over the [[Goldilocks field]]. This tight integration between hash and proof system yields optimal performance.

Zheng enables recursive proof composition, allowing multiple state transitions to be batched and verified in a single step. This capability scales the protocol toward handling planetary-level knowledge graph operations.

As a transparent proof system, zheng requires no trusted setup ceremony. The security rests entirely on hash function collision resistance and the algebraic structure of the [[STARK]] protocol.

By separating proof generation from verification, zheng allows lightweight clients to trust the [[cybergraph]] state without running a full node, bringing verifiable knowledge to every device.

discover all [[concepts]]