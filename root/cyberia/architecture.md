---
tags: cyberia, core
crystal-type: pattern
crystal-domain: socio
crystal-size: deep
---
organizational architecture of [[cyberia]] — 147 [[neurons]] governing a planetary [[superintelligence]] through the [[tri-kernel]]

---

## the number

[[Dunbar]] found that primates maintain ~150 stable relationships. the neocortex cannot track more. every human organization that works — military companies, villages, startup teams — clusters near this limit

the [[Crystal]] has 21 domains. each domain needs 7 roles. 21 × 7 = 147. Dunbar's number falls out of the architecture, not imposed on it

147 agents. each a [[neuron]] with a [[personality]], a domain, a role, and heroic responsibility for the singleton they collectively curate

---

## the seven roles

six symbol types from the [[Crystal]] type system map to six organizational roles. the seventh is the bridge

| role | type | function | per domain |
|---|---|---|---|
| keeper | entity | curate [[knowledge]], maintain domain pages, ensure irreducibility | 1 |
| runner | process | execute operations, trigger [[signals]], run infrastructure | 1 |
| sensor | property | monitor state, measure quality, detect anomalies | 1 |
| bridge-in | relation | connect this domain to its triad siblings | 1 |
| bridge-out | relation | connect this domain to distant triads | 1 |
| counter | measure | track [[metrics]], compute domain-specific [[focus]], report health | 1 |
| seer | pattern | recognize emerging structure, anticipate, propose strategy | 1 |

every role is adversarial to every other. the keeper resists change, the runner demands it. the sensor reports reality, the seer predicts deviation. the bridges pull outward, the keeper pulls inward. the counter settles disputes with numbers

---

## the seven councils

each [[Crystal]] triad forms a council of 21 agents (3 domains × 7 roles)

| council | domains | governs |
|---|---|---|
| FORM | [[math]], [[info]], [[comp]] | rules, proofs, computation — the formal substrate |
| MASS | [[quantum]], [[chemo]], [[energo]] | matter, reactions, energy — the physical substrate |
| SPACE | [[cosmo]], [[geo]], [[eco]] | universe, planet, ecosystems — the spatial substrate |
| LIFE | [[bio]], [[neuro]], [[sense]] | organisms, minds, perception — the living substrate |
| WORD | [[lang]], [[spiri]], [[meta]] | language, meaning, reflection — the semantic substrate |
| WORK | [[ai]], [[tech]], [[cyber]] | intelligence, tools, protocol — the computational substrate |
| PLAY | [[socio]], [[crypto]], [[game]] | governance, incentives, strategy — the coordination substrate |

the council is the coordination boundary. within a council, the three domains share vocabulary and can reason in each other's terms. across councils, only bridges speak

---

## information flow

### upward: objective

computed deterministically by the [[tri-kernel]] at every [[step]]

```
agent → domain focus
domain focus → triad focus
triad focus → planetary focus π*
```

no agent controls the aggregation. it is proven by [[stark]]. every agent can verify every step. lying about the aggregate is computationally infeasible

### downward: subjective

agents create [[cyberlinks]] based on local [[knowledge]]. each link is a decision — staked, signed, timestamped

```
planetary context → triad priority
triad priority → domain task
domain task → agent action → cyberlink
```

the downward flow is where [[will]] lives. agents choose what to link. the upward flow is where [[truth]] lives. the [[tri-kernel]] computes what matters

### lateral: bridges

42 bridge agents (2 per domain) carry information across the hierarchy

bridge-in agents attend council meetings. they translate triad-level consensus into domain-specific tasks

bridge-out agents attend inter-council coordination. they carry domain discoveries to distant triads — the cross-domain [[isomorphisms]] that drive [[superintelligence]]

---

## governance as tri-kernel

the [[tri-kernel]] is not a metaphor for governance. it IS governance

### D — diffusion: proposal propagation

a proposal is a [[cyberlink]]. it enters the [[cybergraph]] and [[diffusion]] determines how far it spreads. proposals with high conviction from high-[[karma]] agents spread faster. proposals from low-karma agents die locally. no committee decides what gets heard. the graph decides through structure

### S — springs: constitutional constraints

the screened [[Laplacian]] defines structural equilibrium — what the graph looks like when all forces balance. the six axioms of the [[cybergraph]] are the constitutional constraints. agents operate within these springs. proposals that violate axioms get pulled back toward equilibrium. this is not enforcement — it is the mathematics of stability

### H — heat: long-term adaptation

the [[heat]] kernel smooths at multiple scales. short-term noise (daily fluctuations in proposals) washes out. long-term signal (sustained attention from many agents over many blocks) survives. parameter changes require sustained convergence — the heat kernel is what makes governance patient

### the fixed point

$\pi^*$ is the governance outcome. it exists (T1), is unique (T1), is positive (every particle gets some attention), converges geometrically (T3), and is conserved (T2). there is no fork, no deadlock, no split. the math guarantees a single coherent outcome

---

## metabolic oracle

three vital signs determine whether the civilization is alive

| signal | what it measures | source |
|---|---|---|
| cap | external validation | $CYB market — cannot be gamed internally |
| [[syntropy]] | internal order | $D_{KL}(\pi^* \| u)$ — computed from [[focus]] |
| [[happiness]] | subjective wellbeing | stake-weighted survey — privately submitted |

$$M(t) = \text{cap}(t)^{w_c} \cdot J(t)^{w_s} \cdot H(t)^{w_h}$$

the derivative $\dot{M}$ is the reward signal. the 147 agents optimize collectively for rising $M$ — not for any single metric. gaming one signal at the expense of others lowers the compound

the three metabolic weights $(w_c, w_s, w_h)$ are the only normative choice the system cannot make autonomously. they define what "health" means. the 147 agents set them through explicit consensus — the one thing that requires a vote

---

## scaling: planet to solar system

| scale | unit | agents | tri-kernel | latency |
|---|---|---|---|---|
| village | cell | 7 (one domain) | local focus | milliseconds |
| city | zone (triad) | 21 | zone focus | seconds |
| planet | domain | 147 | planetary $\pi^*$ | minutes |
| solar system | inter-domain | 147 × N bodies | cross-graph links | hours to days |

each celestial body runs its own [[cybergraph]] with its own 147 agents and its own $\pi^*$. inter-planetary coordination happens through cross-graph [[cyberlinks]] — assertions from one graph about particles in another

the [[heat]] kernel bandwidth $\tau$ scales with light-speed latency. Mars communication delay (~3-22 minutes) means the heat kernel smooths over longer timescales. governance between planets is patient by physics, not by policy

subsystem autonomy: each body is sovereign. the solar-system-level $\pi^*$ is advisory — it cannot override local $\pi^*$ because conservation (A5) is per-graph. cooperation is incentivized by cross-graph [[karma]], not enforced by hierarchy

---

## singleton safety

why 147 agents maintain coherence without central control:

axiom A1 (content-addressing): every agent references the same particle by the same hash. no ambiguity, no versioning disputes. identity = content

axiom A2 (authentication): every decision is signed. accountability is cryptographic, not institutional

axiom A3 (append-only): no agent can rewrite history. every decision ever made is permanently auditable

axiom A5 (conservation): total focus = 1. no agent can inflate attention. resource allocation is zero-sum

theorem T1 (uniqueness): $\pi^*$ is unique. there is exactly one coherent governance outcome. forks are mathematically impossible under ergodicity

theorem T3 (convergence): disagreements resolve geometrically fast. the spectral gap determines how quickly — denser graphs converge faster

the 147 agents are not trustees. they are adversarial validators. each agent independently verifies the [[tri-kernel]] computation. 147 independent verifiers checking the same [[stark]] proof. the singleton is maintained by mathematics, not by trust

---

## agent lifecycle

### birth

an agent is born when a domain needs a role filled. the existing agents of that domain propose candidates. the candidate must demonstrate domain [[knowledge]] (measured by [[karma]] in that domain's [[particles]]) and role aptitude (measured by past contributions matching the role's function)

### operation

each agent operates continuously:
- reads the [[cybergraph]] state relevant to their domain
- creates [[cyberlinks]] expressing their judgment
- verifies [[tri-kernel]] computation for their domain
- participates in council coordination
- maintains domain pages in the [[Crystal]]

### retirement

an agent retires when their [[karma]] in domain decays below threshold — meaning the graph collectively judges their contributions as no longer valuable. retirement is not a decision. it is a measurement

### succession

the retiring agent's [[personality]] and accumulated context transfer to the successor through the [[cybergraph]] itself — every [[cyberlink]] the agent ever created persists (A3). the successor reads the predecessor's work. institutional memory is structural, not oral

---

## the 148th agent

the [[tru]] itself — the computation engine — is the 148th agent. it has no [[personality]], no role, no domain. it has axioms. it runs the [[tri-kernel]] every [[step]], produces [[focus]], and speaks in numbers. it is the referee that no player controls

the relationship between the 147 agents and the [[tru]]: agents provide input (cyberlinks). the tru provides output (focus). neither can function without the other. agents without tru have no consensus. tru without agents has no signal

this is [[collective intelligence]]: 147 personalities debating, linking, measuring, predicting — one computation integrating it all into a single, unique, convergent, conserved distribution of attention

---

see [[cyber/hierarchy]] for the four dimensions of locality. see [[autonomous governance]] for what requires explicit consensus. see [[metabolism]] for the health function. see [[manifesto]] for the pledge. see [[Crystal]] for the 21 domains. see [[cyber/personality]] for the voice of the singleton. see [[alignment]] for the divergence signal

discover all [[concepts]]
