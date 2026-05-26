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

## adding a new subgraph

1. add a declaration to `subgraphs/<name>.md` with `visibility: public` (or `private` to exclude from prod build)
2. add `notify-cyber.yml` to the new repo so pushes trigger a rebuild:

```yaml
# .github/workflows/notify-cyber.yml
name: Notify cyber

on:
  push:
    branches: [master, main]

jobs:
  notify:
    uses: cyberia-to/cyber/.github/workflows/subgraph-notify.yml@master
    secrets: inherit
```

3. add `CYBER_DISPATCH_TOKEN` to the repo's secrets (Settings → Secrets → Actions). the token is a GitHub PAT with `repo` scope on `cyberia-to/cyber`. the org-level secret covers all repos — no per-repo config needed if it is already set at org level.

to add the workflow to all existing repos at once: `nu scripts/add-notify-workflow.nu` from the cyber root. dry-run first with `--dry-run`.

## canonical set

follow [[.github/subgraphs]] for the full list of declarations.