---
tags: cyb, core
crystal-type: entity
crystal-domain: cyber
alias: cyb core, proof pipeline
stake: 13747295805047214
---
# Core

the proof pipeline — five [[Rust]] crates in a strict dependency chain that transform field arithmetic into authenticated state. remove any one and the system has no foundation. everything else in [[cyb]] is built on top of this chain.

```
nebu (field)  →  hemera (hash)  →  nox (VM)  →  zheng (proofs)  →  bbg (state)
```

## the five crates

### nebu — field arithmetic

the [[Goldilocks field]] $\mathbb{F}_p$ where $p = 2^{64} - 2^{32} + 1$. six operations: add, sub, mul, inv, eq, lt. plus [[NTT]] over $2^{32}$ roots of unity. every number in [[cyb]] is a nebu field element. every computation reduces to nebu operations. the field is the atom.

nebu is shared across 12 of 14 [[cyb/languages]] — only [[Bt]] (characteristic 2) needs its own field. see [[nebu]]

### hemera — hashing and trees

Poseidon2 sponge over [[nebu]]. takes field elements in, produces 4-element digests out. ~300 constraints in a [[stark]] proof (vs ~50,000 for Blake3). one hash function for the entire system: content addressing, Merkle trees, commitments, key derivation, verified streaming.

hemera gives [[particles]] their identity. every CID in the [[cybergraph]] is a hemera output. see [[hemera]]

### nox — virtual machine

sixteen deterministic reduction patterns over hemera-authenticated trees. five structural (axis, quote, compose, cons, branch), six field (add, sub, mul, inv, eq, lt), four bitwise (xor, and, not, shl), one hash. plus non-deterministic hint injection and five jets for verifier acceleration.

the execution trace IS the algebraic constraint system — no translation layer between program and proof. nox is simultaneously the structural IR that all [[cyb/languages]] compile through, the node runtime, and the composition tier for proof aggregation. see [[nox]]

### zheng — proof system

[[stark]] proofs over nox execution traces. WHIR polynomial commitments, SuperSpartan constraint satisfaction. every nox computation produces a proof of correct execution as a byproduct. recursive composition via field tower $\mathbb{F}_{p^3}$.

zheng verifies that a nox program ran correctly without re-executing it. this is what makes the [[cybergraph]] trustless — you don't trust the node, you verify the proof. see [[zheng]]

### bbg — authenticated state

the Big Badass Graph. stores the [[cybergraph]] with polynomial commitment indexes: edges by neuron, edges by particle, focus values, balances, token supply, cards. each index provides cryptographic completeness proofs — when you sync a namespace, you get mathematical proof nothing was withheld.

five layers: edge store (content-addressed, immutable) → neuron index → particle index → focus & balance → UTXO state (mutator set for privacy). see [[bbg]]

## the chain

each crate consumes only the one before it:

| crate | consumes | provides | enables |
|-------|----------|----------|---------|
| nebu | — | field arithmetic | every number |
| hemera | nebu | hashing, trees | every identity |
| nox | hemera | computation, proofs | every program |
| zheng | nox | verification | every trust claim |
| bbg | zheng | authenticated state | every graph query |

the pipeline boundary is between zheng and bbg: everything before bbg is computation, everything after is state. nox programs produce proofs via zheng; bbg stores the results with commitments that zheng can verify.

## what the core is NOT

the core is the proof pipeline. two additional crates complete the [[cyb/stack]] but branch off hemera rather than extending the chain:

- [[mudra]] — post-quantum crypto for [[neurons]] (confidentiality, key exchange, FHE). the agent-facing complement to hemera's content-facing identity
- [[radio]] — P2P transport (QUIC, verified streaming, gossip). hemera hashes replace Blake3 throughout

```
                    ┌→ mudra (crypto for agents)
nebu → hemera ──────┤
                    ├→ nox → zheng → bbg (proof pipeline)
                    └→ radio (transport for data)
```

see [[cyb/stack]] for all seven crates as a system. see [[cyb/os]] for the kernel they compose into. see [[cyb/architecture]] for the design they implement
