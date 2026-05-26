---
tags: cyber, core
alias: Sec, privacy language, security language, anonymous computation
crystal-type: entity
crystal-domain: cyber
---
the privacy computation language. [[Curve]] type system for anonymous operations over the [[cybergraph]].

computation in Sec uses the commutative group action on supersingular curves: the [[genies]] algebra. the Curve type carries this semantics — the [[Trident]] compiler sees Curve operands and routes to [[genies]] regime automatically. no annotations needed.

## types

```
Curve            supersingular elliptic curve over F_q
Secret           exponent vector (secret key)
StealthAddress   receiver-anonymous address
RingSignature    one-of-n anonymous signature
BlindSignature   signer-blind signed message
VRFOutput        verifiable random value + proof
SharedSecret     derived shared secret (hemera digest)
```

## operations

| operation | Trident | what it computes |
|-----------|---------|-----------------|
| group_action(sk, pk) | Secret, Curve → Curve | isogeny walk |
| agree(sk, pk) | Secret, Curve → SharedSecret | non-interactive key exchange |
| stealth_send(sk, pk) | Secret, Curve → StealthAddress | anonymous addressing |
| ring_sign(msg, ring, sk) | Hash, [Curve], Secret → RingSignature | anonymous signing |
| vrf_eval(sk, input) | Secret, Hash → VRFOutput | verifiable randomness |
| vdf_prove(prev, T) | Hash, u64 → VDFProof | time proof |

## regime

Curve type → [[genies]] regime → Isogeny [[lens]] → [[zheng]] proves via F_q Brakedown

jets: jet_group_action, jet_isogeny_walk, jet_vrf_eval, jet_vdf_step, jet_secret_hash

## why a language

every time a [[neuron]] transacts without revealing identity, proves membership without revealing which member, delegates authority without exposing the chain, or generates verifiable randomness — it thinks in Sec. privacy is not a feature — it is a computation domain with its own algebra, its own [[lens]], and its own jets.

discover all [[concepts]]
