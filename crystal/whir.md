---
tags: cyber, cryptography
alias: WHIR, whir protocol
crystal-type: entity
crystal-domain: cyber
---

WHIR: a lookup argument protocol based on the sum-check

## mechanism

WHIR allows a prover to demonstrate that a set of committed values all appear in a predefined table, without revealing which entries were looked up. the protocol reduces the lookup argument to a sum-check instance, leveraging the algebraic structure of [[polynomial]] commitments

## role in cyber

used in [[stark]] proof composition to efficiently verify that committed values appear in a table. this is essential for [[nox]] circuits, where complex computations are decomposed into table lookups for range checks, bitwise operations, and hash evaluations

## properties

the protocol achieves logarithmic verification complexity — the verifier's work scales with $O(\log n)$ where $n$ is the table size. this makes WHIR practical for large-scale proof systems where lookup tables contain millions of entries

WHIR composes cleanly with other sum-check-based protocols, enabling modular proof construction in the [[cyber]] [[computation]] stack

## relation to proof systems

lookup arguments are a critical building block in modern [[zero-knowledge]] proof systems. WHIR provides a sum-check-native approach that avoids the pairing-based assumptions of earlier lookup protocols, aligning with the [[stark]] philosophy of transparent, post-quantum security

see [[stark]], [[nox]], [[computation]], [[zero-knowledge]], [[cryptography]]
