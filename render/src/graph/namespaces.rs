use super::PageStore;

pub fn build_namespace_tree(store: &mut PageStore) {
    let entries: Vec<(String, Option<String>)> = store
        .pages
        .iter()
        .map(|(id, page)| (id.clone(), page.namespace.clone()))
        .collect();

    for (page_id, namespace) in entries {
        if let Some(ns) = namespace {
            store
                .namespace_tree
                .entry(ns)
                .or_default()
                .push(page_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{PageMeta, PageKind, ParsedPage};
    use std::collections::{HashMap, HashSet};
    use std::path::PathBuf;

    /// Helper to create a minimal PageStore with given pages.
    fn make_store(pages: Vec<ParsedPage>) -> PageStore {
        let mut page_map = HashMap::new();
        for page in pages {
            page_map.insert(page.id.clone(), page);
        }
        PageStore {
            pages: page_map,
            forward_links: HashMap::new(),
            backlinks: HashMap::new(),
            tag_index: HashMap::new(),
            namespace_tree: HashMap::new(),
            alias_map: HashMap::new(),
            stub_pages: HashSet::new(),
            subgraph_pages: HashMap::new(),
            pagerank: HashMap::new(),
            focus: HashMap::new(),
            gravity: HashMap::new(),
        }
    }

    fn make_page(id: &str, namespace: Option<&str>) -> ParsedPage {
        ParsedPage {
            id: id.to_string(),
            meta: PageMeta {
                title: id.to_string(),
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
            namespace: namespace.map(|s| s.to_string()),
            subgraph: None,
            content_md: String::new(),
            outgoing_links: vec![],
        }
    }

    #[test]
    fn test_namespace_tree_basic() {
        let pages = vec![
            make_page("ns-a", Some("ns")),
            make_page("ns-b", Some("ns")),
        ];
        let mut store = make_store(pages);
        build_namespace_tree(&mut store);

        assert!(store.namespace_tree.contains_key("ns"));
        let children = store.namespace_tree.get("ns").unwrap();
        assert_eq!(children.len(), 2);
        assert!(children.contains(&"ns-a".to_string()));
        assert!(children.contains(&"ns-b".to_string()));
    }

    #[test]
    fn test_namespace_tree_root_pages_excluded() {
        let pages = vec![
            make_page("root-page", None),
            make_page("another-root", None),
            make_page("ns-child", Some("ns")),
        ];
        let mut store = make_store(pages);
        build_namespace_tree(&mut store);

        // Root pages (namespace = None) should not appear in namespace_tree
        assert!(!store.namespace_tree.contains_key(""));
        // Only "ns" should exist
        assert_eq!(store.namespace_tree.len(), 1);
        assert!(store.namespace_tree.contains_key("ns"));
    }

    #[test]
    fn test_namespace_tree_nested() {
        let pages = vec![
            make_page("a-b-c", Some("a/b")),
            make_page("a-b-d", Some("a/b")),
            make_page("a-x", Some("a")),
        ];
        let mut store = make_store(pages);
        build_namespace_tree(&mut store);

        // "a/b" should have the two nested children
        assert!(store.namespace_tree.contains_key("a/b"));
        let nested = store.namespace_tree.get("a/b").unwrap();
        assert_eq!(nested.len(), 2);
        assert!(nested.contains(&"a-b-c".to_string()));
        assert!(nested.contains(&"a-b-d".to_string()));

        // "a" should have the direct child
        assert!(store.namespace_tree.contains_key("a"));
        let direct = store.namespace_tree.get("a").unwrap();
        assert_eq!(direct.len(), 1);
        assert!(direct.contains(&"a-x".to_string()));
    }
}
