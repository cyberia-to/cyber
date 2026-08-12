---
title: install
tags: cyber, soft3, cyb, install
crystal-type: guide
crystal-domain: cyber
---
# install

get the soft3 stack on your machine. after install, **default sync network is [[space pussy]]**.

## cargo (recommended)

```bash
cargo install soft3
cargo install cyb
```

| binary | crate | role |
|--------|-------|------|
| `soft3` | [soft3](https://crates.io/crates/soft3) | stack CLI · `soft3 sync` probes the graph |
| `cy` | [cyb](https://crates.io/crates/cyb) | runtime cell · product face |

```bash
soft3 sync
# soft3 sync · space-pussy
#   chain_id        space-pussy
#   latest_height   …
#   rpc             https://rpc.space-pussy.cybernode.ai
```

override network:

```bash
soft3 sync --network bostrom
soft3 network bostrom
cy network space-pussy
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

[[bostrom]] remains available via `--network bostrom`.

## what about `cyber` CLI?

there is **no first-party `cargo install cyber`** for the product stack:

- crates.io [`cyber`](https://crates.io/crates/cyber) is an **unrelated** crate (CipherDogs / cyber-rs)
- the **chain daemons** are built from [[go-cyber]] (`cyber` binary for bostrom) and [[space pussy]] (`pussy` binary)
- the **product CLIs** you install today are **`soft3`** + **`cy`** (from `cyb`)

so for day-one graph sync and light status: use soft3, not `cyber`.

## libs

```toml
soft3 = "0.3"   # facade + network defaults
cyb = "0.2"     # runtime cell
cyb-lens = "0.1"
foculus = { version = "0.1", default-features = false }
```

## more

- site: [soft3.org](https://soft3.org)
- stack docs: [[soft3]]
- space pussy: [[space pussy]]
- bostrom: [[bostrom]]
