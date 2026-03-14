use crate::parser::slugify_page_name;
use std::collections::HashMap;

use super::PageStore;

/// Build link indices and return a map of slug → original page name
/// (for pages that don't have source files, captured from wikilink text).
pub fn build_link_indices(store: &mut PageStore) -> HashMap<String, String> {
    // Pre-collect page metadata needed for namespace-aware resolution
    let page_info: Vec<(String, Vec<String>, Vec<String>, Option<String>, Option<String>)> = store
        .pages
        .iter()
        .map(|(id, page)| {
            (
                id.clone(),
                page.outgoing_links.clone(),
                page.meta.tags.clone(),
                page.namespace.clone(),
                page.subgraph.clone(),
            )
        })
        .collect();

    // Track original names for slugs (from wikilink text)
    let mut original_names: HashMap<String, String> = HashMap::new();

    // Initialize backlinks for all pages
    for (id, _, _, _, _) in &page_info {
        store.backlinks.entry(id.clone()).or_default();
    }

    // Build forward links and backlinks
    for (id, outgoing, tags, namespace, subgraph) in &page_info {
        let mut forward = Vec::new();

        for link_name in outgoing {
            let resolved =
                resolve_link(link_name, namespace.as_deref(), subgraph.as_deref(), store);

            // Remember original name for stubs (only if no source page exists)
            if !store.pages.contains_key(&resolved) {
                original_names
                    .entry(resolved.clone())
                    .or_insert_with(|| link_name.clone());
            }

            if !forward.contains(&resolved) {
                forward.push(resolved.clone());
            }

            // Add backlink
            store
                .backlinks
                .entry(resolved)
                .or_default()
                .push(id.clone());
        }

        // Add tags as forward links + backlinks (tags are pages)
        for tag in tags {
            let resolved =
                resolve_link(tag, namespace.as_deref(), subgraph.as_deref(), store);

            if !store.pages.contains_key(&resolved) {
                original_names
                    .entry(resolved.clone())
                    .or_insert_with(|| tag.clone());
            }

            if !forward.contains(&resolved) {
                forward.push(resolved.clone());
            }

            store
                .backlinks
                .entry(resolved)
                .or_default()
                .push(id.clone());
        }

        store.forward_links.insert(id.clone(), forward);
    }

    // Deduplicate backlinks
    for backlinks in store.backlinks.values_mut() {
        backlinks.sort();
        backlinks.dedup();
    }

    original_names
}

/// Resolve a wikilink or tag name to a PageId.
///
/// Resolution order:
///   1. Exact slug match in pages
///   2. Alias match
///   3. Namespace-qualified: slugify(namespace/name) — only if source has namespace
///   4. Subgraph-qualified: slugify(subgraph/name) — only if source has subgraph
///   5. Unresolved → return slug as-is (will become stub)
fn resolve_link(
    name: &str,
    source_namespace: Option<&str>,
    source_subgraph: Option<&str>,
    store: &PageStore,
) -> String {
    let target_slug = slugify_page_name(name);

    // 1. Direct page match
    if store.pages.contains_key(&target_slug) {
        return target_slug;
    }

    // 2. Alias match
    if let Some(canonical) = store.alias_map.get(&target_slug) {
        return canonical.clone();
    }

    // 3. Namespace-qualified (e.g., from page in "trident/docs/explanation",
    //    try "trident/docs/explanation/foo")
    if let Some(ns) = source_namespace {
        let ns_slug = slugify_page_name(&format!("{}/{}", ns, name));
        if store.pages.contains_key(&ns_slug) {
            return ns_slug;
        }
    }

    // 4. Subgraph-qualified (e.g., from page in subgraph "trident", try "trident/foo")
    if let Some(sg) = source_subgraph {
        let sg_slug = slugify_page_name(&format!("{}/{}", sg, name));
        if store.pages.contains_key(&sg_slug) {
            return sg_slug;
        }
    }

    // 5. Unresolved — return slug as-is
    target_slug
}
