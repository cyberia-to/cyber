---
alias: true false problem, true-false problem
tags: cyber
crystal-type: pattern
crystal-domain: cyber
---
the foundational problem of [[cyber]] [[inference]]

if `true` has [[cyberank]] 10 and `false` has [[cyberank]] 9, then for any question cyberlinked to both, the answer is always `true` — regardless of context. global rank dominates

the problem generalizes: any high-rank [[particle]] wins every contextual query it appears in. a question "what causes malaria?" linked to both "plasmodium" (rank 50) and "bad air" (rank 5000) answers "bad air" — not because it is correct, but because it is popular. [[cyberank]] measures what the graph attends to globally, not what is true locally

## why global rank fails for inference

[[cyberank]] is a per-[[particle]] score. it answers "how important is this particle across the whole [[cybergraph]]?" — not "how relevant is this particle to this question?" a system that answers every question with the most popular connected particle is a search engine, not [[intelligence]]

the insight: [[inference]] requires contextual truth. the same [[particle]] can be the right answer to one question and wrong for another. a single global number cannot encode this

## the solutions

[[cyber/truth/standard inference]] — the naive first attempt. multiply global [[cyberank]] by concentrated [[will]] per [[cyberlink]] in context. breaks global dominance by introducing a per-[[neuron]] conviction signal. simple and zero-cost, but still a single-factor approximation with no honesty guarantee and no market correction

[[cyber/truth]] — the full architecture. three layers that together make contextual truth emerge:

| Layer | Mechanism | What it solves |
|-------|-----------|---------------|
| [[tri-kernel]] local reconvergence | context [[particles]] shift the [[probability]] distribution locally | global rank dominance |
| [[serum]] + [[valence]] | honesty is a Bayes-Nash [[equilibrium]] | strategic voting |
| [[coupling|ICBS]] markets | capital flows against false edges | persistence of incorrect answers |

[source of discussion](https://github.com/cybercongress/go-cyber/issues/694)