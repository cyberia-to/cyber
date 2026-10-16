---
title: install
tags: cyber, soft3, cyb, install
crystal-type: guide
crystal-domain: cyber
alias: install cyber
---
# install

two doors. this page is the terminal. the robot is [[cyb]] at [cyb.ai](https://cyb.ai).

```bash
cd ~/cyber/cyber
cargo build --release --locked
./target/release/cyber init
./target/release/cyber auth enable
./target/release/cyber node
```

`auth enable` activates signed native publication for this home. it creates no subject key. a new genesis is its own local network; the native network id is not the spacepussy-test display label. http receipts report endpoint acceptance; peer replication and validator consensus have their own gates.

in another terminal:

```bash
./target/release/cyber status --json
./target/release/cyber cyb --live
```

`cyber cyb` prints the offline descriptor. `--live` asks the endpoint and prints the actual native network id and profile.

## pin in cyb

```
net add local http://127.0.0.1:7780
net probe local
net pin local NETWORK_HEX
```

use the 64-digit network id the local node reports. `net set local URL` changes an existing endpoint. a name/url without a pin stays read-only; ordering cannot choose a destination. submission needs an explicitly controlled neuron on that network. the robot attaches keys, networks and devices; progs run under those neurons with their own work ids.

cyb's default public endpoint is spacepussy-test — the [[soft3]] chaosnet on cybernode (`https://cyb.ai/spacepussy-test`). a name/url stays read-only until its native network/profile is pinned. the local node above serves loopback. tokens and state are test. mainnet arrives at [[launch]].

## existing home

stop all writers, then `cyber --home PATH storage import-legacy` before activation, or one shot: `cyber --home PATH auth enable --import-legacy`. both reuse the shared storage owner and keep the original source bytes. existing configuration stays; activation is a permanent reader-generation upgrade.

bounds, exact reports, recovery: [[specs/cli]] · [[specs/cyb-node]].

## more

this repo is the `cyber` product binary, its configuration, and the protocol graph. source builds use sibling [[soft3]] checkouts. [[specs/node-product]] maps the components. [[specs/node-sources]] assembles a pinned checkout. [[roadmap]] is local cyb reliability → joy → verified network participation.

`nu scripts/release.nu --locked-sources` writes a host binary, checksum, provenance and the soft3 inventory into `dist/`. friday draft candidates land in [GitHub Releases](https://github.com/cyberia-to/cyber/releases). the [release contract](specs/releases.md) is [[specs/releases]].

crates.io [`cyber`](https://crates.io/crates/cyber) is a different crate. the product binary is `cyber` from this repo.
