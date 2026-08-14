---
title: install
tags: cyber, soft3, cyb, install
crystal-type: guide
crystal-domain: cyber
---
# install

one crate. one binary. default network: [[space pussy]].

```bash
cargo install true-cyber
cyber sync
```

| | |
|--|--|
| crate | [true-cyber](https://crates.io/crates/true-cyber) on crates.io |
| binary | `cyber` (name `cyber` is taken on crates.io by an unrelated crate) |
| default network | `space-pussy` |
| override | `cyber sync -n bostrom` |

```bash
cyber sync
# cyber sync · space-pussy
#   chain_id        space-pussy
#   latest_height   …
#   rpc             https://rpc.space-pussy.cybernode.ai
```

```bash
cyber network                 # endpoints for the default network
cyber network bostrom
cyber sync -n bostrom
cyber manifesto
cyber help
```

## default endpoints (space-pussy)

| | |
|--|--|
| chain-id | `space-pussy` |
| rpc | `https://rpc.space-pussy.cybernode.ai` |
| lcd | `https://lcd.space-pussy.cybernode.ai` |
| index | `https://index.space-pussy.cybernode.ai/v1/graphql` |
| bech32 | `pussy` |
| denom | `pussy` |

[[bostrom]] remains available via `-n bostrom`. both chains still run on cosmos-sdk ([[go-cyber]]) and are exercised as [[soft3]] chaosnets for a fast cutover.

## optional faces

day-one is `cyber` alone. deeper faces when you want them:

| face | install | role |
|------|---------|------|
| [[soft3]] | `cargo install soft3` | stack CLI · also reachable as `cyber soft3 …` if on PATH |
| [[cyb]] | `cargo install cyb` | robot · binary `cy` · also `cyber cy …` if on PATH |

## why not `cargo install cyber`?

crates.io [`cyber`](https://crates.io/crates/cyber) is an unrelated crate (CipherDogs). the product package is **true-cyber**; the binary it installs is **cyber**.

chain daemons remain separate: [[go-cyber]] (`cyber` binary on bostrom nodes) and [[space pussy]] (`pussy` binary). different programs, different jobs.

## libs (builders)

```toml
soft3 = "0.3"   # facade + network defaults
cyb = "0.2"     # runtime cell
```

## more

- site: [cyber.page](https://cyber.page)
- stack: [[soft3]] · robot: [[cyb]] · bootloader: [[bootloader]]
