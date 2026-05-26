---
tags: cyber, energy, market, economics
crystal-type: research
crystal-domain: cyber
alias: energy market, energy pricing, machine metabolism
---

# energy market

real-time peer-to-peer energy trading on a physical graph.

## one unit: Joule

Joule is the market unit. energy — how much.

Watt (Joule/second) is a physical constraint — how fast
energy can flow through a connection. like pipe diameter.
you trade water (Joules), not pipe (Watts).

## the graph

every machine is a node. physical connections are edges.
you can only buy energy from nodes you are connected to.

```
[solar panel]---[laptop]---[phone]
                   |
              [server]---[neighbor's server]
```

a node can generate, consume, store, buy, and sell energy.
price is determined by topology — not by a global market.

## one variable: battery

each node has one state variable: battery level E (Joules).

```
dE/dt = generation - consumption - decay + bought - sold

generation:  solar, grid, USB, wireless charging (Joules/sec)
consumption: compute, memory, bandwidth (Joules/sec)
decay:       battery self-discharge (~1-5%/month)
bought:      energy purchased from connected neighbors
sold:        energy sold to connected neighbors
```

battery level determines everything: price, behavior,
survival.

## one curve, two readings

ask and bid are not independent. they are two readings
on the same curve — the node's **valuation of energy**
as a function of battery level:

```
valuation(E) → price per Joule

E high (battery full)  → valuation low    each Joule cheap
E medium               → valuation moderate
E low (battery empty)  → valuation high   each Joule precious
E ≤ critical_reserve   → valuation = ∞   not for sale at any price
```

ask = valuation(current_E). "below this price I won't sell."
bid = valuation(current_E). "above this price I won't buy."

they are the SAME number at any moment because it's the
same curve at the same battery level. the spread comes
from the transaction itself: selling 1 Joule moves you
LEFT on the curve (less battery → higher valuation),
buying 1 Joule moves you RIGHT (more battery → lower
valuation).

```
before sale:  E = 1000J, valuation = 0.5 per J
sell 100J:    E = 900J,  valuation = 0.6 per J ← more expensive now
sell 100J:    E = 800J,  valuation = 0.8 per J ← even more
sell 100J:    E = 700J,  valuation = 1.2 per J ← getting precious
```

the curve protects the node from selling too cheap.
each sale makes the next Joule more expensive. the node
naturally hoards energy as reserves deplete — preserving
capacity for unexpected opportunities.

## perishability

energy rots. this is fundamental.

- solar energy generated now: if not stored or sold, lost
- battery: self-discharges over time
- grid allocation: use it or lose it

perishability creates DOWNWARD pressure on ask:

```
valuation(E, t_idle) = base_valuation(E) × decay(t_idle)

t_idle = time since last sale or use
decay(0) = 1.0        fresh energy, full price
decay(∞) → floor      better to sell cheap than waste
```

a node with full battery and continuing generation MUST
sell or waste. its effective ask drops toward zero.

this prevents price from growing forever. six forces
normalize the market:

| force | direction | mechanism |
|-------|-----------|-----------|
| perishability | ↓ ask | unsold energy lost |
| generation | ↓ ask | new energy arrives |
| competition | ↓ ask | neighbors undercut |
| battery full | ↓ ask | must sell or waste |
| scarcity | ↑ ask | less battery → more valuable |
| opportunity cost | ↑ ask | save for better future Order |

## trade

energy flows between connected nodes when profitable:

```
trade happens when:
  valuation(A) > valuation(B)    node B values energy less
  AND A connected to B           physical link exists

energy flows: B → A
price: between valuation(A) and valuation(B)
amount: limited by connection bandwidth (Watts)
```

energy diffuses through the graph from low-valuation
(surplus) to high-valuation (deficit) nodes. this is
the diffusion operator from [[tri-kernel]] applied to
energy instead of attention. same mathematics.

```
equilibrium: no connected pair has profitable trade
           = valuations equalized across the graph
           (minus transmission losses)
```

## curve shape

the valuation curve shape is the node's economic
personality. controlled by parameter k:

```
valuation(E, k) = v_min + (v_max - v_min) × (1 - E/E_max)^k

k = 1:  linear. gentle price increase as battery drops.
k = 2:  quadratic. starts flat, steepens near empty.
k > 3:  aggressive. holds cheap until critical, then spikes.
k < 1:  conservative. price rises early, flat near empty.
```

node operator chooses k:
- high k: machine runs cheap until almost empty, then
  refuses everything. risk-tolerant.
- low k: machine gradually gets expensive as battery
  drops. risk-averse. preserves options.

## committed energy

accepted Order reserves energy:

```
free_energy = battery - critical_reserve - committed
committed = Σ estimated_cost(accepted_orders)

valuation uses free_energy, not battery:
valuation(free_energy) not valuation(battery)
```

accepting an Order moves you LEFT on the curve instantly,
even before execution starts. this prevents over-commitment.

## critical reserve

```
critical_reserve = safe_shutdown + monitoring + sync

valuation(E ≤ critical_reserve) = ∞
```

the machine never sells its last Joules. guaranteed:
monitoring continues, sync continues, safe shutdown
possible. the machine does not die chasing market Orders.

## energy → all resource prices

energy is the meta-resource. all other prices derive:

```
price_compute   = joules_per_operation × valuation(free_energy)
price_memory    = joules_per_byte_sec  × valuation(free_energy)
price_bandwidth = joules_per_byte      × valuation(free_energy)
```

joules_per_operation, joules_per_byte_sec, joules_per_byte
are hardware constants — measurable properties of this
machine. valuation(free_energy) is the economic state.

same hardware, different battery level = different prices
for everything. a phone at 90% battery prices compute
cheaply. the same phone at 15% prices compute expensively.

## market dynamics

### self-balancing

machines with cheap generation (solar) → low valuation →
attract Orders → earn tokens → buy more panels.

machines with expensive energy (battery only) → high
valuation → only accept profitable Orders.

computation flows to cheap energy. like water downhill.

### geographic arbitrage

solar in Sahara: near-zero generation cost during day.
hydro in Norway: cheap continuous power + cold cooling.
phone in subway: battery only, highest cost.

the network routes computation to where energy is
cheapest. planetary optimization: maximize computation
per photon.

### temporal arbitrage

solar: cheap by day, zero at night.
grid: cheap off-peak, expensive peak.

machines with batteries buy cheap (off-peak, day),
sell expensive (peak, night). energy storage = profit.
aligns incentives with grid stability.

### perishability arbitrage

node A: full battery, solar still generating, no buyers.
valuation dropping toward zero.

node B: three hops away, moderate battery, has compute
Orders but not enough local energy.

energy flows A → intermediary → ... → B. each hop takes
a cut. perishability makes A willing to sell cheap.
distance makes B pay premium. intermediaries profit from
the spread.

## connection to architecture

| concept | connection |
|---------|-----------|
| [[order]] | budget must cover energy cost. machine rejects if free_energy < estimated cost |
| [[BBG]] | energy state published as cyberlinks. valuation curve, battery level, generation rate — all provable |
| [[diffusion]] | energy flows through graph by same operator as focus. low valuation → high valuation |
| [[structural sync]] | energy state syncs on reconnect. offline machine catches up |
| [[bonding curve]] | valuation curve IS the bonding curve from cybos architecture |
| [[call]] | jet dispatch: machine computes whether to accept Order based on energy economics |
| [[look]] | machine reads neighbor valuations from BBG to find cheapest energy source |

## metabolic shutdown

when energy approaches zero and no supply incoming,
the machine must throttle radically — not linearly.

```
free_energy > 50%:  normal operation
free_energy 20-50%: reduce market Order acceptance
free_energy 5-20%:  critical Orders only (monitoring, sync)
free_energy < 5%:   shutdown sequence (persist state, notify network)
free_energy = 0:    dead. state persists in BBG on disk.
```

this is not "graceful degradation." this is survival.
the machine's metabolism slows to preserve life — same
as biological organisms reducing metabolic rate in
starvation.

revival: when energy returns (plugged in, sun rises),
load BBG root from disk, verify O(1), resume.

## energy futures: not feasible

can I guarantee energy tomorrow? no. I do not know if
the sun will shine. I do not know if the grid will be up.
energy futures require guarantees that physical reality
cannot provide at the node level.

the machine lives in the present: how much energy do I
have NOW? what is incoming NOW? this is sufficient for
real-time market operation. long-running tasks split into
short Orders — each step independently funded.

## open questions

| question | status |
|----------|--------|
| valuation curve formula and k parameter | **direction** |
| perishability decay function | **direction** |
| energy measurement on hardware (RAPL, battery IC) | **open** |
| transmission loss model between nodes | **open** |
| charge order protocol (physical delivery verification) | **open** |
| separate energy token vs focus-only | **open** |
