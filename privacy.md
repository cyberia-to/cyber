---
tags: cyber, core, privacy
alias: private, privacy by design
crystal-type: pattern
crystal-domain: cyber
crystal-size: bridge
---
the right to compute on your own state without showing it. in a world where every query trains someone's model and every payment feeds someone's ledger, privacy is the condition for honest thought — a mind under observation optimizes for the observer

[[soft3]] treats privacy as physics, [[license|not policy]]: the field that proves also encrypts. prove the property, keep the number

# our stack

| layer | mechanism |
|-------|-----------|
| [[bbg]] | aggregate public, individual private — block-level proofs over a [[bbg/privacy\|mutator set]], commitment and nullifier polynomials |
| [[strata]] | the FHE regime: jali $R_q$ hosts encrypted arithmetic in the same [[nebu|Goldilocks field]] that proves |
| [[mudra]] | TFHE in the key ladder, isogeny-based exchange (dCTIDH) — no discrete log for Shor to unwind |
| [[zheng]] | hash-and-field proofs — verification without re-exposure |
| [[hemera]] | identity as a hash, not a name |
| [[lytics]] | measurement on-device: the observer never receives the raw signal |

# lessons of the field

a decade of cypherpunk engineering already paid for the big lessons. each system below earned one:

- [[privacy/monero]] — privacy must be the default, or the anonymity set collapses
- [[privacy/zcash]] — the strongest cryptography loses to opt-in economics
- [[privacy/aztec]] — private state and public consensus can share one settlement layer
- [[privacy/starknet]] — transparent hash-based proofs scale and survive quantum; trusted setups age badly
- [[privacy/aleo]] — execute off-chain, verify on-chain: the chain never needs to see the program run
- [[privacy/mina]] — recursion compresses history; verification cost can stay constant forever
- [[privacy/ergo]] — composable sigma protocols make privacy a toolbox, one proof per need

# the gaps we still close

encrypted state that stays computable ([[strata]]), proofs cheap enough for a pocket ([[honeycrisp]]), and an attention economy where [[focus]] is provable while reading stays unobserved — the aggregate is public knowledge, the individual is private life. that boundary is the design

[[security]] · [[bbg]] · [[$CYB]] · [[soft3]]
