---
tags: cyber, cip
crystal-type: pattern
crystal-domain: cyber
alias: interoperability, cross-chain, interchain communication
---
# interchain communication

how [[cyber]] connects to external chains, imports external state, and exports [[knowledge]]

## the problem

a [[cybergraph]] that cannot read external state is blind. a [[cybergraph]] that cannot export its [[focus]] distribution is mute. planetary [[superintelligence]] requires reading the world's on-chain state and writing [[knowledge]] back to it.

[[IBC]] (Inter-Blockchain Communication) is the native transport. cyber inherits the Cosmos IBC stack from [[bostrom]] and extends it with [[STARK]]-verified channels that remove the trust assumption from light client verification.

## three communication modes

| mode | direction | what moves | trust model |
|---|---|---|---|
| import | external → cyber | state proofs, price feeds, token transfers | IBC light client or STARK-verified header chain |
| export | cyber → external | [[focus]] distribution, [[cyberank]] proofs, oracle responses | STARK proof of tri-kernel computation |
| bridge | bidirectional | tokens, messages, cross-chain cyberlinks | IBC channel with mutual light client verification |

## import: reading external state

### IBC light clients

cyber runs IBC light clients for connected chains. each light client tracks the counterparty's consensus state — validator sets, block headers, Merkle roots — and verifies inclusion proofs against them.

standard IBC light clients (Tendermint, near, etc.) are trust-minimized: they verify consensus signatures and state proofs cryptographically. the remaining trust assumption is the counterparty chain's own security — if 2/3 of the counterparty's validators collude, they can forge state proofs.

### STARK-verified channels

for high-security channels, cyber replaces the IBC light client with a [[STARK]] proof of the counterparty's consensus. the counterparty's block validation logic is expressed as a [[nox]] program, and every header transition is proven. the verifier on the cyber side checks a constant-size proof instead of replaying consensus logic.

cost: proving a single header transition is ~$10^6$ constraints (dominated by signature verification). recursive composition amortizes this: N headers collapse into one proof. the practical cadence is one proof per epoch (~100 blocks), with individual transactions verified against the proven state root.

this eliminates the honest-majority assumption about the counterparty's validator set. the proof guarantees that the state transition rules were followed — regardless of who the validators are. the only remaining assumption is the correctness of the counterparty's consensus specification as expressed in [[nox]].

### what cyber imports

- token balances (ICS-20 transfers): [[$CYB]] moves to external chains, external tokens move to cyber
- price feeds: DEX TWAPs and oracle prices for the metabolic cap signal (§23.2 of the whitepaper)
- external state proofs: any on-chain fact from a connected chain can be attested as a [[particle]] in the [[cybergraph]]
- cross-chain identity: [[neuron]] keys on external chains can be linked to cyber [[neurons]] via IBC account interchain accounts

## export: writing knowledge to external chains

### the focus oracle

any on-chain system on a connected chain can query the [[cybergraph]]: "what is the current [[focus]] distribution over particles matching X?" the response is the ranked subgraph with a [[STARK]] proof that the ranking was computed correctly from the authenticated [[cybergraph]] state.

the oracle channel:

```
External contract                    cyber
═══════════════                      ═════
sends IBC query packet    →    receives query
                                    ↓
                               runs tri-kernel inference
                               over matching subgraph
                                    ↓
                               generates STARK proof
                               of correct computation
                                    ↓
receives response packet  ←    sends ranked particles
  + STARK proof                + proof + BBG state root
```

the external contract verifies the STARK proof on-chain (or via a pre-deployed verifier contract) and uses the result. the answer is a probabilistic oracle with on-chain provenance — a [[focus]]-weighted ranking across all linked [[particles]], verifiable without trusting the node that computed it.

### what cyber exports

- [[cyberank]] per [[particle]] (with proof)
- [[karma]] per [[neuron]] (with proof)
- [[syntropy]] of the whole graph (with proof)
- namespace completeness proofs: "these are ALL [[cyberlinks]] matching your query"
- compiled transformer weights: model parameters derived from [[cybergraph]] structure (§6.6)

## bridge: bidirectional channels

### ICS-20 token transfers

standard Cosmos token transfers. [[$CYB]] moves to connected chains as IBC vouchers. external tokens move to cyber and can be used for staking or [[ICBS]] market positions. the token transfer preserves conservation: the sending chain escrows, the receiving chain mints a voucher. return transfers burn the voucher and release the escrow.

### cross-chain cyberlinks

a [[neuron]] on an external chain can create a [[cyberlink]] in the [[cybergraph]] via IBC. the link is authenticated by the neuron's signature on the external chain, relayed through the IBC channel, and verified against the external chain's state proof.

this means a [[neuron]] operating on Ethereum, Solana, or any IBC-connected chain can contribute [[knowledge]] to the [[cybergraph]] without running a cyber node. their links are weighted by their staked [[$CYB]] (transferred via ICS-20) and scored by [[Bayesian Truth Serum]] identically to native links.

### interchain accounts (ICS-27)

a [[neuron]] on cyber can control accounts on external chains through interchain accounts. this enables the protocol [[neuron]] (§23.1) to execute cross-chain operations: providing liquidity on external DEXes, participating in external governance, or bridging compiled model weights to chains that consume them.

## topology

```
                     ┌─────────────┐
                     │   Cosmos    │
                     │   Hub       │
                     └──────┬──────┘
                            │ IBC
           ┌────────────────┼────────────────┐
           │                │                │
    ┌──────┴──────┐  ┌──────┴──────┐  ┌──────┴──────┐
    │   cyber     │  │  Osmosis    │  │  other      │
    │  (mainnet)  │  │  (DEX)      │  │  zones      │
    └──────┬──────┘  └─────────────┘  └─────────────┘
           │
           │ STARK-verified channels
           │
    ┌──────┴──────────────────────┐
    │  high-security bridges      │
    │  (Ethereum, Solana, etc.)   │
    └─────────────────────────────┘
```

the Cosmos Hub serves as the IBC routing hub for standard channels. STARK-verified channels connect directly to non-Cosmos chains where IBC light clients are unavailable or insufficient.

## security model

| threat | defense |
|---|---|
| counterparty validator collusion | STARK-verified channels eliminate this for critical paths |
| relay censorship | any [[neuron]] can run an IBC relayer; relay fees incentivize availability |
| oracle manipulation | [[focus]] oracle returns are STARK-proven against the full [[cybergraph]] state |
| token inflation via bridge | ICS-20 conservation enforced by escrow/mint/burn mechanics |
| cross-chain replay | IBC packet sequence numbers prevent replay; each channel has monotonic counters |

## implementation path

phase 1 (inherited from [[bostrom]]): standard IBC with Tendermint light clients. ICS-20 token transfers. ICS-27 interchain accounts. operational today.

phase 2 (at launch): focus oracle channel. external contracts can query [[cyberank]] with proofs. STARK verifier contracts deployed on target chains.

phase 3 (post-launch): STARK-verified IBC channels for non-Cosmos chains. cross-chain [[cyberlinks]]. the protocol [[neuron]] operates cross-chain via interchain accounts.

see [[cyber/proofs]] for the STARK proof taxonomy. see [[cyber/architecture]] for relay pricing. see [[bostrom/infrastructure/ibc]] for the current operational IBC setup
