---
tags: meta, spiri
crystal-type: entity
crystal-domain: meta
---
# epistemology

the study of [[knowledge]] — what it is, how we get it, what justifies it, and where it fails. epistemology is the oldest and most consequential branch of [[philosophy]], because every other question presupposes an answer to: how do you know?

## the problem

all knowledge claims face three challenges:

- definition — what counts as knowledge? the classical answer since [[Plato]]: justified true belief. you know something when you believe it, it is true, and you have good reason to believe it. in 1963 [[Edmund Gettier]] showed that justified true belief is insufficient — you can have justified true belief by accident. the definition problem remains open
- justification — what counts as a good reason? every justification rests on prior beliefs. those beliefs rest on others. this is the regress problem: either the chain is infinite (infinitism), or it loops (coherentism), or it terminates in foundations that need no justification (foundationalism). each option has consequences. foundationalism asks what the foundations are. coherentism asks what makes a web of beliefs consistent. infinitism asks how we tolerate infinite chains
- scope — what can be known at all? [[David Hume]] showed that induction (generalizing from observed cases) has no logical justification — the sun rising every day does not prove it will rise tomorrow. this is the problem of induction. it means all empirical knowledge is provisional. [[Karl Popper]] responded: science does not prove, it falsifies. a theory is scientific if it can be refuted. what cannot be refuted is not knowledge but dogma

## historical arc

### ancient

[[Plato]] divided reality into [[phenomena]] (the visible, changing world) and Forms (the eternal, knowable world). true knowledge is of the Forms; the senses deliver only opinion. [[Aristotle]] disagreed: knowledge begins in [[perception]], proceeds through induction, and arrives at universal principles. the Plato-Aristotle split — rationalism vs empiricism, top-down vs bottom-up — echoes through every century since

### early modern

[[Rene Descartes]] radicalized doubt: what if everything I perceive is illusion? the only certainty is the doubting itself — cogito ergo sum. from this foundation he rebuilt knowledge through reason alone. [[John Locke]] countered: the mind at birth is a blank slate (tabula rasa); all knowledge comes from experience. [[George Berkeley]] pushed further: matter itself is nothing but [[perception]]. [[David Hume]] completed the empiricist arc by showing that even [[causation]] is habit, not logic — we see conjunction, not connection

### synthesis and limits

[[Immanuel Kant]] showed that both rationalists and empiricists were half right. the mind imposes [[categories]] — [[space]], [[time]], [[causation]] — on raw sensory input, and only then does experience become [[knowledge]]. knowledge is constructed, not received and not invented. Kant also identified synthetic a priori knowledge — truths that are necessarily true yet go beyond definitions. [[math]] lives here

[[Kurt Goedel]] showed that any sufficiently powerful formal system contains truths it cannot prove (incompleteness, 1931). [[Alan Turing]] showed that some questions cannot be answered by any computation (halting problem, 1936). together they map the hard boundary of what formal reasoning can achieve

### information and collectives

[[Claude Shannon]] quantified knowledge. his 1948 theory defined [[information]] as reduction of uncertainty, gave it a unit (the bit), and proved fundamental limits on how much can be transmitted through a noisy channel. before Shannon, epistemology debated what knowledge is. after Shannon, we can measure how much of it flows

[[Condorcet]] proved (1785) that a group of independent agents each slightly better than chance converges on truth exponentially with group size. this is the foundational theorem of collective epistemology — and its failure mode is equally important: when agents are correlated, errors compound rather than cancel

[[Karl Popper]] made falsification the engine of knowledge: a theory is scientific if it can be refuted. [[Thomas Kuhn]] countered that science does not accumulate smoothly but shifts between paradigms — stable frameworks punctuated by revolutions. this makes epistemology historical: what counts as knowledge depends on which paradigm you inhabit

[[Karl Friston]]'s [[free energy principle]] offers a physical epistemology: every living system minimizes surprise by building internal models that predict sensory input. knowledge is the organism's ongoing attempt to not be surprised by reality. this connects epistemology to [[neuroscience]] — the [[brain]] is a prediction engine, and [[perception]] is controlled hallucination corrected by sensory error

## five stances

| stance | core claim | key problem |
|---|---|---|
| foundationalism | knowledge rests on self-evident bases | which bases? how to identify them? |
| coherentism | knowledge is justified by mutual consistency | consistent fictions pass the test |
| pragmatism | knowledge is what works | works for whom, over what timescale? |
| fallibilism | all knowledge is revisable | how to distinguish revision from loss? |
| social epistemology | knowledge is collective | correlated agents produce correlated errors |

## what cyber inherits

[[cyber]] is a literal implementation of collective epistemology. each classical problem maps onto a protocol mechanism:

- definition: [[knowledge]] in the [[cybergraph]] is the sum of all [[cyberlinks]] — signed, timestamped, public. no private belief, no ungrounded claim. knowledge is what neurons publish
- justification: linking costs [[focus]], proportional to staked [[tokens]]. this is [[Michael Spence]]'s costly signaling applied to knowledge claims. cheap talk produces noise; costly links produce structure
- convergence: the [[collective focus theorem]] proves the [[tri-kernel]] converges to a unique fixed point φ*. this is the [[Condorcet]] mechanism made mathematical — independent neurons, each contributing costly signal, converge on a stable distribution. whether it tracks reality is the open question
- falsification: temporal decay erodes old links exponentially. knowledge must be actively sustained. stale claims decay; fresh corrections compound. this is [[Karl Popper]]'s insight built into the protocol — what is not re-confirmed is forgotten
- structure: the [[crystal]] provides categorical structure (21 domains, 6 types, 720 grammar particles) before any content enters the graph. this is the [[Immanuel Kant]] move: without imposed categories, raw data cannot become knowledge. but the crystal tests its categories empirically via ablation, where Kant relied on intuition
- measurement: [[cyberank]] quantifies the importance of every [[particle]] — [[Claude Shannon]]'s information theory applied to a knowledge graph. entropy, distribution, signal-to-noise: all computable on the live graph
- diversity: the [[tri-kernel]] uses three operators ([[diffusion]], [[springs]], [[heat kernel]]) rather than one, providing structural diversity. but operator diversity is distinct from agent diversity — measuring and incentivizing neuron independence remains open

## open problems

- consensus vs truth: a decentralized system provably converges on collective attention. the gap between convergent attention and [[truth]] is where epistemic quality lives. see [[cyber/epistemology]] for the formal threat model
- epistemic diversity: the [[Condorcet]] theorem requires independent agents. correlated [[neurons]] (same training data, same priors) produce correlated errors. no protocol-level mechanism currently measures or incentivizes diversity
- foundation testing: the [[crystal]] claims 21 irreducible domains. ablation testing can verify this formally, but the answer depends on the corpus — and the corpus is the [[cybergraph]], which is still growing
- external anchoring: the cybergraph is self-referential (φ* computed from links created by neurons weighted by φ*). breaking this loop requires external signals — prediction markets, sensor networks, cross-graph proofs. see [[cyber/epistemology]] for analysis

## key figures

[[Plato]], [[Aristotle]], [[Rene Descartes]], [[John Locke]], [[David Hume]], [[Immanuel Kant]], [[Karl Popper]], [[Kurt Goedel]], [[Alan Turing]], [[Claude Shannon]], [[Condorcet]], [[Thomas Kuhn]], [[Karl Friston]]

see [[cyber/epistemology]] for the protocol-level threat model. see [[knowledge theory]] for the two-kinds framework. see [[phenomena]] for why the crystal organizes by phenomena rather than disciplines