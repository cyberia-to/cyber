---
tags: cyb, core
crystal-type: entity
crystal-domain: cyber
alias: cyb stack, software stack, proof pipeline
---
# Stack

seven [[Rust]] crates that implement [[cyb]]. five form the [[cyb/core]] proof pipeline; two extend it with agent crypto and P2P transport. together they are the complete software foundation — everything else ([[cyb/os]], [[cyb/features]], [[cyb/apps]]) is built from these.

```
                    ┌→ mudra (crypto for agents)
nebu → hemera ──────┤
                    ├→ nox → zheng → bbg (proof pipeline)
                    └→ radio (transport for data)
```

## the seven crates

| # | crate | repo | role | depends on |
|---|-------|------|------|-----------|
| 1 | [[nebu]] | ~/git/nebu | [[Goldilocks field]] arithmetic + [[NTT]] | — |
| 2 | [[hemera]] | ~/git/hemera | Poseidon2 hash, Merkle trees, CIDs | nebu |
| 3 | [[nox]] | ~/git/nox | VM: 16 patterns + hint + 5 jets | hemera |
| 4 | [[zheng]] | ~/git/zheng | [[stark]] proofs: WHIR + SuperSpartan | nox |
| 5 | [[bbg]] | ~/git/bbg | authenticated state: indexes + commitments | zheng |
| 6 | [[mudra]] | ~/git/mudra | post-quantum crypto: KEM, CSIDH, TFHE, threshold | hemera |
| 7 | [[radio]] | ~/git/radio | P2P transport: QUIC, BAO streaming, gossip | hemera |

## proof pipeline (crates 1-5)

five crates in a strict dependency chain that transform field arithmetic into authenticated state. remove any one and the system has no foundation

```
nebu (field)  →  hemera (hash)  →  nox (VM)  →  zheng (proofs)  →  bbg (state)
```

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

### the chain

each crate consumes only the one before it:

| crate | consumes | provides | enables |
|-------|----------|----------|---------|
| nebu | — | field arithmetic | every number |
| hemera | nebu | hashing, trees | every identity |
| nox | hemera | computation, proofs | every program |
| zheng | nox | verification | every trust claim |
| bbg | zheng | authenticated state | every graph query |

the pipeline boundary is between zheng and bbg: everything before bbg is computation, everything after is state. nox programs produce proofs via zheng; bbg stores the results with commitments that zheng can verify.

## agent crypto (crate 6)

[[mudra]] branches off hemera. it handles what proofs cannot: confidentiality, key exchange, private computation.

| module | primitive | what neurons do |
|--------|----------|-----------------|
| kem | ML-KEM (lattice) | interactive encrypted channels |
| ctidh | dCTIDH (isogeny) | non-interactive key exchange via graph |
| aead | Poseidon2 PRF + MAC | encrypt channel traffic |
| tfhe | LWE | compute on encrypted data |
| threshold | Shamir SSS, DKG | distributed key management |

proofs ([[zheng]]) verify and charge. mudra hides and shares. orthogonal concerns.

## transport (crate 7)

[[radio]] branches off hemera. a fork of [[iroh]] where every hash runs through hemera instead of Blake3. 20× cheaper in [[stark]] proofs, one hash function end to end.

| stratum | what | crate |
|---------|------|-------|
| protocols | [[radio/blob]], [[radio/docs]], [[radio/gossip]], [[radio/willow]] | iroh-* |
| verified streaming | [[radio/bao]] (hemera Merkle trees) | cyber-bao |
| content identity | Poseidon2 sponge, compression, KDF | cyber-poseidon2 |
| networking | [[radio/endpoint]], [[radio/relay]], [[radio/hole-punching]] | iroh |

## what each crate enables

| crate | what becomes possible |
|-------|----------------------|
| nebu | all arithmetic. the [[Goldilocks field processor]] accelerates it in hardware |
| hemera | content addressing. [[particles]] get identity. trees get authentication |
| nox | all [[cyb/languages]]. programs compile to nox pattern trees. jets accelerate domain ops |
| zheng | trustless verification. the [[cybergraph]] does not require trusting nodes |
| bbg | completeness proofs. syncing a namespace proves nothing was withheld |
| mudra | agent privacy. [[neurons]] communicate confidentially and compute on encrypted data |
| radio | P2P connectivity. data moves between devices without centralized infrastructure |

## build order

the dependency chain determines the build order. nebu first, always. hemera next. then three independent branches (nox pipeline, mudra, radio) can proceed in parallel.

```
Phase 1:  nebu → hemera                    (foundation)
Phase 2:  nox ──────→ zheng → bbg          (proof pipeline)
          mudra                             (agent crypto)
          radio                             (transport)
Phase 3:  cyb/os                            (kernel + runtime)
Phase 4:  cyb/features                      (render, contracts)
Phase 5:  cyb/apps                          (portal, oracle, sigma...)
```

see [[cyb/core]] for the applications built on this stack. see [[cyb/os]] for the kernel. see [[cyb/architecture]] for the design
