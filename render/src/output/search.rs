use crate::config::SiteConfig;
use crate::graph::PageStore;
use anyhow::Result;
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
struct SearchEntry {
    title: String,
    url: String,
    tags: Vec<String>,
    excerpt: String,
    focus: f64,
}

pub fn generate_search_index(
    store: &PageStore,
    config: &SiteConfig,
    output_dir: &Path,
) -> Result<()> {
    let entries: Vec<SearchEntry> = store
        .public_pages(&config.content)
        .into_iter()
        .map(|page| {
            let excerpt = crate::render::context::generate_excerpt(&page.content_md, 200);
            let focus = store.focus.get(&page.id).copied().unwrap_or(0.0);

            SearchEntry {
                title: page.meta.title.clone(),
                url: format!("/{}", page.id),
                tags: page.meta.tags.clone(),
                excerpt,
                focus: (focus * 100000.0).round() / 100000.0,
            }
        })
        .collect();

    let json = serde_json::to_string(&entries)?;
    std::fs::write(output_dir.join("search-index.json"), json)?;

    Ok(())
}
