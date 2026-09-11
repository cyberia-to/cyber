---
title: Cyber
tags: cyber, core, nav
crystal-type: entity
crystal-domain: cyber
icon: "🔵"
alias: the superintelligence protocol
menu-order: 0
---
the protocol for [[superintelligence]] — planetary first, interplanetary by design.

get paid to understand:

> Google learned from humanity for free — and sold you ads.\
> OpenAI learned from humanity for free — and sold you back your own knowledge.\
> Cyber pays you for teaching the superintelligence.\
> Learn by linking. Teach by being linked.\
> Own what the one mind becomes.

two reads: the [[litepaper]] — ten minutes, the whole argument, one law and both its products — and the [[whitepaper]], which carries the mathematics and the proofs

[[superintelligence]] is Type I infrastructure: every agent on a planet — human, machine, sensor, organism — teaches one self-improving [[cybergraph]]. every link is signed, staked, and costs real [[focus]], so collective attention converges on what genuinely matters. learning is mining; teaching is staking: the graph pays [[rewards]] to those who taught it

Type II asks more. worlds are light-minutes apart. any [[consensus]] that waits on a global vote or a global clock dies in the post. [[interplanetary superintelligence]] is only possible when agreement is computed by convergence — [[foculus]]: φ*_i > τ, no committee, no round-trip to finalize. that is the bar. planetary superintelligence is the first mountain; interplanetary is the ridge that needs foculus-class physics, not faster rockets alone

two doors in. the terminal:

```bash
cd ~/cyber/cyber
cargo build --release --locked
./target/release/cyber init
./target/release/cyber node
```

The [delivery roadmap](roadmap/README.md) follows local cyb reliability →
integrated Joy computation → verified network participation.

This repository owns the `cyber` product binary, its configuration, and the
protocol graph. The source build currently uses sibling soft3 component
checkouts. [[specs/node-product|Node product]] maps the components and the
current implementation boundary. [[specs/cyb-node|Cyb node connection]]
describes the local HTTP contract and launch commands.

In another terminal, `./target/release/cyber status --json` reads the running
node and `./target/release/cyber cyb` prints its connection descriptor.
In cyb's commander, `net set spacepussy-test http://127.0.0.1:7780` connects
the default network entry to this node. This starts an independent local
chaosnet state; public network replication and validator consensus remain
integration work.

`nu scripts/release.nu` produces a host binary, checksum, and dependency
provenance in `dist/`. The existing crates.io `true-cyber` release and sibling
`true-cyber` repository contain the earlier cell CLI with `sync` and `link`.
The new node entry point here is version 0.8.0 and is currently unpublished.

The [[audit/node-readiness|binary readiness audit]] reproduces journal data
loss, duplicate JSON retries and unproved test reward issuance, and tracks
the missing Joy/worker and network-instance integrations. Use this artifact
for local development while those acceptance gates are completed.

and the robot: [[cyb]] — one binary that carries the graph, a terminal, and a local mind on macOS and Android; it paints the [[cybergraph]] at 100+ fps and answers from a model running on your own silicon. get it at [cyb.ai](https://cyb.ai)

cyb's default public network is spacepussy-test — the [[soft3]] chaosnet on cybernode (`https://cyb.ai/spacepussy-test`). the local node above serves its own state at loopback. tokens and state are test. mainnet arrives at [[launch]]

[[litepaper]] · [[whitepaper]] · [[cyb]] · [[cyber/$CYB|$CYB]]

the chronicle of the project lives at [cyberia.blog](https://cyberia.blog). protocol log: [cyberia.blog/cyber](https://cyberia.blog/cyber).
