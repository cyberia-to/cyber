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
auth = stark_proof(∃ x : Hemera(x) = neuron_address)
```

every [[cyberlink]] carries a [[stark]] proof that the author knows the preimage of their [[neuron]] address. the chain verifies the proof. it never sees the secret. it never sees a signature.

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

the [[neuron]] provides its secret as a witness via `hint` (Layer 2). [[nox]] evaluates the lock script and produces a [[stark]] proof that the script executed correctly. the proof goes on-chain. the secret stays private.

### verification

any verifier checks the [[stark]] proof. cost: ~70,000 [[nox]] patterns with jets. constant regardless of what was proven. the verifier learns one fact: someone who knows the preimage authorized this [[cyberlink]].

## programmable identity

lock scripts are [[nox]] programs. the hash preimage check is the default, the simplest case. the same mechanism supports:

| pattern | lock script logic |
|---------|------------------|
| single owner | `Hemera(witness) == address` |
| multisig (m-of-n) | m valid preimages from n committed hashes |
| timelock | preimage valid AND current_time > unlock_time |
| delegation | preimage of delegate OR preimage of owner |
| recovery | any 3-of-5 trusted [[neuron]] preimages |

one mechanism. no new cryptography per pattern. the lock script is a [[nox]] program; the proof is a [[stark]].

## the [[neptune]] precedent

[[neptune]] (Alan Szepieniec, COSIC/KU Leuven) is the first blockchain to replace signatures entirely with [[stark]] proofs of lock script execution. launched mainnet February 2025. their stack:

- Tip5 hash (arithmetization-oriented, over [[Goldilocks field]])
- [[Triton VM]] (stark-native execution)
- lock scripts instead of signatures
- lattice KEM for encryption only (Module-RLWE over Goldilocks)

[[cyber]] inherits the paradigm with its own primitives: [[Hemera]] instead of Tip5, [[nox]] instead of Triton VM. same field. same idea. different hash, different VM, same elimination of signatures.

## stark constraints

```
Hemera hash:          ~300 constraints (vs ~25,000 for SHA-256)
lock script verify:   ~70,000 constraints (with jets)
recursive composition: O(1) verification for O(N) links
```

a [[stark]] proof of [[Hemera]] preimage knowledge is ~100-200 KB. larger than an ECDSA signature (64 bytes). the tradeoff: post-quantum security from genesis, programmable spending conditions, recursive aggregation. N proofs collapse into one.

## anonymous cyberlinks

the [[cybergraph]] is public: [[particles]], links, aggregate weights, [[focus]] vector. authorship of individual links is hidden. a [[neuron]] proves it is valid and has [[stake]], without revealing which neuron it is.

### the circuit

the [[neuron]] constructs a [[stark]] proof covering four constraints:

```
ANONYMOUS CYBERLINK CIRCUIT (~13,000 constraints)
══════════════════════════════════════════════════

PUBLIC INPUTS:
  source:     [F_p; 4]         source particle hash
  target:     [F_p; 4]         target particle hash
  weight:     F_p              stake amount committed to link
  nullifier:  [F_p; 4]         unique link identifier
  bbg_root:   [F_p; 4]         current BBG state root

PRIVATE WITNESS (via hint):
  secret:     [F_p; 4]         neuron preimage
  stake:      F_p              neuron stake amount
  membership_path: [...]       polynomial evaluation proof

CONSTRAINTS:

1. Identity:   Hemera(secret) ∈ neuron_set          ~1,000 (WHIR membership)
   prove the secret hashes to a registered neuron address
   without revealing which address

2. Stake:      stake(Hemera(secret)) ≥ weight        ~1,000 (WHIR lookup)
   prove the neuron has sufficient stake
   without revealing total stake or neuron identity

3. Nullifier:  nullifier == Hemera(secret ∥ source ∥ target)    ~300
   deterministic: same neuron + same particle pair = same nullifier
   reveals duplicate links, conceals author

4. Freshness:  nullifier ∉ spent_set                 ~3,000 (SWBF check)
   prove this nullifier has not been used before
   uses the sliding-window bloom filter from BBG Layer 4
```

the graph sees: `link(source, target, weight)` and `nullifier`.
the graph does not see: which neuron created the link.

### the privacy boundary

this follows the [[BBG]] privacy boundary specification:

```
PUBLIC                          │ PRIVATE
────────────────────────────────┼────────────────────────────
edges exist (A → B)             │ who created the edge
aggregate weight per edge       │ individual stake contributions
focus distribution (π vector)   │ which neurons shaped it
nullifiers (anti-spam)          │ neuron identity behind nullifier
```

the [[mutator set]] (AOCL + SWBF) tracks which nullifiers have been spent. addition records and removal records share zero structural similarity — unlinkability is architectural, following the same pattern [[BBG]] uses for private transfers.

### ranking on anonymous links

[[tri-kernel]] computes [[focus]] from the aggregate graph topology and edge weights. authorship is irrelevant to ranking — only the sum of weights per edge matters.

```
focus = tri-kernel(graph_topology, edge_weights)
```

an observer sees: [[particle]] A is linked to [[particle]] B with total weight W.
an observer does not see: W = w₁ + w₂ + w₃ (three [[neurons]], each contributing their [[stake]]).

### selective disclosure

a [[neuron]] can optionally reveal authorship of specific links while keeping others anonymous. the mechanism: publish the secret-derived nullifier derivation path for chosen links. this is a one-way door — once revealed, authorship is permanent. anonymous by default, transparent by choice.

range proofs extend this further: "my total stake in this subgraph exceeds threshold T" is provable without revealing the exact amount or which specific links carry it. this enables reputation and governance without deanonymization.

## encryption

authentication and anonymity operate on hashes and proofs alone — no algebraic structure beyond [[Goldilocks field]]. encryption is different. when two [[neurons]] need to exchange private data (encrypted messages, shared secrets, stealth addresses), they need key agreement: a protocol where two parties derive a shared secret from their respective keys.

### the problem

key agreement requires mathematical structure that a hash function cannot provide. [[Hemera]] maps inputs to outputs — it has no trapdoor, no commutativity, no homomorphism. these are features for identity (one-way is the point), but limitations for encryption (two-way communication requires shared structure).

### [[lattice KEM]] (interactive)

Module-RLWE (Ring Learning With Errors) over [[Goldilocks field]]. the same field as [[Hemera]], [[nox]], and [[stark]] verification — native arithmetic, no field conversion.

```
LATTICE KEM PROTOCOL
════════════════════

Setup:
  Ring R = Z_p[x] / (x^64 + 1)        cyclotomic polynomial, degree 64
  Module dimension: 4×4 over R
  Field: p = 2^64 - 2^32 + 1           Goldilocks

keygen():
  secret s ← small_distribution(R^4)
  public A ← uniform(R^{4×4})
  public b = A·s + e                    e ← error_distribution
  return (sk=s, pk=(A, b))

enc(pk, message):
  r ← small_distribution(R^4)
  ciphertext_1 = A^T · r + e'
  ciphertext_2 = b^T · r + e'' + encode(message)
  return (c1, c2)

dec(sk, c1, c2):
  message = decode(c2 - s^T · c1)
  return message
```

this is the [[neptune]] approach: the receiver publishes a lattice public key, the sender encrypts with it. post-quantum secure. the receiver decrypts with their secret key. limitation: the receiver must publish their public key first — interactive.

use cases: encrypting [[cyberlink]] metadata so only the intended [[neuron]] can read annotations, private [[particle]] delivery, encrypted [[spell]] parameters.

### isogeny-based key exchange (non-interactive)

CSIDH (Commutative Supersingular Isogeny Diffie-Hellman) and its optimized variant [[dCTIDH]] enable non-interactive key agreement. the unique property: commutativity.

```
CSIDH KEY AGREEMENT
═══════════════════

Setup:
  E₀: supersingular elliptic curve over F_p
  Class group action: [a] · E₀ = E_a    (secret isogeny)

Alice:  secret a → public E_a = [a] · E₀
Bob:    secret b → public E_b = [b] · E₀

Shared secret:
  Alice computes: [a] · E_b = [a] · [b] · E₀
  Bob computes:   [b] · E_a = [b] · [a] · E₀

  [a]·[b]·E₀ = [b]·[a]·E₀               commutativity
```

commutativity means two [[neurons]] derive a shared secret from each other's public data without any message exchange. this enables:

- stealth addresses: sender creates a [[cyberlink]] that only the intended recipient can detect and decrypt, without prior communication
- non-interactive key exchange: two [[neurons]] that have never communicated share a secret derived from public graph data
- anonymous channels: the shared secret reveals nothing about which [[neurons]] are communicating

tradeoffs: CSIDH is slower than [[lattice KEM]] (~5x for [[dCTIDH]]-2048 vs [[ML-KEM]]). the isogeny assumption is less studied than lattice assumptions — SIDH was broken in 2022, though CSIDH survived those specific attacks. active research area.

### privacy layers

| layer | function | primitive | assumption |
|-------|----------|-----------|------------|
| authentication | prove [[neuron]] validity | [[stark]] proof of [[Hemera]] preimage | hash collision resistance |
| anonymity | hide [[cyberlink]] authorship | ZK set membership + [[mutator set]] nullifiers | hash collision resistance |
| encryption (interactive) | private [[neuron]]-to-[[neuron]] data | [[lattice KEM]] (Module-RLWE over Goldilocks) | Module-RLWE hardness |
| encryption (non-interactive) | stealth addresses, anonymous channels | CSIDH / [[dCTIDH]] | isogeny class group action |
| computation privacy | compute on encrypted [[cybergraph]] data | [[TFHE]] over [[Goldilocks field]] | LWE hardness |
| distributed trust | prevent single-party compromise | threshold MPC with Shamir sharing | honest majority |

the first two layers require only hashes and proofs. the last four introduce additional assumptions — each carefully chosen to operate natively over [[Goldilocks field]] arithmetic. see [[privacy trilateral]] for how ZK + FHE + MPC combine to cover each other's blind spots. see [[BBG]] for the complete graph privacy architecture.

## what this means

the [[signer]] page describes the complexity of universal signing: pluggable curves, pluggable schemes, derivation paths, address formats per chain. identity in [[cyber]] reduces to: one hash function, one VM, one proof system. a [[neuron]] is a hash. authorization is a proof. anonymity is a proof of set membership. everything else follows.

see [[Hemera]] for the hash primitive, [[cyber/nox]] for the VM, [[cyber/proofs]] for stark verification, [[cyber/security]] for formal guarantees
