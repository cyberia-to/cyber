---
tags: cyber, language
alias: Arc, topology language
crystal-type: entity
crystal-domain: cyber
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

the [[cybergraph]] is not a data structure that lives inside a program. the [[cybergraph]] IS the program. every [[cyberlink]] is an `Edge`. every CID is a `Node`. CYBERRANK is `rank()`. Arc decomposes into [[Trident]] (field ops for matrix math), [[Bt]] (hash verification for node identities), and [[Nox]] (tree encoding of [[topology]])

see [[cyb/languages]] for the complete language set. see [[cyb/multiproof]] for the proving architecture
