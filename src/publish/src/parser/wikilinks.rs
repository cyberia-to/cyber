use regex::Regex;

lazy_static::lazy_static! {
    /// Matches [[wikilinks]] and [[wikilinks|display text]]
    static ref WIKILINK_RE: Regex = Regex::new(r"\[\[([^\]|]+)(?:\|[^\]]+)?\]\]").unwrap();
}

/// Extract all wikilink targets from markdown content.
/// Returns raw page names (not slugified).
pub fn collect_wikilinks(content: &str) -> Vec<String> {
    let mut links = Vec::new();

    for caps in WIKILINK_RE.captures_iter(content) {
        if let Some(target) = caps.get(1) {
            let name = target.as_str().trim().to_string();
            if !name.is_empty() && !links.contains(&name) {
                links.push(name);
            }
        }
    }

    links
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_wikilink() {
        let links = collect_wikilinks("This links to [[My Page]] here.");
        assert_eq!(links, vec!["My Page"]);
    }

    #[test]
    fn test_wikilink_with_display_text() {
        let links = collect_wikilinks("See [[Target Page|display text]] for more.");
        assert_eq!(links, vec!["Target Page"]);
    }

    #[test]
    fn test_multiple_wikilinks() {
        let links = collect_wikilinks("Both [[Page A]] and [[Page B]] are referenced.");
        assert_eq!(links, vec!["Page A", "Page B"]);
    }

    #[test]
    fn test_no_duplicates() {
        let links = collect_wikilinks("[[Same Page]] and [[Same Page]] again.");
        assert_eq!(links, vec!["Same Page"]);
    }

    #[test]
    fn test_namespace_wikilink() {
        let links = collect_wikilinks("See [[projects/Cyber Valley]].");
        assert_eq!(links, vec!["projects/Cyber Valley"]);
    }

    #[test]
    fn test_no_wikilinks() {
        let links = collect_wikilinks("No links here, just [regular](links).");
        assert!(links.is_empty());
    }

    #[test]
    fn test_wikilink_in_list() {
        let links = collect_wikilinks("- Item with [[Link One]]\n  - Sub-item with [[Link Two]]");
        assert_eq!(links, vec!["Link One", "Link Two"]);
    }
}
