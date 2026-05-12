---
tags: comp, cryptography
crystal-type: entity
crystal-domain: comp
alias: VDFs, verifiable delay function, verifiable delay functions
---
# VDF

a verifiable delay function is a function that requires a specified number of sequential steps to compute, cannot be parallelized, and produces a proof that can be verified in much less time than the computation itself

VDFs prove minimum elapsed time without trusted clocks. the output is deterministic — given the same input and delay parameter, every evaluator produces the same result. the proof convinces any verifier that the evaluator performed the sequential work

## construction

a typical VDF consists of three algorithms:

- setup(T) — produces public parameters for delay parameter T
- eval(x, T) — computes output y and proof φ* after T sequential steps
- verify(x, y, φ*) — checks the proof in O(log T) or O(1) time

the sequential nature comes from iterated squaring in groups of unknown order, or from iterated hashing in a [[hash chain]]. the key property: no amount of parallelism reduces the wall-clock time below T steps

## role in cyber

[[cyber]] uses VDFs for rate limiting and temporal ordering. a [[neuron]] attaches a VDF proof to its [[signal]] to demonstrate that real time has elapsed between consecutive emissions. this prevents burst flooding without requiring synchronized clocks or a central rate limiter

[[foculus]] leverages VDF proofs as one input to the timing model — they provide a lower bound on the real time between events, anchoring the asynchronous gossip protocol to physical time

---

see [[hash chain]] for the iterated hash construction. see [[signal]] for the emission mechanism. see [[foculus]] for how VDF proofs integrate with consensus
