# Claude Code Instructions

## Default Graph

The default Logseq graph is `~/git/cyber`. Always use it unless the user
specifies a different path. The graph config (`logseq-publish.toml`) sets
port 8888 via `base_url`, so no `--port` flag is needed.

## Running the Server

```
cargo run -- serve ~/git/cyber --open
```

This serves on **http://localhost:8888** (port from config `base_url`).
Never use port 8080 — it is reserved for other services.

## Project

logseq-publish is a Rust static site generator for Logseq graphs.
It reads a Logseq graph directory and produces a fast, SEO-friendly
static website.

## Commands

- `cargo run -- serve <graph_path>` — build + dev server with live reload (port from config)
- `cargo run -- build <graph_path>` — generate static site
- `cargo run -- check <graph_path>` — validate graph, find broken links
- `cargo run -- init` — create default config

## Commits

Always commit after completing a task. Each commit must be atomic with a
clean, concise feature description. Do NOT commit only if:
- You still have unresolved questions about the feature
- The user explicitly asked not to commit
