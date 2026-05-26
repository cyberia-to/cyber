---
tags: cyber
alias: genies, commutative group action, isogeny cryptography, CSIDH, dCTIDH, cyber privacy
crystal-type: entity
crystal-domain: cyber
---
commutative group action over supersingular isogenies for [[cyber]]. genies is the privacy engine — where [[nebu]] proves truth, genies proves that truth was revealed selectively.

the one module with a foreign prime. F_q where q = 4·ℓ₁·ℓ₂·...·ℓₙ - 1. not because the design is incomplete, but because mathematics does not permit post-quantum commutative group actions over [[Goldilocks field|Goldilocks]].

## three properties

1. post-quantum security — no known quantum algorithm breaks the class group action
2. commutative group action — non-interactive protocols from one primitive
3. compact representation — public keys ~64 bytes

all three over Goldilocks — open problem in cryptography.

## privacy applications

| application | what it enables |
|-------------|-----------------|
| stealth addresses | receiver-anonymous payments |
| non-interactive key exchange | shared secret without interaction |
| verifiable random functions | deterministic randomness with proof |
| verifiable delay functions | time proofs (sequential computation) |
| threshold protocols | t-of-n key generation, signing |
| oblivious transfer | sender sends N, receiver gets 1 |
| blind signatures | signer signs without seeing message |
| ring signatures | sign as "one of group" anonymously |
| anonymous credentials | prove attributes without revealing identity |
| updatable encryption | re-encrypt without decrypting |

## nox integration

Layer 3 jets: jet_genies_action, jet_genies_dh, jet_genies_vrf, jet_genies_vdf, jet_genies_threshold, jet_genies_blind.

verification: isogeny computation over F_q, [[zheng]] proof folds into Goldilocks accumulator. shadow executes in its own field, proof lands in [[nebu]].

## dependency graph

```
nebu (F_p) — proof backbone, accumulator
  ↓
genies (F_q) ← this repo
  ↓
nox (jets)
  ↓
zheng (proof of correct privacy operation)
  ↓
bbg (private state: UTXO, mutator set)
```

discover all [[concepts]]
