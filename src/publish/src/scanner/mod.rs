mod classify;

use crate::config::ContentSection;
use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct DiscoveredFile {
    pub path: PathBuf,
    pub kind: FileKind,
    /// Page name derived from filename (e.g., "Collective Focus Theorem")
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileKind {
    Page,
    Journal,
    Asset,
}

#[derive(Debug)]
pub struct DiscoveredFiles {
    pub pages: Vec<DiscoveredFile>,
    pub journals: Vec<DiscoveredFile>,
    pub assets: Vec<DiscoveredFile>,
}

pub fn scan(input_dir: &Path, content_config: &ContentSection) -> Result<DiscoveredFiles> {
    let input_dir = input_dir
        .canonicalize()
        .unwrap_or_else(|_| input_dir.to_path_buf());
    let pages_dir = input_dir.join("pages");
    let journals_dir = input_dir.join("journals");
    let assets_dir = input_dir.join("assets");

    let mut result = DiscoveredFiles {
        pages: Vec::new(),
        journals: Vec::new(),
        assets: Vec::new(),
    };

    // Scan pages
    if pages_dir.exists() {
        for entry in WalkDir::new(&pages_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path().to_path_buf();
            let is_md = path
                .extension()
                .map(|ext| ext == "md" || ext == "markdown")
                .unwrap_or(false);
            // Also accept files with no extension (Logseq sometimes creates these)
            let no_ext = path.extension().is_none();
            if is_md || no_ext {
                if classify::is_excluded(&path, &input_dir, &content_config.exclude_patterns) {
                    continue;
                }
                let name = classify::page_name_from_path(&path, &pages_dir);
                result.pages.push(DiscoveredFile {
                    path,
                    kind: FileKind::Page,
                    name,
                });
            }
        }
    }

    // Scan journals
    if content_config.include_journals && journals_dir.exists() {
        for entry in WalkDir::new(&journals_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path().to_path_buf();
            if let Some(ext) = path.extension() {
                if ext == "md" || ext == "markdown" {
                    let name = classify::journal_name_from_path(&path);
                    result.journals.push(DiscoveredFile {
                        path,
                        kind: FileKind::Journal,
                        name,
                    });
                }
            }
        }
    }

    // Scan assets
    if assets_dir.exists() {
        for entry in WalkDir::new(&assets_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path().to_path_buf();
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            result.assets.push(DiscoveredFile {
                path,
                kind: FileKind::Asset,
                name,
            });
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_scan_discovers_pages() {
        let tmp = TempDir::new().unwrap();
        let pages_dir = tmp.path().join("pages");
        fs::create_dir_all(&pages_dir).unwrap();
        fs::write(pages_dir.join("Test Page.md"), "- hello").unwrap();
        fs::write(pages_dir.join("Another.md"), "- world").unwrap();

        let content = ContentSection::default();
        let result = scan(tmp.path(), &content).unwrap();
        assert_eq!(result.pages.len(), 2);
    }

    #[test]
    fn test_scan_discovers_assets() {
        let tmp = TempDir::new().unwrap();
        let pages_dir = tmp.path().join("pages");
        let assets_dir = tmp.path().join("assets");
        fs::create_dir_all(&pages_dir).unwrap();
        fs::create_dir_all(&assets_dir).unwrap();
        fs::write(assets_dir.join("image.png"), b"PNG").unwrap();

        let content = ContentSection::default();
        let result = scan(tmp.path(), &content).unwrap();
        assert_eq!(result.assets.len(), 1);
    }

    #[test]
    fn test_scan_respects_exclude_patterns() {
        let tmp = TempDir::new().unwrap();
        let pages_dir = tmp.path().join("pages");
        let logseq_dir = tmp.path().join("logseq");
        fs::create_dir_all(&pages_dir).unwrap();
        fs::create_dir_all(&logseq_dir).unwrap();
        fs::write(pages_dir.join("Good.md"), "- hello").unwrap();
        fs::write(logseq_dir.join("config.edn"), "{}").unwrap();

        let content = ContentSection::default();
        let result = scan(tmp.path(), &content).unwrap();
        assert_eq!(result.pages.len(), 1);
        assert_eq!(result.pages[0].name, "Good");
    }
}
