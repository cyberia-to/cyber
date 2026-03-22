---
alias: TIR, Trident Intermediate Representation
tags: cyber
crystal-type: entity
crystal-domain: computer science
stake: 7215987788293826
diffusion: 0.00015906972354459177
springs: 0.00019073286265691166
heat: 0.00020009088160865854
focus: 0.00017677289689109883
gravity: 1
density: 8.45
---
the intermediate representation of [[trident]] — a typed graph IR between source [[language]] and [[triton]] assembly (TASM)

TIR captures program structure (nodes, operation types, nesting depth, branch count, loop bounds, memory access patterns) in a form suitable for both classical optimization and [[trident/reference/roadmap]]. the compilation pipeline: trident source → TIR → TASM → [[Triton VM]] → [[stark]] proof