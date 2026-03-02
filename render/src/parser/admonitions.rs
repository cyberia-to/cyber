/// Transform admonition blocks into HTML divs.
/// Logseq uses org-mode-style #+BEGIN_X...#+END_X blocks.
pub fn transform_admonitions(content: &str) -> String {
    if !content.contains("#+BEGIN_") {
        return content.to_string();
    }

    let mut result = String::with_capacity(content.len());
    let mut lines = content.lines().peekable();
    let mut first_line = true;

    while let Some(line) = lines.next() {
        if !first_line && result.chars().last() != Some('\n') {
            result.push('\n');
        }
        first_line = false;

        let trimmed = line.trim();
        if let Some(block_type) = trimmed
            .to_uppercase()
            .strip_prefix("#+BEGIN_")
            .map(|s| s.to_string())
        {
            if block_type.chars().all(|c| c.is_alphanumeric() || c == '_') && !block_type.is_empty()
            {
                let end_marker = format!("#+END_{}", block_type);
                let mut body_lines = Vec::new();

                // Collect lines until matching #+END_TYPE
                let mut found_end = false;
                for next_line in lines.by_ref() {
                    if next_line.trim().eq_ignore_ascii_case(&end_marker) {
                        found_end = true;
                        break;
                    }
                    body_lines.push(next_line);
                }

                if found_end {
                    let body = body_lines.join("\n");
                    let body = body.trim();
                    result.push_str(&render_block(&block_type, body));
                } else {
                    // No matching end — output the BEGIN line and body as-is
                    result.push_str(line);
                    for bl in body_lines {
                        result.push('\n');
                        result.push_str(bl);
                    }
                }
                continue;
            }
        }

        result.push_str(line);
    }

    // Preserve trailing newline if original had one
    if content.ends_with('\n') && !result.ends_with('\n') {
        result.push('\n');
    }

    result
}

fn render_block(block_type: &str, body: &str) -> String {
    let css_class = block_type.to_lowercase();

    let icon = match block_type {
        "NOTE" => "&#x1f4dd;",
        "TIP" => "&#x1f4a1;",
        "WARNING" => "&#x26a0;&#xfe0f;",
        "CAUTION" => "&#x1f6d1;",
        "IMPORTANT" => "&#x2757;",
        "QUOTE" => "&#x201c;",
        "EXAMPLE" => "&#x1f4cb;",
        _ => "",
    };

    let label = match block_type {
        "QUOTE" | "EXAMPLE" | "SRC" => String::new(),
        _ => format!(
            "<div class=\"admonition-title\">{} {}</div>",
            icon, block_type
        ),
    };

    if block_type == "QUOTE" {
        format!("> {}", body.replace('\n', "\n> "))
    } else {
        format!(
            "<div class=\"admonition admonition-{}\">{}\n\n{}\n\n</div>",
            css_class, label, body
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_admonition() {
        let input = "Before\n#+BEGIN_NOTE\nThis is a note.\n#+END_NOTE\nAfter";
        let result = transform_admonitions(input);
        assert!(result.contains("admonition-note"));
        assert!(result.contains("This is a note."));
        assert!(result.contains("Before"));
        assert!(result.contains("After"));
    }

    #[test]
    fn test_quote_becomes_blockquote() {
        let input = "#+BEGIN_QUOTE\nWise words.\n#+END_QUOTE";
        let result = transform_admonitions(input);
        assert!(result.contains("> Wise words."));
    }

    #[test]
    fn test_no_admonitions() {
        let input = "Just regular content";
        let result = transform_admonitions(input);
        assert_eq!(result, input);
    }

    #[test]
    fn test_case_insensitive_end() {
        let input = "#+BEGIN_NOTE\nBody text.\n#+end_note";
        let result = transform_admonitions(input);
        assert!(result.contains("admonition-note"));
        assert!(result.contains("Body text."));
    }

    #[test]
    fn test_unmatched_begin() {
        let input = "#+BEGIN_NOTE\nNo end marker here";
        let result = transform_admonitions(input);
        assert!(result.contains("#+BEGIN_NOTE"));
        assert!(result.contains("No end marker here"));
    }
}
