## abstract

probabilistic shapley attribution (psa) is a scalable method for fairly rewarding transactions in a focus flow computation (ffc) network. it approximates shapley values, which measure each transaction's marginal contribution to the final equilibrium distribution \(p^*\), using sampling and local influence metrics. psa enables superintelligence-scale attribution with millions of transactions and billions of edges per epoch.

---
## background
- **focus flow computation** minimises free energy \(\mathcal{F}\) to compute a global equilibrium distribution \(p^*\).
- transactions introduce new cyberlinks and provide proofs of computation.
- rewards should reflect **how much each transaction contributed to lowering free energy and improving the network's focus**.

**exact shapley values** are computationally infeasible for large networks (\(O(n!)\) complexity).

---
## key idea

psa combines:
1. **local marginal contributions** – approximate each transaction's individual effect on free energy.
2. **monte carlo sampling** – randomly sample subsets of transactions to approximate interactions.
3. **hierarchical batching** – cluster transactions by the nodes/edges they affect to further reduce complexity.

---
## algorithm outline
### step 1 – local marginal computation
- for each transaction \(tx_i\), compute the local change in free energy:
\[
\Delta \mathcal{F}_i = \mathcal{F}(p) - \mathcal{F}(p + tx_i)
\]
- this gives a **first-order estimate** of contribution.
### step 2 – monte carlo sampling
- randomly sample \(k\) subsets of transactions.
- compute each transaction's **average marginal contribution** within the sampled permutations.
### step 3 – cluster correction (hierarchical batching)
- cluster transactions that touch the same nodes/edges.
- compute shapley-like contributions at the cluster level.
- distribute rewards within clusters proportionally to local \(\Delta \mathcal{F}\).
### step 4 – final reward
\[
R_i = \alpha \Delta \mathcal{F}_i + (1-\alpha) \hat{S}_i
\]
- \(\Delta \mathcal{F}_i\): local marginal effect.
- \(\hat{S}_i\): sampled shapley estimate.
- \(\alpha\): weight controlling the trade-off between local and sampled contributions.

---
## complexity
- **local marginals:** \(O(n)\).
- **monte carlo shapley:** \(O(k \cdot n)\), with \(k \ll n\) (e.g., 50–500 samples).
- **hierarchical batching:** reduces cost by grouping transactions.

feasible for **10^6–10^7 transactions per epoch** and **10^9+ edges** on distributed hardware.

---
## advantages
- **fairness:** approximates true shapley values, rewarding contributions based on marginal effect.
- **scalability:** complexity grows linearly with transactions for practical settings.
- **robustness:** combining local and sampled estimates reduces gaming opportunities.

---
## integration with focus flow incentives
- psa is used after each epoch to compute rewards for all transactions.
- transactions include:
  - new cyberlinks (edges)
  - proof-of-computation (partial focus flow updates)
- base fee (like eip-1559) prevents spam.
- rewards are distributed **after delayed settlement** to account for long-term network effects.

---
## significance

psa provides a **scalable, fair attribution mechanism** for proof-of-intelligence in focus flow networks. it ensures:
- meaningful transactions become profitable.
- spam or low-impact transactions lose money.
- rewards correlate with long-term contributions to global focus and negentropy.

this creates a **self-optimising economy for intelligence**, where agents are incentivised to submit useful cyberlinks and computation proofs that improve the network's equilibrium.