mod classify;
pub mod subgraph;

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
    /// Which subgraph this file belongs to (None = root graph)
    pub subgraph: Option<String>,
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

/// Resolve directory with fallback: try primary name first, then legacy name.
fn resolve_dir(input_dir: &Path, primary: &str, fallback: &str) -> PathBuf {
    let primary_dir = input_dir.join(primary);
    if primary_dir.exists() {
        return primary_dir;
    }
    let fallback_dir = input_dir.join(fallback);
    if fallback_dir.exists() {
        return fallback_dir;
    }
    // Neither exists — return primary (scan will skip gracefully)
    primary_dir
}

pub fn scan(input_dir: &Path, content_config: &ContentSection) -> Result<DiscoveredFiles> {
    let input_dir = input_dir
        .canonicalize()
        .unwrap_or_else(|_| input_dir.to_path_buf());
    let graph_dir = resolve_dir(&input_dir, "graph", "pages");
    let blog_dir = resolve_dir(&input_dir, "blog", "journals");
    let media_dir = input_dir.join("media");

    let mut result = DiscoveredFiles {
        pages: Vec::new(),
        journals: Vec::new(),
        media: Vec::new(),
        files: Vec::new(),
    };

    // Scan graph directory — markdown files become Pages, everything else becomes Files
    if graph_dir.exists() {
        for entry in WalkDir::new(&graph_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path().to_path_buf();
            if classify::is_excluded(&path, &input_dir, &content_config.exclude_patterns) {
                continue;
            }
            if is_markdown(&path) {
                let name = classify::page_name_from_path(&path, &graph_dir);
                result.pages.push(DiscoveredFile {
                    path,
                    kind: FileKind::Page,
                    name,
                    subgraph: None,
                });
            } else {
                let name = classify::file_name_from_path(&path, &graph_dir);
                result.files.push(DiscoveredFile {
                    path,
                    kind: FileKind::File,
                    name,
                    subgraph: None,
                });
            }
        }
    }

    // Scan blog entries
    if content_config.include_journals && blog_dir.exists() {
        for entry in WalkDir::new(&blog_dir)
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
                        subgraph: None,
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
                subgraph: None,
            });
            // Also add as a File node for the graph
            result.files.push(DiscoveredFile {
                path,
                kind: FileKind::File,
                name: format!("media/{}", name),
                subgraph: None,
            });
        }
    }

    // Directories to skip entirely — prevents WalkDir from descending into
    // build/ (thousands of generated HTML files), .git/, target/, etc.
    let skip_dirs: std::collections::HashSet<&str> =
        [".git", "target", "node_modules", "build", ".claude"].into();

    // Scan all other files in the repo (outside graph/, blog/, media/)
    for entry in WalkDir::new(&input_dir)
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy();
                !skip_dirs.contains(name.as_ref())
            } else {
                true
            }
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path().to_path_buf();

        // Skip files already handled by the dedicated directory scans
        if path.starts_with(&graph_dir)
            || path.starts_with(&blog_dir)
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
            subgraph: None,
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
    fn test_scan_discovers_pages_from_graph_dir() {
        let tmp = TempDir::new().unwrap();
        let graph_dir = tmp.path().join("graph");
        fs::create_dir_all(&graph_dir).unwrap();
        fs::write(graph_dir.join("Test Page.md"), "- hello").unwrap();
        fs::write(graph_dir.join("Another.md"), "- world").unwrap();

        let content = ContentSection::default();
        let result = scan(tmp.path(), &content).unwrap();
        assert_eq!(result.pages.len(), 2);
    }

    #[test]
    fn test_scan_fallback_to_pages_dir() {
        let tmp = TempDir::new().unwrap();
        // Use legacy "pages" directory name
        let pages_dir = tmp.path().join("pages");
        fs::create_dir_all(&pages_dir).unwrap();
        fs::write(pages_dir.join("Test.md"), "hello").unwrap();

        let content = ContentSection::default();
        let result = scan(tmp.path(), &content).unwrap();
        assert_eq!(result.pages.len(), 1);
    }

    #[test]
    fn test_scan_discovers_media() {
        let tmp = TempDir::new().unwrap();
        let graph_dir = tmp.path().join("graph");
        let media_dir = tmp.path().join("media");
        fs::create_dir_all(&graph_dir).unwrap();
        fs::create_dir_all(&media_dir).unwrap();
        fs::write(media_dir.join("image.png"), b"PNG").unwrap();

        let content = ContentSection::default();
        let result = scan(tmp.path(), &content).unwrap();
        assert_eq!(result.media.len(), 1);
        assert!(result.files.iter().any(|f| f.name == "media/image.png"));
    }

    #[test]
    fn test_scan_respects_exclude_patterns() {
        let tmp = TempDir::new().unwrap();
        let graph_dir = tmp.path().join("graph");
        let logseq_dir = tmp.path().join("logseq");
        fs::create_dir_all(&graph_dir).unwrap();
        fs::create_dir_all(&logseq_dir).unwrap();
        fs::write(graph_dir.join("Good.md"), "- hello").unwrap();
        fs::write(logseq_dir.join("config.edn"), "{}").unwrap();

        let content = ContentSection::default();
        let result = scan(tmp.path(), &content).unwrap();
        assert_eq!(result.pages.len(), 1);
        assert_eq!(result.pages[0].name, "Good");
        assert!(!result.files.iter().any(|f| f.name.contains("config.edn")));
    }

    #[test]
    fn test_scan_discovers_non_md_files() {
        let tmp = TempDir::new().unwrap();
        let graph_dir = tmp.path().join("graph");
        let stats_dir = tmp.path().join("stats");
        fs::create_dir_all(&graph_dir).unwrap();
        fs::create_dir_all(&stats_dir).unwrap();
        fs::write(graph_dir.join("Page.md"), "# hello").unwrap();
        fs::write(graph_dir.join("data.zip"), b"PK").unwrap();
        fs::write(stats_dir.join("script.nu"), "echo hello").unwrap();
        fs::write(tmp.path().join("Makefile"), "all:").unwrap();

        let content = ContentSection::default();
        let result = scan(tmp.path(), &content).unwrap();
        assert_eq!(result.pages.len(), 1);
        assert!(result.files.iter().any(|f| f.name == "data.zip"));
        assert!(result.files.iter().any(|f| f.name == "stats/script.nu"));
        assert!(result.files.iter().any(|f| f.name == "Makefile"));
    }
}
