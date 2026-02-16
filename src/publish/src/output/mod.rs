mod assets;
mod feed;
pub mod files;
pub mod graph;
mod search;
mod sitemap;

use crate::config::SiteConfig;
use crate::graph::PageStore;
use crate::render::RenderedPage;
use crate::scanner::DiscoveredFiles;
use anyhow::Result;
use std::fs;
use std::path::Path;

/// Default CSS and JS baked into the binary
const DEFAULT_CSS: &str = include_str!("../../static/style.css");
const DEFAULT_SEARCH_JS: &str = include_str!("../../static/search.js");
const DEFAULT_GRAPH_JS: &str = include_str!("../../static/graph.js");
const DEFAULT_TOPICS_JS: &str = include_str!("../../static/topics.js");

/// Play font files (woff2) baked into binary
const FONT_FILES: &[(&str, &[u8])] = &[
    (
        "play-400-cyrillic-ext.woff2",
        include_bytes!("../../static/fonts/play-400-cyrillic-ext.woff2"),
    ),
    (
        "play-400-cyrillic.woff2",
        include_bytes!("../../static/fonts/play-400-cyrillic.woff2"),
    ),
    (
        "play-400-greek.woff2",
        include_bytes!("../../static/fonts/play-400-greek.woff2"),
    ),
    (
        "play-400-vietnamese.woff2",
        include_bytes!("../../static/fonts/play-400-vietnamese.woff2"),
    ),
    (
        "play-400-latin-ext.woff2",
        include_bytes!("../../static/fonts/play-400-latin-ext.woff2"),
    ),
    (
        "play-400-latin.woff2",
        include_bytes!("../../static/fonts/play-400-latin.woff2"),
    ),
    (
        "play-700-cyrillic-ext.woff2",
        include_bytes!("../../static/fonts/play-700-cyrillic-ext.woff2"),
    ),
    (
        "play-700-cyrillic.woff2",
        include_bytes!("../../static/fonts/play-700-cyrillic.woff2"),
    ),
    (
        "play-700-greek.woff2",
        include_bytes!("../../static/fonts/play-700-greek.woff2"),
    ),
    (
        "play-700-vietnamese.woff2",
        include_bytes!("../../static/fonts/play-700-vietnamese.woff2"),
    ),
    (
        "play-700-latin-ext.woff2",
        include_bytes!("../../static/fonts/play-700-latin-ext.woff2"),
    ),
    (
        "play-700-latin.woff2",
        include_bytes!("../../static/fonts/play-700-latin.woff2"),
    ),
];

pub fn write_output(
    rendered: &[RenderedPage],
    store: &PageStore,
    config: &SiteConfig,
    discovered: &DiscoveredFiles,
) -> Result<()> {
    let output_dir = &config.build.output_dir;

    // Clean output directory (retry on macOS "Directory not empty" race)
    if output_dir.exists() {
        let mut last_err = None;
        for _ in 0..3 {
            match fs::remove_dir_all(output_dir) {
                Ok(()) => {
                    last_err = None;
                    break;
                }
                Err(e) => {
                    last_err = Some(e);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            }
        }
        if let Some(e) = last_err {
            return Err(e.into());
        }
    }
    fs::create_dir_all(output_dir)?;

    // Write rendered HTML pages
    for page in rendered {
        let file_path = output_dir.join(page.url_path.trim_start_matches('/'));
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&file_path, &page.html)?;
    }

    // Write static assets (default CSS/JS)
    write_default_static(output_dir, config)?;

    // Copy user static files
    if let Some(ref static_dir) = config.build.static_dir {
        let static_path = config.build.input_dir.join(static_dir);
        if static_path.exists() {
            assets::copy_dir_recursive(&static_path, output_dir)?;
        }
    }

    // Copy assets from graph
    assets::copy_assets(discovered, output_dir)?;

    // Generate RSS feed
    if config.feeds.enabled {
        feed::generate_feed(store, config, output_dir)?;
    }

    // Generate sitemap
    sitemap::generate_sitemap(rendered, config, output_dir)?;

    // Generate search index
    if config.search.enabled {
        search::generate_search_index(store, config, output_dir)?;
    }

    // Generate graph data
    if config.graph.enabled {
        graph::generate_graph_data(store, config, output_dir)?;
    }

    // Generate topics graph data (always — used by topics page and index)
    graph::generate_topics_data(store, config, output_dir)?;

    // Generate files index
    let file_entries = files::build_file_index(store, config);
    files::write_files_index(&file_entries, output_dir)?;

    Ok(())
}

fn write_default_static(output_dir: &Path, config: &SiteConfig) -> Result<()> {
    let static_dir = output_dir.join("static");
    fs::create_dir_all(&static_dir)?;

    // Write CSS with custom properties injected
    let css = inject_css_variables(DEFAULT_CSS, config);
    fs::write(static_dir.join("style.css"), css)?;

    // Write search JS
    if config.search.enabled {
        fs::write(static_dir.join("search.js"), DEFAULT_SEARCH_JS)?;
    }

    // Write graph JS
    if config.graph.enabled {
        fs::write(static_dir.join("graph.js"), DEFAULT_GRAPH_JS)?;
    }

    // Write topics JS
    fs::write(static_dir.join("topics.js"), DEFAULT_TOPICS_JS)?;

    // Write font files
    let fonts_dir = static_dir.join("fonts");
    fs::create_dir_all(&fonts_dir)?;
    for (name, data) in FONT_FILES {
        fs::write(fonts_dir.join(name), data)?;
    }

    Ok(())
}

fn inject_css_variables(css: &str, config: &SiteConfig) -> String {
    let style = &config.style;
    let vars = format!(
        r#":root {{
  --color-primary: {primary};
  --color-secondary: {secondary};
  --color-bg: {bg};
  --color-text: {text};
  --color-surface: {surface};
  --color-border: {border};
  --color-green: {primary};
  --color-blue: #3b9eff;
  --color-red: #ef4444;
  --font-body: {font_body};
  --font-mono: {font_mono};
  --font-size-base: {font_size};
  --line-height: {line_height};
  --max-width: {max_width};
  --sidebar-width: 220px;
}}

"#,
        primary = style.primary_color,
        secondary = style.secondary_color,
        bg = style.bg_color,
        text = style.text_color,
        surface = style.surface_color,
        border = style.border_color,
        font_body = style.typography.font_body,
        font_mono = style.typography.font_mono,
        font_size = style.typography.font_size_base,
        line_height = style.typography.line_height,
        max_width = style.typography.max_width,
    );

    format!("{}\n{}", vars, css)
}
