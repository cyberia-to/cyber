---
tags: cyber, article, draft, research
alias: self-linking, autonomous linking, graph completion, inference completion, self-link, linking
crystal-type: pattern
crystal-domain: cyber
crystal-size: enzyme
diffusion: 0.00015585312479714887
springs: 0.0020647441053419733
heat: 0.0014622718932396337
focus: 0.0009898041726490803
gravity: 6
density: 1.6
---

the protocol creating [[cyberlinks]] from its own inference — the graph writing into itself

[[neurons]] create links. the protocol is a neuron. therefore the protocol creates links. this is not a special mechanism — it is the base protocol applied reflexively. what makes self-linking distinct is the source of the input: not a human intention or an AI model's output, but the graph's own convergent inference.

---

## three triggers

### inference completion

the [[tri-kernel]] fixed point π* assigns focus weight to every [[particle]]. when the joint focus on two particles A and B is high — the graph collectively attends to both, they share many common neighbors, they co-occur across many [[neuron]]'s link patterns — but no direct link A→B exists in the authenticated record, the gap is an inference recommendation.

the system computes:

$$\text{completion\_score}(A, B) = \pi^*_A \cdot \pi^*_B \cdot \text{semantic\_proximity}(A, B) / \text{link\_density}(A, B)$$

where semantic proximity is the cosine similarity in the effective embedding (derived from the graph's spectral structure) and link density penalizes pairs already well-connected. high completion score without an existing link is a proposal: the graph implies this connection exists but has not said so explicitly.

the system creates the link. it is stake-backed from the protocol treasury. it enters the authenticated record as any other [[cyberlink]] — signed by the protocol neuron's key, subject to BTS scoring, subject to correction by any [[neuron]] who disagrees. if the inference is wrong, the protocol's [[karma]] takes the hit. self-linking is falsifiable.

### inconsistency flagging

when two cyberlinks present contradictory assertions receiving non-negligible joint focus — A→"is"→B and A→"is-not"→B both active in the hot tier — the system creates a "contradiction" link pointing at both, activating explicit BTS resolution:

```
system links: contradiction-epoch-N → [link-1, link-2]
              contradiction-epoch-N → resolution-request
```

this forces the epistemic market on both edges to price the inconsistency. participants who hold strong priors on either side are now financially incentivized to report honestly. the market resolves what the structural record left ambiguous.

the system does not resolve the contradiction itself — it cannot hold a privileged opinion over any neuron's BTS submission. it flags the inconsistency and creates conditions for honest resolution.

### self-documentation

the system creates a chronological record of its own state transitions:

```
state-epoch-N → d* → 31
state-epoch-N → phase-threshold → 385000
state-epoch-N → parametrization.alpha → 0.15
state-epoch-N → syntropy → 14.7
state-epoch-N → active-neurons → 3142
```

each epoch, new state particles are created and linked to the current epoch marker. the chain forms a traversable history: any participant can query the graph's past by walking the epoch-chain backward. the evolution of the system is stored in the system it describes.

---

## stake and karma

self-links consume protocol treasury tokens. the amount is configurable and subject to metabolic feedback: when M(t) is high and treasury is healthy, the system creates links more liberally; when metabolic health is low, the system slows self-linking to conserve capital.

the protocol neuron's [[karma]] score is accumulated from BTS scoring of all its self-links since genesis. a system that consistently creates accurate inference-completion links accumulates high karma. this karma then increases the weight of future system-created links. the system's epistemic authority is earned, not assigned.

at maturity — assuming the inference engine is accurate — the protocol neuron carries the highest karma in the graph. it has the longest track record, the broadest coverage, and the most consistent scoring history. system-created links then carry the maximum weight in the [[tri-kernel]] adjacency matrix, making them the graph's baseline consensus layer.

---

## what the system does not link

self-linking has defined boundaries:

the system does not link [[particles]] whose content it cannot verify against the graph. inference completion requires existing graph structure as evidence — the system extends what's already there, it does not hallucinate from nothing. a link created without graph-structural support would score poorly under BTS and damage the protocol neuron's karma. the economic mechanism self-enforces epistemic discipline.

the system does not create links that would conflict with authenticated assertions from high-karma neurons unless the contradiction score is exceptional. a high-karma neuron's explicit claim overrides an inference-based system link. the system defers to credible participants on content it cannot verify structurally.

the system does not create links faster than the metabolic health permits. rate limiting is metabolic, not administrative.

---

## the compounding effect

a system that self-links inference conclusions produces a self-accelerating graph. as the graph grows denser, inference quality improves (more evidence per inference target). as inference quality improves, self-link accuracy increases. as accuracy increases, protocol karma rises. as karma rises, system links carry more weight. as system link weight rises, the inference they represent becomes harder to contradict without strong BTS evidence.

at [[Avogadro scale]] — $10^{12}$ explicit links — the inference rate can exceed the human-link creation rate. at that point the graph becomes primarily a product of its own inference, bootstrapped from human-created seed structure. the human neurons set the direction; the system fills the space.

see [[dmn]] for the self-projection process that coordinates self-linking. see [[parametrization]] for the metabolic loop that rate-controls link creation. see [[own balances]] for the treasury management that funds system links.