---
title: true-cyber
tags: cyber, cli
crystal-type: source
crystal-domain: cyber
alias: cyber cli, true-cyber
---
# true-cyber

`cyber` is the command line of the phase-1 root chain
([soft3/specs/chain.md](../../cybernode/specs/chain.md)): the three things a neuron
can do — transfer, cyberlink, claim — signed with the key your spell derives
and submitted to a chain node over the HTTP surface of
[chain-wire §7](../../cybernode/specs/chain-wire.md). Encoding and signing are
`soft3::wallet`, shared with cyb; this binary is the command line around it.

```sh
cargo install --path ~/cyber/true-cyber --locked
cyber neuron new                      # mints ~/.cyber/spell (0600); or reuse cyb's ~/cyb/spell
cyber status --rpc https://cyb.ai/pussy
cyber balance
cyber tx transfer pussy1…-or-64-hex 1000
cyber tx link "what lives?" "the graph" --valence 1
cyber tx claim                        # address-only genesis balance → your neuron
```

Every transaction is NSIG1 over `hemera("soft3/chain/sign/v1\0" ‖ chain ‖ tx)`:
the chain id (the node's `/status` `genesis` field, or `--chain-id`) is inside
the signature, so nothing signed for pussy is valid on bostrom. The nonce is
asked from `/nonce/{neuron}` unless `--nonce` says otherwise; the fee is the
floor (1 unit per byte: 224 / 257 / 140) unless `--fee` says more. `cyber tx …`
prints the txid, then polls `/tx/{txid}` until the transaction is in a block
(`--wait SECS`, `--no-wait`); `--json` prints the receipt.

A bostrom or pussy holder whose pubkey the old chain saw signs with the same
words (the implicit bridge of chain.md §2.8). An account that never sent a
transaction is parked under a dormant owner; `cyber tx claim` moves every
token of it to `neuron_of(pubkey)`.

`cyber help` lists every command and option. Validation:
`cargo test` runs the binary against a mock node that verifies each submitted
transaction with `soft3::wallet::verify` and rejects wrong nonce, wrong chain
id and forged keys.

The `cyber` binary name also exists in the old Cosmos go-cyber daemon. Check
`cyber version` when selecting a binary.

License: cyber — don't trust. don't fear. don't beg.
