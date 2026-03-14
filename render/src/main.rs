use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::path::{Path, PathBuf};

use cyber_publish::config::SiteConfig;
use cyber_publish::graph::PageStore;

#[derive(Parser)]
#[command(name = "cyber-publish")]
#[command(
    version,
    about = "A Rust-native static site publisher for the cyber knowledge graph"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to config file
    #[arg(short, long, default_value = "publish.toml")]
    config: PathBuf,

    /// Increase verbosity (-v info, -vv debug, -vvv trace)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Suppress output
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Build the static site
    Build {
        /// Logseq graph directory
        #[arg(default_value = ".")]
        input: PathBuf,

        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Include non-public pages
        #[arg(long)]
        drafts: bool,

        /// Override base URL
        #[arg(long)]
        base_url: Option<String>,
    },

    /// Build and serve with live reload
    Serve {
        /// Logseq graph directory
        #[arg(default_value = ".")]
        input: PathBuf,

        /// Server port (overrides base_url port from config)
        #[arg(short, long)]
        port: Option<u16>,

        /// Bind address
        #[arg(short, long, default_value = "127.0.0.1")]
        bind: String,

        /// Disable live reload
        #[arg(long)]
        no_reload: bool,

        /// Open browser automatically
        #[arg(long)]
        open: bool,

        /// Include non-public pages
        #[arg(long)]
        drafts: bool,
    },

    /// Initialize a new publish.toml config
    Init {
        /// Directory to initialize in
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Validate graph and report broken links
    Check {
        /// Logseq graph directory
        #[arg(default_value = ".")]
        input: PathBuf,
    },
}

/// Try to extract port from a URL like "http://localhost:8888"
fn port_from_url(url: &str) -> Option<u16> {
    // Look for :PORT at the end
    if let Some(pos) = url.rfind(':') {
        let after_colon = &url[pos + 1..];
        // Strip trailing slash
        let port_str = after_colon.trim_end_matches('/');
        port_str.parse::<u16>().ok()
    } else {
        None
    }
}

fn resolve_config(cli_config: &PathBuf, input: &Path) -> (PathBuf, SiteConfig) {
    let config_path = if cli_config.is_relative() {
        input.join(cli_config)
    } else {
        cli_config.clone()
    };
    let mut config = SiteConfig::load(&config_path).unwrap_or_default();
    config.build.input_dir = input.to_path_buf();

    // Resolve output_dir relative to input_dir if it's relative
    if config.build.output_dir.is_relative() {
        config.build.output_dir = input.join(&config.build.output_dir);
    }

    (config_path, config)
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build {
            input,
            output,
            drafts,
            base_url,
        } => {
            let (_config_path, mut config) = resolve_config(&cli.config, &input);
            if let Some(ref out) = output {
                config.build.output_dir = out.clone();
            }
            if let Some(ref url) = base_url {
                config.site.base_url = url.clone();
            }
            if drafts {
                config.content.public_only = false;
            }

            build_site(&config, cli.quiet)?;
        }
        Commands::Serve {
            input,
            port,
            bind,
            no_reload,
            open,
            drafts,
        } => {
            let (_config_path, mut config) = resolve_config(&cli.config, &input);

            // Resolve port: CLI flag > config base_url > default 8080
            let port = port
                .or_else(|| port_from_url(&config.site.base_url))
                .unwrap_or(8080);

            config.site.base_url = format!("http://{}:{}", bind, port);

            if drafts {
                config.content.public_only = false;
            }

            build_site(&config, cli.quiet)?;

            cyber_publish::server::serve(&config, &bind, port, !no_reload, open)?;
        }
        Commands::Init { path } => {
            std::fs::create_dir_all(&path)?;
            let config_path = path.join("publish.toml");
            if config_path.exists() {
                eprintln!("{} publish.toml already exists", "Warning:".yellow());
                return Ok(());
            }
            let default_config = include_str!("../default-config.toml");
            std::fs::write(&config_path, default_config)?;
            if !cli.quiet {
                println!(
                    "{} Created {}",
                    "Done!".green().bold(),
                    config_path.display()
                );
            }
        }
        Commands::Check { input } => {
            let (_config_path, config) = resolve_config(&cli.config, &input);
            check_site(&config)?;
        }
    }

    Ok(())
}

fn build_site(config: &SiteConfig, quiet: bool) -> Result<()> {
    let start = std::time::Instant::now();

    if !quiet {
        println!("{} {}", "Building".cyan().bold(), config.site.title);
    }

    // Step 1: Scan root graph
    let discovered = cyber_publish::scanner::scan(&config.build.input_dir, &config.content)?;
    if !quiet {
        println!(
            "  {} Discovered {} pages, {} journals, {} media, {} files",
            "Scan".dimmed(),
            discovered.pages.len(),
            discovered.journals.len(),
            discovered.media.len(),
            discovered.files.len()
        );
    }

    // Step 2: Parse root graph
    let mut parsed_pages = cyber_publish::parser::parse_all(&discovered)?;
    if !quiet {
        println!("  {} Parsed {} pages", "Parse".dimmed(), parsed_pages.len());
    }

    // Step 3: Discover and scan subgraphs
    let subgraph_decls = cyber_publish::scanner::subgraph::discover_subgraphs(
        &parsed_pages,
        &config.build.input_dir,
    );

    if !subgraph_decls.is_empty() {
        let subgraph_namespaces: Vec<String> =
            subgraph_decls.iter().map(|d| d.name.clone()).collect();

        // Enforce namespace monopoly on root pages
        let evicted = cyber_publish::scanner::subgraph::enforce_namespace_monopoly(
            &mut parsed_pages,
            &subgraph_namespaces,
        );
        if !quiet && !evicted.is_empty() {
            for (id, reason) in &evicted {
                println!("  {} Evicted '{}': {}", "Monopoly".yellow(), id, reason);
            }
        }

        // Scan and parse each subgraph
        for decl in &subgraph_decls {
            let subgraph_files = cyber_publish::scanner::subgraph::scan_subgraph(decl)?;
            let sg_page_count = subgraph_files.iter().filter(|f| f.kind == cyber_publish::scanner::FileKind::Page).count();
            let sg_file_count = subgraph_files.iter().filter(|f| f.kind == cyber_publish::scanner::FileKind::File).count();

            // Find the declaring page so we can merge its metadata into the README
            let declaring_page = parsed_pages
                .iter()
                .find(|p| p.id == decl.declaring_page_id)
                .cloned();

            for file in &subgraph_files {
                let mut page = if file.kind == cyber_publish::scanner::FileKind::Page {
                    cyber_publish::parser::parse_file(file)?
                } else {
                    continue; // non-md handled below
                };

                // If this is the repo-root README (id == subgraph name == declaring page id),
                // merge the declaring page's metadata so tags/aliases/stake etc. are preserved.
                if page.id == decl.declaring_page_id {
                    if let Some(ref decl_page) = declaring_page {
                        page.meta.tags = decl_page.meta.tags.clone();
                        page.meta.aliases = decl_page.meta.aliases.clone();
                        page.meta.properties = decl_page.meta.properties.clone();
                        page.meta.public = decl_page.meta.public;
                        page.meta.icon = decl_page.meta.icon.clone();
                        page.meta.stake = decl_page.meta.stake;
                    }
                }

                parsed_pages.push(page);
            }

            // Remove the declaring page from root — its slot is now taken by the README
            if declaring_page.is_some() {
                parsed_pages.retain(|p| {
                    !(p.id == decl.declaring_page_id && p.subgraph.is_none())
                });
            }

            // Parse non-markdown files via a temporary DiscoveredFiles
            let sg_files: Vec<_> = subgraph_files
                .into_iter()
                .filter(|f| f.kind == cyber_publish::scanner::FileKind::File)
                .collect();
            let sg_discovered = cyber_publish::scanner::DiscoveredFiles {
                pages: Vec::new(),
                journals: Vec::new(),
                media: Vec::new(),
                files: sg_files,
            };
            let sg_file_pages = cyber_publish::parser::parse_all(&sg_discovered)?;
            parsed_pages.extend(sg_file_pages);

            if !quiet {
                println!(
                    "  {} Subgraph '{}': {} pages, {} files",
                    "Scan".dimmed(),
                    decl.name,
                    sg_page_count,
                    sg_file_count
                );
            }
        }
    }

    // Step 4: Build graph
    let page_store = cyber_publish::graph::build_graph(parsed_pages)?;
    if !quiet {
        let total_links: usize = page_store.forward_links.values().map(|v| v.len()).sum();
        let public_count = page_store.public_pages(&config.content).len();
        let total_count = page_store.pages.len();
        println!(
            "  {} Built graph with {} links",
            "Graph".dimmed(),
            total_links
        );

        if config.content.public_only && public_count < total_count {
            println!(
                "  {} {}/{} pages are public (set default_public = true or add public:: true to pages)",
                "Filter".yellow(),
                public_count,
                total_count
            );
        }
    }

    // Step 4: Render
    let rendered = cyber_publish::render::render_all(&page_store, config)?;
    if !quiet {
        println!("  {} Rendered {} pages", "Render".dimmed(), rendered.len());
    }

    // Step 5: Output
    cyber_publish::output::write_output(&rendered, &page_store, config, &discovered)?;

    let elapsed = start.elapsed();
    if !quiet {
        println!(
            "{} Built in {:.2}s → {}",
            "Done!".green().bold(),
            elapsed.as_secs_f64(),
            config.build.output_dir.display()
        );
    }

    Ok(())
}

fn check_site(config: &SiteConfig) -> Result<()> {
    println!("{} {}", "Checking".cyan().bold(), config.site.title);

    let discovered = cyber_publish::scanner::scan(&config.build.input_dir, &config.content)?;
    let mut parsed_pages = cyber_publish::parser::parse_all(&discovered)?;

    // Discover and scan subgraphs (same as build_site)
    let subgraph_decls = cyber_publish::scanner::subgraph::discover_subgraphs(
        &parsed_pages,
        &config.build.input_dir,
    );
    if !subgraph_decls.is_empty() {
        let subgraph_namespaces: Vec<String> =
            subgraph_decls.iter().map(|d| d.name.clone()).collect();
        let evicted = cyber_publish::scanner::subgraph::enforce_namespace_monopoly(
            &mut parsed_pages,
            &subgraph_namespaces,
        );
        for (id, reason) in &evicted {
            println!("  {} Evicted '{}': {}", "Monopoly".yellow(), id, reason);
        }
        for decl in &subgraph_decls {
            let subgraph_files = cyber_publish::scanner::subgraph::scan_subgraph(decl)?;
            let declaring_page = parsed_pages
                .iter()
                .find(|p| p.id == decl.declaring_page_id)
                .cloned();
            for file in &subgraph_files {
                if file.kind == cyber_publish::scanner::FileKind::Page {
                    let mut page = cyber_publish::parser::parse_file(file)?;
                    if page.id == decl.declaring_page_id {
                        if let Some(ref dp) = declaring_page {
                            page.meta.tags = dp.meta.tags.clone();
                            page.meta.aliases = dp.meta.aliases.clone();
                            page.meta.properties = dp.meta.properties.clone();
                            page.meta.public = dp.meta.public;
                            page.meta.icon = dp.meta.icon.clone();
                            page.meta.stake = dp.meta.stake;
                        }
                    }
                    parsed_pages.push(page);
                }
            }
            if declaring_page.is_some() {
                parsed_pages.retain(|p| {
                    !(p.id == decl.declaring_page_id && p.subgraph.is_none())
                });
            }
            let sg_files: Vec<_> = subgraph_files
                .into_iter()
                .filter(|f| f.kind == cyber_publish::scanner::FileKind::File)
                .collect();
            let sg_discovered = cyber_publish::scanner::DiscoveredFiles {
                pages: Vec::new(),
                journals: Vec::new(),
                media: Vec::new(),
                files: sg_files,
            };
            parsed_pages.extend(cyber_publish::parser::parse_all(&sg_discovered)?);
            println!(
                "  {} Subgraph '{}' scanned",
                "Scan".dimmed(),
                decl.name
            );
        }
    }

    let page_store = cyber_publish::graph::build_graph(parsed_pages)?;

    let public_count = page_store.public_pages(&config.content).len();
    let total_count = page_store.pages.len();
    println!(
        "  {} {}/{} pages pass public filter",
        "Pages".dimmed(),
        public_count,
        total_count
    );

    // Broken links grouped by subgraph
    let mut broken_by_subgraph: std::collections::HashMap<String, Vec<(String, String)>> =
        std::collections::HashMap::new();

    for (page_id, links) in &page_store.forward_links {
        if !PageStore::is_page_public(&page_store.pages[page_id], &config.content) {
            continue;
        }
        let subgraph_name = page_store
            .subgraph_pages
            .iter()
            .find(|(_, ids)| ids.contains(page_id))
            .map(|(name, _)| name.clone())
            .unwrap_or_else(|| "root".to_string());

        for link in links {
            if !page_store.pages.contains_key(link) {
                broken_by_subgraph
                    .entry(subgraph_name.clone())
                    .or_default()
                    .push((page_id.clone(), link.clone()));
            }
        }
    }

    let total_broken: usize = broken_by_subgraph.values().map(|v| v.len()).sum();
    if total_broken == 0 {
        println!("{} No broken links found!", "OK".green().bold());
    } else {
        for (sg, broken) in &broken_by_subgraph {
            println!(
                "\n  {} [{}] {} broken link(s):",
                "Broken:".red(),
                sg,
                broken.len()
            );
            for (from, to) in broken {
                println!("    {} → {}", from, to);
            }
        }
        println!(
            "\n{} {} broken link(s) found",
            "Warning:".yellow().bold(),
            total_broken
        );
    }

    // Crystal metadata validation
    let mut crystal_warnings = 0;
    for (page_id, page) in &page_store.pages {
        if page_store.stub_pages.contains(page_id) {
            continue;
        }
        // Skip crystal validation for subgraph pages
        if page.subgraph.is_some() {
            continue;
        }
        for warn in cyber_publish::validator::validate_page(page) {
            println!(
                "  {} {} — {}",
                "Invalid:".red(),
                warn.source_path.display(),
                warn.message
            );
            crystal_warnings += 1;
        }
    }

    if crystal_warnings == 0 {
        println!(
            "{} Crystal metadata valid on all pages!",
            "OK".green().bold()
        );
    } else {
        println!(
            "\n{} {} crystal metadata warning(s) found",
            "Warning:".yellow().bold(),
            crystal_warnings
        );
    }

    Ok(())
}
