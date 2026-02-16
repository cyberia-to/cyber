use crate::config::SiteConfig;
use crate::graph::PageStore;
use anyhow::Result;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct FileEntry {
    pub filename: String,
    pub display_name: String,
    pub ipfs_cid: Option<String>,
    pub ipfs_url: Option<String>,
    pub referencing_pages: Vec<PageRef>,
    pub file_type: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PageRef {
    pub title: String,
    pub url: String,
}

fn classify_file_type(filename: &str) -> &'static str {
    let lower = filename.to_lowercase();
    if lower.ends_with(".png")
        || lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".gif")
        || lower.ends_with(".webp")
        || lower.ends_with(".svg")
    {
        "image"
    } else if lower.ends_with(".pdf") {
        "pdf"
    } else if lower.ends_with(".mp4")
        || lower.ends_with(".mov")
        || lower.ends_with(".webm")
        || lower.ends_with(".avi")
    {
        "video"
    } else if lower.ends_with(".mp3")
        || lower.ends_with(".wav")
        || lower.ends_with(".ogg")
        || lower.ends_with(".flac")
    {
        "audio"
    } else {
        "other"
    }
}

/// Generate a human-readable display name from a filename.
/// Strips Logseq timestamps (_1234567890123_0), replaces underscores/hyphens with spaces.
fn humanize_filename(filename: &str) -> String {
    // Remove extension
    let name = if let Some(pos) = filename.rfind('.') {
        &filename[..pos]
    } else {
        filename
    };

    // Strip Logseq timestamp suffix: _TIMESTAMP_0 (13-digit unix ms + _0)
    let stripped = Regex::new(r"_\d{13}_\d$")
        .unwrap()
        .replace(name, "")
        .to_string();

    // Replace underscores and hyphens with spaces, collapse multiple spaces
    let humanized = stripped.replace('_', " ").replace('-', " ");

    // Collapse whitespace
    let collapsed: String = humanized.split_whitespace().collect::<Vec<_>>().join(" ");

    if collapsed.is_empty() {
        filename.to_string()
    } else {
        collapsed
    }
}

/// Build file index by scanning all public pages for asset references.
/// Loads ipfs-cache.json from input_dir if available to resolve CIDs.
pub fn build_file_index(store: &PageStore, config: &SiteConfig) -> Vec<FileEntry> {
    // Load CID cache if available
    let cache_path = config.build.input_dir.join("ipfs-cache.json");
    let cid_cache: HashMap<String, String> = if cache_path.exists() {
        std::fs::read_to_string(&cache_path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        HashMap::new()
    };

    // Regex for markdown image/link with alt text and local asset path
    // Captures: ![alt text](../assets/filename) or [alt text](../assets/filename)
    let alt_re = Regex::new(r"!?\[([^\]]*)\]\(\.\./assets/([^)\s]+)\)").unwrap();
    // Regex for local asset references without alt text context
    let local_re = Regex::new(r#"\.\./assets/([^)\s"']+)"#).unwrap();
    // Regex for IPFS URLs already rewritten
    let ipfs_re = Regex::new(r"https?://[^/]+/ipfs/(Qm[a-zA-Z0-9]{44,})").unwrap();

    // filename -> (cid, alt_text, vec<page_ref>)
    let mut file_map: HashMap<String, (Option<String>, Option<String>, Vec<PageRef>)> =
        HashMap::new();

    for page in store.public_pages(&config.content) {
        let page_ref = PageRef {
            title: page.meta.title.clone(),
            url: format!("/{}", page.id),
        };

        // First pass: extract alt text from markdown image/link syntax
        for cap in alt_re.captures_iter(&page.content_md) {
            let alt_text = cap[1].to_string();
            let filename = cap[2].to_string();
            let entry = file_map.entry(filename.clone()).or_insert_with(|| {
                let cid = cid_cache.get(&filename).cloned();
                (cid, None, Vec::new())
            });
            // Use alt text as display name if it's meaningful
            if entry.1.is_none() && !alt_text.is_empty() && alt_text != "image.png" {
                entry.1 = Some(alt_text);
            }
            if !entry.2.iter().any(|r| r.url == page_ref.url) {
                entry.2.push(page_ref.clone());
            }
        }

        // Second pass: catch any remaining local asset references not matched by alt_re
        for cap in local_re.captures_iter(&page.content_md) {
            let filename = cap[1].to_string();
            let entry = file_map.entry(filename.clone()).or_insert_with(|| {
                let cid = cid_cache.get(&filename).cloned();
                (cid, None, Vec::new())
            });
            if !entry.2.iter().any(|r| r.url == page_ref.url) {
                entry.2.push(page_ref.clone());
            }
        }

        // Find already-rewritten IPFS URLs
        for cap in ipfs_re.captures_iter(&page.content_md) {
            let cid = cap[1].to_string();
            let filename = cid_cache
                .iter()
                .find(|(_, v)| **v == cid)
                .map(|(k, _)| k.clone())
                .unwrap_or_else(|| cid.clone());

            let entry = file_map
                .entry(filename)
                .or_insert_with(|| (Some(cid.clone()), None, Vec::new()));
            if entry.0.is_none() {
                entry.0 = Some(cid);
            }
            if !entry.2.iter().any(|r| r.url == page_ref.url) {
                entry.2.push(page_ref.clone());
            }
        }
    }

    let gateway = "https://gateway.pinata.cloud";

    let mut entries: Vec<FileEntry> = file_map
        .into_iter()
        .map(|(filename, (cid, alt_text, pages))| {
            let ipfs_url = cid.as_ref().map(|c| format!("{}/ipfs/{}", gateway, c));
            let display_name = alt_text
                .filter(|a| {
                    // Skip generic alt texts and those that are just the filename
                    !a.ends_with(".pdf")
                        && !a.ends_with(".png")
                        && *a != "image.png"
                        && *a != "image"
                })
                .unwrap_or_else(|| {
                    let name = humanize_filename(&filename);
                    // If humanization yields a very short generic name, use first referencing page title
                    if name == "image" || name.is_empty() {
                        if let Some(first_page) = pages.first() {
                            format!("{} (image)", first_page.title)
                        } else {
                            filename.clone()
                        }
                    } else if name.starts_with("Qm") && name.len() > 40 {
                        // Raw CID as filename — use first referencing page
                        if let Some(first_page) = pages.first() {
                            format!("{} (file)", first_page.title)
                        } else {
                            format!("{}…", &name[..12])
                        }
                    } else {
                        name
                    }
                });
            FileEntry {
                file_type: classify_file_type(&filename).to_string(),
                display_name,
                filename,
                ipfs_cid: cid,
                ipfs_url,
                referencing_pages: pages,
            }
        })
        .collect();

    // Sort by number of referencing pages (most referenced first)
    entries.sort_by(|a, b| {
        b.referencing_pages
            .len()
            .cmp(&a.referencing_pages.len())
            .then_with(|| a.filename.cmp(&b.filename))
    });

    entries
}

/// Write files-index.json to output directory
pub fn write_files_index(entries: &[FileEntry], output_dir: &Path) -> Result<()> {
    let json = serde_json::to_string(entries)?;
    std::fs::write(output_dir.join("files-index.json"), json)?;
    Ok(())
}
