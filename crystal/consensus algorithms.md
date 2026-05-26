---
tags: computer science, cyber
crystal-type: entity
crystal-domain: computer science
---

Protocols enabling distributed nodes to agree on a single state despite failures and adversaries. The foundation of decentralized [[computation]].

## problem

The Byzantine Generals Problem: how to reach agreement when some participants may be faulty or malicious. FLP impossibility theorem: deterministic consensus is impossible in asynchronous networks with even one crash fault.

## classical BFT

- PBFT (Practical Byzantine Fault Tolerance): tolerates up to f < n/3 Byzantine faults, three-phase protocol (pre-prepare, prepare, commit)
- Tendermint: BFT consensus used by Cosmos SDK and [[bostrom]], deterministic finality, round-based voting
- HotStuff: linearly scaling BFT, pipelining, used in Diem/Libra

## Nakamoto consensus

Proof of Work: Bitcoin's probabilistic consensus, longest chain rule, Sybil resistance through energy expenditure. Probabilistic finality.

## proof of stake

Validators [[staking]] tokens as collateral. Slashing for misbehavior. Used in [[cyber]], Ethereum 2.0. [[delegation]] transfers stake to trusted validators.

## connections

[[complexity theory]] bounds what consensus can achieve. [[formal verification]] proves safety and liveness properties. [[zero knowledge proofs]] enable privacy-preserving consensus. [[encryption]] secures message channels between nodes.