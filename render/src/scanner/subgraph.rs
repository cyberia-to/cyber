use crate::parser::{PageId, ParsedPage};
use crate::scanner::{DiscoveredFile, FileKind};
use anyhow::{bail, Result};
use globset::{Glob, GlobSetBuilder};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Declaration of an external repository to include as a subgraph.
#[derive(Debug, Clone)]
pub struct SubgraphDecl {
    pub name: String,
    pub repo_path: PathBuf,
    pub exclude_patterns: Vec<String>,
    pub declaring_page_id: PageId,
}

/// Default exclude patterns applied to all subgraphs.
const DEFAULT_EXCLUDES: &[&str] = &[
    ".git/**",
    "target/**",
    "node_modules/**",
    "build/**",
    ".claude/**",
    "**/.DS_Store",
    "Cargo.lock",
];

/// Discover subgraph declarations from parsed root graph pages.
/// Looks for pages with `subgraph: true` in frontmatter properties.
pub fn discover_subgraphs(pages: &[ParsedPage], input_dir: &Path) -> Vec<SubgraphDecl> {
    let mut decls = Vec::new();

    for page in pages {
        let props = &page.meta.properties;

        // Check for subgraph: true
        let is_subgraph = props
            .get("subgraph")
            .map(|v| v.trim().eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        if !is_subgraph {
            continue;
        }

        // Parse repo path (required)
        let repo_raw = match props.get("repo") {
            Some(v) => v.trim().to_string(),
            None => {
                eprintln!(
                    "Warning: subgraph page '{}' has subgraph: true but no repo: path",
                    page.id
                );
                continue;
            }
        };

        // Resolve repo path relative to input_dir
        let repo_path = input_dir.join(&repo_raw);
        let repo_path = repo_path
            .canonicalize()
            .unwrap_or_else(|_| repo_path.clone());

        // Parse exclude patterns (optional, comma-separated)
        let custom_excludes: Vec<String> = props
            .get("exclude")
            .map(|v| {
                v.split(',')
                    .map(|s| s.trim().trim_matches('"').to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default();

        // Merge default + custom excludes
        let mut exclude_patterns: Vec<String> =
            DEFAULT_EXCLUDES.iter().map(|s| s.to_string()).collect();
        exclude_patterns.extend(custom_excludes);

        // Derive subgraph name from the declaring page's filename
        // e.g., page id "trident" → name "trident"
        // For namespaced pages like "cyber/foo" → use just the last component
        let name = page
            .meta
            .title
            .rsplit('/')
            .next()
            .unwrap_or(&page.meta.title)
            .to_string();

        decls.push(SubgraphDecl {
            name,
            repo_path,
            exclude_patterns,
            declaring_page_id: page.id.clone(),
        });
    }

    decls
}

/// Scan an external repository and return discovered files under the subgraph namespace.
/// All files are collected; markdown files become Pages, everything else becomes Files.
pub fn scan_subgraph(decl: &SubgraphDecl) -> Result<Vec<DiscoveredFile>> {
    if !decl.repo_path.exists() {
        bail!(
            "Subgraph '{}' repo path does not exist: {}",
            decl.name,
            decl.repo_path.display()
        );
    }

    // Build exclude glob set
    let mut builder = GlobSetBuilder::new();
    for pattern in &decl.exclude_patterns {
        if let Ok(glob) = Glob::new(pattern) {
            builder.add(glob);
        }
    }
    let exclude_set = builder.build()?;

    let mut files = Vec::new();

    for entry in WalkDir::new(&decl.repo_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path().to_path_buf();

        // Get path relative to repo root for exclusion matching
        let relative = path
            .strip_prefix(&decl.repo_path)
            .unwrap_or(&path)
            .to_string_lossy();

        if exclude_set.is_match(relative.as_ref()) {
            continue;
        }

        let is_md = path
            .extension()
            .map(|ext| ext == "md" || ext == "markdown")
            .unwrap_or(false);

        if is_md {
            let name = subgraph_page_name(&path, &decl.repo_path, &decl.name);
            files.push(DiscoveredFile {
                path,
                kind: FileKind::Page,
                name,
                subgraph: Some(decl.name.clone()),
            });
        } else {
            let name = subgraph_file_name(&path, &decl.repo_path, &decl.name);
            files.push(DiscoveredFile {
                path,
                kind: FileKind::File,
                name,
                subgraph: Some(decl.name.clone()),
            });
        }
    }

    Ok(files)
}

/// Derive page name for a markdown file in a subgraph.
/// Repo-root README.md maps to just the subgraph name (becomes the root page).
/// e.g., ~/git/trident/README.md → "trident"
/// e.g., ~/git/trident/docs/explanation/vision.md → "trident/docs/explanation/vision"
/// e.g., ~/git/trident/src/README.md → "trident/src/README"
fn subgraph_page_name(path: &Path, repo_root: &Path, subgraph_name: &str) -> String {
    let relative = path.strip_prefix(repo_root).unwrap_or(path);
    let stem = relative.with_extension("");
    let name = stem.to_string_lossy();

    // Repo-root README becomes the subgraph root page
    if name.eq_ignore_ascii_case("README") {
        return subgraph_name.to_string();
    }

    format!("{}/{}", subgraph_name, name)
}

/// Derive file name for a non-markdown file in a subgraph (preserves extension).
/// e.g., ~/git/trident/src/main.rs → "trident/src/main.rs"
fn subgraph_file_name(path: &Path, repo_root: &Path, subgraph_name: &str) -> String {
    let relative = path.strip_prefix(repo_root).unwrap_or(path);
    let name = relative.to_string_lossy().to_string();
    format!("{}/{}", subgraph_name, name)
}

/// Enforce namespace monopoly: remove root pages whose namespace conflicts
/// with a claimed subgraph namespace.
/// Returns list of (evicted_page_id, reason) for reporting.
pub fn enforce_namespace_monopoly(
    root_pages: &mut Vec<ParsedPage>,
    subgraph_namespaces: &[String],
) -> Vec<(PageId, String)> {
    let mut evicted = Vec::new();

    root_pages.retain(|page| {
        if let Some(ref ns) = page.namespace {
            for sg_ns in subgraph_namespaces {
                if ns == sg_ns || ns.starts_with(&format!("{}/", sg_ns)) {
                    evicted.push((
                        page.id.clone(),
                        format!(
                            "namespace '{}' claimed by subgraph '{}'",
                            ns, sg_ns
                        ),
                    ));
                    return false;
                }
            }
        }
        true
    });

    evicted
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_subgraph_page_name() {
        let repo = PathBuf::from("/git/trident");
        // Repo-root README maps to just the subgraph name
        assert_eq!(
            subgraph_page_name(&PathBuf::from("/git/trident/README.md"), &repo, "trident"),
            "trident"
        );
        // Nested files keep full path
        assert_eq!(
            subgraph_page_name(
                &PathBuf::from("/git/trident/docs/explanation/vision.md"),
                &repo,
                "trident"
            ),
            "trident/docs/explanation/vision"
        );
        // Nested README keeps "README" in name
        assert_eq!(
            subgraph_page_name(
                &PathBuf::from("/git/trident/src/README.md"),
                &repo,
                "trident"
            ),
            "trident/src/README"
        );
    }

    #[test]
    fn test_subgraph_file_name() {
        let repo = PathBuf::from("/git/trident");
        assert_eq!(
            subgraph_file_name(&PathBuf::from("/git/trident/src/main.rs"), &repo, "trident"),
            "trident/src/main.rs"
        );
        assert_eq!(
            subgraph_file_name(&PathBuf::from("/git/trident/Cargo.toml"), &repo, "trident"),
            "trident/Cargo.toml"
        );
    }

    #[test]
    fn test_namespace_monopoly_evicts_matching() {
        use crate::parser::{PageKind, PageMeta};
        use std::collections::HashMap;

        let make = |id: &str, ns: Option<&str>| ParsedPage {
            id: id.to_string(),
            meta: PageMeta {
                title: id.to_string(),
                properties: HashMap::new(),
                tags: vec![],
                public: Some(true),
                aliases: vec![],
                date: None,
                icon: None,
                menu_order: None,
                stake: None,
            },
            kind: PageKind::Page,
            source_path: PathBuf::new(),
            namespace: ns.map(|s| s.to_string()),
            subgraph: None,
            content_md: String::new(),
            outgoing_links: vec![],
        };

        let mut pages = vec![
            make("root-page", None),
            make("trident-thesis", None), // root level, no namespace — should NOT be evicted
            make("trident-sub-thing", Some("trident")), // namespace = trident — EVICTED
            make("other-ns-page", Some("cyber")),
        ];

        let evicted = enforce_namespace_monopoly(&mut pages, &["trident".to_string()]);

        assert_eq!(pages.len(), 3);
        assert_eq!(evicted.len(), 1);
        assert_eq!(evicted[0].0, "trident-sub-thing");
        // root-level pages with no namespace stay
        assert!(pages.iter().any(|p| p.id == "trident-thesis"));
    }
}
