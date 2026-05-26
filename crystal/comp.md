---
tags: cyber, comp
alias: computation
icon: "\U000026A1"
crystal-type: entity
crystal-domain: comp
---
# comp

the science of [[step]]. what can be transformed — and how many steps it takes

the primitive object is the step: one state transition. apply a rule to an input, get an output. one reduction in [[nox]]. one gate in a circuit. one tick of a [[Turing machine]]. remove steps and nothing changes — the universe is frozen

comp is the third element of the [[form]] triad: [[proof]], [[bit]], [[step]]. together they produce the [[graph]]. [[math]] verifies the graph. [[info]] populates it with distinctions. comp traverses it with transformations

---

## the primitive

a step is not a number and not a distinction — it is a transformation. input → rule → output. the simplest step: apply one pattern to one noun → get one noun. this is [[nox]] `reduce(subject, formula)` — one pattern application

[[Alan Turing]] showed that a machine executing steps can simulate any other machine ([[universality]]). [[Kurt Goedel]] showed that no system of steps powerful enough to describe arithmetic can prove all truths about itself

the [[step]] is to comp what the [[bit]] is to info: the minimum unit. everything above — [[algorithms]], programs, circuits, operating systems — is composition of steps

---

## objects of comp

| object | what it is |
|--------|-----------|
| step | one state transition |
| [[algorithm]] | finite sequence of steps that solves a problem |
| circuit | parallel composition of steps (gates) |
| [[Turing machine]] | minimal universal step-executor |
| data structure | arrangement of [[bits]] for efficient stepping |
| complexity class | set of problems solvable in bounded steps |

---

## the two questions

comp asks two questions about any problem:

CAN it be computed? — computability. some problems have no algorithm (halting problem). the boundary between computable and uncomputable is sharp and proven

HOW MANY steps? — complexity. some computable problems need exponentially many steps (intractable). the boundary between tractable and intractable is the deepest open question in mathematics (P vs NP)

---

## for [[cyber]]

[[nox]] is the step-executor: 16 reduction patterns, each one step. `ask(ν, subject, formula, τ, a, v, t)` — the seven fields of a [[cyberlink]] are the seven arguments of computation. ordering a computation and asserting [[knowledge]] are the same act

the [[cybergraph]] is a universal memo cache: before stepping, [[nox]] checks if the result already exists. the more the graph grows, the fewer steps actually execute. computation accelerates itself

[[zheng|STARK]] proofs compress arbitrary steps into a constant-size certificate. the verifier checks one proof instead of re-executing all steps. this is what makes the [[cybergraph]] trustless

---

## bridges

- comp → [[math]]: proofs are computations. Curry-Howard maps every type to a proposition
- comp → [[info]]: [[compression]] is computation. [[entropy]] bounds the output of any lossless compressor
- comp → [[ai]]: [[machine learning]] is stepping through parameter space. [[inference]] is a forward pass
- comp → [[crypto]]: [[zheng|STARK]] proofs compress computation. [[zero knowledge]] proves execution without revealing inputs
- comp → [[cyber]]: every block is a state transition. the protocol is a planetary step-executor

## key figures

[[Alan Turing]], [[John von Neumann]], [[Charles Babbage]], [[Ada Lovelace]], [[Edsger Dijkstra]], [[Gottfried Leibniz]]

## pages

{{query (and (page-tags [[comp]]))}}