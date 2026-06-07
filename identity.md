---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: signatureless identity, hash-based identity, identity primitive, identity tiers, authentication regimes, neuron identity
---

# identity

a [[neuron]]'s identity is the [[Hemera]] hash of its public key — no registration, no authority, collision-free across the planet. see [[cybergraph/reference/identity]].

identity is uniform across every regime in which a neuron operates. the mechanism that authenticates a message varies with the trust boundary being crossed; the identity behind that message does not.

## the axiom

identity ≠ signature. every neuron at every level holds one keypair. the discipline of authentication varies across regimes; the identity model stays the same.

## three transport regimes

| tier | scope | mechanism | cost per message | safe because |
|------|-------|-----------|------------------|--------------|
| A | intra-process | capability handle granted at actor spawn | zero | rust type and memory safety form the perimeter |
| B | inter-process on a host | one-time DH → Poly1305 MAC, key bound to Secure Enclave | 50-100 ns | symmetric MAC, machine-pinned session key |
| C | inter-host | post-quantum signature, batched Merkle-root amortization for streams | ~10 ns/msg at batch 1000 | full adversarial WAN crypto |

## tier A — capability discipline

inside a single OS process, kernel memory protection and rust type/memory safety form one trust domain. cryptographically authenticating intra-process messages would authenticate memory to itself.

the parent actor grants typed capability handles at spawn. possession of the handle proves the holder was authorized at construction. unix-fd discipline applied to the actor layer: the kernel checks `open()` once, not every `read()`.

## tier B — symmetric MAC on host

across processes on the same host, kernel memory protection separates trust domains while leaving cross-process messages unauthenticated. the cost budget admits a per-message MAC, not a per-message signature.

the pattern: one DH key exchange at connection establishment derives a session secret. the secret lives in [[honeycrisp]]'s Secure Enclave so the host kernel cannot extract it. each message carries a Poly1305 tag over the body. verification is symmetric and constant-time.

## tier C — full signature on WAN

across hosts the network is adversarial. each message that becomes a [[cyberlink]] carries a post-quantum signature from [[mudra]]/seal. streams amortize by signing the [[hemera]] Merkle root of the last N messages and attaching the signature to message N. per-message overhead amortizes to ~10 ns at N = 1000.

## tier selection

the caller writes `send(target_neuron, message)`. [[soma]] holds a locality oracle that resolves the target to a regime: same-process → A, same-host → B, remote → C. the transport selects the matching authentication discipline. the call site stays uniform across regimes.

## cybergraph as the trust boundary

[[cybergraph]] admits only tier-C-authenticated [[cyberlinks]] for persistence. tier A and tier B messages stay ephemeral by definition. every cyberlink in the graph carries a full post-quantum signature; this invariant is what makes the graph a public record.

## verification erases authentication

the level of authentication shrinks as the level of verification grows. tier A signing-free coordination is safe inside surroundings that are verified by the language ([[rune]]), provable by construction ([[trident]]), proven ([[nox]]), or witnessed ([[zheng]]). [[soft3]]'s progressive verification stack earns the right to drop runtime authentication inside its own trust domain.

## implementation surface

| concern | where |
|---------|-------|
| axiom and tier classification | this page |
| crypto primitive catalog per tier | [[mudra]]/specs |
| tier A capability dispatch, tier B host transport, locality oracle | [[soma]] |
| tier C wire transport, batched signature amortization | [[radio]] |
| admission rule (tier-C only) | [[cybergraph]] |

[[cyb]] is downstream: it holds a neuron identity for its user and routes everything it sends through [[soma]]'s tier selector.
