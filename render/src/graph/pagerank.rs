use crate::parser::PageId;
use std::collections::HashMap;

use super::PageStore;

/// Compute PageRank scores for all pages in the graph.
/// Uses the standard iterative algorithm with damping factor 0.85.
pub fn compute_pagerank(store: &PageStore) -> HashMap<PageId, f64> {
    let page_ids: Vec<&PageId> = store.pages.keys().collect();
    let n = page_ids.len();
    if n == 0 {
        return HashMap::new();
    }

    let damping = 0.85;
    let max_iterations = 50;
    let convergence = 1e-6;
    let base = (1.0 - damping) / n as f64;

    // Index mapping: page_id -> index
    let id_to_idx: HashMap<&PageId, usize> = page_ids
        .iter()
        .enumerate()
        .map(|(i, id)| (*id, i))
        .collect();

    // Build outgoing link counts and adjacency (who links to whom)
    // For each node, collect which nodes point to it and their out-degree
    let mut in_links: Vec<Vec<(usize, usize)>> = vec![vec![]; n]; // [(source_idx, source_out_degree)]

    for (source_id, targets) in &store.forward_links {
        let Some(&src_idx) = id_to_idx.get(source_id) else {
            continue;
        };
        // Only count targets that exist in our page set
        let valid_targets: Vec<usize> = targets
            .iter()
            .filter_map(|t| id_to_idx.get(t).copied())
            .collect();
        let out_degree = valid_targets.len();
        if out_degree == 0 {
            continue;
        }
        for tgt_idx in valid_targets {
            in_links[tgt_idx].push((src_idx, out_degree));
        }
    }

    // Identify dangling nodes (no outgoing links to valid pages)
    let has_outlinks: Vec<bool> = (0..n)
        .map(|i| {
            let id = page_ids[i];
            store
                .forward_links
                .get(id)
                .map(|targets| targets.iter().any(|t| id_to_idx.contains_key(t)))
                .unwrap_or(false)
        })
        .collect();

    let mut rank = vec![1.0 / n as f64; n];

    for _ in 0..max_iterations {
        // Sum of ranks of dangling nodes
        let dangling_sum: f64 = rank
            .iter()
            .enumerate()
            .filter(|(i, _)| !has_outlinks[*i])
            .map(|(_, r)| r)
            .sum();

        let dangling_contrib = damping * dangling_sum / n as f64;

        let mut new_rank = vec![base + dangling_contrib; n];

        for i in 0..n {
            for &(src_idx, out_degree) in &in_links[i] {
                new_rank[i] += damping * rank[src_idx] / out_degree as f64;
            }
        }

        // Check convergence
        let delta: f64 = new_rank
            .iter()
            .zip(rank.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();

        rank = new_rank;

        if delta < convergence {
            break;
        }
    }

    // Map back to PageId
    page_ids
        .into_iter()
        .enumerate()
        .map(|(i, id)| (id.clone(), rank[i]))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::build_graph;
    use crate::parser::{slugify_page_name, PageKind, PageMeta, ParsedPage};
    use std::path::PathBuf;

    fn make_page(name: &str, links: Vec<&str>) -> ParsedPage {
        ParsedPage {
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
            source_path: PathBuf::from(format!("pages/{}.md", name)),
            namespace: None,
            subgraph: None,
            content_md: String::new(),
            outgoing_links: links.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn test_pagerank_simple() {
        // A -> B -> C -> A (cycle), plus D -> A
        let pages = vec![
            make_page("A", vec!["B"]),
            make_page("B", vec!["C"]),
            make_page("C", vec!["A"]),
            make_page("D", vec!["A"]),
        ];
        let store = build_graph(pages).unwrap();
        let ranks = compute_pagerank(&store);

        let a = slugify_page_name("A");
        let d = slugify_page_name("D");

        // A should have highest rank (gets link from C and D)
        assert!(ranks[&a] > ranks[&d]);
    }

    #[test]
    fn test_pagerank_sums_to_one() {
        let pages = vec![
            make_page("X", vec!["Y", "Z"]),
            make_page("Y", vec!["Z"]),
            make_page("Z", vec!["X"]),
        ];
        let store = build_graph(pages).unwrap();
        let ranks = compute_pagerank(&store);
        let total: f64 = ranks.values().sum();
        assert!((total - 1.0).abs() < 0.01);
    }
}
