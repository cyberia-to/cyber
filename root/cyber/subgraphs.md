---
tags: cyber
crystal-type: entity
crystal-domain: cyber
alias: subgraphs
---
# subgraphs

subgraphs are repositories imported into the graph. their pages and files become particles alongside cyber content — linkable, searchable, and ranked together by the tri-kernel.

## how the system works

the workspace anchor at [[.github]] owns all subgraph concerns. under it, [[.github/subgraphs]] holds one declaration per repo, each with visibility, archive state, and last-seen metadata written by `.github/scripts/sync-org.nu`.

`.github/scripts/build.nu` materializes the filtered list into a TOML manifest and hands it to [[optica]] via `--subgraphs <TOML>`. optica then pulls each declared repo, renders its pages and files as particles, and merges them with cyber content into one ranked graph.

cyber itself declares no subgraphs — that concern belongs to the workspace. the authoritative, always-current set is the namespace itself.

## canonical set

follow [[.github/subgraphs]] for the full list of declarations.