---
tags: cyberia, article
crystal-type: entity
crystal-domain: cyberia
alias: automation, sensors dev control, community leadership
icon: "📡"
---

# sensors, dev and control

how a 37-hectare network state closes the loop between physical reality and digital governance

---

## the problem with autonomy claims

every intentional community claims autonomy. few can prove it. the difference between a claim and a fact is a measurement system. a city that cannot measure its own energy production, water quality, food yield, and network uptime cannot govern those systems — it is governed by them.

[[cyber valley]] runs on seven sovereignty layers: data, computation, energy, food, water, shelter, finance. each layer requires sensors, each sensor requires dev infrastructure to process and authenticate its readings, each reading requires a control loop that can act on the data. this is what sensors, dev and control means: the complete feedback architecture of a living city.

---

## sensors — reading the physical state

### what we measure

across the 37 ha estate, sensors form a distributed nervous system:

- soil — moisture, temperature, pH, nitrogen, carbon content across terraced growing zones
- water — spring flow rates, reservoir levels, filtration output quality, irrigation pressure
- energy — solar production per array, battery state-of-charge, grid draw/export balance, load by venue
- climate — microclimate temperature and humidity gradients across seven canyons and elevation bands
- air — CO₂ concentration in enclosed venues (soft, organiq, laba workshop)
- biological — camera trap motion for wildlife corridors, acoustic monitoring for bird presence

### why robonomics

[[robonomics]] is a web3-native IoT protocol. each sensor node publishes signed readings as transactions to a substrate-based chain. this is not telemetry — it is authenticated data. every reading carries the cryptographic identity of the device that produced it, a block timestamp, and a content-addressed hash linkable to the [[cybergraph]].

the consequence: physical state becomes a particle. soil moisture at field #4 at 06:30 on a given day is a unique, provable, permanent fact — not a log entry that can be overwritten, not a dashboard number that exists only in RAM. it is a [[cyberlink]] from the sensor's identity to a measurement particle.

this is data sovereignty applied to physical reality. when the city's water system reads "spring flow: 4.2 L/s", that fact belongs to the city's graph, not to a cloud service.

---

## dev — the software substrate

### github projects for coordination

city development runs through [[github]] projects. each project is a bounded effort with clear scope, timeline, and accountable person. current active tracks:

- sensor network expansion: 12 nodes live, 40 planned across three elevation zones
- robonomics integration: data pipeline from sensor nodes into the cybergraph
- KPI dashboard: real-time view of the eight autonomy metrics
- optica publisher: the knowledge graph publishing engine
- cyb.land: the visitor-facing website and booking layer

github projects provide linear task tracking. crucially, they are the shared reality for a distributed team: a task is either open, in-progress, or done — no ambiguity, no status meetings.

the bridge between github coordination and the [[cybergraph]] is documentation: every completed project produces a page in the graph. the graph is the city's memory. github is its nervous system during construction.

### the stack

sensor nodes run nushell scripts that push to robonomics. the data pipeline validates schemas, transforms readings into cybergraph-compatible particles, and submits them via the standard content-addressing interface. optica renders the graph into the public web. the booking layer ([cyb.land](/cyber-valley/cyb.land)) lets visitors interact with the physical estate through the digital interface.

all infrastructure runs on-site: two servers in the data room, solar-powered, connected by the estate's fiber mesh. no dependency on external hosting for core city functions.

---

## control — closing the loop

### the eight autonomy KPIs

measuring is not enough. measurement produces value when it changes behavior. the KPI framework for [[cyber valley]] tracks eight metrics, each representing a sovereignty dimension:

| KPI | target | current |
|---|---|---|
| energy self-sufficiency | >90% solar | 70% |
| water self-sufficiency | >95% on-site | 85% |
| food self-sufficiency | >60% on-site calories | 40% |
| network uptime | >99.5% | 99.1% |
| sensor coverage | >80% critical areas | 30% |
| response latency | <4h anomaly to action | 12h |
| carbon balance | net negative | measuring |
| biodiversity index | trending up | 12 mo baseline |

the KPI review happens weekly. each metric has an owner (a [[runner]] role in the architecture). deviations from target trigger a project: identify root cause, design intervention, implement, measure result.

### control architecture

the full loop:

```
sense → transmit → validate → index → alert → decide → act → sense
```

- sense: robonomics node reads physical state every 15 min
- transmit: signed transaction to substrate chain
- validate: schema check, plausibility filter (spike detection), deduplication
- index: particle written to cybergraph, linked to location and timestamp
- alert: if reading crosses threshold (e.g. soil moisture below 20%), notify the responsible runner
- decide: runner assesses — automated irrigation trigger, or human inspection required
- act: valve opens, pump starts, or a person walks to the site
- sense: next reading confirms the action took effect

the threshold parameters themselves are tracked in the graph. adjusting a threshold is a [[cyberlink]] — a decision with a timestamp, author, and rationale. there are no hidden settings.

### from reactive to predictive

the current system is reactive: it reads state and alerts on anomalies. the next phase is predictive: training models on twelve months of sensor history to anticipate failures before they happen.

soil moisture trends predict irrigation needs 48 hours ahead, allowing pre-scheduled watering rather than emergency response. energy production forecasts (based on weather sensor data + historical solar patterns) allow battery charging schedules to be optimized overnight. spring flow seasonality models predict dry-season shortfalls months in advance.

this is the path from a city that responds to a city that anticipates — the distinction between reactive governance and [[autonomous governance]].

---

## community leadership through transparency

the sensor network is not only an operational tool. it is a governance tool.

every resident, visitor, and stakeholder with network access can see the city's vital signs in real time. soil carbon is rising — the [[permaculture]] methods are working. water use per resident is below target — the conservation culture is real. energy export is positive — the city is net contributor, not consumer.

transparent metrics replace authority with accountability. a manager cannot claim the water system is fine when the sensor says otherwise. a worker cannot dispute the harvest record when the yield is logged. the data is the authority.

this is what community leadership through sensors means: the city leads by showing its own state, not by asserting it. residents who can see the metrics become stewards of the metrics. the feedback loop from sensor to display to behavior is the loop from monitoring to culture.

---

## sequencing

phase 1 (complete): establish data infrastructure, deploy 12 nodes at critical points (water intake, main solar array, food storage temperature, server room), integrate robonomics pipeline, establish KPI baseline.

phase 2 (in progress): expand to 40 nodes, automate irrigation control loops, build KPI dashboard visible to residents, publish weekly autonomy report as cybergraph particle.

phase 3 (planned): predictive models on 12-month dataset, integrate sensor readings into resident [[karma]] feedback (stewards of systems they monitor earn recognition), open sensor API for external researchers studying intentional communities.

---

see [[cyber valley]] for the physical estate. see [[autonomous governance]] for how sensor data feeds governance. see [[robonomics]] for the IoT protocol. see [[bostrom]] for the chain where sensor particles live. see [[sensors]] for the sensor role in the 147-agent architecture.

discover all [[concepts]]
