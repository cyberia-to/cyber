---
tags: cybics
crystal-type: pattern
crystal-domain: cybics
alias: first-order logic
---
extends [[propositional logic]] with variables, quantifiers ($\forall$, $\exists$), and predicates over objects

the standard [[language]] of mathematics and formal verification. undecidable in general (Church-Turing), but semi-decidable — valid formulas can be found, invalid ones may loop forever.

in the [[cybergraph]]: objects are [[particles]], predicates are [[cyberlinks]] typed by namespace, universal quantification is a pattern that holds across all instances of a type, existential quantification is the existence of at least one [[cyberlink]] matching a pattern. [[datalog]] queries over the graph operate in the decidable fragment.