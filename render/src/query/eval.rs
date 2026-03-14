use crate::graph::PageStore;
use crate::parser::{slugify_page_name, PageId};
use std::collections::HashSet;

use super::parse::QueryExpr;

/// Evaluate a query expression against the PageStore.
/// Returns a list of matching page IDs.
pub fn evaluate(expr: &QueryExpr, store: &PageStore) -> Vec<PageId> {
    let result_set = eval_set(expr, store);
    let mut results: Vec<PageId> = result_set.into_iter().collect();
    results.sort();
    results
}

fn eval_set(expr: &QueryExpr, store: &PageStore) -> HashSet<PageId> {
    match expr {
        QueryExpr::Tag(tag) => {
            let tag_lower = tag.to_lowercase();
            let mut result = HashSet::new();

            // Strictly match pages whose frontmatter tags contain this value
            if let Some(ids) = store.tag_index.get(&tag_lower) {
                result.extend(ids.iter().cloned());
            }

            result
        }

        QueryExpr::And(exprs) => {
            if exprs.is_empty() {
                return all_page_ids(store);
            }
            let mut result = eval_set(&exprs[0], store);
            for expr in &exprs[1..] {
                let other = eval_set(expr, store);
                result = result.intersection(&other).cloned().collect();
            }
            result
        }

        QueryExpr::Or(exprs) => {
            let mut result = HashSet::new();
            for expr in exprs {
                result.extend(eval_set(expr, store));
            }
            result
        }

        QueryExpr::Not(inner) => {
            let excluded = eval_set(inner, store);
            let all = all_page_ids(store);
            all.difference(&excluded).cloned().collect()
        }

        QueryExpr::Property { key, value } => {
            let mut result = HashSet::new();
            for (id, page) in &store.pages {
                match value {
                    Some(val) => {
                        if page
                            .meta
                            .properties
                            .get(key)
                            .map(|v| v.eq_ignore_ascii_case(val))
                            .unwrap_or(false)
                        {
                            result.insert(id.clone());
                        }
                    }
                    None => {
                        if page.meta.properties.contains_key(key) {
                            result.insert(id.clone());
                        }
                    }
                }
            }
            result
        }

        QueryExpr::Namespace(ns) => {
            let ns_slug = slugify_page_name(ns);
            let mut result = HashSet::new();
            if let Some(children) = store.namespace_tree.get(&ns_slug) {
                result.extend(children.iter().cloned());
            }
            // Also include the namespace page itself
            if store.pages.contains_key(&ns_slug) {
                result.insert(ns_slug);
            }
            result
        }

        QueryExpr::Page(name) => {
            let slug = slugify_page_name(name);
            let mut result = HashSet::new();
            if store.pages.contains_key(&slug) {
                result.insert(slug);
            }
            result
        }

        QueryExpr::TextSearch(text) => {
            let text_lower = text.to_lowercase();
            let mut result = HashSet::new();
            for (id, page) in &store.pages {
                if page.content_md.to_lowercase().contains(&text_lower)
                    || page.meta.title.to_lowercase().contains(&text_lower)
                {
                    result.insert(id.clone());
                }
            }
            result
        }
    }
}

fn all_page_ids(store: &PageStore) -> HashSet<PageId> {
    store.pages.keys().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::build_graph;
    use crate::parser::{PageKind, PageMeta, ParsedPage};
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn make_page(name: &str, tags: Vec<&str>, props: Vec<(&str, &str)>) -> ParsedPage {
        let mut properties = HashMap::new();
        for (k, v) in props {
            properties.insert(k.to_string(), v.to_string());
        }
        ParsedPage {
            id: slugify_page_name(name),
            meta: PageMeta {
                title: name.to_string(),
                properties,
                tags: tags.into_iter().map(|s| s.to_string()).collect(),
                public: Some(true),
                aliases: vec![],
                date: None,
                icon: None,
                menu_order: None,
                stake: None,
            },
            kind: PageKind::Page,
            source_path: PathBuf::from(format!("pages/{}.md", name)),
            namespace: None,
            subgraph: None,
            content_md: format!("Content of {}", name),
            outgoing_links: vec![],
        }
    }

    #[test]
    fn test_eval_tag() {
        let store = build_graph(vec![
            make_page("Page A", vec!["research"], vec![]),
            make_page("Page B", vec!["research", "math"], vec![]),
            make_page("Page C", vec!["math"], vec![]),
        ])
        .unwrap();

        let result = evaluate(&QueryExpr::Tag("research".to_string()), &store);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_eval_property_existence() {
        let store = build_graph(vec![
            make_page("Page A", vec![], vec![("supply", "yes")]),
            make_page("Page B", vec![], vec![("supply", "no")]),
            make_page("Page C", vec![], vec![]),
        ])
        .unwrap();

        let result = evaluate(
            &QueryExpr::Property {
                key: "supply".to_string(),
                value: None,
            },
            &store,
        );
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_eval_property_value() {
        let store = build_graph(vec![
            make_page("Page A", vec![], vec![("supply", "yes")]),
            make_page("Page B", vec![], vec![("supply", "no")]),
        ])
        .unwrap();

        let result = evaluate(
            &QueryExpr::Property {
                key: "supply".to_string(),
                value: Some("yes".to_string()),
            },
            &store,
        );
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_eval_and() {
        let store = build_graph(vec![
            make_page("Page A", vec!["research", "math"], vec![]),
            make_page("Page B", vec!["research"], vec![]),
            make_page("Page C", vec!["math"], vec![]),
        ])
        .unwrap();

        let result = evaluate(
            &QueryExpr::And(vec![
                QueryExpr::Tag("research".to_string()),
                QueryExpr::Tag("math".to_string()),
            ]),
            &store,
        );
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], slugify_page_name("Page A"));
    }

    #[test]
    fn test_eval_text_search() {
        let store = build_graph(vec![
            make_page("Page A", vec![], vec![]),
            make_page("Page B", vec![], vec![]),
        ])
        .unwrap();

        let result = evaluate(&QueryExpr::TextSearch("Page A".to_string()), &store);
        assert_eq!(result.len(), 1);
    }
}
