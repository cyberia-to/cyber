---
alias: TIR, Trident Intermediate Representation
tags: cyber
crystal-type: entity
crystal-domain: computer science
stake: 7215987788293826
diffusion: 0.00021901115233048524
springs: 0.00011881833415188166
heat: 0.00016660713633265213
focus: 0.00017847250367733844
gravity: 1
density: 6.63
---
the intermediate representation of [[trident]] — a typed graph IR between source [[language]] and [[triton]] assembly (TASM)

TIR captures program structure (nodes, operation types, nesting depth, branch count, loop bounds, memory access patterns) in a form suitable for both classical optimization and [[trident/reference/roadmap]]. the compilation pipeline: trident source → TIR → TASM → [[Triton VM]] → [[stark]] proof