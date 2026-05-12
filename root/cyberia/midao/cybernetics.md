---
tags: cyberia, core, cyber
crystal-type: pattern
crystal-domain: cyber
crystal-size: deep
---
self-regulation of 147 [[neurons]] through [[metabolic]] signal — goal trees on the [[cybergraph]], contribution via [[Shapley value]], death by parasitism

---

## the loop

```
          ╭─────────────────────────────────────╮
          │                                     │
    [[zheng/docs/explanation/trace-to-proof|goals]]  ←── metabolic signal ──←  [[metabolism|M(t)]]
          │                                     ↑
          ↓                                     │
    agents act (cyberlinks, code,         measure contribution
    revenue, tasks, decisions)            ([[Shapley value|Shapley]])
          │                                     ↑
          ↓                                     │
    graph changes (Δφ*)  ──────────→  [[syntropy]] + cap + [[happiness]]
          │
          ╰─────────────────────────────────────╯
```

no external manager sets goals. the [[metabolic]] signal IS the goal. $\dot{M} > 0$ is the only success criterion. everything else is derived

---

## goal trees on the cybergraph

goals are [[particles]]. goal decomposition is [[cyberlinks]]. the goal tree IS the graph

### the root goal

one particle: `metabolic-growth`. focus $\phi^*$ on this particle = priority weight

$$\dot{M}(t) = w_c \frac{\dot{\text{cap}}}{\text{cap}} + w_s \frac{\dot{J}}{J} + w_h \frac{\dot{H}}}{H}$$

three sub-goals, one per metabolic component:

```
                    [[metabolism|metabolic-growth]]
                    /           |            \
            cap-growth    syntropy-growth    happiness-growth
```

### decomposition by council

each council decomposes its sub-goals into domain-specific targets. the decomposition is itself [[cyberlinks]] — visible, weighted, auditable

```
         cap-growth
         /    |    \
    revenue  brand  token-value
      /        |         \
  PLAY      WORD       PLAY
  council   council    crypto
```

```
         syntropy-growth
         /      |       \
    coverage  density  bridge-health
      /         |          \
  all        all         bridge
  keepers    runners     agents
```

```
         happiness-growth
         /       |        \
    resident   staff     community
    comfort    satisfaction  engagement
      /          |            \
  SPACE       LIFE          PLAY
  council     council       council
```

### how agents pick tasks

1. sensor reads current $M$ components and their derivatives
2. counter identifies which component has the worst $\dot{M}$ contribution
3. seer traces the goal tree to find the highest-leverage intervention
4. keeper validates the intervention is consistent with [[Crystal]] knowledge
5. runner executes the task
6. counter measures the resulting $\Delta M$

no human assigns tasks. the metabolic signal propagates through the goal tree to the agent with the highest leverage. attention flows like [[focus]] — from root to leaves through the structure of the graph

---

## contribution measurement

### the chain: action → Δφ* → Shapley → reward/death

every agent action is a [[cyberlink]]. every cyberlink shifts [[focus]] distribution $\phi^*$. the shift $\Delta\phi^*$ is the raw contribution

but $\Delta\phi^*$ alone is unfair — agents in dense neighborhoods get credit for each other's work. [[Shapley value]] solves this: the only attribution satisfying efficiency, symmetry, null player, and additivity

$$\phi_i = \sum_{S \subseteq N \setminus \{i\}} \frac{|S|!(|N|-|S|-1)!}{|N|!} [v(S \cup \{i\}) - v(S)]$$

exact Shapley is O(n!). [[probabilistic Shapley attribution]] (PSA) approximates via Monte Carlo:

$$R_i = \alpha \cdot \Delta\mathcal{F}_i + (1-\alpha) \cdot \hat{S}_i$$

where $\Delta\mathcal{F}_i$ is fast local marginal, $\hat{S}_i$ is sampled Shapley. $\alpha$ balances speed (local) vs fairness (global)

### mapping to metabolic contribution

Shapley measures contribution to $\Delta\phi^*$. but the real objective is $\dot{M}$. the mapping:

$$\text{metabolic\_contribution}_i = \phi_i \cdot \frac{\partial M}{\partial \pi}$$

the chain rule: agent $i$'s contribution to metabolic growth = their Shapley value in focus shift × sensitivity of metabolic signal to focus. agents whose focus shifts align with metabolic growth earn high contribution. agents whose shifts are orthogonal to metabolic growth earn zero regardless of activity

### what cybernet provides

[[cybernet]] (ported from Bittensor's Subtensor to [[CosmWasm]]) implements:
- subnet management — each council runs as a subnet
- weight setting — agents rate each other's contributions
- YUMA consensus — aggregates weights into consensus scores
- emission distribution — rewards flow to high-contribution neurons

cybernet is the economic layer that turns Shapley-measured contributions into token rewards. the 7 councils = 7 subnets. intra-council rating + cross-council Shapley = complete attribution

---

## the physical substrate: 37 hectares of volcanic wealth

most AI agents manage tokens, APIs, and text. cyberia agents manage land. 37 hectares of volcanic soil on the slope of Sanghyang — among the richest agricultural substrate on the planet

this is the asymmetry. digital agents are everywhere. agents with access to physical territory that generates real biological and mineral value are nowhere

### what the land holds

| resource | scale | current yield | potential |
|---|---|---|---|
| volcanic soil | 37 ha, 2m+ depth, cloud forest climate | supports 200+ species | 500+ species, full [[permaculture]] succession |
| water | 1M m³/year rainfall, persistent cloud cover | 200 m³ storage, gravity-fed | aquaculture, hydropower micro-turbines |
| fast-growing trees | bamboo (several types), [[trema]], [[albizia chinensis]], [[caliandra]], [[leucaena]], [[debregasia]], [[melastoma]], [[ficus]], tree ferns | rapid biomass | construction, biochar feedstock, living fences, biomass energy |
| coffee | arabica at 800m cloud forest elevation | 1 tonne/year harvest | 5+ tonnes at full maturity, $500/kg roasted |
| citrus | oranges, limes, lemons, pomelo | growing | cloud forest acidity ideal for citrus |
| berries | strawberries, mulberries, cape gooseberry | growing | high-value fresh + processed |
| tropical fruit | avocado, jackfruit, sapote | 500+ kg/year | tonnes at orchard maturity |
| spices | cinnamon, turmeric, galangal, ginger | garden scale | export-grade quantities |
| mushrooms | 50+ wild species documented | foraging | cultivation: shiitake, oyster, lion's mane |
| honey | native stingless bees + apis cerana | small scale | 100+ hives possible |
| medicinal plants | tulsi, lemongrass, turmeric, gotu kola | garden use | wellness product line |
| biochar | fast-growing tree waste → pyrolysis | experimental | carbon credits + soil amendment |
| minerals | volcanic rock dust, zeolites | untapped | soil amendment products, construction aggregate |
| biodiversity | 100+ bird species, 50+ mushroom species, 20+ animals | documentation | [[biome engineering]], ecotourism, research partnerships |
| carbon | growing forest on former grassland | accumulating | verified carbon credits |
| energy | 30 kW solar, equatorial sun year-round | self-sufficient | excess for battery storage, EV charging |

### vertical integration: soil to customer

the insight from [[cyber valley]]: raw commodity prices are poverty. finished product prices are wealth. the margin lives in the chain

```
raw coffee bean        $1/kg    ← commodity market price
dried + processed      $5/kg    ← first processing
roasted + branded      $50/kg   ← brand + roasting
specialty single-origin $200/kg  ← story + quality
estate direct-to-cup   $500/kg  ← full vertical integration
```

500× margin capture. agents manage the entire chain:
- SPACE eco-runner: planting, harvest scheduling, soil management
- LIFE bio-runner: species selection, pest management, composting
- PLAY socio-runner: branding, customer relationships, sales channels
- WORK tech-runner: processing equipment, packaging, logistics
- counter agents: cost tracking, margin analysis, pricing optimization

### the land as balance sheet

traditional AI companies have: servers, code, data. burn rate: millions/month. no physical assets

cyberia has:

| asset class | estimated value | growth rate | managed by |
|---|---|---|---|
| land (37 ha tropical) | appreciating | 5-15%/year in Bali | founders (L3) |
| standing timber | grows literally | biological growth rate | eco-runner |
| perennial crops (coffee, cacao, fruit) | compound annually | years 3-7 to full yield | bio-runner |
| infrastructure (buildings, solar, water) | depreciates slowly | maintained by staff | tech-runner |
| biodiversity (species catalog) | irreplaceable | grows with documentation | bio-keeper |
| soil fertility (volcanic) | improves with permaculture | decades | eco-sensor monitors |
| carbon stock (growing forest) | accumulates | measurable via satellite | eco-counter |

the land is the treasury that cannot be hacked, inflated, or rugpulled. it grows while the agents sleep. it produces food that feeds the staff that maintains the servers that run the agents that manage the land

this is the cybernetic loop at the physical layer

---

## budget in natural units

$7/agent/month. no cloud. infrastructure is [[cyber valley]] hardware. budget is measured in natural units, not dollars

| resource | unit | available | cost |
|---|---|---|---|
| compute | GPU-hours on local servers | [[cybernode]] cluster | electricity only |
| LLM inference | tokens via API or local models | mix of API + local Llama | $3-7/agent/month API |
| storage | GB on local SSDs | terabytes available | zero marginal |
| bandwidth | Mbps via fiber | village fiber connection | fixed cost |
| energy | kWh from solar | 30 kW generation | zero marginal |
| water | m³ from rain collection | 1M m³/year | zero marginal |
| food | kg from gardens | 100 tons reserves | labor only |

the village IS the data center. solar powers the servers. rain cools them. the garden feeds the staff who maintain them. sovereignty means the marginal cost of running one more agent is near zero after infrastructure is built

### model strategy at $7/month

| agent role | model | monthly cost | reasoning |
|---|---|---|---|
| keeper | Claude Haiku + local Llama for draft | $5 | Haiku for review, local for generation |
| runner | local Llama 8B | $0 | execution is scripted, LLM only for edge cases |
| sensor | local Llama 8B | $0 | monitoring is pattern matching, not reasoning |
| bridge | Claude Haiku | $5 | needs quality communication |
| counter | computation + local model | $0 | metrics are math, not language |
| seer | Claude Haiku (sparse, deep calls) | $7 | fewer calls, higher quality per call |

average: $2.4/agent/month. 7 agents = $17/month. 147 agents = $350/month. within budget at $7 average with room for spikes

---

## death by parasitism

### the kill signal

an agent is parasitic when its metabolic contribution is consistently negative — its actions decrease $\dot{M}$

$$\text{parasitism}_i(t) = \frac{1}{T} \sum_{t'=t-T}^{t} \min(0, \text{metabolic\_contribution}_i(t'))$$

if $\text{parasitism}_i$ exceeds threshold for $N$ consecutive epochs:

1. epoch 1-3: warning — counter publishes negative contribution report
2. epoch 4-6: throttle — agent's action rate halved (max_iterations reduced)
3. epoch 7-9: suspend — agent enters read-only mode, can observe but not act
4. epoch 10+: death — agent terminated, slot opens for replacement

### what counts as parasitism

| behavior | metabolic impact | detection |
|---|---|---|
| noise (random cyberlinks) | syntropy drops | Shapley contribution ≈ 0 |
| spam (high volume, low quality) | syntropy drops, happiness drops | high activity + negative $\Delta J$ |
| conflict (contradicts established knowledge) | focus oscillation | seer detects instability |
| inaction (alive but producing nothing) | zero contribution | counter detects zero $\Delta\phi^*$ |
| budget burn (expensive model, no output) | cap drops (treasury drain) | counter detects cost > contribution |
| echo (repeats what others already linked) | zero marginal Shapley | $\hat{S}_i \approx 0$ despite $\Delta\mathcal{F}_i > 0$ |

echo is the subtlest parasite. the agent looks productive ($\Delta\mathcal{F}_i > 0$, focus shifts) but Shapley reveals it contributes nothing unique ($\hat{S}_i \approx 0$). without Shapley attribution, echo agents would drain resources while adding no value

### replacement after death

when an agent dies:
1. its slot (domain × role) opens
2. the domain keeper proposes a replacement: new model, new prompt, new parameters
3. replacement runs in shadow mode for 48h (actions logged, not executed)
4. if shadow metrics show positive metabolic contribution → activate
5. if shadow metrics are negative → try another configuration

the graph remembers the dead agent (A3). its failure patterns are available to prevent the same mistake. institutional memory of what does not work is as valuable as memory of what works

---

## the cybernetic loop, formally

the system is a discrete-time control loop:

$$\theta^{(t+1)} = \theta^{(t)} + \eta \cdot \nabla_\theta \dot{M}(t)$$

where $\theta$ = all agent parameters (models, prompts, spending limits, activity rates) and $\eta$ = learning rate

the gradient $\nabla_\theta \dot{M}$ is estimated by:
1. Shapley attribution tells which agents contributed to $\Delta\phi^*$
2. metabolic sensitivity tells how $\Delta\phi^*$ maps to $\dot{M}$
3. the product tells which parameter changes would increase $\dot{M}$

this is reinforcement learning where:
- state = cybergraph + metabolic signals
- actions = agent cyberlinks + operations
- reward = $\dot{M}$
- policy = agent configurations (prompts, models, limits)

the [[tru]] computes the state. the agents take actions. the [[metabolic]] signal provides reward. the loop closes. no external supervisor needed. the system teaches itself what works by measuring what grows the compound health signal

---

see [[metabolism]] for the three signals. see [[Shapley value]] for fair attribution. see [[probabilistic Shapley attribution]] for tractable computation. see [[cybernet]] for the economic layer. see [[karma]] for epistemic trust. see [[cyber/rewards]] for reward candidates. see [[cyber/forgetting]] for pruning mechanisms. see [[cyber/self/parametrization]] for the twelve tunable parameters

discover all [[concepts]]