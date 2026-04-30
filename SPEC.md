# Org Workspace: `.github` as Workspace Root, Sync as the Engine

## One-sentence goal

Every contributor gets the full cyberia-to workspace from a single clone and a single command, with identical ergonomics for external and internal contributors, while keeping content repos, the build tool, and the workspace machinery each responsible for exactly one thing.

## Onboarding

```
git clone git@github.com:cyberia-to/.github.git ~/cyberia-to/.github
cd ~/cyberia-to/.github
nu scripts/sync-org.nu --apply
nu scripts/serve.nu
```

One clone. One command. The sync step materializes every non-archived org repo the contributor is authenticated to see; the serve step builds the graph with optica and opens a browser.

- External contributor: `gh auth` sees public repos only. Workspace is public-complete. Contributor can PR into any public repo as external.
- Internal contributor: `gh auth` sees everything. Workspace includes private repos and local private builds.

Same script, same layout, same workflow. Auth scope is the only difference.

## Three repos, three responsibilities

| repo | role | knows about |
|---|---|---|
| `cyberia-to/.github` | workspace root — org meta, sync scripts, subgraph declarations, contributor context, build orchestration | the org, every subgraph, which one donates the root namespace |
| `cyberia-to/cyber` | content — a graph that donates the root namespace | itself, its own pages |
| `cyberia-to/optica` | build tool — given a config, produce a site | nothing about orgs, clones, or GitHub |

The root namespace donor is a config choice. Tomorrow another repo could take that role by flipping one line in `workspace.toml`; cyber's content would stay unchanged.

Optica loses all org-awareness. It accepts a generated config listing absolute paths to subgraph roots and renders. Any org that adopts this pattern can reuse optica with its own `.github` repo.

## Layout on disk

```
~/cyberia-to/
├── .github/                         # workspace anchor
│   ├── README.md                    # how to join the workspace
│   ├── profile/README.md            # org profile page (GH convention)
│   ├── CLAUDE.md                    # workspace-level agent instructions
│   ├── workspace.toml               # org config, root graph, build config
│   ├── subgraphs.lock.toml          # reconciler memory
│   ├── scripts/
│   │   ├── sync-org.nu              # the reconciler
│   │   ├── build.nu                 # materializes optica config, invokes optica build
│   │   ├── serve.nu                 # materializes optica config, invokes optica serve
│   │   └── context.nu               # workspace context builder for LLMs
│   ├── subgraphs/                   # one declaration file per org repo
│   │   ├── cyber.md
│   │   ├── trident.md
│   │   ├── hemera.md
│   │   └── ...
│   └── context/                     # shared contributor onboarding context
├── cyber/                           # content — root namespace donor
├── trident/
├── hemera/
├── mudra/                           # private — cloned only with org auth
└── ...
```

Non-org repos never appear in `~/cyberia-to/`. They stay in `~/git/` or wherever the contributor keeps unrelated work.

## Dependency direction

One-way, always:

```
.github → cyber, trident, hemera, ...
         → optica (as a build dep)
```

`.github` knows every subgraph. No subgraph knows about `.github`. Content repos stay portable — any of them can be cloned standalone, read, edited, and PR'd without the workspace layer present. This rule is stated in `.github/CLAUDE.md` so nobody reintroduces org-awareness into content repos.

## Configuration

`.github/workspace.toml`:

```toml
org      = "cyberia-to"
root_dir = "~/cyberia-to"

[graph]
root_subgraph = "cyber"              # donates the root namespace
output        = "build"

[subgraphs.policy]
clone_archived  = false
include_private = true               # local builds; public-only flag excludes

[sync]
lock_file       = "subgraphs.lock.toml"
default_action  = "dry-run"          # --apply required for any mutation
```

`.github/subgraphs/<name>.md` — one file per org repo, declaration frontmatter plus optional hand-written workspace context:

```markdown
---
name: trident
repo: trident                        # resolved relative to root_dir
visibility: public                   # reconciler writes this from gh api
archived: false                      # reconciler writes this
---
# trident

field-native language. proof-grade execution. companion to nox.
```

The declaration file's frontmatter holds reconciler-scoped keys. The body is human-owned and never touched by the reconciler.

## Layers and authority

| layer | authoritative for | reconciler access |
|---|---|---|
| GitHub org | existence, visibility, archived, canonical name | read-only (via `gh api`) |
| filesystem `<root_dir>/<name>/` | actual repo contents | fetch-only (`git fetch`) |
| `.github/subgraphs.lock.toml` | last-seen org snapshot, transition memory | read/write (committed) |
| `.github/subgraphs/<name>.md` | declaration frontmatter + human context | read/write scoped keys only |

Scoped frontmatter keys the reconciler may write or clear: `visibility`, `archived`, `orphan`, `renamed-from`, `local-only`. Every other frontmatter field and the entire body are human-owned.

## Principles

1. The GitHub org is the scope boundary and source of truth for membership, visibility, archive state, canonical name.
2. Local clones are a cache. The reconciler never touches working trees or pushes to the org.
3. Declaration files are authoritative for graph inclusion. The reconciler writes scoped keys only.
4. Every run is idempotent. Every mutation requires `--apply`. Destructive-adjacent operations require a second opt-in.
5. Never delete. Orphans are reported, never removed.
6. Optica knows nothing about the org. Org-awareness lives in `.github/scripts/` only.

## Lock file format

`.github/subgraphs.lock.toml` — generated, committed, reviewable:

```toml
schema    = 1
org       = "cyberia-to"
synced_at = "2026-04-23T14:22:00Z"

[repos.trident]
visibility     = "public"
archived       = false
default_branch = "main"
last_seen      = "2026-04-23T14:22:00Z"

[repos.mudra]
visibility     = "private"
archived       = false
default_branch = "main"
last_seen      = "2026-04-23T14:22:00Z"

[orphans.old-name]
last_seen_visibility = "public"
disappeared_at       = "2026-04-17T09:10:00Z"
possible_rename_to   = "new-name"    # present when gh 301 redirect resolved
```

The lock is the memory between runs. Without it, rename and delete-then-add are indistinguishable.

## Folder classification

Every directory in `<root_dir>/` is classified on each run:

| filesystem state | org state | class | action |
|---|---|---|---|
| `.git`, remote = `<org>/<name>`, name matches | exists, matches | tracked | `git fetch`; reconcile flags |
| `.git`, remote = `<org>/<name>`, name differs | exists under new name | renamed | emit migration plan; no action without `--apply-renames` |
| `.git`, remote = `<org>/<name>` | not in org, no redirect | orphan | report; never delete |
| `.git`, remote elsewhere | — | foreign | report as misfiled |
| plain directory | org has same name, not yet cloned | shadow conflict | hard error; halt run |
| plain directory | no collision | local-only subgraph | include only if declaration has `local-only: true` |
| missing | exists, non-archived | to-clone | clone on `--apply` |
| missing | archived | skip | do not clone archived repos by default |

The shadow conflict case is the safety cliff: a hand-created scratch folder must never silently bind to an org repo's name. The run halts until the contributor renames the folder or clones explicitly.

## State transitions

Transitions are computed by diffing `(org state, lock state, filesystem state, declaration state)`.

| event | detection | reconciler action | requires |
|---|---|---|---|
| add | in org, not in lock, no folder | clone to `<root_dir>/<name>`, create declaration stub, write lock entry | `--apply` |
| adopt | in org, not in lock, folder present and matches | write lock entry, create declaration stub if missing | `--apply` |
| fetch | in lock + org, name matches | `git fetch` only | `--apply` |
| archive | org `archived: true`, declaration lacks flag | set `archived: true` in declaration | `--apply` |
| unarchive | org `archived: false`, declaration has flag | remove `archived:` from declaration | `--apply` |
| publish | org visibility `public`, declaration has `visibility: private` | set `visibility: public` | `--apply` |
| privatize | org visibility `private`, declaration has `visibility: public` | set `visibility: private` | `--apply` |
| rename | folder or declaration name ≠ org canonical; `gh api /repos/<org>/<old>` returns 301 | emit rename plan (folder mv, declaration rename, wiki-link scan, `renamed-from` stamp) | `--apply-renames` |
| delete | in lock, not in org, no redirect | mark `orphans.<name>` in lock; set `archived: true, orphan: true` in declaration | `--apply` |
| return | previously orphaned, reappears in org | clear orphan flag, resume normal handling | `--apply` |

Declaration stub template (written only when the file is missing):

```markdown
---
name: <name>
repo: <name>
visibility: <public|private>
archived: <false|true>
---
# <name>

<!-- auto-generated stub. replace freely with human-written context. -->
```

## Safety invariants

1. `rm -rf` appears nowhere in the script. No clone, no declaration, no directory is ever removed.
2. Working trees are never modified. Only `git fetch`. A contributor decides when to `git pull`, `git merge`, or `git rebase`.
3. Frontmatter writes are key-scoped. The reconciler uses a YAML edit that preserves key order, comments, and unknown fields. A diff on a modified declaration shows only scoped keys changed.
4. Declaration stubs are write-once. If `.github/subgraphs/<name>.md` exists, the script edits scoped keys only; bodies stay untouched.
5. Plan before apply. `--dry-run` is the default. The plan output lists every intended mutation grouped by category, with file paths and before/after values.
6. Destructive-adjacent operations have their own gate. `--apply` will not execute a rename. `--apply-renames` is additional and explicit.
7. The lock file is committed. Every transition is a git diff a human can review before merging.

## Rename handling

Rename is the only transition that touches wiki-links, and wiki-link edits are the only edits that can silently corrupt cross-page meaning. The rename plan has four steps, all emitted together, applied only on `--apply-renames`:

1. Move the clone directory: `mv <root_dir>/<old> <root_dir>/<new>`.
2. Rename the declaration: `git mv .github/subgraphs/<old>.md .github/subgraphs/<new>.md`.
3. Update `name:` and `repo:` to `<new>` in the declaration. Add `renamed-from: <old>` and `alias: <old>` so `[[<old>]]` still resolves in the graph.
4. Emit a wiki-link impact report: every graph page containing `[[<old>]]` is listed. The script does not rewrite links. The contributor reviews and decides per-link, since `[[<old>]]` may intentionally reference the historical name.

The `renamed-from` + `alias` approach keeps the graph linkable through the transition window. A follow-up housekeeping pass rewrites `[[<old>]]` to `[[<new>]]` where appropriate, then removes the alias.

## Orphan handling

An orphan is a tracked repo that disappeared from the org (deleted or transferred out). Action:

1. Move the lock entry from `[repos.<name>]` to `[orphans.<name>]` with `disappeared_at`.
2. Set `archived: true, orphan: true` in the declaration.
3. Leave the clone directory untouched. A contributor may have uncommitted work there.
4. Emit a report listing the orphan and the wiki-link count referring to it.

Orphans stay in the build (with `orphan: true` for styling) until a contributor explicitly removes the declaration. The reconciler never removes declarations.

## Optica changes

Optica stops reading org or GitHub concepts. It accepts a generated config and renders.

1. New flag: `optica build --config <path>`. The config is a TOML or JSON document listing the root graph path, the output directory, and an array of subgraph entries `{name, path, public, archived}`. Paths are absolute.
2. Remove `[subgraphs]` reading from `publish.toml` in content repos. Optica no longer resolves subgraph paths from frontmatter across the graph.
3. Remove auto-discovery of sibling directories. Optica includes only subgraphs explicitly listed in the config.
4. Keep `archived:` and `orphan:` skip at the declaration level — materialized into the config by `build.nu`, not read from org state.
5. `--public-only` becomes a flag on `build.nu`, not optica. `build.nu` filters the subgraph list before passing it to optica.

Net change to optica: smaller, more focused, reusable by any org.

## Build orchestration

`.github/scripts/build.nu`:

```nu
def main [--public-only] {
    let ws = (open workspace.toml)
    let decls = (glob subgraphs/*.md | each { open --raw $in | parse-frontmatter })

    let subgraphs = ($decls
        | where archived != true
        | where orphan != true
        | where (not $public_only) or (visibility != "private" and local-only != true)
        | each {|d| {name: $d.name, path: ($ws.root_dir | path join $d.repo)} })

    let config = {
        root_graph: ($ws.root_dir | path join $ws.graph.root_subgraph),
        output:     $ws.graph.output,
        subgraphs:  $subgraphs
    }

    $config | save --force /tmp/optica-config.json
    ^optica build --config /tmp/optica-config.json
}
```

The script is the translation layer between the workspace model and optica's pure build model.

## Command interface

All commands run from `.github/`:

```
nu scripts/sync-org.nu                    # dry-run; prints plan
nu scripts/sync-org.nu --apply            # applies adds, fetches, flag flips, orphan marks
nu scripts/sync-org.nu --apply-renames    # applies rename plans in addition
nu scripts/sync-org.nu --json             # machine-readable plan for CI
nu scripts/sync-org.nu --only <repo>      # scope to one repo

nu scripts/build.nu                       # full local build
nu scripts/build.nu --public-only         # public-safe build for deploy

nu scripts/serve.nu                       # build + optica serve
```

Exit codes:

| code | meaning |
|---|---|
| 0 | clean — no transitions needed |
| 1 | plan emitted — human review required before apply |
| 2 | applied successfully |
| 10 | shadow conflict — halted |
| 11 | foreign folder — halted if in-scope |
| 20 | auth failure (`gh` not authenticated, insufficient scopes) |

## CI integration

A scheduled workflow in `.github`:

```yaml
# .github/workflows/sync-and-publish.yml
on:
  schedule: [{ cron: "0 6 * * *" }]
  workflow_dispatch:
jobs:
  sync-and-publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with: { path: .github }
      - name: Install nu + optica
        run: |
          curl -L <nu-release> | tar -xz
          curl -L <optica-release> | tar -xz
      - name: Sync org state
        run: cd .github && nu scripts/sync-org.nu --apply --json > /tmp/plan.json
        env: { GH_TOKEN: ${{ secrets.GH_TOKEN }} }
      - name: Commit lock drift
        run: |
          cd .github
          if [ -n "$(git status --porcelain)" ]; then
            git config user.name "cyber-bot"
            git config user.email "bot@cyber.page"
            git add subgraphs.lock.toml subgraphs/
            git commit -m "chore: sync subgraph state $(date -u +%Y-%m-%d)"
            git push
          fi
      - name: Public build
        run: cd .github && nu scripts/build.nu --public-only
      - name: Deploy
        run: netlify deploy --dir=.github/build --prod
```

CI never clones private org repos; it syncs metadata only. Public builds use `--public-only`, so private and local-only repos stay absent regardless of runner auth.

## Migration from the current state

The current state:
- Subgraph declarations live in `cyber/root/*.md` with `subgraph: true, repo: ../<name>`.
- Clones live flat in `~/git/`.
- No `.github` repo yet.

Migration is phased:

Phase 1 — create `.github` skeleton:
- Bootstrap `cyberia-to/.github` with README, CLAUDE.md, workspace.toml, empty scripts/ and subgraphs/.
- Write `sync-org.nu` in dry-run-only mode.

Phase 2 — move declarations:
- For each `cyber/root/<name>.md` with `subgraph: true`, split:
  - Keep the content page in cyber (strip `subgraph:` and `repo:` keys).
  - Create `.github/subgraphs/<name>.md` with the declaration frontmatter and a short hand-written workspace-context paragraph.
- Optica config change ships in the same PR so builds keep working.

Phase 3 — layout migration:
- `sync-org.nu --apply-migration` moves `~/git/<name>/` → `~/cyberia-to/<name>/` for every org repo. Non-org repos stay in `~/git/`.
- One-time, scripted, reviewable.

Phase 4 — CI cutover:
- Replace cyber's current build/deploy workflow with `.github`'s.
- Cyber's CI reduces to content checks (lint, link-check).

Each phase is independently valuable and reversible.

## Deliverables

Three reviewable PRs:

1. optica: add `--config <path>`, remove auto-discovery and `[subgraphs]` reading. Pure subtraction; optica gets simpler.
2. `cyberia-to/.github`: workspace.toml, sync-org.nu, build.nu, serve.nu, initial subgraphs/, initial lock. Sync runs in dry-run until every current cyber/root declaration has been ported.
3. cyber: strip subgraph declarations from root/*.md; update CLAUDE.md to point contributors at `.github`; update README.

## Non-goals

1. The sync script does not push. It never edits org state.
2. The sync script does not resolve merge conflicts. Working trees are contributor-owned.
3. The sync script does not enforce a canonical branch. `default_branch` is recorded in the lock for auditability; checkout stays with the contributor.
4. The sync script does not mirror org-level settings (topics, descriptions, protection rules). Those live on GitHub.
5. Optica does not read anything about orgs, GitHub, cloning, or authentication. Ever.

## Open questions

1. Per-subgraph hand-written context: `.github/subgraphs/<name>.md` (central, workspace-level) or also inside each subgraph's own README/CLAUDE.md (distributed, repo-level)? Recommendation: both, with different purposes. The `.github` file describes the subgraph's role in the workspace; the repo's own README describes the repo standalone.
2. Does `.github` appear as a subgraph in the graph itself? Recommendation: yes. Its scripts, spec, and context become navigable graph nodes, and `.github` becomes self-describing.
3. Archive policy on `git fetch`: fetch archived repos once at the archive transition, then stop? Recommendation: stop. Archived repos are frozen at the last-seen commit.
4. Wiki-link auto-rewrite on rename: add `--apply-link-rewrite` or leave it manual? Recommendation: manual for the first release. Add the flag only after real rename cases teach us the pattern.
