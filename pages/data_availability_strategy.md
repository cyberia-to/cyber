- prefer transparent cryptography with no trusted setup or social committees.
- separate compute/consensus from data availability; keep a clean, swappable boundary.
- tier data by criticality and expected half-life to optimize cost versus permanence.
- make light verification the default path; anyone should verify on a phone.
- design for graceful degradation and reversible choices via governance.

threat model (high-level)
- da withholding by block producers or operators.
- long-range or governance capture of a single da provider.
- corruption of trusted-setup ceremonies (where used) or committee collusion.
- network partition and economic attacks that raise posting costs beyond thresholds.

trust stack summary
- no-ceremony cryptography: hash trees + erasure coding (celestia), fri/stark-style sampling (frida research).
- escape hatch: ethereum calldata for minimal checkpoints (fully trustless, expensive).
- avoid committee trust for mission-critical data; allow as optional low-cost tier if ever needed.

layered da stack
- tier 0 — critical roots: post checkpoint roots to ethereum calldata once per epoch; immutable forever; low bandwidth (≈32–64 kb/epoch); used for ultimate recovery and dispute resolution.
- tier 1 — active graph: post "focus blobs" (≈10k links + proofs) to celestia; retain for ≥30 days; mirror to ipfs/filecoin; verified by celestia light sampling on phones.
- tier 2 — historical tails: erasure-coded archival to filecoin/arweave/ipfs pinning; refreshed by archivers; used for deep replay and research analytics.

target sLOs and kpis
- blob post success rate ≥ 99.95% over rolling 7 days.
- median light-client acceptance latency ≤ 2 seconds per blob.
- cost per mb (tier 1) below budget b (governance parameter) with automated reroute if exceeded.
- checkpoint cadence hit rate 100% with zero missed epochs.

posting unit: focus blob
- content: batch of cyberlinks + minimal per-link metadata + optional sampling metadata.
- size target: 1–4 mb per blob (tunable); bounded by da provider limits.
- retention: min 30 days on tier 1; long-term via tier 2.

light-client verification path
- today: celestia das with √n random share sampling until acceptance threshold reached.
- feature flag: embed fri/frida-style proofs in blob metadata to enable poly-log sampling when available.
- phones-first: default clients verify availability before accepting any on-chain reference.

checkpoints and replay
- every n blocks form a state commitment (e.g., merkle/patricia root + rolling hash of blob ids).
- write the commitment to ethereum calldata; store a small proof-of-inclusion reference on our chain.
- replay requires: retrieve blobs from celestia/ipfs → apply to state → compare to last checkpoint root.

failure and degradation modes
- if celestia posting cost > max\_blob\_fee, pause tier-1 posting and increase checkpoint frequency; queue blobs for later or route to backup da if approved by governance.
- if retrieval fails from celestia, fetch via ipfs/filecoin mirrors; if still unavailable, mark range as contested and trigger dispute protocol using checkpoints.
- if governance capture suspected, switch da target via on-chain parameter without hard fork.

on-chain parameters (governance knobs)
- min\_sampling\_confidence: default 0.999 (probability data is available given samples).
- max\_blob\_fee: absolute or dynamic cap (e.g., usd/mb); triggers reroute or defer.
- checkpoint\_interval: blocks per ethereum checkpoint; trade cost vs recovery speed.
- redundancy\_factor: number of independent pinning providers for each blob.
- retention\_days\_tier1: minimum retention requirement before archiving.

kzg/committee exposure policy
- mission-critical data (tier 0/1) must avoid trusted setup and committee trust.
- optional experimental tier may use kzg-based or dac-backed da for non-critical, short-lived analytics; disabled by default.

interfaces and data structures
- blob descriptor (on-chain):
  - hash: blake3/sha256 of blob payload.
  - da\_target: enum { celestia, future\_frida, reserved }.
  - size\_bytes: uint64.
  - retention\_class: enum { hot, warm, cold }.
  - mirrors: list for ipfs/filecoin.
- posting api (off-chain service → da adaptor):
  - post\_blob(payload, policy) → da\_receipt { target, height/slot, commitment, inclusion\_proof }.
  - verify\_availability(da\_receipt, confidence) → bool.
  - publish\_descriptor(da\_receipt, mirrors) → tx\_hash (our chain).
- routing policy (pseudo):

```
fn route(policy, market):
  if policy.trust_minimized_only and market.target != celestia:
    target = celestia
  if market.celestia_fee_per_mb > policy.max_blob_fee:
    target = pause_and_queue
  return target
```



operational runbook (excerpt)
- assemble batch every block or at size threshold.
- run erasure-coding locally for redundancy hints; compute hash.
- call post\_blob to celestia; receive da receipt.
- perform light-client sampling to target confidence; on success, publish descriptor on our chain.
- push payload to ipfs/filecoin with redundancy\_factor mirrors; store multihashes in descriptor.
- emit metrics: cost per mb, acceptance latency, failure reasons.

implementation timeline
- q3 2025: celestia blob poster + ipfs fallback; light sampling in mobile client; basic dashboards.
- q4 2025: ethereum calldata checkpoint writer; incident automation; cost benchmarks public.
- q1 2026: embed fri/frida-style sampling metadata; feature-flag rollout; adversarial testing.
- h2 2026: evaluate production frida-based da; migration playbook; staged cutover if justified.

comparison snapshot (july 2025)

\| solution | type | trusted setup | committee trust | throughput (now/roadmap) | retention | light sampling | maturity | |---|---|---|---|---|---|---|---| | celestia | modular da l1 | none | none | high → roadmap to \~1 gb blocks | persistent | yes (das) | mainnet | | ethereum eip-4844 blobs | l1 blobspace | kzg ceremony | none | moderate; cheap for l2s | \~18 days then pruned | commitment only | mainnet (mar 2024) | | eigenda | restaked da | kzg ceremony | none | high (operator-limited) | operator-configurable | roadmap | mainnet | | avail | modular da | kzg ceremony | none | high; "infinity blocks" roadmap | persistent | yes | mainnet | | arbitrum anytrust dac | committee da | none | ≥1 honest member | variable | committee policy | no (assumption-based) | mainnet | | starknet volition/validium | hybrid | none (stark proofs) | optional dac | variable | mode-dependent | n/a | rolling out | | frida (research) | sampling algo | none | none | theoretical poly-log overhead | n/a | yes (core) | academic |

risk register (initial)
- celestia throughput or fee volatility makes tier-1 uneconomical → mitigation: max\_blob\_fee guard, dynamic batch sizing, temporary checkpoint frequency increase.
- ipfs/filecoin mirror unreliability → mitigation: redundancy\_factor ≥ 3, periodic verify-and-repin jobs.
- ethereum calldata spikes in gas price → mitigation: elastic checkpoint\_interval with floor; pre-funded gas vault and hedging.
- future frida networks slip or underperform → mitigation: keep feature-flagged and optional; do not block tier-1 operations.

monitoring and alerts
- blob\_post\_cost\_usd\_mb (p95) crossing threshold.
- light\_accept\_latency\_ms (p95) crossing threshold.
- missing\_checkpoints count > 0 within window.
- mirror\_retrieve\_errors > x per day.

open questions
- optimal batch sizing under varying network conditions without hurting light verification latency.
- standardized receipt format across da providers to simplify bridges and wallets.
- best-in-class erasure coding parameters for our data shape and retrieval patterns.

next actions
- implement the da adaptor skeleton and wire to celestia light clients.
- ship the calldata checkpoint writer and recovery/replay tool.
- draft the governance parameters and defaults; socialize with stakeholders.
- define the archiver incentives and service-level requirements for mirrors.

glossary (brief)
- da: data availability — the guarantee that posted data can be retrieved by anyone.
- light client: a verifier that checks availability/proofs without downloading full data.
- kzg: polynomial commitment scheme needing a multi-party trusted setup ceremony.
- fri/stark: transparent proof systems with no trusted setup; used by frida/starknet.
- dac: data availability committee; a social trust model requiring one or more honest members.