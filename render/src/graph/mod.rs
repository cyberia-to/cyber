mod links;
mod namespaces;
pub mod pagerank;
mod tags;
pub mod trikernel;

use crate::config::ContentSection;
use crate::parser::{slugify_page_name, PageId, ParsedPage};
use anyhow::Result;
use serde::Serialize;
use std::collections::{HashMap, HashSet};

/// The complete knowledge graph.
#[derive(Debug, Clone)]
pub struct PageStore {
    pub pages: HashMap<PageId, ParsedPage>,
    pub forward_links: HashMap<PageId, Vec<PageId>>,
    pub backlinks: HashMap<PageId, Vec<PageId>>,
    pub tag_index: HashMap<String, Vec<PageId>>,
    pub namespace_tree: HashMap<String, Vec<PageId>>,
    /// Alias → canonical PageId
    pub alias_map: HashMap<String, PageId>,
    /// Pages created as stubs (referenced but have no source file)
    pub stub_pages: HashSet<PageId>,
    /// Subgraph name → set of PageIds belonging to that subgraph
    pub subgraph_pages: HashMap<String, HashSet<PageId>>,
    /// PageRank scores for each page
    pub pagerank: HashMap<PageId, f64>,
    /// Tri-kernel focus distribution π (diffusion + springs + heat)
    pub focus: HashMap<PageId, f64>,
    /// Gravity: G_i = π_i × Σ(π_j / d²), 2-hop approximation
    pub gravity: HashMap<PageId, f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BacklinkEntry {
    pub title: String,
    pub url: String,
    pub page_id: PageId,
}

impl PageStore {
    pub fn resolve_page_id(&self, name: &str) -> Option<&PageId> {
        let slug = slugify_page_name(name);
        if self.pages.contains_key(&slug) {
            return Some(self.pages.get_key_value(&slug).unwrap().0);
        }
        // Check aliases
        self.alias_map.get(&slug)
    }

    pub fn get_backlinks(&self, page_id: &str) -> Vec<BacklinkEntry> {
        self.backlinks
            .get(page_id)
            .map(|ids| {
                ids.iter()
                    .filter_map(|id| {
                        self.pages.get(id).map(|page| BacklinkEntry {
                            title: page.meta.title.clone(),
                            url: format!("/{}", id),
                            page_id: id.clone(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn get_namespace_children(&self, namespace: &str) -> Vec<&ParsedPage> {
        self.namespace_tree
            .get(namespace)
            .map(|ids| ids.iter().filter_map(|id| self.pages.get(id)).collect())
            .unwrap_or_default()
    }

    /// Check if a page should be published given content settings.
    pub fn is_page_public(page: &ParsedPage, content: &ContentSection) -> bool {
        if !content.public_only {
            return true;
        }
        match page.meta.public {
            Some(true) => true,
            Some(false) => false,
            None => content.default_public,
        }
    }

    /// Get all pages that pass the public/private filter.
    pub fn public_pages(&self, content: &ContentSection) -> Vec<&ParsedPage> {
        self.pages
            .values()
            .filter(|p| Self::is_page_public(p, content))
            .collect()
    }

    pub fn all_tags(&self, content: &ContentSection) -> Vec<(String, usize)> {
        let mut tag_counts: HashMap<String, usize> = HashMap::new();
        for page in self.pages.values() {
            if !Self::is_page_public(page, content) {
                continue;
            }
            for tag in &page.meta.tags {
                *tag_counts.entry(tag.to_lowercase()).or_default() += 1;
            }
        }
        let mut tags: Vec<_> = tag_counts.into_iter().collect();
        tags.sort_by(|a, b| b.1.cmp(&a.1));
        tags
    }

    /// Get filtered tag → page_ids mapping (only public pages).
    pub fn public_tag_index(&self, content: &ContentSection) -> HashMap<String, Vec<PageId>> {
        let mut index: HashMap<String, Vec<PageId>> = HashMap::new();
        for (id, page) in &self.pages {
            if !Self::is_page_public(page, content) {
                continue;
            }
            for tag in &page.meta.tags {
                index
                    .entry(tag.to_lowercase())
                    .or_default()
                    .push(id.clone());
            }
        }
        index
    }

    pub fn recent_pages(&self, count: usize, content: &ContentSection) -> Vec<&ParsedPage> {
        let mut pages: Vec<_> = self
            .pages
            .values()
            .filter(|p| Self::is_page_public(p, content))
            .collect();
        // Sort by date if available, otherwise by title
        pages.sort_by(|a, b| match (&b.meta.date, &a.meta.date) {
            (Some(bd), Some(ad)) => bd.cmp(ad),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.meta.title.cmp(&b.meta.title),
        });
        pages.truncate(count);
        pages
    }
}

pub fn build_graph(pages: Vec<ParsedPage>) -> Result<PageStore> {
    let mut store = PageStore {
        pages: HashMap::new(),
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
    };

    // First pass: insert all pages, build alias map, and populate subgraph index
    for page in pages {
        let id = page.id.clone();

        // Register aliases
        for alias in &page.meta.aliases {
            let alias_slug = slugify_page_name(alias);
            store.alias_map.insert(alias_slug, id.clone());
        }

        // Track subgraph membership
        if let Some(ref sg) = page.subgraph {
            store
                .subgraph_pages
                .entry(sg.clone())
                .or_default()
                .insert(id.clone());
        }

        store.pages.insert(id, page);
    }

    // Second pass: build forward links and backlinks
    let original_names = links::build_link_indices(&mut store);

    // Third pass: create stub pages for referenced-but-missing pages.
    // Any page that has at least 1 incoming link but no source file gets a stub.
    create_stub_pages(&mut store, &original_names);

    // Build tag index
    tags::build_tag_index(&mut store);

    // Build namespace tree
    namespaces::build_namespace_tree(&mut store);

    // Compute PageRank
    store.pagerank = pagerank::compute_pagerank(&store);

    // Compute tri-kernel focus distribution
    store.focus = trikernel::compute_trikernel(&store);

    // Compute gravity: G_i = π_i × Σ(π_j / d²), 2-hop approximation
    store.gravity = compute_gravity(&store);

    Ok(store)
}

/// Compute gravity for each node: G_i = π_i × Σ_{j: d(i,j) ≤ 2}(π_j / d(i,j)²).
/// Uses 2-hop BFS approximation: O(n × avg_degree²) instead of O(n²).
fn compute_gravity(store: &PageStore) -> HashMap<PageId, f64> {
    let mut gravity = HashMap::new();
    for (id, _) in &store.pages {
        let pi = store.focus.get(id).copied().unwrap_or(0.0);
        if pi == 0.0 {
            gravity.insert(id.clone(), 0.0);
            continue;
        }

        // d=1 neighbors (forward ∪ backward)
        let mut d1: HashSet<&PageId> = HashSet::new();
        if let Some(fwd) = store.forward_links.get(id) {
            for t in fwd {
                d1.insert(t);
            }
        }
        if let Some(back) = store.backlinks.get(id) {
            for t in back {
                d1.insert(t);
            }
        }

        // Σ π_j / 1² for d=1 neighbors
        let mut sum = 0.0;
        for j in &d1 {
            sum += store.focus.get(*j).copied().unwrap_or(0.0);
        }

        // d=2 neighbors (neighbors of d1, excluding d1 and self)
        let mut d2: HashSet<&PageId> = HashSet::new();
        for j in &d1 {
            if let Some(fwd) = store.forward_links.get(*j) {
                for t in fwd {
                    if t != id && !d1.contains(t) {
                        d2.insert(t);
                    }
                }
            }
            if let Some(back) = store.backlinks.get(*j) {
                for t in back {
                    if t != id && !d1.contains(t) {
                        d2.insert(t);
                    }
                }
            }
        }

        // Σ π_j / 4 for d=2 neighbors
        for j in &d2 {
            sum += store.focus.get(*j).copied().unwrap_or(0.0) / 4.0;
        }

        gravity.insert(id.clone(), pi * sum);
    }
    gravity
}

/// Create stub pages for every page ID that is referenced (has backlinks)
/// but doesn't have a source file. These stubs get rendered with backlinks
/// and appear in the graph as real nodes.
fn create_stub_pages(store: &mut PageStore, original_names: &HashMap<String, String>) {
    use crate::parser::{PageKind, PageMeta};
    use std::path::PathBuf;

    // Collect all page IDs that have incoming links but no page entry
    let missing_ids: Vec<PageId> = store
        .backlinks
        .keys()
        .filter(|id| !store.pages.contains_key(*id))
        .cloned()
        .collect();

    for id in missing_ids {
        // Use the original wikilink name if we captured it, otherwise reconstruct from slug
        let title = original_names
            .get(&id)
            .cloned()
            .unwrap_or_else(|| id.replace('-', " "));

        let stub = ParsedPage {
            id: id.clone(),
            meta: PageMeta {
                title,
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

        store.stub_pages.insert(id.clone());
        store.pages.insert(id, stub);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{PageKind, PageMeta};
    use std::path::PathBuf;

    fn make_page(name: &str, links: Vec<&str>, tags: Vec<&str>) -> ParsedPage {
        ParsedPage {
            id: slugify_page_name(name),
            meta: PageMeta {
                title: name.to_string(),
                properties: HashMap::new(),
                tags: tags.into_iter().map(|s| s.to_string()).collect(),
                public: Some(true),
                aliases: Vec::new(),
                date: None,
                icon: None,
                menu_order: None,
                stake: None,
            },
            kind: PageKind::Page,
            source_path: PathBuf::from(format!("pages/{}.md", name)),
            namespace: None,
            subgraph: None,
            content_md: String::new(),
            outgoing_links: links.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn test_backlink_symmetry() {
        let pages = vec![
            make_page("Page A", vec!["Page B"], vec![]),
            make_page("Page B", vec!["Page A"], vec![]),
        ];

        let store = build_graph(pages).unwrap();
        let a_id = slugify_page_name("Page A");
        let b_id = slugify_page_name("Page B");

        assert!(store.backlinks[&a_id].contains(&b_id));
        assert!(store.backlinks[&b_id].contains(&a_id));
    }

    #[test]
    fn test_tag_index() {
        let pages = vec![
            make_page("Page A", vec![], vec!["rust", "programming"]),
            make_page("Page B", vec![], vec!["rust"]),
        ];

        let store = build_graph(pages).unwrap();
        assert_eq!(store.tag_index["rust"].len(), 2);
        assert_eq!(store.tag_index["programming"].len(), 1);
    }

    /// Direct page match must win over alias when resolving tags and links.
    /// Reproduces: pages/core.md exists, pages/cyber/core.md has alias "CORE"
    /// → tag "core" must backlink to "core", not "cyber-core".
    #[test]
    fn test_direct_page_wins_over_alias() {
        let mut core_page = make_page("core", vec![], vec![]);
        // page with alias that slugifies to "core"
        let mut namespaced = ParsedPage {
            id: "cyber-core".to_string(),
            meta: PageMeta {
                title: "cyber/core".to_string(),
                aliases: vec!["CORE".to_string()],
                ..core_page.meta.clone()
            },
            ..core_page.clone()
        };
        namespaced.source_path = PathBuf::from("pages/cyber/core.md");
        core_page.meta.aliases = vec![];

        // a page tagged "core" — should backlink to "core", not "cyber-core"
        let tagged = make_page("knowledge", vec![], vec!["core"]);

        let store = build_graph(vec![core_page, namespaced, tagged]).unwrap();

        let core_backlinks = &store.backlinks["core"];
        let cyber_core_backlinks = &store.backlinks["cyber-core"];

        assert!(
            core_backlinks.contains(&"knowledge".to_string()),
            "tag 'core' should backlink to direct page 'core'"
        );
        assert!(
            !cyber_core_backlinks.contains(&"knowledge".to_string()),
            "tag 'core' should not backlink to aliased page 'cyber-core'"
        );
    }
}
