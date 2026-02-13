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

This Logseq graph is the seed knowledge base for planetary
superintelligence. It contains only the essential concepts needed for
further development and survival — not everything, just the core.

## Tagging Conventions

Every page should have a `tags::` line. Key project tags (lenses):
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
  emphasis use: `property::` for key-value pairs at the start of a line,
  `# heading` for section titles, `[[wiki-link]]` for inline emphasis on
  concepts. If a term does not deserve its own page, it does not need
  emphasis — just write it plain.

## Parallel Agents for Graph-Wide Tasks

When a task touches many pages across the graph (bulk tagging, renaming,
formatting fixes), split the work into non-overlapping scopes by filename
or other criteria, then launch several agents in parallel. Before
splitting: enumerate the full file list, partition it into disjoint sets
(e.g. by alphabetical range, by tag, by namespace), and assign each set
to a separate agent. No two agents should ever touch the same file.

## License

Cyber License: Don't trust. Don't fear. Don't beg.
