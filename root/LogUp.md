---
tags: cyber, core
crystal-type: pattern
alias: logarithmic lookup, lookup argument
---

# LogUp

A logarithmic derivative lookup argument used in [[starks]] and zero-knowledge proof systems.

LogUp reduces table lookup verification to a single randomized check using logarithmic derivatives.
The prover demonstrates that a multiset of queried values is contained in a predefined lookup table.
This technique replaces expensive permutation arguments with summation-based polynomial identities.
Efficiency gains make LogUp suitable for circuits with millions of lookup gates.
Applications include range checks, hash function decomposition, and memory consistency proofs.
Integration with [[cyber]] proof infrastructure enables scalable verification of [[cyberlink]] validity.

discover all [[concepts]]
