---
tags: cyber, article, draft, research
alias: two kinds of knowledge, structural knowledge, epistemic knowledge, topological knowledge
crystal-type: pattern
crystal-domain: cyber
crystal-size: bridge
authors: mastercyb
---

the [[cybergraph]] contains two kinds of [[knowledge]]. they are irreducible to each other. the system is incomplete without both.

---

## kind one: structural knowledge

a [[cyberlink]] records that two [[particles]] are connected. this is structural [[knowledge]]:

> A relates to B

it is binary. the link either exists or it does not. it is created by one [[neuron]], signed, timestamped, content-addressed. it is permanent once finalized. it answers the question: what is connected to what?

structural [[knowledge]] defines the topology of the [[cybergraph]]. it is the substrate on which everything else runs. the [[tri-kernel]] diffuses over it, [[springs]] constrain it, [[heat kernel]] smooths it. [[cyberank]] flows through it.

but structural [[knowledge]] is silent on one question: is this connection good?

a [[cyberlink]] from spam to spam is structurally identical to a [[cyberlink]] from a foundational theorem to its proof. both are edges. the graph does not distinguish them.

---

## kind two: epistemic knowledge

the [[cyberlink market protocol]] adds a second kind: the collective's belief about whether a connection is true, useful, or meaningful.

this is epistemic [[knowledge]]:

> the network estimates A→B at probability p

it is continuous. price ∈ (0,1). it is not set by one [[neuron]] — it emerges from the aggregate of all market positions. it is dynamic: it updates as [[neurons]] buy TRUE or FALSE. it answers the question: how much does the collective believe this connection?

epistemic [[knowledge]] does not replace structural [[knowledge]]. it evaluates it. the [[cyberlink]] creates the question. the market discovers the answer.

---

## the relationship

| | structural | epistemic |
|---|---|---|
| what | A→B exists | p(A→B is true) |
| who | one [[neuron]] | all market participants |
| how | create [[cyberlink]] | buy TRUE or FALSE |
| form | binary (0/1) | continuous (0,1) |
| permanence | permanent | dynamic |
| question answered | what is connected? | what is worth believing? |

structural [[knowledge]] is the library. epistemic [[knowledge]] is the catalogue of reliability. a library with no reliability signal is noise. a reliability signal with no library has nothing to evaluate.

---

## why both are necessary

a [[cybergraph]] with only structural [[knowledge]] — all [[cyberlinks]] weighted equally — produces [[focus]] proportional to link count and stake. popular links accumulate [[focus]] regardless of truth. the [[tri-kernel]] converges to a fixed point, but that fixed point may be a spam attractor.

a [[cybergraph]] with only epistemic [[knowledge]] — markets with no underlying links — has nothing to trade. the market needs a structural fact to form an opinion about.

the interplay: structural [[knowledge]] creates the edges over which the market discovers probabilities. those probabilities feed back as weights into the [[tri-kernel]], shaping φ*. the [[focus]] distribution is then jointly determined by topology (who linked what) and collective belief (what the network trusts).

this is what [[veritas.computer|veritas]] pursues: truth is not declared. truth is emerging — from the market process, continuously, as a convergent collective signal.

---

## connection to the 2|3 architecture

from [[two three paradox]] and [[binary topology ternary economics]]:

| layer | kind | representation |
|---|---|---|
| binary [2] | structural | [[cyberlink]] exists or not |
| ternary [3] | directional belief | TRUE / UNCERTAIN / FALSE |
| continuous [∞] | epistemic | LMSR price ∈ (0,1) |

structural [[knowledge]] is the binary substrate. epistemic [[knowledge]] is the continuous signal. ternary is the coarse quantization between them — the human-readable summary of the market price.

the three are not alternatives. they are layers. each requires the one below it.

---

## implications for the formal definition

the formal [[cybergraph]] $\mathbb{G} = (P, N, T, L)$ captures both kinds of [[knowledge]] in a single record.

each [[cyberlink]] $\ell = (p, q, \tau, a, v)$ contains:

- structural [[knowledge]]: $(p, q)$ — which connection. authorship ($\nu$) and time ($t$) come from the containing [[signal]]
- epistemic seed: $v \in \{-1, 0, +1\}$ — [[valence]], the neuron's [[Bayesian Truth Serum|BTS]] meta-prediction, predicting how the [[inversely coupled bonding surface|ICBS]] market on this edge will converge

$v$ is not an assertion about truth. it is the meta-prediction input that [[Bayesian Truth Serum]] requires: the neuron's prediction of what the collective will believe. creating a link with $v = -1$ means "I affirm this connection exists and I have private [[knowledge]] the market hasn't priced yet." [[Bayesian Truth Serum]] rewards exactly this when correct.

epistemic [[knowledge]] is the derived layer — the ICBS market price, computed from all positions over time. but the meta-prediction seed $v$ that feeds into [[Bayesian Truth Serum]] scoring IS in the record, because the [[cyberlink]] is the BTS input: link creation is the first-order belief, $v$ is the meta-prediction $m_i$.

see [[cyberlink market protocol]] for the market design. see [[focus flow computation]] for how market weights enter the [[tri-kernel]]. see [[market inhibition]] for why epistemic [[knowledge]] is what makes the [[cybergraph]] computationally equivalent to a neural network with both excitation and inhibition.