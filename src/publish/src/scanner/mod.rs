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
    Media,
    /// Non-markdown file (code, config, binary, etc.) treated as a graph node
    File,
}

#[derive(Debug)]
pub struct DiscoveredFiles {
    pub pages: Vec<DiscoveredFile>,
    pub journals: Vec<DiscoveredFile>,
    pub media: Vec<DiscoveredFile>,
    pub files: Vec<DiscoveredFile>,
}

fn is_markdown(path: &Path) -> bool {
    path.extension()
        .map(|ext| ext == "md" || ext == "markdown")
        .unwrap_or(false)
        || path.extension().is_none()
}

pub fn scan(input_dir: &Path, content_config: &ContentSection) -> Result<DiscoveredFiles> {
    let input_dir = input_dir
        .canonicalize()
        .unwrap_or_else(|_| input_dir.to_path_buf());
    let pages_dir = input_dir.join("pages");
    let journals_dir = input_dir.join("journals");
    let media_dir = input_dir.join("media");

    let mut result = DiscoveredFiles {
        pages: Vec::new(),
        journals: Vec::new(),
        media: Vec::new(),
        files: Vec::new(),
    };

    // Scan pages directory — markdown files become Pages, everything else becomes Files
    if pages_dir.exists() {
        for entry in WalkDir::new(&pages_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path().to_path_buf();
            if classify::is_excluded(&path, &input_dir, &content_config.exclude_patterns) {
                continue;
            }
            if is_markdown(&path) {
                let name = classify::page_name_from_path(&path, &pages_dir);
                result.pages.push(DiscoveredFile {
                    path,
                    kind: FileKind::Page,
                    name,
                });
            } else {
                let name = classify::file_name_from_path(&path, &pages_dir);
                result.files.push(DiscoveredFile {
                    path,
                    kind: FileKind::File,
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

    // Scan media — still copied to output, but also registered as graph nodes
    if media_dir.exists() {
        for entry in WalkDir::new(&media_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path().to_path_buf();
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            result.media.push(DiscoveredFile {
                path: path.clone(),
                kind: FileKind::Media,
                name: name.clone(),
            });
            // Also add as a File node for the graph
            result.files.push(DiscoveredFile {
                path,
                kind: FileKind::File,
                name: format!("media/{}", name),
            });
        }
    }

    // Scan all other files in the repo (outside pages/, journals/, media/)
    for entry in WalkDir::new(&input_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path().to_path_buf();

        // Skip files already handled by the dedicated directory scans
        if path.starts_with(&pages_dir)
            || path.starts_with(&journals_dir)
            || path.starts_with(&media_dir)
        {
            continue;
        }

        if classify::is_excluded(&path, &input_dir, &content_config.exclude_patterns) {
            continue;
        }

        let name = classify::file_name_from_path(&path, &input_dir);
        result.files.push(DiscoveredFile {
            path,
            kind: FileKind::File,
            name,
        });
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
    fn test_scan_discovers_media() {
        let tmp = TempDir::new().unwrap();
        let pages_dir = tmp.path().join("pages");
        let media_dir = tmp.path().join("media");
        fs::create_dir_all(&pages_dir).unwrap();
        fs::create_dir_all(&media_dir).unwrap();
        fs::write(media_dir.join("image.png"), b"PNG").unwrap();

        let content = ContentSection::default();
        let result = scan(tmp.path(), &content).unwrap();
        assert_eq!(result.media.len(), 1);
        // Media files also appear as graph nodes
        assert!(result.files.iter().any(|f| f.name == "media/image.png"));
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
        // logseq/ is excluded, so config.edn should not appear in files
        assert!(!result.files.iter().any(|f| f.name.contains("config.edn")));
    }

    #[test]
    fn test_scan_discovers_non_md_files() {
        let tmp = TempDir::new().unwrap();
        let pages_dir = tmp.path().join("pages");
        let nu_dir = tmp.path().join("nu");
        fs::create_dir_all(&pages_dir).unwrap();
        fs::create_dir_all(&nu_dir).unwrap();
        fs::write(pages_dir.join("Page.md"), "# hello").unwrap();
        fs::write(pages_dir.join("data.zip"), b"PK").unwrap();
        fs::write(nu_dir.join("script.nu"), "echo hello").unwrap();
        fs::write(tmp.path().join("Makefile"), "all:").unwrap();

        let content = ContentSection::default();
        let result = scan(tmp.path(), &content).unwrap();
        assert_eq!(result.pages.len(), 1); // only Page.md
        // data.zip in pages/, script.nu in nu/, Makefile at root
        assert!(result.files.iter().any(|f| f.name == "data.zip"));
        assert!(result.files.iter().any(|f| f.name == "nu/script.nu"));
        assert!(result.files.iter().any(|f| f.name == "Makefile"));
    }
}
