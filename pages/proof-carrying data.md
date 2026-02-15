---
alias: PCD
tags: cyber, cryptographic proofs
crystal-type: entity
crystal-domain: computer science
---
generalization of [[incrementally verifiable computation]] from sequential chains to arbitrary DAGs

allows multiple independent computations to be combined into a single proof

each node in the DAG carries a proof that

- all predecessor proofs are valid
- the local computation at this node is correct

where [[incrementally verifiable computation]] handles a linear chain of steps, PCD handles branching and merging computation paths

a node can have multiple parents: it absorbs and combines their proofs via [[folding]] into a shared [[accumulator]]

enables distributed proof generation where different [[neurons]] prove different parts of the computation and results are merged

constructions

- built on top of IVC schemes like [[Nova]], [[HyperNova]], [[Protostar]]
- requires a compliance predicate that defines what "valid computation" means at each node
- the compliance predicate checks predecessor proofs and the local transition function

applications in [[cyber]]

- DAG-structured [[cybergraph]] verification: the knowledge graph is not a chain but a DAG of [[cyberlinks]], PCD matches this topology naturally
- parallel [[validator]] proving: different validators prove different subgraphs, then merge proofs at shard boundaries
- cross-shard integrity: when a query spans multiple shards of the [[cybergraph]], PCD combines per-shard proofs into a global certificate
- multi-agent reasoning: when multiple [[neurons]] contribute to a computation (e.g. collective ranking), PCD proves the aggregate is correct without re-executing each contribution
- [[authenticated_graphs]] with fractional cascading: PCD enables composition of proofs across the shard hierarchy described in [[authenticated_graphs]]

properties

- succinctness: final proof is small regardless of the DAG size
- parallelism: independent branches can be proved concurrently
- composability: any subtree of proofs can be verified independently
- generality: subsumes [[incrementally verifiable computation]] as the special case of a single-path DAG

related

- [[incrementally verifiable computation]]
- [[folding]]
- [[hash path accumulator]]
- [[cryptographic proofs]]
- [[interactive proofs]]
- [[authenticated_graphs]]