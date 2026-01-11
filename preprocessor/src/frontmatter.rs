use std::collections::HashMap;

/// Generate YAML frontmatter from Logseq properties
pub fn generate(
    filename: &str,
    properties: &HashMap<String, String>,
    git_dates: Option<(&str, &str)>,
) -> String {
    let mut fm = String::from("---\n");

    // Title
    let title = if let Some(icon) = properties.get("icon") {
        format!("{} {}", icon, properties.get("title").map_or(filename.replace('_', " "), |t| t.clone()))
    } else {
        properties.get("title").map_or(filename.replace('_', " "), |t| t.clone())
    };
    fm.push_str(&format!("title: \"{}\"\n", escape_yaml(&title)));

    // Icon (separate field)
    if let Some(icon) = properties.get("icon") {
        fm.push_str(&format!("icon: \"{}\"\n", escape_yaml(icon)));
    }

    // Tags
    if let Some(tags) = properties.get("tags") {
        let tags: Vec<&str> = tags
            .split(',')
            .map(|t| t.trim().trim_start_matches("[[").trim_end_matches("]]"))
            .filter(|t| !t.is_empty())
            .collect();
        if !tags.is_empty() {
            fm.push_str("tags:\n");
            for tag in tags {
                fm.push_str(&format!("  - {}\n", tag));
            }
        }
    }

    // Aliases
    if let Some(alias) = properties.get("alias") {
        let aliases = parse_aliases(alias);
        if !aliases.is_empty() {
            fm.push_str("aliases:\n");
            for a in aliases {
                fm.push_str(&format!("  - {}\n", a));
            }
        }
    }

    // Description
    if let Some(desc) = properties.get("description") {
        fm.push_str(&format!("description: \"{}\"\n", escape_yaml(desc)));
    }

    // Git dates (from batch lookup)
    if let Some((modified, created)) = git_dates {
        fm.push_str(&format!("modified: {}\n", modified));
        fm.push_str(&format!("created: {}\n", created));
    }

    fm.push_str("---\n");
    fm
}

/// Parse aliases, handling wikilinks and comma separation
fn parse_aliases(alias_str: &str) -> Vec<String> {
    let mut aliases = Vec::new();
    let mut current = String::new();
    let mut in_wikilink = false;
    let chars: Vec<char> = alias_str.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();

        if c == '[' && next == Some('[') {
            in_wikilink = true;
            i += 2;
            continue;
        }

        if c == ']' && next == Some(']') {
            in_wikilink = false;
            i += 2;
            continue;
        }

        if c == ',' && !in_wikilink {
            let trimmed = current.trim().to_string();
            if !trimmed.is_empty() {
                aliases.push(trimmed);
            }
            current.clear();
            i += 1;
            continue;
        }

        current.push(c);
        i += 1;
    }

    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        aliases.push(trimmed);
    }

    aliases
}

/// Escape special characters for YAML strings
fn escape_yaml(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}
