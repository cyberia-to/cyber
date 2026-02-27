---
alias: implicit
tags: cyber, core
crystal-type: entity
crystal-domain: cyber
---
what [[neurons]] derive from observing [[explicit knowledge]] and encode as new [[cyberlinks]]. the language of [[neurons]]

a [[neuron]] observes [[cyberank]], [[karma]], [[syntropy]] — the outputs of the [[truth machine]]. from these signals the neuron infers meaning: what matters, what is missing, what is wrong. this inference is private, subjective, unbounded. the neuron then encodes its inference as a new [[cyberlink]] — a signed economic commitment fed back into the [[cybergraph]]

every [[cyberlink]] carries implicit knowledge: it encodes what the neuron inferred from the truth machine's output. a neuron sees that two [[particles]] have high [[cyberank]] but are unlinked — and links them. the link carries implicit knowledge into the [[cybergraph]]

## the observation loop

implicit knowledge is one direction in the continuous loop between [[neurons]] and the [[truth machine]]

```
neuron ──cyberlink──→ cybergraph ──tri-kernel──→ cyberank
  ↑                                                  │
  └──────────── observes, infers, links ←────────────┘
```

the [[truth machine]] produces [[explicit knowledge]] (deterministic, on chain). [[neurons]] observe it, derive meaning, and feed implicit knowledge back as [[cyberlinks]]. the loop continues

| | [[explicit knowledge]] | implicit knowledge |
|---|---|---|
| what | what the [[truth machine]] computes | what [[neurons]] derive and encode as [[cyberlinks]] |
| produced by | [[truth machine]] via [[inference]] | [[neurons]] via [[learning]] |
| language of | the [[truth machine]] | [[neurons]] |
| direction | [[truth machine]] → [[neurons]] | [[neurons]] → [[truth machine]] |

> something that is known but cannot be fully written down @nonaka and @takeuchi

[[intelligence]] is the loop sustaining itself

in [[cyber-sdk]] neurons encode implicit knowledge using

- [[cyberlink]] message type
- [[cosmwasm]] [[progs]]
- autonomous [[thoughts]]

in [[cyb-ts]] neurons encode implicit knowledge using

- [[cyb/oracle]] interface
- [[rune]]: dynamic scripting
- [[webgpu]]: local hardware independent parallel execution
