---
alias: cascades
tags: cyber
crystal-type: entity
crystal-domain: cyber
---
a cascade is a multiparty interactive async pattern where a lead [[neuron]] declares an [[intent]] and subscribers self-organize into coordinated sub-[[signals]]

```
cascade = (lead_intent, sub_signals[], parent_signal)
```

| component | role |
|-----------|------|
| lead intent | [[intent]] broadcast by the lead [[neuron]] with open scope |
| sub-[[signals]] | [[signals]] produced by participating [[neurons]] in response |
| parent [[signal]] | sealed by the lead; includes recursive [[zheng]] proof over all sub-signals |

the cascade protocol:

1. lead [[neuron]] declares [[intent]] with scope that invites participation
2. subscribers observe the intent in the [[cybergraph]] at [[inception]] height
3. each subscriber self-assigns, produces their own sub-[[signal]]
4. lead [[neuron]] collects sub-signals and seals a parent [[signal]] with a recursive [[zheng]] proof covering the entire cascade
5. verifiers run `decide(σ_parent)` — one $O(\log n)$ check covers all contributions

the recursive proof is what makes cascade composable: a cascade can itself be a sub-signal in a larger cascade, and the proof size stays $O(\log n)$ at every level

cascades enable large-scale coordination while preserving the atomicity and verifiability of individual [[signals]]. participants do not need to trust the lead [[neuron]] — the proof speaks for the collective computation

discover all [[concepts]]
