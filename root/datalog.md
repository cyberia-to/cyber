---
tags: cyber, computer science
crystal-type: entity
crystal-domain: cyber
---

a declarative logic programming language used for querying graph structures

## role in cyber

one of the 19 [[computation]] [[languages]] in [[cyber]]. datalog enables recursive queries over [[cyberlinks]] and [[particles]], making it possible to express transitive relationships, reachability, and pattern matching across the [[cybergraph]]

## semantics

a datalog program consists of facts and rules. facts correspond to [[cyberlinks]] — ground assertions in the graph. rules define derived relations through logical inference

```
link(X, Y) :- cyberlink(X, Y).
path(X, Y) :- link(X, Y).
path(X, Y) :- link(X, Z), path(Z, Y).
```

this recursive structure allows querying multi-hop relationships that a single [[cyberlink]] traversal cannot reach

## properties

datalog programs always terminate — the language restricts recursion to guarantee finite computation. this makes it suitable for on-chain query evaluation where unbounded execution is unacceptable

the language operates over the same [[particle]] namespace as the rest of [[cyber]], treating content-addressed identifiers as constants in the logic

see [[computation]], [[languages]], [[cyberlink]], [[cybergraph]], [[particle]], [[neural language]]
