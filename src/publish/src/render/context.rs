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

    let children: Vec<Value> = if page.namespace.is_some() {
        vec![] // This page is a child, not a parent
    } else {
        // Check if this page is a namespace parent
        let page_name_lower = page.meta.title.to_lowercase();
        store
            .get_namespace_children(&page_name_lower)
            .iter()
            .map(|child| {
                minijinja::context! {
                    title => child.meta.title.rsplit('/').next().unwrap_or(&child.meta.title).to_string(),
                    url => format!("/{}", child.id),
                }
            })
            .collect()
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
        page => minijinja::context! {
            title => page.meta.title.clone(),
            display_name => page.meta.title.rsplit('/').next().unwrap_or(&page.meta.title).to_string(),
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
        },
        backlinks => backlink_data,
    }
}
