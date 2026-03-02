use crate::parser::PageMeta;
use chrono::NaiveDate;
use regex::Regex;
use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref PROPERTY_RE: Regex = Regex::new(r"^([a-zA-Z_-]+)::\s*(.*)$").unwrap();
}

/// Extract properties from a page file.
/// Supports two formats:
/// 1. YAML frontmatter (--- delimited) — primary format
/// 2. Logseq property:: value lines — legacy fallback
///
/// Returns the PageMeta and the remaining content with properties stripped.
pub fn extract_properties(content: &str, page_name: &str) -> (PageMeta, String) {
    if content.starts_with("---\n") || content.starts_with("---\r\n") {
        extract_yaml_frontmatter(content, page_name)
    } else {
        extract_logseq_properties(content, page_name)
    }
}

/// Parse YAML frontmatter delimited by --- markers.
fn extract_yaml_frontmatter(content: &str, page_name: &str) -> (PageMeta, String) {
    // Find the closing ---
    let after_first = if content.starts_with("---\r\n") {
        &content[5..]
    } else {
        &content[4..] // "---\n"
    };

    let end_pos = if let Some(pos) = after_first.find("\n---\n") {
        pos
    } else if let Some(pos) = after_first.find("\n---\r\n") {
        pos
    } else if after_first.ends_with("\n---") {
        after_first.len() - 3
    } else {
        // No closing ---, treat entire file as content
        return build_meta(HashMap::new(), content.to_string(), page_name);
    };

    let yaml_str = &after_first[..end_pos];
    let remaining_start = end_pos + 5; // skip "\n---\n"
    let remaining = if remaining_start <= after_first.len() {
        after_first[remaining_start..].to_string()
    } else {
        String::new()
    };

    // Parse YAML into a mapping
    let properties = match serde_yaml::from_str::<serde_yaml::Value>(yaml_str) {
        Ok(serde_yaml::Value::Mapping(map)) => {
            let mut props = HashMap::new();
            for (key, value) in map {
                if let serde_yaml::Value::String(k) = key {
                    let v = yaml_value_to_string(&value);
                    // Strip trailing colons so Logseq "alias::" syntax
                    // in YAML frontmatter normalizes to "alias"
                    let normalized = k.trim_end_matches(':').to_lowercase();
                    props.insert(normalized, v);
                }
            }
            props
        }
        _ => HashMap::new(),
    };

    build_meta(properties, remaining, page_name)
}

/// Convert a serde_yaml::Value to a string, preserving the original format.
fn yaml_value_to_string(value: &serde_yaml::Value) -> String {
    match value {
        serde_yaml::Value::String(s) => s.clone(),
        serde_yaml::Value::Bool(b) => b.to_string(),
        serde_yaml::Value::Number(n) => n.to_string(),
        serde_yaml::Value::Sequence(seq) => {
            // Convert [a, b, c] to "a, b, c"
            seq.iter()
                .map(yaml_value_to_string)
                .collect::<Vec<_>>()
                .join(", ")
        }
        serde_yaml::Value::Null => String::new(),
        _ => format!("{:?}", value),
    }
}

/// Legacy Logseq property:: value parser (fallback).
fn extract_logseq_properties(content: &str, page_name: &str) -> (PageMeta, String) {
    let mut properties = HashMap::new();
    let mut remaining_lines = Vec::new();
    let mut in_properties = true;
    let mut found_any_property = false;

    for line in content.lines() {
        if in_properties {
            let check_line = line.strip_prefix("- ").unwrap_or(line);
            let trimmed = check_line.trim();

            if trimmed.is_empty() {
                if found_any_property {
                    in_properties = false;
                }
                continue;
            }

            if let Some(caps) = PROPERTY_RE.captures(trimmed) {
                let key = caps[1].to_lowercase();
                let value = caps[2].trim().to_string();
                properties.insert(key, value);
                found_any_property = true;
                continue;
            }

            in_properties = false;
            remaining_lines.push(line.to_string());
        } else {
            remaining_lines.push(line.to_string());
        }
    }

    let remaining = remaining_lines.join("\n");
    build_meta(properties, remaining, page_name)
}

/// Build PageMeta from a property map and remaining content.
fn build_meta(
    properties: HashMap<String, String>,
    remaining: String,
    page_name: &str,
) -> (PageMeta, String) {
    let title = properties
        .get("title")
        .cloned()
        .unwrap_or_else(|| page_name.to_string());

    let tags = properties
        .get("tags")
        .map(|v| {
            v.split(',')
                .map(|s| {
                    s.trim()
                        .trim_start_matches("[[")
                        .trim_end_matches("]]")
                        .to_string()
                })
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();

    let public = properties
        .get("public")
        .map(|v| v.eq_ignore_ascii_case("true") || v == "1" || v.eq_ignore_ascii_case("yes"));

    let aliases = properties
        .get("alias")
        .or_else(|| properties.get("aliases"))
        .map(|v| {
            v.split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();

    let date = properties
        .get("date")
        .and_then(|v| NaiveDate::parse_from_str(v.trim(), "%Y-%m-%d").ok())
        .or_else(|| NaiveDate::parse_from_str(page_name.trim(), "%Y-%m-%d").ok());

    let icon = properties.get("icon").cloned();

    let menu_order = properties
        .get("menu-order")
        .and_then(|v| v.trim().parse::<i32>().ok());

    let stake = properties
        .get("stake")
        .and_then(|v| v.trim().parse::<u64>().ok());

    let meta = PageMeta {
        title,
        properties,
        tags,
        public,
        aliases,
        date,
        icon,
        menu_order,
        stake,
    };

    (meta, remaining)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yaml_frontmatter_basic() {
        let content =
            "---\ntitle: My Page\ntags: rust, programming\npublic: \"true\"\n---\nHello world";
        let (meta, remaining) = extract_properties(content, "My Page");
        assert_eq!(meta.title, "My Page");
        assert_eq!(meta.tags, vec!["rust", "programming"]);
        assert_eq!(meta.public, Some(true));
        assert!(remaining.contains("Hello world"));
    }

    #[test]
    fn test_yaml_frontmatter_with_icon() {
        let content = "---\nicon: \"\\U0001F535\"\ntags: cyber, menu\ncrystal-type: entity\n---\ncontent here";
        let (meta, remaining) = extract_properties(content, "test");
        assert_eq!(meta.tags, vec!["cyber", "menu"]);
        assert_eq!(meta.properties.get("crystal-type").unwrap(), "entity");
        assert!(remaining.contains("content here"));
    }

    #[test]
    fn test_yaml_menu_order_quoted() {
        let content = "---\nmenu-order: \"2\"\ntags: cyber\n---\n";
        let (meta, _) = extract_properties(content, "test");
        assert_eq!(meta.menu_order, Some(2));
    }

    #[test]
    fn test_yaml_menu_order_number() {
        let content = "---\nmenu-order: 2\ntags: cyber\n---\n";
        let (meta, _) = extract_properties(content, "test");
        assert_eq!(meta.menu_order, Some(2));
    }

    #[test]
    fn test_fallback_logseq_properties() {
        let content = "title:: My Page\ntags:: rust, programming\npublic:: true\n\n- Hello world";
        let (meta, remaining) = extract_properties(content, "My Page");
        assert_eq!(meta.title, "My Page");
        assert_eq!(meta.tags, vec!["rust", "programming"]);
        assert_eq!(meta.public, Some(true));
        assert!(remaining.contains("Hello world"));
    }

    #[test]
    fn test_fallback_with_bullet_properties() {
        let content = "- title:: Bullet Props\n- tags:: test\n\n- Content here";
        let (meta, remaining) = extract_properties(content, "Test");
        assert_eq!(meta.title, "Bullet Props");
        assert_eq!(meta.tags, vec!["test"]);
        assert!(remaining.contains("Content here"));
    }

    #[test]
    fn test_no_properties() {
        let content = "Just content\nMore content";
        let (meta, remaining) = extract_properties(content, "Default Name");
        assert_eq!(meta.title, "Default Name");
        assert!(meta.tags.is_empty());
        assert!(remaining.contains("Just content"));
    }

    #[test]
    fn test_tags_with_wikilinks() {
        let content = "tags:: [[research]], [[math]]";
        let (meta, _) = extract_properties(content, "Test");
        assert_eq!(meta.tags, vec!["research", "math"]);
    }

    #[test]
    fn test_aliases() {
        let content = "---\nalias: CFT, theorem\n---\n";
        let (meta, _) = extract_properties(content, "Test");
        assert_eq!(meta.aliases, vec!["CFT", "theorem"]);
    }

    #[test]
    fn test_aliases_double_colon_in_yaml() {
        // Logseq "alias::" syntax inside YAML frontmatter
        // YAML parses key as "alias:" — must strip trailing colon
        let content = "---\nalias:: Shapley, Shapley values\n---\n";
        let (meta, _) = extract_properties(content, "Test");
        assert_eq!(meta.aliases, vec!["Shapley", "Shapley values"]);
    }

    #[test]
    fn test_date_parsing() {
        let (meta, _) = extract_properties("", "2025-02-08");
        assert!(meta.date.is_some());
        assert_eq!(
            meta.date.unwrap(),
            NaiveDate::from_ymd_opt(2025, 2, 8).unwrap()
        );
    }

    #[test]
    fn test_yaml_empty_frontmatter() {
        let content = "---\n---\nContent only";
        let (meta, remaining) = extract_properties(content, "test");
        assert_eq!(meta.title, "test");
        assert!(remaining.contains("Content only"));
    }

    #[test]
    fn test_yaml_boolean_as_string() {
        // YAML normally parses "true" as boolean, but we want string
        let content = "---\nscalable: \"true\"\n---\n";
        let (meta, _) = extract_properties(content, "test");
        assert_eq!(meta.properties.get("scalable").unwrap(), "true");
    }
}
