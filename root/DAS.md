---
tags: comp, data availability
crystal-type: entity
crystal-domain: comp
alias: data availability sampling
---
# DAS

data availability sampling is a technique that allows light nodes to verify that a block's data is available without downloading the entire block. the cost per verifier is O(sqrt(n)) instead of O(n), where n is the block data size

## mechanism

the block producer arranges data in a 2D matrix and applies Reed-Solomon [[erasure coding]] along both rows and columns. this extends a k x k data matrix into a 2k x 2k coded matrix. the producer commits to each row and column using a [[NMT]] (namespaced Merkle tree) or a polynomial commitment

light nodes sample random cells from the extended matrix and request the corresponding data and Merkle proofs. if a malicious producer has withheld any portion of the data, the erasure coding guarantees that the withholding is detectable with high probability after O(sqrt(n)) samples

## properties

the 2D Reed-Solomon structure ensures that if at least 50% of the extended data is available, the entire original data can be reconstructed. combined with random sampling, this gives exponential confidence: k random samples catch a withholding attack with probability 1 - (1/2)^k

## role in cyber

[[BBG]] uses DAS to ensure data availability across the network. validators and light nodes sample encoded chunks rather than downloading full blocks. [[structural sync]] coordinates the sampling protocol — each participant verifies local samples and contributes to the collective availability guarantee

---

see [[erasure coding]] for the Reed-Solomon foundation. see [[NMT]] for the commitment scheme. see [[BBG]] for the data availability layer. see [[structural sync]] for the sync protocol
