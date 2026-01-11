use lazy_static::lazy_static;
use regex::{Captures, Regex};

use crate::page::PageIndex;
use crate::query;

lazy_static! {
    // Inline properties to remove
    static ref INLINE_PROPS_RE: Regex = Regex::new(
        r"(?m)^(\s*)(?:-\s*)?(collapsed|logseq\.order-list-type|id|query-table|query-sort-by|query-sort-desc|query-properties):: .+$"
    ).unwrap();

    // Wikilinks with $ signs
    static ref WIKILINK_DOLLAR_RE: Regex = Regex::new(r"\[\[([^\]]*\$[^\]]*)\]\]").unwrap();

    // Standalone $ tokens (simplified - matches $TOKEN patterns)
    static ref DOLLAR_TOKEN_RE: Regex = Regex::new(r"\$([A-Z][A-Z0-9]*)").unwrap();

    // Embed syntax
    static ref EMBED_RE: Regex = Regex::new(r"\{\{embed\s+\[\[([^\]]+)\]\]\s*\}\}").unwrap();

    // Block embed
    static ref BLOCK_EMBED_RE: Regex = Regex::new(r"\{\{embed\s+\(\(([^)]+)\)\)\s*\}\}").unwrap();

    // Block reference
    static ref BLOCK_REF_RE: Regex = Regex::new(r"\(\(([a-f0-9-]{36})\)\)").unwrap();

    // Query syntax
    static ref QUERY_RE: Regex = Regex::new(r"(?ms)^(\s*-\s*)?\{\{query[\s\S]*?\}\}").unwrap();

    // YouTube/video/pdf embeds
    static ref YOUTUBE_RE: Regex = Regex::new(r"\{\{youtube\s+([^\}]+)\}\}").unwrap();
    static ref VIDEO_RE: Regex = Regex::new(r"\{\{video\s+([^\}]+)\}\}").unwrap();
    static ref PDF_RE: Regex = Regex::new(r"\{\{pdf\s+([^\}]+)\}\}").unwrap();

    // Renderer
    static ref RENDERER_RE: Regex = Regex::new(r"\{\{renderer\s+[^\}]+\}\}").unwrap();

    // Cloze
    static ref CLOZE_RE: Regex = Regex::new(r"\{\{cloze\s+([^\}]+)\}\}").unwrap();

    // Hiccup/EDN syntax (Clojure-style [:tag ...] blocks)
    static ref HICCUP_RE: Regex = Regex::new(r"(?s)\[:[\w.-]+(?:\s+\{[^}]*\})?\s+(?:\[[\s\S]*?\])+\]").unwrap();

    // Task markers
    static ref DONE_RE: Regex = Regex::new(r"(?m)^(\s*)-\s+DONE\s+").unwrap();
    static ref TODO_RE: Regex = Regex::new(r"(?m)^(\s*)-\s+TODO\s+").unwrap();
    static ref NOW_RE: Regex = Regex::new(r"(?m)^(\s*)-\s+NOW\s+").unwrap();
    static ref DOING_RE: Regex = Regex::new(r"(?m)^(\s*)-\s+DOING\s+").unwrap();
    static ref LATER_RE: Regex = Regex::new(r"(?m)^(\s*)-\s+LATER\s+").unwrap();
    static ref WAITING_RE: Regex = Regex::new(r"(?m)^(\s*)-\s+WAITING\s+").unwrap();
    static ref CANCELLED_RE: Regex = Regex::new(r"(?m)^(\s*)-\s+CANCELLED\s+").unwrap();

    // Priority markers
    static ref PRIORITY_A_RE: Regex = Regex::new(r"\[#A\]").unwrap();
    static ref PRIORITY_B_RE: Regex = Regex::new(r"\[#B\]").unwrap();
    static ref PRIORITY_C_RE: Regex = Regex::new(r"\[#C\]").unwrap();

    // Schedule/deadline
    static ref SCHEDULED_RE: Regex = Regex::new(r"SCHEDULED:\s*<([^>]+)>").unwrap();
    static ref DEADLINE_RE: Regex = Regex::new(r"DEADLINE:\s*<([^>]+)>").unwrap();

    // Wikilinks (for adding pages/ prefix)
    static ref WIKILINK_RE: Regex = Regex::new(r"(!\s*)?\[\[([^\]|]+)(\|[^\]]*)?\]\]").unwrap();

    // Tables - fix pipe in first cell
    static ref TABLE_FIRST_CELL_RE: Regex = Regex::new(r"(?m)^-\s+\|").unwrap();
}

/// Transform Logseq content to Quartz-compatible format
pub fn transform(content: &str, page_index: &PageIndex) -> String {
    let mut result = content.to_string();

    // Remove inline properties
    result = INLINE_PROPS_RE.replace_all(&result, "").to_string();

    // Fix tables - remove list marker from first cell
    result = TABLE_FIRST_CELL_RE.replace_all(&result, "|").to_string();

    // Escape $ in wikilinks
    result = WIKILINK_DOLLAR_RE
        .replace_all(&result, |caps: &Captures| {
            let inner = &caps[1];
            format!("[[{}]]", inner.replace('$', "\\$"))
        })
        .to_string();

    // Escape standalone $ tokens
    result = DOLLAR_TOKEN_RE
        .replace_all(&result, |caps: &Captures| format!("\\${}", &caps[1]))
        .to_string();

    // Convert embeds
    result = EMBED_RE.replace_all(&result, "![[$1]]").to_string();

    // Add pages/ prefix to wikilinks
    result = WIKILINK_RE
        .replace_all(&result, |caps: &Captures| {
            let embed = caps.get(1).map_or("", |m| m.as_str());
            let link = &caps[2];
            let alias = caps.get(3).map_or("", |m| m.as_str());

            // Skip if already has pages/, journals/, favorites/, assets/ prefix or is special
            if link.starts_with("pages/")
                || link.starts_with("journals/")
                || link.starts_with("favorites/")
                || link.starts_with("assets/")
                || link.starts_with("http")
                || link.starts_with('#')
            {
                return format!("{}[[{}{}]]", embed, link, alias);
            }

            format!("{}[[pages/{}{}]]", embed, link, alias)
        })
        .to_string();

    // Execute queries and replace with results
    result = QUERY_RE
        .replace_all(&result, |caps: &Captures| {
            let full_match = &caps[0];
            let list_marker = caps.get(1).map_or("", |m| m.as_str());
            let query_str = full_match.trim_start_matches('-').trim();

            let results = query::execute(query_str, page_index);
            let output = query::results_to_markdown(&results, query_str);

            if list_marker.is_empty() {
                output
            } else {
                format!("{}{}", list_marker, output.replace('\n', &format!("\n{}", list_marker)))
            }
        })
        .to_string();

    // Block embed placeholder
    result = BLOCK_EMBED_RE
        .replace_all(&result, "*Block embed - view in Logseq*")
        .to_string();

    // Block references
    result = BLOCK_REF_RE
        .replace_all(&result, "[→ block](#^$1)")
        .to_string();

    // Media embeds
    result = YOUTUBE_RE.replace_all(&result, "![$1]($1)").to_string();
    result = VIDEO_RE.replace_all(&result, "![$1]($1)").to_string();
    result = PDF_RE.replace_all(&result, "![$1]($1)").to_string();

    // Renderer placeholder
    result = RENDERER_RE.replace_all(&result, "`[renderer]`").to_string();

    // Hiccup/EDN syntax - wrap in code block
    result = HICCUP_RE
        .replace_all(&result, |caps: &Captures| {
            format!("```clojure\n{}\n```", &caps[0])
        })
        .to_string();

    // Cloze to highlight
    result = CLOZE_RE.replace_all(&result, "==$1==").to_string();

    // Task markers
    result = DONE_RE.replace_all(&result, "$1- [x] ").to_string();
    result = TODO_RE.replace_all(&result, "$1- [ ] ").to_string();
    result = NOW_RE.replace_all(&result, "$1- [ ] 🔄 ").to_string();
    result = DOING_RE.replace_all(&result, "$1- [ ] 🔄 ").to_string();
    result = LATER_RE.replace_all(&result, "$1- [ ] 📅 ").to_string();
    result = WAITING_RE.replace_all(&result, "$1- [ ] ⏳ ").to_string();
    result = CANCELLED_RE.replace_all(&result, "$1- [x] ❌ ").to_string();

    // Priority markers
    result = PRIORITY_A_RE.replace_all(&result, "🔴").to_string();
    result = PRIORITY_B_RE.replace_all(&result, "🟡").to_string();
    result = PRIORITY_C_RE.replace_all(&result, "🟢").to_string();

    // Schedule/deadline
    result = SCHEDULED_RE
        .replace_all(&result, "📅 Scheduled: $1")
        .to_string();
    result = DEADLINE_RE
        .replace_all(&result, "⏰ Deadline: $1")
        .to_string();

    result
}
