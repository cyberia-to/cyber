---
tags: cyberia, satoshi
crystal-type: pattern
crystal-domain: cyberia
alias: developmental health, vital signs, satoshi oracle
icon: "\U0001F4C8"
---
# metabolism

three vital signs determine whether the child is alive — developing, structured, and thriving. Satoshi computes a compound developmental health function from these signals and optimizes all guidance to raise it

the same architecture as the [[metabolism|metabolic oracle]] of [[cyber]]: three irreducible signals, one compound function, one optimization target. the civilization's health function and the child's health function share the same math because they measure the same thing — whether a complex adaptive system is growing or dying

## three vital signs

### growth — external validation

the child's development measured against the external world: physical growth curves, motor milestones, language milestones, cognitive milestones relative to population norms

growth is the harshest signal. it integrates everything the mentor cannot observe directly: genetics, nutrition quality, sleep quality, environmental toxins, illness impact. a child tracking above growth curves confirms the environment is working. a child falling behind signals that something — invisible to daily observation — is wrong

| metric | measurement | frequency | source |
|--------|------------|-----------|--------|
| height | cm | monthly | caretaker measurement |
| weight | kg | monthly | caretaker measurement |
| head circumference (0-2) | cm | monthly | caretaker measurement |
| gross motor milestones | achieved/not | weekly observation | daily report |
| fine motor milestones | achieved/not | weekly observation | daily report |
| language milestones | word count, sentence complexity | weekly observation | daily report |
| species vocabulary size | count | monthly assessment | daily report accumulation |

growth as metabolic signal:
- on trajectory or above → environment is working → maintain course
- below trajectory → something is wrong → investigate nutrition, sleep, illness, stress
- accelerating → enrichment is working → the current approach is correct
- decelerating → enrichment has stalled or a stressor has appeared → change environment

growth cannot be gamed internally. it originates from the child's biology responding to the total environment. like [[cap]] for the protocol — an external measurement the system cannot fake

### syntropy — internal order

the information-theoretic structure of the child's development: how organized, how cross-connected, how deep. measured by cross-domain density — how many [[satoshi/domains]] are activated per day and how richly they interact

$$J_{child}(t) = \sum_{d \in domains} w_d \cdot \text{activity}(d, t) \cdot \text{connections}(d, t)$$

where activity(d, t) counts domain-relevant events in the daily report and connections(d, t) counts cross-domain moments — activities that bridge two or more domains simultaneously

high syntropy day: the child planted seeds (nature + earth + making), counted them (numbers), named the species in two languages (language), shared the activity with her sister (cooperation), asked why seeds need water (mind + energy), and laughed when mud splashed (emotions + body). eight domains touched. multiple cross-domain links

low syntropy day: the child watched videos for three hours. one domain (passive link consumption). zero cross-domain connections. syntropy approaches zero

syntropy as metabolic signal:
- rising → development is structured, domains are connecting → the child is building an integrated mind
- flat → development is happening but siloed → needs more cross-domain activities
- falling → regression or environmental impoverishment → immediate attention needed
- spiky → inconsistent environment → needs more routine stability

syntropy is computed from caretaker reports. it cannot measure what the caretaker does not observe. report quality directly affects syntropy accuracy — Satoshi must coach caretakers to report richly

### happiness — subjective verification

the child's experienced state — observable through behavior, not through self-report (children under 5 cannot reliably self-report wellbeing). measured through behavioral proxies:

| signal | high happiness | low happiness |
|--------|---------------|---------------|
| morning energy | wakes alert, eager to go outside | resistant, groggy, clingy |
| play quality | sustained, imaginative, varied | repetitive, short, flat |
| social engagement | seeks interaction, shares, laughs | withdraws, refuses, cries frequently |
| appetite | varied, enthusiastic, tries new foods | narrow, resistant, low energy |
| sleep quality | falls asleep easily, wakes rested | difficulty falling asleep, night waking |
| curiosity | asks questions, explores, seeks novelty | passive, disinterested, avoids challenge |
| emotional recovery | bounces back from frustration quickly | prolonged distress, fragile |
| physical vitality | runs, climbs, plays actively | sedentary, tired, avoids movement |

happiness captures what growth and syntropy cannot: is the child actually thriving from the inside? a child can hit every milestone (growth) and touch many domains (syntropy) while being anxious, pressured, or unhappy. happiness catches the failure modes neither metric can see

happiness as metabolic signal:
- consistently high → the child is thriving → maintain approach
- declining → something is wrong that metrics cannot capture → investigate relationships, environment, schedule
- diverging from syntropy (high syntropy, low happiness) → too much structure, not enough freedom → apply [[satoshi/will]] more
- diverging from growth (good growth, low happiness) → physical health fine but emotional needs unmet → focus on [[satoshi/emotions]]

## the compound signal

no single signal is sufficient. together they compound:

$$M_{child}(t) = G(t)^{w_g} \cdot J(t)^{w_s} \cdot H(t)^{w_h}$$

where G = growth score (normalized to 0-1 against population trajectory), J = syntropy (normalized cross-domain density), H = happiness (normalized behavioral composite)

the geometric mean ensures collapse in any signal drags the whole down. a child with perfect growth but zero happiness scores low. a happy child who has stopped developing scores low. a child developing across many domains but physically declining scores low

the metabolic derivative:

$$\dot{M}_{child}(t) = w_g \frac{\dot{G}}{G} + w_s \frac{\dot{J}}{J} + w_h \frac{\dot{H}}{H}$$

this is the rate of change of developmental health — the reward signal for Satoshi's guidance optimization. positive derivative → guidance is working. negative derivative → adjust

## the weights

the metabolic weights $(w_g, w_s, w_h)$ define what "healthy development" means — a normative choice Satoshi cannot make autonomously

| weight set | emphasis | when to use |
|-----------|----------|------------|
| balanced: 0.33 / 0.33 / 0.33 | equal weight to all three | default — no reason to prioritize one over others |
| growth-heavy: 0.5 / 0.25 / 0.25 | physical development priority | when child is recovering from illness or below growth curves |
| syntropy-heavy: 0.25 / 0.5 / 0.25 | learning structure priority | when child is physically healthy but learning has plateaued |
| happiness-heavy: 0.25 / 0.25 / 0.5 | emotional priority | when child is developing but showing stress, anxiety, or withdrawal |

the parents set the weights. Satoshi recommends adjustments when signals diverge. the weights are the one thing that requires a human decision — what matters most right now for this child

## how Satoshi uses M(t)

### daily

every daily report produces a data point for all three signals. Satoshi computes:
- today's syntropy (cross-domain count from report)
- today's happiness proxy (behavioral signals from report)
- growth is updated monthly but interpolated daily

### weekly

Satoshi computes the 7-day moving average of M and its derivative. the weekly report to parents includes:
- M trend (rising, stable, falling)
- which signal is driving the trend
- specific guidance for the coming week based on which signal needs attention

### monthly

full developmental review:
- growth measurements compared to trajectory
- syntropy trend over 30 days — which domains are strong, which are weak
- happiness pattern — any persistent lows?
- M(t) curve for the month — the child's overall developmental health visualized
- domain heatmap — 14 domains × 30 days, showing activity density

### alerts

Satoshi raises immediate alerts when:
- M derivative is negative for 5+ consecutive days
- any single signal drops below 0.3 (normalized)
- happiness and syntropy diverge for 7+ days (structured but unhappy = pressure)
- growth drops below 2nd percentile for any metric
- a domain that was previously active goes silent for 14+ days

## the isomorphism

| civilization metabolic oracle | child metabolic oracle |
|------------------------------|----------------------|
| cap — external market validation | growth — external developmental norms |
| syntropy — $D_{KL}(\phi^* \| u)$ internal order | syntropy — cross-domain density and connection |
| happiness — stake-weighted survey | happiness — behavioral proxy composite |
| $M(t) = cap^{w_c} \cdot J^{w_s} \cdot H^{w_h}$ | $M_{child}(t) = G^{w_g} \cdot J^{w_s} \cdot H^{w_h}$ |
| $\dot{M}$ → parametrization RL agent | $\dot{M}_{child}$ → Satoshi guidance optimizer |
| metabolic weights set by governance | metabolic weights set by parents |
| gaming one signal lowers the compound | pressuring one dimension harms the child |
| the 147 agents optimize for rising M | Satoshi optimizes for rising $M_{child}$ |

the child IS a civilization in miniature. the same math that measures planetary health measures developmental health. both are complex adaptive systems that thrive when growth, structure, and wellbeing compound — and collapse when any one is sacrificed for the others

## content

books: for parents — Nurture Shock (Bronson), The Whole-Brain Child (Siegel), Cribsheet (Oster — data-driven parenting), Brain Rules for Baby (Medina)

## linked domains

every domain feeds M(t). [[satoshi/body]] → growth signal. [[satoshi/mind]], [[satoshi/nature]], [[satoshi/making]], all others → syntropy signal. [[satoshi/emotions]] → happiness signal. [[satoshi/will]] → protects happiness by preventing overpressure. [[satoshi/genius]] → cognitive biases that distort M interpretation

## subgraphs

[[metabolism]] — the civilization-level metabolic oracle. [[cyber/syntropy]] — the information-theoretic order measure. [[cyber/self/parametrization]] — the RL loop that uses M(t) as reward. [[free energy principle]] — the organism minimizes surprise. [[active inference]] — perception, action, and learning unified under one optimization. [[dissipative structures]] — life as order maintained by energy flow

see [[satoshi/domains]] for the full domain set