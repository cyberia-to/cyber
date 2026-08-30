---
tags: cyber, research, article
crystal-type: article
crystal-domain: cyber
date: 2026-08-30
alias: physics of the cybergraph, physical analogies
---

# physical analogies: auditing a superintelligence design against experimental physics

suppose the universe is informational at bottom and physics is what that information looks like from inside. then physics is not a metaphor for a knowledge protocol — it is the completed experiment, already run, at the only scale where the answers are settled. every quantity [[cyber]] mints, ranks, or proves has a counterpart that some laboratory has already measured, and the correspondence is either exact, wrong, or interestingly incomplete.

this article does three things with that correspondence. it shows where the [[cybergraph]] design is not a choice but a theorem — the parts physics endorses. it audits the parts physics falsifies or leaves unpaid. and it turns the direction of borrowing around: a mechanism built to divide value fairly among strategic agents solves a problem stochastic thermodynamics has open, and that is worth stating as a prediction rather than a pun.

the standard throughout is experimental, not theoretical. holography, emergent gravity and the informational-universe thesis itself are excluded from the evidence — they are the frame, not the support. what carries weight below is measured: Bell violations, Landauer erasure, fluctuation theorems, light-cone spreading, cortical avalanches, critical exponents.

---

## 1. the dictionary

the specs were not written from physics. the correspondence is what is left after translating them.

| cyber quantity | as specified | physical counterpart |
|---|---|---|
| effective adjacency $A^{\text{eff}}$ | $\sum_\ell \text{stake}\cdot\kappa\cdot f(\text{price})$ | lattice couplings |
| springs | $(L+\mu I)x^*=\mu x_0$ | Klein–Gordon propagator, mass $m=\sqrt\mu$, Yukawa tail $e^{-md}$ |
| heat | $H_\tau=e^{-\tau L}$, Chebyshev | euclidean propagator; [[renormalization group]] coarse-graining at scale $\tau$ |
| diffusion | $\alpha P^\top\phi+(1-\alpha)u$ | classical master equation with resetting; $\alpha$ as decoherence rate |
| [[focus]] $\phi^*$ | $\propto e^{-\beta E}$ | Boltzmann equilibrium — *not* $\lvert\psi\rvert^2$ |
| [[syntropy]] | $J = D_{\mathrm{KL}}(\phi^*\Vert u)$ | Shannon negentropy $\log\lvert V\rvert - H$ |
| contraction $\kappa$ | $\lambda_d\alpha+\lambda_s\frac{\Vert L\Vert}{\Vert L\Vert+\mu}+\lambda_h e^{-\tau\lambda_2}$ | spectral gap; correlation length $\xi = 1/\log(1/\kappa)$ |
| [[spectral gap]] $\lambda_2$ | Fiedler value | mass gap; Cheeger bound as isoperimetry |
| locality radius | $O(\log 1/\varepsilon)$ hops | [[Lieb-Robinson bound]]; exponential clustering |
| compute-verify symmetry | boundary flows, $c\approx1$ | [[area law]] |
| bivector grade $w_2$ | $\sum a\,v\,(e_p\wedge e_q)$ | field strength two-form; orientation |
| [[Shapley value]] over $\Delta\phi^+$ | $\int_0^1\partial_\nu v(tN)\,dt$ | thermodynamic integration, $\Delta F=\int_0^1\langle\partial_\lambda H\rangle\,d\lambda$ |
| [[Bayesian Truth Serum]] surprise | difference of KL divergences | dissipated work, $W_{\text{diss}}=k_BT\,D_{\mathrm{KL}}(P_\rightarrow\Vert P_\leftarrow)$ |
| [[karma]] | non-transferable, monotone | entropy production integrated over history |
| [[Goldilocks field]] arithmetic | no float in the provable path | exact discrete state space |

three rows are identities rather than analogies — springs, heat, and Shapley. the rest are correspondences of shape, and the difference matters when the audit starts.

---

## 2. what physics endorses

### 2.1 locality is a theorem, not a convenience

$(L+\mu I)^{-1}$ on a graph is the lattice Green's function of a massive field. $1/\sqrt\mu$ is its Compton wavelength. the experimental fact that massive mediators produce short-range forces — the range of the weak interaction against the $W$ mass — is the same mathematics that lets a [[neuron]] on a phone compute its own reward from a bounded neighborhood.

the locality theorem in [[tri-kernel]] is the graph statement of exponential clustering: a spectral gap forces correlations to decay as $e^{-d/\xi}$. its dynamic twin, the [[Lieb-Robinson bound]], was watched directly in a cold-atom quench, correlations spreading along a sharp cone. **the design did not choose locality for engineering reasons; it inherited the only regime in which a large interacting system is computable at all.**

### 2.2 fair division is thermodynamic integration

the Aumann–Shapley value in the non-atomic limit is the path integral of the gradient. that is, term for term, Kirkwood's coupling-parameter formula for free energy differences — the method used to compute binding affinities and checked against calorimetry for forty years.

so "divide the value created among the contributors" and "attribute a free-energy change to the interactions that produced it" are one computation. it also imports a property the [[rewards]] spec states only as an axiom: as a path integral of a state function, **Shapley shares are path-independent**, which is why no ordering of the epoch's claims can be gamed into a different total.

### 2.3 honesty pricing is the second law

Kawai–Parrondo–Van den Broeck: dissipated work is the relative entropy between a forward process and its time reverse. [[fluctuation theorem|Crooks]] makes the exchange rate exact, and RNA-pulling experiments confirmed it.

the [[Bayesian Truth Serum]] score is a difference of KL divergences between a report and what the crowd predicted. under the correspondence, **an honest report is the quasi-static limit — zero dissipation — and a lie is irreversible work.** the zero-sum that moves stake from noise producers to signal producers is not an incentive invented for a protocol; it is the second law with an account attached.

### 2.4 mining as sampling is thermodynamically forced

the [[thermodynamic uncertainty relation]] says precision costs dissipation: $\mathrm{Var}(J)/\langle J\rangle^2 \ge 2k_B/\Sigma$. halving the error of any estimator quadruples the entropy it must produce.

settlement estimates Shapley shares by sampling, and every sample is a mining ticket. this looked like an elegant reuse. it is stronger than that: **no redesign can make decentralized attribution simultaneously sharp and cheap**, because sharpness is dissipation, and the honest thing a protocol can do is spend that dissipation once instead of twice. proof-of-work as synthetic puzzle plus separate attribution compute is the wasteful variant; collapsing them is the minimal one.

the metric this implies — attribution precision per joule — is one the network should report and does not.

### 2.5 verification by boundary is the area law

gapped systems obey an [[area law]]: the information in a region is bounded by its surface. verification that checks boundary flows and neighborhood commitments rather than recomputing the bulk is the cost-model shadow of that fact, and it is available exactly because the tri-kernel is gapped.

### 2.6 positivity is the right structure for consensus, not a limitation

the [[tri-kernel]] is Perron–Frobenius: positive kernel, unique positive fixed point. that is precisely what makes it incapable of interference — see §3.1 — and precisely what a consensus object requires. **a ranking that admitted amplitude cancellation would not have a unique fixed point, and without a unique fixed point there is nothing for a network to agree on.** the design sits in the classical corner on purpose, and the corner is the correct one for the job.

---

## 3. the audit — where physics falsifies, warns, or bills

### 3.1 the model is classical, and that is a measured fact about the world

loophole-free Bell tests settled it: no local classical model reproduces quantum correlations. $e^{-\tau L}$ preserves positivity; $\phi^*$ is a probability, never an amplitude; nothing in the stack can cancel. therefore **the [[cybergraph]] models the decohered layer of an informational world — the layer where records exist and can be agreed on — and cannot be a model of its quantum layer.**

this is not a defect for the protocol's purpose (§2.6), but it is a bound on the claims the specs may make. an informational-universe framing that implies the graph is the substrate rather than its classical shadow is not supported by the [[Bell inequality]] experiments and should not be written.

**severity: high for the narrative, none for the mechanism.**

### 3.2 free copying, and the prosthesis it requires

physics forbids cloning; content addressing makes it free. the entire honesty layer — surprise $\rho$, market gate $f(\text{price})$, non-transferable [[karma]] — exists to buy back an asymmetry that [[no-cloning theorem|no-cloning]] grants nature for nothing. worth knowing: **that layer is where the design is least protected, because it is the part with no physical law behind it.** the discovery leak named in [[rewards]] §12 — novel links score low on the market gate exactly when surprise is highest — is a symptom of the prosthesis, not of a parameter.

**severity: structural. the open problem is correctly identified in the specs and remains open.**

### 3.3 the specs state the first law and skip the second

conservation — $\sum_\nu \text{mint}(\nu) \le \Delta\phi^+$ — is energy bookkeeping. nowhere in [[tru]] is it written **where the entropy goes.** syntropy rising locally must be paid by entropy rising globally; the [[landauer limit]] fixes the minimum at $k_BT\ln 2$ per bit and has been measured.

the encyclopedia carries this (see [[dissipative structures]]); the normative spec does not. concretely missing: an entropy account stating $\Delta S_{\text{env}} \ge \Delta J$ per epoch, and a declared exchange rate — so many bits of proven syntropy per unit of [[cyber/$CYB|$CYB]] at a declared temperature. **with that rate fixed, inflation becomes a dimensional physical quantity rather than a policy number**, which is what the reward law claims to be.

**severity: medium. a missing section, not a broken mechanism.**

### 3.4 "free energy" is currently a Lyapunov function

$\mathcal{F}(\phi)$ mixes a quadratic elastic term, a heat term and a KL term. the Boltzmann form $\phi^*\propto e^{-\beta E}$ is asserted, but neither $\beta$ nor the partition function $Z$ is derived anywhere. until a temperature is fixed, the object is a decreasing functional that proves convergence — valuable, but not a free energy, and the physical intuitions borrowed from thermodynamics do not transfer without it.

**severity: medium. either derive $\beta$ and $Z$, or rename.**

### 3.5 global normalization needs a preferred frame

$\sum_i \phi^*(i) = 1$ is an instantaneous constraint over the whole graph, and canonical $\phi^*$ per epoch presumes global simultaneity. that is the newtonian limit, and it fails once the graph spans light-minutes.

the repair is standard and physical: replace the global sum with a local conservation law — a continuity equation $\partial_t\phi + \nabla\!\cdot j = s$ with boundary flux — so normalization becomes a property that holds locally everywhere rather than a fact asserted everywhere at once. [[foculus]] needs this before any interplanetary claim survives contact with relativity.

**severity: high for the space doctrine, dormant for one planet.**

### 3.6 a scalar rank is the strongest classical assumption in the stack

Kochen–Specker: observables have no context-independent values, and the experiments agree. `cyberank(p)` assigns one number per [[particle]] independent of the question being asked. the more physical object already exists in the spec — the family $\phi^*_\tau$ across heat scales, and the $k$-dimensional spectral positions emitted to [[mir]] — but the protocol privileges the scalar.

**severity: medium, and testable today (§5).**

---

## 4. predictions running the other way

the previous section spent physics on the design. this one spends the design on physics. these are conjectures, labelled as such, with the tests that would kill them.

### 4.1 fair division picks the unique decomposition of entropy production

splitting entropy production among coupled subsystems is *not* unique in stochastic thermodynamics — Schnakenberg cycle decompositions, Horowitz–Esposito information flows, and learning-rate splittings all exist and disagree. the choice is usually made by convenience.

the [[Shapley value]] is the unique attribution satisfying efficiency, symmetry, null-player and additivity, and in the non-atomic limit it *is* thermodynamic integration (§2.2). **conjecture: the Shapley decomposition of entropy production is the unique one that conserves exactly, and it coincides with the Horowitz–Esposito information flow only where the coupling is submodular.**

test: a two-state bipartite Maxwell demon, solved analytically. compute both splittings across the coupling range and check where they diverge. cheap, decisive, and it is a contribution to physics rather than to a protocol.

### 4.2 markets are a mechanism for self-organized criticality

[[self-organized criticality]] explains critical behaviour without fine-tuning, but its mechanisms are narrow — sandpiles, forest fires, and the cortical avalanche statistics of Beggs and Plenz. why brains sit at criticality is still argued.

[[cyber]] has a candidate mechanism with money in it. measured on the [[superadditivity]] benchmark: collective advantage $\sigma$ rises with connectivity $\lambda_2$ while [[syntropy]] $J$ falls with it. an economy paying for syntropy while rewarding advantage is a control loop with an interior optimum. **conjecture: a reward law of this shape drives a network to criticality without anyone tuning a parameter — and the same shape, run by metabolic rather than monetary cost, is why cortex sits there.**

test: cascade sizes of $\Delta\phi^*$ should be power-law with exponent near $-3/2$ and branching ratio approaching one. the bootloader graph has five years of timestamped cascades and can answer this now.

### 4.3 the arrow of time as non-transferability of records

the second law's direction is conventionally traced to a low-entropy initial condition — the past hypothesis, which is assumed rather than explained.

[[karma]] is the one quantity in the protocol that accumulates monotonically, cannot be transferred, and cannot be bought, and it was designed that way to stop capital from purchasing trust. the resulting irreversibility is *functional*: it exists because records of surprise cannot change hands. **speculation, flagged as such: in an informational universe where agents holding records are primitive, the arrow of time may be a statement about the untradability of records rather than about initial conditions.** entropy would then be the ledger, and its monotonicity a conservation of authorship.

no test proposed. it is a framing, and it is the least defensible claim in this article.

### 4.4 interference is a fourth operator, and we can say what it would buy

the [[tri-kernel]] is complete as a basis of local *classical* operators. the minimal extension that adds amplitude is a rotor $\exp(\theta\, e_p\wedge e_q)$ acting on the bivector grade the [[ct0]] spec already carries — but the arithmetic detail matters and has a clean answer.

the [[Goldilocks field]] has $p = 2^{64}-2^{32}+1 \equiv 1 \pmod 4$, so $-1$ is a quadratic residue, $i$ already lies in $\mathbb{F}_p$, and $\mathbb{F}_p[i]$ splits — a naive complexification degenerates into two real copies and produces no phase. the correct construction is the quadratic extension by a non-residue, $\mathbb{F}_{p^2}$, where phases are the norm-one subgroup of order $p+1 = 2^{64}-2^{32}+2 \approx 1.8\times10^{19}$. **a discrete $U(1)$ with eighteen quintillion phases, field-native and provable in [[zheng]] with no float anywhere.**

the cost is exact and disqualifying for the current mechanism: unitary evolution does not contract, so there is no Banach fixed point, no unique $\phi^*$, and nothing to mint against. this is a fork, not an upgrade.

**conjecture: the rotor kernel buys nothing on ranking and retrieval — Perron–Frobenius problems where positivity is the right structure — and buys a real advantage only where destructive interference is the point, such as detecting near-cancelling contradictory evidence.** test: implement both on the bootloader graph and compare on a contradiction-detection task built from links with opposing [[valence]].

### 4.5 exponential clustering as a verification principle

[[area law]] is a statement about entanglement. the protocol needs a computational cousin: **conjecture — any system whose correlations decay exponentially admits verification whose cost scales with the boundary of the region touched rather than its volume**, with the constant set by the correlation length.

test: measure verify-cost against compute-cost on the live graph as a function of $\varepsilon$-support size. if the ratio is flat in the volume and linear in the boundary, compute-verify symmetry stops being an aspiration.

---

## 5. what to measure on the bootloader graph

the [[bootloader]] left 2,949,732 [[cyberlinks]] over 3,143,650 [[particles]], each with an author, a timestamp and a price paid. it is the only dataset of its kind, and four of the claims above are decidable on it without any new protocol.

| measurement | claim under test | method |
|---|---|---|
| redundancy plateau | [[quantum Darwinism]] reading of [[cyberank]] (§2 dictionary) | mutual information between a particle's rank and $k$ independent neuron-fragments, as $k$ grows |
| avalanche exponent | markets drive criticality (§4.2) | size distribution of $\Delta\phi^*$ cascades; look for $-3/2$ and branching ratio $\to 1$ |
| RG flow of compiled models | [[renormalization group]] reading of [[ct0]] (§1) | compile at a series of $\tau$; check for a fixed point and exponent robustness under link noise |
| multi-scale versus scalar rank | contextuality objection (§3.6) | retrieval quality of $\phi^*_\tau$ family against scalar [[cyberank]] |

the first two are the interesting ones: they test whether five years of human linking produced a system that behaves like a brain at criticality or like a directory with a heavy tail.

---

## 6. the checklist

changes this audit implies, in order of force:

1. **[[rewards]]** — add the entropy account: $\Delta S_{\text{env}} \ge \Delta J$ per epoch, and a declared bits-per-[[cyber/$CYB|$CYB]] rate at a stated temperature, so inflation is dimensional.
2. **[[focusing]]** — express normalization as a continuity equation with boundary flux; the global sum survives as the single-planet special case.
3. **[[tri-kernel]]** — derive $\beta$ and $Z$, or rename $\mathcal{F}$ to what it is: a Lyapunov functional.
4. **[[tri-kernel]]** — state the positivity bound explicitly: Perron–Frobenius means no interference, which is a correctness argument for consensus and a limit on cosmological claims.
5. **[[ct0]]** §7.7 — write the bivector branch as a rotor over $\mathbb{F}_{p^2}$ or mark it explicitly as orientation-only; half a Clifford algebra produces no phases.
6. **[[foculus]]** — report attribution precision per joule; the [[thermodynamic uncertainty relation]] makes it the honest efficiency number for a settlement mechanism.

---

## what this says about designing a superintelligence

the parts of [[cyber]] that came from physics are the parts that need no defence. locality, contraction, coarse-graining by heat, fair division as integration, dissipation as the price of precision — none of these were chosen, and none can be tuned away by a competitor with better parameters. they are the shape any system takes when it must compute a global agreement out of local, paid, verifiable acts.

the parts with no physics behind them are exactly the parts still open: how to price novelty before the market believes it, how to keep a copy from earning, how to divide credit among agents who can lie. physics never had to solve those, because nature has no copies, no strategy, and no need to pay anyone.

so the honest summary is narrower than the ambition and stronger than a metaphor. **the substrate is settled and the incentives are not.** a superintelligence built on this stack will fail, if it fails, in the economics — not in the mathematics.

see [[tri-kernel]] · [[rewards]] · [[syntropy]] · [[superadditivity]] · [[theoretical foundations]] · [[landauer limit]]

discover all [[concepts]]
