# cyb.money

The site of CYB: one page, static, no build. `index.html` and `og.jpg`, moved from `cyb/landing/money` on 2026-11-22 as they stood: the code of cyb.money lives in the repository of CYB. The copy in cyb leaves when this lands.

Check: `python3 -m http.server -d site 8767`, then `python3 ~/cyber/cyber/scripts/cdp.py http://localhost:8767/`.

## two statements older than the repository

The page arrives here as it stood, byte for byte, so where it disagrees with the files of the network the page is the older of the two. Both differences are the owner's to settle, and the page takes the answers before cyb.money is deployed from this repository.

### the tax on movement

`index.html` states the tax as 0.1 %, split in thirds — burn, earners, self (lines 871, 1083, 1106, 1725). The authority of the figure is [`cyber/$CYB.md`](../cyber/$CYB.md), which states 1 % of every transfer, lock and unlock, half burned and half to the budget of rewards, and `docs/economics.md` follows it. Both numbers are in the tree and the page's is the stale one. Which of the two the network launches with is the owner's call.

### the genesis as a balance at block 0

`index.html` puts CYB in circulation at block 0: the supply chart starts from `G0 = 187416` and draws M(t) = G0 + (cap − G0)(1 − (1 + t/τ)^−k) (lines 1604, 1605), the reading at year 0 says "genesis: 187,416 CYB out, 1.0 % of the cap. we are here." (line 1630), and the money card notes "genesis 187,416 CYB" (line 1504). The quantity is the right one — 187,416 CYB is 187,416,084,623,451,570 cy, the `claimable` of [`genesis/distribution.toml`](../genesis/distribution.toml), at 1 CYB = 10¹² cy — but the mechanism is the one this repository replaced: the rules give block 0 no balance (`genesis_supply = "0"`, `balance_frames = 0`) and mint that quantity by claims against the proven state of bostrom, so the schedule of the repository is M(t) = p(1 − (1 + t/τ)^−k) with M(0) = 0. The page knows this elsewhere — its launch caption reads "Then CYB, claims first" (line 1113) — and it is the chart and the money card that still read as a block-0 balance. How the first year is drawn when nothing is out at block 0 is the owner's call.
