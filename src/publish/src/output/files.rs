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

/// Check if a filename is "opaque" — has no meaningful human-readable name.
/// Raw CIDs, generic image_TIMESTAMP, telegram blobs, bare dates.
fn is_opaque_filename(filename: &str) -> bool {
    let name = filename.split('.').next().unwrap_or(filename);
    // Raw IPFS CID
    if name.starts_with("Qm") && name.len() > 40 {
        return true;
    }
    // Generic image_TIMESTAMP_0
    let ts_re = Regex::new(r"^image_\d{13}_\d$").unwrap();
    if ts_re.is_match(name) {
        return true;
    }
    // Telegram blob filenames
    if name.starts_with("telegram-cloud-") {
        return true;
    }
    // Bare date filenames like 2025-10-28_15.04.35_TIMESTAMP_0 or 2025-10-23_13.26.04_...
    let date_re = Regex::new(r"^\d{4}[-_]\d{2}[-_]\d{2}").unwrap();
    if date_re.is_match(name) {
        return true;
    }
    // Screenshot filenames like Screenshot_2024-05-12_at_14.30.39_...
    if name.starts_with("Screenshot") {
        return true;
    }
    // AI-generated image prompts (joyrocket._ or similar UUID-containing names)
    if name.starts_with("joyrocket") {
        return true;
    }
    // Long filenames with UUIDs embedded
    let uuid_re = Regex::new(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}").unwrap();
    if uuid_re.is_match(name) {
        return true;
    }
    // Regex artifacts (backtick, brackets, truncated filenames)
    // Check full filename for ][ since it may appear after dots
    if name.starts_with('`') || name.ends_with('`') || filename.contains("][") || name.len() < 2 {
        return true;
    }
    false
}

/// Generate a human-readable display name from a filename.
/// Strips Logseq timestamps, parenthetical numbers, replaces separators with spaces.
fn humanize_filename(filename: &str) -> String {
    // Remove extension (handle double extensions like .drawio.svg)
    let name = {
        let mut n = filename;
        // Strip known extensions from the end
        for ext in &[
            ".drawio.svg",
            ".svg",
            ".png",
            ".jpg",
            ".jpeg",
            ".gif",
            ".webp",
            ".pdf",
            ".mp4",
            ".mov",
            ".webm",
            ".avi",
            ".mp3",
            ".wav",
            ".ogg",
            ".flac",
            ".skp",
            ".json",
        ] {
            if let Some(stripped) = n.strip_suffix(ext) {
                n = stripped;
                break;
            }
        }
        n
    };

    // Strip Logseq timestamp suffix: _TIMESTAMP_0 (13-digit unix ms + _0)
    let stripped = Regex::new(r"_\d{13}_\d$")
        .unwrap()
        .replace(name, "")
        .to_string();

    // After timestamp removal, an embedded extension may remain (e.g. Biogas_plant.svg)
    // Strip it if present
    let stripped = {
        let mut s = stripped.as_str();
        for ext in &[
            ".drawio.svg",
            ".svg",
            ".png",
            ".jpg",
            ".jpeg",
            ".gif",
            ".webp",
            ".pdf",
            ".mp4",
            ".mov",
            ".webm",
            ".skp",
        ] {
            if let Some(inner) = s.strip_suffix(ext) {
                s = inner;
                break;
            }
        }
        s.to_string()
    };

    // Remove parenthetical numbers like (4), (5) — also handles truncated parens
    let no_parens = Regex::new(r"\s*\(?\d+\)?\s*$")
        .unwrap()
        .replace(&stripped, "")
        .to_string();
    // Also strip mid-string parenthetical like "qr-code_(4)"
    let no_parens = Regex::new(r"[_\s]*\(\d+\)")
        .unwrap()
        .replace_all(&no_parens, "")
        .to_string();

    // Replace underscores and hyphens with spaces
    let humanized = no_parens.replace('_', " ").replace('-', " ");

    // Collapse whitespace
    let collapsed: String = humanized.split_whitespace().collect::<Vec<_>>().join(" ");

    if collapsed.is_empty() {
        filename.to_string()
    } else {
        collapsed
    }
}

/// Assign display names to file entries.
/// Opaque filenames (CIDs, timestamps, telegram blobs) get named after their
/// referencing page. When multiple opaque files share a page, they get numbered.
fn assign_display_names(entries: &mut [FileEntry]) {
    // First pass: count opaque files per page to know when numbering is needed
    let mut page_opaque_count: HashMap<String, usize> = HashMap::new();
    for entry in entries.iter() {
        if is_opaque_filename(&entry.filename) {
            if let Some(page) = entry.referencing_pages.first() {
                *page_opaque_count.entry(page.title.clone()).or_insert(0) += 1;
            }
        }
    }

    // Second pass: track per-page index for numbering
    let mut page_index: HashMap<String, usize> = HashMap::new();

    for entry in entries.iter_mut() {
        // If display_name from alt-text is meaningful, keep it even for opaque filenames.
        // A display name is "meaningful" if it doesn't look like a raw filename/hash itself.
        if !entry.display_name.is_empty() {
            let dn = &entry.display_name;
            // Check if the display name itself looks opaque/meaningless
            let looks_opaque = is_opaque_filename(dn)
                || dn.contains("][")
                || dn.ends_with(".svg")
                || dn.ends_with(".skp")
                // Space-separated dates from humanized filenames
                || Regex::new(r"^\d{4}\s+\d{2}\s+\d{2}").unwrap().is_match(dn);
            if !looks_opaque {
                continue;
            }
        }

        if is_opaque_filename(&entry.filename) {
            // Name after the referencing page
            if let Some(page) = entry.referencing_pages.first() {
                let total = page_opaque_count.get(&page.title).copied().unwrap_or(1);
                if total > 1 {
                    let idx = page_index.entry(page.title.clone()).or_insert(0);
                    *idx += 1;
                    entry.display_name = format!("{} {}", page.title, idx);
                } else {
                    entry.display_name = page.title.clone();
                }
            } else {
                // No referencing page — use abbreviated CID
                entry.display_name =
                    format!("{}…", &entry.filename[..12.min(entry.filename.len())]);
            }
        } else {
            // Has a real filename — humanize it
            entry.display_name = humanize_filename(&entry.filename);
        }
    }
}

/// Build file index by scanning all public pages for media references.
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

    // Regex for markdown image/link with alt text and local media path
    // Captures: ![alt text](../media/filename) or [alt text](../media/filename)
    // Also matches legacy ../assets/ references for backwards compatibility
    let alt_re = Regex::new(r"!?\[([^\]]*)\]\(\.\./(?:media|assets)/([^)\s]+)\)").unwrap();
    // Regex for local media references without alt text context
    let local_re = Regex::new(r#"\.\./(?:media|assets)/([^)\s"']+)"#).unwrap();
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
            // Initial display name: use meaningful alt-text if available, empty otherwise
            let display_name = alt_text
                .filter(|a| {
                    // Reject generic/useless alt texts
                    *a != "image.png" && *a != "image" && !a.is_empty()
                })
                .map(|a| {
                    // Strip file extension from alt text
                    let stripped = Regex::new(r"\.(png|jpg|jpeg|gif|webp|svg|pdf|mp4|mov)$")
                        .unwrap()
                        .replace(&a, "")
                        .to_string();
                    // Remove parenthetical numbers like (4)
                    let cleaned = Regex::new(r"\s*\(\d+\)\s*")
                        .unwrap()
                        .replace_all(&stripped, "")
                        .to_string();
                    // Replace hyphens with spaces
                    let cleaned = cleaned.replace('-', " ");
                    let cleaned = cleaned.trim().to_string();
                    if cleaned.is_empty() {
                        a
                    } else {
                        cleaned
                    }
                })
                .unwrap_or_default();
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

    // Sort by number of referencing pages (most referenced first), then by filename
    entries.sort_by(|a, b| {
        b.referencing_pages
            .len()
            .cmp(&a.referencing_pages.len())
            .then_with(|| a.filename.cmp(&b.filename))
    });

    // Assign display names (handles opaque filenames, numbering, humanization)
    assign_display_names(&mut entries);

    entries
}

/// Write files-index.json to output directory
pub fn write_files_index(entries: &[FileEntry], output_dir: &Path) -> Result<()> {
    let json = serde_json::to_string(entries)?;
    std::fs::write(output_dir.join("files-index.json"), json)?;
    Ok(())
}
