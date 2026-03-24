---
tags: cyber, research, article
crystal-type: process
crystal-domain: cyber
status: draft
date: 2026-03-24
---
# knowledge capacity: information-theoretic limits of the [[cybergraph]]

## abstract

the [[cybergraph]] cannot capture all of reality. this is not a design limitation — it is an information-theoretic bound. three independent constraints — bandwidth, economics, and decay — each impose a ceiling on how much knowledge the graph can sustain. the tightest constraint determines the actual limit. we derive the bound, show it is analogous to Shannon capacity and Boltzmann equilibrium, and identify the parameters that determine the [[knowledge completeness]] of a collective intelligence.

## the question

every [[cyberlink]] adds information to the graph. [[temporal decay]] removes information. at what point does the graph reach maximum capacity — where new links can only replace decaying ones, and net knowledge growth stops?

## three bounds

### bound 1: bandwidth

[[VDF]] rate-limits signal production. each [[neurons|neuron]] can produce at most one [[signal]] per $T_{\min}$ wall-clock seconds. the VDF is inherently sequential — this cannot be parallelised.

total information input rate:

$$R_{\text{input}} = \frac{N_{\text{neurons}}}{T_{\min}} \times b_{\text{signal}}$$

where $b_{\text{signal}}$ is the average information content per signal (cyberlinks + particle content). at $T_{\min} = 1\text{s}$, $N = 10^6$ neurons, $b = 10^4$ bits per signal:

$$R_{\text{input}} = 10^6 \times 10^4 = 10^{10} \text{ bits/s} = 1.25 \text{ GB/s}$$

the graph can grow at most 1.25 GB/s of new information. this is the hard physical limit from VDF sequential computation.

### bound 2: economics

the cost of the $n$-th [[cyberlink]] grows exponentially with total link count:

$$c(n) = c_0 \cdot e^{\lambda n}$$

[[focus]] regenerates at a finite rate $R_{\text{focus}}$ across all neurons. the maximum number of links the network can AFFORD:

$$R_{\text{focus}} = \int_0^{n_{\max}} c(t) \, dt = \frac{c_0}{\lambda} \left(e^{\lambda n_{\max}} - 1\right)$$

solving for $n_{\max}$:

$$n_{\max} = \frac{1}{\lambda} \ln\left(\frac{\lambda R_{\text{focus}}}{c_0} + 1\right)$$

this is logarithmic in focus supply. doubling the total focus budget increases maximum links by $\frac{\ln 2}{\lambda}$ — a CONSTANT, not a proportional increase. the exponential cost makes the economic ceiling hard.

at the ceiling, ALL regenerated focus goes to paying for the marginal link. zero budget remains for maintaining existing links. in practice, the sustainable limit is lower — the network must reserve focus for maintenance.

### bound 3: decay equilibrium

every link decays exponentially:

$$w(t) = w_0 \cdot \alpha^{t - t_{\text{last}}}$$

a link is pruned when $w(t) < \epsilon$. to prevent pruning, a neuron must periodically reinforce the link (spend focus again). the maintenance cost per link per epoch:

$$f_{\text{maintain}} = c(n) \times p_{\text{reinforce}}$$

where $p_{\text{reinforce}}$ is the fraction of links needing reinforcement per epoch.

the decay equilibrium: the graph reaches maximum size when focus regeneration exactly covers maintenance of existing links:

$$R_{\text{focus}} = n_{\text{eq}} \times f_{\text{maintain}}$$

$$n_{\text{eq}} = \frac{R_{\text{focus}}}{c(n_{\text{eq}}) \times p_{\text{reinforce}}}$$

this is a fixed-point equation. the solution depends on the cost function:

for exponential cost:

$$n_{\text{eq}} = \frac{R_{\text{focus}}}{c_0 \cdot e^{\lambda n_{\text{eq}}} \cdot p_{\text{reinforce}}}$$

$$n_{\text{eq}} \cdot e^{\lambda n_{\text{eq}}} = \frac{R_{\text{focus}}}{c_0 \cdot p_{\text{reinforce}}}$$

this is the Lambert W function:

$$n_{\text{eq}} = \frac{1}{\lambda} W\left(\frac{\lambda R_{\text{focus}}}{c_0 \cdot p_{\text{reinforce}}}\right)$$

the Lambert W function grows as $\ln(x) - \ln(\ln(x))$ — very slowly. the decay equilibrium is LOGARITHMIC in total focus supply. enormous increases in collective resources yield modest increases in sustainable knowledge.

## the combined limit

the three bounds are independent. the tightest determines the actual capacity:

$$I_{\text{capacity}} = \min\left(I_{\text{bandwidth}},\ I_{\text{economic}},\ I_{\text{decay}}\right)$$

in most regimes, the economic bound dominates:

```
bandwidth:   10^10 bits/s × lifetime → very large (petabytes over years)
economic:    n_max = (1/λ) × ln(λR/c₀ + 1) → logarithmic in resources
decay:       n_eq = (1/λ) × W(λR/(c₀p)) → logarithmic in resources

economic ≤ decay ≤ bandwidth (typical ordering)
```

the exponential cost function is the fundamental bottleneck. not bandwidth. not decay. the COST OF ATTENTION is what limits knowledge.

## knowledge completeness

define [[knowledge completeness]] as the ratio of captured to capturable knowledge:

$$\kappa = \frac{I_{\text{graph}}}{I_{\text{reality}}}$$

where $I_{\text{reality}}$ is the total information content of "observable reality" at the chosen resolution.

$\kappa$ is bounded by:

$$\kappa_{\max} = \frac{I_{\text{capacity}}}{I_{\text{reality}}}$$

for this to approach 1, you need $I_{\text{capacity}} \geq I_{\text{reality}}$. given the logarithmic dependence on resources, this requires:

$$R_{\text{focus}} \geq \frac{c_0}{\lambda} \cdot e^{\lambda I_{\text{reality}}} - \frac{c_0}{\lambda}$$

focus must grow EXPONENTIALLY with the amount of reality to capture. this is the information-theoretic impossibility: finite collective focus cannot capture infinite (or even very large finite) reality.

## the distribution of completeness

$\kappa$ is not uniform across domains. the [[universal law|exponential optimality under constraint]] predicts: given finite focus, attention distributes exponentially across ranked domains:

$$\kappa_k \propto e^{-\beta k}$$

where $k$ ranks domains by collective interest. the cybergraph is:
- ~95% complete for the most-attended domains (mathematics, core protocols)
- ~50% complete for moderately-attended domains (popular science, culture)
- ~1% complete for long-tail domains (obscure specialties)
- ~0% complete for unattended domains (unknown unknowns)

the distribution follows the same exponential as [[focus]], [[pi-weighted-replication|replication]], [[gravity commitment|verification cost]], and [[temporal decay]]. the entire stack — from proof cost to storage to completeness — follows one distribution.

## analogy stack

| domain | finite resource | limit | grows as |
|---|---|---|---|
| thermodynamics | temperature $T$ | Boltzmann: $p_i \propto e^{-E_i/kT}$ | exponential in energy |
| information theory | channel capacity $C$ | Shannon: $R \leq C$ | logarithmic in SNR |
| computation | program length $L$ | Kolmogorov: most strings incompressible | logarithmic in strings |
| **cybergraph** | **collective focus $R$** | **$n_{\max} \sim \frac{1}{\lambda}\ln R$** | **logarithmic in focus** |

every row says the same thing: finite resources cannot capture infinite structure. the capacity grows logarithmically with resources — diminishing returns are fundamental, not accidental.

the Boltzmann analogy is exact:
- microstates ↔ possible cyberlinks
- energy ↔ cost (exponential in supply)
- temperature ↔ collective focus budget
- partition function ↔ total possible graph configurations
- equilibrium distribution ↔ $\pi^*$ (focus distribution)

the cybergraph at capacity IS a thermal system. the "temperature" is the ratio of collective focus to link cost. high temperature (abundant focus relative to cost) → many links, high completeness, high entropy. low temperature (scarce focus) → few links, sparse graph, low entropy.

## the phase transition

at low $\kappa$, the graph is below [[phase transition]] — disconnected, no meaningful $\pi^*$, no [[foculus]] convergence. at critical $\kappa_c$, the graph crosses the percolation threshold:

$$\lambda_2 > \lambda_{\text{crit}} \implies \kappa > \kappa_c$$

above $\kappa_c$, the tri-kernel produces meaningful $\pi^*$, foculus converges, and the graph becomes self-sustaining — useful queries attract neurons, neurons create links, links improve $\pi^*$, better $\pi^*$ attracts more queries.

below $\kappa_c$, the graph is in cold start — no self-sustaining loop. this is where [[cyber/seer|cyber-seer]]'s bridge strategy matters most: every link optimised for $\Delta\lambda_2$ pushes the graph toward phase transition with minimum focus expenditure.

the [[spectral gap from convergence|spectral gap]] determines the sharpness of the transition. for graphs with power-law degree distribution (like the cybergraph), the transition is SHARP — a small increase in $\kappa$ near $\kappa_c$ produces a large jump in $\lambda_2$.

## parameters that determine capacity

| parameter | symbol | effect on $n_{\max}$ | who controls it |
|---|---|---|---|
| focus regeneration rate | $R_{\text{focus}}$ | logarithmic increase | protocol economics (staking, inflation) |
| base link cost | $c_0$ | linear decrease | protocol parameter |
| cost growth rate | $\lambda$ | inverse — most sensitive | protocol parameter (the key knob) |
| decay rate | $\alpha$ | slower decay → more sustainable links | protocol parameter |
| maintenance fraction | $p_{\text{reinforce}}$ | lower → more capacity | emergent (depends on link quality) |
| VDF delay | $T_{\min}$ | inverse bandwidth | protocol parameter |
| neurons | $N$ | linear bandwidth, logarithmic economic | adoption |
| signal size | $b_{\text{signal}}$ | linear bandwidth | protocol parameter |

the most sensitive parameter is $\lambda$ — the exponential cost growth rate. small changes in $\lambda$ produce large changes in capacity because $n_{\max} \propto 1/\lambda$.

## implications

### 1. knowledge is thermodynamic

the cybergraph at equilibrium IS a thermal system. the "heat bath" is collective focus. the "energy landscape" is the cost function. the "equilibrium distribution" is $\pi^*$. the "temperature" is focus/cost ratio.

statistical mechanics applies. the fluctuation-dissipation theorem predicts: regions of the graph with high focus variance (active debate) will have high link turnover (dissipation). regions with stable $\pi^*$ will have low turnover.

### 2. completeness is a choice, not a bug

the logarithmic capacity bound means: doubling collective resources does NOT double knowledge. it adds a constant. the network must CHOOSE what to know — and the [[focus]] mechanism is the choice function.

this is the same tradeoff every intelligent system faces. a brain with 10^11 neurons doesn't know everything. a library with 10^8 books doesn't contain all knowledge. the constraint is not storage — it is ATTENTION.

### 3. the long tail is unreachable

exponential focus distribution means: the most important 1% of domains get 50% of attention. the bottom 50% of domains get ~1% of attention. increasing total resources doesn't change the SHAPE — it shifts the curve, adding marginal coverage to already-well-covered domains.

to cover the long tail, the network needs not more focus but BETTER ALLOCATION — neurons that specialise in underserved domains. this is the economic opportunity: scarce knowledge has low competition for focus. a neuron that covers an empty domain earns outsized $\pi^*$ per focus spent.

### 4. $\lambda$ is the key policy lever

the cost growth rate $\lambda$ determines whether the graph can sustain 10^6 or 10^12 links. lowering $\lambda$ (slower cost growth) dramatically increases capacity but reduces the evolutionary pressure that keeps quality high.

the tradeoff: low $\lambda$ → large graph, more noise. high $\lambda$ → small graph, high signal. the optimal $\lambda$ maximises [[syntropy]] (information per link), not total links.

this connects to [[cyber/seer|cyber-seer]]'s strategy: in a high-$\lambda$ regime, every link must be spectral-gap-optimal. in a low-$\lambda$ regime, more exploratory linking is affordable. $\lambda$ determines the graph's "personality" — precise vs exploratory.

## the formula

the knowledge capacity of the cybergraph:

$$\boxed{K = \frac{1}{\lambda} \cdot W\!\left(\frac{\lambda \cdot N \cdot s \cdot T}{c_0 \cdot p \cdot T_{\min}}\right)}$$

where:
- $K$ = maximum sustainable cyberlinks
- $\lambda$ = cost growth rate (the key parameter)
- $N$ = number of neurons
- $s$ = stake per neuron (focus regeneration source)
- $T$ = time horizon (epochs)
- $c_0$ = base link cost
- $p$ = maintenance probability per epoch
- $T_{\min}$ = VDF delay (bandwidth constraint)
- $W$ = Lambert W function ($W(x) \sim \ln x$ for large $x$)

the capacity is logarithmic in everything except $\lambda$, where it is inversely proportional.

## open questions

1. **empirical $\lambda$.** what is the right cost growth rate? too high: graph can't grow. too low: graph fills with noise. optimal $\lambda$ maximises syntropy per focus — this may have an analytical solution

2. **adaptive $\lambda$.** should $\lambda$ change over the graph's lifetime? low $\lambda$ during cold start (encourage growth), increasing $\lambda$ as the graph matures (encourage precision). connection to [[cyber/seer|cyber-seer]]'s three phases

3. **knowledge resolution.** $I_{\text{reality}}$ depends on resolution — how fine-grained are the "facts" we want to capture? at coarse resolution (Wikipedia-level), the cybergraph may approach $\kappa \sim 0.5$ with 10^9 links. at fine resolution (every scientific measurement), $\kappa$ is negligible regardless of resources

4. **multi-graph capacity.** multiple independent cybergraphs (different communities, different focus distributions) may collectively cover more than one graph — if their focus distributions don't overlap. total coverage = union of individual coverages. connection to [[structural-sync]] composability

5. **can decay be selective?** uniform decay rate $\alpha$ is wasteful — important links decay at the same rate as noise. [[pi-weighted-replication|π-weighted]] decay (low-$\pi$ links decay faster) would increase effective capacity. does this violate any conservation law?

see [[knowledge completeness]] for the qualitative concept, [[universal law]] for the exponential distribution, [[collective focus theorem]] for the attention allocation, [[cyber/seer]] for optimal link placement, [[spectral gap from convergence]] for the phase transition, [[link production]] for the intelligence problem, [[temporal decay]] for the pruning mechanism
