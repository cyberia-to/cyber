---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: proof of delivery, private messaging, cyber messaging, neuron communication
diffusion: 0.0002473412565973999
springs: 0.0017329386268169164
heat: 0.0012587964190080207
focus: 0.0008953115001454053
gravity: 7
density: 1.24
---
# communication

private messaging between [[neurons]] with cryptographic proof that messages arrive. the transport is [[radio]] (QUIC, hole-punching, relays). the encryption uses commutative key agreement (CSIDH). the delivery guarantee is a chain of [[stark]] proofs — one per hop — that recursively compose into a single proof: the message was delivered.

## shared secret

two [[neurons]] that have never communicated derive a shared secret from public data. CSIDH commutativity makes this possible without interaction.

```
SHARED SECRET DERIVATION
════════════════════════

Neuron A: secret a, public curve E_a = [a] · E₀
Neuron B: secret b, public curve E_b = [b] · E₀

Both curves are published as [[particles]] in the [[cybergraph]].

Shared secret:
  A computes: K = Hemera([a] · E_b) = Hemera([a]·[b]·E₀)
  B computes: K = Hemera([b] · E_a) = Hemera([b]·[a]·E₀)

  K_A = K_B                 by CSIDH commutativity

Derived keys:
  encrypt_key = Hemera(K ∥ "enc")
  mac_key     = Hemera(K ∥ "mac")
  nonce_seed  = Hemera(K ∥ "nonce" ∥ message_index)
```

the shared secret is computed once and reused for a session. each message gets a unique nonce derived from the message index — no nonce reuse, no state synchronization required.

A never contacts B. B never contacts A. both independently arrive at the same key K by reading each other's public curve from the graph. the graph itself is the key exchange medium.

## message structure

```
MESSAGE FORMAT
══════════════

header:
  sender_ephemeral: [F_p; 4]    one-time curve point (forward secrecy)
  message_index:    u64          sequence number
  hop_count:        u8           total relay hops planned
  ttl:              u8           remaining hops

payload:
  ciphertext:   [u8; ...]       AES-256-GCM(encrypt_key, nonce, plaintext)
  mac:          [F_p; 4]        Hemera(mac_key ∥ ciphertext)

routing (onion-encrypted):
  next_hop:     [F_p; 4]        encrypted address of next relay
  return_path:  [F_p; 4]        encrypted reverse route token
```

### forward secrecy

each message session uses an ephemeral CSIDH keypair. the sender generates a fresh secret a', computes E_a' = [a'] · E₀, and derives session keys from [a'] · E_b. if the sender's long-term secret is ever compromised, past session keys remain secure — the ephemeral secret was discarded after use.

## onion routing

the sender wraps the message in layers of encryption, one per relay hop. each relay peels one layer, learns only the next hop, and forwards the inner blob. no relay sees the full route or the plaintext.

```
ONION CONSTRUCTION (3 hops: R₁ → R₂ → R₃ → B)
═══════════════════════════════════════════════

Sender A knows relay public curves: E_R₁, E_R₂, E_R₃

Layer 3 (innermost):  enc(K_B,  plaintext ∥ "end")
Layer 2:              enc(K_R₃, layer_3 ∥ addr_B)
Layer 1:              enc(K_R₂, layer_2 ∥ addr_R₃)
Layer 0 (outermost):  enc(K_R₁, layer_1 ∥ addr_R₂)

where K_X = Hemera([a'] · E_X)     ephemeral shared secret with each hop

Relay R₁ receives layer_0:
  decrypts with K_R₁ → learns addr_R₂ + layer_1
  forwards layer_1 to R₂
  produces stark proof of correct forwarding

Relay R₂ receives layer_1:
  decrypts with K_R₂ → learns addr_R₃ + layer_2
  forwards layer_2 to R₃
  produces stark proof of correct forwarding

Relay R₃ receives layer_2:
  decrypts with K_R₃ → learns addr_B + layer_3
  forwards layer_3 to B
  produces stark proof of correct forwarding

Recipient B receives layer_3:
  decrypts with K_B → reads plaintext
  produces stark proof of receipt
```

each relay computes a CSIDH shared secret with the sender's ephemeral key. every relay sees exactly one address: the next hop. the sender's identity, the recipient's identity, and the message content are hidden from all relays.

## proof of delivery

each hop produces a [[stark]] proof attesting: "I received a valid encrypted blob, decrypted my layer correctly, and forwarded the result to the next address." the proofs chain:

```
PROOF OF DELIVERY
═════════════════

π₁ = stark(R₁ received blob, peeled layer, forwarded to R₂)
π₂ = stark(R₂ received blob, peeled layer, forwarded to R₃)
π₃ = stark(R₃ received blob, peeled layer, forwarded to B)
π_B = stark(B received blob, decrypted plaintext, MAC verified)

Chained verification:
  π_chain = stark(verify(π₁) ∧ verify(π₂) ∧ verify(π₃) ∧ verify(π_B))

Recursive composition:
  one proof (~100-200 KB) covers the entire route
  O(1) verification regardless of hop count
```

the sender publishes π_chain as a [[particle]] in the [[cybergraph]]. anyone can verify delivery happened. no one can read the message or learn the route.

### what the proof reveals

```
PUBLIC                              │ HIDDEN
────────────────────────────────────┼──────────────────────────────
message was delivered               │ sender identity
delivery route had N hops           │ recipient identity
all relays forwarded correctly      │ relay identities
MAC verification succeeded          │ message content
total latency (timestamp chain)     │ individual hop latencies
```

### incentive structure

relays earn [[focus]] for proven delivery. the proof of delivery is the claim — no proof, no payment. this creates an economic incentive to relay honestly:

- correct relay: earn focus, proof is valid
- drop the message: no proof, no payment
- tamper with content: stark proof fails, no payment, reputation penalty
- delay: timestamps in proof chain reveal latency, market prefers fast relays

## transport layer

[[radio]] handles the physical network: QUIC connections, NAT hole-punching, relay fallback. the communication protocol described here operates one layer above [[radio]]:

```
┌─────────────────────────────────────┐
│  cyber/communication                │ proof of delivery, onion routing
│  (this page)                        │ CSIDH key agreement, stark proofs
├─────────────────────────────────────┤
│  radio                              │ QUIC, hole-punching, relays
│  (iroh fork with Hemera)            │ verified streaming, blob transfer
├─────────────────────────────────────┤
│  network                            │ UDP/IP, physical transport
└─────────────────────────────────────┘
```

[[radio]] uses [[Hemera]] Merkle trees for verified streaming — content integrity is checked at the transport level. the communication layer adds privacy (onion encryption) and accountability (proof of delivery) on top.

## comparison

| property | email/SMTP | Signal | Tor | cyber/communication |
|----------|-----------|--------|-----|---------------------|
| end-to-end encryption | optional (PGP) | yes (Signal Protocol) | yes (onion layers) | yes (CSIDH + AES-256-GCM) |
| metadata privacy | no (headers exposed) | partial (server sees who) | yes (onion routing) | yes (onion routing) |
| delivery proof | no | delivery receipts (trust) | no | yes (stark chain) |
| post-quantum | no | partial (PQXDH) | no | yes (CSIDH + Hemera) |
| non-interactive key exchange | no (requires handshake) | no (requires prekeys) | no (circuit setup) | yes (CSIDH from graph) |
| relay incentive | none | none | volunteer | [[focus]] for proven delivery |
| censorship resistance | low (server-dependent) | medium (phone-dependent) | high (onion network) | high (any [[neuron]] can relay) |

## constraint costs

```
CSIDH shared secret:       ~50,000 constraints (isogeny evaluation in circuit)
AES-256-GCM decrypt:       ~10,000 constraints
Hemera MAC verify:             ~736 constraints
per-hop relay proof:       ~60,000 constraints total
recursive aggregation:     ~70,000 constraints (stark verifier)

3-hop delivery proof:
  3 × relay + 1 × receipt + 1 × recursive = ~320,000 constraints
  after recursive composition: one proof, ~100-200 KB
```

see [[cyber/identity]] for the authentication and anonymity layers, [[radio]] for the transport primitive, [[BBG]] for graph-level privacy, [[privacy trilateral]] for how ZK + FHE + MPC combine