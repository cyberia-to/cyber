---
tags: cyber, article
alias: project structure, repo layout, project conventions
crystal-type: pattern
crystal-domain: cyber
---
# project structure

conventions for organizing repositories.

## repo layout

```
project/
├── src/                 implementation source code
├── reference/           canonical specification (source of truth)
│   ├── language.md      core language/API spec
│   ├── grammar.md       formal grammar (if applicable)
│   ├── ir.md            internal representations
│   ├── quality.md       review methodology
│   └── props/           design proposals (draft → accepted → implemented)
├── docs/                documentation (Diataxis framework)
│   ├── tutorials/       learning by building
│   ├── guides/          task-oriented how-tos
│   └── explanation/     conceptual understanding
├── .claude/             agent state (not ephemeral)
│   ├── plans/           design decisions and implementation plans
│   ├── audits/          audit logs and summaries
│   └── other/           performance reports, analysis, findings
├── CLAUDE.md            agent instructions (project-specific)
├── LICENSE.md           license (links to canonical source)
└── README.md            project entry point
```

## namespace conventions

four namespaces partition functionality:

| namespace | purpose |
|-----------|---------|
| vm.* | intrinsics — lowest-level primitives |
| std.* | libraries — reusable standard modules |
| os.* | portable runtime — platform abstraction |
| os.\<platform\>.* | platform-specific implementations |

source code mirrors namespaces as directories.

## reference/ vs docs/

reference/ is specification — defines what the system does. austere,
complete, organized by system structure. this is the source of truth.
when code and reference disagree, fix reference first, then propagate.

docs/ is documentation — teaches, guides, explains. organized by the
reader's needs (Diataxis). references spec but does not duplicate it.

## design proposals

`reference/props/` holds proposals for changes not yet committed to
the spec. each proposal has status frontmatter (draft, accepted,
rejected, implemented). proposals document desire before commitment.
accepted proposals migrate to the relevant reference/ file.
rejected proposals stay for rationale.

## agent state

`.claude/plans/` persists across conversations. plans are committed
to the repo so the user can review them in their editor. budget:
1000 lines total across `.claude/`. merge or delete weak entries
before adding new ones.

## configuration

project configuration lives in a manifest file at the root
(Cargo.toml, package.json, trident.toml, etc.). target-specific
configuration lives alongside the target code, not in the root.

## do not touch zones

every project has files that should not be modified without explicit
discussion: dependency manifests, canonical reference structure,
target configurations, license. document these in CLAUDE.md.

## git workflow

- atomic commits — one logical change per commit
- conventional prefixes: feat:, fix:, refactor:, docs:, test:, chore:
- feature branches for all work, PRs to merge
- branch naming: feat/, fix/, refactor/, docs/, test/, chore/
- test names describe the property, not the method

## companion repos

when a system spans multiple repos, document the relationship in
CLAUDE.md: what each repo does, how they depend on each other,
how to rebuild after changes, and how to reference files across repos.
use repo-qualified paths (e.g. `trident/src/` vs `trisha/src/`).
