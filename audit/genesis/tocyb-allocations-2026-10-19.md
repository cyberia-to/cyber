---
title: TOCYB allocations and the CYB genesis, 2026-10-19
tags: cyber, audit, genesis
---

# TOCYB allocations and the CYB genesis — 2026-10-19

Where every TOCYB of the bostrom genesis went between 2021 and the halt,
what each allocation becomes at the cyber genesis (1 TOCYB unit = 666 cy,
10¹² cy = 1 CYB, [[tokens]], [[$CYB]]), and what sits across IBC.

Inputs: go-cyber `networks/bostrom/genesis.json` (the 2021 allocation); the
burial snapshot in `~/.cache/cyber/snapshots/bostrom` and `…/pussy` (files of
2026-10-15: `holdings.jsonl`, `balances.csv`, `supply.json`,
`state_export.json.gz`, `passports_by_owner.json`); the bostrom archive
`lcd.bostrom.cybernode.ai` for transaction history; Osmosis live state from
`lcd.osmosis.zone` on 2026-10-19 (bostrom is halted, so the Osmosis side of
channel-95 is frozen except for in-flight packets). Names come from passports,
module and contract labels from the state export. Amounts are raw chain units.

## the allocations, genesis → halt → CYB

| allocation | account | TOCYB at the 2021 genesis | share | burned in v6 (2025-09-26) | moved | TOCYB at the halt | CYB at the cyber genesis | at the cyber genesis |
|---|---|---|---|---|---|---|---|---|
| cybergift | multisig 2/3 bostrom1qs9w7ry45axf…5p6e | 700,000,000,000,000 | 70.0 % | 603,000,000,000,000 | — | 97,000,000,000,000 | 64,602.00 | dormant (multisig) |
| cybercongress | multisig bostrom1xszmhkfjs3s0…e4pw | 115,880,796,057,845 | 11.6 % | 115,594,467,532,355 | 286,328,525,490 out before v6 | 0 | 0 | — |
| epizode zero community | 555 genesis accounts | 83,392,706,320,830 | 8.3 % | — | 3,453,802,940,173 out over five years | 79,938,903,380,657 (471 accounts) | 53,239.31 | live: key known or kind-3 claim |
| senate | community pool, distribution module bostrom1jv65s3grqf6v…s0hc | 50,726,497,621,325 | 5.1 % | — | none (untouched since genesis) | 50,726,497,621,325 | 33,783.85 | dormant (module) |
| great web foundation | multisig 2/3 bostrom1ha4pw9w2qgc2…qwev | 50,000,000,000,000 | 5.0 % | — | all 50 T to the congress key, 2025-09-16 | 0 | 0 | — |
| congress key (receives the GWF) | bostrom1mqn6z7tpyy3g…s3a | 0 | — | — | +50 T from the GWF | 50,000,000,000,000 | 33,300.00 | live (single key) |
| accounts outside the genesis | 951 accounts | 0 | — | — | received over five years | 3,638,292,724,476 | 2,423.10 | live: key known or kind-3 claim |
| IBC escrow (lives on Osmosis and others) | 13 channel escrows | 0 | — | — | sent across IBC | 101,838,741,187 | 67.82 | dormant unless the genesis credits the Osmosis holders |
| total | | 1,000,000,000,000,000 | 100 % | 718,594,467,532,355 | | 281,405,532,467,645 | 187,416.08 | |

Conservation: 1,000 T − 603 T (gift) − 115,594,467,532,355 (congress
multisig) = 281,405,532,467,645, the snapshot supply to the unit; the flows
out of the epizode zero accounts and the congress multisig (3.74 T) equal
what the other accounts and the escrows hold.

By status at the cyber genesis: live 88,962.41 CYB (47.5 %), dormant
98,453.67 CYB (52.5 %: the gift multisig, the community pool and the
escrows). The dormant share is what a genesis rule can still direct.

## top 20 holders at the halt

At the halt: supply 281,405,532,467,645 over 1,434 holders; the top 20 hold 93.6 %. CYB from the exact balances.

| # | name | address | TOCYB | ≈ CYB | share | treatment at the cyber genesis (soft3/specs/genesis.md) |
|---|---|---|---|---|---|---|
| 1 | cybergift multisig (2 of 3) | bostrom1qs9w7ry45axfxjgxa4jmuhjthzfvj78sxh5p6e | 97.0 T | 64,602 | 34.5 % | `legacy_owner`, dormant (multisig) |
| 2 | community pool (distribution module; the senate) | bostrom1jv65s3grqf6v6jl3dp4t6c9t9rk99cd8yvs0hc | 50.7 T | 33,784 | 18.0 % | `legacy_owner`, dormant (module) |
| 3 | congress | bostrom1mqn6z7tpyy3glwqfsmg9lly87vjs8dgkt3zs3a | 50.0 T | 33,300 | 17.8 % | key known |
| 4 | founder | bostrom16230lc9r27vrxlzd4wlvcz2l66quzj84vlkxc5 | 10.4 T | 6,907 | 3.7 % | key known |
| 5 | cyberhead, wndrfl | bostrom1gxhs5wzam5rlgf6cs0lkme400ut64d4ncpfs2h | 10.0 T | 6,660 | 3.6 % | key known |
| 6 | master | bostrom1qn8sr2hzmktlecusdtxj9hwj0upnm0jft9snar | 9.2 T | 6,158 | 3.3 % | key known |
| 7 | rockets | bostrom1s4czxghmh29aw2ldynk8r9lnkfccw5ph8rjpxa | 4.4 T | 2,914 | 1.6 % | key known |
| 8 | superhuman | bostrom1pntx8ql2v7cqxu05etg8c4v0r2vz7qnq9uqmpy | 4.0 T | 2,666 | 1.4 % | key known |
| 9 | cyberdeep | bostrom1ttd4h585vgy29gj0xe0p338drvw9w4m4hzwj9p | 4.0 T | 2,664 | 1.4 % | key known |
| 10 | soul | bostrom1qwzsnnuk2u6avvh059c9p50gqqquvcu5svvwrs | 4.0 T | 2,664 | 1.4 % | key known |
| 11 | zlobabah | bostrom1m8yf3g2m9p7danjvt4ga6vzewy380pk52kcu9z | 4.0 T | 2,664 | 1.4 % | key known |
| 12 | sigma | bostrom1g07gj9ph0r2hjx6ffqn8vl38u4agd5mpqx9m7f | 4.0 T | 2,664 | 1.4 % | key known |
| 13 | encom | bostrom1t3222ju7ssu58xsm3m2xw445p94cqn2qrex3vy | 2.0 T | 1,332 | 0.7 % | key known |
| 14 | andromeda | bostrom1edlrze5f48jjaw0wxcve4asqqf2tcjdl59ln8q | 2.0 T | 1,332 | 0.7 % | key known |
| 15 | dreamcitizencosmos | bostrom1wnpak7sfawsfv9c8vqe7naxfa4g99lv77d7c0z | 1.6 T | 1,083 | 0.6 % | key known; 404 links |
| 16 | — | bostrom10c8znaj94y900syues06dz50hw6y9qpkhunn25 | 1.4 T | 932 | 0.5 % | key known |
| 17 | — | bostrom1s3lgc3mgy4ltwr796qk6j73xmau4njueds0mgu | 1.3 T | 893 | 0.5 % | never signed; kind-3 claim |
| 18 | — | bostrom10556xtnp39nvx3ec3yjafug662pyt6af84n477 | 1.3 T | 869 | 0.5 % | never signed; kind-3 claim |
| 19 | main | bostrom1t4vzrx79r8pfunwhrhqzlkmplh3zxfdk8lwwcx | 1.1 T | 729 | 0.4 % | key known |
| 20 | rodiki | bostrom1s4wntr9selfaznc2ezjkwnwt2y0terx9tqpda3 | 1.0 T | 666 | 0.4 % | key known |

The periodic-vesting accounts among them vest only H/A/V (investmint), never
TOCYB; every TOCYB balance above is liquid.

## the multisig is the cybergift multisig

- `go-cyber/app/upgrades/v6/upgrades.go:233–243` (commit 1b62e564,
  2025-09-03, "added gift burn on upgrade") names
  `bostrom1qs9w7ry45axfxjgxa4jmuhjthzfvj78sxh5p6e` as `giftMSAddress` and
  burns 603 T BOOT and 603 T TOCYB from it.
- `space-pussy/ts/src/utils/finalResultGoL.js` gives it `gift: 700 T` in the
  game-of-links final result; cyber-prophet/bostrom-journal `bj001.md`: "70 %
  of supply is allocated to the Gift" linking this address.
- `bostrom/finalization of $BOOT distribution.md`: "cybergift multisig under
  cybercongress control: 603T BOOT and 700T TOCYB". 700 T − 603 T burned =
  the 97 T TOCYB left; its BOOT is 0 (97 T BOOT went to the cybergift
  contract earlier, the rest was burned).
- Members (secp256k1, threshold 2): bostrom18wmu3larp6xxjaxn0gk2tzxte29mtch2eg765f,
  bostrom1tvz7kpf0pwwf3pfguxqxgynkx9lhyv8zvqz9h8,
  bostrom17jrh028fcsq9zk44mumfudm6rhcds8uf07ka2c. None of them holds a
  balance, a pubkey, a delegation or a passport on bostrom, and none appears
  in any public GitHub file: keys used only inside the multisig. Account
  sequence 2.
- Why 97 T TOCYB survived the burn: at genesis the multisig held 700 T BOOT
  and 700 T TOCYB (`go-cyber/networks/bostrom/genesis.json`); 97 T BOOT left
  it for the cybergift claim contract, its TOCYB never moved; v6 burned a
  fixed 603 T of each, sized to the BOOT left, so the BOOT went to 0 and
  97 T TOCYB stayed. The supply confirms the upgrade ran: 1,000 T − 603 T
  (gift) − 115,594,467,532,355 (congress) = 281,405,532,467,645, the
  snapshot supply to the unit.
- The congress multisig of v6 is a different account,
  bostrom1xszmhkfjs3s00z2nvtn7evqxw3dtus6yr8e4pw: its 115.59 T TOCYB and
  its BOOT were burned; at the halt it holds only A and V. The `congress`
  passport account (#3) is a single key.

## IBC escrow on bostrom (tokens that live on other chains)

Escrow address = `sha256("ics20-1\0transfer/<channel>")[:20]`, for every
transfer channel in the export.

| denom | in escrow | supply | share | of which Osmosis (channel-2 ↔ osmosis channel-95) |
|---|---|---|---|---|
| boot | 30,031,818,136,617 | 482,287,925,778,234 | 6.23 % | 29,948,781,923,024 |
| hydrogen | 16,356,013,373,264 | 305,452,862,328,021 | 5.35 % | 16,307,648,876,474 |
| tocyb | 101,838,741,187 | 281,405,532,467,645 | 0.04 % | 101,031,208,714 |

Other channels: space-pussy channel-11 62.8 G BOOT, 48.0 G H, 0.81 G TOCYB;
cosmoshub channels 4/5/8 19.7 G BOOT; gravity, juno, desmos, evmos, axelar,
deep below 0.05 G BOOT each.

Osmosis supply of the bostrom denoms on 2026-10-19: BOOT
29,948,183,715,294 (0.6 G below the escrow: in-flight or timed-out packets),
H 16,307,648,876,474, TOCYB 101,031,208,714 — equal to escrow.

Holders on Osmosis (`denom_owners_by_query`): BOOT 1,876 accounts holding
25.74 T (4.21 T of the supply is not on any listed account, unresolved);
51 % of the listed BOOT sits in six pool module accounts, the rest on
20-byte keys whose bostrom twins are the same key (e.g. singleton/nexus
1.04 T, asadovka 0.56 T, mrgalaxy 0.52 T). H: 291 accounts, master 3.59 T,
zlobobah 2.10 T, ibcbot 1.18 T, pools ≈ 5.4 T. TOCYB: 62 accounts, asadovka
79.9 G and one unnamed key 19.3 G hold 99 %.

At the cyber genesis the escrow accounts are address-only, so they fall to
`legacy_owner` and stay dormant: the 30 T BOOT held on Osmosis has no owner
on the reborn bostrom unless the genesis credits the Osmosis holders.

## pussy

PUSSY escrow 5,389,836,483,227,616 (0.49 % of 1.0994 E), almost all on
channel-0 to bostrom (it lives on bostrom as an ibc denom); liquidpussy
1.93 P (2.2 %). The pussy distribution module holds 982.67 P PUSSY, 98 % of
the supply — dormant at genesis like bostrom's community pool.

## the great web foundation account

- Genesis (`go-cyber/networks/bostrom/genesis.json`): the 5 % allocation is
  bostrom1ha4pw9w2qgc2ce9jwfrwrmaft5fneus58nqwev, 50 T BOOT and 50 T
  TOCYB, a 2-of-3 multisig (LegacyAminoPubKey). Members:
  bostrom10e85swjxcpwvjr583jghwxnnsl46yd6m7wsxrn,
  bostrom1hfs27j6j6p63jewgkspm3x3cm972hj4tpmphz0, and
  bostrom1mqn6z7tpyy3glwqfsmg9lly87vjs8dgkt3zs3a (the `congress` passport
  key, #3 above). The first two hold no balance and no passport.
- It sent exactly one transaction in its life (archive
  `lcd.bostrom.cybernode.ai`, `message.sender`): height 20,518,010,
  2025-09-16 11:48:42 UTC, tx
  9F8D815C3ACE1A6AA8BB2F95BECB0182817BCFA7605B9DA6E5AEDCF024B40DD7 — MsgSend
  of all 50 T BOOT and 50 T TOCYB to bostrom1mqn6z7…3a (congress). Signed by
  members 1 and 3 (bitarray 0xA0): congress signed the transfer to itself
  with one co-signer. Fee 42,000 BOOT, funded minutes earlier by
  bostrom1nfmvw8x37w00p3geuu8lrt3vt5kadxa5xd9us7 (`developer42`, heights
  20,517,532 and 20,517,907); 42 BOOT dust remains.
- Ten days before the v6 upgrade (applied at height 20,667,000,
  2025-09-26 11:46 UTC, `/cosmos/upgrade/v1beta1/applied_plan/v6`), so the v6 burns (gift multisig 603 T,
  congress multisig 115.59 T) did not touch it: the GWF's 50 T TOCYB is the
  50 T TOCYB on the congress key today, a single key, live at the cyber
  genesis.
