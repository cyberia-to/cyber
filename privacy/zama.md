---
tags: privacy, crypto
alias: fhEVM, TFHE-rs, confidential contracts
crystal-type: entity
crystal-domain: crypto
crystal-size: atom
---
the company that made FHE programmable (TFHE-rs → fhEVM, protocol live 2025). smart contracts execute over TFHE ciphertexts — confidential balances, sealed bids, hidden game state — on ordinary EVM chains, with a co-processor doing the heavy homomorphic lifting and a threshold KMS holding the network decryption key as MPC shares. no single party can ever decrypt what the contracts compute on

the lesson: FHE left the laboratory, and it shipped fused with MPC — encrypted execution plus threshold keys — exactly the pairing the [[privacy trilateral]] derives from first principles. the same TFHE lineage runs through [[mudra]]'s key ladder and [[strata]]'s jali $R_q$: one torus, theirs over the EVM, ours over the [[nebu|Goldilocks field]] that also proves

[[privacy]] · [[privacy/xelis]] · [[privacy/arcium]] · [[mudra]]
