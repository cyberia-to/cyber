---
tags: cyber, page
---
# Plan: Structural Refactoring — Align Repo with Interface

## Context

The project's directory names don't match the interface concepts. The user wants
to align them and merge the `/files` and `/pages` synthetic pages into one
unified `/files` view with a size column.

## Part 1: Directory Renames (filesystem)

Rename on disk (git mv):
- `pages/` → `graph/`
- `journals/` → `blog/`
- `nu/` → `stats/`

## Part 2: Scanner — New Directory Names

**File:** `src/publish/src/scanner/mod.rs`

- `input_dir.join("pages")` → `input_dir.join("graph")`
- `input_dir.join("journals")` → `input_dir.join("blog")`
- Support both old and new names for backwards compatibility:
  try `graph/` first, fall back to `pages/` if it exists
  (same for `blog/` vs `journals/`)
- Update tests to use new directory names

**File:** `src/publish/src/scanner/classify.rs`
- `page_name_from_path` — rename param `pages_dir` → `graph_dir` (cosmetic)

## Part 3: Merge /files and /pages → Unified /files

**The new `/files` page** shows ALL public pages in one table:

| # | file | size | rank | links |
|---|------|------|------|-------|
| 1 | Cyber | 12.3 KB | 4.52 | 89 |
| 2 | nu/analyze.nu | 3.1 KB | 0.01 | 0 |

Columns:
- `#` — row number (sorted by PageRank descending)
- `file` — title + icon + tags (same as current `/pages`)
- `size` — file size on disk (human-readable: KB/MB)
- `rank` — PageRank score
- `links` — backlink count

**File:** `src/publish/src/render/mod.rs`
- Remove `render_pages_index()` function
- Remove the `/pages/index.html` synthetic page generation
- Remove `"pages"` from `reserved_slugs`
- Update `render_files_page()` to use the new unified data

**File:** `src/publish/src/render/mod.rs` — `render_files_page()`
- Instead of calling `output::files::build_file_index()` (which scans markdown
  for media references), iterate `store.public_pages()` directly
- For each page: get file size from `source_path`, compute pagerank and
  backlinks
- Sort by PageRank descending
- Pass to template

**File:** `src/publish/templates/pages-index.html` — DELETE
**File:** `src/publish/templates/files.html` — REWRITE
- Merge the columns from pages-index.html (rank, pagerank, links) with
  the new size column
- Keep the scroll-position tracking JS from pages-index.html

**File:** `src/publish/src/output/files.rs`
- Keep `build_file_index` for `files-index.json` output (API consumers may
  use it), but it's no longer used for the rendered `/files` page

## Part 4: Nav Template

**File:** `src/publish/templates/partials/nav.html`
- Change sidebar builtin link: `/pages` "Pages" → `/files` "Files"

## Part 5: Topics/Graph Merge (design note)

Tags already appear as graph nodes (stub pages created for every tag). The
graph visualization already includes topic connections. No code change needed
now — the graph IS the merged view. Consider adding a "topics overlay" toggle
to the graph JS in a future iteration.

## Part 6: Config and CLAUDE.md

**File:** `publish.toml` — no changes needed (scanner handles fallback)
**File:** `CLAUDE.md` — update references: `pages/` → `graph/`, `journals/`
→ `blog/`, `nu/` → `stats/`, script paths updated

## Files Modified

| File | Change |
|------|--------|
| `src/publish/src/scanner/mod.rs` | `pages/` → `graph/`, `journals/` → `blog/`, fallback support |
| `src/publish/src/scanner/classify.rs` | Rename `pages_dir` param cosmetic |
| `src/publish/src/render/mod.rs` | Remove `render_pages_index`, update `render_files_page` |
| `src/publish/templates/files.html` | Rewrite: unified table with size column |
| `src/publish/templates/pages-index.html` | DELETE |
| `src/publish/templates/partials/nav.html` | `/pages` → `/files` in sidebar |
| `src/publish/src/main.rs` | Update scan log labels |
| `CLAUDE.md` | Update directory references |

## Status: COMPLETED