---
tags: computer science
crystal-type: pattern
crystal-domain: computer science
---

Abstract machines that process input strings according to formal rules. The theoretical foundation of [[computation]] and [[compilers]].

## hierarchy (Chomsky)

1. finite automata (FA): recognize regular languages, used in lexers, regex engines. Deterministic (DFA) and nondeterministic (NFA) variants with equivalent power.
2. pushdown automata (PDA): recognize context-free languages, used in parsers. Add a stack to finite automata.
3. linear bounded automata (LBA): recognize context-sensitive languages. Turing machine with bounded tape.
4. Turing machine (TM): recognizes recursively enumerable languages, the universal model of computation. Equivalent to [[lambda calculus]].

## key results

- Church-Turing thesis: any effectively computable function is computable by a Turing machine
- halting problem: undecidable -- there is no algorithm that determines whether an arbitrary program halts ([[complexity theory]])
- pumping lemma: tool for proving a language is irregular or context-free

## connections

[[compilers]] use finite automata for lexing and pushdown automata for parsing. [[type theory]] and [[formal verification]] reason about what automata can and cannot compute. [[neural networks]] approximate functions that classical automata enumerate.