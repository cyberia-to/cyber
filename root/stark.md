---
tags: cyber, core, cryptography
alias: starks, STARK, STARKs, stark proofs
crystal-type: entity
crystal-domain: cyber
---
# stark

Scalable Transparent Argument of Knowledge. the proof system of [[cyber]]. every computation in the protocol — from a single [[cyber/link]] to a chained delivery receipt to a trillion-parameter inference — produces a stark proof of correct execution

transparent: no trusted setup ceremony. security rests entirely on collision resistance of a hash function ([[Hemera]]). no elliptic curves, no pairings, no toxic waste. post-quantum secure by construction

scalable: prover runs in $O(N)$ time (linear in computation size). verifier runs in $O(\log^2 N)$ time. proof size is $O(\log^2 N)$ — polylogarithmic, independent of what was proven

## architecture

[[cyber]] uses multilinear starks via the Whirlaway architecture:

$$\text{nox trace} \xrightarrow{\text{encode}} f(x_1, \ldots, x_{n+m}) \xrightarrow{\text{WHIR}} C \xrightarrow{\text{SuperSpartan}} \pi$$

three layers compose into one pipeline:

| layer | component | role |
|---|---|---|
| IOP | [[SuperSpartan]] | interactive oracle proof via sumcheck over CCS constraints |
| PCS | [[WHIR]] | multilinear polynomial commitment scheme |
| hash | [[Hemera]] | Fiat-Shamir instantiation, Merkle trees, all randomness |

all arithmetic is native [[Goldilocks field]]: $\mathbb{F}_p$ where $p = 2^{64} - 2^{32} + 1$

## the three primitives

one hash, one VM, one field. every stark in [[cyber]] reduces to:

1. run a [[nox]] program — the execution trace is the constraint system
2. commit the trace via [[WHIR]] — multilinear polynomial commitment
3. verify constraints via [[sumcheck]] — [[SuperSpartan]] handles AIR natively
4. output a proof $\pi$ — verifiable by anyone in polylogarithmic time

the [[nox]] VM's sixteen reduction patterns map directly to AIR transition constraints. each pattern becomes a polynomial equation relating register state before and after a reduction step. there is no translation layer between the program and the proof

## recursive composition

the stark verifier is itself expressible as a [[nox]] program. this closes the loop:

$$\text{verify}(\pi_0) \to \pi_1 \to \text{verify}(\pi_1) \to \pi_2 \to \cdots$$

$N$ proofs aggregate into a single proof. verification cost remains $O(1)$ regardless of how many computations were proven. this enables:

- batch verification of all [[cyberlinks]] in a block
- chained [[cyber/communication]] delivery proofs across arbitrary hop counts
- verifiable AI inference via [[trident]]

## proof taxonomy

[[cyber/proofs]] catalogs every proof type the protocol generates. categories span identity, [[cybergraph]] operations, communication, execution, data structures, storage, recursive composition, location, and epistemological proofs. every entry in that taxonomy is a stark — no SNARKs, no trusted setup, no curves

## why starks

| property | SNARK | stark |
|---|---|---|
| trusted setup | required | transparent |
| quantum resistance | vulnerable | resistant |
| proof size | ~200 bytes | ~60-157 KB |
| security basis | discrete log / pairing | hash collision resistance |
| prover complexity | $O(N \log N)$ | $O(N)$ linear |
| verifier complexity | $O(1)$ pairing | $O(\log^2 N)$ hash |

the tradeoff is proof size. starks are larger. but the security model is strictly stronger: hash-only, no algebraic assumptions, no ceremony, no quantum vulnerability. for a protocol designed to run at planetary scale for centuries, the transparency guarantee outweighs the bandwidth cost

see [[cyber/proofs]] for the full proof taxonomy. see [[Hemera]] for the hash function. see [[nox]] for the virtual machine. see [[Goldilocks field]] for the arithmetic. see [[trident]] for verifiable AI. see [[WHIR]] for the polynomial commitment scheme

discover all [[concepts]]
