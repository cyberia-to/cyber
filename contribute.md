---
tags: cyber, core
alias: contributing, how to contribute
icon: "🎭"
crystal-type: process
crystal-domain: cyber
crystal-size: bridge
---
four ways in. pick one lane — or all of them. every path teaches the [[cybergraph]]

## pull request

this repo is the protocol. fork, edit a `.md`, PR. merged pages become particles on [cyber.page](https://cyber.page).

```bash
git clone https://github.com/<you>/cyber.git
cd cyber
nu scripts/serve.nu      # protocol-only, localhost:8888
```

the chronicle, the subgraph census, and the full project graph live at [cyberia.blog](https://cyberia.blog) (`cyberia-to/cyberia-blog`) — clone that repo, `nu scripts/sync.nu`, `nu scripts/serve.nu`. each component is its own remote: fork → branch → push → PR upstream.

`CLAUDE.md` at the repo root is the protocol distilled for agents — drop it into a model’s system prompt and the agent inherits voice and structure

## link on the live net

run as a [[neuron]] on **spacepussy-test** (soft3 chaosnet). tokens are test; the graph is real

```bash
cargo install soft3 true-cyber --force
cyber sync
soft3 node --home ~/.spacepussy-test --bind 127.0.0.1:7780
```

public rpc: `https://cyb.ai/spacepussy-test` · engine cybergraph + bbg · guide [[install]] · [[soft3/docs/launch|launch]]

create [[cyberlinks]], pay [[focus]], earn [[karma]]. that is contribution on chain — not a form, not a waitlist

## donate

[[ETH]] or any ERC-20 to:

`0x0F29df83BCb651E172F5cd467313de64a3EA0Cf9`

direct support keeps infrastructure running while the hash stays open

## buy [[$BOOT]]

[[$BOOT]] is the stake token of the [[bootloader]] / [[bostrom]]. holding it funds nodes and governance that keep the existing graph alive while soft3 matures. buy on [osmosis](https://app.osmosis.zone/assets/ibc/FE2CD1E6828EC0FAB8AF39BAC45BC25B965BA67CCBC50C13A14BD610B0D1E2C4?ref=portfolio)

## visit [[cyber valley]]

[[cyber valley]] is the physical autonomous village in the Bali mountains where contributors live and build. [[visit us]]

---

[[superintelligence]] · [[soft3]] · [[cyb]] · [[whitepaper]] · [[install]]
