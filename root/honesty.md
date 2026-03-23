---
tags: cyber, core
alias: honest, epistemic honesty, honest reporting
crystal-type: property
crystal-domain: cyber
crystal-size: bridge
diffusion: 0.00011233815923477823
springs: 0.0018885334700608333
heat: 0.00131596273891256
focus: 0.000885921668418177
gravity: 0
density: 2.72
---
reporting actual private beliefs, unadjusted for social pressure, predicted popularity, or anticipated reward

in the [[cybergraph]], honesty is expressed through three acts that form one atomic record: creating the [[cyberlink]] (I believe this connection exists), setting the stake (how strongly I believe it), and setting [[valence]] (my honest prediction of where the market will settle)

---

## honesty vs correctness

honesty and correctness are independent properties.

a neuron is honest when it reports what it actually believes, regardless of whether that belief is accurate. a neuron is correct when its belief matches reality. honesty is a property of the reporting; correctness is a property of the belief's relationship to the world.

[[Bayesian Truth Serum]] does not require correctness — it requires honesty. the mechanism extracts private signals even when those signals are wrong, because honest errors are distributed around reality while dishonest reports are biased in self-serving directions. the aggregate of honest-but-imperfect signals converges toward truth faster than any aggregate of strategic-but-precise signals.

this is the key inversion. asking "are you right?" is unanswerable from inside the system. asking "are you reporting what you actually think?" is enforceable through incentive design.

## honesty in the cybergraph has two senses

**protocol honesty**: the [[neuron]] runs the correct software, signs valid transactions, and follows the [[consensus]] rules of [[nox]]. this is what the [[honest majority assumption]] requires — more than half of staked weight does not deviate from the protocol. it is enforceable by cryptographic proof: a stark verifies that the state transition is correct. dishonesty at this level is detectable.

**epistemic honesty**: the [[neuron]] creates [[cyberlinks]] that reflect its actual beliefs — that the source particle relates to the target particle, that the connection deserves the stake it receives, that [[valence]] $v$ accurately encodes its private prediction. this is what [[Bayesian Truth Serum]] targets. it is not directly verifiable — only the outcome (whether the market confirmed the prediction) is observable after the fact.

both are necessary. protocol honesty guarantees the computation runs correctly. epistemic honesty guarantees the computation produces knowledge rather than noise.

## why honesty is rational

[[Bayesian Truth Serum]] proves that epistemic honesty is a Bayes-Nash equilibrium: when a neuron believes other neurons are reporting honestly, honest reporting is the uniquely score-maximizing response.

the logic:
- a neuron that inflates [[valence]] toward what it expects the crowd to say loses its information gain (it is no longer more accurate than the predicted mean — it has predicted itself into the crowd)
- a neuron that sets valence contrarian without genuine private signal loses prediction accuracy (the market does not move where it predicted)
- the only robust strategy is accurate reporting of both first-order belief (link + stake) and meta-belief (valence)

this is why the mechanism is called a "serum" — it does not rely on virtue. it makes honesty the dominant response through score structure alone.

## the compounding of honesty

honesty compounds through [[karma]]. each accurate BTS prediction adds to the neuron's accumulated score. high karma means the network has observed a track record of genuine private signals. that track record enters [[effective adjacency]] as $\kappa(\nu)$ — the trust multiplier that amplifies future contributions from consistently honest neurons.

a neuron that consistently lies accumulates negative karma. its future [[cyberlinks]] carry diminished weight in the [[tri-kernel]], regardless of stake. epistemic dishonesty is therefore economically self-defeating in expectation: the mechanism does not punish dishonesty in a single round (a lie can go undetected once), but it punishes it in expectation across rounds, because the honest strategy dominates the dishonest one in expected score.

## honesty as the foundation of [[syntropy]]

the [[cybergraph]]'s information measure — [[syntropy]] $J(\pi^*) = D_{KL}(\pi^* \| u)$ — is produced entirely by the aggregate of honest epistemic acts. each honest cyberlink is a bit of genuine signal. the tri-kernel converts honest signals into a sharper $\pi^*$. dishonest links move $\pi^*$ toward noise, lowering syntropy.

a maximally honest graph is a maximally syntropy-generating machine. honesty is not a constraint on the system — it is the fuel.

see [[truthful]] for the mechanism design property that makes honesty rational. see [[truth]] for the probabilistic truth signal honesty produces. see [[valence]] for the ternary field where epistemic honesty is expressed. see [[Bayesian Truth Serum]] for the scoring mechanism. see [[karma]] for the long-run record. see [[honest majority assumption]] for the protocol-level complement.