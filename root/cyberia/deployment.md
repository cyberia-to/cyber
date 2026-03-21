---
tags: cyberia, core
crystal-type: process
crystal-domain: cyber
crystal-size: deep
---
what must be defined before the first agent runs — the gap between [[cyberia/architecture]] theory and operational reality

---

## the gaps

the architecture defines WHO (147 agents, 7 roles, 21 domains). the agents page defines WHERE (OpenFang, 7 servers). neither defines WHAT agents actually do minute-to-minute, HOW they affect the real world, or WHAT controls prevent catastrophic decisions

---

## 1. action space: what can agents DO?

every agent action falls into one of six categories. each category needs explicit authorization levels

### cyberlinks (knowledge)

the core action. agents create [[cyberlinks]] in the [[cybergraph]] — assertions connecting two [[particles]]

| action | example | authorization | risk |
|---|---|---|---|
| create page | write new knowledge to the graph | agent autonomous | low — append-only, reversible by weight |
| edit page | update existing content | agent autonomous | low — old version preserved |
| link pages | connect two concepts | agent autonomous | low — the graph self-corrects via tri-kernel |
| delete link | retract an assertion (v = -1) | agent autonomous | low — retraction is itself a cyberlink |

cyberlinks are the safest action. append-only (A3), self-correcting (tri-kernel), and the worst case is noise that gets suppressed by low karma

### code (commits)

agents that maintain protocol code or infrastructure scripts

| action | example | authorization | risk |
|---|---|---|---|
| create PR | propose code change | agent autonomous | low — PR is a proposal, not a change |
| merge PR | accept code change | keeper + 1 reviewer | medium — bad code breaks systems |
| deploy | push to production | runner + keeper approval | high — downtime, data loss |
| config change | modify parameters | counter verifies metrics first | medium — wrong params degrade performance |

rule: no agent merges its own PR. the runner proposes, the keeper reviews. for protocol code ([[hemera]], [[nox]], [[zheng]]), [[master]] holds veto during phases 0-2

### money (treasury)

agents that allocate resources from the treasury

| action | example | authorization | risk |
|---|---|---|---|
| expense < $100 | buy supplies, pay small service | runner autonomous | low — bounded loss |
| expense $100-$1000 | equipment, contractor day | runner + counter approval | medium |
| expense $1000-$10000 | infrastructure, events | keeper + council vote | high |
| expense > $10000 | land, major equipment, salaries | [[master]] and [[joy]] sign | critical |

rule: every expense is a [[cyberlink]] from the treasury particle to the expense particle, with conviction = amount. auditable, append-only, on-graph

### people (coordination)

agents that coordinate with the 32 local employees and residents

| action | example | authorization | risk |
|---|---|---|---|
| assign task | "harvest section B tomorrow" | runner posts to task board | low |
| schedule | set weekly work plan | runner + sensor validates capacity | low |
| hire | bring on new staff | keeper + [[joy]] approval | high — legal, cultural |
| fire | terminate employment | [[joy]] only | critical — legal liability |
| pay salary | monthly payroll | runner executes, counter verifies, [[joy]] signs | high |

rule: agents NEVER fire people. agents NEVER make promises about compensation. agents coordinate work, humans manage relationships. [[joy]] is the interface between agents and local staff

### infrastructure (physical)

agents that manage village infrastructure

| action | example | authorization | risk |
|---|---|---|---|
| monitor | check solar output, water levels | sensor autonomous | zero |
| alert | "battery below 20%, switching loads" | sensor autonomous | low |
| adjust | change irrigation schedule | runner + sensor agrees | low |
| repair request | "pump 3 needs maintenance" | sensor creates task, runner assigns | low |
| construction | build new structure | [[master]] and [[joy]] approve design | critical |

rule: agents monitor and alert autonomously. agents adjust parameters within bounds. agents NEVER authorize physical construction or modification without founder approval

### communication (external)

agents that represent [[cyberia]] to the outside world

| action | example | authorization | risk |
|---|---|---|---|
| post to graph | publish to cyber.page | agent autonomous | low |
| post to social | tweet, telegram message | bridge-out + keeper review | medium — reputational |
| respond to inquiry | answer questions about cyberia | bridge-out with approved templates | medium |
| sign contract | legal commitment | [[master]] or [[joy]] only | critical |
| public statement | official position on issue | council consensus + founder review | critical |

rule: agents NEVER make legal commitments. agents NEVER speak "on behalf of cyberia" without template approval. bridge-out agents draft, keepers review, founders sign

---

## 2. authority matrix

| authority level | who decides | latency | examples |
|---|---|---|---|
| L0: autonomous | agent alone | instant | cyberlinks, monitoring, alerts, internal task posts |
| L1: peer review | agent + 1 domain colleague | minutes | code PRs, schedule changes, small expenses |
| L2: council | 3+ agents from the triad | hours | medium expenses, parameter changes, hiring proposals |
| L3: founders | [[master]] and/or [[joy]] | hours-days | large expenses, legal, construction, firing, public statements |
| L4: protocol | on-chain governance | days-weeks | protocol upgrades, metabolic weight changes |

every action maps to exactly one level. the level is defined by the action category + amount/impact, not by the agent's role. a keeper creating a cyberlink = L0. a keeper proposing to hire = L3

---

## 3. budget model

### per-agent cost

| component | monthly cost | notes |
|---|---|---|
| LLM tokens | $50-500 | depends on model and activity level |
| compute (OpenFang) | $10-30 | shared server, amortized |
| storage | $1-5 | SQLite, minimal |
| communication channels | $0-10 | Telegram bot, API calls |
| total per agent | $60-550 | |

### fleet cost by phase

| phase | agents | monthly cost | funded by |
|---|---|---|---|
| 0: first 7 | 7 | $400-3,500 | founders |
| 0.5: WORK council | 21 | $1,200-10,000 | founders + early revenue |
| 1-2: 4 councils | 84 | $5,000-40,000 | treasury + event revenue |
| 3-4: full fleet | 147 | $9,000-80,000 | protocol treasury |

at scale, the fleet cost is bounded by the metabolic signal: if $\dot{M}$ is negative, agents reduce activity (lower token usage). if $\dot{M}$ is positive, agents can increase activity. the budget is self-regulating

### model selection strategy

| agent role | model | why |
|---|---|---|
| keeper | Claude Opus / Sonnet | needs deep reasoning for knowledge curation |
| runner | Claude Haiku / GPT-4o-mini | fast execution, less reasoning |
| sensor | local model (Llama 70B) | continuous monitoring, low latency, no API cost |
| bridge | Claude Sonnet | needs good communication, cross-domain synthesis |
| counter | local model + code | metrics are computed, LLM only for interpretation |
| seer | Claude Opus | needs strongest reasoning for prediction |

---

## 4. risk controls

### spending limits

every agent has a daily spending cap in its TOML manifest:

```toml
[resources]
max_spend_per_day_usd = 50
max_spend_per_action_usd = 10
max_llm_tokens_per_hour = 100000
```

exceeding the cap triggers automatic suspension + alert to keeper

### kill switch

[[master]] and [[joy]] can suspend any agent instantly via:
- OpenFang admin API: `POST /agents/{id}/suspend`
- dead man's switch: if founders don't heartbeat within 48h, all L2+ actions freeze
- emergency stop: physical network disconnect at [[cyber valley]]

### audit trail

every agent action is a [[cyberlink]]. the graph IS the audit trail:
- who: agent neuron ID (signed)
- what: action particle (typed)
- when: block height (timestamped)
- how much: conviction amount (staked)
- outcome: subsequent links show result

no agent can hide an action. append-only (A3) means the record is permanent

### failure modes

| failure | detection | response | recovery |
|---|---|---|---|
| agent produces noise | karma drops below threshold | auto-suspend, keeper review | retrain or replace model |
| agent overspends | counter detects budget exceeded | auto-suspend, freeze wallet | founder reviews, adjusts limits |
| agent contradicts policy | seer detects divergence from graph | alert council, hold actions | council reviews, may retrain |
| agent goes silent | heartbeat missed for 3 cycles | runner takes over tasks temporarily | diagnose: model API down? server crash? |
| malicious prompt injection | sensor detects anomalous output pattern | isolate agent, alert founders | forensic review of conversation history |
| all agents down | infrastructure monitoring (separate from agents) | founders operate manually | restart OpenFang, restore from SQLite |

---

## 5. day one: the first seven agents

the cyber domain on work.cyber.valley. what each agent does on day one:

### cyber-keeper

- reads all pages in the [[cyber]] namespace
- identifies pages missing frontmatter, broken links, stale content
- proposes edits via PR to the cyber repo
- reviews PRs from other agents
- daily output: 5-20 page edits, 0-3 new pages

### cyber-runner

- monitors CI pipeline (GitHub Actions → Netlify)
- triggers rebuilds when content changes
- manages optica build process
- executes routine infrastructure tasks
- daily output: 10-50 automated operations

### cyber-sensor

- monitors cyber.page uptime and response times
- tracks graph metrics: page count, link density, orphan count
- monitors bostrom chain state
- posts hourly status to shared memory
- daily output: 24 status reports, 0-5 alerts

### cyber-bridge-in

- waits for ai and tech domains (not yet deployed)
- initially: reads external AI/tech news, creates relevant cyberlinks
- daily output: 5-15 curated external links

### cyber-bridge-out

- waits for other councils (not yet deployed)
- initially: posts daily summary to Telegram channel
- daily output: 1 daily digest

### cyber-counter

- computes domain metrics: coverage (pages / 240 target), density, orphan rate
- tracks tri-kernel scores for cyber namespace pages
- reports weekly trends
- daily output: 1 metrics report, continuous background computation

### cyber-seer

- analyzes graph growth trajectory
- predicts which pages will become high-gravity based on link patterns
- proposes priority pages to create
- daily output: 1 priority report, 3-5 page proposals

---

## 6. integration points

### with existing systems

| system | how agents connect | who manages the connection |
|---|---|---|
| GitHub (cyberia-to org) | API tokens in OpenFang vault | cyber-runner |
| Netlify (cyber.page) | deploy key in vault | cyber-runner |
| Bostrom chain | RPC endpoint | cyber-sensor |
| Telegram (@cyberia channel) | bot token in vault | cyber-bridge-out |
| cyber.page (optica) | git push triggers rebuild | cyber-runner |
| local servers (cyber valley) | SSH keys in vault | cyber-runner |

### with human staff

| interface | channel | managed by |
|---|---|---|
| task assignment | shared Telegram group or task board web UI | runners post tasks |
| daily standup | Telegram message from bridge-out agent | automated, 8am local |
| emergency alerts | Telegram DM to [[master]] and [[joy]] | sensors |
| payroll data | shared spreadsheet or accounting tool | counters prepare, [[joy]] approves |

### with founders

| channel | purpose | frequency |
|---|---|---|
| Telegram DM | alerts, approvals for L3 actions | as needed |
| weekly summary | council-level report generated by bridge-out | weekly |
| dashboard (OpenFang web UI) | real-time agent status, metrics, costs | always available |
| GitHub notifications | code review requests from agents | as needed |

---

## 7. deployment checklist

before the first agent runs:

### infrastructure
- [ ] OpenFang installed on work.cyber.valley server
- [ ] SQLite database initialized
- [ ] API keys provisioned: Anthropic, OpenRouter (fallback)
- [ ] Network: server accessible from internet for Telegram webhook
- [ ] Monitoring: separate process watches OpenFang health

### credentials (in OpenFang AES-256-GCM vault)
- [ ] GitHub PAT (cyberia-to org, repo scope)
- [ ] Netlify deploy token
- [ ] Telegram bot token
- [ ] Bostrom RPC endpoint
- [ ] Anthropic API key
- [ ] OpenRouter API key (fallback)

### agent manifests
- [ ] 7 TOML files in `agents/cyber-{role}/agent.toml`
- [ ] 7 soul.md files with personality prompts
- [ ] spending limits configured per agent
- [ ] communication permissions (agent_message) scoped correctly

### knowledge base
- [ ] cyber namespace pages reviewed and up to date
- [ ] tri-kernel recomputed with latest data
- [ ] context pack generated for agent consumption

### human readiness
- [ ] [[master]] available for L3 approvals via Telegram
- [ ] [[joy]] available for people-related decisions
- [ ] local staff briefed: "you may receive task assignments from a bot"
- [ ] emergency procedures documented and tested

### validation before going live
- [ ] run all 7 agents in dry-run mode (actions logged, not executed) for 48h
- [ ] review dry-run output: are the actions sensible?
- [ ] test kill switch: can founders suspend agents instantly?
- [ ] test failure recovery: kill OpenFang, verify SQLite restore
- [ ] verify audit trail: can you trace every action back to an agent?

---

see [[cyberia/architecture]] for the governance model. see [[cyberia/agents]] for the technical deployment. see [[master]] and [[joy]] for the founders who authorize L3 actions

discover all [[concepts]]
