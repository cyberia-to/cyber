use crate::graph::PageStore;
use crate::parser::slugify_page_name;
use comrak::{
    arena_tree::Node,
    nodes::{Ast, AstNode, NodeValue},
    plugins::syntect::SyntectAdapterBuilder,
    Arena, Options, Plugins,
};
use regex::Regex;

use std::cell::RefCell;

fn setup_comrak_options() -> Options<'static> {
    let mut options = Options::default();
    // Enable WikiLink parsing
    options.extension.wikilinks_title_after_pipe = true;
    // GFM extensions
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    // Parse options
    options.parse.relaxed_autolinks = true;
    // Render options — we control input, allow raw HTML
    options.render.unsafe_ = true;
    options
}

use super::toc::{self, TocEntry};

/// Result of rendering markdown, including TOC data.
pub struct RenderResult {
    pub html: String,
    pub toc: Vec<TocEntry>,
}

/// Escape `|` inside `[[...]]` so comrak's table parser doesn't split wikilinks.
/// Uses U+FFFF as placeholder — comrak's wikilinks_title_after_pipe looks for `|`,
/// so this preserves the separator for comrak while hiding it from the table parser.
/// The placeholder is restored to `|` just before comrak processes wikilinks.
const PIPE_PLACEHOLDER: char = '\u{FFFF}';

fn escape_pipes_in_wikilinks(markdown: &str) -> String {
    let mut result = String::with_capacity(markdown.len());
    let mut inside = false;
    let chars: Vec<char> = markdown.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if i + 1 < len && chars[i] == '[' && chars[i + 1] == '[' {
            inside = true;
            result.push('[');
            result.push('[');
            i += 2;
            continue;
        }
        if inside && i + 1 < len && chars[i] == ']' && chars[i + 1] == ']' {
            inside = false;
            result.push(']');
            result.push(']');
            i += 2;
            continue;
        }
        if inside && chars[i] == '|' {
            result.push(PIPE_PLACEHOLDER);
        } else {
            result.push(chars[i]);
        }
        i += 1;
    }
    result
}

/// Restore pipe placeholders in a string back to `|`.
fn restore_pipes(s: &str) -> String {
    s.replace(PIPE_PLACEHOLDER, "|")
}

/// Extract math blocks ($..$ and $$..$$) from markdown, replacing them with placeholders.
/// Returns the processed markdown and a list of extracted math strings.
fn extract_math_blocks(markdown: &str) -> (String, Vec<String>) {
    let mut math_blocks: Vec<String> = Vec::new();
    let mut result = String::with_capacity(markdown.len());
    let chars: Vec<char> = markdown.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut inside_wikilink = false;

    while i < len {
        // Track wiki-link boundaries: [[ opens, ]] closes
        if i + 1 < len && chars[i] == '[' && chars[i + 1] == '[' {
            inside_wikilink = true;
            result.push('[');
            result.push('[');
            i += 2;
            continue;
        }
        if inside_wikilink && i + 1 < len && chars[i] == ']' && chars[i + 1] == ']' {
            inside_wikilink = false;
            result.push(']');
            result.push(']');
            i += 2;
            continue;
        }
        // Skip $ inside wiki-links — not math
        if inside_wikilink && chars[i] == '$' {
            result.push('$');
            i += 1;
            continue;
        }

        // Check for $$ (display math) first
        if i + 1 < len && chars[i] == '$' && chars[i + 1] == '$' {
            // Find closing $$
            let start = i;
            i += 2;
            let mut found = false;
            while i + 1 < len {
                if chars[i] == '$' && chars[i + 1] == '$' {
                    let math_str: String = chars[start..i + 2].iter().collect();
                    let idx = math_blocks.len();
                    math_blocks.push(math_str);
                    result.push_str(&format!("\n\nMATH_PLACEHOLDER_{}\n\n", idx));
                    i += 2;
                    found = true;
                    break;
                }
                i += 1;
            }
            if !found {
                // No closing $$, output as-is
                let remainder: String = chars[start..].iter().collect();
                result.push_str(&remainder);
                break;
            }
        }
        // Check for $ (inline math) — skip if preceded by \
        else if chars[i] == '$' && (i == 0 || chars[i - 1] != '\\') {
            let start = i;
            i += 1;
            // Skip if immediately followed by space (not math)
            if i < len && chars[i] != ' ' && chars[i] != '$' {
                let mut found = false;
                while i < len {
                    if chars[i] == '$' && (i == 0 || chars[i - 1] != '\\') {
                        let math_str: String = chars[start..i + 1].iter().collect();
                        let idx = math_blocks.len();
                        math_blocks.push(math_str);
                        result.push_str(&format!("MATH_PLACEHOLDER_{}", idx));
                        i += 1;
                        found = true;
                        break;
                    }
                    // Don't cross newlines for inline math
                    if chars[i] == '\n' {
                        break;
                    }
                    i += 1;
                }
                if !found {
                    let remainder: String = chars[start..i].iter().collect();
                    result.push_str(&remainder);
                }
            } else {
                result.push('$');
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    (result, math_blocks)
}

/// Ensure underscores inside \text{} blocks are escaped as \_ for KaTeX.
/// KaTeX requires `\_` for literal underscores in \text{} (bare `_` triggers subscript).
/// Handles both already-escaped `\_` and bare `_` by normalizing first.
fn fix_text_underscores(math: &str) -> String {
    let re = Regex::new(r"\\text\{([^}]*)\}").unwrap();
    re.replace_all(math, |caps: &regex::Captures| {
        let inner = &caps[1];
        // Normalize: strip \_ to _, then re-escape all _ to \_
        let normalized = inner.replace("\\_", "_");
        let fixed = normalized.replace('_', "\\_");
        format!("\\text{{{}}}", fixed)
    })
    .to_string()
}

/// Restore math blocks from placeholders in the rendered HTML.
fn restore_math_blocks(html: &str, math_blocks: &[String]) -> String {
    let re = Regex::new(r"MATH_PLACEHOLDER_(\d+)").unwrap();
    re.replace_all(html, |caps: &regex::Captures| {
        let idx: usize = caps[1].parse().unwrap_or(0);
        if idx < math_blocks.len() {
            fix_text_underscores(&math_blocks[idx])
        } else {
            caps[0].to_string()
        }
    })
    .to_string()
}

/// Render markdown to HTML with wikilink resolution, embed expansion, block refs, and queries.
pub fn render_markdown(markdown: &str, store: &PageStore, _code_theme: &str) -> RenderResult {
    // Pre-process: resolve embeds and block references in the markdown source
    let processed = resolve_embeds_and_refs(markdown, store, 0);

    // Pre-process: resolve query blocks
    let processed = crate::query::resolve_queries(&processed, store);

    // Protect pipes inside wikilinks from being parsed as table column separators.
    // Replace | with PIPE_PLACEHOLDER inside [[...]], then restore in the parsed AST.
    let processed = escape_pipes_in_wikilinks(&processed);

    // Protect math blocks from comrak processing
    let (processed, math_blocks) = extract_math_blocks(&processed);

    let arena = Arena::new();
    let options = setup_comrak_options();

    let root = comrak::parse_document(&arena, &processed, &options);

    // Extract TOC from headings before transforming
    let toc_entries = toc::extract_toc(root);

    // Transform wikilinks to proper HTML links
    transform_wikilinks(root, store, &arena);

    // Transform external links to open in new tab
    transform_external_links(root, &arena);

    // Add heading IDs for TOC anchors
    inject_heading_ids(root, &arena);

    // Render to HTML with syntax highlighting (CSS class mode — no inline styles)
    let adapter = SyntectAdapterBuilder::new()
        .css()
        .build();
    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let mut html = Vec::new();
    comrak::format_html_with_plugins(root, &options, &mut html, &plugins).unwrap();

    let mut html = String::from_utf8(html).unwrap_or_default();

    // Restore math blocks after HTML rendering
    if !math_blocks.is_empty() {
        html = restore_math_blocks(&html, &math_blocks);
    }

    RenderResult {
        html,
        toc: toc_entries,
    }
}

/// Inject id attributes into heading nodes by wrapping with anchor spans.
fn inject_heading_ids<'a>(root: &'a AstNode<'a>, arena: &'a Arena<AstNode<'a>>) {
    let mut headings: Vec<(&'a AstNode<'a>, String)> = Vec::new();

    for node in root.descendants() {
        let data = node.data.borrow();
        if let NodeValue::Heading(_) = data.value {
            let text = get_node_text_content(node);
            if !text.is_empty() {
                let id = slug::slugify(&text);
                headings.push((node, id));
            }
        }
    }

    for (node, id) in headings {
        // Prepend an anchor before the heading content
        let anchor_html = format!(r#"<a class="heading-anchor" id="{}"></a>"#, id);
        let anchor_node = arena.alloc(Node::new(RefCell::new(Ast::new(
            NodeValue::HtmlInline(anchor_html),
            node.data.borrow().sourcepos.start,
        ))));
        node.prepend(anchor_node);
    }
}

lazy_static::lazy_static! {
    /// Matches `{{embed [[Page Name]]}}` page embeds
    static ref EMBED_PAGE_RE: Regex = Regex::new(
        r"\{\{embed\s+\[\[([^\]]+)\]\]\s*\}\}"
    ).unwrap();
}

/// Resolve {{embed [[Page]]}} in markdown.
/// `depth` prevents infinite recursion for circular embeds.
fn resolve_embeds_and_refs(markdown: &str, store: &PageStore, depth: usize) -> String {
    if depth > 3 {
        return markdown.to_string();
    }

    // Fast path: if no embed patterns exist, skip regex processing
    if !markdown.contains("{{embed") {
        return markdown.to_string();
    }

    // Resolve {{embed [[Page Name]]}} → inline the page's content
    EMBED_PAGE_RE
        .replace_all(markdown, |caps: &regex::Captures| {
            let page_name = &caps[1];
            let slug = slugify_page_name(page_name);
            if let Some(page) = store.pages.get(&slug) {
                let content = resolve_embeds_and_refs(&page.content_md, store, depth + 1);
                format!(
                    "\n<div class=\"embed embed-page\" data-page=\"{}\">\n<div class=\"embed-header\"><a href=\"/{}\" class=\"internal-link\">{}</a></div>\n\n{}\n\n</div>\n",
                    slug, slug, page.meta.title, content
                )
            } else {
                format!(
                    "<div class=\"embed embed-page embed-missing\"><em>Embed: page \"{}\" not found</em></div>",
                    page_name
                )
            }
        })
        .to_string()
}

fn transform_wikilinks<'a>(
    root: &'a AstNode<'a>,
    store: &PageStore,
    arena: &'a Arena<AstNode<'a>>,
) {
    // Collect nodes that need transformation first to avoid borrow issues
    let mut nodes_to_transform: Vec<(&'a AstNode<'a>, String)> = Vec::new();

    for node in root.descendants() {
        let data = node.data.borrow();
        if let NodeValue::WikiLink(ref wl) = data.value {
            let url = wl.url.clone();
            nodes_to_transform.push((node, url));
        }
    }

    for (node, raw_url) in nodes_to_transform {
        // Restore pipe placeholders that were escaped to protect from table parser.
        // If the URL contains PIPE_PLACEHOLDER, comrak couldn't split url|title,
        // so the whole string is in the URL. Split it manually.
        let restored = restore_pipes(&raw_url);
        let (url, forced_display) = if restored.contains('|') {
            let mut parts = restored.splitn(2, '|');
            let u = parts.next().unwrap().to_string();
            let d = parts.next().map(|s| s.to_string());
            (u, d)
        } else {
            (restored, None)
        };
        let slug = slugify_page_name(&url);

        // Resolve alias to canonical page ID
        let resolved_slug = if store.pages.contains_key(&slug) {
            slug.clone()
        } else if let Some(canonical_id) = store.alias_map.get(&slug) {
            canonical_id.clone()
        } else {
            slug.clone()
        };

        let class = if store.stub_pages.contains(&resolved_slug) {
            "internal-link stub-link"
        } else if store.pages.contains_key(&resolved_slug) {
            "internal-link"
        } else {
            "internal-link stub-link"
        };

        // Get display text: forced from pipe split, or from children, or URL fallback
        let display = if let Some(ref d) = forced_display {
            d.clone()
        } else {
            let child_text = restore_pipes(&get_node_text_content(node));
            if child_text.trim().is_empty() {
                url.rsplit('/').next().unwrap_or(&url).to_string()
            } else {
                child_text
            }
        };

        let html = format!(
            r#"<a href="/{resolved_slug}" class="{class}" data-page="{resolved_slug}">{display}</a>"#,
        );

        // Replace WikiLink node with inline HTML
        let new_node = arena.alloc(Node::new(RefCell::new(Ast::new(
            NodeValue::HtmlInline(html),
            node.data.borrow().sourcepos.start,
        ))));

        node.insert_before(new_node);
        node.detach();
    }
}

/// Transform external links (http/https) to open in new tab.
fn transform_external_links<'a>(root: &'a AstNode<'a>, arena: &'a Arena<AstNode<'a>>) {
    let mut nodes_to_transform: Vec<(&'a AstNode<'a>, String, String)> = Vec::new();

    for node in root.descendants() {
        let data = node.data.borrow();
        if let NodeValue::Link(ref link) = data.value {
            if link.url.starts_with("http://") || link.url.starts_with("https://") {
                let url = link.url.clone();
                let title = link.title.clone();
                nodes_to_transform.push((node, url, title));
            }
        }
    }

    for (node, url, title) in nodes_to_transform {
        let display = get_node_text_content(node);
        let title_attr = if title.is_empty() {
            String::new()
        } else {
            format!(r#" title="{}""#, html_escape(&title))
        };
        let html = format!(
            r#"<a href="{}" class="external-link" target="_blank" rel="noopener noreferrer"{}>{}</a>"#,
            html_escape(&url),
            title_attr,
            html_escape(&display),
        );

        let new_node = arena.alloc(Node::new(RefCell::new(Ast::new(
            NodeValue::HtmlInline(html),
            node.data.borrow().sourcepos.start,
        ))));
        node.insert_before(new_node);
        node.detach();
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn get_node_text_content<'a>(node: &'a AstNode<'a>) -> String {
    let mut text = String::new();
    for child in node.children() {
        let data = child.data.borrow();
        match &data.value {
            NodeValue::Text(t) => text.push_str(t),
            _ => {
                text.push_str(&get_node_text_content(child));
            }
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::build_graph;
    use crate::parser::{PageKind, PageMeta, ParsedPage};
    use std::collections::HashMap;
    use std::path::PathBuf;

    const TEST_THEME: &str = "base16-ocean.dark";

    fn empty_store() -> PageStore {
        build_graph(vec![]).unwrap()
    }

    fn store_with_page(name: &str) -> PageStore {
        let page = ParsedPage {
            id: slugify_page_name(name),
            meta: PageMeta {
                title: name.to_string(),
                properties: HashMap::new(),
                tags: vec![],
                public: Some(true),
                aliases: vec![],
                date: None,
                icon: None,
                menu_order: None,
                stake: None,
            },
            kind: PageKind::Page,
            source_path: PathBuf::new(),
            namespace: None,
            subgraph: None,
            content_md: String::new(),
            outgoing_links: vec![],
        };
        build_graph(vec![page]).unwrap()
    }

    #[test]
    fn test_basic_markdown() {
        let store = empty_store();
        let result = render_markdown("# Hello\n\nWorld", &store, TEST_THEME);
        assert!(result.html.contains("<h1>"));
        assert!(result.html.contains("Hello"));
        assert!(result.html.contains("<p>World</p>"));
    }

    #[test]
    fn test_wikilink_resolved() {
        let store = store_with_page("My Page");
        let result = render_markdown("Link to [[My Page]]", &store, TEST_THEME);
        assert!(result.html.contains("class=\"internal-link\""));
        assert!(result.html.contains("href=\"/my-page\""));
    }

    #[test]
    fn test_wikilink_stub() {
        let store = empty_store();
        let result = render_markdown("Link to [[Missing Page]]", &store, TEST_THEME);
        assert!(result.html.contains("stub-link"));
    }

    #[test]
    fn test_gfm_features() {
        let store = empty_store();
        let result = render_markdown("| A | B |\n|---|---|\n| 1 | 2 |", &store, TEST_THEME);
        assert!(result.html.contains("<table>"));
    }

    #[test]
    fn test_math_block_protection() {
        let store = empty_store();
        // Inline math with backslash-brace should be preserved
        let result = render_markdown(
            "The set $\\left\\{x \\in \\mathbb{R}\\right\\}$ is open.",
            &store,
            TEST_THEME,
        );
        assert!(
            result.html.contains("\\left\\{"),
            "backslash-brace should be preserved in inline math"
        );
        // Display math
        let result = render_markdown("$$\\left\\{x > 0\\right\\}$$", &store, TEST_THEME);
        assert!(
            result.html.contains("\\left\\{"),
            "backslash-brace should be preserved in display math"
        );
        assert!(
            result.html.contains("x > 0"),
            "greater-than should be preserved in display math"
        );
    }

    #[test]
    fn test_text_underscore_fix() {
        // Already-escaped \_ should be preserved for KaTeX
        let store = empty_store();
        let result = render_markdown(
            "$\\text{type\\_tag}(a)$",
            &store,
            TEST_THEME,
        );
        assert!(
            result.html.contains("\\text{type\\_tag}"),
            "escaped underscore should be preserved in \\text{{}}: {}",
            result.html
        );

        // Bare underscores in \text{} should be escaped to \_
        let result = render_markdown(
            "$\\text{staking_share}$",
            &store,
            TEST_THEME,
        );
        assert!(
            result.html.contains("\\text{staking\\_share}"),
            "bare underscore should be escaped in \\text{{}}: {}",
            result.html
        );

        // Multiple \text{} blocks
        let result = render_markdown(
            "$\\text{BBG\\_root} = H(\\text{by\\_neuron.commit})$",
            &store,
            TEST_THEME,
        );
        assert!(result.html.contains("\\text{BBG\\_root}"));
        assert!(result.html.contains("\\text{by\\_neuron.commit}"));
    }

    #[test]
    fn test_toc_generation() {
        let store = empty_store();
        let result = render_markdown("# First\n\n## Second\n\n### Third\n\nContent", &store, TEST_THEME);
        assert_eq!(result.toc.len(), 3);
        assert_eq!(result.toc[0].text, "First");
        assert_eq!(result.toc[0].level, 1);
        assert_eq!(result.toc[1].text, "Second");
        assert_eq!(result.toc[1].level, 2);
    }
}
