---
alias: TIR, Trident Intermediate Representation
tags: cyber
crystal-type: entity
crystal-domain: computer science
---
the intermediate representation of [[trident]] — a typed graph IR between source [[language]] and [[triton]] assembly (TASM)

TIR captures program structure (nodes, operation types, nesting depth, branch count, loop bounds, memory access patterns) in a form suitable for both classical optimization and [[trident/reference/roadmap]]. the compilation pipeline: trident source → TIR → TASM → [[Triton VM]] → [[stark]] proof