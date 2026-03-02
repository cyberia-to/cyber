use comrak::nodes::{AstNode, NodeValue};
use serde::Serialize;

/// A table of contents entry extracted from headings.
#[derive(Debug, Clone, Serialize)]
pub struct TocEntry {
    pub level: u8,
    pub text: String,
    pub id: String,
}

/// Extract headings from a comrak AST to build a TOC.
pub fn extract_toc<'a>(root: &'a AstNode<'a>) -> Vec<TocEntry> {
    let mut entries = Vec::new();

    for node in root.descendants() {
        let data = node.data.borrow();
        if let NodeValue::Heading(ref heading) = data.value {
            let text = get_text_content(node);
            if !text.is_empty() {
                let id = slug::slugify(&text);
                entries.push(TocEntry {
                    level: heading.level,
                    text,
                    id,
                });
            }
        }
    }

    entries
}

fn get_text_content<'a>(node: &'a AstNode<'a>) -> String {
    let mut text = String::new();
    for child in node.children() {
        let data = child.data.borrow();
        match &data.value {
            NodeValue::Text(t) => text.push_str(t),
            NodeValue::Code(c) => text.push_str(&c.literal),
            _ => text.push_str(&get_text_content(child)),
        }
    }
    text
}

/// Render TOC entries as nested HTML list.
pub fn render_toc_html(entries: &[TocEntry]) -> String {
    if entries.is_empty() {
        return String::new();
    }

    let mut html = String::from("<nav class=\"toc\" aria-label=\"Table of Contents\">\n<h3>Contents</h3>\n<ul>\n");
    let mut prev_level = entries[0].level;

    for entry in entries {
        if entry.level > prev_level {
            for _ in 0..(entry.level - prev_level) {
                html.push_str("<ul>\n");
            }
        } else if entry.level < prev_level {
            for _ in 0..(prev_level - entry.level) {
                html.push_str("</ul>\n");
            }
        }
        html.push_str(&format!(
            "<li><a href=\"#{}\">{}</a></li>\n",
            entry.id, entry.text
        ));
        prev_level = entry.level;
    }

    // Close remaining nested lists
    for _ in entries[0].level..prev_level {
        html.push_str("</ul>\n");
    }

    html.push_str("</ul>\n</nav>");
    html
}

