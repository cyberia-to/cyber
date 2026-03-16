---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: nouns, noun tree, binary tree
---
# noun

the universal data type of [[nox]] and the atomic unit of content-addressed storage in [[cyber]]. a noun is either an atom (a single [[Goldilocks field]] element) or a cell (an ordered pair of nouns). there is nothing else. every program, every state, every message, every proof witness — all are nouns.

## what is radical

every other content-addressed system has a serialization layer. Git has object headers (`blob 42\0`). IPFS has CBOR-encoded DAG nodes with link tables. Ethereum has RLP encoding. Protocol Buffers have field tags and wire types. every one of them pays framing overhead to make the byte stream self-describing.

nox has no serialization format. the store maps 64-byte identity to content, and content length IS the type. this is not an optimization — it is a category elimination. the serialization layer does not exist.

the hash function IS the type system at the protocol level. a field atom with value 7 and a word atom with value 7 have different identities — not because of a tag byte, but because [[Hemera]] capacity carries different values during permutation. the type distinction is enforced by the mathematics of the sponge, not by a convention on top of it. you cannot forge a field-typed identity from a word-typed value because the Poseidon2 permutation is one-way.

fixed-size everything. three content sizes: 8, 64, 128. the store is a fixed-size key-value map. no variable-length records. no allocation decisions. no fragmentation. uniform record sizes with content-addressed keys.

cells store hashes, not data. a cell is always 128 bytes: two child identities. the tree is navigated by hash lookup, not by pointer chasing through variable-length buffers. structural sharing is automatic — two cells with the same left subtree store the same left hash, and the store deduplicates by identity. this is Merkle tree semantics applied to the entire data model, not just to a specific data structure.

one hash function for everything. [[Hemera]] does structural hashing, Merkle trees, Fiat-Shamir challenges, content addressing, domain separation, commitment schemes, nullifier derivation. one function, one output size (64 bytes), one security assumption. the entire cryptographic surface area is one Poseidon2 instance.

## honest tradeoffs

storage amplification for small nouns. a field atom is 8 bytes of data but has a 64-byte identity. a cell of two small atoms: 16 bytes of actual data, but the cell stores 128 bytes of child hashes, plus each atom has a 64-byte identity. the store entry for the cell is 128 bytes, pointing to two 8-byte entries. total: 128 + 8 + 8 = 144 bytes for 16 bytes of data. 9x overhead.

for deep trees this amortizes (hashes are shared, deduplication kicks in). but for flat formulas with many small atoms, storage is heavier than a serialized format would be.

resolution latency. materializing a noun of depth d requires d sequential store lookups. a flat serialization reads one contiguous buffer. for proof verification (the hot path), the verifier processes nouns that might be 20-30 levels deep — that is 20-30 lookups. with an SSD that is microseconds; in memory it is nanoseconds. acceptable, but not free.

no streaming decode. with a serialized format, you can read bytes and build the noun in one pass. with content-addressed resolution, you must fetch the root, then its children, then their children — breadth-first or depth-first, but always recursive. you cannot pipe a noun through a socket and process it incrementally.

the atom identity paradox. an atom identity (64 bytes) is larger than its content (8 bytes). you carry more metadata than data for leaves. in systems with many small atoms (which formulas are — pattern tags are atoms 0-16), the identity overhead dominates.

## why the tradeoffs are acceptable

every tradeoff above trades throughput for verifiability. in a proof-native system, this is the right trade:

- storage amplification does not matter when the [[stark]] proof compresses everything to 60-157 KiB regardless of computation size
- resolution latency does not matter when the hot path is the prover (which processes the noun in memory anyway) and the verifier (which only checks the proof, not the noun)
- no streaming is fine because nouns enter the system through reduction, not through deserialization — the VM builds nouns, it does not parse them
- the atom identity paradox is actually a feature: small values get strong identities, making the content-addressed cache effective even for trivial sub-expressions

the system is designed for one thing: produce a computation, prove it, verify the proof. every design choice optimizes for that path. the 64-byte identity is the unit of trust — and having it be clean, uniform, and prefix-free means the trust layer has zero accidental complexity.

see [[nox]], [[Hemera]], [[Goldilocks field]], [[cyber/stark]], [[cyber/architecture]]
