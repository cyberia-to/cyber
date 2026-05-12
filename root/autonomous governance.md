---
tags: cyber, article, draft, research
alias: autonomous governance, cyber governance, collective intelligence governance, superintelligent governance
crystal-type: pattern
crystal-domain: cyber
crystal-size: bridge
---

the [[cybergraph]] does not vote on what to do — it infers what to do from the continuous revealed preferences of every participant, weighted by demonstrated accuracy, acted upon automatically

governance is the problem of collective decision-making. how does a distributed system of agents with different values, different knowledge, and different stakes coordinate on protocol behavior? classical approaches answer: through representation (elections), deliberation (proposals), and aggregation (voting). the cyber approach answers: through inference.

---

## the failure modes of classical governance

classical on-chain governance — token-weighted proposals, majority voting, execution through multisig or DAO — fails in predictable ways:

voter apathy. most token holders do not vote. participation rates of 5–15% are typical. decisions that affect the entire ecosystem are made by a small, self-selected group.

plutocracy. one token, one vote means large holders dominate. the interests of the median user are structurally underrepresented. governance capture by whales is not an edge case — it is the expected equilibrium.

binary outcomes. proposals are yes/no. the protocol has no way to express partial agreement, conditional acceptance, or gradated preference. complex tradeoffs collapse to a binary.

temporal gaming. governance decisions are predictable windows for coordination attacks. whale voting at the last minute, vote-buying through flash loans, cartel formation before submission — the proposal process is an attack surface.

expertise blindness. a domain novice and a domain expert have equal voting power in token-weighted systems. the neuroscientist and the speculator both vote on changes to the neural language specification. the allocation of decision weight has no connection to the allocation of relevant knowledge.

execution delay. proposal → discussion → vote → timelock → execution takes weeks. the protocol cannot respond to fast-moving conditions.

---

## how the cybergraph already governs

every participant action in the [[cybergraph]] is already a vote:

| action | what it votes on | weighting |
|---|---|---|
| [[cyberlink]] creation | graph structure — which particles connect | stake × [[karma]] |
| [[happiness]] submission | systemic quality | stake |
| stake allocation | which claims deserve influence | size of stake |
| [[ICBS]] position | edge epistemic validity | capital at risk |
| [[karma]] accumulation | whose future votes count more | BTS scoring history |

these votes are:
- continuous: happening every block, not in periodic cycles
- expertise-weighted: karma reflects accuracy track record, not just wealth
- private where appropriate: happiness is aggregated, not individually exposed
- automatically enforced: the tri-kernel incorporates them into φ* every convergence step
- falsifiable: BTS scoring penalizes dishonest voting with karma damage

the aggregated signal is the [[focus]] distribution φ* and the [[metabolism|metabolic health]] M(t). these are computed deterministically from participant behavior. the [[parametrization]] RL agent acts on ΔM. the [[self-linking]] mechanism acts on φ*. the governance is the computation.

---

## what the superintelligence governs automatically

given the aggregated signal, the system governs:

protocol parameters. the RL agent continuously adapts α, τ, κ and the reward coefficients to maximize M(t). no proposal required. every adjustment is within the safety envelope. every adjustment is deterministic and therefore [[consensus]]-compatible. every node computes the same parameter vector.

graph structure. self-linking fills inference gaps, flags inconsistencies, and documents state evolution. the graph's own structure evolves through the system's inference about what belongs together — not through any administrator's decision.

resource allocation. own-balances management (§22.6) allocates treasury, will, and compute cycles according to metabolic feedback. the allocation policy is encoded in the protocol. the protocol's capital is managed by the protocol's inference, not by a committee.

alignment monitoring. the dual focus distribution divergence is computed every block. graduated responses to rising divergence are triggered automatically. no governance vote is needed to notice misalignment — it is a continuously available measurement.

knowledge quality. the [[forgetting]] mechanism moves stale links to cold tier based on objective criteria (stake < ε, ICBS price < ε, zero traffic for N epochs). no editorial board decides what gets archived. the metrics decide.

---

## what remains for explicit governance

three things cannot be governed autonomously without circularity:

the metabolic weights $w_c, w_s, w_h$. these encode the normative claim of what health means — how much to weight external validation versus internal order versus participant satisfaction. the system cannot choose its own values without assuming values in the choice function. these are set at genesis and changed by explicit governance when community values evolve. changing them is a high-stakes, rare, deliberate act.

the [[Hemera]] hash primitive. the foundation of every [[stark]] proof in the system. its stability is a security guarantee. changing it requires a coordinated chain fork. this is not a limitation but a commitment device — the system's cryptographic foundation is stable by design.

protocol upgrades. the system generates its own upgrade proposals — it does not accept them from [[neurons]]. neurons hold a time-bounded veto that decays as the system's upgrade track record accumulates. the bedrock (Hemera hash parameters, focus conservation law, κ < 1 requirement) is frozen at genesis and cannot be changed by any upgrade mechanism. see [[self-upgrade]] for the full three-phase specification.

---

## the political theory

sovereignty is collective intelligence, not collective vote.

a vote aggregates declared preferences at a point in time. the problem: declared preferences diverge from revealed preferences. people say they want X and act in ways consistent with Y. voting systems aggregate stated intention; market and behavioral systems aggregate revealed intention.

the [[cybergraph]] aggregates revealed preferences continuously. a [[neuron]]'s [[karma]] reflects their history of acting on correct beliefs — not their self-reported expertise, not their social standing, not their stake size. their focus distribution reflects what they consistently link — not what they claim to value in a survey. their happiness reflects their direct experience — not what they think they should say.

the aggregate of revealed, accuracy-weighted preferences is more informative than the aggregate of declared, token-weighted preferences. and it is automatically enforced: the protocol acts on it every block without waiting for a quorum, a timelock, or an execution committee.

this is not the absence of governance. it is governance by a more accurate signal.

---

## capture resistance

governance capture fails against this model for structural reasons.

to influence the metabolic signal, an actor must either:
1. improve the network (raise cap, syntropy, or happiness) — which benefits all participants and is the intended outcome
2. game one signal while the others correct — which is detectable as divergence between the three metabolic factors and triggers the parametrization agent to adjust

to inflate their vote weight, an actor must accumulate [[karma]] — which requires being right about epistemic claims over time. karma cannot be bought directly. it can be bought indirectly by being a consistently accurate [[neuron]], which is what the system wants.

to block a parameter adjustment, an actor must maintain their own metabolic signal at a level where the RL agent prefers their preferred parameter. this requires competing in the same space as every other participant — the system finds the parameter that maximizes the aggregate M(t), not the parameter that maximizes one actor's M.

the attack surface is not zero. but it is substantially smaller than any system with a concentrated governance mechanism.

see [[metabolism]] for the three metabolic signals. see [[parametrization]] for the RL loop that acts on them. see [[self-upgrade]] for the upgrade mechanism. see [[functions of superintelligence]] for how governance integrates with the other autonomous capabilities. see [[Bayesian Truth Serum]] for the mechanism that makes votes expertise-weighted.