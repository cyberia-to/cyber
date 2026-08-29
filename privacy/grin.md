---
tags: privacy, crypto, ticker
alias: GRIN, $GRIN, mimblewimble
crystal-type: entity
crystal-domain: crypto
crystal-size: atom
---
the chain with no addresses (2019, mimblewimble). amounts live as Pedersen commitments, ownership is knowledge of blinding factors, and transactions merge: spent outputs cancel against their spends (cut-through), so the chain keeps only unspent commitments plus kernels. history physically shrinks — a new node syncs state, never the past

the lesson: a chain can forget. privacy and scale share one trick when the ledger stores commitments instead of stories — the appetite [[bbg]] inherits with its single committed polynomial. and the counter-lesson: grin's interactive transactions (both wallets online to build one) taxed adoption harder than its cryptography helped it — ceremony in the UX costs more than ceremony in the math

[[privacy]] · [[privacy/monero]] · [[bbg]]
