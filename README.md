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
./target/release/cyber auth enable
./target/release/cyber node
```

The [delivery roadmap](roadmap/README.md) follows local cyb reliability →
integrated Joy computation → verified network participation.

This repository owns the `cyber` product binary, its configuration, and the
protocol graph. The source build currently uses sibling soft3 component
checkouts. [[specs/node-product|Node product]] maps the components and the
current implementation boundary. [Pinned source builds](specs/node-sources.md)
assemble the exact committed dependencies in an independent checkout.
[[specs/cyb-node|Cyb node connection]]
describes the local HTTP contract and launch commands.

The explicit `auth enable` step activates signed native publication for this
home and retires incompatible old readers. It creates no subject key. A new
genesis starts an independent local network; its native network ID is separate
from the `spacepussy-test` display label. HTTP receipts report endpoint acceptance;
verified peer replication and validator consensus have their own delivery gates.

In another terminal, `./target/release/cyber status --json` reads local state.
`./target/release/cyber cyb` prints an offline configuration descriptor with
unknown live network/profile fields. `./target/release/cyber cyb --live` validates
the endpoint's capabilities and prints the actual native network ID and profile.

In cyb's commander, add and explicitly pin the local endpoint:

```text
net add local http://127.0.0.1:7780
net probe local
net pin local NETWORK_HEX
```

Use the expected 64-digit network ID reported by the local node. For an existing
entry, `net set local URL` changes its endpoint. A name/URL without a network pin
remains read-only; endpoint ordering cannot choose an action's destination.
Submission also requires an explicitly controlled neuron attachment for that
network. The robot can attach different keys, networks and devices; progs run
under those neurons with their own retained work IDs.

For an existing legacy home, stop all writers and run
`cyber --home PATH storage import-legacy` before activation, or combine the steps
with `cyber --home PATH auth enable --import-legacy`. Both reuse the shared
storage owner and preserve the original source bytes. Existing configuration
stays intact; activation is a permanent reader-generation upgrade. See
[[specs/cli]] and [[specs/cyb-node]] for bounds, exact reports and recovery.

`nu scripts/release.nu --locked-sources` produces a host binary, checksum,
build provenance and the exact soft3 dependency inventory in `dist/` from a
pinned checkout. The Node workflow publishes tagged prereleases to
[GitHub Releases](https://github.com/cyberia-to/cyber/releases), with that
inventory and native binaries for Linux x86_64/ARM64 and macOS ARM64.
Before a version tag is selected, the same archives are available as artifacts
in [Node workflow runs](https://github.com/cyberia-to/cyber/actions/workflows/node.yml).
The [release contract](specs/releases.md) describes the build and publication cycle.

The sibling `true-cyber` source now provides a headless
client over the shared GraphSession and neuron registry: explicit key attachment,
network-bound signed publication, receipt reconciliation and history sync.
Its [migration contract](../true-cyber/specs/native-client.md) retains exact
earlier CLI logs and retires their old writer. The node entry point here is
version 0.8.0.

The [[audit/node-readiness|binary readiness audit]] records earlier journal,
retry and reward-admission failures and the broader product release gates.
Current native acceptance, signed retries, source-preserving migration and
actual client checks are recorded in the
[convergence evidence](../soft3/audit/neuron-cell/implementation.md). Those
local guarantees remain separate from Joy product scheduling, distributed
finality and release-platform acceptance.

and the robot: [[cyb]] — one binary that carries the graph, a terminal, and a local mind on macOS and Android; it paints the [[cybergraph]] at 100+ fps and answers from a model running on your own silicon. get it at [cyb.ai](https://cyb.ai)

cyb retains a default public endpoint for spacepussy-test — the [[soft3]] chaosnet on cybernode (`https://cyb.ai/spacepussy-test`). a legacy name/URL entry is read-only until its native network/profile is explicitly pinned. the local node above serves its own state at loopback. tokens and state are test. mainnet arrives at [[launch]]

[[litepaper]] · [[whitepaper]] · [[cyb]] · [[cyber/$CYB|$CYB]]

the chronicle of the project lives at [cyberia.blog](https://cyberia.blog). protocol log: [cyberia.blog/cyber](https://cyberia.blog/cyber).
