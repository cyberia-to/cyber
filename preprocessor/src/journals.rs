use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::path::Path;

use crate::config::Config;
use crate::content;
use crate::page::{parse_properties, PageIndex};

lazy_static! {
    // Journal date patterns
    static ref DATE_UNDERSCORE_RE: Regex = Regex::new(r"^(\d{4})_(\d{2})_(\d{2})$").unwrap();
    static ref DATE_DASH_RE: Regex = Regex::new(r"^(\d{4})-(\d{2})-(\d{2})$").unwrap();
}

/// Process all journal files
pub fn process_journals(
    journals_dir: &Path,
    output_dir: &Path,
    page_index: &PageIndex,
    config: &Config,
) -> Result<usize> {
    let mut count = 0;
    let mut entries = Vec::new();

    for entry in fs::read_dir(journals_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "md") {
            if let Some(filename) = path.file_stem() {
                let filename = filename.to_string_lossy();

                if let Some((date, title)) = parse_journal_date(&filename) {
                    match process_journal_file(&path, output_dir, &date, &title, page_index, config) {
                        Ok(true) => {
                            entries.push((date.clone(), title.clone(), filename.to_string()));
                            count += 1;
                        }
                        Ok(false) => {}
                        Err(e) => {
                            if config.verbose {
                                eprintln!("Error processing journal {:?}: {}", path, e);
                            }
                        }
                    }
                }
            }
        }
    }

    // Create journal index
    if !entries.is_empty() {
        create_journal_index(output_dir, &entries)?;
    }

    Ok(count)
}

/// Parse journal filename to date and title
fn parse_journal_date(filename: &str) -> Option<(String, String)> {
    let months = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December",
    ];

    // Try underscore format: 2024_08_16
    if let Some(caps) = DATE_UNDERSCORE_RE.captures(filename) {
        let year = caps.get(1)?.as_str();
        let month: usize = caps.get(2)?.as_str().parse().ok()?;
        let day: usize = caps.get(3)?.as_str().parse().ok()?;

        if month >= 1 && month <= 12 && day >= 1 && day <= 31 {
            let date = format!("{}-{:02}-{:02}", year, month, day);
            let title = format!("{} {}, {}", months[month - 1], day, year);
            return Some((date, title));
        }
    }

    // Try dash format: 2024-08-16
    if let Some(caps) = DATE_DASH_RE.captures(filename) {
        let year = caps.get(1)?.as_str();
        let month: usize = caps.get(2)?.as_str().parse().ok()?;
        let day: usize = caps.get(3)?.as_str().parse().ok()?;

        if month >= 1 && month <= 12 && day >= 1 && day <= 31 {
            let date = format!("{}-{:02}-{:02}", year, month, day);
            let title = format!("{} {}, {}", months[month - 1], day, year);
            return Some((date, title));
        }
    }

    None
}

/// Process a single journal file
fn process_journal_file(
    source_path: &Path,
    output_dir: &Path,
    date: &str,
    title: &str,
    page_index: &PageIndex,
    config: &Config,
) -> Result<bool> {
    let content = fs::read_to_string(source_path)?;
    let (properties, remaining) = parse_properties(&content);

    // Skip private journals
    if !config.include_private {
        if let Some(private) = properties.get("private") {
            if private.to_lowercase() == "true" {
                return Ok(false);
            }
        }
    }

    // Generate frontmatter
    let mut frontmatter = format!(
        "---\ntitle: \"{}\"\ndate: {}\n",
        title.replace('"', "\\\""),
        date
    );

    // Add tags if present
    if let Some(tags) = properties.get("tags") {
        frontmatter.push_str("tags:\n");
        for tag in tags.split(',') {
            let tag = tag
                .trim()
                .trim_start_matches("[[")
                .trim_end_matches("]]");
            if !tag.is_empty() {
                frontmatter.push_str(&format!("  - {}\n", tag));
            }
        }
    }

    frontmatter.push_str("---\n");

    // Transform content
    let transformed = content::transform(&remaining, page_index);

    // Write output
    let output_path = output_dir.join(format!("{}.md", date));
    let output = format!("{}\n{}", frontmatter, transformed);
    fs::write(output_path, output)?;

    Ok(true)
}

/// Create journal index page
fn create_journal_index(output_dir: &Path, entries: &[(String, String, String)]) -> Result<()> {
    let mut sorted = entries.to_vec();
    sorted.sort_by(|a, b| b.0.cmp(&a.0)); // Sort by date descending

    let mut content = String::from("---\ntitle: \"📅 Journals\"\n---\n\n");

    for (date, title, _) in sorted {
        content.push_str(&format!("## [[journals/{}|{} - {}]]\n\n", date, date, title));
    }

    fs::write(output_dir.join("index.md"), content)?;

    Ok(())
}
