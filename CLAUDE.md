# Claude Code Instructions
## Git Workflow
- **Commit by default.** After completing a change, commit it. Don't wait
  for the user to say "commit". Only stage without committing when the user
  explicitly asks to stage.
- **Atomic commits.** One logical change per commit. Never combine two
  independent features, fixes, or refactors in a single commit. If you
  made two separate changes, make two separate commits. Don't commit
  half-finished work either — if unsure whether the change is complete,
  ask before committing.
- **Conventional commits.** Use prefixes: `feat:`, `fix:`, `refactor:`,
  `docs:`, `test:`, `chore:`.
## Knowledge Graph Purpose

This is the seed knowledge base for planetary superintelligence. Pages
are pure markdown with YAML frontmatter. The publisher is
[optica](https://github.com/cyberia-to/optica) — a standalone knowledge
graph publisher.

## Page Format

Pages use YAML frontmatter for metadata and standard markdown for content:
```yaml
---
tags: cyber, menu
crystal-type: entity
crystal-domain: cyber
icon: "\U0001F535"
---
```
Wiki-links (`[[page]]`) and query expressions (`{{query (...)}}`,
`{{embed [[page]]}}`) are the graph's own syntax, evaluated by the
publisher.

Namespaced pages live in directories: `root/bostrom/infrastructure/servers.md`

## Building cyber

cyber is the workspace anchor. It holds the content graph plus the
subgraph declarations (`subgraphs/*.md`), the optica config
generator (`scripts/build.nu`), the publish workflow
(`.github/workflows/publish.yml`), and the IPFS cache
(`ipfs-cache.json`). One repo to fork, one repo to clone.

Local workflow (subgraph repos are siblings of cyber/):

```
cd ~/cyber/cyber
nu scripts/sync.nu        # bootstrap: clone every active subgraph
nu scripts/build.nu       # full graph build
nu scripts/serve.nu       # build + serve with live reload
nu scripts/dev.nu         # rebuild optica + restart serve
```

For pure content edits without subgraphs, optica works standalone:

```
optica build ~/cyber/cyber
optica serve ~/cyber/cyber --open
```

Port 8888 (from `publish.toml` base_url). Port 8080 is reserved.
## Tagging Conventions

Every page should have a `tags:` field in frontmatter. Key project tags (lenses):
- `cyber` — the superintelligence protocol
- `cyb` — the browser/interface
- `cyberia` — the cyber network state
- `bostrom` — the bootloader chain
- `cyber valley` — the physical city/estate

Domain tags: `article`, `cybernomics`, `compound`, `ticker`, `person`,
`ui`, `recipe`. Biology pages use `species`, `genus`. Body pages use
`muscle`. Ops pages use `operation`.
## Writing Style
- **Never define by negation.** Do not write "this is not X" or "not a Y
  but a Z". Say what something IS. Negation is a crutch — state the
  positive identity directly.
- **Never use bold (`**text**`).** Bold is banned from the graph. For
  emphasis use: YAML frontmatter for key-value pairs, `# heading` for
  section titles, `[[wiki-link]]` for inline emphasis on concepts. If a
  term does not deserve its own page, it does not need emphasis — just
  write it plain.
## Wiki-Link Plurals

Never write `[[term]]s` with a floating `s` outside the link. Every
concept page that has a meaningful plural must include both forms in its
`alias:` line (e.g. `alias: isomorphisms` on the `isomorphism` page).
Then link the plural directly: `[[isomorphisms]]` instead of
`[[isomorphism]]s`. This keeps links clean and resolvable.
## Shell: Nushell

Use `nu -c '...'` or `nu script.nu` for all scripting. Nushell has
structured data pipelines, built-in dataframes, and powerful search/filter
commands — use them instead of bash+sed+awk+grep chains. Examples:
- list pages: `ls root/*.md | get name`
- find untagged: `glob root/**/*.md | where {|f| not ((open --raw $f) | str starts-with "---\n") }`
- count by tag: `glob root/**/*.md | each {|f| open --raw $f | lines | where $it =~ 'tags:' | first } | where $it =~ 'species' | length`
- dataframe ops: `dfr open`, `dfr filter`, `dfr group-by` for bulk analysis

Reserve bash only for git commands and system tools that have no nu equivalent.
### Nushell input/output formatting
- **Input**: for non-trivial analysis (>3 lines), write a `.nu` script
  into `analizer/` in this repo (cyber) and run via `nu analizer/script.nu <graph-path>`.
  One-liners are fine as `nu -c '...'`.
- **Chat display**: always use ` ```nu ` fenced code blocks when showing
  nushell code in conversation so syntax highlighting works in Zed.
- **Output in scripts**: wrap table pipelines in `print (... | table)`
  so all sections render. Bare `| table` at end of pipeline only works
  for the last expression — intermediate tables need explicit `print`.
### Nushell script library (`analizer/`)

All nushell scripts live in `~/cyber/cyber/analizer/`. Scripts are graph-agnostic:
they take the graph path as an argument via `def main [graph_path: string]`.

Usage from any directory:
```
nu ~/cyber/cyber/analizer/stats.nu ~/git/cloud-forest
nu ~/cyber/cyber/analizer/analyze.nu ~/cyber/cyber
```

Scripts:
- `analizer/analyze.nu` — general analytics (files, tags, categories, links, IPFS)
- `analizer/stats.nu` — graph statistics (orphans, broken links, content types)
- `analizer/migrate.nu` — migrate Logseq format to pure markdown (YAML frontmatter, directories)
- `analizer/ipfs.nu` — pre-commit hook: upload media/ to Pinata IPFS, rewrite URLs in markdown (credentials from `~/.config/cyber/env`)
- `analizer/crosslink_topology.nu` — crosslink topology analysis for semantic core (wiki-link classification, hub/island detection, statistics)
- `analizer/concat.nu` — concatenate entire graph into single file for LLM context loading
- `analizer/context.nu` — smart context packer: scores pages by gravity/density, greedy knapsack into token budget. `--pinned [rel/path.md ...]` forces specific pages in before the greedy loop
- `analizer/trikernel.nu` — compute tri-kernel (diffusion, springs, heat) over wiki-link graph, write focus + gravity + density to frontmatter. Runs on new moon only (±1 day); use `--force` to override, `--dry-run` to preview

- `analizer/dangling.nu` — alias-aware dangling wiki-link detector (basenames, paths, alias: frontmatter; case-insensitive; top 50 + stats)
- `analizer/add-footer.nu` — append `discover all [[concepts]]` footer to core-tagged pages missing it
- `analizer/codematter.nu` — add comment-frontmatter to code files (.rs, .nu, .toml, .py, .sh, .yml), integrating source into cybergraph as particles

When adding a new script: place it in `analizer/`, accept `graph_path` as first
arg, and update this list.
## Tri-Kernel Weight Updates (Lunar Cycle)

Frontmatter weights (diffusion, springs, heat, focus, gravity, density)
are updated once per lunar cycle on the new moon (±1 day). This prevents
constant git noise from auto-computed fields across all repos.

- `trikernel.nu` enforces this with a date guard; `--force` overrides
- `--dry-run` computes and prints without writing (always allowed)
- After a new moon run: commit all weight changes in one atomic commit
  per repo with prefix `chore: new moon weights YYYY-MM-DD`
- Between moons: never run trikernel without `--dry-run` or `--force`

2026 new moons: Jan 18, Feb 17, Mar 19, Apr 17, May 16, Jun 15,
Jul 14, Aug 12, Sep 11, Oct 10, Nov 9, Dec 8.

## Parallel Agents for Graph-Wide Tasks

When a task touches many pages across the graph (bulk tagging, renaming,
formatting fixes), split the work into non-overlapping scopes by filename
or other criteria, then launch several agents in parallel. Before
splitting: enumerate the full file list, partition it into disjoint sets
(e.g. by alphabetical range, by tag, by namespace), and assign each set
to a separate agent. No two agents should ever touch the same file.
## License

Cyber License: Don't trust. Don't fear. Don't beg.