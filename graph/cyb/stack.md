---
tags: cyb, core
crystal-type: entity
crystal-domain: cyber
alias: cyb stack, software stack
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

the strict chain from field to state. see [[cyb/core]] for detailed descriptions.

```
nebu          field elements, NTT
  ↓
hemera        hash(field elements) → 4-element digest
  ↓
nox           reduce(authenticated tree) → execution trace
  ↓
zheng         prove(trace) → ~100-200 KB proof
  ↓
bbg           store(state + commitments) → verifiable graph
```

each stage's output is the next stage's input. the chain is linear — no stage skips another.

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

see [[cyb/core]] for why these five are irreducible. see [[cyb/os]] for the kernel built on top. see [[cyb/architecture]] for the design
