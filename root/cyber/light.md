---
tags: cyber, cip
crystal-type: pattern
crystal-domain: cyber
alias: light client, light node, cyber light client
---
# light client

a client that downloads and verifies the chain of headers. nothing more. a ~64 KB blockchain.

## the design

the cyber light client does not re-execute transactions, does not store the full [[cybergraph]], and does not run the [[tri-kernel]]. it verifies a chain of block headers, each containing a [[BBG]] state root. every claim about the system — [[cyberank]], [[karma]], [[focus]] distribution, [[cyberlink]] existence, balance, namespace completeness — is verified against that root with a cryptographic proof.

```
┌──────────────────────────────────────────────┐
│  FULL NODE                                    │
│                                               │
│  stores: full cybergraph, all cyberlinks,     │
│          all proofs, all history              │
│  computes: tri-kernel, focus, karma, syntropy │
│  produces: [[zheng]] proofs of block execution    │
│  size: unbounded (grows with graph)           │
└───────────────────┬──────────────────────────┘
                    │ headers + proofs
                    ▼
┌──────────────────────────────────────────────┐
│  LIGHT CLIENT                                 │
│                                               │
│  stores: chain of headers (~64 KB)            │
│  verifies: [[zheng]] proofs against header roots  │
│  trusts: nothing — proof is the guarantee     │
│  size: constant                               │
└───────────────────────────────────────────────┘
```

## what a header contains

```
BLOCK HEADER
════════════

prev_header_hash:    [F_p; 4]     chain link
height:              u64           monotonic counter
timestamp:           u64           block time
bbg_root:            [F_p; 4]     root of the Big Badass Graph
focus_root:          [F_p; 4]     commitment to current π* distribution
execution_proof:     [F_p; 4]     hash of [[zheng]] proof of block execution
validator_set_hash:  [F_p; 4]     commitment to current validator set

total: 29 field elements = 232 bytes
```

the header chain is the spine. every header commits to the full system state via `bbg_root`. the `execution_proof` field commits to a [[zheng]] proof that all state transitions in the block were valid. the light client never needs to see the proof itself during normal sync — it trusts the header chain's continuity and the validator signatures (or, post-stark-verification, the recursive proof).

## sync protocol

### initial sync

1. obtain the genesis header (hardcoded, ~232 bytes)
2. download the header chain from any peer (or multiple peers for redundancy)
3. verify header chain continuity: each header's `prev_header_hash` matches the hash of the previous header
4. verify validator signatures on each header (or verify the recursive [[zheng]] proof that covers the entire chain)
5. store the latest header as the trusted state root

at ~232 bytes per header and ~1 block per second, one year of headers is ~7.3 GB uncompressed. with recursive stark composition, the entire chain collapses into a single proof of ~100-200 KB plus the latest header. the light client can sync from genesis in one verification step.

### steady-state

once synced, the light client follows new headers as they arrive:

1. receive new header from any peer
2. verify it extends the current chain (prev_header_hash matches)
3. verify validator signatures (or [[zheng]] proof of consensus)
4. update trusted state root

one verification per block. no re-execution. no graph download.

## querying with proofs

the light client queries full nodes and verifies responses against the trusted `bbg_root`:

### cyberank query

"what is the [[cyberank]] of particle P?"

response: `(particle_id, π_value, proof)` where `proof` is a polynomial opening against `focus_root` proving that `π[particle_id] = π_value`.

verification: check the polynomial opening against the `focus_root` in the trusted header. cost: O(log² |G|) field operations.

### namespace sync

"give me all [[cyberlinks]] created by [[neuron]] N"

response: `(edges[], completeness_proof)` where the proof demonstrates that the returned set is complete — no edges were withheld.

verification: check that the completeness proof is valid against the `bbg_root`. the [[BBG]]'s sorted polynomial commitment structure enables range proofs over the neuron index. cost: O(|edges|) data + O(log² |G|) proof overhead.

### balance query

"what is [[neuron]] N's balance?"

response: `(neuron_id, balance, proof)` — polynomial opening against the balance commitment in `bbg_root`.

### cyberlink existence

"does the link A → B by neuron N exist?"

response: `(link, inclusion_proof)` — membership proof in the edge store polynomial.

### completeness (non-existence)

"prove that NO link from A to B exists"

response: `(exclusion_proof)` — range proof showing no edge in the sorted polynomial falls between the boundaries that would contain A → B. this is what [[BBG]] makes possible and what Merkle trees cannot: proving absence.

## proof sizes

| query type | proof size | verification cost |
|---|---|---|
| single value (rank, balance) | ~1-2 KB (polynomial opening) | O(log² &#124;G&#124;) |
| membership (link exists) | ~1-2 KB | O(log² &#124;G&#124;) |
| completeness (namespace sync) | ~2-4 KB + O(log² &#124;G&#124;) | O(log² &#124;G&#124;) |
| non-existence (absence proof) | ~2-4 KB | O(log² &#124;G&#124;) |
| full chain (recursive [[zheng]]) | ~100-200 KB | O(1) |

all proofs are constant-size relative to the query, logarithmic in graph size. a phone verifies any claim about a $10^{15}$-particle graph with a few KB proof and milliseconds of computation.

## what the light client cannot do

- run the [[tri-kernel]] (requires the full graph)
- compute [[focus]] independently (requires all [[cyberlinks]])
- produce [[zheng]] proofs (requires full execution trace)
- serve as a relay for other light clients (has no data to relay)

the light client is a pure verifier. it consumes proofs, never produces them. it trusts mathematics, never nodes.

## devices

the constant-size proof model makes the light client viable on:

- phones (the primary target): verify [[cyberank]] queries, check balances, validate [[cyberlinks]]
- browsers: embedded in [[cyb]] web interface
- IoT sensors: verify commands are authentic before acting
- embedded systems: minimal RAM, no disk, proof verification only

## comparison

| property | SPV (Bitcoin) | Tendermint light client | cyber light client |
|---|---|---|---|
| trusts | miners (longest chain) | 2/3 validators | nothing ([[zheng]] proofs) |
| verifies | PoW + Merkle inclusion | signatures + Merkle inclusion | [[zheng]] proofs + polynomial openings |
| can prove absence | no | no | yes (BBG completeness) |
| sync from genesis | download all headers | download validator set changes | verify one recursive proof |
| proof size | O(log n) per tx | O(1) per header | O(log² n) per query, O(1) for chain |
| post-quantum | no | no | yes (hash-based starks) |

## the 64 KB blockchain

at maturity with recursive stark composition: the entire blockchain state from any light client's perspective is the latest header (~232 bytes) plus the recursive proof covering the full chain history (~100-200 KB). this is the state. everything else — the full graph, every cyberlink, every proof, every transaction — is verified against this constant-size commitment.

a blockchain that fits in a QR code.

see [[cyber/proofs]] for the stark proof taxonomy. see [[cyber/bbg]] for the polynomial commitment structure. see [[foculus]] for the consensus mechanism that produces headers. see [[cyber/architecture]] for the fractal layer model where light clients operate at L3