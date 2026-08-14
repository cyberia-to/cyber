---
title: install
tags: cyber, soft3, cyb, install
crystal-type: guide
crystal-domain: cyber
---
# install

one crate. one binary. default network: **spacepussy-test** (soft3 chaosnet).

```bash
cargo install true-cyber
cyber sync
```

| | |
|--|--|
| crate | [true-cyber](https://crates.io/crates/true-cyber) |
| binary | `cyber` |
| default network | `spacepussy-test` |
| default rpc | `http://127.0.0.1:7780` (local soft3 node) |

```bash
cyber sync
# cyber sync · spacepussy-test
#   role             soft3 chaosnet (product default)
#   rpc              http://127.0.0.1:7780
#   reachable        yes | no
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

## more

- [cyber.page](https://cyber.page) · [[soft3]] · [[cyb]] · [[bootloader]] · [[launch]]
