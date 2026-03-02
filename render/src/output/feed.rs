use crate::config::SiteConfig;
use crate::graph::PageStore;
use anyhow::Result;
use std::path::Path;

pub fn generate_feed(store: &PageStore, config: &SiteConfig, output_dir: &Path) -> Result<()> {
    let mut channel = rss::ChannelBuilder::default();

    let title = config
        .feeds
        .title
        .clone()
        .unwrap_or_else(|| config.site.title.clone());

    channel
        .title(title)
        .link(config.site.base_url.clone())
        .description(config.site.description.clone())
        .language(Some(config.site.language.clone()));

    let recent = store.recent_pages(config.feeds.items, &config.content);

    let items: Vec<rss::Item> = recent
        .iter()
        .map(|page| {
            let mut item = rss::Item::default();
            item.set_title(page.meta.title.clone());
            item.set_link(format!("{}/{}", config.site.base_url, page.id));

            if let Some(date) = page.meta.date {
                item.set_pub_date(
                    date.and_hms_opt(0, 0, 0)
                        .map(|dt| dt.and_utc().to_rfc2822())
                        .unwrap_or_default(),
                );
            }

            let desc = crate::render::context::generate_excerpt(&page.content_md, 200);
            item.set_description(desc);

            item
        })
        .collect();

    channel.items(items);

    let channel = channel.build();
    let rss_xml = channel.to_string();

    std::fs::write(output_dir.join("feed.xml"), rss_xml)?;

    Ok(())
}
