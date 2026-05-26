---
tags: cryptography, cyber
crystal-type: entity
crystal-domain: cyber
alias: TFHE, torus fully homomorphic encryption, fully homomorphic encryption
---
torus fully homomorphic encryption — a scheme enabling computation on encrypted data

in [[cyber]], TFHE enables private [[cyberlinks]] and confidential [[focus]] computation without revealing plaintext content. [[neurons]] can submit encrypted claims about [[relevance]], and the [[tru]] can process them while they remain ciphertext

how it works: data is encoded as elements of the real torus $\mathbb{T} = \mathbb{R}/\mathbb{Z}$. homomorphic gates (AND, OR, XOR) operate on ciphertext with bootstrapping to reset noise after each operation. this allows arbitrary circuits on encrypted inputs

applications in cyber:
- private [[cyberlinks]] — link [[particles]] without revealing the connection publicly
- confidential [[focus]] — compute [[cyberank]] over encrypted graph segments
- sealed [[knowledge]] — [[neurons]] contribute to collective [[intelligence]] while preserving data sovereignty

TFHE is one of several FHE schemes. its advantage is fast bootstrapping: each gate operation refreshes the ciphertext in milliseconds, making it practical for real-time [[consensus]] workloads

the integration path: [[stark]] proofs verify that encrypted computations were performed correctly, combining [[zero-knowledge]] with homomorphic [[encryption]]

see [[cryptography]], [[encryption]], [[zero-knowledge]], [[stark]], [[cyberlinks]]
