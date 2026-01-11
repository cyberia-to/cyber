#[cfg(test)]
mod path_tests {
    use crate::content;
    use crate::page::{self, PageIndex};

    fn empty_index() -> PageIndex {
        Vec::new()
    }

    #[test]
    fn test_wikilink_adds_pages_prefix() {
        let input = "Check out [[devops]] for more info.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[[pages/devops]]"),
            "Expected pages/ prefix, got: {}",
            result
        );
    }

    #[test]
    fn test_wikilink_preserves_existing_path() {
        let input = "See [[cyber/energy]] for details.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[[cyber/energy]]"),
            "Should preserve existing path, got: {}",
            result
        );
    }

    #[test]
    fn test_wikilink_with_alias() {
        let input = "Check [[devops|DevOps Guide]] here.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[[pages/devops|DevOps Guide]]"),
            "Should preserve alias with pages/ prefix, got: {}",
            result
        );
    }

    #[test]
    fn test_embed_adds_pages_prefix() {
        let input = "{{embed [[intro]]}}";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("![[pages/intro]]"),
            "Embed should have pages/ prefix, got: {}",
            result
        );
    }

    #[test]
    fn test_http_links_unchanged() {
        let input = "Visit [[https://example.com]] for info.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[[https://example.com]]"),
            "HTTP links should be unchanged, got: {}",
            result
        );
    }

    #[test]
    fn test_anchor_links_unchanged() {
        let input = "See [[#section]] below.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[[#section]]"),
            "Anchor links should be unchanged, got: {}",
            result
        );
    }

    #[test]
    fn test_task_markers_converted() {
        let input = "- TODO Buy groceries\n- DONE Clean room";
        let result = content::transform(input, &empty_index());
        assert!(result.contains("- [ ] Buy groceries"), "TODO should convert to [ ]");
        assert!(result.contains("- [x] Clean room"), "DONE should convert to [x]");
    }

    #[test]
    fn test_dollar_tokens_escaped() {
        let input = "Token price: $ETH is rising.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("\\$ETH"),
            "Dollar tokens should be escaped, got: {}",
            result
        );
    }

    #[test]
    fn test_cloze_converted_to_highlight() {
        let input = "The answer is {{cloze 42}}.";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("==42=="),
            "Cloze should convert to highlight, got: {}",
            result
        );
    }

    #[test]
    fn test_block_reference_converted() {
        let input = "See ((a1b2c3d4-e5f6-7890-abcd-ef1234567890)).";
        let result = content::transform(input, &empty_index());
        assert!(
            result.contains("[→ block](#^a1b2c3d4-e5f6-7890-abcd-ef1234567890)"),
            "Block ref should convert, got: {}",
            result
        );
    }
}

#[cfg(test)]
mod property_tests {
    use crate::page::parse_properties;

    #[test]
    fn test_parse_simple_property() {
        let content = "title:: My Page\n- Content here";
        let (props, remaining) = parse_properties(content);
        assert_eq!(props.get("title"), Some(&"My Page".to_string()));
        assert!(remaining.contains("Content here"));
    }

    #[test]
    fn test_parse_multiple_properties() {
        let content = "title:: Test\ntags:: foo, bar\nicon:: 🔵\n\n- Content";
        let (props, _) = parse_properties(content);
        assert_eq!(props.get("title"), Some(&"Test".to_string()));
        assert_eq!(props.get("tags"), Some(&"foo, bar".to_string()));
        assert_eq!(props.get("icon"), Some(&"🔵".to_string()));
    }

    #[test]
    fn test_parse_property_with_list_marker() {
        let content = "- title:: My Page\n- Content";
        let (props, _) = parse_properties(content);
        assert_eq!(props.get("title"), Some(&"My Page".to_string()));
    }
}

#[cfg(test)]
mod frontmatter_tests {
    use crate::frontmatter;
    use std::collections::HashMap;

    #[test]
    fn test_frontmatter_with_icon() {
        let mut props = HashMap::new();
        props.insert("icon".to_string(), "🔵".to_string());
        props.insert("title".to_string(), "Test Page".to_string());

        let fm = frontmatter::generate("test", &props, None);
        assert!(fm.contains("title: \"🔵 Test Page\""));
        assert!(fm.contains("icon: \"🔵\""));
    }

    #[test]
    fn test_frontmatter_with_tags() {
        let mut props = HashMap::new();
        props.insert("tags".to_string(), "foo, bar, baz".to_string());

        let fm = frontmatter::generate("test", &props, None);
        assert!(fm.contains("tags:"));
        assert!(fm.contains("  - foo"));
        assert!(fm.contains("  - bar"));
        assert!(fm.contains("  - baz"));
    }

    #[test]
    fn test_frontmatter_with_dates() {
        let props = HashMap::new();
        let fm = frontmatter::generate("test", &props, Some(("2025-01-01", "2024-01-01")));
        assert!(fm.contains("modified: 2025-01-01"));
        assert!(fm.contains("created: 2024-01-01"));
    }

    #[test]
    fn test_frontmatter_escapes_quotes() {
        let mut props = HashMap::new();
        props.insert("title".to_string(), "Test \"quoted\" page".to_string());

        let fm = frontmatter::generate("test", &props, None);
        assert!(fm.contains("Test \\\"quoted\\\" page"));
    }
}

#[cfg(test)]
mod query_tests {
    use crate::page::Page;
    use crate::query;
    use std::collections::HashMap;

    fn create_test_page(name: &str, tags: Vec<&str>) -> Page {
        Page {
            name: name.to_string(),
            name_lower: name.to_lowercase(),
            content: String::new(),
            properties: HashMap::new(),
            tags: tags.into_iter().map(|s| s.to_string()).collect(),
            namespace: None,
            modified: None,
            created: None,
        }
    }

    #[test]
    fn test_query_page_tags() {
        let pages = vec![
            create_test_page("page1", vec!["rust", "programming"]),
            create_test_page("page2", vec!["rust"]),
            create_test_page("page3", vec!["python"]),
        ];

        let results = query::execute("{{query (page-tags [[rust]])}}", &pages);
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_query_page_tags_strips_pages_prefix() {
        let pages = vec![
            create_test_page("page1", vec!["rust"]),
        ];

        let results = query::execute("{{query (page-tags [[pages/rust]])}}", &pages);
        assert_eq!(results.len(), 1, "Should strip pages/ prefix from query");
    }

    #[test]
    fn test_query_and() {
        let pages = vec![
            create_test_page("page1", vec!["rust", "programming"]),
            create_test_page("page2", vec!["rust"]),
        ];

        let results = query::execute("{{query (and (page-tags [[rust]]) (page-tags [[programming]]))}}", &pages);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "page1");
    }

    #[test]
    fn test_query_results_include_pages_prefix() {
        let pages = vec![
            create_test_page("my-page", vec!["test"]),
        ];

        let results = query::execute("{{query (page-tags [[test]])}}", &pages);
        let markdown = query::results_to_markdown(&results, "test query");

        assert!(
            markdown.contains("[[pages/my-page"),
            "Results should include pages/ prefix, got: {}",
            markdown
        );
    }
}
