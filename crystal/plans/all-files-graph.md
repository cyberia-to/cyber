---
tags: cyber, page
---
# Plan: All-Files Graph — Every File Is a Node

## Context

The publisher (`cyber-publish`) currently only builds graph nodes from markdown
files in `pages/` and `journals/`. Non-markdown files (`.nu`, `.rs`, `.py`,
`.toml`, `.zip`, etc.) are invisible to the graph. The user wants every file
in the repository to be a first-class graph node — connected, searchable,
ranked by PageRank, and rendered as a page.

## Scope of Changes

5 files modified, 0 new files.

### 1. Scanner: walk entire repo (`src/publish/src/scanner/mod.rs`)

Current behavior: scans only `pages/`, `journals/`, `media/` subdirectories.
Filters pages by `.md`/`.markdown`/no-extension.

Changes:
- Remove the extension filter from the pages/ scan — accept ALL files in `pages/`
- Add a fourth scan loop: walk entire `input_dir` recursively, collect files
  that are NOT inside `pages/`, `journals/`, or `media/`
- These files get a new `FileKind::File` variant
- `DiscoveredFiles` gets a new field: `files: Vec<DiscoveredFile>`
- Default exclude patterns updated: add `public/*`, `target/*`, `.DS_Store`,
  `*.o`, `*.rmeta`, `*.rlib`, `*.dylib`, `*.d`, `*.timestamp`, `*.bin`,
  `*.lock`, `*.cargo-lock` to skip build artifacts
- Media files in `media/` stay as `FileKind::Media` (still copied to output) —
  but ALSO get a parallel entry in `files` so they become graph nodes too

### 2. Scanner classify: file name helper (`src/publish/src/scanner/classify.rs`)

- Add `file_name_from_path(path, base_dir) -> String` that returns the path
  relative to base_dir WITH extension preserved (unlike `page_name_from_path`
  which strips `.md`)
- For files in `pages/` that are NOT markdown: use filename with extension
  (e.g. `pages/sw-v2.2.2-macos.zip` → `sw-v2.2.2-macos.zip`)
- For files outside `pages/`: relative path from input_dir
  (e.g. `nu/analyze.nu` → `nu/analyze.nu`, `Makefile` → `Makefile`)

### 3. Parser: handle non-markdown files (`src/publish/src/parser/mod.rs`)

- Add `PageKind::File` enum variant
- Update `parse_all` to iterate `discovered.files` and call a new
  `parse_non_md_file` function
- `parse_non_md_file(file: &DiscoveredFile) -> Result<ParsedPage>`:
  - Detect text vs binary: try `read_to_string`; if it fails (invalid UTF-8) →
    binary
  - For text files:
    - Detect language from extension (map: `.rs`→rust, `.nu`→nu, `.py`→python,
      `.toml`→toml, `.yml`/`.yaml`→yaml, `.js`→javascript, `.css`→css,
      `.json`→json, `.sh`→bash, etc.; no extension → plaintext)
    - `content_md` = triple-backtick code fence with language tag wrapping
      the entire file content
    - Try to extract `[[wikilinks]]` from the raw text (reuse
      `wikilinks::collect_wikilinks`)
  - For binary files:
    - `content_md` = file metadata block (extension, size in human-readable
      format)
  - `PageMeta`:
    - `title` = filename (relative path)
    - `tags` = auto-generated from extension (e.g. `["nushell"]` for `.nu`,
      `["rust"]` for `.rs`) + directory-based tag (e.g. `["nu"]` for files in
      `nu/` dir)
    - `public` = `Some(true)` (all files are public by default in the graph)
  - `kind` = `PageKind::File`

### 4. Render: template for file pages (`src/publish/src/render/mod.rs`)

- Add `PageKind::File` match arm in the template selection — use `"page.html"`
  (reuse existing template; the code-fenced content renders fine through comrak)

### 5. Config: update default excludes (`src/publish/src/config.rs`)

- Update `ContentSection::default()` exclude_patterns to include:
  `[".git/*", "logseq/*", "draws/*", "public/*", "target/*", "*.o", "*.rmeta",
   "*.rlib", "*.dylib", "*.d", "*.timestamp", "*.bin", "*.lock",
   "*.cargo-lock", ".DS_Store"]`

## Files to Modify

| File | Change |
|------|--------|
| `src/publish/src/scanner/mod.rs` | Add FileKind::File, new scan loop, expand pages/ scan |
| `src/publish/src/scanner/classify.rs` | Add `file_name_from_path` |
| `src/publish/src/parser/mod.rs` | Add PageKind::File, `parse_non_md_file`, language map |
| `src/publish/src/render/mod.rs` | Add PageKind::File template match |
| `src/publish/src/config.rs` | Update default exclude patterns |

## What Does NOT Change

- Graph module (`graph/`) — works with `ParsedPage` regardless of kind
- Output module (`output/`) — writes HTML for all rendered pages uniformly
- Templates — `page.html` renders any HTML content
- PageRank, backlinks, tag index — all work automatically on new nodes
- Media copying — still copies media/ to output as before

## Verification

1. `cd ~/git/cyber && cargo build -p cyber-publish` — compiles
2. `cargo test -p cyber-publish` — existing tests pass
3. `./target/debug/cyber-publish build .` — builds successfully, prints
   higher page count than before (should include `.nu`, `.rs`, `.toml`, etc.)
4. Check output: `ls public/nu-analyze.nu/index.html` exists
5. Check graph: `cat public/graph-data.json | python3 -c "import sys,json; d=json.load(sys.stdin); print(len(d['nodes']))"` — count is higher than before
6. Open `http://localhost:8080/nu-analyze.nu` — shows code-fenced content of analyze.nu