---
tags: computer science
crystal-type: entity
crystal-domain: computer science
stake: 5222361394690787
diffusion: 0.0005312846679735202
springs: 0.0001063901304584046
heat: 0.0002600502969133643
focus: 0.00034956943250696453
gravity: 10
density: 3.53
---

Translators that convert programs written in a high-level language into machine code executable by hardware. The bridge from human intent to [[computation]].

## phases

1. lexical analysis: breaking source text into tokens
2. parsing: building an abstract syntax tree (AST) from tokens, context-free grammars ([[automata]])
3. semantic analysis: [[type theory]] checking, scope resolution, name binding
4. intermediate representation: platform-independent code (SSA, three-address code)
5. optimization: dead code elimination, inlining, loop unrolling, register allocation
6. code generation: emitting target machine instructions

## key concepts

- type checking: verifying expressions conform to type rules, static vs dynamic ([[type theory]])
- optimization levels: trading compile time for runtime performance
- just-in-time (JIT): compiling at runtime, used by JVM and V8
- bootstrapping: a compiler written in its own language compiling itself

## formal foundations

Parsing relies on [[automata]] (finite state machines for lexing, pushdown automata for grammars). Correctness proofs use [[formal verification]]. Complexity of optimization problems connects to [[complexity theory]].