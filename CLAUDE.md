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
are pure markdown with YAML frontmatter. The publisher lives at
`src/publish/` (Rust crate `cyber-publish`).

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

Namespaced pages live in directories: `pages/bostrom/infrastructure/servers.md`
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
`alias::` line (e.g. `alias:: isomorphisms` on the `isomorphism` page).
Then link the plural directly: `[[isomorphisms]]` instead of
`[[isomorphism]]s`. This keeps links clean and resolvable.
## Shell: Nushell

Use `nu -c '...'` or `nu script.nu` for all scripting. Nushell has
structured data pipelines, built-in dataframes, and powerful search/filter
commands — use them instead of bash+sed+awk+grep chains. Examples:
- list pages: `ls pages/*.md | get name`
- find untagged: `glob pages/**/*.md | where {|f| not ((open --raw $f) | str starts-with "---\n") }`
- count by tag: `glob pages/**/*.md | each {|f| open --raw $f | lines | where $it =~ 'tags:' | first } | where $it =~ 'species' | length`
- dataframe ops: `dfr open`, `dfr filter`, `dfr group-by` for bulk analysis

Reserve bash only for git commands and system tools that have no nu equivalent.
### Nushell input/output formatting
- **Input**: for non-trivial analysis (>3 lines), write a `.nu` script
  into `nu/` in this repo (cyber) and run via `nu nu/script.nu <graph-path>`.
  One-liners are fine as `nu -c '...'`.
- **Chat display**: always use ` ```nu ` fenced code blocks when showing
  nushell code in conversation so syntax highlighting works in Zed.
- **Output in scripts**: wrap table pipelines in `print (... | table)`
  so all sections render. Bare `| table` at end of pipeline only works
  for the last expression — intermediate tables need explicit `print`.
### Nushell script library (`nu/`)

All nushell scripts live in `~/git/cyber/nu/`. Scripts are graph-agnostic:
they take the graph path as an argument via `def main [graph_path: string]`.

Usage from any directory:
```
nu ~/git/cyber/nu/stats.nu ~/git/cloud-forest
nu ~/git/cyber/nu/analyze.nu ~/git/cyber
```

Scripts:
- `nu/analyze.nu` — general analytics (files, tags, categories, links, IPFS)
- `nu/stats.nu` — graph statistics (orphans, broken links, content types)
- `nu/migrate.nu` — migrate Logseq format to pure markdown (YAML frontmatter, directories)
- `nu/ipfs.nu` — pre-commit hook: upload assets/ to Pinata IPFS, rewrite URLs in markdown (credentials from `~/.config/cyber/env`)

When adding a new script: place it in `nu/`, accept `graph_path` as first
arg, and update this list.
## Parallel Agents for Graph-Wide Tasks

When a task touches many pages across the graph (bulk tagging, renaming,
formatting fixes), split the work into non-overlapping scopes by filename
or other criteria, then launch several agents in parallel. Before
splitting: enumerate the full file list, partition it into disjoint sets
(e.g. by alphabetical range, by tag, by namespace), and assign each set
to a separate agent. No two agents should ever touch the same file.
## License

Cyber License: Don't trust. Don't fear. Don't beg.