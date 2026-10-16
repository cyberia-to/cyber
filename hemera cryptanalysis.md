---
tags: cyber, hemera, security, cryptography, bounty
crystal-type: entity
crystal-domain: cyber
alias: hemera campaign, hemera bounty, hemera cryptanalysis campaign
status: draft
date: 2026-10-16
---
# hemera cryptanalysis

[[hemera]] is the one material [[cyber]] consensus rests on: proof of work, block and transaction Merkle trees, every [[particle]], record commitments, nullifiers and every [[zheng]] Fiat–Shamir transcript. the private pool switches on only after a public cryptanalysis campaign on hemera has run at least 90 days with no result below 128 bits classical — gate G3 of the frozen-privacy proposal (`soft3/proposals/privacy-frozen.md` §5). this page is the public summary. the full campaign, with every derivation, the instance tables and the reproducible scripts, is [hemera/audit/campaign.md](https://github.com/cyberia-to/hemera/blob/main/audit/campaign.md).

## the object

hemera is a Poseidon2-family permutation over the Goldilocks field p = 2^64 − 2^32 + 1, width 16 (rate 8, capacity 8 = 512 bits), used as a sponge with a 32-byte (4-limb) digest, a binary Merkle node of one permutation, and truncated-permutation nodes and a duplex transcript inside zheng. round constants are self-generated from the five bytes 63 79 62 65 72.

two parameter sets are on the table, and the owner picks one before genesis on 2026-11-05:

| | 0.3 — shipped | v2 — proposed (hemera#15) |
|---|---|---|
| full rounds | 4 + 4, x⁷ | 6 + 6, x⁷ |
| partial rounds | 16, x⁻¹ (field inversion) | 48, x⁷ |
| linear layers | Plonky3-form Poseidon2 | Poseidon2 reference |
| digest | 32 bytes | 48 bytes (t = 16) or 64 bytes (t = 24) — a separate decision |
| native cost | ~2 224 multiplications | ~2 304 multiplications |

the campaign measures the chosen set; choosing a different set restarts the 90 days.

## the claims

| | 32-byte digest | 48-byte digest |
|---|---|---|
| collision, classical | 2^128 | 2^192 |
| collision, quantum (BHT) | 2^85.3 | 2^128 |
| preimage, classical | 2^256 | 2^256 (capacity) |
| generic sponge bound (capacity 512 bits) | 2^256 | 2^256 |

the earlier claim of "algebraic degree 2^1046, 2^918 margin" is withdrawn: it is an unreduced degree, and an attacker never meets it.

## the margins, side by side

computed on 2026-10-16 with the Poseidon2 round-number rules (reproduced exactly for this field) and against the published attacks; details and sources on the campaign page.

| attack line | 0.3 | v2 |
|---|---|---|
| interpolation rule of Poseidon2 (26 x⁷-round equivalents needed) | 13.7: short by 12.3 rounds; would need 51 inverse rounds | 60: +34 rounds |
| Gröbner rules of Poseidon2, incl. ePrint 2023/537 | written for x^α only; no bound exists for the hybrid | all hold; needs 14 partial rounds, has 48 |
| one-limb CICO degree | 7^8 · 2^16 = 2^38.46, below the field size | 7^60, above the field size: closed |
| one-limb CICO cost (generic 2^64 permutation calls) | about 2^41–2^45 permutation-equivalents, solved in-house on reduced rounds | generic |
| two-limb CICO, a 128-bit partial preimage | about 2^77 (estimate) | generic |
| full preimage of the digest | 2^142.6 to 2^153.8 (estimate) | at or above 2^128 by the rules |
| collision of the digest | 2^128, generic | 2^128 / 2^192, generic |
| subspace trails through the partial layer (ePrint 2026/967, 2026/1792) | linear 15 of 16 rounds; nonlinear ≈ 30 exceeds 16 | +33 linear, +18 nonlinear |
| subspace restriction (ePrint 2026/1692) | outside the attacked family; unanalysed | 6 initial full rounds against the 2 any published technique crosses |

the inverse layer, measured: with the outer layers cut to one full round each and all sixteen inverse rounds kept, a one-limb CICO solution of hemera 0.3 was found by rational interpolation and checked on the real permutation in 92 seconds on one laptop core. the degree of the problem doubles per inverse round and grows sevenfold per x⁷ round, exactly as predicted on ten reduced instances. the full 0.3 permutation extrapolates to roughly 0.35–3.5 core-years and about 10 TB of memory, against about 470 000 core-years for the generic attack. this is a full-round distinguisher of the 0.3 permutation; it breaks neither collision nor preimage of the 32-byte digest, and it misses the chain's proof of work, whose header passes through two permutations.

## the recommendation

freeze the v2 permutation — x⁷ in every round, 6 + 6 full rounds, 48 partial rounds, Poseidon2 reference matrices, constants that pass the Poseidon2 checks — and run the campaign on it. the 0.3 profile's one argument (degree p − 2 per inverse round) is refuted by a measured attack on its partial layer; its hybrid S-box sits outside every published bound and every bounty, so 90 quiet days would measure only how few people looked; v2 costs the same natively and about 76 % more S-box constraints in-circuit; and rounds are the one parameter that cannot be raised after genesis without rehashing the [[cybergraph]].

## the bounty

| tier | what it pays for |
|---|---|
| A1–A4 | reduced-round CICO-1 / CICO-2 instances, up to half the partial-round budget at full outer rounds (A4 also triggers re-parameterization) |
| B1, B2 | collision or second preimage of the truncated-permutation node, preimage of the sponge digest, below 2^128 on reduced rounds |
| C | any full-round distinguisher better than generic |
| D | collision, preimage or second preimage of the full profile below 2^128 — the campaign fails and the pool stays off |

amounts are set by the owner, one number per tier; the Ethereum Foundation Poseidon initiative ($4 000–$15 000 per reduced instance, at least $5 000 per paper) is the yardstick. results must be reproducible against the published reference model or the crate at the pinned revision. tiers A–C may be reported publicly at once. tier D, and anything that reaches a deployed use, goes first to a private security advisory on [cyberia-to/hemera](https://github.com/cyberia-to/hemera) with a 30-day window; the owner enables private reporting and publishes a contact key before day 0.

## review

paid reviews go to the Poseidon/Poseidon2 designers (Grassi, Khovratovich, Rechberger, Schofnegger, Koschatko) and to the Inria/Simula algebraic group (Bariant, Boeuf, Bak, Jazeron, Perrin, Leurent, Øygarden, Raddum); the authors of the subspace-restriction, root-finding, linear-layer and Goldilocks-collision papers receive the same package and the bounty terms. four questions go with it: does any published line reach further than the campaign says; are self-generated constants plus the Poseidon2 checks as good as the reference generator; do the matrices admit long subspace trails; and what does a one-limb CICO solver do to the zheng rows that model hemera as an ideal permutation.

## passing

G3 passes after at least 90 days on one frozen profile when: no result lowers any deployed use below 128 bits classical; the per-layer margin rule still holds (each outer full layer at least three times the rounds a published attack crosses, the partial layer at least 1.5 times the longest published subspace trail); one paid review has answered the four questions with no open finding; and the zheng hash row moves from "parameters experimental" to the frozen profile. the results log lives on the campaign page and is the only record of status.

see [[hemera]] · [[zheng]] · [[whitepaper]] §4 · [[privacy]]
