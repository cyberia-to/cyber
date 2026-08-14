---
title: install
tags: cyber, soft3, cyb, install
crystal-type: guide
crystal-domain: cyber
---
# install

one crate. one binary. default network: **spacepussy-test** (soft3 chaosnet).

```bash
rustup update stable         # soft3 needs ≥ 1.85
export PATH="$HOME/.cargo/bin:$PATH"

cargo install soft3 --force    # real node + operator CLI
cargo install true-cyber --force

cyber version                  # cyber … (true-cyber)
cyber sync
soft3 sync                     # same network via soft3
```

| | |
|--|--|
| network | **spacepussy-test** (soft3 chaosnet) |
| engine | cybergraph + bbg |
| public rpc | `https://cyb.ai/spacepussy-test` |
| node crate | [soft3](https://crates.io/crates/soft3) `soft3 node` |
| product face | [true-cyber](https://crates.io/crates/true-cyber) binary `cyber` |

```bash
cyber sync
#   engine           cybergraph+bbg
#   chain_id         spacepussy-test
#   moniker          cyberproxy-spt
#   latest_height    …
#   bbg_root         …
#   signals / particles / axons
```

run your own node: see [[soft3/docs/launch|launch spacepussy-test]].

```bash
soft3 node --home ~/.spacepussy-test --bind 127.0.0.1:7780
curl -sS -X POST http://127.0.0.1:7780/v1/link \
  -H 'content-type: application/json' \
  -d '{"neuron":"01","from":"0a","to":"0b","amount":1}'
```

```bash
cyber network
cyber manifesto
cyber help
```

## two different "space pussy" names

| name | substrate | role |
|------|-----------|------|
| **spacepussy-test** | soft3 | product chaosnet — default after install |
| **space-pussy** | cosmos-sdk / [[go-cyber]] on cybernode | bootloader experimental chain — migration source |

`cyber sync` never points at cybernode cosmos RPC. typing `-n space-pussy` or `-n bostrom` is rejected with a clear error.

[[bostrom]] and cosmos [[space pussy]] remain live bootloader history — see [[bootloader]]. they are not the soft3 product network.

## optional faces

| face | install | role |
|------|---------|------|
| [[soft3]] | `cargo install soft3` | stack CLI |
| [[cyb]] | `cargo install cyb` | robot · binary `cy` |

day-one is `true-cyber` alone.

## why not `cargo install cyber`?

crates.io [`cyber`](https://crates.io/crates/cyber) is an unrelated crate. product package is **true-cyber**; binary is **cyber**.

## launch the network

product chaosnet is **spacepussy-test**. operator manual: [[soft3/docs/launch|launch spacepussy-test]].

```bash
cyber network
cyber sync                 # public spacepussy-test on cybernode
```

## more

- [cyber.page](https://cyber.page) · [[soft3]] · [[cyb]] · [[bootloader]] · [[cyber/launch|launch plan]] · [[soft3/docs/launch|launch guide]]
