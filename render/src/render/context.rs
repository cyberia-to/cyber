use crate::config::SiteConfig;
use crate::graph::PageStore;
use crate::parser::ParsedPage;
use crate::render::toc::{self, TocEntry};
use minijinja::Value;

/// Resolve nav menu items: convert page names to URLs, use page icons when available.
/// When `nav.menu_tag` is set, auto-generates menu from pages that have that tag.
pub fn resolve_nav_menu(config: &SiteConfig, store: &PageStore) -> Vec<Value> {
    if let Some(ref tag) = config.nav.menu_tag {
        resolve_nav_menu_from_tag(tag, store)
    } else {
        resolve_nav_menu_from_config(config, store)
    }
}

/// Build menu from pages that have a specific tag (e.g. "menu").
/// Sorted by `menu-order::` property (ascending), then alphabetically by title.
fn resolve_nav_menu_from_tag(tag: &str, store: &PageStore) -> Vec<Value> {
    let tag_lower = tag.to_lowercase();
    let mut menu_pages: Vec<&crate::parser::ParsedPage> = store
        .pages
        .values()
        .filter(|page| page.meta.tags.iter().any(|t| t.to_lowercase() == tag_lower))
        .collect();

    menu_pages.sort_by(|a, b| {
        let ord_a = a.meta.menu_order.unwrap_or(i32::MAX);
        let ord_b = b.meta.menu_order.unwrap_or(i32::MAX);
        ord_a
            .cmp(&ord_b)
            .then_with(|| a.meta.title.cmp(&b.meta.title))
    });

    menu_pages
        .iter()
        .map(|page| {
            let url = format!("/{}", page.id);
            let icon = page.meta.icon.clone();
            // Title-case the label: capitalize first letter of each word
            let label = title_case(&page.meta.title);

            minijinja::context! {
                label => label,
                url => url,
                external => false,
                active => false,
                icon => icon,
            }
        })
        .collect()
}

/// Capitalize the first letter of each word.
fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generate a clean plain-text excerpt from raw markdown.
/// Strips wikilinks, headings, bullets, code fences, and collapses whitespace.
/// Truncates at word boundary to `max_chars` (default 160), appending `…`.
pub fn generate_excerpt(md: &str, max_chars: usize) -> String {
    let mut lines: Vec<&str> = Vec::new();
    let mut in_code_fence = false;

    for line in md.lines() {
        let trimmed = line.trim();

        // Skip code fences and their content
        if trimmed.starts_with("```") {
            in_code_fence = !in_code_fence;
            continue;
        }
        if in_code_fence {
            continue;
        }

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Skip frontmatter markers
        if trimmed == "---" {
            continue;
        }

        // Skip heading markers but keep text
        let text = if trimmed.starts_with('#') {
            trimmed.trim_start_matches('#').trim()
        } else {
            trimmed
        };

        // Strip bullet prefixes
        let text = text
            .strip_prefix("- ")
            .or_else(|| text.strip_prefix("* "))
            .unwrap_or(text);

        if text.is_empty() {
            continue;
        }

        lines.push(text);
    }

    let joined = lines.join(" ");

    // Strip [[wikilink]] syntax → keep inner text
    let mut result = String::with_capacity(joined.len());
    let chars: Vec<char> = joined.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if i + 1 < len && chars[i] == '[' && chars[i + 1] == '[' {
            // Find closing ]]
            i += 2;
            while i + 1 < len && !(chars[i] == ']' && chars[i + 1] == ']') {
                result.push(chars[i]);
                i += 1;
            }
            if i + 1 < len {
                i += 2; // skip ]]
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }

    // Strip {{query ...}} and {{embed ...}} expressions
    let mut clean = String::with_capacity(result.len());
    let chars: Vec<char> = result.chars().collect();
    let len = chars.len();
    let mut i = 0;
    while i < len {
        if i + 1 < len && chars[i] == '{' && chars[i + 1] == '{' {
            // Skip until }}
            i += 2;
            while i + 1 < len && !(chars[i] == '}' && chars[i + 1] == '}') {
                i += 1;
            }
            if i + 1 < len {
                i += 2;
            }
        } else {
            clean.push(chars[i]);
            i += 1;
        }
    }

    // Collapse whitespace
    let collapsed: String = clean.split_whitespace().collect::<Vec<_>>().join(" ");

    // Truncate at word boundary (char-aware for UTF-8)
    let char_count = collapsed.chars().count();
    if char_count <= max_chars {
        return collapsed;
    }

    let truncated: String = collapsed.chars().take(max_chars).collect();
    if let Some(last_space) = truncated.rfind(' ') {
        format!("{}…", &truncated[..last_space])
    } else {
        format!("{}…", truncated)
    }
}

/// Build menu from static config entries (original behavior).
fn resolve_nav_menu_from_config(config: &SiteConfig, store: &PageStore) -> Vec<Value> {
    config
        .nav
        .menu
        .iter()
        .map(|item| {
            let slug = item
                .page
                .as_ref()
                .map(|p| crate::parser::slugify_page_name(p));
            let url = if let Some(ref s) = slug {
                format!("/{}", s)
            } else if let Some(ref url) = item.url {
                url.clone()
            } else {
                "#".to_string()
            };

            // Prefer page's own icon:: property over nav config icon
            let icon = slug
                .as_ref()
                .and_then(|s| store.pages.get(s))
                .and_then(|p| p.meta.icon.clone())
                .or_else(|| item.icon.clone());

            minijinja::context! {
                label => item.label.clone(),
                url => url,
                external => item.external,
                active => false,
                icon => icon,
            }
        })
        .collect()
}

/// Build the complete template context for rendering a page.
pub fn build_page_context(
    page: &ParsedPage,
    html_body: &str,
    toc_entries: &[TocEntry],
    store: &PageStore,
    config: &SiteConfig,
) -> Value {
    let backlinks = store.get_backlinks(&page.id);
    let backlink_data: Vec<Value> = backlinks
        .iter()
        .map(|bl| {
            minijinja::context! {
                title => bl.title.clone(),
                url => bl.url.clone(),
            }
        })
        .collect();

    let word_count = page.content_md.split_whitespace().count();
    let reading_time = (word_count as f64 / 200.0).ceil() as usize;

    let children: Vec<Value> = {
        // Any page can be a namespace parent — check by its title
        let page_name_lower = page.meta.title.to_lowercase();

        // Direct children (pages whose namespace == this page's name)
        let mut items: Vec<Value> = store
            .get_namespace_children(&page_name_lower)
            .iter()
            .map(|child| {
                minijinja::context! {
                    title => child.meta.title.rsplit('/').next().unwrap_or(&child.meta.title).to_string(),
                    url => format!("/{}", child.id),
                }
            })
            .collect();

        // Immediate sub-namespaces (folders one level deeper).
        // e.g., for "trident" find "trident/docs", "trident/src", "trident/editor" etc.
        let prefix = format!("{}/", page_name_lower);
        let mut seen_subns: std::collections::HashSet<String> = std::collections::HashSet::new();
        let mut folder_slugs: std::collections::HashSet<String> = std::collections::HashSet::new();
        for ns_key in store.namespace_tree.keys() {
            if let Some(rest) = ns_key.strip_prefix(&prefix) {
                let sub = rest.split('/').next().unwrap_or(rest);
                if seen_subns.insert(sub.to_string()) {
                    let sub_page_slug = crate::parser::slugify_page_name(&format!("{}/{}", page_name_lower, sub));
                    folder_slugs.insert(sub_page_slug.clone());
                    let url = format!("/{}", sub_page_slug);
                    items.push(minijinja::context! {
                        title => format!("{}/", sub),
                        url => url,
                    });
                }
            }
        }

        // Remove direct children that have a matching folder entry (avoid duplicates)
        items.retain(|item| {
            let url: String = item.get_attr("url").ok().and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
            let title: String = item.get_attr("title").ok().and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
            if title.ends_with('/') {
                return true; // Keep all folder entries
            }
            // Strip leading / from url to get slug
            let slug = url.trim_start_matches('/');
            !folder_slugs.contains(slug)
        });

        // Sort: folders (ending with /) first, then files
        items.sort_by(|a, b| {
            let a_title: String = a.get_attr("title").ok().and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
            let b_title: String = b.get_attr("title").ok().and_then(|v| v.as_str().map(|s| s.to_string())).unwrap_or_default();
            let a_is_dir = a_title.ends_with('/');
            let b_is_dir = b_title.ends_with('/');
            match (a_is_dir, b_is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a_title.cmp(&b_title),
            }
        });

        items
    };

    let nav_menu = resolve_nav_menu(config, store);

    // Generate TOC HTML if page has headings
    let toc_html = if toc_entries.len() >= 2 {
        toc::render_toc_html(toc_entries)
    } else {
        String::new()
    };

    // Build namespace breadcrumb parts
    let namespace_parts: Vec<Value> = if let Some(ref ns) = page.namespace {
        let segments: Vec<&str> = ns.split('/').collect();
        let mut parts = Vec::new();
        for (i, seg) in segments.iter().enumerate() {
            let full_path = segments[..=i].join("/");
            let slug = crate::parser::slugify_page_name(&full_path);
            parts.push(minijinja::context! {
                name => seg.to_string(),
                url => format!("/{}", slug),
            });
        }
        parts
    } else {
        vec![]
    };

    // Resolve description: frontmatter description > auto-excerpt > title fallback
    let description = page
        .meta
        .properties
        .get("description")
        .filter(|d| !d.is_empty())
        .cloned()
        .unwrap_or_else(|| {
            let excerpt = generate_excerpt(&page.content_md, 160);
            if excerpt.is_empty() {
                page.meta.title.clone()
            } else {
                excerpt
            }
        });

    let canonical_url = format!("{}/{}", config.site.base_url, page.id);

    // Dimensional peers: pages with the same base name in different namespaces.
    // e.g., "truth" (root) and "cyber/truth" are dimensional peers.
    let base_name = page.meta.title.rsplit('/').next()
        .unwrap_or(&page.meta.title).to_lowercase();
    let mut dimensional_peers: Vec<Value> = store.pages.values()
        .filter(|p| {
            p.id != page.id
                && p.meta.title.rsplit('/').next()
                    .unwrap_or(&p.meta.title)
                    .to_lowercase() == base_name
                && PageStore::is_page_public(p, &config.content)
        })
        .map(|peer| {
            let peer_render = super::transform::render_markdown(
                &peer.content_md, store, &config.style.code.theme,
            );
            let depth = peer.meta.title.matches('/').count();
            minijinja::context! {
                title => peer.meta.title.clone(),
                path => format!("/{}", peer.id),
                icon => peer.meta.icon.clone(),
                namespace => peer.namespace.clone(),
                html_content => peer_render.html,
                depth => depth,
            }
        })
        .collect();
    // Sort: when viewing a namespaced page, root peer comes first (depth 0).
    // When viewing root, most specific peer comes first (highest depth).
    let current_depth = page.meta.title.matches('/').count();
    if current_depth == 0 {
        // Root page: show most specific peers first
        dimensional_peers.sort_by(|a, b| {
            let ad: i64 = a.get_attr("depth").ok().and_then(|v| i64::try_from(v).ok()).unwrap_or(0);
            let bd: i64 = b.get_attr("depth").ok().and_then(|v| i64::try_from(v).ok()).unwrap_or(0);
            bd.cmp(&ad)
        });
    } else {
        // Namespaced page: show root (depth 0) first
        dimensional_peers.sort_by(|a, b| {
            let ad: i64 = a.get_attr("depth").ok().and_then(|v| i64::try_from(v).ok()).unwrap_or(0);
            let bd: i64 = b.get_attr("depth").ok().and_then(|v| i64::try_from(v).ok()).unwrap_or(0);
            ad.cmp(&bd)
        });
    }

    // Resolve favicon: page icon > namespace parent icon > site favicon
    let favicon = page
        .meta
        .icon
        .clone()
        .or_else(|| {
            // Walk up namespace parents to find an icon
            if let Some(ref ns) = page.namespace {
                let segments: Vec<&str> = ns.split('/').collect();
                for i in (0..segments.len()).rev() {
                    let parent_path = segments[..=i].join("/");
                    let parent_slug = crate::parser::slugify_page_name(&parent_path);
                    if let Some(parent) = store.pages.get(&parent_slug) {
                        if parent.meta.icon.is_some() {
                            return parent.meta.icon.clone();
                        }
                    }
                }
            }
            None
        })
        .or_else(|| config.site.favicon.clone());

    minijinja::context! {
        site => config.site,
        style => config.style,
        nav_menu => nav_menu,
        graph => config.graph,
        analytics => config.analytics,
        search => config.search,
        favicon => favicon,
        description => description,
        canonical_url => canonical_url,
        page => minijinja::context! {
            title => page.meta.title.clone(),
            display_name => {
                let base = page.meta.title.rsplit('/').next().unwrap_or(&page.meta.title);
                match page.kind {
                    crate::parser::PageKind::Page | crate::parser::PageKind::Journal => format!("{}.md", base),
                    crate::parser::PageKind::File => base.to_string(),
                }
            },
            id => page.id.clone(),
            html_content => html_body,
            meta => page.meta.properties.clone(),
            tags => page.meta.tags.clone(),
            aliases => page.meta.aliases.clone(),
            url => format!("/{}", page.id),
            namespace => page.namespace.clone(),
            namespace_parts => namespace_parts,
            children => children,
            word_count => word_count,
            reading_time_minutes => reading_time,
            date => page.meta.date.map(|d| d.format("%Y-%m-%d").to_string()),
            icon => page.meta.icon.clone(),
            kind => format!("{:?}", page.kind),
            toc => toc_html,
            focus => store.focus.get(&page.id).copied().unwrap_or(0.0),
        },
        backlinks => backlink_data,
        dimensional_peers => dimensional_peers,
    }
}
