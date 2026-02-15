use std::path::Path;

/// Extract page name from file path relative to pages directory.
/// e.g., "pages/Collective Focus Theorem.md" → "Collective Focus Theorem"
/// e.g., "pages/projects/Cyber Valley.md" → "projects/Cyber Valley" (namespace)
/// e.g., "pages/%2Emoon names.md" → ".moon names" (percent-decoded)
pub fn page_name_from_path(path: &Path, pages_dir: &Path) -> String {
    let relative = path
        .strip_prefix(pages_dir)
        .unwrap_or(path);

    let name = relative
        .with_extension("")
        .to_string_lossy()
        .to_string();

    // Logseq uses ∕ (DIVISION SLASH U+2215) or ___  for namespace separators in filenames
    // Normalize to /
    let name = name.replace('∕', "/").replace("___", "/");

    // Logseq percent-encodes special characters in filenames (e.g., %2E → . , %3A → : , %3F → ?)
    percent_decode(&name)
}

/// Decode percent-encoded characters in a string (e.g., %2E → ., %3F → ?).
fn percent_decode(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hi = bytes[i + 1];
            let lo = bytes[i + 2];
            if let (Some(h), Some(l)) = (hex_val(hi), hex_val(lo)) {
                result.push((h << 4 | l) as char);
                i += 3;
                continue;
            }
        }
        result.push(bytes[i] as char);
        i += 1;
    }
    result
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Extract journal name from file path.
/// e.g., "journals/2025_02_08.md" → "2025-02-08"
pub fn journal_name_from_path(path: &Path) -> String {
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    // Logseq journal filenames: 2025_02_08.md → 2025-02-08
    stem.replace('_', "-")
}

/// Check if a path matches any exclusion pattern.
pub fn is_excluded(path: &Path, base_dir: &Path, patterns: &[String]) -> bool {
    let relative = path
        .strip_prefix(base_dir)
        .unwrap_or(path)
        .to_string_lossy();

    for pattern in patterns {
        if matches_glob(pattern, &relative) {
            return true;
        }
    }
    false
}

/// Simple glob matching: supports * and ** patterns.
fn matches_glob(pattern: &str, path: &str) -> bool {
    let pattern = pattern.replace('\\', "/");
    let path = path.replace('\\', "/");

    if pattern.ends_with("/*") {
        let prefix = &pattern[..pattern.len() - 2];
        return path.starts_with(prefix);
    }

    if pattern.contains("**") {
        let parts: Vec<&str> = pattern.split("**").collect();
        if parts.len() == 2 {
            let prefix = parts[0];
            let suffix = parts[1].trim_start_matches('/');
            if !prefix.is_empty() && !path.starts_with(prefix) {
                return false;
            }
            if !suffix.is_empty() && !path.ends_with(suffix) {
                return false;
            }
            return true;
        }
    }

    // Simple wildcard
    if pattern.contains('*') {
        let parts: Vec<&str> = pattern.split('*').collect();
        if parts.len() == 2 {
            return path.starts_with(parts[0]) && path.ends_with(parts[1]);
        }
    }

    path == pattern
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_page_name_simple() {
        let pages_dir = PathBuf::from("/graph/pages");
        let path = PathBuf::from("/graph/pages/My Page.md");
        assert_eq!(page_name_from_path(&path, &pages_dir), "My Page");
    }

    #[test]
    fn test_page_name_namespace_unicode() {
        let pages_dir = PathBuf::from("/graph/pages");
        let path = PathBuf::from("/graph/pages/projects∕Cyber Valley.md");
        assert_eq!(
            page_name_from_path(&path, &pages_dir),
            "projects/Cyber Valley"
        );
    }

    #[test]
    fn test_page_name_percent_encoded() {
        let pages_dir = PathBuf::from("/graph/pages");
        let path = PathBuf::from("/graph/pages/%2Emoon names.md");
        assert_eq!(page_name_from_path(&path, &pages_dir), ".moon names");
    }

    #[test]
    fn test_page_name_percent_encoded_colon() {
        let pages_dir = PathBuf::from("/graph/pages");
        let path = PathBuf::from("/graph/pages/Offer%3A CEO of DEV.md");
        assert_eq!(
            page_name_from_path(&path, &pages_dir),
            "Offer: CEO of DEV"
        );
    }

    #[test]
    fn test_journal_name() {
        let path = PathBuf::from("/graph/journals/2025_02_08.md");
        assert_eq!(journal_name_from_path(&path), "2025-02-08");
    }

    #[test]
    fn test_exclusion() {
        let base = PathBuf::from("/graph");
        let patterns = vec!["logseq/*".to_string()];
        assert!(is_excluded(
            &PathBuf::from("/graph/logseq/config.edn"),
            &base,
            &patterns
        ));
        assert!(!is_excluded(
            &PathBuf::from("/graph/pages/test.md"),
            &base,
            &patterns
        ));
    }
}
