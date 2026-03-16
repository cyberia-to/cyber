---
tags: cyber
alias: mudra, मुद्रा, crypto primitives
crystal-type: entity
crystal-domain: cyber
subgraph: true
repo: ../mudra
exclude: ".claude/**, target/**, CLAUDE.md"
---
post-quantum cryptographic primitives for [[neurons]]. mudra (मुद्रा — seal/gesture in Sanskrit) is to [[neurons]] what [[hemera]] is to [[particles]]: hemera gives content its identity and integrity (hashing, commitment, tree proofs); mudra gives agents their voice and privacy (signing, encrypting, coordinating, computing privately).

[[hemera]] answers: what exists, and how to verify it. mudra answers: who acts, and how to protect them.

## modules

| module | primitive | security assumption | what neurons do |
|--------|----------|-------------------|-----------------|
| sig | post-quantum signatures (ML-DSA) | Module-LWE (NIST FIPS 204) | sign p2p messages, validator attestations |
| kem | lattice KEM (ML-KEM) | Module-RLWE (NIST FIPS 203) | establish encrypted channels (interactive) |
| ctidh | dCTIDH (isogeny NIKE) | CSIDH (conjectured post-quantum) | establish encrypted channels (non-interactive) |
| aead | authenticated encryption | symmetric (Poseidon2 PRF + MAC) | encrypt channel traffic after key exchange |
| tfhe | fully homomorphic encryption | LWE | compute on encrypted data without decrypting |
| threshold | Shamir SSS, VSS, DKG | information-theoretic + hash | distributed key management, threshold signing |
| vrf | verifiable random function | signature + hash | leader election, unbiasable randomness beacons |

each module has its own security boundary. they share no cryptographic code with each other. [[hemera]] provides commitments for verifiable secret sharing in the threshold module and the PRF for authenticated encryption. [[aurum]] provides field arithmetic for lattice KEM, signatures, and TFHE polynomial rings.

## the neuron lifecycle through mudra

```
neuron creates identity    → hemera (hash preimage)
neuron signs messages      → sig (ML-DSA)
neuron exchanges keys      → kem (interactive) or ctidh (non-interactive)
neuron encrypts channels   → aead (Poseidon2-based)
neuron computes privately  → tfhe (homomorphic)
neuron coordinates         → threshold (distributed keys, DKG)
neuron participates in     → vrf (leader election, randomness)
  consensus
```

on-chain, [[nox]] replaces traditional signatures with [[stark]] proofs — the [[hint]] mechanism lets a neuron prove knowledge of its secret key without revealing it. mudra handles everything off-chain: p2p networking, validator coordination, encrypted delivery, private computation.

## dependency graph

```
aurum (field)
  ↓
hemera (hash)
  ↓
mudra (crypto) ← this repo
```

mudra is consumed at the protocol/node level — not part of the core proof pipeline (aurum → hemera → nox → zheng → bbg). it is the agent-facing complement to the content-facing hemera.

see [[lattice KEM]] for interactive key exchange, [[dCTIDH]] for non-interactive key exchange, [[TFHE]] for homomorphic encryption
