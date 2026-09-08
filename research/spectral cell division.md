---
tags: cyber, research, article
crystal-type: pattern
crystal-domain: cyber
status: proposal
alias: spectral cell division, laplacian cell division, cell division
---
# spectral cell division

*status: proposal, explicitly not for implementation — the idea needs finishing. extracted from [[cell]] so the canon page states what a cell IS, and this page argues how cells might MULTIPLY.*

## the mechanism

a knowledge-cell that grows past what one validator set can process must split, and the graph itself says where. the [[Laplacian]] of the cell's internal graph reads its shape: when two dense communities joined by a thin waist have formed, the second eigenvalue λ₂ falls toward zero and an eigengap opens. the Fiedler vector — the eigenvector at λ₂ — assigns every particle a number; its sign two-colors the cell, and the zero line of that coloring is the **minimal cut**: the split that severs the fewest [[cyberlinks]].

division follows the biology it is named for: state migrates along the spectral bisection boundary; two cells exist where one was; each inherits its half of the particles, links, [[mutator set]] and routing table, and the two share boundary [[focus]] state. applied recursively, division grows the whole [[hierarchy]] — cells → zones → domains — with the [[heat]] kernel at temperature τ revealing each level. nothing is designed; the hierarchy is born.

## why it is not ready

open questions that keep this a proposal:

1. **atomicity of migration** — a split moves half a mutator set and half a routing table; what serves queries mid-division, and what proves the division itself was clean?
2. **validator economics** — who serves a daughter cell the moment it exists? stake must split along the same boundary as state, and no design says how
3. **the trigger** — an eigengap is a signal, not a decision: who pays for the spectral watch, what threshold fires, and can an adversary sculpt links to force or forbid a split (split-griefing)?
4. **merging** — biology has fusion; two under-used sibling cells should re-merge, and the reverse operation is entirely unspecified
5. **the empirical null** — [[bostrom]] ran 25.1M blocks and 2.9M links and never needed to divide: one cell carried the whole bootloader. division may be a 10⁹-particle problem, which is exactly why it can wait

## relation to oikos

[[research/oikos|oikos]] introduces a second cell kind — the ledger-cell, born by a name rather than by division, conserving a balance rather than minimizing a cut. the two kinds coexist in one hierarchy: knowledge divides, value registers. this page owns only the first kind.

discover all [[concepts]]
