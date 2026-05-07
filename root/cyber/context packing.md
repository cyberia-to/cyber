---
tags: cyber, optica
crystal-type: process
crystal-domain: cyber
---
loading the [[cybergraph]] into an LLM [[context]] window — selecting the most valuable pages to fit a token budget

the full graph with [[subgraphs]] is ~6.5 MB (~1.4M tokens). a 1M context window holds ~900K tokens. context packing selects which pages enter the window and which stay outside

## method

each page receives a score derived from graph structure:

$$\text{score} = \gamma_{\text{eff}}^2 \times (1 + \delta) \times \log_2(s)$$

| symbol | meaning |
|---|---|
| $\gamma_{\text{eff}}$ | effective gravity — inbound link count + reflected gravity from outbound targets |
| $\delta$ | density — outbound [[wiki-links]] per KB |
| $s$ | substance — content size in bytes |

reflected gravity: if a page links to high-gravity pages, it inherits a fraction of their gravity. one step of [[diffusion]] with coefficient $\alpha = 0.05$. this ensures [[subgraphs]] pages that reference core [[concepts]] receive nonzero score even with zero inbound links

pages with a `stake:` field receive a 1.5× bonus

the packer sorts by score and greedily fills the token budget — largest score first, skip if exceeds remaining capacity

## usage

```nu
# graph only, 900K token budget (default)
nu analizer/context.nu ~/git/cyber

# full graph + all subgraphs
nu analizer/context.nu ~/git/cyber --subgraphs

# custom budget
nu analizer/context.nu ~/git/cyber --subgraphs --budget 500

# show ranking table without packing
nu analizer/context.nu ~/git/cyber --subgraphs --stats
```

output goes to `/tmp/cyber-context-{budget}k.md` by default, or specify `-o path`

## results on current graph

| metric | value |
|---|---|
| total pages scanned | ~2700 |
| pages packed (900K budget) | ~1400 (50% coverage) |
| zero-gravity pages excluded | ~450 |
| [[subgraphs]] pages included | proportional to their cross-graph gravity |

## connection to [[cyberank]]

context packing is a simplified [[cyberank]]. gravity is the degree centrality analog. reflected gravity is one iteration of the [[diffusion]] operator. the scoring formula approximates what the [[tri-kernel]] computes in full: which [[particles]] deserve the most [[focus]]

the key difference: [[cyberank]] runs to convergence on the full [[cybergraph]] in [[consensus]]. context packing runs one step offline as a build tool

see [[analizer/context.nu]] for implementation. see [[concat.nu]] for the simpler alternative that packs everything without ranking