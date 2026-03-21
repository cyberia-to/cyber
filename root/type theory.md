---
tags: computer science
crystal-type: entity
crystal-domain: computer science
stake: 5334237174354257
diffusion: 0.0005212539981649944
springs: 0.00011474033504361593
heat: 0.0002645744620858084
focus: 0.0003479639920127454
gravity: 11
density: 2.06
---

A formal system that classifies expressions by their types, preventing meaningless computations. The mathematical foundation of programming language safety and proof assistants.

## foundations

- [[lambda calculus]]: the untyped core -- functions as first-class values, application and abstraction
- simply typed lambda calculus: each term has a type, type-safe evaluation
- polymorphism: System F, parametric types, generics
- dependent types: types that depend on values, enabling specifications as types (Coq, Agda, Idris, Lean)

## Curry-Howard correspondence

Propositions are types. Proofs are programs. Implication is function type. Conjunction is product type. Disjunction is sum type. This bridge connects [[formal verification]] and programming into a single activity.

## type systems in practice

- static typing: types checked at compile time ([[compilers]]), catching errors early. Rust, Haskell, OCaml
- dynamic typing: types checked at runtime. Python, JavaScript
- gradual typing: mixing static and dynamic. TypeScript
- linear types: tracking resource usage, ensuring values are used exactly once (Rust ownership)

## connections

[[compilers]] implement type checkers. [[formal verification]] tools are proof assistants built on dependent type theory. [[neural networks]] operate on tensor types with shape constraints.