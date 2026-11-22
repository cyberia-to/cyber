# cyb.money

The site of CYB: one page, static, no build. `index.html` and `og.jpg`, moved from `cyb/landing/money` on 2026-11-22 as they stood: the code of cyb.money lives in the repository of CYB. The copy in cyb leaves when this lands.

Check: `python3 -m http.server -d site 8767`, then `python3 ~/cyber/cyber/scripts/cdp.py http://localhost:8767/`.

## one figure older than the repository

`index.html` states the tax on movement as 0.1 %, split in thirds — burn, earners, self (lines 871, 1083, 1106, 1725). The authority of the figure is [`cyber/$CYB.md`](../cyber/$CYB.md), which states 1 % of every transfer, lock and unlock, half burned and half to the budget of rewards, and `docs/economics.md` follows it. The page arrives here as it stood, byte for byte, so the two numbers are both in the tree and the page's is the stale one. Which of the two the network launches with is the owner's call; the page takes the answer before cyb.money is deployed from this repository.
