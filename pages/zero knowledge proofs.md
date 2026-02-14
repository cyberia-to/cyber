tags:: computer science, cryptography

# zero knowledge proofs

A cryptographic protocol where a prover convinces a verifier that a statement is true while revealing nothing beyond the truth of the statement itself.

## properties

- completeness: an honest prover convinces an honest verifier
- soundness: a dishonest prover fails to convince (except with negligible probability)
- zero knowledge: the verifier learns only that the statement is true

## constructions

- ZK-SNARKs (Succinct Non-interactive Arguments of Knowledge): short proofs, fast verification, require trusted setup. Used in Zcash, rollups
- ZK-STARKs (Scalable Transparent Arguments of Knowledge): transparent setup (no trusted ceremony), quantum-resistant, larger proofs. StarkWare
- Groth16: efficient SNARK construction, widely deployed
- PLONK: universal and updatable trusted setup, flexible circuit design
- Bulletproofs: short proofs without trusted setup, used in Monero

## applications

- privacy: shielded transactions, private smart contracts
- scalability: validity rollups (ZK-rollups) compress thousands of transactions into one proof
- identity: proving attributes (age, membership) without revealing the underlying data
- [[cyber]]: zero knowledge proofs enable private [[knowledge graph]] queries and verifiable computation

## foundations

Rooted in [[complexity theory]] (interactive proof systems, IP = PSPACE). Related to [[encryption]] as a branch of [[cryptography]]. Circuits compiled by specialized [[compilers]].
