---
tags: bio, cyber
alias: topoisomerases
crystal-type: entity
crystal-domain: bio
crystal-size: enzyme
---
# topoisomerase

an enzyme that modifies the topology of [[DNA]] by transiently breaking one or both strands, passing another segment through, and resealing. the operation changes the linking number Lk = Tw + Wr (twist + writhe) by a controlled integer. the enzyme counts topological charge — it does not operate blindly.

two classes:

| type | strands cut | ΔLk | biological role |
|------|------------|-----|----------------|
| Type I | one | ±1 | relax supercoiling during replication |
| Type II | both | ±2 | untangle catenanes, decatenate chromosomes |

topoisomerases are the cell's topological bookkeepers. during DNA replication the polymerase introduces positive supercoiling ahead of itself (the two strands wind tighter as they are separated). without topoisomerase the replication fork would stall. the enzyme runs ahead, cleaving, passing, resealing — burning one ATP per step — maintaining the linking number in the correct range for the polymerase to proceed.

## the computational interpretation

topoisomerase is nature's [[zheng]] verifier:

- temporary break = introducing an intermediate witness into the proof circuit
- passing the strand through = the polynomial commitment passing through an evaluation
- resealing = the verification check that the algebraic constraint holds
- linking number conservation = the algebraic invariants preserved through the computation

the sequential bottleneck in [[zheng]] proving — each step constrained by the previous — mirrors Type II topoisomerase processing one crossing at a time. the speedup from parallel proving — identifying topologically independent segments and processing simultaneously — mirrors multiple topoisomerase molecules working different regions of the same chromosome.

## ATP as proof cost

each Type II topoisomerase step consumes 2 ATP. this is not waste — it is the thermodynamic cost of creating a topological singularity (the temporary break) and resealing it. the Carnot-like bound: you cannot change a topological invariant without paying energy.

in the [[zheng]] proof system the analog is the verifier's computational cost per step (~5μs). you cannot verify a topological invariant without checking it.

[[analogous-to]] [[helix]], [[topological-invariant]], [[zheng]]

discover all [[concepts]]
