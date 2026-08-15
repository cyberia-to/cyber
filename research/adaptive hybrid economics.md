---
tags: cyber, research, cybernomics, design
alias: adaptive hybrid economics, adaptive hybrid consensus economics, hybrid PoW PoS, hybrid emission, self-calibrating emission
crystal-type: pattern
crystal-domain: cyber
crystal-size: deep
status: draft
---
# adaptive hybrid economics

design article: how cyber splits security spend between compute work and active stake without hard-coding arbitrary percentages. parameters self-calibrate from on-chain observables via control-theoretic feedback (P / PD / PID). applies to [[CYB]] hybrid mint under [[tok]] / [[plumb]]

normative equations for rewards: [[rewards]]. token utility surface: [[cyber/$CYB|$CYB]]. historical long essay + minimal implementation notes lived under cybics game; this page is the cyber design home

## problem

fixed issuance schedules and fixed PoW/PoS splits are bets about the future: security cost, fee volume, capital opportunity cost. the protocol cannot know those a priori. hard-coding them (21M, fixed 50/50, fixed tail emission) freezes ignorance into consensus

objective triad (in tension):

| objective | meaning |
|-----------|---------|
| security | attack cost > attack profit |
| efficiency | do not overpay for security |
| dilution | mint only as much as security requires |

mechanism: sense and adapt — thermostat, not calendar

## objects

| symbol | meaning |
|--------|---------|
| \(M\) | circulating supply |
| \(S \in [0,1]\) | active staked fraction (epistemic lock with \(v \neq 0\); not idle bag) |
| \(E(t)\) | baseline emission rate from schedule (for cyber: power-law head of [[cyber/$CYB\|CYB]] M(t), not a free knob) |
| \(F\) | fees collected in window |
| \(\beta \in [0,1)\) | fee burn fraction |
| \(\alpha \in [0.3, 0.7]\) | allocation curve exponent |
| \(B\) | gross reward budget this window |

## allocation curve

Given staking ratio \(S\):

\[
R_{\mathrm{PoS}} = B \cdot S^{\alpha},\qquad R_{\mathrm{PoW}} = B \cdot (1 - S^{\alpha})
\]

(neutral prior \(\alpha = 0.5\): equal marginal treatment of stake and work. \(\alpha < 0.5\) favors stake at low participation; \(\alpha > 0.5\) pulls toward compute)

why power: maps \([0,1]\to[0,1]\), one parameter, smooth, no kink that governance can fight over

### mapping to cyber mint channels

| share | pays | cyber realization |
|-------|------|-------------------|
| \(R_{\mathrm{PoW}}\) | compute | [[mining]]: prove Δφ* division + [[fold mining\|fold]] |
| \(R_{\mathrm{PoS}}\) | active risk | [[staking]] with valence ≠ 0; passive lock earns rank only |

see [[cyber/$CYB]] utility §mint

## gross vs net

\[
B = \mathrm{floor}\cdot M + (1-\gamma)(1-\beta)F
\]

\[
I_{\mathrm{net}} = \mathrm{floor} - \frac{F\beta}{M}
\]

when fee burn exceeds floor, net inflation is negative: security funded by velocity, supply shrinks. gross rewards to workers can still exceed net mint (fee recycle)

fee path for CYB: universal pay tax \(\tau_{\mathrm{pay}}=1\%\), of which \(\beta\) burns and \(1-\beta\) feeds \(F\) — [[cyber/$CYB]]

## security floor

floor is the one emission component not gated by Δφ* — paid only to work providers (PoW compute + active stake), never to idle capital. derived bound (attack economics sketch):

\[
\mathrm{floor} \ge c_{\mathrm{sec}} \cdot \frac{\mathrm{TVL}}{M} \cdot r_{\mathrm{atk}}
\]

with \(c_{\mathrm{sec}}\) safety margin and \(r_{\mathrm{atk}}\) attacker cost of capital per epoch. floor PID-decays toward zero as fees cover security (healthy coverage + healthy security margin)

## staking equilibrium (sketch)

per-token active stake yield scales as \(B \cdot S^{\alpha-1}/M\). capital enters until yield meets opportunity cost \(r\):

\[
S^* = \min\!\left(1,\ \Big(\frac{B}{r M}\Big)^{\frac{1}{1-\alpha}}\right)
\]

protocol does not target \(S^*\); it emerges. α moves the equilibrium sensitivity

## feedback (PID)

errors from observables only — no price oracle required for the core loop:

| error | definition | drives |
|-------|------------|--------|
| efficiency | \(\eta_{\mathrm{PoW}} - \eta_{\mathrm{PoS}}\) (security per reward unit) | \(\alpha\) |
| fee coverage | \(F/E(t) - 1\) (or \(F/\mathrm{floor}-1\)) | \(\beta\), floor |

updates (conceptual; gains as ops parameters):

\[
\alpha \leftarrow \mathrm{clamp}\big(\alpha + K_{p\alpha} e_{\eta} + K_{d\alpha}\dot e_{\eta},\ 0.3,\ 0.7\big)
\]

\[
\beta \leftarrow \mathrm{clamp}\big(\beta + K_{p\beta} e_{F} + K_{d\beta}\dot e_{F},\ 0,\ 0.9\big)
\]

derivatives via EMA. start P-only; add D if oscillation; full PID if volatility demands it

**not PID-controlled:** the long-horizon supply shape M(t) for CYB (power law, field cap) — that is genesis physics. hybrid control only allocates B and recycles fees inside that envelope

## what this is not

- not a third consensus finality rule — [[foculus]] decides what is final; this decides how the security budget is paid
- not passive staking yield — idle lock does not mint (anti-compounding; see [[rewards]])
- not a second emission schedule for robots — robot cards only redirect creator shares of mints/pays ([[cyber/$CYB]])

## cyber placement

| layer | owns |
|-------|------|
| [[tru]] / [[rewards]] | Δφ*, Shapley, karma, mint eligibility |
| [[foculus]] | finality, settlement lottery timing |
| this design | α, β, floor dynamics; PoW/PoS split of B |
| [[tok]] / [[plumb]] | conservation of mint/burn/pay/lock |

## sources

consolidated from:

- long form motivation + PID thesis (formerly cybics game adaptive hybrid consensus economics)
- minimal implementation notes (sliding window difficulty, on-proof handler) — formerly cybics game adaptive hybrid economics

implementation detail and simulation notebooks stay optional attachments; normative reward equations remain in [[rewards]]

discover all [[concepts]]
