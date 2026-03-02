---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: signatureless identity, hash-based identity, identity primitive
---
# identity

a [[neuron]] proves itself by demonstrating knowledge of a secret that hashes to its address. no [[signature]] scheme. no elliptic curves. no lattices. one hash, one proof.

```
neuron_secret → Hemera(neuron_secret) = neuron_address
auth = STARK_proof(∃ x : Hemera(x) = neuron_address)
```

every [[cyberlink]] carries a [[STARK]] proof that the author knows the preimage of their [[neuron]] address. the chain verifies the proof. it never sees the secret. it never sees a signature.

## why

traditional identity requires a [[signature]] scheme: a mathematical structure (elliptic curve, lattice, hash tree) that binds a public key to a private key and produces a verifiable tag on each message. every scheme carries assumptions. every assumption is an attack surface.

| scheme | assumption | quantum status |
|--------|-----------|----------------|
| ECDSA/secp256k1 | discrete log on elliptic curves | broken by Shor |
| Ed25519 | discrete log on twisted Edwards | broken by Shor |
| BLS | pairing on BLS12-381 | broken by Shor |
| ML-DSA (Dilithium) | Module-LWE | post-quantum, 2.4 KB signatures |
| FN-DSA (Falcon) | NTRU lattice | post-quantum, needs float sampling |
| SLH-DSA (SPHINCS+) | hash-only | post-quantum, 8-50 KB signatures |

[[cyber]] eliminates the entire column. the only assumption is collision resistance of [[Hemera]] — the same assumption the rest of the protocol already requires.

## mechanism

### address generation

```
1. neuron generates a random secret s (256 bits of entropy)
2. neuron_address = Hemera(s)
3. the address is public. the secret is kept.
```

the address IS the [[Hemera]] output. 64 raw bytes. no prefix, no encoding.

### authentication

when a [[neuron]] creates a [[cyberlink]], it runs a lock script on [[nox]]:

```
lock_script(witness):
  assert Hemera(witness) == neuron_address
  return 0  // success
```

the [[neuron]] provides its secret as a witness via `hint` (Layer 2). [[nox]] evaluates the lock script and produces a [[STARK]] proof that the script executed correctly. the proof goes on-chain. the secret stays private.

### verification

any verifier checks the [[STARK]] proof. cost: ~70,000 [[nox]] patterns with jets. constant regardless of what was proven. the verifier learns one fact: someone who knows the preimage authorized this [[cyberlink]].

## programmable identity

lock scripts are [[nox]] programs. the hash preimage check is the default, the simplest case. the same mechanism supports:

| pattern | lock script logic |
|---------|------------------|
| single owner | `Hemera(witness) == address` |
| multisig (m-of-n) | m valid preimages from n committed hashes |
| timelock | preimage valid AND current_time > unlock_time |
| delegation | preimage of delegate OR preimage of owner |
| recovery | any 3-of-5 trusted [[neuron]] preimages |

one mechanism. no new cryptography per pattern. the lock script is a [[nox]] program; the proof is a [[STARK]].

## the [[neptune]] precedent

[[neptune]] (Alan Szepieniec, COSIC/KU Leuven) is the first blockchain to replace signatures entirely with [[STARK]] proofs of lock script execution. launched mainnet February 2025. their stack:

- Tip5 hash (arithmetization-oriented, over [[Goldilocks field]])
- [[Triton VM]] (STARK-native execution)
- lock scripts instead of signatures
- lattice KEM for encryption only (Module-RLWE over Goldilocks)

[[cyber]] inherits the paradigm with its own primitives: [[Hemera]] instead of Tip5, [[nox]] instead of Triton VM. same field. same idea. different hash, different VM, same elimination of signatures.

## STARK constraints

```
Hemera hash:          ~300 constraints (vs ~25,000 for SHA-256)
lock script verify:   ~70,000 constraints (with jets)
recursive composition: O(1) verification for O(N) links
```

a [[STARK]] proof of [[Hemera]] preimage knowledge is ~100-200 KB. larger than an ECDSA signature (64 bytes). the tradeoff: post-quantum security from genesis, programmable spending conditions, recursive aggregation. N proofs collapse into one.

## what this means

the [[signer]] page describes the complexity of universal signing: pluggable curves, pluggable schemes, derivation paths, address formats per chain. identity in [[cyber]] reduces to: one hash function, one VM, one proof system. a [[neuron]] is a hash. authorization is a proof. everything else follows.

see [[Hemera]] for the hash primitive, [[cyber/nox]] for the VM, [[cyber/proofs]] for STARK verification, [[cyber/security]] for formal guarantees
