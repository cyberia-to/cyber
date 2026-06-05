---
alias: intents
tags: cyber
crystal-type: entity
crystal-domain: cyber
---
an intent is a [[signal]] with [[inception]] but no [[sealing]] — a neuron's public declaration of a directed action before proof is produced

```
intent = (neuron, inception, scope, identity_proof)
```

| component | role |
|-----------|------|
| [[neuron]] | the declaring agent |
| [[inception]] | block height when declared |
| scope | structured description: target [[particles]], predicate, deadline |
| identity proof | [[neuron]]'s signature over (neuron \|\| inception \|\| scope) |

an intent carries an identity proof (non-repudiable declaration) but no content [[zheng]] proof. the proof arrives at [[sealing]], when computation is complete and the intent becomes a [[signal]]

three fates for an intent:

- sealed — neuron finalizes with sealing height + zheng proof → becomes [[signal]]
- abandoned — never sealed; record persists in the [[cybergraph]] at inception height
- cascaded — triggers coordinated multiparty sub-signals → see [[cascade]]

intents are publicly readable. any [[neuron]] can observe pending intents and self-assign as a participant, enabling coordination without a central scheduler

discover all [[concepts]]
