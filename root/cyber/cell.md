---
tags: cyber, core
alias: cells, shard, shards, cyber cell
crystal-type: entity
crystal-domain: cyber
stake: 30000000000000000
---
the atomic unit of the [[cyber/hierarchy]] — a group of ~10^3 [[particles]] that share a 4D coordinate and maintain their own local state

a cell is not designed. it is not assigned. cells emerge from the [[cybergraph]] through splitting and merging — the same way biological cells divide and fuse. there is no mechanism for a cell to appear from nowhere

## birth

at genesis there is one cell — the [[root cell]]. it contains the [[crystal]] and all early [[particles]]. as [[neurons]] create [[cyberlinks]] and the graph grows denser, the cell becomes too large for a single validator set to process efficiently

when the [[Laplacian]] eigengap of a cell's internal graph shows two distinct communities ([[springs]] reveals the split): the cell divides. state migrates along the spectral bisection boundary. two cells exist where one was. each inherits half the [[particles]], half the [[mutator set]], half the routing table

this is how the [[hierarchy]] is born — not by decree but by division. the first split produces two cells. each grows, accumulates [[cyberlinks]], and eventually splits again. cells → zones → domains emerge from repeated division over time

## what a cell holds

| Component | What it is |
|---|---|
| [[particles]] | ~10^3 content-addressed nodes in this cell's scope |
| [[cyberlinks]] | all edges between particles in this cell |
| [[mutator set]] | [[AOCL]] + [[SWBF]] — private UTXO creation and spending |
| local [[focus]] | the [[tri-kernel]] running at full resolution within this cell |
| routing table | maps particle hashes to this cell's particles |
| boundary state | [[focus]] values at boundary [[particles]] shared with neighboring cells |

## 4D coordinate

every cell has a position in four dimensions:

```
cell = (semantic, social, economic, geographic)
```

determined by where its [[particles]] cluster in the semantic space ([[tri-kernel]]), which [[neurons]] interact with it (social), which [[tokens]] flow through it (economic), and where its validators are located (geographic)

## splitting

when a cell grows too large (too many [[particles]], too much UTXO traffic, [[tri-kernel]] convergence slows):

1. [[springs]] computes the [[Laplacian]] eigenvectors of the cell's internal graph
2. the Fiedler vector (second-smallest eigenvalue) reveals the natural split
3. [[particles]] on each side of the split become two new cells
4. [[mutator set]] state partitions along the same boundary
5. routing tables update on the slow timescale

the split is proven via [[zheng|STARK]] — any observer can verify the division was correct

## merging

when two cells have become tightly coupled (high cross-cell [[focus]] flow, many cross-cell UTXO transfers, the boundary between them carries more traffic than the boundary with other neighbors):

the cells merge. state combines. the [[mutator set]] unifies. routing tables update. merging is the reverse of splitting — also proven via [[zheng|STARK]]

## the lifecycle

```
root cell (genesis)
    ↓ split
two cells
    ↓ grow, split
four cells
    ↓ grow, split, merge, split
...
10^20 cells (Avogadro scale)
```

no cell appears from nowhere. every cell descends from the root cell through a chain of splits. every merge combines cells that share ancestry. the [[hierarchy]] is a living tree that grows by division — the same mechanism that builds biological organisms from a single fertilized cell

see [[cyber/hierarchy]] for the full scaling architecture. see [[root cell]] for the genesis state. see [[AOCL]] and [[SWBF]] for the [[mutator set]]
