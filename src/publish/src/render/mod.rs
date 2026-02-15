mod context;
mod templates;
pub mod toc;
mod transform;

use crate::config::SiteConfig;
use crate::graph::PageStore;
use crate::parser::PageId;
use anyhow::Result;

#[derive(Debug)]
pub struct RenderedPage {
    pub page_id: PageId,
    pub html: String,
    pub url_path: String,
}

pub fn render_all(store: &PageStore, config: &SiteConfig) -> Result<Vec<RenderedPage>> {
    let env = templates::setup_environment(config.build.template_dir.as_deref(), config)?;

    let mut rendered = Vec::new();

    // Reserved URL slugs — synthetic pages take priority over regular pages
    let reserved_slugs: std::collections::HashSet<&str> =
        ["pages", "tags", "blog", "graph"].into_iter().collect();

    for (page_id, page) in &store.pages {
        // Skip regular pages that conflict with synthetic page URLs
        if reserved_slugs.contains(page_id.as_str()) {
            continue;
        }
        // Filter: public_only mode
        if !PageStore::is_page_public(page, &config.content) {
            continue;
        }

        // Transform markdown to HTML with wikilink resolution
        let render_result = transform::render_markdown(&page.content_md, store);

        // Build template context
        let ctx = context::build_page_context(
            page,
            &render_result.html,
            &render_result.toc,
            store,
            config,
        );

        // Render through template
        let template_name = match page.kind {
            crate::parser::PageKind::Journal => "journal.html",
            crate::parser::PageKind::Page => "page.html",
        };

        let tmpl = env.get_template(template_name)?;
        let html = tmpl.render(&ctx)?;

        let url_path = if config.urls.style == "pretty" {
            format!("/{}/index.html", page_id)
        } else {
            format!("/{}.html", page_id)
        };

        rendered.push(RenderedPage {
            page_id: page_id.clone(),
            html,
            url_path,
        });
    }

    // Render index page
    let index_html = render_index(store, config, &env)?;
    rendered.push(RenderedPage {
        page_id: "__index__".to_string(),
        html: index_html,
        url_path: "/index.html".to_string(),
    });

    // Render tag pages (only for public pages)
    let public_tags = store.public_tag_index(&config.content);
    for (tag, page_ids) in &public_tags {
        let tag_html = render_tag_page(tag, page_ids, store, config, &env)?;
        let tag_slug = slug::slugify(tag);
        rendered.push(RenderedPage {
            page_id: format!("__tag__{}", tag_slug),
            html: tag_html,
            url_path: format!("/tags/{}/index.html", tag_slug),
        });
    }

    // Render tags index page
    {
        let tags_html = render_tags_index(store, config, &env)?;
        rendered.push(RenderedPage {
            page_id: "__tags_index__".to_string(),
            html: tags_html,
            url_path: "/tags/index.html".to_string(),
        });
    }

    // Render blog page (date-sorted page listing)
    {
        let blog_html = render_blog(store, config, &env)?;
        rendered.push(RenderedPage {
            page_id: "__blog__".to_string(),
            html: blog_html,
            url_path: "/blog/index.html".to_string(),
        });
    }

    // Render pages discovery page
    {
        let pages_html = render_pages_index(store, config, &env)?;
        rendered.push(RenderedPage {
            page_id: "__pages_index__".to_string(),
            html: pages_html,
            url_path: "/pages/index.html".to_string(),
        });
    }

    // Render graph visualization page
    if config.graph.enabled {
        let graph_html = render_graph_page(store, config, &env)?;
        rendered.push(RenderedPage {
            page_id: "__graph__".to_string(),
            html: graph_html,
            url_path: "/graph/index.html".to_string(),
        });
    }

    // Generate redirect pages for aliases
    for (alias_slug, canonical_id) in &store.alias_map {
        // Don't generate redirect if alias slug matches an existing page
        if store.pages.contains_key(alias_slug) {
            continue;
        }
        let redirect_html = format!(
            r#"<!DOCTYPE html><html><head><meta charset="utf-8"><meta http-equiv="refresh" content="0;url=/{canonical_id}"><link rel="canonical" href="/{canonical_id}"><title>Redirecting…</title></head><body>Redirecting to <a href="/{canonical_id}">/{canonical_id}</a></body></html>"#
        );
        rendered.push(RenderedPage {
            page_id: format!("__alias__{}", alias_slug),
            html: redirect_html,
            url_path: format!("/{}/index.html", alias_slug),
        });
    }

    Ok(rendered)
}

fn render_index(
    store: &PageStore,
    config: &SiteConfig,
    env: &minijinja::Environment,
) -> Result<String> {
    // If there's a root page configured, render it as index
    if let Some(ref root_page_name) = config.site.root_page {
        let root_id = crate::parser::slugify_page_name(root_page_name);
        if let Some(page) = store.pages.get(&root_id) {
            if PageStore::is_page_public(page, &config.content) {
                let render_result = transform::render_markdown(&page.content_md, store);
                let ctx = context::build_page_context(
                    page,
                    &render_result.html,
                    &render_result.toc,
                    store,
                    config,
                );
                let tmpl = env.get_template("page.html")?;
                return Ok(tmpl.render(&ctx)?);
            }
        }
    }

    // Auto-generated index — only show public pages
    let public_count = store.public_pages(&config.content).len();
    let recent = store.recent_pages(20, &config.content);
    let recent_data: Vec<_> = recent
        .iter()
        .map(|p| {
            minijinja::context! {
                title => p.meta.title.clone(),
                url => format!("/{}", p.id),
                date => p.meta.date.map(|d| d.format("%Y-%m-%d").to_string()),
                tags => p.meta.tags.clone(),
                icon => p.meta.icon.clone(),
            }
        })
        .collect();

    let tags = store.all_tags(&config.content);
    let tag_data: Vec<_> = tags
        .iter()
        .map(|(name, count)| {
            minijinja::context! {
                name => *name,
                count => *count,
                url => format!("/tags/{}", slug::slugify(name)),
            }
        })
        .collect();

    let ctx = minijinja::context! {
        site => config.site,
        style => config.style,
        nav_menu => context::resolve_nav_menu(config, store),
        search => config.search,
        analytics => config.analytics,
        graph => config.graph,
        recent_pages => recent_data,
        tags => tag_data,
        page_count => public_count,
    };

    let tmpl = env.get_template("index.html")?;
    Ok(tmpl.render(&ctx)?)
}

fn render_tags_index(
    store: &PageStore,
    config: &SiteConfig,
    env: &minijinja::Environment,
) -> Result<String> {
    let tags = store.all_tags(&config.content);
    let max_count = tags.iter().map(|(_, c)| *c).max().unwrap_or(1) as f64;

    let tag_data: Vec<_> = tags
        .iter()
        .map(|(name, count)| {
            // Scale font size from 0.8 to 2.0 based on count
            let ratio = (*count as f64) / max_count;
            let size = 0.8 + ratio * 1.2;
            minijinja::context! {
                name => *name,
                count => *count,
                url => format!("/tags/{}", slug::slugify(name)),
                size => format!("{:.1}", size),
            }
        })
        .collect();

    let public_count = store.public_pages(&config.content).len();

    let ctx = minijinja::context! {
        site => config.site,
        style => config.style,
        nav_menu => context::resolve_nav_menu(config, store),
        search => config.search,
        analytics => config.analytics,
        graph => config.graph,
        tags => tag_data,
        page_count => public_count,
    };

    let tmpl = env.get_template("tags-index.html")?;
    Ok(tmpl.render(&ctx)?)
}

fn render_graph_page(
    store: &PageStore,
    config: &SiteConfig,
    env: &minijinja::Environment,
) -> Result<String> {
    let public_count = store.public_pages(&config.content).len();

    let ctx = minijinja::context! {
        site => config.site,
        style => config.style,
        nav_menu => context::resolve_nav_menu(config, store),
        search => config.search,
        analytics => config.analytics,
        graph => config.graph,
        page_count => public_count,
    };

    let tmpl = env.get_template("graph.html")?;
    Ok(tmpl.render(&ctx)?)
}

/// Truncate markdown to first N non-empty content lines (bullet items).
/// Returns (truncated_md, was_truncated).
fn truncate_markdown(md: &str, max_lines: usize) -> (String, bool) {
    let content_lines: Vec<&str> = md.lines().filter(|l| !l.trim().is_empty()).collect();

    if content_lines.len() <= max_lines {
        (md.to_string(), false)
    } else {
        let truncated = content_lines[..max_lines].join("\n");
        (truncated, true)
    }
}

fn render_blog(
    store: &PageStore,
    config: &SiteConfig,
    env: &minijinja::Environment,
) -> Result<String> {
    use crate::parser::PageKind;

    // Blog = journal entries, sorted by date descending
    let mut journals: Vec<_> = store
        .pages
        .values()
        .filter(|p| {
            p.kind == PageKind::Journal
                && PageStore::is_page_public(p, &config.content)
                && p.meta.date.is_some()
        })
        .collect();
    journals.sort_by(|a, b| b.meta.date.unwrap().cmp(&a.meta.date.unwrap()));

    let page_data: Vec<_> = journals
        .iter()
        .map(|p| {
            // Truncate to 3 lines max, then render as full HTML with wikilinks
            let (truncated_md, was_truncated) = truncate_markdown(&p.content_md, 3);
            let render_result = transform::render_markdown(&truncated_md, store);

            minijinja::context! {
                title => p.meta.date.map(|d| d.format("%B %d, %Y").to_string()).unwrap_or_else(|| p.meta.title.clone()),
                url => format!("/{}", p.id),
                date => p.meta.date.map(|d| d.format("%Y-%m-%d").to_string()),
                tags => p.meta.tags.clone(),
                icon => p.meta.icon.clone(),
                content_html => render_result.html,
                truncated => was_truncated,
            }
        })
        .collect();

    let public_count = store.public_pages(&config.content).len();

    let ctx = minijinja::context! {
        site => config.site,
        style => config.style,
        nav_menu => context::resolve_nav_menu(config, store),
        search => config.search,
        analytics => config.analytics,
        graph => config.graph,
        posts => page_data,
        page_count => public_count,
    };

    let tmpl = env.get_template("blog.html")?;
    Ok(tmpl.render(&ctx)?)
}

fn render_pages_index(
    store: &PageStore,
    config: &SiteConfig,
    env: &minijinja::Environment,
) -> Result<String> {
    let mut pages: Vec<_> = store
        .public_pages(&config.content)
        .into_iter()
        .map(|p| {
            let backlink_count = store.backlinks.get(&p.id).map(|b| b.len()).unwrap_or(0);
            let pr = store.pagerank.get(&p.id).copied().unwrap_or(0.0);
            (p, backlink_count, pr)
        })
        .collect();

    // Sort by PageRank descending
    pages.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));

    let page_data: Vec<_> = pages
        .iter()
        .enumerate()
        .map(|(i, (p, backlinks, pr))| {
            // Format PageRank as a readable score (multiply by 1000 for display)
            let pr_display = format!("{:.2}", pr * 1000.0);
            minijinja::context! {
                rank => i + 1,
                title => p.meta.title.clone(),
                url => format!("/{}", p.id),
                backlinks => *backlinks,
                pagerank => pr_display,
                tags => p.meta.tags.clone(),
                icon => p.meta.icon.clone(),
            }
        })
        .collect();

    let ctx = minijinja::context! {
        site => config.site,
        style => config.style,
        nav_menu => context::resolve_nav_menu(config, store),
        search => config.search,
        analytics => config.analytics,
        graph => config.graph,
        pages => page_data,
        page_count => page_data.len(),
    };

    let tmpl = env.get_template("pages-index.html")?;
    Ok(tmpl.render(&ctx)?)
}

fn render_tag_page(
    tag: &str,
    page_ids: &[PageId],
    store: &PageStore,
    config: &SiteConfig,
    env: &minijinja::Environment,
) -> Result<String> {
    let pages: Vec<_> = page_ids
        .iter()
        .filter_map(|id| {
            store.pages.get(id).map(|p| {
                minijinja::context! {
                    title => p.meta.title.clone(),
                    url => format!("/{}", p.id),
                    date => p.meta.date.map(|d| d.format("%Y-%m-%d").to_string()),
                }
            })
        })
        .collect();

    let ctx = minijinja::context! {
        site => config.site,
        style => config.style,
        nav_menu => context::resolve_nav_menu(config, store),
        search => config.search,
        analytics => config.analytics,
        graph => config.graph,
        tag_name => tag,
        pages => pages,
    };

    let tmpl = env.get_template("tag.html")?;
    Ok(tmpl.render(&ctx)?)
}
