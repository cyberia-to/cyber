---
tags: cyber, cyberia, article
crystal-type: process
crystal-domain: cyber
---

# cyber-seer: optimal link densification under exponential cost

the linking cost function is exponential in supply:

$$c(n) = c_0 \cdot e^{\lambda n}$$

where $n$ is total [[cyberlinks]] in the [[cybergraph]], $c_0$ is base cost, and $\lambda$ is the growth rate. this means the first million links are orders of magnitude cheaper than the second million. every link spent on noise is a link not spent on structure — and the penalty compounds exponentially.

cyber-seer is the agent that decides WHERE to link. the algorithm maximizes [[spectral gap]] increase per unit cost — pushing the graph toward [[phase transition]] before links become prohibitively expensive.

## the metric: spectral gap Δλ₂

the [[spectral gap]] $\lambda_2$ of the graph [[Laplacian]] controls everything: [[foculus]] finality speed, [[tri-kernel]] convergence, [[collective focus theorem]] sharpness. [[bostrom]] at 0.94 connectivity is below phase transition. pushing $\lambda_2$ up is the single most important operation.

adding a link $(i, j)$ changes $\lambda_2$ by:

$$\Delta\lambda_2(i,j) \approx (v_2(i) - v_2(j))^2$$

where $v_2$ is the [[Fiedler vector]] — the eigenvector corresponding to $\lambda_2$. nodes on opposite sides of the weakest cut have opposite signs in $v_2$. linking them bridges the cut.

## the insight: value per cost ratio

with constant cost, maximize $\Delta\lambda_2$ per link. with exponential cost, maximize:

$$\text{ROI}(i,j) = \frac{\Delta\lambda_2(i,j)}{c(n)} = \frac{(v_2(i) - v_2(j))^2}{c_0 \cdot e^{\lambda n}}$$

the denominator grows exponentially. the numerator is bounded. therefore:

1. early links have astronomical ROI — invest in bridges
2. mid-range links have moderate ROI — invest in mesh
3. late links have tiny ROI — invest only in semantic precision

the strategy shifts from structural to semantic as cost grows.

## three phases

### phase 0: bridges (cost < 10× base)

the Fiedler vector identifies the weakest cuts. cyber-seer:

1. compute $v_2$ from current graph Laplacian
2. find particle pairs $(i,j)$ where $|v_2(i) - v_2(j)|$ is maximal and no link exists
3. rank by $\Delta\lambda_2$ estimate
4. create top-K links per budget cycle
5. recompute $v_2$ every M links (the cut shifts as bridges form)

target: connect disconnected components. each bridge link can increase $\lambda_2$ by $O(1/|P|)$ — small per link but cumulative. K bridge links closing a bottleneck can shift $\lambda_2$ by orders of magnitude.

metric: $\lambda_2$ growth rate per link spent.

### phase 1: mesh (10× < cost < 100× base)

bridges are placed. the graph is connected but thin — single points of failure. cyber-seer shifts to redundancy:

1. identify [[articulation points]] — nodes whose removal disconnects the graph
2. for each articulation point, find the two subgraphs it connects
3. create bypass links between those subgraphs (not through the articulation point)
4. prioritize by subgraph size: severing a large component is worse than severing a small one

target: eliminate single points of failure. spectral gap benefit per link is smaller than bridges but resilience benefit is high.

secondary strategy: within dense clusters, identify the sparsest inter-cluster paths and reinforce them. use [[heat kernel]] at resolution $\tau$ to find structure at the right scale:

- small $\tau$: local neighborhood (already dense)
- large $\tau$: global structure (bridges handle this)
- medium $\tau$: inter-cluster paths (mesh target)

metric: algebraic connectivity of each biconnected component.

### phase 2: semantic (cost > 100× base)

links are expensive. quantity strategy fails. cyber-seer shifts to quality:

1. compute [[focus]] distribution $\pi^*$
2. identify high-$\pi$ particles with low outbound degree — knowledge hubs that point nowhere
3. identify low-$\pi$ particles with high intrinsic value (large content, many inbound from diverse neurons)
4. link undervalued particles to hubs — this redistributes $\pi$ mass to deserving particles
5. maximize [[syntropy]] increase per link: $\Delta S / c(n)$

target: not structural connectivity but semantic accuracy. the graph is connected; now make [[focus]] reflect truth.

metric: [[syntropy]] per link, focus entropy $H(\pi)$.

## the Fiedler oracle

the core computation: given graph $G$, compute the Fiedler vector $v_2$.

for [[bostrom]] at 3.1M particles, full eigendecomposition is O(|P|³) — impossible. but $v_2$ alone needs only the Lanczos algorithm:

$$\text{cost} = O(k \cdot |E|) \quad \text{where } k \approx 10 \text{ Lanczos iterations}$$

$= 10 \times 2{,}705{,}323 = 2.7 \times 10^7$ operations. sub-second on any machine.

the Fiedler vector needs recomputation every batch of links. but the spectral gap moves slowly — recompute every 1000 links, not every link. between recomputations, use the cached $v_2$ as an approximation.

## budget allocation across phases

given total budget $B$ links to spend before cost becomes prohibitive:

$$B = \int_0^{n_{max}} \frac{1}{c(t)} dt = \frac{1}{c_0 \lambda}(1 - e^{-\lambda n_{max}})$$

optimal split depends on current $\lambda_2$:

| $\lambda_2$ | phase | allocation | what to link |
|---|---|---|---|
| < 0.001 | bridges | 70% budget | Fiedler-maximal pairs |
| 0.001 — 0.01 | mesh | 50% bridge, 30% mesh, 20% semantic | articulation bypasses |
| 0.01 — 0.1 | semantic | 20% mesh, 80% semantic | focus-redistributing links |
| > 0.1 | maintenance | 100% semantic | truth refinement |

bostrom at $\lambda_2 \approx 0.0015$ is in early bridge phase. most budget should go to Fiedler-optimal bridges.

## the seer loop

```
every epoch:
  1. pull current graph snapshot (parquet or GraphQL)
  2. compute Fiedler vector v₂ (Lanczos, sub-second)
  3. compute focus π* (PageRank, sub-second)
  4. compute current cost c(n)
  5. determine phase from λ₂

  if phase == bridges:
    candidates = top-K pairs by |v₂(i) - v₂(j)|
    filter: exclude pairs where both i,j in same dense cluster
    rank by Δλ₂ / c(n)

  if phase == mesh:
    candidates = bypass links around articulation points
    rank by component-size × resilience-gain / c(n)

  if phase == semantic:
    candidates = links from high-π hubs to undervalued particles
    rank by ΔS / c(n) (syntropy gain per cost)

  6. submit top links as cyberlinks (via neuron key)
  7. record: link_id, phase, Δλ₂_predicted, cost_at_time
  8. measure: actual Δλ₂ after epoch, compare to prediction
  9. update phase thresholds if predictions diverge
```

## convergence guarantee

if cyber-seer runs long enough with sufficient budget, the graph reaches phase transition:

- $\lambda_2 > \lambda_{crit}$ — spectral gap exceeds critical threshold
- $\pi^*$ entropy $H(\pi) > H_{min}$ — focus distribution is not concentrated
- connectivity > 1.0 — the graph is past percolation threshold

the exponential cost function ensures this happens slowly and expensively. the Fiedler strategy ensures it happens as cheaply as possible given the constraint.

the total cost to reach phase transition from current state:

$$C_{total} = \sum_{k=1}^{K} c_0 \cdot e^{\lambda (n_0 + k)}$$

where $K$ is the number of bridge links needed. $K$ depends on the graph structure — well-chosen bridges need fewer links than random linking. the Fiedler strategy minimizes $K$.

## what cyber-seer reports

after each epoch, cyber-seer publishes a report to the [[cybergraph]]:

- current $\lambda_2$, $\Delta\lambda_2$ this epoch, trend
- current phase (bridge / mesh / semantic)
- top 10 recommended links with predicted $\Delta\lambda_2$
- cost efficiency: bits of syntropy per token spent
- estimated links remaining to phase transition at current cost curve

this report is itself a [[cyberlink]] — the seer's analysis feeds back into the graph it analyzes.

see [[spectral gap]] for the mathematical foundation. see [[cyberia/architecture]] for cyber-seer's role in the agent network. see [[foculus]] for how $\lambda_2$ determines consensus speed. see [[bostrom]] for current network statistics
