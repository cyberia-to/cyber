use lazy_static::lazy_static;
use regex::Regex;

use crate::page::{Page, PageIndex};

lazy_static! {
    // Query patterns
    static ref PAGE_TAGS_RE: Regex = Regex::new(r"(?i)^\(page-tags\s+\[\[([^\]]+)\]\]\)$").unwrap();
    static ref PAGE_RE: Regex = Regex::new(r"(?i)^\(page\s+\[\[([^\]]+)\]\]\)$").unwrap();
    static ref NAMESPACE_RE: Regex = Regex::new(r"(?i)^\(namespace\s+\[\[([^\]]+)\]\]\)$").unwrap();
    static ref PROPERTY_RE: Regex = Regex::new(r#"(?i)^\((?:page-)?property\s+:?(\w+[-\w]*)(?:\s+(?:"([^"]+)"|(\S+)))?\)$"#).unwrap();
    static ref TASK_RE: Regex = Regex::new(r"(?i)^\(task\s+(TODO|DONE|NOW|DOING|LATER|WAITING|CANCELLED)\)$").unwrap();
    static ref PAGE_REF_RE: Regex = Regex::new(r"^\[\[([^\]]+)\]\]$").unwrap();
    static ref TEXT_SEARCH_RE: Regex = Regex::new(r#"^"([^"]+)"$"#).unwrap();

    // Query options
    static ref QUERY_PROPS_RE: Regex = Regex::new(r"query-properties::\s*\[([^\]]+)\]").unwrap();
    static ref QUERY_SORT_BY_RE: Regex = Regex::new(r"query-sort-by::\s*(\S+)").unwrap();
    static ref QUERY_SORT_DESC_RE: Regex = Regex::new(r"query-sort-desc::\s*(true|false)").unwrap();
}

/// Execute a Logseq query and return matching pages
pub fn execute<'a>(query_str: &str, index: &'a PageIndex) -> Vec<&'a Page> {
    // Extract the query expression from {{query ...}}
    let expr = query_str
        .trim()
        .trim_start_matches("{{query")
        .trim_end_matches("}}")
        .trim();

    execute_expr(expr, index)
}

fn execute_expr<'a>(expr: &str, index: &'a PageIndex) -> Vec<&'a Page> {
    let expr = expr.trim();

    // Handle (and ...)
    if expr.starts_with("(and ") && expr.ends_with(')') {
        let inner = &expr[5..expr.len() - 1];
        return execute_and(inner, index);
    }

    // Handle (or ...)
    if expr.starts_with("(or ") && expr.ends_with(')') {
        let inner = &expr[4..expr.len() - 1];
        return execute_or(inner, index);
    }

    // Handle (not ...)
    if expr.starts_with("(not ") && expr.ends_with(')') {
        let inner = &expr[5..expr.len() - 1];
        let excluded = execute_expr(inner, index);
        let excluded_names: std::collections::HashSet<_> =
            excluded.iter().map(|p| &p.name).collect();
        return index.iter().filter(|p| !excluded_names.contains(&p.name)).collect();
    }

    // Handle (task STATE)
    if let Some(caps) = TASK_RE.captures(expr) {
        let state = caps.get(1).unwrap().as_str().to_uppercase();
        return index
            .iter()
            .filter(|p| {
                p.content.contains(&format!("- {} ", state))
                    || p.content.contains(&format!("\n{} ", state))
            })
            .collect();
    }

    // Handle (page [[name]])
    if let Some(caps) = PAGE_RE.captures(expr) {
        let page_name = caps.get(1).unwrap().as_str().to_lowercase();
        // Strip pages/ prefix if present
        let page_name = page_name.strip_prefix("pages/").unwrap_or(&page_name);
        return index.iter().filter(|p| p.name_lower == page_name).collect();
    }

    // Handle (page-tags [[tag]])
    if let Some(caps) = PAGE_TAGS_RE.captures(expr) {
        let tag = caps.get(1).unwrap().as_str().to_lowercase();
        // Strip pages/ prefix if present
        let tag = tag.strip_prefix("pages/").unwrap_or(&tag);
        return index.iter().filter(|p| p.tags.contains(&tag.to_string())).collect();
    }

    // Handle (namespace [[x]])
    if let Some(caps) = NAMESPACE_RE.captures(expr) {
        let ns = caps.get(1).unwrap().as_str().to_lowercase();
        let ns = ns.strip_prefix("pages/").unwrap_or(&ns);
        return index
            .iter()
            .filter(|p| p.namespace.as_ref().map_or(false, |n| n.to_lowercase() == ns))
            .collect();
    }

    // Handle (property :key value)
    if let Some(caps) = PROPERTY_RE.captures(expr) {
        let key = caps.get(1).unwrap().as_str().to_lowercase().replace('-', "");
        let value = caps
            .get(2)
            .or_else(|| caps.get(3))
            .map(|m| m.as_str().to_lowercase().trim_matches('"').to_string())
            .unwrap_or_default();

        return index
            .iter()
            .filter(|p| {
                let prop_val = p.properties.get(&key).map(|v| v.to_lowercase()).unwrap_or_default();
                if value.is_empty() {
                    !prop_val.is_empty()
                } else {
                    prop_val == value || prop_val.contains(&value)
                }
            })
            .collect();
    }

    // Handle [[page]] reference
    if let Some(caps) = PAGE_REF_RE.captures(expr) {
        let page_name = caps.get(1).unwrap().as_str().to_lowercase();
        let page_name = page_name.strip_prefix("pages/").unwrap_or(&page_name);
        return index
            .iter()
            .filter(|p| {
                p.name_lower == page_name
                    || p.content.to_lowercase().contains(&format!("[[{}]]", page_name))
            })
            .collect();
    }

    // Handle "text" search
    if let Some(caps) = TEXT_SEARCH_RE.captures(expr) {
        let search = caps.get(1).unwrap().as_str().to_lowercase();
        return index
            .iter()
            .filter(|p| p.content.to_lowercase().contains(&search))
            .collect();
    }

    // Plain text search
    if !expr.starts_with('(') && !expr.starts_with('[') {
        let search = expr.to_lowercase().replace(['"', '\''], "");
        if search.len() > 2 {
            return index
                .iter()
                .filter(|p| p.content.to_lowercase().contains(&search))
                .collect();
        }
    }

    Vec::new()
}

fn execute_and<'a>(inner: &str, index: &'a PageIndex) -> Vec<&'a Page> {
    let parts = parse_query_parts(inner);
    if parts.is_empty() {
        return Vec::new();
    }

    let mut result: Vec<&Page> = execute_expr(&parts[0], index);
    for part in parts.iter().skip(1) {
        let matching = execute_expr(part, index);
        let matching_names: std::collections::HashSet<_> =
            matching.iter().map(|p| &p.name).collect();
        result.retain(|p| matching_names.contains(&p.name));
    }
    result
}

fn execute_or<'a>(inner: &str, index: &'a PageIndex) -> Vec<&'a Page> {
    let parts = parse_query_parts(inner);
    let mut seen = std::collections::HashSet::new();
    let mut result = Vec::new();

    for part in parts {
        for page in execute_expr(&part, index) {
            if seen.insert(&page.name) {
                result.push(page);
            }
        }
    }
    result
}

/// Parse query parts handling nested parentheses
fn parse_query_parts(expr: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    let mut in_brackets = 0;

    for c in expr.chars() {
        match c {
            '(' => {
                depth += 1;
                current.push(c);
            }
            ')' => {
                depth -= 1;
                current.push(c);
                if depth == 0 {
                    let trimmed = current.trim().to_string();
                    if !trimmed.is_empty() {
                        parts.push(trimmed);
                    }
                    current.clear();
                }
            }
            '[' => {
                in_brackets += 1;
                current.push(c);
            }
            ']' => {
                in_brackets -= 1;
                current.push(c);
                if depth == 0 && in_brackets == 0 {
                    let trimmed = current.trim().to_string();
                    if !trimmed.is_empty() && (trimmed.starts_with('(') || trimmed.starts_with('[')) {
                        parts.push(trimmed);
                    }
                    current.clear();
                }
            }
            ' ' | '\n' | '\t' if depth == 0 && in_brackets == 0 => {
                let trimmed = current.trim().to_string();
                if !trimmed.is_empty() && (trimmed.starts_with('(') || trimmed.starts_with('[')) {
                    parts.push(trimmed);
                }
                current.clear();
            }
            _ => current.push(c),
        }
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() && (trimmed.starts_with('(') || trimmed.starts_with('[')) {
        parts.push(trimmed);
    }

    parts
}

/// Convert query results to markdown
pub fn results_to_markdown(results: &[&Page], query_str: &str) -> String {
    if results.is_empty() {
        return format!(
            "> [!info] Query Results\n> No pages match this query.\n> `{}`",
            if query_str.len() > 80 {
                format!("{}...", &query_str[..80])
            } else {
                query_str.to_string()
            }
        );
    }

    // Check for query options in the query string (simplified - would need context)
    // For now, just render as list
    let mut sorted: Vec<_> = results.iter().collect();
    sorted.sort_by(|a, b| a.name.cmp(&b.name));

    sorted
        .iter()
        .map(|p| {
            let icon = p.properties.get("icon").map_or("", |s| s.as_str());
            let title = p.properties.get("title").map_or(
                p.name.replace('_', " "),
                |t| t.clone(),
            );
            format!(
                "- [[pages/{}|{}{}]]",
                p.name,
                if icon.is_empty() { String::new() } else { format!("{} ", icon) },
                title
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
