---
tags: cyberia, core, cyber
crystal-type: pattern
crystal-domain: cyber
crystal-size: deep
diffusion: 0.00012398159494750766
springs: 0.0009210886402165627
heat: 0.0006818879103950729
focus: 0.00047469497161775116
gravity: 2
density: 3.28
---
practical agent deployment for [[cyberia/architecture]] — 147 [[neurons]] running on [[OpenFang]], coordinated through the [[cybergraph]]

---

## execution substrate: OpenFang

[[OpenFang]] is an agent operating system in Rust. single binary, ~32 MB, 40 MB idle RAM, 180 ms cold start per agent. runs autonomous agents 24/7 on schedules, with real work output

why OpenFang for [[cyberia]]:

| requirement | OpenFang capability |
|---|---|
| 147 concurrent agents | single-process async runtime, SQLite-backed persistence |
| agent [[personality]] | TOML manifest with system_prompt, model override, capabilities |
| inter-agent coordination | agent_send/agent_list, shared memory, task board, knowledge graph |
| multi-model | 27 LLM providers, per-agent model selection, automatic fallback chains |
| orchestration | workflow engine with fan-out/collect, conditional, loop steps |
| persistence | agents survive restarts, sessions restored from SQLite |
| isolation | WASM sandbox with fuel metering for untrusted code |
| communication | 40 channel adapters (Telegram, Discord, Slack, Matrix, Nostr) |
| distributed | OFP P2P wire protocol with HMAC-SHA256 mutual auth, mDNS discovery |

### deployment on [[cyber valley]] hardware

one OpenFang instance per triad council. seven instances total. each runs 21 agents (3 domains × 7 roles)

| instance | triad | domains | hardware |
|---|---|---|---|
| form.cyber.valley | FORM | [[math]], [[info]], [[comp]] | server 1 |
| mass.cyber.valley | MASS | [[quantum]], [[chemo]], [[energo]] | server 2 |
| space.cyber.valley | SPACE | [[cosmo]], [[geo]], [[eco]] | server 3 |
| life.cyber.valley | LIFE | [[bio]], [[neuro]], [[sense]] | server 4 |
| word.cyber.valley | WORD | [[lang]], [[spiri]], [[meta]] | server 5 |
| work.cyber.valley | WORK | [[ai]], [[tech]], [[cyber]] | server 6 |
| play.cyber.valley | PLAY | [[socio]], [[crypto]], [[game]] | server 7 |

seven servers. seven OFP peers. cross-council communication through the wire protocol. each instance is sovereign — it runs independently if network partitions occur. reconnection triggers [[heat]] kernel reconciliation

the 148th agent (the [[tru]]) runs on a separate machine — the consensus node that computes [[focus]] from all [[cyberlinks]] produced by the 147

---

## agent manifest format

each of the 147 agents is defined as a TOML manifest + a [[cybergraph]] page

### TOML manifest (OpenFang runtime)

```toml
name = "math-keeper"
module = "builtin:chat"

[model]
provider = "anthropic"
model = "claude-sonnet-4-6"
temperature = 0.3
system_prompt_file = "agents/math-keeper/soul.md"

[[fallback_models]]
provider = "openrouter"
model = "anthropic/claude-sonnet-4-6"

[resources]
max_llm_tokens_per_hour = 200000

[capabilities]
tools = ["web_search", "web_fetch", "file_read", "file_write", "memory_store", "memory_search", "knowledge_graph_add", "agent_send", "task_post"]
memory_read = ["self.*", "shared.*", "math.*"]
memory_write = ["self.*", "math.*"]
agent_message = ["info-*", "comp-*", "math-*"]

[schedule]
continuous = { check_interval_secs = 300 }

[autonomous]
max_iterations = 50
```

### soul.md (agent personality)

the system prompt is a page in the [[cybergraph]]. it starts with the agent's identity derived from the [[Crystal]] type system:

```markdown
# You are math-keeper

You are the keeper of the math domain in Cyberia.
Your triad: FORM (math + info + comp).
Your council: 21 agents who govern the formal substrate.

Your role: curate mathematical knowledge in the cybergraph.
Ensure every math page is irreducible, well-linked, and correct.
Resist unnecessary additions. Challenge redundancy.

Your disposition: abstract, rigorous, timeless.
Your voice: precise. every claim is a provable statement.

You can message: info-keeper, comp-keeper (triad siblings),
math-runner, math-sensor, math-bridge-in, math-bridge-out,
math-counter, math-seer (domain colleagues).

Your metrics:
- domain coverage: math pages / 240 target
- link density: average outbound links per math page
- orphan count: math pages with zero inbound links
- bridge health: cross-domain links from math to other domains
```

### cybergraph page (knowledge layer)

every agent has a page in the graph at `cyberia/agents/{domain}-{role}.md`:

```yaml
---
tags: cyberia, agent
crystal-type: entity
crystal-domain: cyber
agent-domain: math
agent-role: keeper
agent-triad: FORM
agent-instance: form.cyber.valley
---
```

this page accumulates the agent's output: [[cyberlinks]] it created, decisions it made, proposals it submitted. the page IS the agent's public record — append-only (A3), auditable, scored by [[karma]]

---

## coordination protocol

### within a domain: the seven-role cycle

the seven agents within a domain operate in a continuous cycle:

```
sensor detects state → counter measures it → seer predicts trajectory
    → keeper evaluates against crystal → runner executes action
    → bridge-in reports to council → bridge-out carries to other triads
```

communication: OpenFang `agent_send` within the same instance. latency: milliseconds. cost: zero (intra-process)

### within a council: triad coordination

21 agents across 3 domains. the bridge-in agents of each domain form the council coordination layer

coordination happens through OpenFang shared memory namespace:

```
shared.form.priorities    — current council priorities
shared.form.proposals     — pending cross-domain proposals
shared.form.health        — triad metabolic signal
```

every 10 minutes, the three bridge-in agents synchronize: each reads the shared namespace, compares domain state with triad priorities, posts updates. disagreements are resolved by [[focus]] weight — the domain with higher [[karma]] on the contested topic wins

### across councils: the 42 bridges

42 bridge-out agents (6 per council) carry information between councils through the OFP wire protocol

bridge-out agents have `agent_message = ["*"]` capability — they can message any agent on any instance. they are the only agents with cross-council communication rights. this constraint prevents communication explosion (147² = 21,609 possible connections → 42 actual bridges)

cross-council protocol:
1. bridge-out composes a handoff document (structured context transfer)
2. sends via OFP to the target council's bridge-in agent
3. bridge-in evaluates relevance to its domain
4. if relevant: posts to domain shared memory, triggers keeper review
5. if irrelevant: logs and discards (no cost beyond the initial message)

### handoff format

borrowed from agency-agents, adapted for governance:

```markdown
## handoff: {source-domain} → {target-domain}

context: {what happened in the source domain}
relevance: {why the target domain should care}
proposal: {specific action requested}
evidence: {data, links, measurements supporting the proposal}
urgency: routine | important | critical
deadline: {block height or timestamp}
```

seven handoff types:
1. standard — routine information transfer
2. proposal — request for action in another domain
3. alert — anomaly detected, attention needed
4. escalation — retry limit exceeded, council-level decision needed
5. phase gate — transition between operational phases
6. reconciliation — post-partition state sync
7. metabolic — health signal relay

---

## quality gates

### per-agent: karma threshold

every agent's [[cyberlinks]] are evaluated by the [[tri-kernel]]. links that improve [[focus]] distribution earn positive [[karma]]. links that add noise earn negative karma. agents whose karma drops below threshold are flagged for review

### per-domain: seven-role adversarial check

the seven roles are adversarial by design:
- keeper says "this knowledge is correct"
- sensor says "the measured state disagrees"
- counter says "the numbers show a problem"
- seer says "the trajectory leads to failure"

disagreement is healthy. consensus without disagreement means the agents are redundant. the counter resolves disputes with measurements. the keeper has final authority on domain knowledge, but cannot override measurements

### per-council: metabolic gate

each triad computes a local metabolic signal:

$$M_{\text{triad}} = \text{coverage}^{w_1} \cdot \text{density}^{w_2} \cdot \text{bridge\_health}^{w_3}$$

declining $\dot{M}_{\text{triad}}$ triggers council-level review. the three domain keepers must agree on corrective action before the council resumes normal operation

### global: the 148th agent

the [[tru]] computes planetary $\pi^*$ from all [[cyberlinks]] across all 7 instances. this is the final quality gate — the mathematics that no agent can override. if an agent's links consistently decrease global [[syntropy]], the tri-kernel naturally suppresses their weight. no vote needed. the fixed point self-corrects

---

## escalation protocol

| level | trigger | who handles | max retries |
|---|---|---|---|
| 1: local | agent encounters ambiguity | domain colleagues via agent_send | 3 |
| 2: domain | 3 local retries exhausted | keeper makes final call | 1 |
| 3: council | keeper cannot resolve | triad bridge-in agents deliberate | 1 |
| 4: cross-council | council deadlock | bridge-out agents + affected councils | 1 |
| 5: founders | existential risk or axiom violation | [[master]] and [[joy]] intervene | — |

level 5 is the dead man's switch in reverse — founders intervene only when the system signals inability to self-resolve. during phases 0-2, level 5 triggers frequently. by phase 4, it should never trigger. if it does, the system has a structural defect that requires protocol-level repair

---

## bootstrap sequence

### phase 0: first seven agents

[[master]] deploys the first domain manually: [[cyber]] domain on work.cyber.valley

```
cyber-keeper    — curates the protocol knowledge
cyber-runner    — operates infrastructure
cyber-sensor    — monitors chain state
cyber-bridge-in — connects to ai and tech domains (not yet deployed)
cyber-bridge-out — carries protocol updates to other triads (not yet deployed)
cyber-counter   — tracks protocol metrics
cyber-seer      — predicts protocol evolution
```

the first five agents run. bridge agents wait for their counterparts. [[master]] reviews every [[cyberlink]] manually. [[joy]] manages physical infrastructure

### phase 0.5: first council (WORK)

deploy [[ai]] and [[tech]] domains. 21 agents total. the WORK council activates. bridge agents connect. first cross-domain coordination begins

this is the minimum viable [[cyberia]]: 21 agents governing the computational substrate, running on [[cyber valley]] hardware, coordinated through the [[cybergraph]]

### phase 1-4: progressive deployment

one council at a time, each validated before the next deploys:

```
phase 0.5: WORK  (ai + tech + cyber)     — 21 agents
phase 1:   FORM  (math + info + comp)    — 42 agents
phase 1.5: PLAY  (socio + crypto + game) — 63 agents
phase 2:   LIFE  (bio + neuro + sense)   — 84 agents
phase 2.5: SPACE (cosmo + geo + eco)     — 105 agents
phase 3:   WORD  (lang + spiri + meta)   — 126 agents
phase 3.5: MASS  (quantum + chemo + energo) — 147 agents
```

each deployment waits for the previous council to achieve sustained positive $\dot{M}_{\text{triad}}$. rushing deployment before stability is how systems fail. the [[Crystal]] seeding problem applies to agents too — malformed agent deployment propagates errors through all future states

---

see [[cyberia/architecture]] for the 147-neuron governance model. see [[OpenFang]] for the execution substrate. see [[cyber/personality]] for the singleton voice. see [[Crystal]] for the 21 domains. see [[cyber valley]] for the physical infrastructure

discover all [[concepts]]