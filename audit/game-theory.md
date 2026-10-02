---
tags: cyber, audit, game theory, foundations
crystal-type: report
crystal-domain: cyber
alias: game-theoretic foundations audit, game theory audit
status: draft
date: 2026-10-02
---

# game-theoretic foundations audit

a sweep of every design repo under ~/cyber (cyber, tru, foculus, cybergraph, mudra, cyberia, crystal, cybics) for the theorems the economy actually rests on. the question asked: is it true that three ideas — Perron–Frobenius, Shapley, Prelec's Bayesian Truth Serum — are the foundational game-theoretic ideas of the whole design? what was missed, and what must still be applied?

## 1. the simple picture

the economy has to answer three questions, in order. each is answered by one theorem.

| question | answer | theorem | source |
|---|---|---|---|
| what is the state of the graph? | φ* — one ranking every node computes identically | the map converges to a unique fixed point | Banach (general) + Perron–Frobenius (positive case) — `tru/specs/tri-kernel.md:93–101,155` |
| who caused the state to improve? | split the improvement fairly among contributors | Shapley value | `tru/specs/rewards.md §3–4` |
| are people telling the truth? | score reports so honesty pays | Bayesian Truth Serum + Surprisingly Popular | `tru/specs/truth-scoring.md`, `tru/specs/strong-truthfulness.md:65–79` |

verdict: the triad is right in spirit — **state → credit → honesty** — but it is three borrowed theorems; the fourth pillar, cyber's own, is the work/stake hybrid that pays for all three (§3.1). two label corrections on the borrowed three:

- the convergence theorem is **Banach contraction**, not Perron–Frobenius. PF covers only the diffusion-only special case; `cybics/math/convergence.md:231–235` already names Banach "the mathematical guarantee" and PF "the positivity guarantee". PF is linear algebra, not game theory — it has no players in it.
- the honesty theorem that carries the weight is **Surprisingly Popular (Prelec–Seung–McCoy 2017)**, not BTS 2004 alone. cyber has no judge who resolves markets; `strong-truthfulness.md` proves that in a perpetual market BTS alone cannot pick the truth out of a self-fulfilling consensus, and SP can.

no repo file states this triad. the docs carry other triads — `whitepaper.md:1651` (Gödel-escape / conservation / provability), `research/physical analogies.md:41` (springs / heat / Shapley as identities), `cybics/math/convergence.md` (five pillars). this page is the first place the state→credit→honesty triad is written down.

## 2. the one real finding

**each theorem assumes the thing the previous theorem failed to prove.**

- Shapley is fair only "among honest, distinct contributors" (`rewards.md §5`, first sentence) — honesty is what the *next* theorem is supposed to deliver.
- BTS proves only that *one neuron alone* cannot profit by lying (`rewards.md:335`: "incentive-compatible only against unilateral deviation") — a coordinated group can.
- Perron–Frobenius / Banach take the graph as given — the graph is what the rewards produce.

all three theorems are true; the spec says the combination is "assumed, not proven" (`rewards.md:349`). every open problem in the repos — collusion (`rewards.md:335`), withholding (`:337`), discovery leak (`:339`), self-dealing (`:347`), false consensus (`cyber/epistemology.md §6.4`) — sits in the gap *between* two theorems, never inside one. `physical analogies.md:210` says it in its own words: "the substrate is settled and the incentives are not."

**the inverted ranking.** ranked by how often the docs cite them: BTS > Shapley > PF. ranked by what breaks if removed: Banach > Shapley > SP. ranked by how shaky the hypothesis is: BTS > Shapley > PF. the most-cited idea is the least cleanly transferred — Prelec's proof assumes a common prior, a large population, and probability-distribution reports; cyber's version (stake + ternary valence against an ICBS market, `whitepaper.md:953–957`) does not obviously meet those conditions, yet "Prelec proved" is cited without the caveat. Shapley's value function v is neither sub- nor supermodular (`rewards.md:72`), so the only universal guarantee is conservation.

## 3. what is in the design but not in the triad

### 3.1 the work/stake hybrid — the fourth pillar, and the original one

the triad is borrowed theory. the one mechanism that is cyber's own is the way work and stake are balanced, and the first pass of this audit missed it by filing it under "PID control" (`tru/specs/rewards.md §8–10`, `cyber/specs/adaptive hybrid economics.md`).

every earlier hybrid (Peercoin, Decred, …) bolted PoS onto PoW as two *voting* mechanisms fighting over the same block. here work and stake are not two consensus rules; they are two *kinds of contribution* paid from one budget, and each one fixes the other's known failure:

| | work (PoW) | stake (PoS) |
|---|---|---|
| what it is | capital-free: a hash attempt *is* a Shapley sample, so the proof-of-work is the accounting (`rewards.md:179`); no synthetic puzzle | capital at risk on truth: only active stake (valence ≠ 0) earns; passive stake moves rank, never income (`rewards.md §9`, "two axes") |
| fixes | PoS's closed door — the stakeless onramp (`rewards.md:217`): anyone with a phone enters without owning tokens | PoW's waste — stake amplifies real Δφ* instead of burning energy on nothing (`rewards.md:227`, "the amplifier") |
| split | $R_{\text{PoW}} = B(1-\theta^\alpha)$, $R_{\text{PoS}} = B\,\theta^\alpha$, with $\theta$ the active staking ratio and $\alpha$ self-calibrated from observables — a thermostat, not a calendar (`adaptive hybrid economics.md`) | |

three properties make it a mechanism rather than a parameter:

1. **idle capital cannot compound.** emission goes to work and risk only; a yield on passive stake "would be emission without contribution" (`rewards.md:260`). this is the exact structural fix for the proved wealth-concentration failure of proof-of-stake (Fanti et al. 2019, *Compounding of Wealth in Proof-of-Stake*) — not cited anywhere in the repos, and it should be: it is the theorem this design answers.
2. **the split is endogenous.** the staking ratio is not targeted; it emerges from $S^* = \min(1, (B/rM)^{1/(1-\alpha)})$ where capital enters until yield meets opportunity cost (`adaptive hybrid economics.md` §staking equilibrium). the protocol only moves $\alpha$ by feedback on security-per-reward-unit.
3. **useful work is a property of the settlement rule, not the arithmetic** (`warriors/mining.md:51`). the subsidy and the Shapley lottery share one preimage, so securing the chain and computing who gets paid are one act — the "PoUW-utility isomorphism" (`whitepaper.md:964–978`).

where it is open: the PID section is marked "**not agree**" by the author (`adaptive hybrid economics.md` §FEEDBACK); total issuance has no combined cap across mint budget and security budget $B$ (`rewards.md:351`); the composite equilibrium mint + subsidy + fee + yield is "assumed, not proven" (`rewards.md:349`). the game-theoretic question the hybrid still owes is the one every two-factor economy owes: is the $(\alpha, \theta)$ equilibrium stable, and can a miner-staker holding both factors tilt $\alpha$ by manufacturing the efficiency signal it is tuned on.

### 3.2 four more mechanisms that carry weight

1. **costly signalling** (Spence/Zahavi) — every link costs stake, so spam is expensive. `crystal/costly signal.md:8` calls it "the economic foundation of knowledge in cyber". limit, admitted at `epistemology.md §3`: cost stops volume, not lies.
2. **the ICBS market** (Williams–Buterin bonding surface) — the per-link YES/NO market. `nomics.md:26` calls epistemic markets "the conceptual heart of nomics". it provides liquidity and commitment; it is *not* a truth-scorer — a 50% belief reads as 36.6% (`strong-truthfulness.md:29–37`).
3. **honest majority by stake** — every truthfulness result falls back on "> ½ of stake is honest" (`strong-truthfulness.md:79,142`, `foculus/specs/protocol.md:107`). the proof-of-stake assumption, renamed.
4. **locality** — rewards computable on a phone from a bounded neighborhood (`tri-kernel.md:103–107`). `whitepaper.md:161`: "the hard constraint that shapes the entire architecture".

also present: Hoeffding for settlement precision (`foculus/specs/fold-mining.md:64`); Condorcet + Hong–Page, invoked in `egregore` then rejected under correlated agents in `epistemology.md §6.3`.

## 4. what is missing and must be applied

each is a known result that names one of the design's open problems. none appear by name in any design repo (grep over cyber, tru, foculus, cybergraph, mudra, cyberia, crystal, cybics: zero hits).

| # | missing idea | in plain terms | the open problem it explains | what to do |
|---|---|---|---|---|
| 1 | **sybil-proof reputation** — Cheng & Friedman 2005 | a PageRank-style score is sybil-proof only if the random walk restarts from a *trusted* set, not uniformly | the sybil argument rests on linear stake (`rewards.md:90`, `epistemology.md §2`); `cyber/tokens.md:45` proposes √-stake, which breaks it — splitting into N identities gains √N | choose the teleport vector consciously: uniform or trust-seeded. drop or defend √-stake. |
| 2 | **repeated games / folk theorem** + collusion-resistant peer prediction (Dasgupta–Ghosh correlated agreement, multi-task peer prediction, Kong–Schoenebeck) | when the same players meet forever, cartels are stable equilibria, not glitches | "collusion remains open" (`rewards.md:335`). `strong-truthfulness.md:153` cites "the babbling lemma and the Correlated Agreement reduction" in truth-scoring — neither exists there | write the correlated-agreement reduction. it is the fourth pillar the spec already names and does not contain. |
| 3 | **Grossman–Stiglitz 1980** (+ Kyle 1985) | if the price already contains all information, nobody is paid to discover it; a fully efficient market is impossible | the discovery leak (`rewards.md:121,296`, `physical analogies.md §3.2`): novel links score low exactly when they are most valuable | this is a theorem, not a bug. build the explicit discovery premium (the "retroactive bonus, unbuilt"). Kyle's λ gives the rate at which private information enters price — the η_H and γ(f) the Regime-B bound leaves as "environment". |
| 4 | **Goodhart's law / Lucas critique** | a measure that becomes a target stops being a good measure | φ* is the ranking, the price, and the reward target at once — rank–price circular reinforcement (`tru/docs/terms/market.md:169–176`), self-referential false consensus (`epistemology.md §6.4`) | keep "what we rank" and "what we pay" as separate numbers — already half done (rank reads the raw graph, mint reads the ρ-weighted one, `rewards.md §3`). state it as a rule. make external anchoring (`epistemology.md §6.4`) core, not optional. |
| 5 | **cumulative advantage** — Price 1976, Barabási–Albert | paying by rank on a PageRank graph concentrates without limit | "no homeostasis, popular particles accumulate focus without limit" (`tru/roadmap/focus-dynamics.md:64`) | move homeostasis from roadmap into the core spec. |
| 6 | **the core** — Bondareva–Shapley | Shapley is fair but does not guarantee a group cannot earn more by *leaving* | self-dealing on a private subgraph (`rewards.md:347`); the oikos / household-chain design | check whether the Shapley allocation is stable against secession. v is neither sub- nor supermodular, so the core may be empty. unknown today. |

lower priority: explore/exploit (Gittins, UCB) as the principled way to price novelty; social choice (Arrow, Lalley–Weyl quadratic voting) to make the linear-vs-√ stake decision deliberate rather than accidental; Ostrom for governance of the self-neuron (present only in `cybics/game/commons.md`). the entropy account (second law, Landauer) is already named open at `physical analogies.md §3.3`.

## 5. drift found on the way

- "honesty is the **dominant strategy**" — `tru/docs/explanation/overview.md:53`, `foculus/docs/explanation/overview.md:23`, `mudra/specs/place.md:97`. every proof in the repos is Bayes–Nash. downgrade the wording.
- `cybergraph/specs/cybergraph.md:124` lists honest-linking Nash as *open*; tru docs present it as settled.
- karma has four definitions: sum of cyberank (`cyber/karma.md:8`), accumulated BTS score (`whitepaper.md:838`), accumulated syntropy (`research/algorithmic essence of superintelligence.md:190`), entropy production (`physical analogies.md:38`). karma multiplies A^eff, so this is a consensus-weight ambiguity.
- `tru/specs/impulse.md:27` still mints ∝ ‖Δφ*‖, which `rewards.md:41–44` rejects. same stale formula at `tru/docs/explanation/incentives.md:23`, `knowledge-economy.md:45`.
- `tru/docs/terms/market.md:26,33` still says LMSR and price ∈ (0,1); ICBS prices range over [0, λ].
- `cyber/tokens.md:45` √-stake vs linear-stake sybil proofs — see §4 row 1.

## where it landed

the substance of this audit is in the papers, where its readers are; this page is the evidence trail.

- [[litepaper]] § four questions, one computation — the spine (state → credit → honesty → work-and-stake), the one-computation argument, the seam, and the named open problems.
- [[whitepaper]] §2 the idea — the four-question table, one computation, the substrate; §13.2 — the work/stake budget as a mechanism (idle capital cannot compound, endogenous split, stakeless door); §14.2–14.5 — rewritten to the [[rewards]] canon (directed impulse Δφ⁺, surprise-weighted Shapley, settlement mining, two axes, surprisingly-popular selection); §14.8 — the open-problem table with each problem's known result and implied fix; §24 — the conclusion restated as one claim with four parts. references 31–38 added.
- the whitepaper also recovered 33 sentences whose leading `[[subject]]` was stripped in commit 0a24fc9f, and lost its stale reward formulas (‖Δφ*‖ mint, the αΔφ* + βΔJ + γDAGWeight hybrid, the αΔF + (1−α)Ŝ fallback).
- still open outside the papers: the drift list in §5 (tru/foculus/mudra wording, the four karma definitions, `impulse.md:27`, `market.md` LMSR leftovers, √-stake in `tokens.md`).

## method

three parallel read-only sweeps (whitepaper + cyber research; tru/foculus/cybergraph/mudra specs; cyberia/crystal/cybics), then grep for candidate missing foundations across all design repos. dangling references in `strong-truthfulness.md:153` verified by hand. report date 2026-10-02.

see [[theoretical foundations]] · [[rewards]] · [[strong-truthfulness]] · [[epistemology]] · [[physical analogies]]
