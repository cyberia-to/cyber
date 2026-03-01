use crate::config::SiteConfig;
use anyhow::Result;
use minijinja::Environment;
use std::path::Path;

// Default templates baked into the binary
const DEFAULT_BASE: &str = include_str!("../../templates/base.html");
const DEFAULT_PAGE: &str = include_str!("../../templates/page.html");
const DEFAULT_INDEX: &str = include_str!("../../templates/index.html");
const DEFAULT_TAG: &str = include_str!("../../templates/tag.html");
const DEFAULT_JOURNAL: &str = include_str!("../../templates/journal.html");
const DEFAULT_BACKLINKS: &str = include_str!("../../templates/partials/backlinks.html");
const DEFAULT_NAV: &str = include_str!("../../templates/partials/nav.html");
const DEFAULT_GRAPH: &str = include_str!("../../templates/graph.html");
const DEFAULT_TAGS_INDEX: &str = include_str!("../../templates/tags-index.html");
const DEFAULT_BLOG: &str = include_str!("../../templates/blog.html");
const DEFAULT_FILES: &str = include_str!("../../templates/files.html");

pub fn setup_environment(
    custom_template_dir: Option<&Path>,
    _config: &SiteConfig,
) -> Result<Environment<'static>> {
    let mut env = Environment::new();

    // Add slugify filter for generating page URLs from names
    env.add_filter("slugify", |value: String| -> String {
        crate::parser::slugify_page_name(&value)
    });

    // Load default templates
    env.add_template("base.html", DEFAULT_BASE)?;
    env.add_template("page.html", DEFAULT_PAGE)?;
    env.add_template("index.html", DEFAULT_INDEX)?;
    env.add_template("tag.html", DEFAULT_TAG)?;
    env.add_template("journal.html", DEFAULT_JOURNAL)?;
    env.add_template("partials/backlinks.html", DEFAULT_BACKLINKS)?;
    env.add_template("partials/nav.html", DEFAULT_NAV)?;
    env.add_template("graph.html", DEFAULT_GRAPH)?;
    env.add_template("tags-index.html", DEFAULT_TAGS_INDEX)?;
    env.add_template("blog.html", DEFAULT_BLOG)?;
    env.add_template("files.html", DEFAULT_FILES)?;

    // If user has custom templates, override
    if let Some(dir) = custom_template_dir {
        if dir.exists() {
            load_custom_templates(&mut env, dir)?;
        }
    }

    Ok(env)
}

fn load_custom_templates(env: &mut Environment, dir: &Path) -> Result<()> {
    let template_files = [
        "base.html",
        "page.html",
        "index.html",
        "tag.html",
        "journal.html",
        "graph.html",
        "tags-index.html",
        "blog.html",
        "files.html",
        "partials/backlinks.html",
        "partials/nav.html",
    ];

    for name in &template_files {
        let path = dir.join(name);
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            // We need to leak the string to get a 'static lifetime for minijinja
            let leaked: &'static str = Box::leak(content.into_boxed_str());
            env.add_template(name, leaked)?;
        }
    }

    Ok(())
}
