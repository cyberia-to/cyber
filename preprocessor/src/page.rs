use anyhow::Result;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use crate::config::Config;
use crate::content;
use crate::frontmatter;

/// Represents a page in the index
#[derive(Debug, Clone)]
pub struct Page {
    pub name: String,
    pub name_lower: String,
    pub content: String,
    pub properties: HashMap<String, String>,
    pub tags: Vec<String>,
    pub namespace: Option<String>,
    pub modified: Option<String>,
    pub created: Option<String>,
}

/// Page index for query execution
pub type PageIndex = Vec<Page>;

/// Build index of all pages for query execution
pub fn build_index(pages_dir: &Path) -> Result<PageIndex> {
    let mut index = Vec::new();

    // Get all git dates in one batch call
    let repo_root = pages_dir.parent().unwrap_or(pages_dir);
    let git_dates = get_all_git_dates(repo_root);

    for entry in walkdir::WalkDir::new(pages_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
    {
        if let Ok(page) = parse_page_for_index(entry.path(), &git_dates, repo_root) {
            index.push(page);
        }
    }

    Ok(index)
}

/// Parse a page file for indexing (properties, tags, content)
fn parse_page_for_index(
    path: &Path,
    git_dates: &HashMap<String, (String, String)>,
    repo_root: &Path,
) -> Result<Page> {
    let content = fs::read_to_string(path)?;
    let filename = path.file_stem().unwrap().to_string_lossy().to_string();

    // Handle namespace (filename with ___)
    let (name, namespace) = if filename.contains("___") {
        let parts: Vec<&str> = filename.splitn(2, "___").collect();
        (parts.join("/"), Some(parts[0].to_string()))
    } else {
        (filename.clone(), None)
    };

    let (properties, _remaining) = parse_properties(&content);
    let tags = extract_tags(&properties, &content);

    // Get git dates from batch lookup
    let relative_path = path.strip_prefix(repo_root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let (modified, created) = git_dates
        .get(&relative_path)
        .map(|(m, c)| (Some(m.clone()), Some(c.clone())))
        .unwrap_or((None, None));

    Ok(Page {
        name: name.clone(),
        name_lower: name.to_lowercase(),
        content,
        properties,
        tags,
        namespace,
        modified,
        created,
    })
}

/// Parse Logseq properties from content
pub fn parse_properties(content: &str) -> (HashMap<String, String>, String) {
    lazy_static::lazy_static! {
        static ref PROP_RE: Regex = Regex::new(r"^-?\s*([a-zA-Z_-]+)::\s*(.+)$").unwrap();
    }

    let mut properties = HashMap::new();
    let mut end_index = 0;
    let lines: Vec<&str> = content.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        let clean_line = line.trim_start_matches('-').trim();

        if let Some(caps) = PROP_RE.captures(clean_line) {
            let key = caps.get(1).unwrap().as_str().to_lowercase();
            let value = caps.get(2).unwrap().as_str().trim().to_string();
            properties.insert(key, value);
            end_index = i + 1;
        } else if clean_line.is_empty() && !properties.is_empty() {
            end_index = i + 1;
        } else if !properties.is_empty() {
            break;
        } else if !clean_line.is_empty() && !clean_line.starts_with('-') {
            break;
        }
    }

    let remaining = lines[end_index..].join("\n");
    (properties, remaining)
}

/// Extract tags from properties and content
fn extract_tags(properties: &HashMap<String, String>, content: &str) -> Vec<String> {
    let mut tags = Vec::new();

    // From properties
    if let Some(tags_str) = properties.get("tags") {
        for tag in tags_str.split(',') {
            let tag = tag.trim()
                .trim_start_matches("[[")
                .trim_end_matches("]]")
                .to_lowercase();
            if !tag.is_empty() {
                tags.push(tag);
            }
        }
    }

    // From content #tags
    lazy_static::lazy_static! {
        static ref TAG_RE: Regex = Regex::new(r"#([a-zA-Z][a-zA-Z0-9_-]*)").unwrap();
    }

    for caps in TAG_RE.captures_iter(content) {
        let tag = caps.get(1).unwrap().as_str().to_lowercase();
        if !tags.contains(&tag) {
            tags.push(tag);
        }
    }

    tags
}

/// Get all git dates in batch (much faster than per-file)
pub fn get_all_git_dates(repo_root: &Path) -> HashMap<String, (String, String)> {
    use std::process::Command;

    let mut dates: HashMap<String, (String, String)> = HashMap::new();

    // Get last modified date for all files
    if let Ok(output) = Command::new("git")
        .args(["log", "--format=%aI", "--name-only", "--diff-filter=AM"])
        .current_dir(repo_root)
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut current_date = String::new();

        for line in stdout.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            // Date lines start with a year
            if line.starts_with("20") && line.contains('T') {
                current_date = line.split('T').next().unwrap_or("").to_string();
            } else if line.ends_with(".md") && !current_date.is_empty() {
                let entry = dates.entry(line.to_string()).or_insert_with(|| {
                    (current_date.clone(), current_date.clone())
                });
                // First time we see the file = most recent (modified)
                // Last time = oldest (created)
                entry.1 = current_date.clone(); // Update created to older date
            }
        }
    }

    dates
}

/// Process a single page file
pub fn process_page(
    source_path: &Path,
    output_dir: &Path,
    page_index: &PageIndex,
    config: &Config,
    git_dates: &HashMap<String, (String, String)>,
    repo_root: &Path,
) -> Result<bool> {
    let content = fs::read_to_string(source_path)?;
    let filename = source_path.file_stem().unwrap().to_string_lossy();

    // Parse properties
    let (properties, remaining_content) = parse_properties(&content);

    // Skip private pages
    if !config.include_private {
        if let Some(private) = properties.get("private") {
            if private.to_lowercase() == "true" {
                return Ok(false);
            }
        }
    }

    // Convert namespace separator
    let output_filename = filename.replace("___", "/");
    let output_path = output_dir.join(format!("{}.md", output_filename));

    // Create parent directories if needed
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Get git dates for this file
    let relative_path = source_path.strip_prefix(repo_root)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let dates = git_dates.get(&relative_path)
        .map(|(m, c)| (m.as_str(), c.as_str()));

    // Generate frontmatter
    let frontmatter = frontmatter::generate(&filename, &properties, dates);

    // Transform content
    let transformed = content::transform(&remaining_content, page_index);

    // Write output
    let output = format!("{}\n{}", frontmatter, transformed);
    fs::write(output_path, output)?;

    Ok(true)
}

/// Create stub pages for missing linked pages
pub fn create_stubs(output_dir: &Path, page_index: &PageIndex) -> Result<usize> {
    let pages_dir = output_dir.join("pages");

    // Collect all existing files
    let existing: HashSet<String> = walkdir::WalkDir::new(&pages_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        .filter_map(|e| {
            e.path()
                .strip_prefix(&pages_dir)
                .ok()
                .map(|p| p.to_string_lossy().to_lowercase())
        })
        .collect();

    // Collect all wikilinks from output files
    let mut all_links = HashSet::new();
    for entry in walkdir::WalkDir::new(output_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
    {
        if let Ok(content) = fs::read_to_string(entry.path()) {
            extract_wikilinks(&content, &mut all_links);
        }
    }

    // Create stubs for missing pages
    let mut created = 0;
    for link in all_links {
        // Strip pages/ prefix if present
        let link = link.strip_prefix("pages/").unwrap_or(&link);

        // Skip special folders
        if link.starts_with("journals/") || link.starts_with("favorites/") || link.starts_with("assets/") {
            continue;
        }

        let file_path = format!("{}.md", link.to_lowercase());
        if existing.contains(&file_path) {
            continue;
        }

        // Skip non-page links
        if link.starts_with("http") || link.starts_with('#') || link.contains("://") {
            continue;
        }
        if link.len() <= 1 || link.len() > 200 {
            continue;
        }

        // Sanitize link for filesystem
        let safe_link = link.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");

        let stub_path = pages_dir.join(format!("{}.md", safe_link));
        if stub_path.exists() {
            continue;
        }

        // Create stub
        if let Some(parent) = stub_path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("Failed to create dir for stub '{}': {}", link, e);
                continue;
            }
        }

        let title = link.replace('_', " ");
        let stub_content = format!(
            "---\ntitle: \"{}\"\nstub: true\n---\n\n> [!note] Stub Page\n> This page was auto-generated.\n",
            title
        );

        match fs::write(&stub_path, &stub_content) {
            Ok(_) => created += 1,
            Err(e) => eprintln!("Failed to write stub '{}': {}", stub_path.display(), e),
        }
    }

    Ok(created)
}

/// Extract wikilinks from content
fn extract_wikilinks(content: &str, links: &mut HashSet<String>) {
    lazy_static::lazy_static! {
        static ref LINK_RE: Regex = Regex::new(r"\[\[([^\]|]+)(?:\|[^\]]+)?\]\]").unwrap();
    }

    for caps in LINK_RE.captures_iter(content) {
        let link = caps.get(1).unwrap().as_str().trim();
        if !link.starts_with("http") && !link.starts_with('#') && !link.starts_with('!') {
            links.insert(link.to_lowercase());
        }
    }
}
