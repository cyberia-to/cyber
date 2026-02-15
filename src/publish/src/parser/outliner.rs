use regex::Regex;

lazy_static::lazy_static! {
    static ref BULLET_RE: Regex = Regex::new(r"^(\s*)- (.*)$").unwrap();
    static ref HEADING_RE: Regex = Regex::new(r"^(#{1,6})\s+(.*)$").unwrap();
}

/// Normalize Logseq outliner markdown into standard markdown.
///
/// Rules:
/// 1. Top-level `- ` bullets with no sub-bullets → paragraphs (strip the `- `)
/// 2. Top-level `- ` bullets with sub-bullets → keep as list structure
/// 3. `- ## Heading` → promote to actual heading (strip `- `)
/// 4. Indentation depth tracked for block hierarchy
/// 5. Continuation lines (non-bullet lines at matching indent) merged with parent bullet
/// 6. Pipe tables auto-get GFM separator rows if missing
pub fn normalize(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return String::new();
    }

    let parsed: Vec<BulletLine> = lines.iter().map(|l| parse_bullet_line(l)).collect();

    let mut output = Vec::new();
    let mut i = 0;

    while i < parsed.len() {
        let line = &parsed[i];

        match line.kind {
            LineKind::Empty => {
                output.push(String::new());
                i += 1;
            }
            LineKind::NonBullet => {
                output.push(line.raw.to_string());
                i += 1;
            }
            LineKind::Bullet { indent_level } => {
                // Collect continuation lines for this bullet
                let (full_content, next_i) = collect_bullet_content(&parsed, i);
                i = next_i;

                if indent_level == 0 {
                    let content_text = &full_content;

                    // Check if it's a heading: `- ## Heading`
                    if HEADING_RE.is_match(content_text.lines().next().unwrap_or("")) {
                        output.push(String::new());
                        output.push(full_content);
                        output.push(String::new());
                        continue;
                    }

                    // Check if this starts a pipe table sequence
                    if looks_like_table_row(content_text) {
                        let mut table_lines: Vec<String> =
                            full_content.lines().map(String::from).collect();

                        // Collect subsequent top-level bullets that are also table rows
                        while i < parsed.len() {
                            if let LineKind::Bullet { indent_level: 0 } = parsed[i].kind {
                                let (next_content, peek_i) = collect_bullet_content(&parsed, i);
                                if looks_like_table_row(&next_content) {
                                    table_lines.extend(next_content.lines().map(String::from));
                                    i = peek_i;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }

                        let table = table_lines.join("\n");
                        output.push(String::new());
                        output.push(ensure_table_separator(&table));
                        output.push(String::new());
                        continue;
                    }

                    // Check if this top-level bullet has sub-bullets
                    let has_children = i < parsed.len()
                        && matches!(
                            parsed[i].kind,
                            LineKind::Bullet { indent_level } if indent_level > 0
                        );

                    if has_children {
                        // Keep as paragraph, then sub-bullets become a list
                        output.push(String::new());
                        output.push(full_content);
                        output.push(String::new());

                        // Collect all sub-bullets
                        while i < parsed.len() {
                            match &parsed[i].kind {
                                LineKind::Bullet { indent_level } if *indent_level > 0 => {
                                    let (sub_content, next_i) = collect_bullet_content(&parsed, i);
                                    i = next_i;
                                    let sub_indent = indent_level - 1;

                                    // Check if this sub-bullet is a table
                                    if is_table_block(&sub_content) {
                                        output.push(String::new());
                                        output.push(ensure_table_separator(&sub_content));
                                        output.push(String::new());
                                    } else {
                                        let prefix = "  ".repeat(sub_indent);
                                        emit_list_item(&mut output, &prefix, &sub_content);
                                    }
                                }
                                LineKind::Empty => {
                                    output.push(String::new());
                                    i += 1;
                                }
                                _ => break,
                            }
                        }
                    } else {
                        // Single top-level bullet → paragraph
                        output.push(String::new());
                        output.push(full_content);
                        output.push(String::new());
                    }
                } else {
                    // Sub-bullet without a parent context
                    let sub_indent = indent_level.saturating_sub(1);

                    if is_table_block(&full_content) {
                        output.push(String::new());
                        output.push(ensure_table_separator(&full_content));
                        output.push(String::new());
                    } else {
                        let prefix = "  ".repeat(sub_indent);
                        emit_list_item(&mut output, &prefix, &full_content);
                    }
                }
            }
        }
    }

    // Clean up: remove leading/trailing empty lines, collapse multiple blanks
    let result = output.join("\n");
    collapse_blank_lines(&result)
}

/// Emit a markdown list item, properly indenting continuation lines.
fn emit_list_item(output: &mut Vec<String>, prefix: &str, content: &str) {
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() <= 1 {
        output.push(format!("{}- {}", prefix, content));
    } else {
        output.push(format!("{}- {}", prefix, lines[0]));
        for l in &lines[1..] {
            output.push(format!("{}  {}", prefix, l));
        }
    }
}

/// Collect a bullet's content including continuation lines.
/// Continuation lines are non-bullet lines whose raw text starts with
/// the bullet's indent + 2 spaces (matching where "- " was).
/// Empty lines between continuations are bridged (skipped) so that
/// table rows separated by blank lines still merge correctly.
fn collect_bullet_content(parsed: &[BulletLine], start: usize) -> (String, usize) {
    let bullet = &parsed[start];
    let mut content = bullet.content.clone();
    let raw_indent = bullet.raw_indent;

    // Continuation prefix: same indent as bullet + 2 spaces (replacing "- ")
    let continuation_prefix = format!("{}  ", raw_indent);

    let mut i = start + 1;
    while i < parsed.len() {
        match &parsed[i].kind {
            LineKind::NonBullet => {
                let raw = parsed[i].raw;
                if raw.starts_with(&continuation_prefix) && !raw.trim().is_empty() {
                    let cont = &raw[continuation_prefix.len()..];
                    content.push('\n');
                    content.push_str(cont);
                    i += 1;
                } else {
                    break;
                }
            }
            LineKind::Empty => {
                // Peek past empty lines: if followed by a valid continuation, bridge the gap
                let mut peek = i + 1;
                while peek < parsed.len() && matches!(parsed[peek].kind, LineKind::Empty) {
                    peek += 1;
                }
                if peek < parsed.len() {
                    if let LineKind::NonBullet = &parsed[peek].kind {
                        if parsed[peek].raw.starts_with(&continuation_prefix)
                            && !parsed[peek].raw.trim().is_empty()
                        {
                            // Bridge: for table rows skip blank lines, otherwise preserve them
                            let cont_text = &parsed[peek].raw[continuation_prefix.len()..];
                            if !cont_text.trim_start().starts_with('|') {
                                content.push('\n');
                            }
                            i = peek;
                            continue;
                        }
                    }
                }
                break;
            }
            _ => break,
        }
    }

    (content, i)
}

/// Check if content looks like a table row (first line starts with |).
fn looks_like_table_row(content: &str) -> bool {
    content
        .lines()
        .next()
        .map(|l| l.trim_start().starts_with('|'))
        .unwrap_or(false)
}

/// Check if content is a multi-line table block (all lines start with |, >= 2 lines).
fn is_table_block(content: &str) -> bool {
    let lines: Vec<&str> = content.lines().collect();
    lines.len() >= 2 && lines.iter().all(|l| l.trim_start().starts_with('|'))
}

/// Ensure a pipe table has a GFM separator row after the header.
/// If the second line is NOT a separator (|---|---|), insert one.
fn ensure_table_separator(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    if lines.len() < 2 {
        return content.to_string();
    }

    // Check if second line is already a separator
    let second = lines[1].trim();
    if is_separator_row(second) {
        return content.to_string();
    }

    // Count columns from first line
    let header = lines[0].trim();
    let col_count = count_table_columns(header);
    if col_count == 0 {
        return content.to_string();
    }

    let separator = format!("|{}|", vec!["---"; col_count].join("|"));

    let mut result = Vec::new();
    result.push(lines[0].to_string());
    result.push(separator);
    for line in &lines[1..] {
        result.push(line.to_string());
    }
    result.join("\n")
}

/// Check if a line is a GFM table separator row (e.g., |---|---|).
fn is_separator_row(line: &str) -> bool {
    if !line.starts_with('|') {
        return false;
    }
    let inner = line.trim_start_matches('|').trim_end_matches('|');
    inner.split('|').all(|cell| {
        let trimmed = cell.trim();
        !trimmed.is_empty() && trimmed.chars().all(|c| c == '-' || c == ':' || c == ' ')
    })
}

/// Count the number of columns in a pipe table row.
fn count_table_columns(row: &str) -> usize {
    let trimmed = row.trim();
    if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
        // Count pipes and subtract 1 for leading pipe
        let pipes = trimmed.matches('|').count();
        if pipes >= 2 {
            return pipes - 1;
        }
        return 0;
    }
    let inner = trimmed.trim_start_matches('|').trim_end_matches('|');
    inner.split('|').count()
}

fn collapse_blank_lines(s: &str) -> String {
    let mut result = Vec::new();
    let mut prev_blank = false;

    for line in s.lines() {
        let is_blank = line.trim().is_empty();
        if is_blank {
            if !prev_blank {
                result.push("");
            }
            prev_blank = true;
        } else {
            result.push(line);
            prev_blank = false;
        }
    }

    // Trim leading and trailing blank lines
    let s = result.join("\n");
    s.trim().to_string()
}

#[derive(Debug)]
struct BulletLine<'a> {
    raw: &'a str,
    raw_indent: &'a str,
    kind: LineKind,
    content: String,
}

#[derive(Debug)]
enum LineKind {
    Empty,
    NonBullet,
    Bullet { indent_level: usize },
}

fn parse_bullet_line<'a>(line: &'a str) -> BulletLine<'a> {
    if line.trim().is_empty() {
        return BulletLine {
            raw: line,
            raw_indent: "",
            kind: LineKind::Empty,
            content: String::new(),
        };
    }

    if let Some(caps) = BULLET_RE.captures(line) {
        let indent = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        // Each indent level is 2 spaces or 1 tab
        let indent_level = if indent.contains('\t') {
            indent.matches('\t').count()
        } else {
            indent.len() / 2
        };
        let content = caps.get(2).map(|m| m.as_str()).unwrap_or("").to_string();

        BulletLine {
            raw: line,
            raw_indent: indent,
            kind: LineKind::Bullet { indent_level },
            content,
        }
    } else {
        BulletLine {
            raw: line,
            raw_indent: "",
            kind: LineKind::NonBullet,
            content: line.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_paragraphs() {
        let input = "- First paragraph.\n- Second paragraph.";
        let output = normalize(input);
        assert!(output.contains("First paragraph."));
        assert!(output.contains("Second paragraph."));
        assert!(!output.contains("- "));
        // Paragraphs must be separated by a blank line for comrak
        assert!(output.contains("First paragraph.\n\nSecond paragraph."));
    }

    #[test]
    fn test_heading_promotion() {
        let input = "- ## My Heading\n- Some content";
        let output = normalize(input);
        assert!(output.contains("## My Heading"));
        assert!(!output.starts_with("- ## "));
    }

    #[test]
    fn test_nested_bullets_become_list() {
        let input = "- Parent item\n  - Child one\n  - Child two";
        let output = normalize(input);
        assert!(output.contains("Parent item"));
        assert!(output.contains("- Child one"));
        assert!(output.contains("- Child two"));
    }

    #[test]
    fn test_deeply_nested() {
        let input = "- Top\n  - Level 1\n    - Level 2";
        let output = normalize(input);
        assert!(output.contains("- Level 1"));
        assert!(output.contains("  - Level 2"));
    }

    #[test]
    fn test_full_logseq_example() {
        let input = "\
- This is the introduction to the theorem.
- The core principle states that:
  - Consensus emergence follows predictable patterns
  - These patterns can be modeled mathematically
    - Using graph theory and information theory
- ## Applications
  - [[Bostrom]] network uses this for GPU consensus
  - Biological systems exhibit similar behavior";

        let output = normalize(input);
        assert!(output.contains("This is the introduction"));
        assert!(output.contains("## Applications"));
        assert!(output.contains("- [[Bostrom]]"));
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(normalize(""), "");
    }

    #[test]
    fn test_continuation_lines_table() {
        // Logseq table with continuation lines in a sub-bullet
        let input = "- Overview\n\t- | Header 1 | Header 2 |\n\t  | Cell A | Cell B |\n\t  | Cell C | Cell D |";
        let output = normalize(input);
        assert!(
            output.contains("<table>") || output.contains("|---"),
            "Table should have separator: {}",
            output
        );
        assert!(output.contains("| Header 1 | Header 2 |"));
        assert!(output.contains("| Cell A | Cell B |"));
        assert!(output.contains("| Cell C | Cell D |"));
        // Should NOT have "- |" as a list item
        assert!(
            !output.contains("- |"),
            "Table rows should not be list items: {}",
            output
        );
    }

    #[test]
    fn test_multi_bullet_table() {
        // Each row is a separate top-level bullet
        let input = "- | Name | Value |\n- | foo | 1 |\n- | bar | 2 |";
        let output = normalize(input);
        assert!(output.contains("| Name | Value |"));
        assert!(output.contains("|---|---|"));
        assert!(output.contains("| foo | 1 |"));
        assert!(output.contains("| bar | 2 |"));
    }

    #[test]
    fn test_table_with_existing_separator() {
        let input = "- | A | B |\n- |---|---|\n- | 1 | 2 |";
        let output = normalize(input);
        // Should not duplicate the separator
        let sep_count = output.matches("|---|---|").count();
        assert_eq!(
            sep_count, 1,
            "Should have exactly one separator: {}",
            output
        );
    }

    #[test]
    fn test_continuation_lines_non_table() {
        // Multi-line content in a bullet (like code or continuation text)
        let input = "- Start of block\n  continues here\n  and here";
        let output = normalize(input);
        assert!(output.contains("Start of block"));
        assert!(output.contains("continues here"));
        assert!(output.contains("and here"));
    }

    #[test]
    fn test_table_with_empty_lines_between_rows() {
        // Logseq sometimes has empty lines between table rows in continuation
        let input = "- Parent\n\t- | block height | neuron |\n\t  \n\t  | 42 | bostrom1d8 |\n\t  \n\t  | 43 | bostrom1d8 |";
        let output = normalize(input);
        assert!(
            output.contains("| block height | neuron |"),
            "Header missing: {}",
            output
        );
        assert!(
            output.contains("| 42 | bostrom1d8 |"),
            "Row 1 missing: {}",
            output
        );
        assert!(
            output.contains("| 43 | bostrom1d8 |"),
            "Row 2 missing: {}",
            output
        );
        assert!(
            output.contains("|---|---|"),
            "Separator missing: {}",
            output
        );
        assert!(
            !output.contains("- |"),
            "Table rows should not be list items: {}",
            output
        );
    }

    #[test]
    fn test_ensure_table_separator() {
        let table = "| A | B |\n| 1 | 2 |";
        let result = ensure_table_separator(table);
        assert!(result.contains("|---|---|"));

        let table_with_sep = "| A | B |\n|---|---|\n| 1 | 2 |";
        let result = ensure_table_separator(table_with_sep);
        assert_eq!(result.matches("|---|---|").count(), 1);
    }
}
