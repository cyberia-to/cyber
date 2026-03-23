---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: content addressing, particle addressing, nox CID
stake: 42267076377875984
diffusion: 0.00011514671148758254
springs: 0.002899252570172658
heat: 0.0019753872300903104
focus: 0.0013224265728136895
gravity: 1
density: 1.31
---
# particle: content addressing

a [[particle]] is a content-addressed node. identity = [[Hemera]] hash of content. 64 raw bytes, no headers, no version prefix. one hash function, one address space, permanent

every other system wraps hashes in self-describing envelopes — IPFS CIDv1 carries version, multicodec, multihash function, digest length, then the digest. at planetary scale ($10^{15}$ [[particles]]), 5 bytes of framing overhead is 5 petabytes of pure waste, forever. worse: headers imply upgradability, but in an immutable graph there is nothing to upgrade. one function means nothing to disambiguate

the address is the identity. `Hemera(content)` — that is the [[particle]]. no registration, no authority, no namespace collision. two agents on opposite sides of the planet hashing the same content produce the same address. the first [[cyberlink]] to that address brings the [[particle]] into the [[cybergraph]]. a naked hash with no links never enters the graph

## [[Hemera]]

```
Hemera = Poseidon2(
  p  = 2⁶⁴ − 2³² + 1     Goldilocks field
  d  = 7                   S-box: x → x⁷
  t  = 16                  state width (elements)
  Rꜰ = 8                   full rounds (4 + 4)
  Rₚ = 64                  partial rounds
  r  = 8                   rate (64 bytes in)
  c  = 8                   capacity (64 bytes)
  out = 8 elements          64 bytes out
)
```

every parameter is a power of 2. the [[Goldilocks field]] gives native 64-bit CPU arithmetic — a field multiplication is a single instruction. the S-box exponent $d = 7$ is the minimum invertible exponent for this field ($\gcd(7, p-1) = 1$; both 3 and 5 divide $p-1$)

capacity 8 (256-bit) provides 256-bit classical collision resistance, 170-bit quantum collision resistance (BHT), and algebraic degree $7^{64} \approx 2^{180}$. production systems use capacity 4 (128-bit) because their hashes are ephemeral — trace commitments that live seconds. particle addresses live decades. the parameter choice matches the lifetime

one mode only: sponge. no compression mode. two modes producing the same 64-byte output from different inputs would break the address space as a function. the sponge is the particle, the particle is the sponge

```
initialize:  state ← [0; 16]
absorb:      for each 8-element chunk of padded input:
               state[0..8] ⊕= chunk
               state ← permute(state)
squeeze:     output ← state[0..8]
```

round constants are self-bootstrapping: Hemera generates its own constants from the seed `"cyber"` (5 bytes) through the zero-constant permutation. no foreign primitives in the dependency chain

see [[hemera/spec]] for the full decision record

## tree

large content splits into 4 KB chunks — OS page aligned, L1 cache fit, 512 field elements per chunk, 64 absorb blocks per leaf

```
leaf:          Hemera(chunk_bytes)
internal node: Hemera(left_id ∥ right_id)    128 bytes in, 64 bytes out
tree shape:    binary, left-balanced
particle:      root hash of the tree
```

left-balanced means the same content prefix always produces the same left subtree. streaming: buffer at most 4 KB + proof per step. deduplication: 4 KB blocks show meaningful repetition in real data. overhead: 1.6% tree metadata

a single chunk (≤4 KB) hashes directly — no tree, just `Hemera(content)`. the particle address is the same whether content is 10 bytes or 10 gigabytes: always 64 bytes, always a Hemera output

## domain separation

different uses of Hemera are separated at the input, not the output:

| prefix | domain |
|--------|--------|
| `0x01` | edge hashing |
| `0x02` | record commitments |
| `0x03` | nullifier derivation |
| `0x04` | Merkle internal nodes (NMT, MMR) |
| `0x05` | Fiat-Shamir challenges (WHIR) |
| `0x06` | proof transcript binding |

`H_edge(x) = Hemera(0x01 ∥ x)`. particle content addressing uses no prefix — bare content in, address out. the particle address space is the default

## output format

```
IPFS CIDv1:  <version><multicodec><multihash><length><digest>   36-69 bytes
nox CID:     <digest>                                           64 bytes
```

inside the protocol, the 64-byte digest is the complete identifier. IPFS compatibility is a thin translation layer at the gateway — inside [[nox]], the wrapper never exists

all identities live in one flat 64-byte namespace: [[particles]], edges, [[neurons]], commitments, nullifiers. no type tags in the address. the type is determined by where the address appears in the [[BBG]] structure, not by what it contains

## endofunction

`Hemera(Hemera(x) ∥ Hemera(y))` type-checks: 64 bytes in one side, 64 bytes the other, 64 bytes out. hash of hashes is a hash. this closure under composition is why Merkle trees, polynomial commitments, and recursive proofs all use the same function without conversion

## permanence

| property | zkVM (SP1, RISC Zero) | cyber |
|----------|----------------------|-------|
| hash lifetime | seconds to hours | decades to permanent |
| parameter update | software release | impossible without rehash |
| rehash cost | zero (ephemeral) | $O(10^{15})$ operations |
| cost of parameter error | reissue proofs | lose the graph |

if Hemera is ever broken: full graph rehash under a new primitive. no version byte, no algorithm agility, no graceful coexistence. one graph, one hash, one identity. storage proofs make this possible — they guarantee content availability for rehashing and must be operational before genesis

## performance

| metric | Hemera | SHA-256 in stark |
|--------|--------|-----------------|
| hash rate (single core) | ~62 MB/s | ~200 MB/s |
| stark constraints per hash | ~1,200 | ~25,000 |
| particles per second (200 B avg) | ~310K | — |

20× cheaper in proofs than SHA-256. 0.6× the raw throughput. the tradeoff is clear: particle addresses are verified far more often than they are created. optimizing for proof cost is optimizing for the common case