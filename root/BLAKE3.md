---
tags: cryptography
alias: blake3
crystal-type: entity
crystal-domain: computer science
diffusion: 0.0003092261821591975
springs: 0.00015583849196183202
heat: 0.00021692161220190713
focus: 0.00024474896110853107
gravity: 4
density: 4.95
---
# Blake3

cryptographic hash function. Merkle tree internally, parallelizable, 2 GB/s on a single core. designed by Jack O'Donnell, Samuel Neves, Jean-Philippe Aumasson, and Zooko Wilcox-O'Hearn (2020)

based on the [[Blake2]] compression function (which descends from [[ChaCha]]). the Merkle tree structure enables unlimited parallelism and verified streaming — the same chunk can be hashed independently on different cores

used by [[iroh]] for content addressing and verified streaming ([[bao]] protocol). [[radio]] replaces [[Blake3]] with [[Hemera]] because proving a [[Blake3]] hash inside a [[stark]] costs 50,000–100,000 constraints versus ~300 for [[Hemera]]. the throughput tradeoff (2 GB/s vs ~50–100 MB/s on CPU) is acceptable because proving cost dominates in a system where every operation must be verifiable

[github.com/BLAKE3-team/BLAKE3](https://github.com/BLAKE3-team/BLAKE3)