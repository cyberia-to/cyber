---
tags: cyber
crystal-type: process
crystal-domain: cyber
---
four ways in.

# pull request

cyberia spans ~50 repos and ~200 MB of source — and we collapsed
setup into a single clone. one fork, one bootstrap, full local
mirror.

```
git clone https://github.com/<you>/cyber.git
cd cyber
nu scripts/sync.nu       # clones every active subgraph as a sibling
nu scripts/serve.nu      # builds + serves at localhost:8888
```

after `sync.nu`, the whole network sits next to `cyber/` —
[[optica]], [[trident]], [[hemera]], every public subgraph. edit
a `.md` under `cyber/root/`, or jump into any sibling repo and
work there. each repo is its own git remote; fork → branch →
push → PR against that upstream.

`CONTEXT.md` at the repo root is the cyber graph distilled for
language models — drop it into your model's system prompt and the
agent inherits the voice and the structural awareness needed to
navigate the graph.

scripts:
- `sync.nu` — bootstrap; clones missing, fetches present
- `build.nu` — full graph build
- `serve.nu` — build + live-reload server
- `dev.nu` — rebuild optica + restart serve

every merged PR adds new context to the [[cybergraph]] — content,
code, or both.

# donate

[[ETH]] or any ERC-20 token to:

`0x0F29df83BCb651E172F5cd467313de64a3EA0Cf9`

direct support keeps the lights on while the [[hash]] stays open.

# buy [[$BOOT]]

[[$BOOT]] is the stake token of the [[bootloader]] — the chain that
runs the [[cybergraph]]. holding [[$BOOT]] funds [[hero]] nodes that
pay the [[infrastructure]] costs and motivates contributors to
advance the code. staking [[$BOOT]] opens write and influence
access to the [[cybergraph]]. buy on
[osmosis](https://app.osmosis.zone/assets/ibc/FE2CD1E6828EC0FAB8AF39BAC45BC25B965BA67CCBC50C13A14BD610B0D1E2C4?ref=portfolio).

# visit [[cyber valley]]

[[cyber valley]] is the physical autonomous village in the Bali
mountains where contributors live and build. [[visit us]].
