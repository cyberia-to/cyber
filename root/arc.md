---
tags: cyber, language
alias: Arc, topology language
crystal-type: entity
crystal-domain: cyber
diffusion: 0.00017182217834897905
springs: 0.0012147642763434756
heat: 0.0008764287581845023
focus: 0.0006256261237144358
gravity: 6
density: 8.47
---
the [[graph]] language. makes graphs first-class — the primitive is a connection, not a number

| Op | Action |
|---|---|
| `link(a, b, w)` | Create weighted directed edge |
| `walk(start, n)` | Random walk of n steps |
| `reach(a, b)` | Test if path exists |
| `neighbors(n)` | Return adjacent nodes |
| `rank(g, steps)` | Compute stationary distribution (PageRank) |
| `spectral(g, k)` | Extract top-k eigenvectors |
| `match(g, pat)` | Subgraph pattern matching |

the [[cybergraph]] is not a data structure that lives inside a program. the [[cybergraph]] IS the program. every [[cyberlink]] is an `Edge`. every CID is a `Node`. CYBERRANK is `rank()`.

[[particles]] are objects ([[Hemera]] CIDs), [[cyberlinks]] are morphisms, linkchains are composition, [[semcons]] are natural transformations. Arc's [[algebra]] is [[category theory]] — the correct [[algebra]] for typed relational structure. Arc describes what the [[cybergraph]] *is*. compiles to [[Hemera]] CIDs for nodes and edges, and to [[Trident]] adjacency constraints for [[proof]]. decomposes into [[Trident]] (field ops for matrix math, [[Hemera]] hash verification for node identities) and [[Nox]] (tree encoding of [[topology]])

see [[cyb/languages]] for the complete language set. see [[cyb/multiproof]] for the proving architecture