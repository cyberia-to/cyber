---
tags: cyber, core
crystal-type: entity
crystal-domain: cyber
alias: signatures, signed, signing
---
cryptographic proof that a [[neuron]] authorized a [[signal]]. the [[spell]] produces the signature — without revealing itself — binding identity to action

a signature proves three things simultaneously: authenticity (the [[neuron]] who holds the [[spell]] produced this), integrity (the [[signal]] content has not been altered since signing), and non-repudiation (the signer cannot deny having signed)

in [[cyber]], every [[signal]] carries a signature. the [[cyberlink]] batch, the [[focus]] allocation, the [[pay]] transfer — each requires the [[neuron]]'s cryptographic commitment before the [[tru]] will process it. a [[signal]] without a valid signature is indistinguishable from noise

the signature scheme (Ed25519, Schnorr, or [[stark]]-compatible variants) determines the security assumptions. [[proof]]-carrying [[signals]] extend the signature concept: the [[stark]] proof itself is a signature — the prover signs the computation with a witness that the verifier checks without re-executing

the relationship between [[spell]], signature, and [[neuron]] is triangular: the spell is what you know, the signature is what you show, the neuron is what they see. possession of the spell implies the ability to produce signatures. loss of the spell means the neuron can never sign again — it becomes an inert record in the [[cybergraph]], observable but silent

see [[spell]] for the secret half. see [[proof]] for the generalized verification framework

discover all [[concepts]]
