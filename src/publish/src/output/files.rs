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

    // Regex for local asset references: ../assets/FILENAME
    let local_re = Regex::new(r#"\.\./assets/([^)\s"']+)"#).unwrap();
    // Regex for IPFS URLs already rewritten: https://GATEWAY/ipfs/QmCID
    let ipfs_re = Regex::new(r"https?://[^/]+/ipfs/(Qm[a-zA-Z0-9]{44,})").unwrap();

    // filename -> (cid, vec<page_ref>)
    let mut file_map: HashMap<String, (Option<String>, Vec<PageRef>)> = HashMap::new();

    for page in store.public_pages(&config.content) {
        let page_ref = PageRef {
            title: page.meta.title.clone(),
            url: format!("/{}", page.id),
        };

        // Find local asset references
        for cap in local_re.captures_iter(&page.content_md) {
            let filename = cap[1].to_string();
            let entry = file_map.entry(filename.clone()).or_insert_with(|| {
                let cid = cid_cache.get(&filename).cloned();
                (cid, Vec::new())
            });
            // Avoid duplicate page refs
            if !entry.1.iter().any(|r| r.url == page_ref.url) {
                entry.1.push(page_ref.clone());
            }
        }

        // Find already-rewritten IPFS URLs (after CI rewrite)
        for cap in ipfs_re.captures_iter(&page.content_md) {
            let cid = cap[1].to_string();
            // Try to find filename from cache reverse lookup
            let filename = cid_cache
                .iter()
                .find(|(_, v)| **v == cid)
                .map(|(k, _)| k.clone())
                .unwrap_or_else(|| cid.clone());

            let entry = file_map
                .entry(filename)
                .or_insert_with(|| (Some(cid.clone()), Vec::new()));
            if entry.0.is_none() {
                entry.0 = Some(cid);
            }
            if !entry.1.iter().any(|r| r.url == page_ref.url) {
                entry.1.push(page_ref.clone());
            }
        }
    }

    let gateway = "https://gateway.pinata.cloud";

    let mut entries: Vec<FileEntry> = file_map
        .into_iter()
        .map(|(filename, (cid, pages))| {
            let ipfs_url = cid.as_ref().map(|c| format!("{}/ipfs/{}", gateway, c));
            FileEntry {
                file_type: classify_file_type(&filename).to_string(),
                filename,
                ipfs_cid: cid,
                ipfs_url,
                referencing_pages: pages,
            }
        })
        .collect();

    entries.sort_by(|a, b| a.filename.cmp(&b.filename));
    entries
}

/// Write files-index.json to output directory
pub fn write_files_index(entries: &[FileEntry], output_dir: &Path) -> Result<()> {
    let json = serde_json::to_string(entries)?;
    std::fs::write(output_dir.join("files-index.json"), json)?;
    Ok(())
}
