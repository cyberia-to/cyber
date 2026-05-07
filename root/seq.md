---
tags: cyber, language
alias: Seq, sequence language, causality language
crystal-type: entity
crystal-domain: cyber
---
the event language. time in distributed systems is not a clock — it is the ordering. the causal structure that determines what could have influenced what

three temporal modes:

```
Stack   = nested time     = depth     = LIFO  = after { after { after } }
Heap    = concurrent time = chaos     = random = concurrent(a, b, c)
Stream  = linear time     = flow      = FIFO  = before(a, b), before(b, c)
```

| Domain | Stack | Heap | Stream |
|---|---|---|---|
| Hardware | Call stack | RAM allocation | I/O bus |
| OS | Process call depth | Dynamic memory | Pipes, sockets |
| Network | Protocol nesting | Concurrent connections | Packet flow |
| Consensus | Nested validation | Parallel validators | Block sequence |
| UI | Modal dialogs, undo | Independent windows | Scrolling, typing |

events form a partial order — not a total order. Seq preserves the partial order and only totalizes when consensus demands it. compiles to Tri ordering constraints

see [[cyb/languages]] for the complete language set. see [[cyb/multiproof]] for the proving architecture