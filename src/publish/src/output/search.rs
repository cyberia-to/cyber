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
            let excerpt = page
                .content_md
                .lines()
                .filter(|l| !l.trim().is_empty())
                .take(3)
                .collect::<Vec<_>>()
                .join(" ")
                .chars()
                .take(200)
                .collect();

            SearchEntry {
                title: page.meta.title.clone(),
                url: format!("/{}", page.id),
                tags: page.meta.tags.clone(),
                excerpt,
            }
        })
        .collect();

    let json = serde_json::to_string(&entries)?;
    std::fs::write(output_dir.join("search-index.json"), json)?;

    Ok(())
}
