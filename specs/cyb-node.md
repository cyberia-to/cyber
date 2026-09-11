---
tags: cyber, specs, cyb, node, api
crystal-type: spec
crystal-domain: cyber
status: implementation
alias: cyb node connection
---

# cyb node connection

The product command specification is [[specs/cli]]; [[specs/worker]] covers
the future execution/proving backend. This page describes current HTTP
behavior independently of those target extensions.

The host executable is `cyber`. Cyb connects to its HTTP endpoint and keeps
its personal cell, identity, and UI state in the cyb home.

## run

From the cyber repository with sibling component checkouts available:

```sh
cargo build --release --locked
./target/release/cyber init --bind 127.0.0.1:7780 --moniker local
./target/release/cyber node
```

In another terminal:

```sh
./target/release/cyber status --json
./target/release/cyber cyb
```

In cyb's commander, repoint its default network entry:

```text
net set spacepussy-test http://127.0.0.1:7780
```

If that entry was removed, use `net add spacepussy-test
http://127.0.0.1:7780`. Cyb relays to the first configured network; use
`net` to inspect the ordering. The new local node begins an independent
state. Existing cyb relay cursors and previously submitted history remain
in cyb's home; changing the endpoint does not import that history.

## launcher contract v1

`cyber --home PATH cyb` outputs JSON with schema `cyber/connection/v1`,
absolute home, network, RPC URL, version, mode, route names, a cyb commander
instruction, and capability flags. It describes configuration; it does not
assert that the process is running.

A launcher runs `cyber --home PATH node` as a foreground child, checks
`GET /health`, then validates `GET /status`. Startup/config errors exit
nonzero and write diagnostics to stderr. Termination currently uses process
signals; a graceful drain protocol is future work. A second cyber process
for the same home fails before opening the engine.

## existing HTTP contract

| route | shape | current cyb consumer |
|---|---|---|
| GET /health | text `ok` | process liveness |
| GET /status | cybermark frontmatter, including `height`, `bbg-root` | body networks |
| POST /v1/link | JSON neuron/from/to/amount/valence; JSON ok/height/root/signals | body relay |
| POST /v1/frame | native foculus frame bytes | native clients |
| GET /root, /stats, /log | root hex, cybermark stats, binary frame log | diagnostics/replication clients |
| GET /blocks, /block/H | cybermark explorer data | oracle |
| POST /v1/pay | JSON transfer request | money UI |

The node forwards this surface from soft3 without a second HTTP server.
`cyber status --json` translates status for command-line consumers; the HTTP
status remains cybermark so existing cyb parsers continue working.

This version exposes unsigned local chaosnet operations. Peer sync,
authenticated writes, consensus participation and joy jobs are explicitly
false in the descriptor. Health and height alone provide liveness and local
progress; trust and finality require [[specs/node-modes]].

## verification

`cargo test --locked` launches a real node in a temporary home, submits a
link using cyb's JSON shape, checks height/root and explorer routes, kills
and restarts it, and checks replay equality. It also verifies home locking,
config preservation, and rejected configuration/status inputs.

discover all [[concepts]]
