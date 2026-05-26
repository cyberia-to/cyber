---
tags: cyber
alias: mudra, मुद्रा, crypto primitives
crystal-type: entity
crystal-domain: cyber
---
post-quantum cryptographic primitives for [[neurons]]. mudra (मुद्रा — seal/gesture in Sanskrit) is to [[neurons]] what [[hemera]] is to [[particles]]: hemera gives content its identity and integrity (hashing, commitment, tree proofs); mudra gives agents their confidentiality and privacy (encrypting, exchanging keys, computing privately, distributing keys).

[[hemera]] answers: what exists, and how to verify it. mudra answers: who acts, and how to protect them.

## why no signatures or VRF

in a proof-native system, [[stark]] proofs replace both. a neuron proves `H(secret) = address` in zero knowledge — this IS a signature, just a more powerful one. every digital signature is a special case of a zero-knowledge proof of knowledge. similarly, a VRF computes `output = H(secret, input)` and proves correctness — the proof system handles this directly.

what proofs provide that signatures cannot: composability (prove arbitrary statements, not just key ownership), chargeability (every proof is metered), and universality (one mechanism for authentication, integrity, randomness, and metering).

the [[call]] mechanism in [[nox]] (pattern 16) makes this concrete: a neuron proves knowledge of its secret key without revealing it, both on-chain and off-chain. every message is proved and charged for — proof of delivery replaces signed delivery.

## the separation

proofs ([[zheng]]) handle: authentication, integrity, randomness, metering.
mudra handles: confidentiality, key agreement, private computation, key distribution.

these are orthogonal concerns. proofs verify and charge; mudra hides and shares.

## modules

| module | primitive | security assumption | what neurons do |
|--------|----------|-------------------|-----------------|
| kem | lattice KEM (ML-KEM) | Module-RLWE (NIST FIPS 203) | establish encrypted channels (interactive) |
| ctidh | dCTIDH (isogeny NIKE) | CSIDH (conjectured post-quantum) | establish encrypted channels (non-interactive) |
| aead | authenticated encryption | symmetric (Poseidon2 PRF + MAC) | encrypt channel traffic after key exchange |
| tfhe | fully homomorphic encryption | LWE | compute on encrypted data without decrypting |
| threshold | Shamir SSS, VSS, DKG | information-theoretic + hash | distributed key management, threshold decryption |

each module has its own security boundary. they share no cryptographic code with each other. [[hemera]] provides the PRF for authenticated encryption and commitments for verifiable secret sharing in the threshold module. [[nebu]] provides field arithmetic for lattice KEM and TFHE polynomial rings.

## the neuron lifecycle through mudra

```
neuron creates identity    → hemera (hash preimage)
neuron authenticates       → zheng proof of key knowledge
neuron exchanges keys      → kem (interactive) or ctidh (non-interactive)
neuron encrypts channels   → aead (Poseidon2-based)
neuron computes privately  → tfhe (homomorphic)
neuron coordinates         → threshold (distributed keys, DKG)
neuron produces randomness → zheng (VRF via proof of H(secret, input))
```

## dependency graph

```
nebu (field)
  ↓
hemera (hash)
  ↓
mudra (crypto) ← this repo
```

mudra is consumed at the protocol/node level — not part of the core proof pipeline (nebu → hemera → nox → zheng → bbg). it is the agent-facing complement to the content-facing hemera.

see [[lattice KEM]] for interactive key exchange, [[dCTIDH]] for non-interactive key exchange, [[TFHE]] for homomorphic encryption
