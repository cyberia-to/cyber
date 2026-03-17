pub mod context;
mod templates;
pub mod toc;
mod transform;

use crate::config::SiteConfig;
use crate::graph::PageStore;
use crate::lunar;
use crate::parser::PageId;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct RenderedPage {
    pub page_id: PageId,
    pub html: String,
    pub url_path: String,
}

pub fn render_all(store: &PageStore, config: &SiteConfig) -> Result<Vec<RenderedPage>> {
    render_cached(store, config, &mut HashMap::new(), None)
}

/// Create a template environment for use with `render_single_page`.
pub fn make_template_env(config: &SiteConfig) -> Result<minijinja::Environment<'static>> {
    templates::setup_environment(config.build.template_dir.as_deref(), config)
}

/// Render a single page (fast path — no iteration over the full store).
pub fn render_single_page(
    page: &crate::parser::ParsedPage,
    page_id: &str,
    store: &PageStore,
    config: &SiteConfig,
    env: &minijinja::Environment<'_>,
) -> Result<RenderedPage> {
    let render_result = transform::render_markdown(&page.content_md, store, &config.style.code.theme);
    let ctx = context::build_page_context(page, &render_result.html, &render_result.toc, store, config);
    let template_name = match page.kind {
        crate::parser::PageKind::Journal => "journal.html",
        crate::parser::PageKind::Page | crate::parser::PageKind::File => "page.html",
    };
    let tmpl = env.get_template(template_name)?;
    let html = tmpl.render(&ctx)?;
    let url_path = if config.urls.style == "pretty" {
        format!("/{}/index.html", page_id)
    } else {
        format!("/{}.html", page_id)
    };
    Ok(RenderedPage { page_id: page_id.to_string(), html, url_path })
}

/// Render pages with optional caching. When `dirty_ids` is Some, only pages in
/// the dirty set are re-rendered; clean pages are served from `cache`.
/// When `dirty_ids` is None, all pages are rendered (full rebuild).
/// The cache is updated with newly rendered pages.
pub fn render_cached(
    store: &PageStore,
    config: &SiteConfig,
    cache: &mut HashMap<PageId, RenderedPage>,
    dirty_ids: Option<&HashSet<PageId>>,
) -> Result<Vec<RenderedPage>> {
    let env = templates::setup_environment(config.build.template_dir.as_deref(), config)?;

    let mut rendered = Vec::new();

    // Reserved URL slugs — synthetic pages take priority over regular pages
    let reserved_slugs: std::collections::HashSet<&str> =
        ["tags", "blog", "graph", "files"]
            .into_iter()
            .collect();

    for (page_id, page) in &store.pages {
        // Skip regular pages that conflict with synthetic page URLs
        if reserved_slugs.contains(page_id.as_str()) {
            continue;
        }
        // Filter: public_only mode
        if !PageStore::is_page_public(page, &config.content) {
            continue;
        }

        // Check cache: skip rendering if page is clean
        if let Some(dirty) = dirty_ids {
            if !dirty.contains(page_id) {
                if let Some(cached) = cache.get(page_id) {
                    rendered.push(cached.clone());
                    continue;
                }
            }
        }

        // Transform markdown to HTML with wikilink resolution
        let render_result = transform::render_markdown(&page.content_md, store, &config.style.code.theme);

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
            crate::parser::PageKind::Page | crate::parser::PageKind::File => "page.html",
        };

        let tmpl = env.get_template(template_name)?;
        let html = tmpl.render(&ctx)?;

        let url_path = if config.urls.style == "pretty" {
            format!("/{}/index.html", page_id)
        } else {
            format!("/{}.html", page_id)
        };

        let rp = RenderedPage {
            page_id: page_id.clone(),
            html,
            url_path,
        };
        cache.insert(page_id.clone(), rp.clone());
        rendered.push(rp);
    }

    // Synthetic pages (index, tags, blog, files) only need re-rendering on
    // structural changes (pages added/removed, tags changed). The caller signals
    // this by including "__structural__" in the dirty set.
    let any_dirty = dirty_ids
        .map(|d| d.contains("__structural__"))
        .unwrap_or(true);

    // Helper: push a synthetic page, using cache when nothing is dirty
    macro_rules! push_synthetic {
        ($id:expr, $html:expr, $url:expr) => {{
            let id = $id.to_string();
            if any_dirty {
                let rp = RenderedPage {
                    page_id: id.clone(),
                    html: $html,
                    url_path: $url.to_string(),
                };
                cache.insert(id, rp.clone());
                rendered.push(rp);
            } else if let Some(cached) = cache.get(&id) {
                rendered.push(cached.clone());
            }
        }};
    }

    // Render index page
    if any_dirty {
        let index_html = render_index(store, config, &env)?;
        push_synthetic!("__index__", index_html, "/index.html");
    } else if let Some(cached) = cache.get("__index__") {
        rendered.push(cached.clone());
    }

    // Render tag pages (only for public pages)
    let public_tags = store.public_tag_index(&config.content);
    for (tag, page_ids) in &public_tags {
        let tag_slug = slug::slugify(tag);
        let id = format!("__tag__{}", tag_slug);
        if any_dirty {
            let tag_html = render_tag_page(tag, page_ids, store, config, &env)?;
            let rp = RenderedPage {
                page_id: id.clone(),
                html: tag_html,
                url_path: format!("/tags/{}/index.html", tag_slug),
            };
            cache.insert(id, rp.clone());
            rendered.push(rp);
        } else if let Some(cached) = cache.get(&id) {
            rendered.push(cached.clone());
        }
    }

    // Render tags index page
    if any_dirty {
        let tags_html = render_tags_index(store, config, &env)?;
        push_synthetic!("__tags_index__", tags_html, "/tags/index.html");
    } else if let Some(cached) = cache.get("__tags_index__") {
        rendered.push(cached.clone());
    }

    // Render blog page (date-sorted page listing)
    if any_dirty {
        let blog_html = render_blog(store, config, &env)?;
        push_synthetic!("__blog__", blog_html, "/blog/index.html");
    } else if let Some(cached) = cache.get("__blog__") {
        rendered.push(cached.clone());
    }

    // Render graph visualization page
    if config.graph.enabled {
        if any_dirty {
            let graph_html = render_graph_page(store, config, &env)?;
            push_synthetic!("__graph__", graph_html, "/graph/index.html");
        } else if let Some(cached) = cache.get("__graph__") {
            rendered.push(cached.clone());
        }
    }

    // Render unified files page (all pages with size column)
    if any_dirty {
        let files_html = render_files_page(store, config, &env)?;
        push_synthetic!("__files__", files_html, "/files/index.html");
    } else if let Some(cached) = cache.get("__files__") {
        rendered.push(cached.clone());
    }

    // Generate redirect pages for aliases
    for (alias_slug, canonical_id) in &store.alias_map {
        // Don't generate redirect if alias slug matches an existing page or reserved slug
        if store.pages.contains_key(alias_slug) || reserved_slugs.contains(alias_slug.as_str()) {
            continue;
        }
        let alias_id = format!("__alias__{}", alias_slug);
        if !any_dirty {
            if let Some(cached) = cache.get(&alias_id) {
                rendered.push(cached.clone());
                continue;
            }
        }
        let redirect_html = format!(
            r#"<!DOCTYPE html><html><head><meta charset="utf-8"><meta http-equiv="refresh" content="0;url=/{canonical_id}"><link rel="canonical" href="/{canonical_id}"><title>Redirecting…</title></head><body>Redirecting to <a href="/{canonical_id}">/{canonical_id}</a></body></html>"#
        );
        let rp = RenderedPage {
            page_id: alias_id.clone(),
            html: redirect_html,
            url_path: format!("/{}/index.html", alias_slug),
        };
        cache.insert(alias_id, rp.clone());
        rendered.push(rp);
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
                let render_result = transform::render_markdown(&page.content_md, store, &config.style.code.theme);
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
        favicon => config.site.favicon,
        description => config.site.description,
        canonical_url => config.site.base_url,
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
        favicon => config.site.favicon,
        description => format!("All tags — {}", config.site.title),
        canonical_url => format!("{}/tags", config.site.base_url),
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
        favicon => config.site.favicon,
        description => format!("Knowledge graph — {}", config.site.title),
        canonical_url => format!("{}/graph", config.site.base_url),
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
            let render_result = transform::render_markdown(&truncated_md, store, &config.style.code.theme);

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
        favicon => config.site.favicon,
        description => format!("Blog — {}", config.site.title),
        canonical_url => format!("{}/blog", config.site.base_url),
        posts => page_data,
        page_count => public_count,
    };

    let tmpl = env.get_template("blog.html")?;
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
        favicon => config.site.favicon,
        description => format!("Pages tagged '{}' — {}", tag, config.site.title),
        canonical_url => format!("{}/tags/{}", config.site.base_url, slug::slugify(tag)),
        tag_name => tag,
        pages => pages,
    };

    let tmpl = env.get_template("tag.html")?;
    Ok(tmpl.render(&ctx)?)
}

/// Format byte size into human-readable string.
fn format_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}

/// Extract git dates for all files under a directory.
/// Returns a map: absolute_path -> (created_iso, modified_iso).
/// Runs a single `git log` command for efficiency.
fn git_dates(input_dir: &Path) -> HashMap<String, (String, String)> {
    // Find the git repo root
    let repo_root = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(input_dir)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| PathBuf::from(String::from_utf8_lossy(&o.stdout).trim().to_string()));

    let repo_root = match repo_root {
        Some(r) => r,
        None => return HashMap::new(),
    };

    let output = std::process::Command::new("git")
        .args(["log", "--format=format:%aI", "--name-only", "--diff-filter=ACMR"])
        .current_dir(&repo_root)
        .output();

    let output = match output {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).to_string(),
        _ => return HashMap::new(),
    };

    // Parse: date line followed by filename lines, separated by blank lines.
    // Git log is newest-first, so first encounter = modified, last = created.
    let mut result: HashMap<String, (String, String)> = HashMap::new();

    let mut current_date = String::new();
    for line in output.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // Date lines start with digit (ISO format: 2026-03-02T...)
        if line.starts_with(|c: char| c.is_ascii_digit()) && line.contains('T') {
            current_date = line.to_string();
        } else if !current_date.is_empty() {
            // Convert git-relative path to absolute
            let abs_path = repo_root.join(line);
            let key = abs_path.to_string_lossy().to_string();
            let entry = result
                .entry(key)
                .or_insert_with(|| (current_date.clone(), current_date.clone()));
            // Newest-first: first insert sets modified, keep overwriting created
            entry.0 = current_date.clone();
        }
    }

    result
}

/// Compute percentile ranks (0.0–1.0) for a slice of values.
fn compute_percentiles(values: &[f64]) -> Vec<f64> {
    let n = values.len();
    if n <= 1 {
        return vec![0.0; n];
    }
    let mut indexed: Vec<(usize, f64)> = values.iter().copied().enumerate().collect();
    indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    let mut result = vec![0.0; n];
    let divisor = (n - 1).max(1) as f64;
    for (rank, (orig_idx, _)) in indexed.iter().enumerate() {
        result[*orig_idx] = rank as f64 / divisor;
    }
    result
}

/// Convert LMT "DD.MM.YY" to sortable "YY.MM.DD" with zero-padding.
fn lmt_sort_key(lmt: &str) -> String {
    let parts: Vec<&str> = lmt.split('.').collect();
    if parts.len() == 3 {
        format!("{:0>3}.{:0>2}.{:0>2}", parts[2], parts[1], parts[0])
    } else {
        String::new()
    }
}

/// Format a value as percentage of total: (val / sum) * 100, with 2 decimal places.
fn format_pct_of_sum(val: f64, sum: f64) -> String {
    if sum <= 0.0 || val <= 0.0 {
        "0".to_string()
    } else {
        format!("{:.2}", val / sum * 100.0)
    }
}

/// Render the unified /files page — all public pages sorted by focus (π) with size column.
fn render_files_page(
    store: &PageStore,
    config: &SiteConfig,
    env: &minijinja::Environment,
) -> Result<String> {
    // Extract git dates for all files
    let dates = git_dates(&config.build.input_dir);

    let mut pages: Vec<_> = store
        .public_pages(&config.content)
        .into_iter()
        .map(|p| {
            let links_in = store.backlinks.get(&p.id).map(|b| b.len()).unwrap_or(0);
            let links_out = store.forward_links.get(&p.id).map(|f| f.len()).unwrap_or(0);
            let focus = store.focus.get(&p.id).copied().unwrap_or(0.0);
            let gravity = store.gravity.get(&p.id).copied().unwrap_or(0.0);
            let size = if p.source_path.as_os_str().is_empty() {
                0u64
            } else {
                std::fs::metadata(&p.source_path)
                    .map(|m| m.len())
                    .unwrap_or(0)
            };
            let density = if size > 0 { focus / size as f64 } else { 0.0 };
            (p, links_in, links_out, focus, size, density, gravity)
        })
        .collect();

    // Sort by focus descending
    pages.sort_by(|a, b| b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal));

    // Compute percentiles for coloring
    let size_vals: Vec<f64> = pages.iter().map(|r| r.4 as f64).collect();
    let focus_vals: Vec<f64> = pages.iter().map(|r| r.3).collect();
    let in_vals: Vec<f64> = pages.iter().map(|r| r.1 as f64).collect();
    let out_vals: Vec<f64> = pages.iter().map(|r| r.2 as f64).collect();
    let den_vals: Vec<f64> = pages.iter().map(|r| r.5).collect();
    let grav_vals: Vec<f64> = pages.iter().map(|r| r.6).collect();

    let size_pcts = compute_percentiles(&size_vals);
    let focus_pcts = compute_percentiles(&focus_vals);
    let in_pcts = compute_percentiles(&in_vals);
    let out_pcts = compute_percentiles(&out_vals);
    let den_pcts = compute_percentiles(&den_vals);
    let grav_pcts = compute_percentiles(&grav_vals);

    // Sum values for normalizing density and gravity to distributions (Σ = 1)
    let sum_den: f64 = den_vals.iter().sum();
    let sum_grav: f64 = grav_vals.iter().sum();

    let files_data: Vec<_> = pages
        .iter()
        .enumerate()
        .map(|(i, (p, links_in, links_out, focus, size, density, gravity))| {
            let focus_display = format!("{:.2}", focus * 100.0);
            let size_display = format_size(*size);
            let file_title = match p.kind {
                crate::parser::PageKind::Page | crate::parser::PageKind::Journal => format!("{}.md", p.meta.title),
                crate::parser::PageKind::File => p.meta.title.clone(),
            };

            // Look up git dates by absolute source path
            let abs_key = p.source_path.to_string_lossy().to_string();
            let (created_lmt, modified_lmt) = dates
                .get(&abs_key)
                .map(|(created, modified)| {
                    let c = lunar::iso_to_lmt(created).unwrap_or_default();
                    let m = lunar::iso_to_lmt(modified).unwrap_or_default();
                    (c, m)
                })
                .unwrap_or_default();

            // HSL lightness: 95% (low) → 35% (high) — maps percentile to green intensity
            let size_light = 95.0 - size_pcts[i] * 60.0;
            let focus_light = 95.0 - focus_pcts[i] * 60.0;
            let in_light = 95.0 - in_pcts[i] * 60.0;
            let out_light = 95.0 - out_pcts[i] * 60.0;
            let den_light = 95.0 - den_pcts[i] * 60.0;
            let grav_light = 95.0 - grav_pcts[i] * 60.0;

            minijinja::context! {
                rank => i + 1,
                title => file_title,
                url => format!("/{}", p.id),
                links_in => *links_in,
                links_out => *links_out,
                pagerank => focus_display,
                size => size_display,
                density => format_pct_of_sum(*density, sum_den),
                gravity => format_pct_of_sum(*gravity, sum_grav),
                tags => p.meta.tags.clone(),
                icon => p.meta.icon.clone(),
                created => created_lmt,
                modified => modified_lmt,
                size_sort => *size,
                focus_sort => format!("{:.8}", focus),
                in_sort => *links_in,
                out_sort => *links_out,
                density_sort => format!("{:.8}", density),
                gravity_sort => format!("{:.10}", gravity),
                created_sort => lmt_sort_key(&created_lmt),
                modified_sort => lmt_sort_key(&modified_lmt),
                size_light => format!("{:.0}", size_light),
                focus_light => format!("{:.0}", focus_light),
                in_light => format!("{:.0}", in_light),
                out_light => format!("{:.0}", out_light),
                den_light => format!("{:.0}", den_light),
                grav_light => format!("{:.0}", grav_light),
            }
        })
        .collect();

    let total = files_data.len();

    let ctx = minijinja::context! {
        site => config.site,
        style => config.style,
        nav_menu => context::resolve_nav_menu(config, store),
        search => config.search,
        analytics => config.analytics,
        graph => config.graph,
        favicon => config.site.favicon,
        description => format!("{} files in the knowledge graph", total),
        canonical_url => format!("{}/files", config.site.base_url),
        page_count => total,
        files => files_data,
        total_files => total,
    };

    let tmpl = env.get_template("files.html")?;
    Ok(tmpl.render(&ctx)?)
}
