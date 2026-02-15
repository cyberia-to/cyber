use crate::config::SiteConfig;
use crate::render::RenderedPage;
use anyhow::Result;
use std::path::Path;

pub fn generate_sitemap(
    rendered: &[RenderedPage],
    config: &SiteConfig,
    output_dir: &Path,
) -> Result<()> {
    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#,
    );

    for page in rendered {
        let url_path = if page.url_path.ends_with("/index.html") {
            page.url_path.trim_end_matches("index.html").to_string()
        } else {
            page.url_path.clone()
        };

        xml.push_str(&format!(
            "  <url>\n    <loc>{}{}</loc>\n  </url>\n",
            config.site.base_url.trim_end_matches('/'),
            url_path
        ));
    }

    xml.push_str("</urlset>\n");

    std::fs::write(output_dir.join("sitemap.xml"), xml)?;

    Ok(())
}
