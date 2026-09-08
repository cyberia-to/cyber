---
tags: cyber, research, article, cip
crystal-type: pattern
crystal-domain: cyber
status: proposal
date: 2026-09-08
alias: nomos, one token one chain, sovereign mints, nomos architecture
---
# nomos — one token, one chain

*status: proposal. argued, not yet law.*

a chain is the balance sheet of one issuer. one non-fungible name roots it; one fungible token is its whole state; nested sub-names are its chart of accounts. that is the entire chain. **nomos** — from νόμος, the law that gave nomisma its name — because each chain is the law of exactly one token.

two prohibitions make the architecture, and both are constitutional, not conventional:

1. **no chain can issue a second token.** issuing a token *is* creating a chain. the mint and the ledger are the same object
2. **no token ever moves to another chain.** there is no transport, no wrapping, no bridge. trading is not moving tokens — it is programming the conditions of their movement on their home chains, against proofs of the other chain's state

the [[cyber]] chain itself obeys both: it is the chain of [[cyber/$CYB|$CYB]] rooted by the root name, and its distinct utility is **routing and registration** — every other chain registers by linking its name and state root into the [[cybergraph]]. the graph is the mesh's routing table.

## why nobody built this

the idea is simple enough that its absence needs explaining. four reasons, each a missing precondition:

1. **proofs were expensive.** conditioning my chain's rule on your chain's state requires verifying your consensus cheaply, recursively, forever. before recursive folding ([[zheng]]: one field, constant-size verification), the only affordable connection was trust — a multisig bridge. the entire bridge-exploit industry is the fossil record of that shortcut
2. **finality was global.** architectures that finalize by global vote cannot let ten thousand chains settle independently — the vote is the bottleneck. [[foculus]] finality is local by construction: domains settle at domain speed, partitions freeze cross-domain trade instead of inventing conflicting truth. nomos needs exactly that and nothing more
3. **the shared-VM economy was path-dependent.** ethereum made token-as-contract-in-shared-state the default; everyone downstream optimized that local maximum (rollups share sequencers, appchains share bridges). RGB and Lightning got halfway — value that never leaves home, conditions instead of transport — but stayed inside bitcoin's constraints and never generalized to *every token is a chain*
4. **sovereignty without a proof mesh is unsafe.** a thousand tiny chains with a thousand tiny validator sets is a thousand cheap attacks — unless every chain anchors into a shared fabric. the missing piece was a graph that wants to hold everyone's roots: registration-as-cyberlink gives any chain a T1 anchor in one hop ([[cyb/parts/state|state]] tiers). the cybergraph is that fabric

## what it dissolves

- **the bridge exploit class** — the largest loss category in crypto history — is not mitigated but made unexpressible: there is nothing in transit to steal
- **global fee markets and MEV**: no shared mempool across assets; a trade touches exactly two chains; no global sequencer to extract from
- **state bloat**: each chain is one ledger and one name tree — small enough to prove in one trace and to verify on a phone at T0. [[cyb/parts/sigma|sigma]] becomes a portfolio of T0 reads
- **governance contagion**: each token community governs its own chain; upgrades are local; no committee anywhere — the money doctrine of the [[litepaper]] extended to every token
- **listing friction**: a token launches by registering a name. the [[tokens/plumb|plumb]] framework becomes a chain-factory

## the two protocols it does not hide

honesty about the hard parts — these must be designed before nomos is law:

1. **the trade without a free option.** conditions on two sovereign chains are an atomic swap; whoever commits second holds an option. cheap proofs shrink the window, not the asymmetry. candidate: epoch-batched matching against the [[foculus]] beacon (both legs bind to the same epoch or neither), deadlines as first-class condition syntax, order *commitments* hosted on the routing chain — information, never tokens; settlement pairwise at home
2. **the freshness contract.** a condition must not say "if event E on chain Y" but "if E, at tier ≥ τ, no older than t" — the [[cyb/parts/state|state]] doctrine (T0/T1/T3, Query{family, loc, object, at}) promoted from client to consensus. and counterparties die: bostrom taught us what a proof from a halted chain is worth — every cross-chain condition needs an expiry

## adoption map — where the graph must account for nomos

| page | what changes |
|---|---|
| [[3c]] | the deepest cut: import/export (proofs both ways) stay; the **bridge mode retires** — "tokens move" contradicts prohibition 2; IBC survives only as a T3 legacy adapter |
| [[cell]] | gains the second cell kind: the **ledger-cell** — born by a name, not by division; invariant is balance conservation, not minimal cut. division of knowledge-cells is its own proposal: [[spectral cell division]] |
| [[network]] · [[communication]] | already compatible: narrowcast frames of proofs; [[cyb/parts/sense|sense]] over [[cyb/parts/radio|radio]] carries conditions and receipts, never value |
| [[tokens/plumb|plumb]] | word → task: the chain-factory; "create a token" = "birth a chain" as one operation |
| [[cyber/$CYB|$CYB]] · [[litepaper]] | a word each: the mind-chain is constitutionally single-token; emission-for-knowledge is prohibition 1 read as monetary policy |
| [[whitepaper]] §3C | task: restate the section under nomos when the trade and freshness protocols exist |
| [[cyb/parts/state|state]] | word: the tier language is the condition language — the doctrine ascends from browser to consensus |
| [[cyb/parts/sigma|sigma]] · [[cyb/parts/vault|vault]] | word: the portfolio is T0 reads of small home chains; keys sign at home, never custody wrapped copies |
| [[aos/teleport|teleport]] · [[aos/hub|hub]] | task: the transfer gate becomes a condition-composer; "multi-chain connections via IBC" rewrites to proof connections |
| soft3 [[soft3/status|status]] · [[crystallization]] §0 | word: cell's *hold* seat covers both cell kinds; the census gains the registry-anchor role of cybergraph |
| [[foculus]] | task: the epoch beacon as the atomic-matching clock — a named requirement for protocol 1 |
| [[zheng]] · [[bbg]] | word: mutual-anchoring mesh (every chain's root as a cyberlink) and per-chain DA are the standing assumptions |
| [[cyberia/protocol/century-index|century index]] | word: CX is born a nomos chain, registered by name |

## the closing claim

nomos is not an addition to the stack — it is what the stack already implies. local convergence says chains settle alone; cheap recursion says they read each other for free; the graph says every name is a particle and every registration is a link; the burial taught that heavy things stay home while pointers travel. apply those four lessons to money and the conclusion is exactly this: **every token a chain, every chain a name, every trade a condition, and the one graph as the routing law between them.**

discover all [[concepts]]
