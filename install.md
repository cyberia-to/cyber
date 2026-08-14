---
title: install
tags: cyber, soft3, cyb, install
crystal-type: guide
crystal-domain: cyber
---
# install

one crate. one binary. default network: **spacepussy-test** (soft3 chaosnet).

```bash
rustup update stable
cargo install true-cyber --force
export PATH="$HOME/.cargo/bin:$PATH"
cyber version    # must say: cyber … (true-cyber)
cyber sync
```

| | |
|--|--|
| crate | [true-cyber](https://crates.io/crates/true-cyber) |
| binary | `cyber` (must be `~/.cargo/bin/cyber`) |
| network | `spacepussy-test` |
| public rpc | `https://cyb.ai/spacepussy-test` |

```bash
cyber sync
# cyber sync · spacepussy-test
#   rpc              https://cyb.ai/spacepussy-test
#   reachable        yes
#   chain_id         spacepussy-test
#   moniker          cyberproxy-spt
#   latest_height    …
```

if install or sync fails:

```bash
which cyber                  # not a random go-cyber binary
curl -sS https://cyb.ai/spacepussy-test/status | head
cargo install true-cyber --force
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
