---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: cyber channel, state channel, proof channel, bilateral channel
---
# channel

a bilateral computation session between two [[neurons]] where state transitions are [[stark]]-proven [[nox]] computations exchanged directly via [[radio]], with no chain involvement. the proof replaces the chain — either party can verify any state independently, and any third party can too.

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
  neurons A and B agree on initial state S₀ (a noun)
  both sign H(S₀)
  exchange via radio — no chain transaction

update:
  A proposes: reduce(S_n, formula_A, focus) → S_{n+1} with proof π_{n+1}
  B verifies π_{n+1}
  B signs H(S_{n+1})
  both hold (S_{n+1}, π_{n+1}, sig_A, sig_B)

  or B counter-proposes: reduce(S_n, formula_B, focus) → S_{n+1}'
  negotiation is just formula exchange

close:
  either neuron publishes the latest signed state
  or neither does — the bilateral state is self-sufficient
```

no dispute window. no timelock. no watchtower. if your counterparty submits state S₃ while you hold state S₇, anyone can verify that π₇ proves a valid chain from S₃ to S₇. the higher nonce with a valid proof chain wins — instantly, mathematically, without waiting.

## what a channel carries

a channel is a shared [[noun]] — a binary tree of [[Goldilocks field]] elements. this means the channel state can be anything expressible as a noun:

- a balance allocation (field elements representing token amounts)
- a shared knowledge structure (a local [[cybergraph]] fragment)
- a game state (board position, move history, scores)
- an AI conversation (context tree, model weights, inference history)
- a negotiation protocol (offers, counteroffers, constraints)
- a collaborative computation (partial results, work allocation)

the state is not limited to "who owes whom how much." it is arbitrary computation. every update is a [[nox]] formula applied to the previous state, with a [[stark]] proof of correctness. the channel is a bilateral computer.

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

a channel is the atomic unit of interaction in [[cyber]]. the [[cybergraph]] is what neurons choose to make public. beneath it, the channel layer is where neurons compute, negotiate, exchange, and prove — bilaterally, privately, at [[radio]] speed.

the network is channels. the graph is publication. the proofs are trust.

see [[cyber/communication]], [[radio]], [[nox]], [[stark]], [[cybergraph]]
