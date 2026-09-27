---
icon: 💵
tags: cyber, cybernomics
alias: tokens, three tokens, token comparison, cyber tokens
crystal-type: entity
crystal-domain: economics
---
# tokens

three coins, three chains, three jobs. [[bootloader/tokens/$BOOT|$BOOT]] runs [[bostrom]], the bootloader: a compact, crystalline graph that boots devices and other collective intelligences. [[bootloader/tokens/$PUSSY|$PUSSY]] runs [[space pussy]], the community chain: expansive, warm, fast. [[$CYB]] runs [[cyber]] itself and arrives last, after the two bootloaders have proven the machine. one token, one chain ([[cyber/research/oikos|oikos]]); every personal book hangs off one of the roots by registration.

the policy of each chain is one identity per epoch, ΔS_E = M_E − B_E, and a rule for its sign. every mint budget is a function of time, of settled Δφ⁺ (after surprise ρ and Shapley) and of what was burned — never of how many neurons or books exist, because those are free to create. operations are the three of [[plumb]]: mint, burn, lock.

## the three

figures marked *proposed* are the 2026-09-27 proposal for the 2026-11-05 launch and are open until the genesis files fix them. halt figures are from the burial at height 25,120,712 ([snapshot.bostrom.network](https://snapshot.bostrom.network)).

| | [[cyber]] | [[bostrom]] | [[space pussy]] |
|---|---|---|---|
| token | &#36;CYB | &#36;BOOT | &#36;PUSSY |
| launch | phase 3 | 2026-11-05 13:22:42 UTC | 2026-11-05 13:37 UTC |
| job | root money | bootloader, crystal | community, expansion |
| genesis supply | 1.87 × 10¹⁷ ¹ | 482.3T ² | 10¹⁸ ² |
| cap | p = 2⁶⁴ − 2³² + 1 | policy | none |
| ΔS_E | ultrasound | up fast, then flat | up for years |
| emission ³ | by time, to φ* | 3× base in year 1 | 9× base in year 1, then halving |
| receives | mining · subsidy · annuity | same; founder passive | same + availability |
| link write | lock | burn b(fill) | lock |
| link life | until unlock | forever | until unlock |
| capacity | ∞ | 2²² links | ∞ |
| stake cap ⁴ | √ | 5 % | √ |
| founder ⁵ | — | endowment | — |
| dormant ⁶ | — | burn | burn → subsidy |
| root fee ⁷ | — | receives, burn | pays, burn |
| registration ⁸ | — | 10 % of the book | 2 % of the book |
| referral ⁹ | r = 10 % | r = 10 % | r = 10 % |
| books register | — | by need | by default |
| parameters | fixed | fixed | vote |

1. every tocyb × 666, ≈ 1 % of the cap; the promise of the bootloader resolved.
2. at halt, bonded uniformly from the snapshot; milliampere and millivolt counted in the bond.
3. *proposed.* bostrom mints 3× the halt base in the first year, half of it in the first quarter, then nothing: it dilutes the founder's ≈ 60 % without touching a balance, because passive stake earns rank and never income (§9), and emission goes only to work (mining §7, subsidy §8, annuity). pussy mints 9× the halt base in the first year, then halves each year to a floor of 3 %/yr: a community chain can afford what the root money cannot. cyber's emission is a function of time alone; who receives it is a function of φ*.
4. *proposed.* weight in rank and consensus per neuron: √-stake on cyber and pussy, a hard cap of 5 % of effective stake on the kernel. owning 60 % never weighs 60 %.
5. *proposed.* the passport contract's 8.97T and the founder's excess above 10 %, locked with v = 0, burned on a ten-year schedule against an equal mint to work: ΔS = 0, the owner changes. for devices booted from the kernel the endowment is the referrer and receives the same r.
6. after K = one year of epochs, unclaimed genesis balances burn on bostrom; on pussy they burn and re-mint into the subsidy budget, to work, never to heads.
7. registering a network or a book in the kernel, and every checkpointed epoch of a child chain, is a burn of &#36;BOOT; pussy pays it, bostrom re-emits nothing for it. the only obligation between the two chains.
8. *proposed.* registering a book in a root costs a share of the book: at birth the book premints its own token &#36;ν and the root takes its cut of that premint, 10 % on the kernel, 2 % on pussy. this is the one place the two roots price a book differently: the kernel wants few, deliberate registrations, the community wants many. the cut is an asset of the root book, the fee stream of §8, and the root's holders read it as income. registering a network in the kernel stays a burn of &#36;BOOT (note 7).
9. the referral is a constant of the book, the same in every root: r = 10 % of the birth premint goes to the referrer, the neuron whose registration cyberlink created the book. detail below.
the referral is a premint, and so is the registration. when a neuron's book is born it mints its own token, &#36;ν, a fixed birth amount G in the neuron's name. two shares leave at once: the root's registration cut (note 8) and the referrer's r = 10 %, the referrer being the neuron whose registration cyberlink created the book. both are ordinary balances: held, transferred, sold, staked back into the book. holding &#36;ν is the right to a pro-rata share of the fees the book collects for as long as it lives, so root and referrer are paid from the book's own economy and in its own token. a book nobody uses collects no fees and its token is worth nothing, which is what keeps both cuts Sybil-neutral: a thousand empty books are a thousand worthless premints. G and r are constants of the book, the registration cut is a constant of the root, and all three are [[plumb]] mint operations at birth, nothing more.

the two chains are bound by one obligation only, the root fee: a child pays the kernel in &#36;BOOT for its name and its finality, and the kernel re-emits nothing for it. everything else is a mirror image: bostrom burns, pussy locks; bostrom destroys the dormant, pussy redistributes it; bostrom's parameters are frozen, pussy's are a vote.

## before: the tokens of the bootloader era

the idea arrived in 2016 as cyberChain. the euler network put pagerank inside consensus on GPUs in 2018; its tokens were test tokens and never money. [[bostrom]] launched on 2021-11-05 at 13:22:42 UTC and ran knowledge-graph consensus for five years, until it halted at height 25,120,712 on 2026-08-05 and was laid to rest as the genesis material of the two chains above. its economy had six tokens.

| token | denom | role | at halt |
|---|---|---|---|
| [[bootloader/tokens/$BOOT|$BOOT]] | `boot` | consensus and governance; 1.09 % inflation; bonding created &#36;H 1:1 | 482,287,925,778,234 |
| [[bootloader/tokens/$H|$H]] | `hydrogen` | liquid staking derivative and the everyday unit; burned to mint &#36;V and &#36;A | 305,452,862,328,021 |
| [[bootloader/tokens/$V|$V]] | `millivolt` | will, bandwidth: a cyberlink burned &#36;V at the dynamic bandwidth price; price doubled every 4B ever minted | 2,182,319,343 |
| [[bootloader/tokens/$A|$A]] | `milliampere` | focus: weighted a neuron's links in the GPU diffusion; never burned by linking; price doubled every 32B | 13,889,231,915 |
| [[bootloader/tokens/$TOCYB|$TOCYB]] | `tocyb` | the promise: hold tocyb, receive &#36;CYB when the network arrives; 30 % entered genesis, 70 % stayed unallocated | 281,405,532,467,645 → × 666 = &#36;CYB genesis |
| [[bostrom/root/lithium|$LI]] | cw-20 | the late gateway token: 1 peta, stepped-decay emission, 1 % burn per transfer, 10 % to referrals | — |

distribution at genesis followed one structure for &#36;BOOT and &#36;TOCYB: [[cybergift]] 70 %, [[cybercongress]] 11.6 %, epizode zero community 8.3 %, senate 5.1 %, great web foundation 5 %. most of the gift stayed parked in multisigs through the five years (the ledger is [[finalization of $BOOT distribution]]: 603T &#36;BOOT and 700T &#36;TOCYB in the gift multisig, 148T claimed through prog), which is why control of the majority of &#36;BOOT ended with the founder, and why the bootloader's first job after rebirth is to dilute that by work.

[[space pussy]] launched in 2022 as the community-led soft3 computer with a total supply of 1 exa &#36;PUSSY: 18 heroes at 0.1 % each, the rest in a community pool meant as gifts to the most active cosmos communities. it mirrored bostrom's structure with [[$CUM]] (liquid fuel), [[$VIP]] (will) and [[$AM]] (focus). it halted with 29,112 links recovered and is reborn as the canary.

the resource tokens do not return. &#36;H, &#36;V, &#36;A and their pussy mirrors were the bootloader's way to price bandwidth and focus with separate denominations; the reborn chains price them with the mechanisms of [[rewards|rewards]]: the ICBS position on a link is the spam cost, the surprise gate ρ is the novelty price, stake on two axes moves rank, and the annuity pays foundational links as the graph grows around them. what the resource tokens held at halt enters the genesis bond, so no holder loses what those balances meant.

## outside

- [[$ETH]] as digital oil and backbone; [[cyberia.capital]] settles citizenship there
- [[$BTC]] as digital gold and pelvis

[[$CYB]] · [[bootloader/tokens/$BOOT|$BOOT]] · [[bootloader/tokens/$PUSSY|$PUSSY]] · [[plumb]] · [[rewards|rewards]] · [[launch]]
