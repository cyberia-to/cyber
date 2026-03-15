---
tags: cyber, comp
alias: computation
crystal-type: entity
crystal-domain: comp
---
# comp

the domain of what can be executed. a computation is a sequence of state transitions that turns input into output according to rules. [[Alan Turing]] showed that a single machine can simulate any other — [[universality]]. [[Kurt Goedel]] showed that no system powerful enough to describe arithmetic can prove all truths about itself — [[limitations of tm]]

for [[cyber]], computation is the substrate. the [[tri-kernel]] runs three operators on the [[cybergraph]] every block. [[nox]] defines 16 reduction patterns — a [[lambda calculus]] variant that is both [[Turing]]-complete and cost-bounded. [[stark]] proofs compress arbitrary computation into a short certificate. the protocol does not store answers; it stores computations that anyone can re-execute and verify

## scope

machines — [[automata]], [[Alan Turing]], [[John von Neumann]], finite state machines, [[lambda calculus]]. the foundations: what can be computed at all, and what happens when you compose machines

complexity — [[complexity theory]], [[algorithms]], tractability, NP, [[optimization]]. some problems are solvable in principle but intractable in practice. [[cyberank]] must be computable in polynomial time; [[stark]] verification must be sublinear

languages — [[compilers]], [[wasm]], [[operating systems]], [[formal verification]], [[rust]], [[datalog]]. how humans and machines describe computations to each other. cyber uses [[wasm]] for on-chain execution, [[datalog]] for graph queries, [[trident]] for provable programs

architectures — [[distributed systems]], [[convergent computation]], [[natural computing]], parallel processing. computation does not require a single processor. the [[cybergraph]] is a distributed computation where every [[neuron]] contributes partial state transitions

## bridges

- comp → [[math]]: proofs are computations. the Curry-Howard correspondence maps every type to a proposition
- comp → [[info]]: [[compression]] is computation. [[entropy]] bounds the minimum output of any lossless compressor
- comp → [[ai]]: [[machine learning]] is computation on data. [[training]] is search in parameter space. [[inference]] is a forward pass
- comp → [[crypto]]: [[cryptographic proofs]] compress computation. [[zero knowledge]] lets you prove execution without revealing inputs
- comp → [[cyber]]: the protocol is a planetary computer. every block is a state transition verified by the entire network

## key figures

[[Alan Turing]], [[John von Neumann]], [[Charles Babbage]], [[Ada Lovelace]], [[Edsger Dijkstra]], [[Gottfried Leibniz]]
