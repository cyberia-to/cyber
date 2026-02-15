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

    // Step 1: Scan
    let discovered = cyber_publish::scanner::scan(&config.build.input_dir, &config.content)?;
    if !quiet {
        println!(
            "  {} Discovered {} pages, {} journals, {} assets",
            "Scan".dimmed(),
            discovered.pages.len(),
            discovered.journals.len(),
            discovered.assets.len()
        );
    }

    // Step 2: Parse
    let parsed_pages = cyber_publish::parser::parse_all(&discovered)?;
    if !quiet {
        println!("  {} Parsed {} pages", "Parse".dimmed(), parsed_pages.len());
    }

    // Step 3: Build graph
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
    let parsed_pages = cyber_publish::parser::parse_all(&discovered)?;
    let page_store = cyber_publish::graph::build_graph(parsed_pages)?;

    let public_count = page_store.public_pages(&config.content).len();
    let total_count = page_store.pages.len();
    println!(
        "  {} {}/{} pages pass public filter",
        "Pages".dimmed(),
        public_count,
        total_count
    );

    let mut broken_count = 0;
    for (page_id, links) in &page_store.forward_links {
        if !PageStore::is_page_public(&page_store.pages[page_id], &config.content) {
            continue;
        }
        for link in links {
            if !page_store.pages.contains_key(link) {
                println!("  {} {} → {}", "Broken link:".red(), page_id, link);
                broken_count += 1;
            }
        }
    }

    if broken_count == 0 {
        println!("{} No broken links found!", "OK".green().bold());
    } else {
        println!(
            "\n{} {} broken link(s) found",
            "Warning:".yellow().bold(),
            broken_count
        );
    }

    Ok(())
}
