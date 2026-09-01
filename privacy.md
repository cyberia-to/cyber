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
| [[bbg]] | aggregate public, individual private — block-level proofs over a [[bbg privacy\|mutator set]], commitment and nullifier polynomials |
| [[strata]] | the FHE regime: jali $R_q$ hosts encrypted arithmetic in the same [[nebu|Goldilocks field]] that proves |
| [[mudra]] | TFHE in the key ladder, isogeny-based exchange (dCTIDH) — no discrete log for Shor to unwind |
| [[zheng]] | hash-and-field proofs — verification without re-exposure |
| [[hemera]] | identity as a hash, not a name |
| [[lytics]] | measurement on-device: the observer never receives the raw signal |

# lessons of the field

a decade of cypherpunk engineering already paid for the big lessons. each system earned one — read the table as a history: the field learned to default, then to prove, then to forget, and by 2025 it learned to clear markets, satisfy regulators, and compute blind:

| system | since | breakthrough | the lesson, kept |
|--------|-------|--------------|------------------|
| [[monero]] | 2014 | ring signatures · RingCT · FCMP++ toward full-chain membership | defaults — an opt-in anonymity set collapses; the set must be everyone |
| [[zcash]] | 2016 | first zk-SNARK money · Halo 2 ended ceremonies · Tachyon scales the pool | proves — and the strongest cryptography still loses to opt-in economics |
| [[grin]] | 2019 | mimblewimble cut-through, no addresses | forgets — history shrinks to commitments; interactive UX taxes adoption |
| [[ergo]] | 2019 | composable sigma protocols | composes — one small proof per need, assembled per contract |
| [[zano]] | 2019 | confidential assets · Zarcanum hidden-amount PoS | stakes — consensus weight proven without being shown |
| [[semaphore]] | 2020 | anonymous membership + nullifier signaling | belongs — identity is membership without a name |
| [[mina]] | 2021 | recursive constant-size chain | compresses — one proof stands for unbounded history |
| [[starknet]] | 2021 | hash-based STARKs at scale | survives — transparent proofs outlive quantum and ceremonies |
| [[aztec]] | 2021 | private rollup on public settlement | settles — private state and public consensus share one layer |
| [[aleo]] | 2024 | off-chain execution, on-chain verification | hides the run — the chain never sees the program execute |
| [[xelis]] | 2024 | homomorphic encrypted balances in accounts | encrypts the account — balances live as ciphertexts |
| [[penumbra]] | 2024 | sealed-intent batch DEX · threshold aggregate decryption | clears on aggregates — individual intent stays sealed |
| [[namada]] | 2024 | one shielded set for all assets · shielded rewards | subsidizes — the anonymity set is a public good, pay to grow it |
| [[neptune cash]] | 2025 | mutator sets · recursive STARK state | accumulates — membership without rings, post-quantum |
| [[nockchain]] | 2025 | zkPoW: mining produces STARK proofs | works — the miners' watts become proofs |
| [[privacy pools\|privacy pools]] | 2025 | association sets on Ethereum | dissociates — compliance by proof, identity intact |
| [[zama]] | 2025 | fhEVM: contracts on ciphertexts · threshold KMS | computes blind — programmable FHE shipped fused with MPC |
| [[arcium]] | 2025 | MPC network as encrypted co-processor | shares — many parties compute, no one holds the input |

read the table against the [[privacy trilateral]]: zcash, mina, starknet, aztec, aleo, semaphore and nockchain grew the ZK vertex; monero, grin, zano, namada, neptune and xelis fought for the anonymity set; zama and xelis opened the FHE vertex; penumbra, arcium and zama's threshold keys put MPC into production; privacy pools made the whole triangle legible to regulators. every system in the table roots its guarantees in mathematics; TEE chains root theirs in a vendor's silicon, which is why the table ends here

# the gaps we still close

encrypted state that stays computable ([[strata]]), proofs cheap enough for a pocket ([[honeycrisp]]), and an attention economy where [[focus]] is provable while reading stays unobserved — the aggregate is public knowledge, the individual is private life. that boundary is the design

[[security]] · [[bbg]] · [[$CYB]] · [[soft3]]
