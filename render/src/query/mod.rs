mod eval;
mod parse;

use crate::graph::PageStore;
use crate::parser::PageId;
use lazy_static::lazy_static;
use regex::Regex;

pub use parse::QueryExpr;

lazy_static! {
    /// Matches {{query ...}} inline queries
    static ref QUERY_INLINE_RE: Regex = Regex::new(
        r"(?s)\{\{query\s+(.*?)\}\}"
    ).unwrap();

    /// Matches #+BEGIN_QUERY ... #+END_QUERY blocks
    static ref QUERY_BLOCK_RE: Regex = Regex::new(
        r"(?sm)^\s*#\+BEGIN_QUERY\s*\n(.*?)\s*#\+END_QUERY"
    ).unwrap();
}

/// Resolve all query blocks in markdown content, returning HTML with results.
pub fn resolve_queries(markdown: &str, store: &PageStore) -> String {
    // Fast path: skip if no query patterns
    if !markdown.contains("{{query") && !markdown.contains("#+BEGIN_QUERY") {
        return markdown.to_string();
    }

    let mut result = markdown.to_string();

    // Resolve inline {{query ...}} blocks
    result = QUERY_INLINE_RE
        .replace_all(&result, |caps: &regex::Captures| {
            let query_text = caps[1].trim();
            match parse::parse_query(query_text) {
                Some(expr) => {
                    let page_ids = eval::evaluate(&expr, store);
                    render_query_results(query_text, &page_ids, store)
                }
                None => render_query_fallback(query_text),
            }
        })
        .to_string();

    // Resolve #+BEGIN_QUERY ... #+END_QUERY blocks
    result = QUERY_BLOCK_RE
        .replace_all(&result, |caps: &regex::Captures| {
            let query_body = caps[1].trim();
            render_query_fallback(query_body)
        })
        .to_string();

    result
}

/// Render query results as an HTML list of linked page titles.
fn render_query_results(query_text: &str, page_ids: &[PageId], store: &PageStore) -> String {
    if page_ids.is_empty() {
        return format!(
            "<div class=\"query-results\">\
             <div class=\"query-results-header\">Query: <code>{}</code></div>\
             <p><em>No results</em></p></div>",
            html_escape(query_text)
        );
    }

    let mut items = String::new();
    for id in page_ids {
        if let Some(page) = store.pages.get(id) {
            items.push_str(&format!(
                "<li><a href=\"/{}\" class=\"internal-link\">{}</a></li>\n",
                id, html_escape(&page.meta.title)
            ));
        }
    }

    format!(
        "<div class=\"query-results\">\
         <div class=\"query-results-header\">Query: <code>{}</code> ({} results)</div>\
         <ul>\n{}</ul></div>",
        html_escape(query_text),
        page_ids.len(),
        items
    )
}

/// Render an unrecognized query as a styled fallback.
fn render_query_fallback(query_text: &str) -> String {
    format!(
        "<div class=\"query-fallback\">\
         <code>{}</code>\
         <div class=\"query-note\">This query uses advanced features. View in Logseq for live results.</div>\
         </div>",
        html_escape(query_text)
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
