use crate::parser::slugify_page_name;
use std::collections::HashMap;

use super::PageStore;

/// Build link indices and return a map of slug → original page name
/// (for pages that don't have source files, captured from wikilink text).
pub fn build_link_indices(store: &mut PageStore) -> HashMap<String, String> {
    let page_ids: Vec<String> = store.pages.keys().cloned().collect();

    // Track original names for slugs (from wikilink text)
    let mut original_names: HashMap<String, String> = HashMap::new();

    // Initialize backlinks for all pages
    for id in &page_ids {
        store.backlinks.entry(id.clone()).or_default();
    }

    // Build forward links and backlinks
    for id in &page_ids {
        let outgoing = store.pages[id].outgoing_links.clone();
        let mut forward = Vec::new();

        for link_name in &outgoing {
            let target_slug = slugify_page_name(link_name);

            // Resolve through aliases
            let resolved = store
                .alias_map
                .get(&target_slug)
                .cloned()
                .unwrap_or(target_slug);

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
        for tag in &store.pages[id].meta.tags.clone() {
            let tag_slug = slugify_page_name(tag);
            let resolved = store
                .alias_map
                .get(&tag_slug)
                .cloned()
                .unwrap_or(tag_slug);

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
