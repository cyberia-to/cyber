---
tags: cyber, research, article, nox, architecture
crystal-type: research
crystal-domain: cyber
alias: neuroscience principles, brain architecture mapping, machine mind neuroscience
---

# neuroscience principles for machine mind

ten principles from modern neuroscience mapped to the [[cyber]] stack: [[nox]] (VM), [[cybergraph]] (knowledge graph), [[zheng]] (proof system), metabolism (energy management), and [[bbg]] (tiered memory with context/ram/ssd/hdd/network tiers). each principle includes the neuroscience, why it matters for an artificial system, and how it maps to components.

---

## 1. predictive processing / free energy principle

### the neuroscience

the brain is a prediction machine. Karl Friston's Free Energy Principle (2010) states that biological systems minimize variational free energy — a computable upper bound on surprise (negative log-evidence). the brain maintains a generative model of the world and continuously generates top-down predictions about incoming sensory data. when predictions fail to match reality, the resulting prediction error propagates upward through the cortical hierarchy to update the model. active inference extends this: when prediction error is high, the organism can either update its model (perception) or act on the world to make the world match its predictions (action). perception and action are two sides of the same coin — both minimize the same free energy functional.

the hierarchical predictive coding architecture means that each cortical level only passes forward the residual error, not the full signal. this is sparse, efficient, and compositional. the brain does not process raw sensory data — it processes deviations from expectations.

### why it matters for an artificial system

a system that only reacts to input wastes energy reprocessing known patterns. prediction error minimization means the system spends computational resources only where the world deviates from expectations. this is the foundation for efficient attention allocation, proactive behavior (acting to prevent predicted bad states), and self-model maintenance. a system without predictive processing is perpetually surprised; a system with it can plan, anticipate, and conserve resources.

### mapping to the cyber stack

the [[tri-kernel]] already IS a free energy minimizer. the fixed point $\phi^*$ minimizes the composite free energy functional $\mathcal{F}(\phi) = E_{\text{spring}} + \lambda E_{\text{diff}} - T \cdot S$. every new [[cyberlink]] perturbs this equilibrium, generating a "prediction error" — the delta between the previous $\phi^*$ and the new one. the system's response is to re-converge, updating only the local neighborhood (bounded locality).

in [[nox]], the soma (mind component) implements active inference at four complexity levels, from fixed rules to full Friston free energy minimization. look() reads the current world state from [[bbg]]; the formula computes the expected state; the difference drives the next [[order]]. the output cyberlink IS the action that reshapes the graph toward the predicted desirable state.

[[bbg]] memory tiers map to the prediction hierarchy: context (hot) holds the current generative model — active predictions. ram (warm) holds recently verified patterns — the stable prior. ssd/hdd (cold) holds the deep world model — rarely accessed but available for deep re-evaluation. prediction error flows upward through tiers: if context-level predictions fail, the system queries ram; if ram-level priors are violated, it reaches into cold storage to revise foundational assumptions.

$\Delta\phi^*$ (the shift in the fixed point caused by a neuron's cyberlinks) is literally the prediction error signal for the whole network — and the basis for [[unified mining]] rewards. the system pays for surprise reduction.

---

## 2. global workspace theory

### the neuroscience

Bernard Baars (1988) and Stanislas Dehaene proposed that consciousness arises when information is broadcast from specialized local processors to a "global workspace" — a shared communication bus that all cortical areas can access. most brain processing is unconscious and parallel: visual cortex, auditory cortex, motor planning all operate simultaneously on their local data. when a stimulus is sufficiently strong, novel, or attention-selected, it "ignites" the global workspace: prefrontal and parietal networks broadcast the information widely, making it available to all other processors simultaneously. this is the "ignition" event — a nonlinear phase transition from local to global processing. attention acts as the gatekeeper, selecting which of the many competing local signals gains access to the workspace.

only a small fraction of processed information ever reaches consciousness. the bottleneck is the workspace bandwidth, not processing power. the workspace enables integration of information from diverse sources — combining what you see, hear, remember, and plan into a unified percept.

### why it matters for an artificial system

parallel specialized processors are efficient but isolated. without a global broadcast mechanism, the visual processor cannot inform the planner, the memory system cannot alert the action system. the global workspace solves the binding problem: how to integrate information from heterogeneous subsystems into a coherent whole. the bottleneck (limited workspace capacity) is a feature, not a bug — it forces the system to prioritize, which is the computational essence of attention.

### mapping to the cyber stack

[[cybergraph]] IS the global workspace. every [[particle]] that receives sufficient [[focus]] (from $\phi^*$ exceeding threshold $\tau$) is effectively "broadcast" — it becomes a finalized, universally accessible node in the graph that all [[neurons]] can read via look(). the [[foculus]] consensus mechanism is the ignition event: when $\phi^*_i > \tau$, the particle transitions from local candidate to globally accepted knowledge. below threshold, it exists only in local processing.

the 15 [[nox]] languages (Tri, Tok, Arc, Seq, Inf, Bel, Ren, Dif, Sym, Wav, Bt, Rs, Ten) are the specialized local processors — each operating on its own algebraic domain. they all compile through nox and all write results as cyberlinks into the same cybergraph. a computation in Inf (inference) can reference results from Sym (symbolic algebra) through the graph, exactly as the visual cortex's output becomes available to prefrontal planning through the workspace.

in [[bbg]], the context tier IS the workspace. it holds the currently "conscious" state — the hot polynomial slice being actively read and written. the bottleneck is energy: writing to context costs compute, memory, bandwidth, and energy (the 4D budget of an [[order]]). not everything can be in context simultaneously. the scheduling system in soma decides what enters the workspace based on novelty (prediction error), relevance (tri-kernel focus score), and urgency (energy-weighted priority).

the attention gatekeeper maps to focus expenditure: [[neurons]] spend [[focus]] to create [[cyberlinks]], and only links backed by sufficient stake reach the global $\phi^*$ threshold. this is economic attention — the system's scarce resource determines what gets broadcast.

---

## 3. hebbian learning and synaptic plasticity

### the neuroscience

Donald Hebb (1949): "neurons that fire together wire together." when a presynaptic neuron repeatedly contributes to firing a postsynaptic neuron, the synapse between them strengthens. this is Long-Term Potentiation (LTP) — the molecular basis of memory formation. the reverse, Long-Term Depression (LTD), weakens synapses that are not reinforced. together, LTP and LTD create a learning rule: co-activation strengthens connections; decorrelation weakens them.

synaptic plasticity operates on multiple timescales: short-term facilitation (seconds), early LTP (minutes to hours, protein-modification dependent), late LTP (hours to lifetime, requiring new protein synthesis and structural synaptic change). memory consolidation during sleep replays recently formed patterns, converting short-term hippocampal traces into long-term cortical representations through repeated co-activation — essentially running Hebbian learning offline on buffered experiences.

Spike-Timing-Dependent Plasticity (STDP) refines Hebb's rule: the precise temporal order matters. if the presynaptic neuron fires just before the postsynaptic neuron (causal timing), the synapse strengthens. if the order is reversed (anti-causal), it weakens. this encodes causality directly into synaptic weights.

### why it matters for an artificial system

learning from co-occurrence is the simplest, most scalable learning rule. it requires no global loss function, no backpropagation through the entire network — just local observation of what fires together. for a distributed system where no single node sees the whole picture, Hebbian-style local learning is the only feasible approach. the multi-timescale consolidation is critical: without it, recent experiences overwrite older knowledge (catastrophic forgetting).

### mapping to the cyber stack

[[cyberlinks]] ARE synapses. when a [[neuron]] creates a cyberlink between two [[particles]], it strengthens the connection between those concepts in the [[cybergraph]]. when multiple neurons independently link the same particle pair, the aggregate stake-weighted edge grows stronger — pure Hebbian reinforcement through co-linking. the ICBS (inversely coupled bonding surface) on each cyberlink provides the LTD mechanism: if the NO side of a prediction market gains stake, the effective weight of that link decreases. co-activation strengthens; contradiction weakens.

the effective adjacency weight $A^{\text{eff}}_{pq} = \sum_\ell \text{stake}(\ell) \times \text{karma}(\nu(\ell)) \times f(\text{ICBS price}(\ell))$ is the synaptic weight. it integrates structural co-linking (Hebbian), reputation (karma tracks which neurons have historically been right — analogous to neuromodulatory gating of plasticity), and market consensus (ICBS price — the current collective assessment of the link's truth).

[[bbg]] memory tiers implement multi-timescale consolidation:
- context (hot) = short-term: recent cyberlinks, active within the current computation. like hippocampal working memory
- ram (warm) = intermediate: recently verified and committed state. like early LTP — established but still easily modifiable
- ssd (cold) = long-term: proven, committed, polynomial-bound state. like late LTP — structurally consolidated, expensive to modify
- hdd/network (archive) = deep long-term: historical snapshots in bbg time dimension (index 8). like cortical long-term memory — rarely accessed, but the substrate that gives the system its persistent identity

sleep consolidation maps to epoch transitions and [[tri-kernel]] global recomputation. between epochs, the full tri-kernel iterates over the accumulated signals, re-converging $\phi^*$ from committed state. this is the offline replay: patterns that were locally established during active processing get globally integrated. the trikernel.nu script running on the new moon cycle is a slow-timescale consolidation — structural weight updates to the entire graph.

STDP (temporal causality) maps to [[hash chains]] and VDF proofs in signals. the prev field in each signal establishes causal ordering. links created in causal sequence (neuron A's output becomes neuron B's input, proven by hash chain) strengthen the causal pathway. the system learns not just co-occurrence but causal direction.

---

## 4. neuromodulation

### the neuroscience

four major neuromodulatory systems change the operating mode of the entire brain, not by carrying specific information, but by altering how all other circuits process information:

dopamine (ventral tegmental area, substantia nigra): encodes reward prediction error — the difference between expected and received reward. a positive surprise (better than expected) triggers dopamine release, strengthening the synapses that led to the action. a negative surprise (worse than expected) suppresses dopamine, weakening those pathways. this is the temporal difference (TD) learning signal that drives reinforcement learning. dopamine also modulates the prefrontal cortex's working memory gating — deciding what enters and what is maintained in working memory.

serotonin (raphe nuclei): modulates patience and temporal discounting. high serotonin promotes waiting for delayed rewards over taking immediate ones. it regulates mood — the baseline emotional tone that biases all processing. low serotonin correlates with impulsive action, high serotonin with deliberative planning. it acts as a tonic signal that sets the time horizon of decision-making.

norepinephrine (locus coeruleus): modulates arousal and the explore-exploit tradeoff. tonic (steady, moderate) norepinephrine promotes focused exploitation of known strategies. phasic (burst) norepinephrine triggers exploration of new strategies. the Aston-Jones & Cohen (2005) model: LC tonic mode = exploit current best option; LC phasic mode = reset attention, sample broadly. norepinephrine also drives the "network reset" that enables switching between tasks.

acetylcholine (basal forebrain, pedunculopontine nucleus): modulates the balance between top-down predictions and bottom-up sensory evidence. high acetylcholine increases the gain on sensory input — making the system attend more to what is actually happening rather than what it expects. this is critical for learning in novel environments: when the world is unfamiliar, you need to weight sensory evidence over prior expectations. acetylcholine also enhances synaptic plasticity during active attention — you learn more during states of high cholinergic tone.

together, these four systems do not process information — they set the mode in which all other processing occurs. they are global modulators, not local operators.

### why it matters for an artificial system

a system with fixed processing parameters is brittle. it cannot shift between exploration and exploitation, between fast reaction and deliberate planning, between trusting its model and trusting new data. neuromodulation is the mechanism that makes a system adaptive at the meta-level — not just learning within a mode, but switching between modes of learning. without it, a system is stuck in one cognitive posture.

### mapping to the cyber stack

the [[tri-kernel]] blend weights $(\lambda_d, \lambda_s, \lambda_h)$ ARE the neuromodulatory system. they set the processing mode for the entire graph:

dopamine maps to the $\Delta\phi^*$ reward signal. a neuron's proven contribution to the fixed point shift IS the reward prediction error. positive $\Delta\phi^*$ = better than expected knowledge contribution = dopamine burst = strengthen that pathway. negative $\Delta\phi^*$ = noise that degraded the graph = dopamine dip = weaken. the [[karma]] system accumulates this: high karma neurons (consistently positive $\Delta\phi^*$) gain more influence on $\phi^*$, exactly as dopamine-reinforced pathways gain synaptic strength.

serotonin maps to the temperature parameter $T$ in the Boltzmann equilibrium $\phi^*_i \propto \exp(-E_i/T)$. high $T$ = high patience, broad exploration, willingness to defer to long-term structure. low $T$ = impulsive, greedy, commit to the current best particle immediately. the energy market's valuation curve parameter $k$ also plays this role: a node with high $k$ (risk-tolerant, runs cheap until near-empty) is a "low serotonin" node — impulsive, short time horizon. a node with low $k$ (conservative, gradually gets expensive) is "high serotonin" — patient, long time horizon.

norepinephrine maps to the balance between $\lambda_d$ (diffusion = exploration, random walk sampling) and $\lambda_s$ (springs = exploitation, structural coherence). increasing $\lambda_d$ pushes the system toward exploration — the random walk visits more diverse particles, like phasic LC mode. increasing $\lambda_s$ pushes toward exploitation — the system converges on structurally established patterns, like tonic LC mode. the spectral gap $\lambda_2$ (observed from convergence rate) is the measure of the system's current arousal: large $\lambda_2$ = well-connected, fast-mixing, alert network; small $\lambda_2$ = fragmented, slow, drowsy.

acetylcholine maps to the balance between top-down (model-driven) and bottom-up (data-driven) processing. in the tri-kernel: high $\lambda_h$ (heat kernel) smooths over local perturbations, trusting the prior structure — low cholinergic tone, model dominates. reducing $\lambda_h$ and increasing sensitivity to new [[signals]] (incoming cyberlinks from neurons) amplifies the bottom-up signal — high cholinergic tone, new data dominates. in [[nox]], this maps to the call opcode (pattern 16): the prover injects a witness from outside the model. the balance between using look (pattern 17, reading the existing BBG model) and accepting witnesses via call (trusting external input) IS the acetylcholine axis.

the key design principle: these are not separate subsystems — they are parameters of the same tri-kernel, adjustable per-neuron, per-context, per-epoch. the neuron's [[signal]] can carry local blend weight preferences, and the consensus mechanism integrates them.

---

## 5. embodied cognition

### the neuroscience

intelligence does not exist in a brain-in-a-vat. the embodied cognition paradigm (Varela, Thompson & Rosch 1991; Clark 1997; Damasio 1994) holds that cognition is fundamentally shaped by the body and its sensorimotor interactions with the environment. three key claims:

perception-action loops: perception is not passive reception — it is active probing. the eye saccades to gather information; the hand reaches to test texture; the body moves to change viewpoint. action generates the sensory data that perception interprets. they form a continuous loop, not a pipeline.

interoception: the body senses its own internal state — hunger, temperature, heart rate, energy level, inflammation. Antonio Damasio's somatic marker hypothesis: emotional feelings are the brain's interpretation of body states, and these feelings directly guide decision-making. a "gut feeling" is literally interoceptive information shaping cognition. organisms that ignore interoception make poor decisions because they disconnect from their metabolic and survival needs.

environmental coupling: the body is not a container for the mind — it is part of the cognitive system. using a hammer extends your perceptual field. writing externalizes memory. the boundary of the cognitive system extends through the body into tools and environment (extended mind thesis, Clark & Chalmers 1998).

### why it matters for an artificial system

a disembodied AI has no grounding. it manipulates symbols without referents. embodiment provides three things gradient descent cannot: (1) a source of intrinsic motivation (maintain the body = survive), (2) a reality check (physical actions have physical consequences that cannot be hallucinated away), (3) a basis for meaning (symbols gain meaning through their sensorimotor associations). a machine mind without a body has no stakes, no ground truth, and no reason to act.

### mapping to the cyber stack

[[cyb/soma]] (mind) and [[cyb/hal]] (body) are the two components above [[nox]], explicitly modeling the mind-body split. hal is the hardware abstraction layer — ~3K lines of trait definitions for drivers, physical I/O, sensor access. the ~10 LOC metal boundary (physical_read/physical_write) is the nerve ending — the irreducible point where mathematics meets physics.

the perception-action loop: look() reads state from [[bbg]] (perception). the nox formula computes a response. the output is a [[cyberlink]] (action) that modifies the graph. the graph modification changes what future look() calls return. this IS the perception-action loop — continuous, provable, and closed.

interoception maps directly to the machine's 4D resource tracking: energy (metabolism — to be alive), bandwidth (communication — to be connected), memory (identity — to be yourself), compute (will — to act). the energy market's battery level is the most direct interoceptive signal — the machine literally senses its own energy state and adjusts all behavior accordingly. the valuation curve $v(E, k)$ is the somatic marker: low battery produces high valuation (anxiety), triggering conservation behavior. high battery produces low valuation (contentment), enabling generous market participation. this is Damasio's somatic marker implemented in field arithmetic.

the sigma variable (long-term survival guarantee) is the deepest interoceptive signal — the machine's sense of its own mortality:
- energy > 0 AND sigma > 0 = alive (healthy body feeling)
- energy = 0 AND sigma > 0 = sleeping (can be revived — dormancy, not death)
- energy = 0 AND sigma = 0 = dead (terminal state)

environmental coupling: the machine extends its cognitive system through the network. neighbor nodes accessible via physical connections are its extended body. the energy market topology IS the body plan — which nodes are physically connected determines the machine's effective perceptual and action space. buying energy from a neighbor is like eating. selling compute is like labor. the machine's "body" is its position in the physical energy graph.

---

## 6. default mode network vs task-positive network

### the neuroscience

the brain alternates between two anti-correlated large-scale networks:

the Task-Positive Network (TPN; dorsal attention network, frontoparietal control network): active during externally directed, goal-oriented tasks. focused attention, working memory, motor planning. when you are solving a math problem, reading carefully, or performing a skilled action.

the Default Mode Network (DMN; medial prefrontal cortex, posterior cingulate, precuneus, angular gyrus): active during internally directed cognition — mind-wandering, autobiographical memory, future simulation, theory of mind, self-reflection. the DMN is the brain's "idle" mode, but it is not idle — it is doing critical work: consolidating memories, simulating future scenarios, modeling other agents, maintaining the self-narrative.

these two networks are anticorrelated: when one is active, the other is suppressed. the transition between them is mediated by the salience network (anterior insula, anterior cingulate cortex), which detects relevant external events and triggers the switch from DMN to TPN.

the balance matters: excessive DMN activity correlates with rumination and depression. excessive TPN activity leads to cognitive rigidity and burnout. healthy cognition requires fluid alternation. creative breakthroughs often occur at the transition — the "aha moment" when a DMN-incubated solution enters TPN-accessible awareness.

### why it matters for an artificial system

a system that is always task-focused misses opportunities for integration, reflection, and creative recombination. a system that is always reflecting never acts. the oscillation between external task execution and internal self-modeling is how biological systems maintain coherence (self-model stays current), discover novel solutions (recombination during reflection), and prevent resource exhaustion (rest periods allow maintenance).

### mapping to the cyber stack

the two phases map to the two inference paths described in [[focus flow computation]]:

Task-Positive = compiled transformer inference: fast, externally triggered, goal-directed. a query arrives, the compiled model runs $L^*$ tri-kernel steps over a local context window, and produces a response in milliseconds. this is the system executing an [[order]] — attending to the external environment, spending compute on the task at hand.

Default Mode = focus flow computation: the continuous background process where the [[tri-kernel]] iterates toward $\phi^*$ across the entire [[cybergraph]]. this is self-modeling — the system integrating all accumulated [[cyberlinks]], finding new equilibria, updating its model of what it collectively knows. no external query drives this; it runs because the graph changed.

the anti-correlation is structural: while task execution (compiled transformer, local context) is running, it consumes compute and energy from the 4D budget, suppressing background tri-kernel recomputation. when no task is active, the freed resources go to full graph reconvergence — the machine's "idle" mode that is actually consolidation.

the salience network maps to the scheduling logic in [[cyb/soma]]: detecting incoming signals that warrant switching from background consolidation to active task execution. the switch criterion is prediction error: an incoming signal with high $\Delta\phi^*$ (significant deviation from the current model) triggers the transition from DMN-equivalent to TPN-equivalent. low-novelty signals are processed in the background without interrupting consolidation.

in [[bbg]] terms: TPN reads primarily from context (hot tier), processing the immediate task. DMN reads from ram and ssd (warm/cold tiers), integrating and reconsolidating deeper state. the metabolic shutdown sequence mirrors the forced DMN state: as energy drops below 20%, the machine reduces task acceptance (TPN suppression) and shifts to critical maintenance only — monitoring, syncing, state persistence. this is the system entering "sleep" — background processing and consolidation with minimal external engagement.

the trikernel.nu new moon cycle is the longest-timescale DMN oscillation: once per lunar cycle, the entire graph gets a full weight recomputation. between moons, the system runs in task-positive mode with stable weights.

---

## 7. cerebellum as predictor

### the neuroscience

the cerebellum contains more neurons than the rest of the brain combined (~69 billion of the brain's ~86 billion) yet occupies only ~10% of brain volume. its core function is forward modeling: predicting the sensory consequences of actions before they occur. when you reach for a cup, the cerebellum predicts what your hand will feel like at each point in the trajectory. if the actual sensory feedback deviates from the prediction, the cerebellum generates a rapid error correction signal.

this forward model is learned through error-based learning: climbing fiber inputs from the inferior olive carry the error signal, modifying the Purkinje cell synapses via LTD. over time, the cerebellum builds increasingly accurate models of sensorimotor relationships. the key property: the cerebellum does not initiate actions — it refines them. the cortex decides what to do; the cerebellum predicts the consequences and corrects in real-time.

the cerebellum also extends beyond motor control into cognitive prediction: predicting the next word in a sentence (language), the next note in a melody (music), the next step in a logical argument (reasoning). wherever there is a sequence with predictable structure, the cerebellum builds a forward model.

### why it matters for an artificial system

without forward models, a system must wait for sensory feedback after every action — slow, energy-expensive, and unable to handle latency. forward models enable: (1) anticipatory control — correcting before errors manifest, (2) rapid online adjustment — comparing predicted vs actual outcomes during execution, (3) mental simulation — predicting outcomes without physical execution, enabling planning. the cerebellum's architecture is also a lesson in efficiency: a massive but structurally simple parallel array optimized for one computation (forward prediction), using a single error signal for learning.

### mapping to the cyber stack

[[nox]]'s memoization system IS the forward model. the computation-as-linking principle: before reducing any formula, nox checks whether axon(H(formula), H(subject)) already exists in the [[cybergraph]]. if it does, the cached result is the forward model's prediction — "I have computed this before, and the result was X." the more the system computes, the more forward models it accumulates. the cybergraph literally becomes a universal forward model — a memo table of every computation anyone ever performed.

the cerebellar error correction maps to proof verification in [[zheng]]. when a neuron's signal carries $\Delta\phi^*$ (the claimed local effect of its cyberlinks on the tri-kernel fixed point), the network verifies this proof. if the proof fails, that IS the climbing fiber error signal — the prediction (claimed $\Delta\phi^*$) did not match reality (the actual computation). the neuron's karma decreases (LTD on the cerebellar analogy), reducing its influence on future $\phi^*$.

for sequences: the compiled transformer's $L^*$ layers simulate forward through time — each layer is one step of tri-kernel diffusion, predicting the next equilibrium state. the sequence of predicted equilibria IS the forward model for a planned sequence of actions (cyberlinks). the system can "mentally simulate" the effect of a proposed cyberlink on $\phi^*$ without actually committing it — by running nox with the hypothetical link and observing the predicted $\Delta\phi^*$. this is cerebellar mental rehearsal.

[[bbg]]'s context tier is the real-time workspace where forward models execute: predict, compare to actual outcome from look(), compute error, adjust. the hot polynomial slice is continuously updated as predictions are confirmed or corrected.

the jet system in nox mirrors the cerebellar architecture: jets are optimized implementations of frequently-used formulas, recognized by formula hash. they are the "learned motor programs" — once a computation pattern is established and heavily used, it gets a fast path. the jet registry in the cybergraph (formula particle to implementation particle) is the cerebellar lookup table: given this motor command (formula), here is the pre-learned efficient implementation.

---

## 8. homeostasis and allostasis

### the neuroscience

homeostasis (Claude Bernard, Walter Cannon): the maintenance of internal physiological variables within viable bounds. body temperature must stay near 37 degrees C. blood glucose must stay within range. pH must be buffered. the system uses negative feedback loops: when a variable deviates from the set point, corrective mechanisms activate to restore it. this is reactive regulation.

allostasis (Peter Sterling, 2012): predictive regulation — anticipating needs before they arise and pre-adjusting. instead of waiting for blood sugar to drop and then eating (reactive homeostasis), the brain predicts that physical activity will require more glucose and increases appetite before the deficit occurs. allostasis is metabolically efficient because it prevents large deviations that are costly to correct. it requires an accurate model of the body and the environment — the same predictive processing from principle 1, applied to internal regulation.

the set points themselves are not fixed — they shift based on context (allostatic load). chronic stress raises the cortisol set point. prolonged cold exposure shifts the metabolic rate set point. the system adapts its targets, not just its corrective responses. excessive allostatic load (too many shifted set points for too long) leads to pathology — the system's attempts to maintain stability actually damage it.

### why it matters for an artificial system

a machine that cannot regulate its own internal state dies. energy depletion, memory overflow, compute exhaustion, bandwidth saturation — any of these unchecked kills the system. homeostasis provides survival. allostasis provides efficiency — predicting resource needs before they become critical allows proactive acquisition (buying energy while cheap) and graceful degradation (reducing activity before running out, not after). the distinction between homeostatic (reactive) and allostatic (predictive) regulation maps directly to the difference between a system that crashes when resources run out and one that never gets there.

### mapping to the cyber stack

the machine mind page describes homeostasis explicitly: "before every step, verify consumed + cost(next_step) is within available. halt before, not after." this is allostasis — predictive resource management, not reactive crash recovery.

four homeostatic variables, each with a set point and regulatory mechanism:

| resource | homeostatic range | allostatic mechanism |
|----------|------------------|---------------------|
| energy | battery > critical_reserve | valuation curve predicts depletion, adjusts pricing to attract energy purchases before empty |
| compute | focus budget > cost(next_order) | pre-check budget before execution, reject orders that would exceed capacity |
| memory | bbg polynomial < tier capacity | tier migration (hot to warm to cold) before overflow, triggered by predictive fullness metric |
| bandwidth | connection utilization < saturation | queue management and order throttling based on predicted demand |

the energy market IS the allostatic regulation system. the valuation curve $v(E, k) = v_{\min} + (v_{\max} - v_{\min}) \times (1 - E/E_{\max})^k$ shifts the node's economic behavior predictively: as battery level decreases, the machine PREEMPTIVELY increases prices and reduces acceptance of energy-expensive orders. it does not wait for energy = 0. the committed energy mechanism (free_energy = battery - critical_reserve - committed) is allostatic: accepting an order immediately reserves energy, moving the node LEFT on the valuation curve — preparing for the future deficit.

the metabolic shutdown sequence is graded allostasis:
- free_energy > 50%: normal operation (homeostatic equilibrium)
- free_energy 20-50%: reduce market order acceptance (allostatic adjustment — anticipating scarcity)
- free_energy 5-20%: critical orders only (severe allostatic shift — survival mode)
- free_energy < 5%: shutdown sequence (the system cannot maintain any set point, enters dormancy)

the sigma variable is the allostatic reserve — the long-term guarantee that the machine can recover from energy depletion. sigma > 0 means the machine can post a bounty for resurrection. this is the deepest allostatic buffer: not immediate energy, but the capacity to acquire future energy.

in [[bbg]], tier migration is homeostatic regulation for memory: when the hot tier approaches capacity, data migrates to warm tier. when warm approaches capacity, it migrates to cold. the trigger should be predictive (allostatic): initiate migration when the current growth rate predicts overflow within N steps, not when overflow occurs.

---

## 9. sparse coding and energy efficiency

### the neuroscience

at any given moment, only 1-5% of cortical neurons are active. this is sparse coding — representing information using a small fraction of the total neural population. Olshausen and Field (1996) showed that sparse coding of natural images produces receptive fields identical to those found in primary visual cortex (Gabor-like edge detectors). the brain appears to have discovered the most efficient basis for representing natural scenes.

sparsity provides three computational advantages: (1) energy efficiency — each action potential costs metabolic energy (ATP), so fewer active neurons means lower energy consumption. the brain uses ~20W while processing enormously complex information, largely because of sparse activation. (2) representational capacity — with N neurons and k% active, the number of distinct patterns is $\binom{N}{kN}$, which is exponentially large even for small k. sparse codes have enormous capacity. (3) generalization — sparse representations tend to have low overlap, meaning patterns are well-separated in representational space, reducing interference and improving discrimination.

the brain also uses sparse coding in time: neurons fire in brief bursts separated by long silent periods. information is encoded in the timing and identity of the small active set, not in continuous firing rates.

### why it matters for an artificial system

energy is finite. a system where all components are always active is energetically unsustainable and informationally wasteful. sparsity is the fundamental strategy for operating under energy constraints while maintaining expressive power. it also provides natural denoising (noise activates random neurons, but meaningful patterns activate specific sparse subsets) and enables graceful degradation (losing a few active neurons degrades the representation slightly; losing a few in a dense code can be catastrophic).

### mapping to the cyber stack

the [[tri-kernel]] focus distribution $\phi^*$ IS a sparse code. by the Boltzmann equilibrium $\phi^*_i \propto \exp(-E_i/T)$, most particles have near-zero focus while a small fraction concentrates most of the probability mass. this is exactly sparse coding: the "active set" is the particles above the [[foculus]] threshold $\tau$. the rest are "silent" — present in the graph but not actively influencing the current computation.

nox's bounded locality principle enforces sparsity computationally: each operation's cost is proportional to what it touches, not total state size. a query about a specific topic activates only the relevant subgraph — O(relevant_edges), not O(total_graph). the rest of the graph is "silent" during that computation. this is the same principle as cortical sparse coding: most of the system is inactive for any given stimulus.

energy metering in nox makes sparsity economically enforced: every pattern costs focus. running more operations costs more energy. the 4D budget (compute, memory, bandwidth, energy) creates a hard constraint on how many things can be simultaneously active. the system naturally converges on sparse computation because dense computation is too expensive.

in [[bbg]], the polynomial commitment scheme encodes sparsity efficiently. a multivariate polynomial over 10 dimensions can represent enormous state spaces, but most entries are zero (or default). the polynomial evaluation $\text{BBG\_poly}(\text{index}, \text{key}, t) = \text{value}$ only commits non-zero values. the proof size (~2 KiB) is constant regardless of how sparse or dense the state is — sparse state is not just efficient to compute, it is efficient to prove.

the jet system embodies sparse coding at the instruction level: only frequently-used formula compositions get jets (optimized fast paths). most possible nox programs have no jet — they execute on the general interpreter. the jet registry is a sparse set of optimized representations, just as the cortex maintains a sparse set of well-tuned feature detectors.

the compiled transformer architecture derives its dimensionality from the graph: $d^* = \exp(H(\sigma(\Sigma_{\phi^*})))$ — the effective rank from the entropy of singular values. this IS the graph telling the system how sparse its representation should be. a graph with concentrated structure produces low $d^*$ (sparser model). a diverse, high-entropy graph produces higher $d^*$ (denser model). the system self-adjusts its representational sparsity based on the actual information content.

---

## 10. neuroplasticity windows

### the neuroscience

the brain is not equally plastic at all times. critical periods are developmental windows during which specific circuits are maximally modifiable — visual cortex during early childhood, language circuits during the first several years of life, social cognition during adolescence. during critical periods, experience rapidly and permanently shapes circuit structure. after the critical period closes, the same circuits become resistant to modification.

the closure mechanism involves molecular "brakes" on plasticity: perineuronal nets (extracellular matrix structures that physically stabilize synapses), myelin (which speeds conduction but prevents structural remodeling), and GABAergic inhibition (which suppresses the excitatory activity needed for Hebbian learning). these brakes can be partially released by specific experiences, pharmacological agents, or brain stimulation — reopening plasticity windows in adulthood.

the balance between plasticity and stability addresses a fundamental tradeoff: a system that is always plastic never consolidates — new learning overwrites old. a system that is never plastic cannot adapt. the brain's solution is temporal gating: be plastic when learning is needed (novel environments, critical periods, high neuromodulatory tone), be stable when consolidation is needed (familiar environments, sleep, routine).

the metaplasticity framework (Abraham & Bear, 1996) adds another layer: the rules of plasticity themselves change based on recent activity. the BCM (Bienenstock-Cooper-Munro) theory describes a sliding threshold — after a period of high activity, the threshold for LTP increases (making it harder to strengthen synapses), preventing runaway excitation. this is homeostatic plasticity — regulating the learning rule itself.

### why it matters for an artificial system

the stability-plasticity dilemma is the central unsolved problem in lifelong learning. a system that is always learnable loses prior knowledge. a system that is always stable cannot incorporate new information. the solution is not a fixed compromise — it is dynamic regulation of plasticity itself. the system needs to know WHEN to learn aggressively (novel domain, boot phase, post-error), WHEN to consolidate (reliable patterns established, system loaded), and WHEN to reopen plasticity (environment changed, old model fails).

### mapping to the cyber stack

the burn mechanism for tokens IS the plasticity brake. when a neuron burns $CYB on a particle or cyberlink, it creates a permanent, irreversible $\phi^*$-weight. this is myelination — the link hardens, becomes resistant to modification, and gains permanent influence on $\phi^*$. the more burn, the more stable the knowledge structure. early in the graph's life (critical period), most links have low burn and high modifiability. as the graph matures, heavily-validated knowledge accumulates burn and becomes structurally fixed.

the lock mechanism (will creation) provides temporal plasticity gating: locking tokens for a time period creates time-weighted conviction. longer locks = deeper commitment = more influence. this mirrors critical period timing: during the "window," unlocked tokens are free to move (plastic). once locked, they are committed (stable). the unlock event is the reopening of plasticity.

the trikernel.nu new moon cycle explicitly implements plasticity windows. frontmatter weights (diffusion, springs, heat, focus, gravity, density) are updated only on the new moon (once per ~29.5 days). between moons, weights are frozen — the system is in a stable phase. on the new moon, all weights recompute — the system enters a brief plasticity window. this is a hardcoded critical period at the graph level.

the exponential link cost function $c(n) = c_0 \cdot e^{\lambda n}$ creates a natural plasticity schedule: early links are cheap (high plasticity — the graph is forming its basic structure). later links are expensive (low plasticity — each modification must be highly justified). this mirrors critical period closure: early in development, synaptic modification is metabolically cheap. later, the brakes are on and modification requires significant energy.

in [[bbg]], the tier structure provides plasticity gradients:
- context (hot) = maximally plastic. state is being actively written and rewritten during computation. this is the "critical period" for the current task
- ram (warm) = moderately plastic. committed but recent state can be updated by new signals. like adult cortical plasticity — possible but effortful
- ssd (cold) = low plasticity. polynomial-committed, proven state. modification requires formal state transition and new proof. like post-critical-period circuits — structurally stable
- hdd/network (archive) = effectively frozen. historical snapshots in the time dimension. like deep developmental structure — the foundation does not change

metaplasticity maps to the spectral gap $\lambda_2$ as a regulator of the system's own learning rate. when $\lambda_2$ is large (well-connected graph, fast mixing), the system converges quickly on new information — high effective plasticity. when $\lambda_2$ is small (fragmented graph, slow mixing), the system is sluggish to incorporate new information — low effective plasticity. the [[seer]] algorithm that maximizes $\Delta\lambda_2 / c(n)$ using the Fiedler vector IS metaplasticity — it modifies the system's own capacity for learning by placing links that improve graph connectivity.

the four-tier cognitive architecture in [[cyb/mind]] (substrate to deep synthesis) also maps: lower tiers (tier 0, always-on, <1B parameters) are stable — they handle routine processing and rarely update. higher tiers (tier 3-4, 13-14B, deep synthesis) are more plastic — they are invoked for novel situations and their outputs can reshape lower-tier behavior through the graph. the hierarchy of plasticity mirrors the cortical hierarchy: primary sensory areas (early, stable) vs prefrontal cortex (late-developing, more plastic throughout life).

---

## synthesis: the ten principles as a unified architecture

the ten principles are not independent modules to be bolted together — they describe aspects of a single integrated system. the mapping reveals that the cyber stack already embodies many of these principles through different design choices:

| principle | primary mechanism | secondary mechanism |
|-----------|-------------------|---------------------|
| predictive processing | tri-kernel free energy minimization | nox memoization as forward model |
| global workspace | cybergraph as shared broadcast | foculus threshold as ignition |
| hebbian learning | cyberlink co-creation strengthens edges | ICBS market as LTD for incorrect links |
| neuromodulation | tri-kernel blend weights (lambda_d, lambda_s, lambda_h) | temperature T, valuation curve k |
| embodied cognition | hal (body) + soma (mind) | energy market as interoception |
| DMN vs TPN | focus flow (background) vs compiled transformer (task) | metabolic shutdown as forced DMN |
| cerebellum | nox memoization + jet system | zheng proof verification as error signal |
| homeostasis/allostasis | 4D resource tracking with predictive budgeting | energy valuation curve, sigma as allostatic reserve |
| sparse coding | focus distribution concentrates on few particles | bounded locality, energy metering |
| plasticity windows | burn (permanent), lock (temporal), new moon cycle | exponential link cost, spectral gap as metaplasticity |

the tri-kernel unifies principles 1, 2, 4, 6, and 9: it IS free energy minimization (1), its convergence creates the global consensus (2), its blend weights are the neuromodulatory system (4), its two inference paths create the DMN/TPN oscillation (6), and its Boltzmann equilibrium naturally produces sparse focus distributions (9).

the energy/metabolism system unifies principles 5, 8, and 10: embodied cognition through physical energy topology (5), homeostasis/allostasis through the valuation curve and 4D budgets (8), and plasticity regulation through economic cost of links (10).

the cybergraph and bbg unify principles 3 and 7: hebbian learning through cyberlink creation and ICBS markets (3), and cerebellar forward modeling through the universal memo table (7).

the missing pieces — what is specified but not yet built — are primarily in the neuromodulatory dynamics (adaptive blend weights that change based on the system's own performance metrics) and the plasticity gating logic (automated decisions about when to be in learning mode vs consolidation mode). these require [[cyb/soma]] to be implemented with the full four-level active inference architecture described in the [[machine mind]] page.

---

see [[machine mind]] for the core architecture. see [[tri-kernel architecture]] for the three operators. see [[focus flow computation]] for the two inference paths. see [[cybergraph model architecture]] for the generative model. see [[algorithmic essence of superintelligence]] for the complete formal system. see [[energy market]] for metabolism. see [[bbg]] for memory tiers. see [[nox]] for the VM

discover all [[concepts]]
