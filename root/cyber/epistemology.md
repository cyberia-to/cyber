---
tags: cyber, article, cip
crystal-type: pattern
crystal-domain: cyber
alias: epistemic correctness, epistemic quality, truth tracking
stake: 28558835390456748
focus: 0.00039285351897076575
gravity: 6
---
# epistemology

## 1. two kinds of correctness

[[cyber]] makes two categories of claim about its [[focus]] distribution π.

Cryptographic correctness: every state transition is valid, every [[stark]] proof is sound, [[focus]] conservation holds structurally. The protocol guarantees this through [[Hemera]] hash binding, [[nox]] deterministic reduction, and polynomial commitment verification. Given the soundness of the proof system, these guarantees hold with probability ≥ 1 − 2⁻¹²⁸.

Epistemic correctness: the [[focus]] distribution π tracks something meaningful about the world — that high-π [[particles]] represent knowledge worth attending to, and that the ranking reflects collective intelligence rather than collective error. The protocol assumes this emerges from [[costly signals]], convergence, and stake-weighted aggregation.

The boundary: cryptographic proof ends at "this computation was performed correctly." Epistemic quality begins at "this computation was worth performing." Everything below that boundary is proven. Everything above it is argued, conjectured, or hoped for.

This article maps the boundary, catalogs the threats that operate above it, and identifies what remains to be proven.

## 2. what cryptographic correctness guarantees

Five properties are mathematically established:

- Convergence: the [[collective focus theorem]] proves that the [[tri-kernel]] operator is a contraction with coefficient κ < 1 under ergodicity assumptions. A unique fixed point π* exists. The system converges to it at linear rate. see [[collective focus theorem]]

- Conservation: Σᵢ focus(i) = 1 at every state. Enforced by [[stark]] circuit constraints on every transition. No minting, no inflation, no forgery — structural invariant. see [[cyber/proofs]]

- Sybil resistance: [[focus]] influence is proportional to staked [[tokens]], not to node count. Creating 1000 [[neurons]] with zero stake produces zero π influence. The cost of shifting π is the cost of acquiring stake. see [[cyber/security]]

- Completeness: [[cyber/bbg]] namespace proofs guarantee that sync responses contain every edge in the requested namespace. An adversary can add false [[cyberlinks]] but cannot hide true ones from any client that asks.

- Unforgeability: every [[cyberlink]] requires a valid signature from the creating [[neuron]]. Every private transfer requires a ZK proof of ownership. Claims without cryptographic backing are rejected at the protocol level.

These five properties compose into a system where every piece of data is authenticated, every computation is verifiable, every resource is conserved, and every query is provably complete. This is a remarkable foundation. It is also insufficient for epistemic quality.

## 3. where epistemic assumption begins

Each proven property has a corresponding epistemic gap:

Convergence proves π* is well-defined. It does not prove π* is desirable. The [[collective focus theorem]] guarantees a unique fixed point — but the fixed point of a network where every [[neuron]] links to propaganda is a propaganda-weighted distribution. Uniqueness is a mathematical property. Quality is not.

Conservation proves resources are scarce. It does not prove that scarcity produces quality. A [[neuron]] can burn all its [[focus]] on a single false [[cyberlink]]. The link is costly. The link is also wrong. Cost constrains volume, not accuracy.

Sybil resistance proves the cost of attack is proportional to stake. It does not characterize what happens at 49% adversarial stake, or at 10% stake with coordination, or at 1% stake sustained over years. The boundary between "too expensive to attack" and "profitably attackable" depends on parameters the protocol leaves unspecified.

The [[collective focus theorem]] proves consensus. The gap between consensus and [[truth]] requires an additional argument that honest linking is incentive-compatible and that the neuron population is epistemically diverse. Neither is proven.

The system currently relies on an implicit chain: costly signals → honest linking → diverse perspectives → convergent π reflects reality. Each arrow is plausible. None is proven. The remainder of this article examines what could break each arrow.

## 4. threat model for epistemic quality

### 4.1 stake cartel

Top N [[neurons]] coordinate to shift π toward a target [[particle]]. Each cartel member creates [[cyberlinks]] from high-π particles to the target, channeling [[diffusion]] flow.

Cost structure: opportunity cost of honest linking. Every [[cyberlink]] spent on manipulation is a link not spent on genuine knowledge contribution. If the cartel controls fraction f of total stake, it controls fraction ~f of regenerated [[focus]] per epoch.

For f = 0.2 (five [[neurons]] with 4% stake each), the cartel can dedicate 20% of per-epoch [[focus]] to coordinated manipulation. Whether this is sufficient to shift π meaningfully depends on graph density around the target — sparse neighborhoods are cheaper to manipulate than dense ones.

Defense: temporal decay erodes gains. Without sustained coordination, manipulated π decays back toward honest equilibrium at rate α per block. The cartel must spend continuously, not once.

Defense gap: if the cartel's revenue from manipulation (e.g., boosting a particle that earns them trading profits) exceeds the ongoing [[focus]] cost, the attack is self-sustaining. No current analysis bounds when this condition holds.

### 4.2 borrow attack

Lease stake from yield farms or lending protocols → delegate to attacker [[neurons]] → create manipulative [[cyberlinks]] → return stake after [[focus]] regeneration window.

Cost structure: borrowing fee for duration T. If [[focus]] regenerates to usable levels within T blocks, the attacker pays only the loan interest, not the full token purchase price.

This reduces the capital requirement from "acquire f fraction of stake" to "pay interest on f fraction of stake for T blocks." At 10% APR and T = 1 day, the cost drops by ~270×.

Defense gap: the protocol does not distinguish owned from borrowed stake. [[focus]] regeneration rate relative to borrowing cost determines whether this attack is profitable. If regeneration is slow (many epochs to full capacity), the borrow window closes before meaningful manipulation. If regeneration is fast (one epoch), the attack is cheap.

### 4.3 long-horizon deception

Gradual π drift via many small [[cyberlinks]] over months. No single link is suspicious — each costs a normal amount of [[focus]] and shifts π by an imperceptible ε. Cumulative effect: large epistemic distortion over thousands of blocks.

This is the epistemic analog of boiling a frog. The tri-kernel's convergence guarantee actually works against defense here — the system smoothly converges to each intermediate state, treating the gradual drift as legitimate evolution of collective attention.

Defense: temporal decay means old links lose weight. If the deception rate (links per epoch) is slower than the decay rate, the attack cannot accumulate. If faster, drift compounds.

Defense gap: the optimal deception rate is just above the decay threshold — fast enough to accumulate, slow enough to avoid detection. No current mechanism detects this regime, because each individual link is indistinguishable from honest linking.

### 4.4 epistemic monoculture

Homogeneous [[neurons]] — same training data, same model architecture, same priors — converge to a shared bias. The [[tri-kernel]] amplifies agreement: [[diffusion]] concentrates probability on particles that many [[neurons]] link, [[springs]] enforces structural consistency, [[heat kernel]] smooths away dissent at low temperature τ.

If 80% of active [[neurons]] are models trained on the same corpus, the [[cybergraph]] inherits the corpus's biases, omissions, and hallucinations — with high π confidence, because all agents agree.

The [[egregore]] page invokes the Condorcet jury theorem: error decays exponentially with group size when each agent has independent probability p > 0.5 of being correct. The critical assumption is independence. Agents sharing training data are correlated, and correlated errors do not cancel — they compound.

Defense: the [[tri-kernel]]'s three operators provide some structural diversity (flow, structure, scale). But operator diversity is distinct from agent diversity. Three views of the same biased graph still yield a biased result.

Defense gap: no protocol-level mechanism measures or incentivizes neuron diversity. A graph-computable diversity metric — correlated with epistemic resilience — is an open problem.

### 4.5 parameter gaming

The [[foculus]] adaptive threshold τ(t) = μ_π + κσ_π depends on the variance of the current π distribution. An attacker can oscillate σ_π by creating and removing [[cyberlinks]] that spike high-π particles, alternating between concentrated and dispersed distributions.

If τ oscillates faster than the convergence rate, finality is repeatedly deferred. During uncertainty windows, the attacker executes side attacks (front-running, double-linking) that exploit the lack of committed state.

The [[cyber/whitepaper]] §14 acknowledges threshold gaming as an open question. The attack is structurally possible — the question is whether the cost (in [[focus]]) of spiking σ_π exceeds the attacker's gain from deferred finality.

## 5. existing partial defenses

### 5.1 [[focus]] cost as [[costly signal]]

Every [[cyberlink]] costs [[focus]]. [[focus]] regenerates proportionally to staked [[tokens]], so linking capacity is bounded by economic commitment. This prevents unbounded spam and ensures that each link carries opportunity cost — [[focus]] spent on one [[cyberlink]] is [[focus]] unavailable for another.

The [[costly signal]] literature (Spence 1973, Zahavi 1975) establishes that signals correlated with cost reveal information about the signaler's type. In [[cyber]], the cost of linking is proportional to the [[neuron]]'s stake — high-stake neurons pay more absolute [[focus]] per link and thus have more to lose from frivolous linking.

Limitation: cost prevents volume, not inaccuracy. A single expensive false link is still false. Cost-based honesty requires that the return to honest linking exceeds the return to dishonest linking — a game-theoretic condition that depends on reward structure, not just signal cost.

### 5.2 temporal decay

Edges lose weight exponentially: w_eff(e, t) = e.weight · α^(t − e.time). False consensus requires sustained expenditure. Stale falsity decays; fresh truth compounds.

This is the protocol's primary passive error correction mechanism. Unlike systems where false consensus persists indefinitely (e.g., early Wikipedia edits that survive decades), the [[cybergraph]] forgets. Every claim must be renewed by ongoing [[focus]] expenditure to maintain its π share.

The decay rate α determines effectiveness. If α is close to 1 (slow decay), false consensus persists for many blocks. If α is close to 0 (fast decay), even true knowledge decays before it accumulates influence. The optimal α balances forgetting errors against remembering signal. No current analysis characterizes this tradeoff.

### 5.3 [[tri-kernel]] operator diversity

Three operators rather than one. [[diffusion]] measures flow (where does probability go?). [[springs]] measures structure (what configuration is internally consistent?). [[heat kernel]] measures scale (what is the graph's shape at resolution τ?).

An attack vector optimized against [[diffusion]] alone (e.g., creating a high-in-degree target to attract random walk probability) may fail against [[springs]] (which penalizes structural inconsistency in the link pattern) or [[heat kernel]] (which detects anomalous local structure at scale τ).

This "operator diversity advantage" is real but unquantified. Formalizing it requires analyzing the intersection of optimal attack strategies across the three kernels — is the set of attacks that simultaneously fool all three strictly smaller than the set fooling any one?

### 5.4 namespace completeness

[[cyber/bbg]] proofs guarantee that every sync response is complete: "these are ALL edges in namespace N." An adversary can create false [[cyberlinks]] — this costs [[focus]] and is visible to all — but cannot suppress true [[cyberlinks]] created by honest [[neurons]].

This is a meaningful asymmetry. In traditional information systems, censorship (hiding true information) is often cheaper than fabrication (creating false information at scale). In the [[cybergraph]], censorship is structurally impossible while fabrication costs [[focus]]. The attacker must outspend truth, not merely silence it.

Limitation: completeness guarantees data availability, not data quality. Every link is visible. Whether a visible link is honest is the epistemic question that completeness does not answer.

## 6. open problems

### 6.1 Nash equilibrium of honest linking

Under what parameter regimes (teleport α, screening μ, temperature τ, focus cost c, decay rate, regeneration rate) is honest linking a Nash equilibrium? "Honest linking" here means: the [[neuron]] maximizes long-term expected reward by creating [[cyberlinks]] that reflect its genuine assessment of relevance.

This requires a formal game-theoretic model where each [[neuron]] chooses a linking strategy, the [[tri-kernel]] computes π from the resulting graph, and rewards accrue proportionally to Δπ contribution. The solution concept is Nash equilibrium in the space of linking strategies.

If honest linking is not a Nash equilibrium for some parameter values, those values represent the protocol's epistemic vulnerability surface.

### 6.2 minimum attack cost

What is the minimum stake s* required to shift π by ε on a target [[particle]]?

$$s^* = f(\text{graph topology}, \pi_{\text{current}}, \alpha, \mu, \tau, \varepsilon)$$

This is the protocol's epistemic security parameter — analogous to the economic security parameter in proof-of-stake (cost to finalize a false block). If s* is known, operators can reason about whether the attack cost exceeds any plausible attacker's budget.

Computing s* requires analyzing the sensitivity of the tri-kernel fixed point to perturbations in the link structure, weighted by the attacker's available [[focus]]. Closed-form bounds exist for simple graphs (e.g., star topology). Bounds for realistic [[cybergraph]] topologies are open.

### 6.3 diversity measurement

The Condorcet jury theorem requires independent agents. The Hong-Page diversity theorem requires genuinely different problem-solving heuristics. Both are invoked in [[egregore]] to argue that collective intelligence emerges from the neuron population.

Neither theorem applies when agents are correlated. A graph-computable diversity metric is needed: given the current neuron population and their linking patterns, how epistemically diverse is the collective? Candidates:

- Linking entropy: H(link distributions across neurons). High when neurons link to different particles; low when they converge on the same targets.
- Spectral diversity: variance in the eigenvector contributions of different neurons to π.
- Prediction independence: correlation between neurons' Δπ contributions over time. Truly independent neurons have low correlation.

None of these is specified in the protocol. Measuring and incentivizing diversity remains open.

### 6.4 external anchoring

The [[cybergraph]] is self-referential: π is computed from [[cyberlinks]], which are created by [[neurons]], whose influence is weighted by π. This loop can stabilize around any self-consistent configuration, including false ones.

Optional external anchoring breaks the self-reference by introducing signals from outside the loop:

- Prediction markets: particles with verifiable outcomes (future events, measurable claims) can anchor π calibration. If π predicts rain tomorrow and it does not rain, the miscalibration is measurable.
- Sensor networks: physical measurement feeds (temperature, location, chemical composition) provide ground truth against which linking accuracy can be evaluated.
- Cross-graph proofs: other [[cybergraph]] instances with different neuron populations provide independent estimates. Divergence between instances signals epistemic vulnerability.

External anchoring is architecturally optional — the protocol operates without it. But calibration against external reality is the only known mechanism for breaking the self-reference loop that enables stable false consensus.

### 6.5 error correction beyond decay

Temporal decay is passive: old links lose weight regardless of truth value. Active error correction mechanisms complement decay:

- Challenge protocols: any [[neuron]] can stake [[focus]] against a [[particle]]'s current π ranking, asserting it is too high or too low. If subsequent π evolution validates the challenge, the challenger is rewarded from the decayed [[focus]] of links that were pushing π in the wrong direction.

- Falsification bounties: [[neurons]] that successfully identify and link refutations of high-π claims earn disproportionate Δπ reward. This incentivizes epistemic auditing as a profitable activity.

- Adversarial auditing: a rewarded role where [[neurons]] deliberately search for manipulated π regions. Detectable patterns include: sudden π spikes from few sources, structural anomalies in link patterns, statistical deviation from expected tri-kernel behavior.

None of these mechanisms exist in the current protocol. Each requires careful design to avoid creating new attack surfaces (e.g., challenge protocols can themselves be used for manipulation if the resolution mechanism is gameable).

## 7. the honest claim

[[cyber]] claims convergent collective attention under conservation laws, provable by anyone, resistant to unfunded manipulation, self-correcting via temporal decay.

This is weaker than "truth." A system that converges to stable collective attention can converge to stable collective error if the neuron population is biased, cartelized, or monocultural. The convergence proof guarantees the destination is well-defined, not that the destination is correct.

This is stronger than "popularity." [[focus]] conservation, stake weighting, and temporal decay impose costs, incentives, and forgetting that raw popularity metrics lack. The result is constrained collective attention — attention that obeys physical laws even if it does not perfectly track reality.

The gap between convergent attention and [[truth]] is the space where epistemic quality lives. Cryptographic correctness builds the floor — provable, permanent, unconditional. Epistemic correctness is the structure above it — argued, measured, refined, and always provisional. The protocol provides the floor. Closing the gap is the work of generations of [[neurons]], the accumulation of external anchors, the development of diversity metrics, and the hard game-theoretic analysis of incentive compatibility.

The floor is built. The gap is mapped. The work continues.

see [[cyber/whitepaper]] for the full protocol specification, [[collective focus theorem]] for the convergence proof, [[cyber/security]] for the cryptographic threat model, [[foculus]] for the consensus mechanism and its open questions