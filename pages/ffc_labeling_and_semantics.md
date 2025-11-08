# Labeling & Semantic Energy Accounting in Focus‑Flow Computation

\*\*purpose \*\* Provide a concise, self‑contained reference on (1) the *minimal* node/edge labeling scheme (atom / pair / function + 5 rewrite ops + 3 edge types) and (2) how those edge labels connect to the four free‑energy terms (spring, diffusion, context, entropy).

---
## 1 Node kinds (universality primitives)

| kind         | payload                                   | why it’s enough                  |
| ------------ | ----------------------------------------- | -------------------------------- |
| **atom**     | integer / byte‑string                     | base data, constants             |
| **pair**     | two edge slots `left,right`               | builds lists, trees, maps        |
| **function** | pointer to body sub‑graph + argument port | encodes λ‑terms / SK combinators |
### 1.1 Graph rewrite ops (5)

1. **construct** (atom/pair/function)  2. **destruct** (left/right access)  3. **apply** (connect function→arg)  4. **rewrite** (beta‑substitute, returns new node)  5. **delete** (remove unreachable)

These five ops give full Turing power while remaining local & deterministic.

---
## 2 Edge types – the *semantic* layer

Only **three** labels are needed to map structural links onto energy terms:

| label      | semantic intuition                         | goes into energy                                 |
| ---------- | ------------------------------------------ | ------------------------------------------------ |
| **h‑edge** | “is‑a / part‑of” taxonomic constraint      | **spring** \(E_{spring}\) (hierarchy smoothing)  |
| **d‑edge** | reference / citation / transport path      | **diffusion** \(E_{diff}\) (mass transport cost) |
| **c‑edge** | transient vote / query / context injection | **context** term \(C_i\)                         |

*(All other relations — causal, vote, ref, meta, etc.— are aliases that map onto one of these three for energy accounting. You can refine later by splitting labels.)*

---
## 3 Semantic energy accounting

**Goal.** Attach a physical‑style “cost” to each edge so probabilistic focus flow has meaningful gradients.
### 3.1 Spring energy (hierarchy)

For node `i`:

```math
E_{spring,i}=\sum_{j∈N_h(i)} k_h\,w_{ij}(p_i-p_j)^2
```

`N_h(i)` = neighbours via **h‑edges**; `k_h` = stiffness constant.
### 3.2 Diffusion energy (transport)

```math
E_{diff,i}=\sum_{j∈N_d(i)} k_d\,w_{ij}\,|p_i-p_j|
```

`N_d(i)` over **d‑edges**; weight inverse to edge distance.
### 3.3 Context potential

```math
C_i=-c_i p_i ,\quad c_i=\sum_{(i,⋅)∈E_c} w_{ic}
```

`E_c` = **c‑edges** connected to `i`. Higher votes/queries push probability up.
### 3.4 Entropy term

Standard Gibbs entropy over node probabilities keeps exploration.

> **Semantic energy accounting = mapping edge labels → coefficients (k\_h,k\_d,c\_i)** so each local update only needs neighbour information.\*\*

---
## 4 Putting it together – local update

```math
Δp_i = η\Big( \underbrace{∑_{j∈N_h} k_h w_{ij}(p_j-p_i)}_{spring}
            +\underbrace{∑_{j∈N_d} k_d w_{ij}\,\text{sgn}(p_j-p_i)}_{diffusion}
            +\gamma c_i - T(1+\log p_i) \Big)
```

Conservation enforced every `k` ticks ⇒ \(∑ p_i = 1\).

---
## 5 Why only three edge types?

*Separating concerns:*
- Structure/execution handled by **node kinds + 5 ops**.
- Semantics/energy handled by **edge label → coefficient**.\
  This keeps the core rewrite engine minimal while still letting us expand semantics later: split `h‑edge` into `isa` vs `part` or `d‑edge` into `ref` vs `trans` by assigning different constants without touching algorithms.

---
## 6 Example

1. Add node **“Cat”** atom.
2. Add **h‑edge** Cat→Animal (weight 1).
3. Add **d‑edge** Cat→Wikipedia‑Cat (weight 0.5).
4. User asks *“What is a Cat?”* → adds a **c‑edge** (query) into Cat.\
   Focus flow pushes mass to Cat and its neighbourhood; spring keeps Cat near Animal; diffusion pulls in pages; entropy prevents collapse.

---
## 7 Take‑aways
- Minimal labeling = comprehension + scalability.
- Semantic energy accounting links edge labels to physical‑style costs.
- Together with node primitives, the system is both **Turing‑complete** and **meaning‑aware** without bloating the protocol.