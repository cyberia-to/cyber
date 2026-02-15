pub mod admonitions;
pub mod outliner;
pub mod properties;
pub mod wikilinks;

use crate::scanner::{DiscoveredFile, DiscoveredFiles, FileKind};
use anyhow::Result;
use chrono::NaiveDate;
use serde::Serialize;
use std::collections::HashMap;
use std::path::PathBuf;

/// Unique identifier for a page (normalized/slugified from page name)
pub type PageId = String;

#[derive(Debug, Clone, Serialize)]
pub struct PageMeta {
    pub title: String,
    pub properties: HashMap<String, String>,
    pub tags: Vec<String>,
    pub public: Option<bool>,
    pub aliases: Vec<String>,
    pub date: Option<NaiveDate>,
    pub icon: Option<String>,
    pub menu_order: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum PageKind {
    Page,
    Journal,
}

#[derive(Debug, Clone)]
pub struct ParsedPage {
    pub id: PageId,
    pub meta: PageMeta,
    pub kind: PageKind,
    pub source_path: PathBuf,
    pub namespace: Option<String>,
    /// Normalized markdown content (after outliner transform, properties stripped)
    pub content_md: String,
    /// Wikilinks found during parsing (raw page names, not yet slugified)
    pub outgoing_links: Vec<String>,
}

pub fn slugify_page_name(name: &str) -> PageId {
    let lower = name.to_lowercase();
    let mut result = String::with_capacity(lower.len());
    let mut prev_hyphen = true; // prevents leading hyphen

    for ch in lower.chars() {
        if ch.is_alphanumeric() || ch == '$' || ch == '.' {
            result.push(ch);
            prev_hyphen = false;
        } else if !prev_hyphen {
            result.push('-');
            prev_hyphen = true;
        }
    }

    result.trim_end_matches('-').to_string()
}

pub fn parse_all(discovered: &DiscoveredFiles) -> Result<Vec<ParsedPage>> {
    let mut pages = Vec::new();

    for file in &discovered.pages {
        let page = parse_file(file)?;
        pages.push(page);
    }

    for file in &discovered.journals {
        let page = parse_file(file)?;
        pages.push(page);
    }

    Ok(pages)
}

pub fn parse_file(file: &DiscoveredFile) -> Result<ParsedPage> {
    let content = std::fs::read_to_string(&file.path)?;

    // Step 1: Extract properties (YAML frontmatter or legacy property:: lines)
    let (meta, content_after_props) = properties::extract_properties(&content, &file.name);

    // Step 2: Normalize outliner bullets only if content looks like outliner format
    let normalized = if looks_like_outliner(&content_after_props) {
        outliner::normalize(&content_after_props)
    } else {
        content_after_props
    };

    // Step 2b: Transform admonition blocks
    let normalized = admonitions::transform_admonitions(&normalized);

    // Step 3: Collect wikilinks from the normalized content
    let outgoing_links = wikilinks::collect_wikilinks(&normalized);

    // Determine namespace
    let namespace = if file.name.contains('/') {
        let parts: Vec<&str> = file.name.rsplitn(2, '/').collect();
        if parts.len() == 2 {
            Some(parts[1].to_string())
        } else {
            None
        }
    } else {
        None
    };

    // Determine kind
    let kind = match file.kind {
        FileKind::Journal => PageKind::Journal,
        _ => PageKind::Page,
    };

    let id = slugify_page_name(&file.name);

    Ok(ParsedPage {
        id,
        meta,
        kind,
        source_path: file.path.clone(),
        namespace,
        content_md: normalized,
        outgoing_links,
    })
}

/// Detect if content is in Logseq outliner format (majority of lines are bullets).
fn looks_like_outliner(content: &str) -> bool {
    let non_empty: Vec<&str> = content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .take(20)
        .collect();

    if non_empty.is_empty() {
        return false;
    }

    let bullet_count = non_empty
        .iter()
        .filter(|l| {
            let trimmed = l.trim_start();
            trimmed.starts_with("- ")
        })
        .count();

    (bullet_count as f64 / non_empty.len() as f64) > 0.5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(
            slugify_page_name("Collective Focus Theorem"),
            "collective-focus-theorem"
        );
        assert_eq!(
            slugify_page_name("projects/Cyber Valley"),
            "projects-cyber-valley"
        );
        assert_eq!(slugify_page_name("2025-02-08"), "2025-02-08");
        assert_eq!(slugify_page_name("$BOOT"), "$boot");
        assert_eq!(slugify_page_name("$PUSSY on $SOL"), "$pussy-on-$sol");
        assert_eq!(slugify_page_name(".moon names"), ".moon-names");
    }
}
