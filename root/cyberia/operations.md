---
tags: cyberia, core
crystal-type: process
crystal-domain: cyber
crystal-size: deep
diffusion: 0.00012302117848223706
springs: 0.0009703709179984784
heat: 0.0007160975325089295
focus: 0.0004958413711424625
gravity: 1
density: 1.71
---
management objects and process system for 147 [[neurons]] — what they control and how work flows through [[cyber valley]]

---

## management objects

every object the agent network manages is a [[particle]] in the [[cybergraph]]. agents create [[cyberlinks]] to objects, recording state, decisions, and measurements. the graph IS the management system

### 1. territory

37 hectares divided into operational zones

| zone | area | function | managing agents |
|---|---|---|---|
| gardens | ~5 ha | food production, [[permaculture]], [[species]] cultivation | SPACE eco-runner, LIFE bio-runner |
| forest | ~15 ha | timber ([[bamboo]], [[trema]], [[albizia chinensis]]), carbon stock, biodiversity | SPACE eco-keeper, eco-sensor |
| infrastructure | ~3 ha | buildings, roads, parking, solar arrays, water systems | WORK tech-runner |
| pasture | ~4 ha | sheep, chickens, composting, fodder | LIFE bio-runner |
| events | ~2 ha | [[soft3]], outdoor spaces, [[Burn.City]] grounds | PLAY socio-runner |
| nursery | ~1 ha | seedlings, propagation, experiments | LIFE bio-keeper |
| buffer/reserve | ~7 ha | untouched forest, watershed protection, wildlife corridor | SPACE eco-sensor (monitor only) |

each zone is a particle. zone state (planted species, soil tests, water flow, canopy cover) is recorded as [[cyberlinks]] from the zone particle to measurement particles. history is append-only

### 2. buildings

| building | function | area | managing agent | key processes |
|---|---|---|---|---|
| [[soft3]] | event space, co-working | 200 m² | PLAY socio-runner | bookings, events, maintenance |
| [[organiq]] | food store, cafe, kitchen | 80 m² | SPACE eco-runner | menu, inventory, food safety |
| [[elona]] | energy showcase, education | 60 m² | WORK tech-runner | solar monitoring, tours |
| [[laba]] | construction hub, workshop | 100 m² | WORK tech-runner | tool inventory, projects |
| Satoshi | children space | 40 m² | — | not agent-managed |
| [[banya]] | sauna, wellness | 50 m² | PLAY socio-runner | bookings, wood supply |
| [[vitalik]] | gym, training | 80 m² | LIFE neuro-runner | equipment, schedules |
| [[sinwood]] | bioluminescent forest, 200 light points | 2 ha | PLAY socio-runner | maintenance, tours |
| carrot house | main residence | 120 m² | — | founder private |
| staff quarters | employee housing | 100 m² | — | [[joy]] manages |

### 3. infrastructure systems

| system | components | sensor data | managing agent | failure mode |
|---|---|---|---|---|
| solar | 30 kW panels, batteries, inverters | voltage, current, charge level, yield | WORK tech-sensor | grid fallback → manual |
| water | rain collection, 200 m³ storage, gravity distribution, filtration | flow rates, tank levels, quality | SPACE geo-sensor | rationing protocol |
| internet | fiber uplink, mesh network, local servers | latency, throughput, uptime | WORK tech-runner | mobile hotspot fallback |
| waste | composting, biochar kiln, recycling | compost temperature, volume | LIFE bio-sensor | manual sorting |
| roads | 14 km hiking paths, 5 parking zones, access roads | erosion, drainage | SPACE geo-sensor | seasonal repair schedule |
| security | perimeter, lighting, access control | camera feeds, motion sensors | WORK tech-sensor | staff patrol |

### 4. biological assets

| category | inventory | tracking | managing agent |
|---|---|---|---|
| coffee plantations | ~2000 trees, arabica, multiple varieties | age, yield per tree, processing batch | SPACE eco-counter |
| fruit trees | avocado, jackfruit, sapote, citrus, berries | species, age, location, yield | LIFE bio-counter |
| spice gardens | turmeric, galangal, ginger, cinnamon | plot, harvest date, quantity | SPACE eco-runner |
| timber species | bamboo, trema, albizia, caliandra, leucaena, ficus | growth rate, harvest schedule | SPACE eco-seer |
| mushrooms | 50+ documented wild species | location, season, identification | LIFE bio-keeper |
| livestock | 5 sheep, 10 chickens | health, feed, output | LIFE bio-runner |
| medicinals | tulsi, lemongrass, turmeric, gotu kola | plot, harvest, drying | LIFE bio-runner |
| nursery seedlings | propagation stock for all species | species, quantity, readiness | LIFE bio-keeper |

### 5. people

| group | count | interface | managing agent | what agents do | what agents never do |
|---|---|---|---|---|---|
| local staff | 32 | task board + Telegram | runners | assign tasks, schedule, track completion | hire, fire, set pay, resolve conflicts |
| residents | 4 core + guests | dashboard + Telegram | bridge agents | provide information, coordinate | make promises, enforce rules |
| event guests | variable (up to 150 for [[Burn.City]]) | booking system | PLAY socio-runner | booking, logistics, communication | negotiate custom deals |
| Nandu farmers | program participants | weekly check-ins | LIFE bio-runner | plot allocation, harvest tracking, knowledge sharing | financial agreements |
| remote contributors | developers, writers | GitHub + Telegram | WORK cyber-bridge | code review, content coordination | employment offers |

[[joy]] is the human interface for all people decisions. agents coordinate logistics. humans manage relationships

### 6. financial objects

| object | type | managed by | tracking |
|---|---|---|---|
| PT PMA bank account | Indonesian operating entity | [[joy]] signs, counter tracks | monthly reconciliation |
| MiDAO treasury | offshore entity | [[master]] signs, counter tracks | on-chain + off-chain |
| $CYB staking | protocol tokens | PLAY crypto-runner | delegation, yield, compounding |
| event revenue | cash + transfer | PLAY socio-counter | per-event P&L |
| product revenue | Organiq sales, coffee, produce | SPACE eco-counter | per-product margin tracking |
| accommodation revenue | glamping, nomad hub | PLAY socio-counter | occupancy, RevPAR |
| petty cash | local operations | runners, <$100 autonomous | daily log |

### 7. digital assets

| asset | location | managed by |
|---|---|---|
| cyber knowledge graph | github.com/cyberia-to/cyber | WORK cyber-keeper |
| 12 subgraph repos | github.com/cyberia-to/* | respective keepers |
| cyber.page domain + hosting | Netlify + Namecheap | WORK cyber-runner |
| cyb.ai domain | DNS | WORK cyber-runner |
| social accounts (X, Telegram) | platform accounts | bridge-out agents draft, founders approve |
| bostrom validator | on-chain | PLAY crypto-runner |
| IPFS pinning (Pinata) | cloud service | WORK cyber-runner |

---

## process system

every process is a cycle: sense → decide → act → measure. sensors trigger it, runners execute it, counters verify it, keepers record it

### daily processes

| process | time | trigger | agents involved | output |
|---|---|---|---|---|
| morning briefing | 06:00 | cron | all sensors → bridge-out | daily status to Telegram: weather, solar yield, water levels, task queue |
| staff task assignment | 07:00 | briefing complete | runners | task board updated with day's work per zone |
| harvest check | 08:00 | seasonal calendar + sensor data | eco-sensor, bio-runner | harvest list: what to pick today, which zone |
| infrastructure check | 09:00 | sensor readings | tech-sensor | alert if any system anomalous, otherwise silent |
| guest coordination | 10:00 | booking calendar | socio-runner | arrivals, departures, special requests for today |
| graph maintenance | continuous | new content, broken links, stale pages | cyber-keeper | edits, new pages, link repairs |
| evening report | 18:00 | cron | counters → bridge-out | daily P&L, task completion rate, anomalies |
| night monitoring | 22:00-06:00 | continuous | sensors (low power mode) | alerts only: security, weather, infrastructure |

### weekly processes

| process | day | trigger | agents involved | output |
|---|---|---|---|---|
| zone planning | Monday | weekly cycle | keepers + runners per zone | week's work plan: which zones, which tasks, which staff |
| market/sales | Tuesday | inventory levels + demand | eco-runner, socio-runner | what to sell, where, at what price |
| maintenance cycle | Wednesday | sensor reports + staff feedback | tech-runner | preventive maintenance schedule |
| knowledge review | Thursday | graph metrics | cyber-keeper, domain keepers | pages to create, update, or link |
| council sync | Friday | weekly cycle | bridge-in agents from each active council | cross-domain priorities, resource conflicts |
| financial review | Saturday | counter reports | counters → bridge-out | weekly P&L, budget vs actual, cash position |
| metabolic review | Sunday | weekly M(t) computation | seer agents | $\dot{M}$ trajectory, which component needs attention |

### monthly processes

| process | trigger | agents involved | output | founder involvement |
|---|---|---|---|---|
| payroll preparation | month end | counters | staff hours, deductions, net pay | [[joy]] approves and signs |
| P&L closing | month end | all counters | consolidated monthly financials | founder review |
| species inventory | monthly | bio-counter | updated species database, growth measurements | none |
| infrastructure audit | monthly | tech-sensor + tech-counter | equipment status, replacement schedule | founder review if >$1000 |
| content publication | monthly | cyber-keeper + bridge-out | monthly highlights for cyber.page, social | bridge drafts, keeper reviews |
| tri-kernel recompute | monthly | WORK cyber-runner | fresh focus scores in all frontmatter, context packs rebuilt | none (automated) |

### seasonal processes

| process | season | trigger | agents involved | output |
|---|---|---|---|---|
| coffee harvest | May-Aug | cherry ripeness (eco-sensor) | eco-runner coordinates staff, bio-runner quality control | dried parchment → processing → roasting → sales |
| planting cycle | Oct-Nov (wet season start) | soil moisture + calendar | eco-keeper selects species, bio-runner coordinates planting | new plantings recorded, nursery restocked |
| [[Burn.City]] | annual (Feb-Mar) | calendar + planning 6mo ahead | all PLAY council + SPACE eco (food) + WORK tech (infra) | event execution: 150 people, 30 days |
| dry season prep | Apr | weather forecast + water levels | geo-sensor, tech-runner | water rationing plan, fire prevention, mulching |
| wet season prep | Sep | weather forecast | tech-runner, eco-runner | drainage check, erosion control, greenhouse prep |
| annual assessment | Dec | year end | all seers → bridge-out | year in review: metabolic trajectory, achievements, failures, plan for next year |

### event-driven processes (ad-hoc)

| trigger | process | responding agents | escalation |
|---|---|---|---|
| booking request | qualify → price → confirm → prepare | socio-runner | auto for standard, L1 for custom |
| equipment failure | detect → diagnose → repair/replace | tech-sensor → tech-runner | L2 if >$500 replacement |
| weather emergency | alert → protect → assess damage | geo-sensor → all runners | L3 if structural damage |
| new species discovered | document → identify → catalog → plan | bio-sensor → bio-keeper | none (knowledge action) |
| visitor/media inquiry | receive → qualify → draft response → review | bridge-in → bridge-out → keeper | L3 for media, L1 for casual |
| staff issue | surface concern → escalate to [[joy]] | any agent detects → bridge-in | always L3 (human domain) |
| security incident | detect → alert → respond | tech-sensor → [[master]] direct | always L3 |
| cash flow crunch | counter detects shortfall → alert | all counters → seers → founders | L3 — founder decision |

---

## process dependencies

```
              morning briefing
              /       |       \
    staff tasks    harvest    infra check
         |            |           |
    zone work    processing   maintenance
         |            |           |
    evening report ←──┴───────────╯
         |
    daily P&L
         |
    weekly review (Friday) ←── all daily P&Ls
         |
    metabolic review (Sunday)
         |
    next week planning (Monday)
```

the cycle is fractal: daily loops feed weekly reviews, weekly reviews feed monthly P&L, monthly P&L feeds seasonal planning, seasonal planning feeds annual assessment. each layer operates at its own timescale. the [[heat]] kernel smooths across scales — daily noise washes out, seasonal signal survives

---

## object-process matrix

which agents touch which objects:

| | territory | buildings | infra | biology | people | finance | digital |
|---|---|---|---|---|---|---|---|
| keeper | defines zones | — | — | species catalog | — | — | graph pages |
| runner | coordinates work | manages usage | operates | planting/harvest | assigns tasks | executes payments | CI/CD, deploys |
| sensor | soil, water, weather | occupancy, condition | system health | pest, disease, growth | — | — | uptime, metrics |
| bridge-in | triad coordination | — | — | — | resident interface | — | — |
| bridge-out | external partnerships | event promotion | — | research collabs | guest communication | investor updates | social, PR |
| counter | area productivity | utilization rates | cost tracking | yield per species | labor hours | P&L, budgets | token costs |
| seer | land use planning | expansion proposals | upgrade planning | succession modeling | capacity planning | revenue forecasting | growth projections |

---

see [[cyberia/architecture]] for the governance structure. see [[cyberia/agents]] for the technical deployment. see [[cyberia/deployment]] for action authorization. see [[cyberia/cybernetics]] for self-regulation. see [[cyber valley]] for the physical site

discover all [[concepts]]