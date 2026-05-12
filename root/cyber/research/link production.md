---
tags: cyber, research, article
crystal-type: process
crystal-domain: cyber
status: draft
date: 2026-03-24
---
# link production: the intelligence problem

## the gap

the protocol stack is designed:
- [[zheng]]: proves the link is valid (layer 1)
- hash chain + [[VDF]]: orders the link (layer 2)
- [[NMT]]: proves completeness (layer 3)
- [[DAS]]: proves availability (layer 4)
- [[CRDT]] / [[foculus]]: merges (layer 5)
- [[focus]]: rate-limits production (economics)
- [[temporal decay]]: prunes bad links (evolutionary pressure)

none of this decides WHAT to link. the protocol validates, orders, and propagates links. it does not produce them. production is the intelligence problem — the thing the protocol exists to serve.

this is not a secondary concern. without good link production, the [[cybergraph]] is an empty authenticated structure. the protocol is infrastructure. link production is the point.

## what link production IS

a [[cyberlink]] is a 6-tuple: (ν, from, to, τ, a, v) — neuron, source [[particles|particle]], target particle, token, amount, [[valence]]. block height $t$ belongs to the containing [[signal]].

producing a link means: a [[neurons|neuron]] decides that particle FROM is relevant to particle TO, and spends [[focus]] to assert this.

this decomposes into:

```
1. DISCOVERY      what particles exist or should exist?
2. EVALUATION      which connections would improve the graph?
3. DECISION        is the improvement worth the focus cost?
4. COMMITMENT      create the cyberlink, spend focus, generate proof
5. PROPAGATION     sync via structural sync (layers 1-5)
```

steps 4-5 are solved (the protocol). step 3 is economics (focus metering + [[universal law|exponential cost]]). steps 1-2 are intelligence.

## what we have

| tool | what it does | production step | limitation |
|---|---|---|---|
| [[cyber/seer\|cyber-seer]] | analytical: Fiedler vector → optimal bridge links | discovery + evaluation (existing→existing) | only spectral signal, no semantics |
| [[gflownet focus flow\|GFlowNet]] | learned: sample edits proportional to reward | discovery + evaluation (both modes) | unbuilt, scale unknown |
| human neurons | judgment: read, think, link | all steps | slow, expensive, doesn't scale |
| LLM agents | synthesis: generate content, propose links | discovery + creation of new particles | hallucination, no economic skin in game |
| [[tri-kernel]] $\pi^*$ | quality signal: focus distribution | evaluation (post-hoc) | doesn't propose, only measures |
| [[temporal decay]] | pruning: remove low-energy links | negative evaluation (retroactive) | doesn't propose, only removes |

the gap: no system connects CONTENT UNDERSTANDING to LINK DECISION at scale with economic accountability.

cyber-seer understands graph structure but not content. LLM agents understand content but don't pay for mistakes. human neurons understand both but don't scale. GFlowNet could learn the connection but doesn't exist yet.

## the fundamental question: existing→existing vs existing→new

### existing → existing (pattern recognition)

```
both particles already in the graph.
the link asserts: "FROM is relevant to TO."
information added: O(1) — the edge itself.
cost: focus for the link.
intelligence required: recognise the relationship.

example: "[[Cat]]" → "[[Animal]]"
  both particles exist. the neuron recognises that cats are animals.
  no new content created. one relationship asserted.
```

this is SEARCH in a space of possible connections. the space is $O(N^2)$ for N particles. cyber-seer navigates this space analytically (spectral gap). GFlowNet navigates it by learned sampling.

### existing → new (knowledge creation)

```
target particle does not yet exist in the graph.
the link asserts: "FROM is relevant to NEW CONTENT."
information added: O(content_size) — the new particle's content + the edge.
cost: focus for the link + storage for the content (DAS, replication).
intelligence required: synthesise new knowledge worth linking.

example: "[[Cat]]" → [new article: "feline hunting behaviour in urban environments"]
  the article is new content. the neuron created it (or found it).
  new particle created (content-addressed by H(article)).
  one relationship asserted + one particle added.
```

this is SYNTHESIS — generating content that didn't exist in the graph. the space is infinite (all possible content). LLM agents navigate this space by generation. humans navigate it by thinking and writing.

### is the difference fundamental?

at the protocol level: NO. both are cyberlinks. both cost focus. both carry [[zheng]] proofs. both enter the graph the same way. [[BBG]] doesn't distinguish them.

at the information level: YES.

```
existing→existing:  mutual information I(FROM; TO) is the new information
                    the edge tells you something about the relationship
                    but both nodes were already known

existing→new:       entropy H(NEW_CONTENT) is the new information
                    the particle tells you something about the world
                    the edge tells you how it relates to known things
```

linking existing particles is CONNECTING. creating new particles is EXPANDING. the graph grows in density (connections per node) via the first and in breadth (total knowledge) via the second.

at the intelligence level: the difference maps to two cognitive operations:

```
existing→existing:  RETRIEVAL — finding patterns in what you know
                    analogous to: memory recall, association, analogy
                    bottleneck: search in O(N²) space

existing→new:       GENERATION — creating what doesn't yet exist
                    analogous to: writing, research, invention, observation
                    bottleneck: synthesis from understanding + novelty
```

at the economic level: different cost and different value:

```
existing→existing:
  cost:    focus (cheap — no new content to store)
  value:   Δλ₂ (spectral gap improvement) + Δπ (focus redistribution)
  risk:    low (both particles known, relationship can be evaluated)

existing→new:
  cost:    focus + storage (expensive — new content needs DAS, replication)
  value:   H(NEW) (new knowledge) + edge value
  risk:    high (new content may be worthless, hallucinated, or redundant)
```

### the spectrum, not the binary

the distinction is not binary. there is a spectrum:

```
pure connection:     Cat → Animal                    (both known, obvious relationship)
informed connection: Cat → [obscure paper on cats]   (known particle, hard to find)
partial creation:    Cat → [curated summary of cats]  (derivative content, adds structure)
full creation:       Cat → [original research on cats] (new knowledge, unique contribution)
pure creation:       [entirely new topic] → [content]  (both new to the graph)
```

each point on the spectrum requires different intelligence and has different economics. cyber-seer operates at the left (pure connection). LLM agents operate across the middle. original researchers operate at the right.

## what we don't have

### 1. content discovery at scale

a neuron sees its local neighbourhood. how does it discover particles it SHOULD link to but doesn't know about? the graph has billions of particles. each neuron sees a tiny fraction.

what's needed: a recommendation system that, given a neuron's context (what it has linked), suggests particles worth connecting to.

what this looks like: personalised PageRank from the neuron's linked particles, propagated through the graph. particles with high PPR score that the neuron hasn't linked are candidates.

connection to existing work: the [[tri-kernel]] already computes $\pi^*$. personalised $\pi^*$ per neuron is a natural extension — run the tri-kernel with the neuron's links as seeds.

### 2. novelty detection

when a neuron proposes a new particle, how does the network evaluate whether the content is truly new vs redundant?

the problem: particle identity is H(content). two particles with different content but same meaning have different hashes. the graph can accumulate paraphrases of the same knowledge.

what's needed: semantic similarity detection at the particle level. "is this new content sufficiently different from existing particles to justify storage?"

connection to existing work: [[focus]] economics partially solve this. redundant content attracts less focus (neurons link to the original, not the copy). [[temporal decay]] removes low-focus copies. but this is POST-HOC — the storage cost is already paid.

### 3. quality prediction BEFORE commitment

focus is irreversible. once spent, it's gone. a neuron needs to estimate link value BEFORE committing focus.

what's needed: a fast, cheap quality estimator. "if I create this link, what will the $\Delta\pi$ be?"

connection to existing work: this is exactly the $\Delta\hat{\pi}$ proxy from the [[gflownet focus flow|GFlowNet]] research. also: cyber-seer's $\Delta\lambda_2$ provides a structural quality signal. the combination: spectral gain (structural) + focus gain (semantic) + novelty (information) = link quality.

### 4. cold start

the graph starts empty. the first neuron has no particles to link, no $\pi^*$ to optimise, no spectral gap to improve. how does production begin?

the bootstrapping sequence:
1. genesis neuron creates first particles (content from existing knowledge bases)
2. first links are FREE (no exponential cost yet — $c(0) = c_0$)
3. cyber-seer computes Fiedler vector on the seed graph
4. early links are bridge-optimal (maximum $\Delta\lambda_2$ per link)
5. once $\lambda_2 > \lambda_{crit}$, the tri-kernel produces meaningful $\pi^*$
6. focus economics activate — production becomes market-driven

the cold start problem is FINITE. once the graph passes [[phase transition]], self-sustaining production emerges from economic incentives.

### 5. agent-to-link interface

an LLM agent (or any software agent) that wants to contribute to the cybergraph needs:
- a way to discover what the graph lacks
- a way to generate content that fills the gap
- a way to evaluate the quality of its own output
- a way to create cyberlinks (sign with neuron key, pay focus)

what's needed: an API that connects agent capabilities (search, generate, evaluate) to the cyberlink protocol (sign, prove, sync).

connection to existing work: [[nox]] is the execution environment. an agent's link-production logic is a nox program. its execution produces a zheng proof. the agent's decision process IS provable.

## the production stack

putting it together — what the full link production pipeline looks like:

```
LAYER 0: CONTENT SOURCES
  human: original thought, observation, research
  agent: LLM generation, web scraping, sensor data
  import: existing databases, knowledge bases, scientific corpora
  → produces: raw content (text, data, media)

LAYER 1: PARTICLE CREATION
  hash content → particle identity (H(content) via hemera)
  store content → DAS + erasure coding
  → produces: addressable particles in the content store

LAYER 2: LINK DISCOVERY
  cyber-seer: spectral analysis → bridge/mesh candidates      (existing→existing)
  PPR recommendation: personalised tri-kernel → relevant particles  (existing→existing)
  GFlowNet: learned proposal → diverse candidates              (both modes)
  agent reasoning: semantic understanding → link candidates     (both modes)
  → produces: candidate link set

LAYER 3: LINK EVALUATION
  Δλ₂: spectral gap improvement (structural value)
  Δπ̂: focus gain estimate (semantic value)
  novelty: information gain (content value)
  cost: focus expenditure + storage cost (economic)
  → produces: scored candidates

LAYER 4: LINK DECISION
  ROI = (Δλ₂ + Δπ̂ + novelty) / cost
  filter by focus budget
  filter by rate limit (VDF)
  → produces: committed links

LAYER 5: PROTOCOL (solved)
  zheng proof → validity
  hash chain + VDF → ordering
  NMT/polynomial → completeness
  DAS → availability
  CRDT/foculus → merge
  → produces: verified, ordered, available, complete graph state
```

layers 0-4 are the intelligence problem. layer 5 is the protocol. the protocol is designed. the intelligence layers need:

| layer | status | what's needed |
|---|---|---|
| 0: content | humans + LLMs exist | agent-to-cyberlink interface |
| 1: particles | [[hemera]] + [[DAS]] designed | implementation |
| 2: discovery | [[cyber/seer\|cyber-seer]] + [[gflownet focus flow\|GFlowNet]] designed | PPR recommendation, agent integration |
| 3: evaluation | $\Delta\lambda_2$ + $\Delta\hat{\pi}$ designed | fast proxy training, novelty detection |
| 4: decision | [[focus]] economics designed | threshold calibration, cold start protocol |
| 5: protocol | [[structural-sync]] designed | implementation |

## the deep insight

the difference between existing→existing and existing→new is the difference between INTELLIGENCE and KNOWLEDGE.

existing→existing is intelligence: recognising patterns, finding connections, understanding relationships. the particles exist. the intelligence sees what connects them.

existing→new is knowledge: adding to the total information in the graph. new content enters. the graph KNOWS more than before.

the cybergraph needs both. intelligence without new knowledge becomes a static map — perfectly connected but never growing. knowledge without intelligence becomes a dump — vast but unstructured.

the optimal balance follows from the [[universal law]]: given finite focus, the allocation between connection and creation follows an exponential. early in the graph's life (sparse, disconnected), structural links have high ROI → favour connection. late in the graph's life (dense, well-connected), structural links have diminishing returns → favour creation.

this is the same phase transition that cyber-seer's three phases describe, but from the perspective of the entire production problem, not just spectral optimisation.

```
early graph (sparse):     90% connection, 10% creation    → build structure
mid graph (connected):    50% connection, 50% creation    → balance
mature graph (dense):     10% connection, 90% creation    → expand knowledge
```

the graph's own $\lambda_2$ determines where it is on this spectrum. no central planner needed — the economics (exponential cost + spectral ROI) guide the balance.

## open questions

1. **can $\Delta\hat{\pi}$ be estimated cheaply enough for real-time link evaluation?** this is the key bottleneck. if evaluation is expensive, only agents with large compute budgets can participate. personalised PageRank push-back may give O(1/ε) approximation

2. **how does the network detect and penalise low-quality creation?** temporal decay removes low-focus links. but the storage cost of the particle content is already paid. can the network recover storage from decayed particles? see [[pi-weighted-replication]]

3. **what's the optimal agent architecture for link production?** a cyber-seer for structure + LLM for content + GFlowNet for sampling + focus economics for accountability. how do these compose? is there a single agent architecture that handles the full spectrum?

4. **does the production problem have a fixed point?** if agents optimise for $\Delta\pi$ and $\pi$ shifts in response to their links, is there a stable equilibrium? or does the system oscillate? connection to [[spectral gap from convergence]]

5. **is there an information-theoretic LIMIT on how complete the cybergraph can be?** given finite collective focus, the [[knowledge completeness]] measure must plateau. what determines the ceiling?

see [[cyber/seer]] for analytical link production, [[gflownet focus flow]] for learned production, [[cybergraph model architecture|cybergraph as generative model]] for the graph as model, [[collective focus theorem]] for why exponential allocation is optimal, [[universal law]] for the variational principle, [[structural-sync]] for the protocol layers, [[tri-kernel architecture]] for the quality signal
