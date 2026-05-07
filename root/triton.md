---
alias: Triton VM
tags: cyber
crystal-type: entity
crystal-domain: computer science
---
a virtual machine designed for generating [[stark]] proofs of program execution

[[trident]] compiles to [[tir]], then to TASM (Triton Assembly), which executes on Triton VM. every instruction produces a valid algebraic execution trace — the trace is the proof. arithmetic operates over the Goldilocks [[field]] ($p = 2^{64} - 2^{32} + 1$)

Triton VM uses nine Algebraic Execution Tables (AET) whose heights determine proof cost. see [[trident/reference/roadmap]] for neural optimization of the compilation pipeline