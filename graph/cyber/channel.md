---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: cyber channel, state channel, proof channel, bilateral channel
---
# channel

a bilateral value exchange between two [[neurons]] where every interaction — message delivery, computation, knowledge — adjusts a mutual token ledger through [[stark]]-proven [[nox]] state transitions, exchanged directly via [[radio]]. the proof replaces the chain. the ledger prices the interaction. the channel is the atomic unit of the network economy.

## the state channel problem

state channels have existed since 2015 (Lightning Network, Raiden, Perun, Nitro). the idea: two parties lock funds on-chain, exchange signed state updates off-chain, settle on-chain when done. elegant in theory, stalled in practice.

the reason is liveness. traditional state channels need the chain as a "court of last resort" — if your counterparty submits an old state while you are offline, you must respond within a dispute window or lose funds. this single requirement poisons everything: watchtowers that must stay online 24/7, dispute timelocks that delay settlement, and an entire class of griefing attacks based on forcing the other party to go to chain.

liveness is the fundamental problem. routing, capital lockup, and channel management are problems of payment channel networks (Lightning), which compound channels into a routing topology. the direct bilateral channel is clean — except for liveness.

## how STARK proofs kill liveness

traditional state channels need dispute windows because the chain cannot verify which state is correct without both parties showing up. the chain sees two signed states and must wait to see if anyone submits a newer one. the chain is a dumb judge that needs time.

[[nox]] changes this. every state transition is a STARK-proven computation:

```
S_{n+1} = reduce(S_n, formula, focus)    with proof π_{n+1}
```

the proof π is self-verifying. it says: "S_{n+1} is the mathematically correct result of applying this formula to S_n." any party can check it. the chain, a third neuron, or a program running a century later — the proof speaks for itself.

```
CHANNEL LIFECYCLE
═════════════════

open:
  neurons A and B agree on initial state S₀ = [ledger₀ data₀]
  ledger₀ = [deposit_A deposit_B]    mutual token commitment
  both sign H(S₀)
  exchange via radio — one optional on-chain tx to lock tokens (or use existing balances)

update:
  A proposes: reduce(S_n, formula_A, focus) → S_{n+1} with proof π_{n+1}
  proof enforces: balance_A + balance_B = deposit_A + deposit_B (conservation)
  B verifies π_{n+1}
  B signs H(S_{n+1})
  both hold (S_{n+1}, π_{n+1}, sig_A, sig_B)

  or B counter-proposes: reduce(S_n, formula_B, focus) → S_{n+1}'
  negotiation is formula exchange — each proposal is a proven transition

close:
  either neuron publishes the latest signed state (claim their balance)
  or neither does — the bilateral state is self-sufficient
  or they roll the balances into a new channel (rebalance without closing)
```

no dispute window. no timelock. no watchtower. if your counterparty submits state S₃ while you hold state S₇, anyone can verify that π₇ proves a valid chain from S₃ to S₇. the higher nonce with a valid proof chain wins — instantly, mathematically, without waiting.

## the mutual ledger

every interaction costs something. a message needs relay — relay costs [[focus]]. a computation needs cycles — cycles cost focus. knowledge has value — value is denominated in tokens. a channel without a mutual ledger is a chat app. the token balance is the foundation.

the channel state is a [[noun]] with a bilateral ledger at its core:

```
CHANNEL STATE
═════════════

S = [ledger shared_data]

ledger:
  balance_A:  F_p     tokens held by neuron A
  balance_B:  F_p     tokens held by neuron B

conservation invariant (enforced by STARK proof):
  balance_A + balance_B = deposit     (constant for the channel lifetime)
```

every state transition adjusts the ledger. the [[stark]] proof guarantees conservation — no tokens created or destroyed within the channel. the formula that updates the state must preserve the sum. if it does not, the proof fails and the counterparty rejects it.

```
EXAMPLE TRANSITIONS
═══════════════════

message delivery:
  A sends message via relay to B
  relay proves delivery (proof of delivery)
  ledger: balance_A -= relay_fee, balance_B unchanged, relay claims fee

computation request:
  A asks B to compute reduce(data, formula, focus)
  B computes, produces result + proof
  ledger: balance_A -= compute_fee, balance_B += compute_fee

knowledge exchange:
  A shares a particle (new knowledge)
  B values it, adjusts balance
  ledger: balance_A += value, balance_B -= value

streaming service:
  B serves data to A continuously
  each chunk adjusts the ledger by a micro-amount
  thousands of adjustments per second, all proven
```

the ledger enables everything. relay payment, compute markets, knowledge pricing, streaming micropayments — all as bilateral ledger adjustments within a single channel. no on-chain transaction per payment. no routing through intermediaries. two neurons, one ledger, proven conservation.

## beyond the ledger

the channel state is a full [[noun]] — the ledger is the foundation, but the `shared_data` subtree carries anything expressible as a binary tree of [[Goldilocks field]] elements:

- a local [[cybergraph]] fragment (bilateral knowledge graph)
- a game state (board position, move history, scores)
- an AI conversation (context tree, inference history)
- a negotiation protocol (offers, counteroffers, constraints)
- a collaborative computation (partial results, work allocation)

every update is a [[nox]] formula applied to the previous state, with a [[stark]] proof of correctness. the channel is a bilateral computer with a built-in economy. the ledger prices the computation. the computation enriches the shared state. the proof guarantees both.

## content-addressed history

every state is content-addressed: `H(S_n)` is a [[Hemera]] digest. the channel history is a hash chain:

```
H(S₀) → H(S₁) → H(S₂) → ... → H(S_n)
```

each transition is a fact in the planetary computation cache: `(H(S_n), H(formula)) → H(S_{n+1})`. this means:

- duplicate computations are detected and skipped (memoization)
- the channel history is tamper-evident (any modification breaks the hash chain)
- either neuron can prove the full history to any third party
- the history can optionally be published to the [[cybergraph]] (some or all states become [[particles]])

## transport

channels use [[radio]] for direct neuron-to-neuron communication:

- QUIC connections with NAT hole-punching
- [[CSIDH]] key agreement from public curves in the [[cybergraph]] (non-interactive)
- end-to-end encryption (AES-256-GCM with session keys)
- onion routing through relays when direct connection fails

the channel protocol operates above [[cyber/communication]] — it inherits privacy, encryption, and proof of delivery. channel updates are narrowcast (neuron-to-neuron), not broadcast.

## optional chain integration

a channel never needs the chain. but it can touch the chain when useful:

- publish the final state as a [[particle]] (make the result public)
- merge a local cybergraph fragment into the global [[cybergraph]] (announce discoveries)
- submit a [[cyber/signal]] that references the channel state (create [[cyberlinks]] from proven bilateral computation)
- claim [[focus]] rewards for proven state transitions (the proof qualifies as an [[cyber/impulse]])

the chain is an option, not a requirement. two neurons can maintain a channel indefinitely without any on-chain presence. the proof is the trust — the chain is the megaphone.

## comparison with traditional state channels

| property | Lightning/Raiden | cyber channel |
|----------|-----------------|---------------|
| liveness required | yes (dispute window) | no (proof is self-verifying) |
| dispute mechanism | timelock + watchtower | none needed (STARK proof) |
| state type | balance allocation | arbitrary noun (any computation) |
| settlement | mandatory on-chain close | optional (proof is self-sufficient) |
| capital lockup | yes (fund channel on-chain) | no (focus flows, not locked) |
| routing | multi-hop with hidden balances | direct bilateral (no routing) |
| proof size | signatures only | ~100 KB STARK proof per transition |
| verification | replay state transitions | O(log n) proof check |
| privacy | partial (channel visible on-chain) | full (channel can be entirely off-chain) |

## the atomic unit

a channel is the atomic unit of the network economy. every service in [[cyber]] reduces to a bilateral exchange: relay a message (pay), compute a result (pay), share knowledge (get paid), store data (pay), verify a proof (pay). the channel is where all of these happen — at [[radio]] speed, with [[stark]] guarantees, priced by the mutual ledger.

the [[cybergraph]] is what neurons choose to make public. the channel layer is where neurons compute, negotiate, exchange, and prove — bilaterally, privately, continuously.

the network is channels. the graph is publication. the ledger is the economy. the proofs are trust.

see [[cyber/communication]], [[radio]], [[nox]], [[stark]], [[cybergraph]], [[cyber/focus]]
