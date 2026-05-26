---
tags: cyber, language
alias: Tok, token language, resource language, resource logic
crystal-type: entity
crystal-domain: cyber
---
the resource language. conservation laws over tokens — mint, burn, transfer, stake, and the sum invariants that make computation costly

| Op | Action |
|---|---|
| `mint(amount, denom)` | Create new tokens (governance-authorized) |
| `burn(amount, denom)` | Destroy tokens irreversibly |
| `transfer(from, to, amount)` | Move tokens between [[neurons]] |
| `stake(amount, validator)` | Lock tokens for [[focus]] generation |
| `unstake(amount)` | Begin unbonding period |
| `link(ν, p, q, τ, a, v)` | Create [[cyberlink]] — moves tokens from wallet to edge ([[UTXO]]) |
| `withdraw(link_id)` | Reclaim tokens from a [[cyberlink]] position |
| `conserve(inputs, outputs)` | Verify sum invariant: Σ inputs = Σ outputs |

every [[cyberlink]] is a [[UTXO]] with conviction (τ, a). creating a link moves tokens from wallet to edge — computation costs something. [[focus]] is conserved: the sum over all [[particles]] equals 1. every allocation is a real choice: directing attention to one [[particle]] directs it away from all others

the conservation invariant is enforced by [[zheng]] — the [[proof]] guarantees that no tokens are created or destroyed within a state transition. this is the same mechanism that prices [[cyber/channel]] interactions: the mutual ledger maintains balance_A + balance_B = deposit throughout the channel lifetime

## the golden standard

four tokens define the resource algebra of [[cyber]]:

| token | role |
|---|---|
| [[CYB]] | governance + linking weight |
| [[HYDROGEN]] | stake, delegation |
| [[VOLT]] | energy — compute access |
| [[AMPERE]] | bandwidth — rate of [[cyberlink]] submission |

stake → [[focus]] regeneration → bandwidth capacity → [[cyberlink]] creation → [[knowledge]]. the economic structure of the [[cybergraph]] IS the permission system. no passwords, no API keys — only tokens and their conservation laws

## why Tok is irreducible

remove Tok and the remaining thirteen languages can compute anything — but nothing costs anything. spam is free. [[focus]] has no scarcity. [[karma]] has no meaning. the [[cybergraph]] accumulates noise instead of [[knowledge]]

no other language provides:
- sum invariants (Σ in = Σ out) as a native algebraic constraint
- irreversible consumption (the `confirm` decision primitive)
- scarcity as a computational property (bounded [[focus]], bounded [[bandwidth]])

Tok is to economics what [[Seq]] is to time — the language that makes a dimension real rather than simulated

## proof path

Tok compiles to [[Trident]] for settlement. every token operation is a [[field]] arithmetic constraint: balance checks are range proofs, conservation is a sum constraint, UTXO transitions are Merkle updates. the [[proof]] guarantees that the economic rules hold — no trust required

see [[cyb/languages]] for the complete language set. see [[cyb/multiproof]] for the proving architecture. see [[cyber/channel]] for how Tok prices bilateral computation