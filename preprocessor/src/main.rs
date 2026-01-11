use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;

mod config;
mod content;
mod favorites;
mod frontmatter;
mod journals;
mod page;
mod query;

#[cfg(test)]
mod tests;

use config::Config;

#[derive(Parser, Debug)]
#[command(name = "logseq-to-quartz")]
#[command(about = "Fast Logseq to Quartz preprocessor")]
struct Cli {
    /// Path to Logseq graph root (contains pages/, journals/, logseq/)
    #[arg(short, long, default_value = ".")]
    input: PathBuf,

    /// Output directory for Quartz content
    #[arg(short, long, default_value = "quartz-content")]
    output: PathBuf,

    /// Include private pages (private:: true)
    #[arg(long, default_value_t = false)]
    include_private: bool,

    /// Create stub pages for missing links
    #[arg(long, default_value_t = false)]
    create_stubs: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let start = Instant::now();

    let config = Config {
        input_dir: cli.input,
        output_dir: cli.output,
        include_private: cli.include_private,
        create_stubs: cli.create_stubs,
        verbose: cli.verbose,
    };

    println!("Preprocessing Logseq content for Quartz...\n");

    // Run the preprocessor
    let stats = run_preprocessor(&config)?;

    let duration = start.elapsed();
    println!("\nPreprocessing complete!");
    println!("  Pages: {} published, {} skipped", stats.pages_published, stats.pages_skipped);
    println!("  Journals: {}", stats.journals_published);
    println!("  Favorites: {}", stats.favorites_created);
    println!("  Stubs: {}", stats.stubs_created);
    println!("  Time: {:.2}s", duration.as_secs_f64());

    Ok(())
}

#[derive(Default)]
pub struct Stats {
    pub pages_published: usize,
    pub pages_skipped: usize,
    pub journals_published: usize,
    pub favorites_created: usize,
    pub stubs_created: usize,
}

fn run_preprocessor(config: &Config) -> Result<Stats> {
    use rayon::prelude::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::fs;

    let mut stats = Stats::default();

    // Create output directories
    let pages_output = config.output_dir.join("pages");
    let journals_output = config.output_dir.join("journals");
    let favorites_output = config.output_dir.join("favorites");
    let assets_output = config.output_dir.join("assets");

    fs::create_dir_all(&pages_output)?;
    fs::create_dir_all(&journals_output)?;
    fs::create_dir_all(&favorites_output)?;
    fs::create_dir_all(&assets_output)?;

    // Step 1: Get all git dates in one batch call
    let repo_root = &config.input_dir;
    let git_dates = page::get_all_git_dates(repo_root);

    // Step 2: Build page index for queries
    println!("Building page index...");
    let pages_dir = config.input_dir.join("pages");
    let page_index = page::build_index(&pages_dir)?;
    println!("Indexed {} pages", page_index.len());

    // Step 3: Process pages in parallel
    println!("\nProcessing pages...");
    let published = AtomicUsize::new(0);
    let skipped = AtomicUsize::new(0);

    let page_files: Vec<_> = walkdir::WalkDir::new(&pages_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "md"))
        .collect();

    page_files.par_iter().for_each(|entry| {
        match page::process_page(entry.path(), &pages_output, &page_index, config, &git_dates, repo_root) {
            Ok(true) => { published.fetch_add(1, Ordering::Relaxed); }
            Ok(false) => { skipped.fetch_add(1, Ordering::Relaxed); }
            Err(e) => {
                if config.verbose {
                    eprintln!("Error processing {:?}: {}", entry.path(), e);
                }
            }
        }
    });

    stats.pages_published = published.load(Ordering::Relaxed);
    stats.pages_skipped = skipped.load(Ordering::Relaxed);
    println!("Published: {} files, Skipped: {} files", stats.pages_published, stats.pages_skipped);

    // Step 4: Process journals
    println!("\nProcessing journals...");
    let journals_dir = config.input_dir.join("journals");
    if journals_dir.exists() {
        stats.journals_published = journals::process_journals(&journals_dir, &journals_output, &page_index, config)?;
        println!("Published: {} journal entries", stats.journals_published);
    }

    // Step 5: Process favorites
    println!("\nProcessing favorites...");
    let config_path = config.input_dir.join("logseq/config.edn");
    if config_path.exists() {
        stats.favorites_created = favorites::process_favorites(&config_path, &favorites_output, &pages_output)?;
        println!("Created: {} favorite pages", stats.favorites_created);
    }

    // Step 6: Create index.md
    let index_path = config.output_dir.join("index.md");
    if !index_path.exists() {
        fs::write(&index_path, "---\ntitle: \"Cyber\"\n---\n\n![[pages/cyber]]\n")?;
        println!("\nCreated index.md");
    }

    // Step 7: Copy assets
    let assets_source = config.input_dir.join("assets");
    if assets_source.exists() {
        let count = copy_dir_recursive(&assets_source, &assets_output)?;
        println!("\nCopied {} asset files", count);
    }

    // Step 8: Create stub pages for missing links
    if config.create_stubs {
        println!("\nCreating stub pages...");
        stats.stubs_created = page::create_stubs(&config.output_dir, &page_index)?;
        println!("Created {} stub pages", stats.stubs_created);
    }

    Ok(stats)
}

fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> Result<usize> {
    use std::fs;
    let mut count = 0;

    for entry in walkdir::WalkDir::new(src) {
        let entry = entry?;
        let relative = entry.path().strip_prefix(src)?;
        let target = dst.join(relative);

        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(entry.path(), &target)?;
            count += 1;
        }
    }

    Ok(count)
}
