use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::path::Path;

lazy_static! {
    // Match :favorites [...] in EDN
    static ref FAVORITES_RE: Regex = Regex::new(r":favorites\s+\[([\s\S]*?)\]").unwrap();

    // Match individual items in favorites list
    static ref FAV_ITEM_RE: Regex = Regex::new(r#""([^"]+)""#).unwrap();
}

/// Process favorites from logseq/config.edn
pub fn process_favorites(
    config_path: &Path,
    favorites_output: &Path,
    pages_output: &Path,
) -> Result<usize> {
    let content = fs::read_to_string(config_path)?;

    // Extract favorites list
    let favorites = extract_favorites(&content);
    if favorites.is_empty() {
        return Ok(0);
    }

    // Create favorites index
    let mut index_content = String::from("---\ntitle: \"⭐ Favorites\"\n---\n\n");

    let mut count = 0;
    for fav in &favorites {
        let slug = slugify(fav);

        // Check if page exists
        let page_path = pages_output.join(format!("{}.md", slug));
        if !page_path.exists() {
            continue;
        }

        // Get icon from page if exists
        let icon = get_page_icon(&page_path).unwrap_or_default();

        // Create favorite embed file
        let fav_path = favorites_output.join(format!("{}.md", slug));
        let fav_content = format!(
            "---\ntitle: \"{}{}\"\n---\n\n![[pages/{}]]\n",
            if icon.is_empty() { String::new() } else { format!("{} ", icon) },
            fav,
            slug
        );

        fs::write(&fav_path, fav_content)?;
        count += 1;

        // Add to index
        index_content.push_str(&format!(
            "- [[favorites/{}|{}{}]]\n",
            slug,
            if icon.is_empty() { String::new() } else { format!("{} ", icon) },
            fav
        ));
    }

    // Write index
    fs::write(favorites_output.join("index.md"), index_content)?;

    Ok(count)
}

/// Extract favorites from config.edn content
fn extract_favorites(content: &str) -> Vec<String> {
    let mut favorites = Vec::new();

    if let Some(caps) = FAVORITES_RE.captures(content) {
        let list = caps.get(1).unwrap().as_str();

        for item in FAV_ITEM_RE.captures_iter(list) {
            let fav = item.get(1).unwrap().as_str().to_string();
            favorites.push(fav);
        }
    }

    favorites
}

/// Convert page name to slug
fn slugify(name: &str) -> String {
    name.to_lowercase()
        .replace(' ', "-")
        .replace('/', "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-' || *c == '_')
        .collect()
}

/// Get icon from page frontmatter or properties
fn get_page_icon(page_path: &Path) -> Option<String> {
    let content = fs::read_to_string(page_path).ok()?;

    // Check for icon:: property
    for line in content.lines().take(20) {
        let line = line.trim().trim_start_matches('-').trim();
        if let Some(rest) = line.strip_prefix("icon::") {
            return Some(rest.trim().to_string());
        }
        if let Some(rest) = line.strip_prefix("icon:") {
            return Some(rest.trim().trim_matches('"').to_string());
        }
    }

    None
}
