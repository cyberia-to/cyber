# logseq-publish: A Rust-native Logseq Static Site Publisher

## 1. Purpose

**logseq-publish** is a single-binary Rust tool that reads a Logseq graph directory and produces a fast, beautiful, SEO-friendly static website. It replaces the current Quartz (Node.js) pipeline with a zero-dependency, purpose-built publisher that natively understands Logseq's outliner markdown, wikilinks, block references, properties, and queries.

### Why this exists

| Problem with current stack | How logseq-publish solves it |
|---|---|
| Quartz requires Node.js + npm + heavy `node_modules` | Single Rust binary, `cargo install logseq-publish` |
| Quartz doesn't natively understand Logseq's outliner format | Purpose-built Logseq markdown parser with AST transforms |
| Logseq queries are ignored by generic SSGs | Query snapshots evaluated at build time |
| Redesign needed anyway | Full control over templates and design |
| Slow builds on large graphs | Rust-native speed, sub-second builds for typical graphs |

### What it is NOT

- Not a general-purpose SSG (use Zola/Hugo for that)
- Not a Logseq replacement or editor
- Not attempting to run Datalog at build time — queries are pre-evaluated as static snapshots

---

## 2. Rationale: Why Compose, Not Adopt

### Why not Zola?

- Zola is a monolithic binary, not usable as a library
- No wikilink support, no backlinks (still in progress after 4+ years)
- Would require a separate Logseq→Zola transformer writing intermediate markdown to disk
- Template data model doesn't fit a knowledge graph (designed for blogs/docs)

### Why not Marmite?

- Closer fit (has wikilinks + backlinks), but also a monolithic binary
- Flat URL structure only (`{name}.html`), no hierarchical namespaces
- Young project (800 stars), opinionated toward blogs
- No Logseq-specific understanding (outliner bullets, properties, block refs)

### Why compose from crates?

- **comrak** already has a `WikiLink` AST node type — native support for `[[links]]`
- **comrak** provides a mutable AST (backed by `typed_arena` + `RefCell`) — we can walk, transform, and re-render
- **minijinja** is lighter than Tera, authored by the creator of Jinja2 itself, minimal deps (only serde)
- Total control over the two-pass build (graph construction → rendering with backlinks)
- Logseq-specific transforms live in our code, not hacked into an SSG's plugin system
- Dependency tree stays small: `comrak + minijinja + syntect + notify + serde` 

---

## 3. Scope

### In Scope (MVP — Phase 1)

- [x] Read Logseq graph directory (`pages/`, `journals/`, `assets/`)
- [x] Parse Logseq outliner markdown (nested `- ` bullets → structured content)
- [x] Parse `property:: value` front matter into structured metadata
- [x] Resolve `[[wikilinks]]` to internal page links
- [x] Build backlink index (two-pass: collect links, then inject backlinks)
- [x] Render pages to HTML via minijinja templates
- [x] Syntax highlighting via syntect (comrak plugin)
- [x] Copy/reference assets
- [x] Generate RSS feed
- [x] Generate sitemap.xml
- [x] Built-in dev server with live reload
- [x] CLI: `logseq-publish build`, `logseq-publish serve`, `logseq-publish init`
- [x] Selective export (public/private filtering via page properties)
- [x] Namespace support (`parent/child` pages → URL hierarchy)

### In Scope (Phase 2)

- [ ] `((block-reference))` resolution and embedding
- [ ] `{{embed [[page]]}}` page transclusion
- [ ] Query engine: parse simple/advanced queries, evaluate against PageStore, render results (see section 6.5)
- [ ] Search index generation (JSON index for client-side search, or Pagefind integration)
- [ ] Graph data export (JSON adjacency list for visualization)
- [ ] Image optimization (resize, WebP conversion)
- [ ] Incremental builds (only rebuild changed pages)

### In Scope (Phase 3)

- [ ] Theme system (bundled default + custom theme directory)
- [ ] Table of contents generation per page
- [ ] Admonition blocks (`#+BEGIN_NOTE`, etc.)
- [ ] Plausible analytics integration (inject script via config, privacy-respecting)

### In Scope (Phase 4 — UX & Visual Polish)

- [ ] Default design system: clean, modern CSS with CSS custom properties for theming
- [ ] Graph visualization page: interactive force-directed graph (D3.js) rendered from link data
- [ ] Graph minimap widget: per-page local neighborhood graph (1-2 hop radius)
- [ ] Page hover previews: tooltip showing first paragraph on internal link hover
- [ ] Smooth transitions between pages (optional, CSS-only or minimal JS)
- [ ] Mobile-responsive layout with collapsible sidebar navigation
- [ ] Reading progress indicator
- [ ] Breadcrumb navigation for namespace hierarchies
- [ ] Tag cloud / tag explorer page
- [ ] Dark/light mode with system preference detection and manual toggle

### Out of Scope

- Full Datalog engine embedding (unnecessary — see section 6.5 for how queries render without it)
- Multi-language / LLM translation (future separate project — compile-time LLM translation is interesting but adds API dependencies, caching complexity, and sync logic that warrants its own tool)
- Real-time collaboration
- CMS / editing interface
- Logseq plugin API compatibility
- PDF export per page

---

## 4. Architecture

### 4.1 High-Level Pipeline

```
┌─────────────────┐
│  Logseq Graph    │
│  /pages/*.md     │
│  /journals/*.md  │
│  /assets/*       │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  1. SCAN         │  Walk directory, discover all .md files
│     Discovery    │  Categorize: page / journal / asset
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  2. PARSE        │  For each .md file:
│     Extract      │  - Strip/transform outliner bullets
│                  │  - Extract property:: values → PageMeta
│                  │  - Parse markdown → comrak AST
│                  │  - Collect [[wikilinks]], ((block-refs))
│                  │  - Store raw AST + metadata in PageStore
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  3. GRAPH        │  Build the knowledge graph:
│     Link Index   │  - Forward links: page → [linked pages]
│                  │  - Backlinks:     page → [pages linking here]
│                  │  - Tag index:     tag  → [pages with tag]
│                  │  - Namespace tree: parent/child hierarchy
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  4. QUERY        │  Evaluate query blocks:
│     Engine       │  - Find {{query}} and #+BEGIN_QUERY blocks
│                  │  - Parse query expression → QueryExpr AST
│                  │  - Evaluate against PageStore (set operations)
│                  │  - Replace query blocks with rendered HTML results
│                  │  - Unrecognized patterns → styled fallback
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  5. RENDER       │  For each page:
│     Transform    │  - Transform AST: resolve wikilinks → <a href>
│                  │  - Inject backlinks data into template context
│                  │  - Render AST → HTML body via comrak
│                  │  - Wrap in minijinja template with full context
│                  │  - Write to output directory
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  6. FINALIZE     │  - Generate RSS feed (rss crate)
│     Output       │  - Generate sitemap.xml
│                  │  - Copy static assets
│                  │  - Build search index (JSON)
│                  │  - Write to /public or configured output dir
└─────────────────┘
```

### 4.2 Module Structure

```
logseq-publish/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point (clap)
│   ├── lib.rs               # Library root, re-exports
│   │
│   ├── config.rs            # Site configuration (logseq-publish.toml)
│   │
│   ├── scanner/
│   │   ├── mod.rs           # Directory walker, file discovery
│   │   └── classify.rs      # Classify files: page/journal/asset/config
│   │
│   ├── parser/
│   │   ├── mod.rs           # Orchestrates parsing pipeline
│   │   ├── outliner.rs      # Logseq outliner bullets → normalized markdown
│   │   ├── properties.rs    # property:: value extraction → PageMeta struct
│   │   ├── wikilinks.rs     # [[wikilink]] detection and collection  
│   │   ├── block_refs.rs    # ((block-reference)) detection (Phase 2)
│   │   └── queries.rs       # #+BEGIN_QUERY block detection and extraction
│   │
│   ├── graph/
│   │   ├── mod.rs           # PageStore: HashMap<PageId, Page>
│   │   ├── links.rs         # Forward link index + backlink index
│   │   ├── tags.rs          # Tag taxonomy index
│   │   └── namespaces.rs    # Namespace hierarchy tree
│   │
│   ├── query/
│   │   ├── mod.rs           # Query engine orchestrator
│   │   ├── detect.rs        # Find query blocks in markdown source
│   │   ├── parse.rs         # Parse query syntax → QueryExpr AST
│   │   ├── eval.rs          # Evaluate QueryExpr against PageStore (set ops)
│   │   └── render.rs        # Query results → HTML table/list
│   │
│   ├── render/
│   │   ├── mod.rs           # Orchestrates rendering pipeline
│   │   ├── transform.rs     # AST transforms (wikilinks→hrefs, etc.)
│   │   ├── templates.rs     # MiniJinja environment setup + helpers
│   │   ├── context.rs       # Build template context per page
│   │   └── highlight.rs     # Syntect integration for code blocks
│   │
│   ├── output/
│   │   ├── mod.rs           # Write files to output directory
│   │   ├── feed.rs          # RSS/Atom feed generation
│   │   ├── sitemap.rs       # sitemap.xml generation
│   │   ├── search.rs        # Search index (JSON) generation
│   │   └── assets.rs        # Asset copying/optimization
│   │
│   └── server/
│       ├── mod.rs           # Dev server with file watching
│       └── reload.rs        # Live reload WebSocket injection
│
├── templates/                # Default bundled templates
│   ├── base.html            # Base layout with <head>, nav, footer
│   ├── page.html            # Single page template
│   ├── journal.html         # Journal entry template  
│   ├── tag.html             # Tag listing page
│   ├── index.html           # Home page / recent pages
│   ├── search.html          # Search page
│   ├── graph.html           # Graph visualization page (Phase 2)
│   └── partials/
│       ├── backlinks.html   # Backlinks section partial
│       ├── nav.html         # Navigation partial
│       ├── page_meta.html   # Properties display partial
│       └── toc.html         # Table of contents partial
│
├── static/                   # Default static assets
│   ├── style.css            # Default stylesheet
│   ├── search.js            # Client-side search (lightweight)
│   └── graph.js             # Graph visualization (Phase 2, optional)
│
└── tests/
    ├── fixtures/             # Sample Logseq graph for testing
    │   ├── pages/
    │   │   ├── Test Page.md
    │   │   ├── Another Page.md
    │   │   └── namespace∕child.md
    │   ├── journals/
    │   │   └── 2025_02_08.md
    │   └── assets/
    │       └── test-image.png
    ├── test_parser.rs
    ├── test_graph.rs
    ├── test_render.rs
    └── integration.rs
```

### 4.3 Key Data Structures

```rust
/// Unique identifier for a page (normalized from filename)
type PageId = String;  // e.g., "collective-focus-theorem" or "2025-02-08"

/// Unique identifier for a block (Logseq UUID)
type BlockId = String; // e.g., "65a1b2c3-d4e5-..."

/// Extracted from property:: value lines at the top of a page
#[derive(Debug, Clone, Serialize)]
struct PageMeta {
    title: String,
    // All properties as key-value pairs
    properties: HashMap<String, String>,
    // Common extracted properties
    tags: Vec<String>,
    public: bool,          // public:: true
    aliases: Vec<String>,  // alias:: name1, name2
    date: Option<NaiveDate>,
    icon: Option<String>,
}

/// A fully parsed page ready for graph construction
#[derive(Debug)]
struct Page {
    id: PageId,
    meta: PageMeta,
    kind: PageKind,            // Page | Journal
    source_path: PathBuf,
    namespace: Option<String>, // e.g., "projects" for "projects/cyber-valley"
    
    // Raw markdown (after outliner normalization, before AST)
    content_md: String,
    
    // Links discovered during parsing
    outgoing_links: Vec<PageId>,    // [[wikilinks]] found
    block_refs: Vec<BlockId>,       // ((block-refs)) found  
    
    // Populated during graph phase
    backlinks: Vec<PageId>,         // Pages that link TO this page
    
    // Blocks with UUIDs (for block reference resolution)
    blocks: Vec<Block>,
}

#[derive(Debug)]
struct Block {
    id: BlockId,
    content: String,
    depth: usize,           // Nesting level in outliner
    children: Vec<BlockId>,
    properties: HashMap<String, String>,
}

/// The complete knowledge graph
struct PageStore {
    pages: HashMap<PageId, Page>,
    
    // Indices built during graph phase
    forward_links: HashMap<PageId, Vec<PageId>>,
    backlinks: HashMap<PageId, Vec<PageId>>,
    tag_index: HashMap<String, Vec<PageId>>,
    namespace_tree: HashMap<String, Vec<PageId>>,  // parent → children
    block_index: HashMap<BlockId, (PageId, usize)>, // block → (page, index)
}

/// Template context passed to MiniJinja for each page render
#[derive(Serialize)]
struct PageContext<'a> {
    site: &'a SiteConfig,
    page: PageRenderData<'a>,
    backlinks: Vec<BacklinkEntry<'a>>,
    nav: NavigationData<'a>,
}

#[derive(Serialize)]
struct PageRenderData<'a> {
    title: &'a str,
    html_content: String,        // Rendered HTML body
    meta: &'a PageMeta,
    url: String,                 // Canonical URL path
    namespace: Option<&'a str>,
    children: Vec<&'a str>,      // Child pages in namespace
    word_count: usize,
    reading_time_minutes: usize,
    date: Option<String>,
}

#[derive(Serialize)]
struct BacklinkEntry<'a> {
    title: &'a str,
    url: String,
    context: String,  // Snippet of text around the link
}
```

---

## 5. Core Dependencies

```toml
[package]
name = "logseq-publish"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
description = "A fast, Rust-native static site publisher for Logseq knowledge graphs"
license = "MIT"
repository = "https://github.com/cybercongress/logseq-publish"
homepage = "https://github.com/cybercongress/logseq-publish"
readme = "README.md"
keywords = ["logseq", "static-site-generator", "markdown", "knowledge-graph", "wiki"]
categories = ["command-line-utilities", "web-programming", "text-processing"]
include = [
    "src/**/*",
    "templates/**/*",
    "static/**/*",
    "Cargo.toml",
    "Cargo.lock",
    "README.md",
    "LICENSE",
]

[dependencies]
# Markdown parsing with AST access (has native WikiLink node!)
comrak = { version = "0.49", default-features = false, features = [
    "syntect",    # syntax highlighting plugin
    "shortcodes", # emoji shortcodes  
] }

# Templating — Jinja2-compatible, minimal deps
minijinja = { version = "2.15", features = ["loader"] }
minijinja-autoreload = "2.15"    # Hot reload for dev mode

# Serialization (required by minijinja, used everywhere)
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"    # For config file
toml = "0.8"           # For config file alternative

# CLI
clap = { version = "4", features = ["derive"] }

# File watching for dev server
notify = "7"
notify-debouncer-mini = "0.5"

# Dev server
tiny_http = "0.12"

# RSS feed generation
rss = "2.0"

# Date/time handling
chrono = { version = "0.4", features = ["serde"] }

# URL/path handling
slug = "0.1"

# Regex for Logseq-specific parsing patterns
regex = "1"
lazy_static = "1"

# Parallel iteration for build speed
rayon = "1.10"

# Colored terminal output
colored = "2"

# Error handling
anyhow = "1"
thiserror = "2"

# Directory walking
walkdir = "2"

# Live reload WebSocket (lightweight)
tungstenite = "0.24"

[dev-dependencies]
tempfile = "3"
pretty_assertions = "1"
insta = "1"       # Snapshot testing for HTML output
```

### Why each dependency

| Crate | Role | Why this one |
|---|---|---|
| **comrak** | Markdown → AST → HTML | Has native `NodeValue::WikiLink` type, mutable AST via arena, syntect plugin built-in. Used by crates.io, docs.rs, GitLab. |
| **minijinja** | HTML templating | Jinja2-compatible by its original creator (Armin Ronacher). Single dep (serde). Template inheritance, filters, macros. Lighter than Tera. |
| **syntect** | Syntax highlighting | Industry standard for Rust. Sublime Text syntaxes. Comes via comrak feature flag. |
| **clap** | CLI parsing | De facto standard. Derive macros for clean code. |
| **notify** | File system watching | Standard Rust file watcher. Cross-platform. |
| **tiny_http** | Dev server | Minimal, no async runtime needed. Perfect for static file serving. |
| **rayon** | Parallelism | Data-parallel iteration. Parse/render pages in parallel. Drop-in `.par_iter()`. |
| **comrak (WikiLink)** | `[[wikilinks]]` | comrak natively parses `[[link]]` and `[[link\|title]]` into `NodeValue::WikiLink` AST nodes. No regex hacks needed. |

---

## 6. Logseq Markdown Specifics

### 6.1 Outliner Normalization

Logseq stores everything as nested bullet lists. The parser must transform this into readable content.

**Input (Logseq markdown):**
```markdown
title:: Collective Focus Theorem
tags:: research, mathematics
public:: true

- This is the introduction to the theorem.
- The core principle states that:
  - Consensus emergence in distributed systems follows predictable patterns
  - These patterns can be modeled mathematically
    - Using graph theory and information theory
- ## Applications
  - [[Bostrom]] network uses this for GPU consensus
  - Biological systems like [[mycorrhizal networks]] exhibit similar behavior
```

**Output (normalized markdown for comrak):**
```markdown
This is the introduction to the theorem.

The core principle states that:

- Consensus emergence in distributed systems follows predictable patterns
- These patterns can be modeled mathematically
  - Using graph theory and information theory

## Applications

- [[Bostrom]] network uses this for GPU consensus
- Biological systems like [[mycorrhizal networks]] exhibit similar behavior
```

**Rules:**
1. Top-level `- ` bullets with no sub-bullets → paragraphs (strip the `- `)
2. Top-level `- ` bullets with sub-bullets → keep as list structure
3. `- ## Heading` → promote to actual heading (strip `- `)
4. Properties block at top → extracted into `PageMeta`, removed from content
5. Indentation depth tracked for block hierarchy

### 6.2 Property Extraction

```
property:: value        →  meta.properties["property"] = "value"
tags:: a, b, c          →  meta.tags = ["a", "b", "c"]  
public:: true           →  meta.public = true
alias:: name1, name2    →  meta.aliases = ["name1", "name2"]
icon:: 🧬              →  meta.icon = Some("🧬")
```

### 6.3 WikiLink Resolution

comrak parses `[[Target Page]]` into `NodeValue::WikiLink { url: "Target Page" }`.

The transform step resolves this to:
```html
<a href="/target-page" class="internal-link" data-page="target-page">Target Page</a>
```

For `[[Target Page|display text]]`:
```html
<a href="/target-page" class="internal-link" data-page="target-page">display text</a>
```

Unresolved links (page doesn't exist):
```html
<a href="/target-page" class="internal-link broken-link" data-page="target-page">Target Page</a>
```

### 6.4 Block References (Phase 2)

```
((65a1b2c3-d4e5-...))
```
→ Look up block UUID in `block_index` → inline the block content or link to it.

### 6.5 Query Rendering Without Datalog

**The core insight:** Logseq queries look like they need a Datalog engine, but 90%+ of real-world queries are just filtering pages/blocks by tags, properties, and link relationships — data we already have in `PageStore` after the graph phase.

#### What Logseq queries actually are

Logseq has two query syntaxes:

**Simple queries** (most common in practice):
```
{{query [[tag1]] AND [[tag2]]}}
{{query (and [[research]] [[mathematics]])}}
{{query (or [[todo]] [[doing]])}}
{{query (property status active)}}
{{query (between -7d today)}}
{{query (page-tags [[research]])}}
```

**Advanced queries** (Datalog syntax, less common):
```clojure
#+BEGIN_QUERY
{:title "Pages tagged research"
 :query [:find (pull ?p [*])
         :where [?p :block/tags ?t]
                [?t :block/name "research"]]}
#+END_QUERY
```

#### How we evaluate them

Both query types ultimately ask questions that reduce to operations on our `PageStore`:

| Query pattern | What it asks | How PageStore answers it |
|---|---|---|
| `[[tag]]` | Pages/blocks with this tag | `tag_index.get("tag")` |
| `(and [[a]] [[b]])` | Pages with both tags | `tag_index.get("a") ∩ tag_index.get("b")` |
| `(or [[a]] [[b]])` | Pages with either tag | `tag_index.get("a") ∪ tag_index.get("b")` |
| `(not [[a]])` | Pages without tag | `all_pages - tag_index.get("a")` |
| `(property key val)` | Pages where property matches | `pages.filter(\|p\| p.meta.properties["key"] == "val")` |
| `(page-tags [[t]])` | Pages tagged with t | Same as `tag_index.get("t")` |
| `(between -7d today)` | Journal pages in date range | `journals.filter(\|j\| j.date >= start && j.date <= end)` |
| `(namespace X)` | Pages under namespace X | `namespace_tree.get("X")` |
| `(page-property key)` | Pages that have property key (any value) | `pages.filter(\|p\| p.meta.properties.contains_key("key"))` |
| `(sort-by property)` | Sort results | `.sort_by(\|a, b\| a.meta.properties[key].cmp(...))` |

Even the "advanced" Datalog queries in practice ask the same questions — the Datalog syntax is just a more formal way to express tag/property/link filters. The common `:where` clauses map directly:

```clojure
;; "find pages tagged research" — this is just tag_index lookup
[?p :block/tags ?t]
[?t :block/name "research"]

;; "find pages with property status = active" — property filter
[?p :block/properties ?props]
[(get ?props :status) ?s]
[(= ?s "active")]

;; "find pages linking to X" — backlink lookup
[?b :block/refs ?p]
[?p :block/name "X"]
```

#### The query evaluation pipeline

```
1. DETECT   — During parse phase, find query blocks in markdown
              (regex for {{query ...}} and #+BEGIN_QUERY...#+END_QUERY)

2. PARSE    — Extract the query expression into a QueryAST:
              enum QueryExpr {
                  Tag(String),
                  And(Vec<QueryExpr>),
                  Or(Vec<QueryExpr>),
                  Not(Box<QueryExpr>),
                  Property { key: String, value: Option<String> },
                  Between { start: DateOffset, end: DateOffset },
                  Namespace(String),
                  Task(Vec<TaskStatus>),
                  SortBy { field: String, direction: SortDir },
              }

3. EVALUATE — After graph phase, run QueryExpr against PageStore:
              fn evaluate(query: &QueryExpr, store: &PageStore) -> Vec<PageId>
              
              This is just set operations on indices we already built.

4. RENDER   — Convert results to HTML:
              - List of linked page titles (default)
              - Table with property columns (if :view :table specified)
              - Replace the query block in the AST with rendered HTML

5. FALLBACK — If a query pattern is unrecognized:
              - Render the query source as a styled code block
              - Add a note: "This query uses advanced Datalog features.
                View in Logseq for live results."
              - Link back to the page in Logseq (if configured)
```

#### What this covers vs. what it doesn't

**Handles (estimated 90%+ of real queries):**
- Tag-based filtering (AND, OR, NOT combinations)
- Property-based filtering (exact match, existence check)
- Date range queries (journal pages between dates)
- Namespace queries
- Task/TODO status queries
- Sort and limit
- Nested boolean logic `(and (or [[a]] [[b]]) (not [[c]]))`

**Graceful fallback (rare edge cases):**
- Arbitrary Datalog joins across multiple entity types
- Custom `:result-transform` functions (ClojureScript code)
- `:view` functions with custom hiccup rendering
- Queries using Datascript predicates like `[(> ?age 30)]` on non-standard properties

The fallback is honest and useful — it shows the query source and tells the user this particular query needs Logseq. No silent failures, no wrong results.

---

## 7. Configuration

**`logseq-publish.toml`** (placed in graph root or specified via CLI):

```toml
# ============================================================
# SITE — Identity and metadata
# ============================================================
[site]
title = "Cyber Valley Knowledge Base"
description = "Permaculture, technology, and distributed systems research"
base_url = "https://cybervalley.wiki"
language = "en"
# The root/home page. This page's content renders at /
# Can be any page name from your graph.
# If unset, generates an auto-index of recent pages.
root_page = "Index"
# Favicon — path relative to static_dir or an emoji
favicon = "🧬"                     # emoji rendered as SVG, or "favicon.png"

# ============================================================
# NAVIGATION — Main menu and sidebar structure
# ============================================================
[nav]
# Main menu items. Each entry is a page name or URL.
# Pages are resolved from your graph; URLs open externally.
# Order here = order in rendered menu.
menu = [
    { label = "Home",       page = "Index" },
    { label = "Research",   page = "Research" },
    { label = "Projects",   page = "projects" },    # namespace parent — lists children
    { label = "Graph",      url  = "/graph" },       # built-in graph page
    { label = "About",      page = "About" },
]

# Show these sections in the sidebar (left panel on desktop)
[nav.sidebar]
show_namespaces = true             # Tree view of namespace hierarchy
show_recent = true                 # Recently updated pages
recent_count = 10
show_tags = true                   # Tag list with counts

# ============================================================
# BUILD — Input/output paths
# ============================================================
[build]
input_dir = "."                    # Logseq graph root
output_dir = "build"               # Generated site output
template_dir = "templates"         # Custom templates (optional, overrides bundled)
static_dir = "static"             # Additional static files to copy

# ============================================================
# CONTENT — What to publish and how
# ============================================================
[content]
public_only = true                 # Only export pages with public:: true
exclude_patterns = [               # Glob patterns to skip
    "logseq/*",
    "draws/*",
    ".git/*",
]
include_journals = false           # Include journal pages in output
default_public = false             # If no public:: property, is page public?

[content.namespaces]
flatten = false                    # true: /parent-child, false: /parent/child

# ============================================================
# URLS — Link structure
# ============================================================
[urls]
style = "pretty"                   # "pretty" (/page/) or "direct" (/page.html)
slugify = true                     # Convert "My Page" → "my-page"

# ============================================================
# FEEDS — RSS/Atom
# ============================================================
[feeds]
enabled = true
title = "Cyber Valley Updates"
items = 20

# ============================================================
# SEARCH — Client-side search
# ============================================================
[search]
enabled = true
engine = "json"                    # "json" (built-in) or "pagefind" (external)

# ============================================================
# ANALYTICS — Privacy-respecting analytics
# ============================================================
[analytics]
plausible_domain = ""              # e.g., "cybervalley.wiki" — empty = disabled
plausible_script = "https://plausible.io/js/script.js"  # self-hosted URL override

# ============================================================
# GRAPH — Knowledge graph visualization
# ============================================================
[graph]
enabled = true                     # Generate /graph page with interactive graph
show_minimap = true                # Per-page local graph widget
minimap_depth = 2                  # How many hops to show in minimap
physics = "force-directed"         # Layout algorithm

# ============================================================
# STYLE — Visual customization without touching CSS
# ============================================================
# These map directly to CSS custom properties.
# Fork-friendly: change these values and the whole site adapts.
[style]
# Colors — used for both light and dark mode base
primary_color   = "#2d6a4f"        # Links, accents, active states
secondary_color = "#264653"        # Secondary accents, headings
bg_color        = "#fafaf9"        # Page background (light mode)
text_color      = "#1a1a1a"        # Body text (light mode)
surface_color   = "#ffffff"        # Card/panel backgrounds (light mode)
border_color    = "#e2e2e0"        # Subtle borders

# Dark mode overrides (auto-applied when prefers-color-scheme: dark)
[style.dark]
bg_color        = "#0f0f0f"
text_color      = "#e5e5e3"
surface_color   = "#1a1a1a"
border_color    = "#2a2a28"

# Typography
[style.typography]
font_body       = "system-ui, -apple-system, 'Segoe UI', sans-serif"
font_mono       = "'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace"
font_size_base  = "1rem"           # Scales everything proportionally
line_height     = "1.7"            # Generous for readability
max_width       = "48rem"          # Content column max-width

# Code blocks
[style.code]
theme_light     = "base16-ocean.light"   # syntect theme for light mode
theme_dark      = "base16-ocean.dark"    # syntect theme for dark mode
show_line_numbers = false
```

### Config Resolution Order

When the tool runs, configuration is resolved in this order (later overrides earlier):

1. **Built-in defaults** — sensible values compiled into the binary
2. **`logseq-publish.toml`** — in graph root or specified via `--config`
3. **CLI flags** — `--base-url`, `--output`, `--drafts`, etc.

This means the tool works with zero config (`logseq-publish build . public`) using defaults, but every aspect can be tuned.

### Root Page Behavior

The `site.root_page` setting controls what renders at `/`:

| Setting | Behavior |
|---|---|
| `root_page = "Index"` | The page named "Index" from your graph renders as the homepage |
| `root_page = "Contents"` | The "Contents" page renders as homepage (Logseq default) |
| Not set / empty | Auto-generated index: site title + recent pages + tag cloud |

The root page is a regular graph page — it has backlinks, appears in search, and its `[[wikilinks]]` resolve normally. It just also renders at `/` in addition to its slug URL.

### Menu Configuration

Menu items can reference graph pages or external/internal URLs:

```toml
# Reference a graph page — resolved to its slug URL, marked active when visiting
{ label = "Research", page = "Research" }

# Reference a namespace — renders as dropdown with child pages
{ label = "Projects", page = "projects", children = true }

# Direct URL — internal paths or external links
{ label = "Graph", url = "/graph" }
{ label = "GitHub", url = "https://github.com/cybercongress", external = true }
```

If `children = true` on a namespace page, the menu item renders as a dropdown listing all child pages in that namespace.

### Style Customization for Forks

The `[style]` section is designed so that someone forking the project (or a site built with it) can restyle the entire site by changing ~10 values without touching CSS or templates. The values are injected as CSS custom properties into `<style>` in the base template:

```html
<!-- Generated by logseq-publish from [style] config -->
<style>
  :root {
    --color-primary: {{ style.primary_color }};
    --color-secondary: {{ style.secondary_color }};
    --color-bg: {{ style.bg_color }};
    --color-text: {{ style.text_color }};
    --color-surface: {{ style.surface_color }};
    --color-border: {{ style.border_color }};
    --font-body: {{ style.typography.font_body }};
    --font-mono: {{ style.typography.font_mono }};
    --font-size-base: {{ style.typography.font_size_base }};
    --line-height: {{ style.typography.line_height }};
    --max-width: {{ style.typography.max_width }};
  }
  [data-theme="dark"] {
    --color-bg: {{ style.dark.bg_color }};
    --color-text: {{ style.dark.text_color }};
    --color-surface: {{ style.dark.surface_color }};
    --color-border: {{ style.dark.border_color }};
  }
</style>
```

The entire default CSS references only these variables — never hardcoded colors or fonts.

---

## 8. Publishing to crates.io

### Cargo.toml for Publishing

The `[package]` section needs specific fields for `cargo publish`:

```toml
[package]
name = "logseq-publish"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"              # MSRV — minimum supported Rust version
description = "A fast, Rust-native static site publisher for Logseq knowledge graphs"
license = "MIT"
repository = "https://github.com/cybercongress/logseq-publish"
homepage = "https://github.com/cybercongress/logseq-publish"
documentation = "https://github.com/cybercongress/logseq-publish/blob/main/README.md"
readme = "README.md"
keywords = ["logseq", "static-site-generator", "markdown", "knowledge-graph", "wiki"]
categories = ["command-line-utilities", "web-programming", "text-processing"]
exclude = [
    "tests/fixtures/*",
    ".github/*",
    "example/*",
    "docs/*",
]
# Ensures default templates and static assets are included in the crate
include = [
    "src/**/*",
    "templates/**/*",
    "static/**/*",
    "Cargo.toml",
    "Cargo.lock",
    "README.md",
    "LICENSE",
    "CHANGELOG.md",
]
```

### Embedding Default Templates and Static Assets

For `cargo install logseq-publish` to produce a self-contained binary, default templates and CSS must be embedded at compile time:

```rust
// src/render/templates.rs

use minijinja::Environment;
use std::path::Path;

/// Default templates baked into the binary via include_str!
const DEFAULT_BASE:      &str = include_str!("../../templates/base.html");
const DEFAULT_PAGE:      &str = include_str!("../../templates/page.html");
const DEFAULT_INDEX:     &str = include_str!("../../templates/index.html");
const DEFAULT_TAG:       &str = include_str!("../../templates/tag.html");
const DEFAULT_JOURNAL:   &str = include_str!("../../templates/journal.html");
const DEFAULT_SEARCH:    &str = include_str!("../../templates/search.html");
const DEFAULT_GRAPH:     &str = include_str!("../../templates/graph.html");
const DEFAULT_BACKLINKS: &str = include_str!("../../templates/partials/backlinks.html");
const DEFAULT_NAV:       &str = include_str!("../../templates/partials/nav.html");

/// Default CSS and JS baked into the binary
const DEFAULT_CSS:       &str = include_str!("../../static/style.css");
const DEFAULT_SEARCH_JS: &str = include_str!("../../static/search.js");
const DEFAULT_GRAPH_JS:  &str = include_str!("../../static/graph.js");

pub fn setup_environment(custom_template_dir: Option<&Path>) -> Environment<'static> {
    let mut env = Environment::new();

    // Load defaults first
    env.add_template("base.html", DEFAULT_BASE).unwrap();
    env.add_template("page.html", DEFAULT_PAGE).unwrap();
    env.add_template("index.html", DEFAULT_INDEX).unwrap();
    // ... etc

    // If user has custom templates, override on top
    if let Some(dir) = custom_template_dir {
        if dir.exists() {
            // minijinja path_loader — user templates override defaults
            env.set_loader(path_loader(dir));
        }
    }

    env
}
```

Static assets (CSS, JS) are similarly embedded and written to the output directory during build, but overridden if the user has matching files in their `static_dir`.

### Pre-publish Checklist

```bash
# 1. Verify it builds clean
cargo build --release
cargo test
cargo clippy -- -D warnings

# 2. Check the package contents
cargo package --list
# Verify: templates/ and static/ are included
# Verify: tests/fixtures/ is excluded

# 3. Dry-run publish
cargo publish --dry-run

# 4. Verify binary works from install
cargo install --path .
logseq-publish --version
logseq-publish build tests/fixtures /tmp/test-output
# Open /tmp/test-output/index.html — should look correct

# 5. Publish
cargo publish
```

### Binary Distribution (Beyond Cargo)

For users who don't have Rust installed:

```yaml
# .github/workflows/release.yml (simplified)
# Triggered on git tag push (v0.1.0, etc.)
# Builds for: x86_64-linux, aarch64-linux, x86_64-macos, aarch64-macos, x86_64-windows
# Uploads binaries to GitHub Releases
# Also publishes to cargo via cargo publish
```

Include a one-liner install script:
```bash
curl -sSfL https://github.com/cybercongress/logseq-publish/releases/latest/download/install.sh | sh
```

### README.md Structure (for crates.io landing page)

```markdown
# logseq-publish

A fast, single-binary static site publisher for Logseq knowledge graphs.

## Quick Start
cargo install logseq-publish
logseq-publish build ./my-graph ./public

## Features
- 🔗 Native [[wikilink]] resolution with backlinks
- 🔍 Built-in search
- 🌐 Interactive knowledge graph visualization
- ⚡ Sub-second builds (Rust-native)
- 🎨 Fully customizable via config and templates
- 📡 RSS feeds, sitemap, SEO-friendly HTML

## Configuration
[link to config docs]

## License
MIT
```

```
logseq-publish 0.1.0
A Rust-native static site publisher for Logseq graphs

USAGE:
    logseq-publish <COMMAND> [OPTIONS]

COMMANDS:
    build     Build the static site
    serve     Build and serve with live reload
    init      Initialize a new logseq-publish.toml config
    check     Validate graph and report broken links

OPTIONS:
    -c, --config <PATH>     Path to config file [default: logseq-publish.toml]
    -v, --verbose           Increase verbosity (-v info, -vv debug, -vvv trace)
    -q, --quiet             Suppress output
    -h, --help              Print help
    -V, --version           Print version

BUILD OPTIONS:
    -o, --output <DIR>      Override output directory
    --drafts                Include non-public pages
    --base-url <URL>        Override base URL (useful for staging)

SERVE OPTIONS:
    -p, --port <PORT>       Server port [default: 8080]
    -b, --bind <ADDR>       Bind address [default: 127.0.0.1]
    --no-reload             Disable live reload
    --open                  Open browser automatically
```

---

## 9. CLI Interface

```
logseq-publish 0.1.0
A Rust-native static site publisher for Logseq graphs

USAGE:
    logseq-publish <COMMAND> [OPTIONS]

COMMANDS:
    build     Build the static site
    serve     Build and serve with live reload
    init      Initialize a new logseq-publish.toml config
    check     Validate graph and report broken links

OPTIONS:
    -c, --config <PATH>     Path to config file [default: logseq-publish.toml]
    -v, --verbose           Increase verbosity (-v info, -vv debug, -vvv trace)
    -q, --quiet             Suppress output
    -h, --help              Print help
    -V, --version           Print version

BUILD OPTIONS:
    -o, --output <DIR>      Override output directory
    --drafts                Include non-public pages
    --base-url <URL>        Override base URL (useful for staging)

SERVE OPTIONS:
    -p, --port <PORT>       Server port [default: 8080]
    -b, --bind <ADDR>       Bind address [default: 127.0.0.1]
    --no-reload             Disable live reload
    --open                  Open browser automatically
```

---

## 10. Implementation Plan

### Phase 1: Core Pipeline (MVP)

**Goal:** Read Logseq graph → produce working HTML site with wikilinks and backlinks.

#### Step 1.1 — Project Scaffold
- `cargo init logseq-publish`
- Set up `Cargo.toml` with core dependencies
- Set up CLI with clap derive macros
- Create test fixtures (sample Logseq graph with 5-10 pages)
- Set up basic test infrastructure

#### Step 1.2 — Scanner Module
- Walk graph directory, discover `.md` files
- Classify as page / journal based on path
- Discover assets directory
- Return `Vec<DiscoveredFile>` with path + kind

#### Step 1.3 — Property Parser
- Regex-based extraction of `property:: value` from top of file
- Parse into `PageMeta` struct
- Handle multi-value properties (tags, aliases)
- Handle boolean properties (public)
- Strip property block from content string
- **Tests:** property extraction for various formats

#### Step 1.4 — Outliner Normalizer
- Transform Logseq bullet markdown into standard markdown
- Rules: top-level single bullets → paragraphs, nested → lists, headings promotion
- Track block UUIDs (`id:: uuid` properties on blocks)
- Preserve indentation semantics
- **Tests:** various outliner patterns → expected markdown

#### Step 1.5 — Markdown Parsing + WikiLink Collection
- Parse normalized markdown through comrak with options:
  - `extension.wikilinks_title_after_pipe = true`
  - `extension.strikethrough = true`
  - `extension.table = true`
  - `extension.tasklist = true`
  - `extension.footnotes = true`
  - `parse.relaxed_autolinks = true`
  - `render.unsafe_ = true` (we control input)
- Walk AST to collect all `NodeValue::WikiLink` → store as `outgoing_links`
- **Tests:** wikilink collection from various documents

#### Step 1.6 — Graph Construction
- Build `PageStore` from all parsed pages
- Iterate all pages: for each outgoing link, add to target's backlinks
- Build tag index
- Build namespace tree
- **Tests:** backlink symmetry, tag index correctness

#### Step 1.7 — AST Transform + Rendering
- Walk comrak AST, transform `NodeValue::WikiLink` to proper `<a>` tags with resolved URLs
- Set up MiniJinja environment with template loader
- Create default templates (base, page, index)
- Build `PageContext` per page (inject backlinks, nav data)
- Render AST → HTML body, wrap in template
- **Tests:** rendered HTML contains correct links, backlinks section populated

#### Step 1.8 — Output + Finalization
- Write rendered HTML to output directory
- Copy assets
- Generate RSS feed
- Generate sitemap.xml
- **Tests:** output file structure, valid RSS XML

#### Step 1.9 — Dev Server
- `tiny_http` static file server
- `notify` file watcher → rebuild on change
- WebSocket-based live reload (inject small JS into pages)
- **Tests:** manual testing, server starts/stops cleanly

#### Step 1.10 — Default Theme (Foundation)
- Clean, minimal CSS with CSS custom properties (Phase 4 will expand this)
- Responsive single-column layout (multi-column comes in Phase 4)
- Basic dark/light mode via `prefers-color-scheme`
- Backlinks section styling: subtle, at bottom of page
- Code block styling with syntect themes
- Internal link styling: distinguish from external, broken links visually distinct
- Basic navigation: page list, tag list
- Must look good enough to use immediately — not just functional wireframes

### Phase 2: Enhanced Features

#### Step 2.1 — Block References
- Build block UUID index during parse phase
- Resolve `((uuid))` → inline block content or linked preview
- Handle circular references gracefully

#### Step 2.2 — Query Engine
- Detect `{{query ...}}` and `#+BEGIN_QUERY...#+END_QUERY` blocks during parse
- Parse query expressions into `QueryExpr` AST (see section 6.5)
- Implement evaluator: `fn evaluate(query: &QueryExpr, store: &PageStore) -> Vec<PageId>`
- Render results as HTML (linked list or property table)
- Replace query blocks in comrak AST with rendered results
- Fallback: unrecognized patterns → styled code block with explanation
- **Tests:** each query pattern type against test fixtures

#### Step 2.3 — Search
- Build JSON search index (page title, first paragraph, tags, properties)
- Lightweight client-side JS search (no framework dependency)
- OR: Pagefind integration (post-processing step)

#### Step 2.4 — Graph Data Export
- Export full link graph as JSON adjacency list: `{ nodes: [...], edges: [...] }`
- Per-page local subgraph extraction (n-hop neighborhood)
- Include node metadata: title, tags, namespace, page kind

#### Step 2.5 — Image Optimization
- Detect images referenced in content
- Resize large images to configurable max dimensions
- Generate WebP variants
- Emit responsive `<picture>` tags

#### Step 2.6 — Incremental Builds
- Content hash per page, skip unchanged
- Dependency tracking (if A links to B, and B changes, re-render A's backlinks)

### Phase 3: Polish

#### Step 3.1 — Table of Contents
- Extract headings from comrak AST per page
- Generate hierarchical TOC data for template
- Configurable: enable/disable globally or per-page via property `toc:: false`

#### Step 3.2 — Admonition Blocks
- Parse Logseq `#+BEGIN_NOTE`, `#+BEGIN_TIP`, `#+BEGIN_WARNING`, `#+BEGIN_CAUTION`
- Render as styled `<aside>` or `<div>` blocks with icons

#### Step 3.3 — Plausible Analytics
- If `analytics.plausible_domain` configured, inject `<script>` into base template
- Support self-hosted Plausible via `plausible_script` URL override
- No-JS fallback: add `<noscript>` pixel for basic pageview tracking

#### Step 3.4 — Theme System
- Theme = directory of templates + static assets
- `logseq-publish init-theme` scaffolds a custom theme
- Bundled default theme as fallback
- Theme inherits from default (only override what you change)

### Phase 4: UX, Visual Design & Graph Rendering

#### Step 4.1 — Default Design System
- CSS custom properties architecture for easy theming:
  ```css
  :root {
    --color-bg: #fafaf9;
    --color-text: #1a1a1a;
    --color-link: #2d6a4f;
    --color-link-broken: #c53030;
    --color-accent: #2d6a4f;
    --color-surface: #ffffff;
    --color-border: #e2e2e0;
    --font-body: system-ui, -apple-system, sans-serif;
    --font-mono: 'JetBrains Mono', 'Fira Code', monospace;
    --font-size-base: 1rem;
    --max-width-content: 48rem;
    --max-width-page: 72rem;
  }
  ```
- Typography: clear hierarchy, generous line height, readable measure
- Whitespace-driven layout (no visual clutter)
- Inspiration: Andy Matuschak's notes, Gwern.net, Bear Blog simplicity
- Code blocks: syntect themes integrated with site palette

#### Step 4.2 — Page Layout
- Three-column layout on desktop:
  - Left: collapsible sidebar (nav tree, namespace hierarchy)
  - Center: page content (constrained max-width for readability)
  - Right: contextual panel (backlinks, local graph minimap, TOC)
- Mobile: single column with hamburger nav, backlinks below content
- Breadcrumbs for namespace pages: `projects / Cyber Valley`

#### Step 4.3 — Interactive Graph Visualization (Full Page)
- Dedicated `/graph` page with full knowledge graph
- D3.js force-directed layout (loaded from graph JSON export)
- Node size weighted by number of connections (PageRank-ish)
- Node color by namespace or tag
- Click node → navigate to page
- Hover → show title + connection count
- Search/filter within graph
- Physics controls: drag nodes, zoom, center on selection
- Bundle as standalone JS file, loaded only on graph page
- **Design:** dark background, glowing nodes, clean edges — constellation aesthetic

#### Step 4.4 — Per-Page Graph Minimap
- Small interactive widget in right panel (or expandable)
- Shows current page as center node + 1-2 hop neighbors
- Same D3 force layout but miniaturized
- Click neighbor → navigate
- Lightweight: only loads subgraph data, not full graph

#### Step 4.5 — Page Hover Previews
- On internal link hover, show tooltip with:
  - Page title
  - Icon (if set)
  - First paragraph (truncated)
  - Tag pills
- CSS-only initial approach (using `<details>` or `:hover` with injected `<div>`)
- Optional enhancement with minimal JS for positioning

#### Step 4.6 — Dark/Light Mode
- System preference detection via `prefers-color-scheme`
- Manual toggle button (persisted in localStorage)
- Smooth CSS transition between modes
- All colors defined as CSS custom properties, toggled via `data-theme` attribute
- Syntax highlighting themes switch too (light/dark syntect variants)

#### Step 4.7 — Navigation & Discovery
- Tag cloud page (`/tags`): all tags with page counts, click to filter
- Recent pages section on index
- "Related pages" section per page (pages sharing most tags)
- Reading progress bar (thin line at top of viewport)
- Keyboard shortcuts: `/` for search, `←` `→` for prev/next in namespace

---

## 11. Claude Code Implementation Guide

### Working Approach

This project should be built **incrementally and test-driven**. Each step from the implementation plan above is a natural unit of work. Claude Code should:

1. **Start with Step 1.1** — scaffold the project, get `cargo build` passing
2. **Write tests first** for each module using the fixtures
3. **Build module by module** following the step order
4. **Run `cargo test` after every meaningful change**
5. **Run `cargo clippy` periodically** for idiomatic Rust

### Key Implementation Notes

#### comrak WikiLinks Setup
```rust
use comrak::{Arena, Options, parse_document, format_html};
use comrak::nodes::{NodeValue, NodeWikiLink};

fn setup_comrak_options() -> Options {
    let mut options = Options::default();
    // Enable WikiLink parsing — CRITICAL
    options.extension.wikilinks_title_after_pipe = true;
    // GFM extensions
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    // Parse options
    options.parse.relaxed_autolinks = true;
    // Render options  
    options.render.unsafe_ = true; // We control input, allow raw HTML
    options
}
```

#### AST Walk Pattern for WikiLink Collection
```rust
fn collect_wikilinks<'a>(root: &'a comrak::nodes::AstNode<'a>) -> Vec<String> {
    let mut links = Vec::new();
    for node in root.descendants() {
        if let NodeValue::WikiLink(ref wl) = node.data.borrow().value {
            links.push(wl.url.clone());
        }
    }
    links
}
```

#### AST Transform Pattern for WikiLink Resolution
```rust
fn transform_wikilinks<'a>(
    root: &'a comrak::nodes::AstNode<'a>,
    page_store: &PageStore,
    arena: &'a Arena<comrak::nodes::AstNode<'a>>,
) {
    for node in root.descendants() {
        let should_transform = matches!(
            node.data.borrow().value,
            NodeValue::WikiLink(_)
        );
        if should_transform {
            let wl = match &node.data.borrow().value {
                NodeValue::WikiLink(wl) => wl.clone(),
                _ => unreachable!(),
            };
            // Create replacement HTML node
            let slug = slugify(&wl.url);
            let exists = page_store.pages.contains_key(&slug);
            let class = if exists { "internal-link" } else { "internal-link broken-link" };
            let title = if wl.title.is_empty() { &wl.url } else { &wl.title };
            let html = format!(
                r#"<a href="/{slug}" class="{class}">{title}</a>"#,
            );
            // Replace WikiLink node with inline HTML
            let new_node = arena.alloc(comrak::nodes::AstNode::new(
                RefCell::new(comrak::nodes::Ast::new(
                    NodeValue::HtmlInline(html),
                    comrak::nodes::LineColumn { line: 0, column: 0 },
                ))
            ));
            node.insert_before(new_node);
            node.detach();
        }
    }
}
```

#### MiniJinja Template Setup
```rust
use minijinja::{Environment, context, path_loader};

fn setup_templates(template_dir: &Path) -> Environment<'static> {
    let mut env = Environment::new();
    env.set_loader(path_loader(template_dir));
    
    // Custom filters
    env.add_filter("reading_time", |word_count: u64| -> u64 {
        (word_count as f64 / 200.0).ceil() as u64
    });
    
    env.add_filter("date_format", |date: String, fmt: String| -> String {
        // Parse and reformat date
        chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
            .map(|d| d.format(&fmt).to_string())
            .unwrap_or(date)
    });
    
    env
}
```

### Test Fixtures

Create a minimal but representative Logseq graph in `tests/fixtures/`:

```
tests/fixtures/
├── pages/
│   ├── Collective Focus Theorem.md    # Has properties, tags, wikilinks
│   ├── Bostrom.md                      # Linked from CFT
│   ├── Mycorrhizal Networks.md         # Linked from CFT
│   ├── projects∕Cyber Valley.md        # Namespace example
│   ├── projects∕Batch Rocket.md        # Namespace example
│   └── Private Page.md                 # public:: false
├── journals/
│   └── 2025_02_08.md                   # Journal with links
├── assets/
│   └── diagram.png
└── logseq/
    └── config.edn                      # Logseq config (for reference)
```

**Sample page content (`Collective Focus Theorem.md`):**
```markdown
title:: Collective Focus Theorem
tags:: research, mathematics, distributed-systems
public:: true
alias:: CFT

- A mathematical framework for understanding consensus emergence in distributed systems.
- ## Core Principles
  - The theorem establishes that collective attention in a network follows predictable patterns
  - These patterns emerge from the interaction of individual agent focus with network topology
  - Related to work on [[Bostrom]] network GPU consensus
- ## Biological Parallels
  - [[Mycorrhizal Networks]] exhibit similar consensus patterns
  - The "wood wide web" demonstrates natural distributed intelligence
- ## Implementation
  - Applied in [[projects/Cyber Valley]] community governance
  - Three years of empirical data from 1000+ agents
```

### Quality Checklist

Before considering any phase complete:

- [ ] `cargo test` passes with no failures
- [ ] `cargo clippy` produces no warnings
- [ ] `cargo fmt` has been run
- [ ] Generated HTML is valid (no unclosed tags)
- [ ] All wikilinks resolve correctly or are marked as broken
- [ ] Backlinks are symmetric (if A→B exists, B's backlinks include A)
- [ ] RSS feed validates against RSS 2.0 spec
- [ ] Dev server starts, serves files, reloads on change
- [ ] Output site works when opened directly from filesystem (file:// URLs)
- [ ] Memory usage is reasonable for graphs with 1000+ pages

---

## 12. Design Philosophy

1. **Logseq-native.** Don't fight the format. Understand outliner bullets, properties, wikilinks as first-class concepts.

2. **Two-pass is mandatory.** The knowledge graph (backlinks, tags, namespaces) can only be built after all pages are parsed. Render phase comes after graph construction. Never try to do it in one pass.

3. **Fail gracefully.** Broken wikilinks → styled as broken, not crash. Missing properties → defaults. Unparseable query → rendered as code block with note.

4. **Fast by default.** Use rayon for parallel page parsing/rendering. Keep the dependency tree lean. Target sub-second builds for graphs under 500 pages.

5. **Hackable templates.** MiniJinja templates should be self-explanatory. Users who know HTML/CSS can customize everything without touching Rust.

6. **Single binary, zero config to start.** `logseq-publish build ./my-graph ./output` should work with no config file, using sensible defaults.
