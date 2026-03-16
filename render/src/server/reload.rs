use crate::config::SiteConfig;
use crate::parser::{ParsedPage, PageId};
use crate::render::RenderedPage;
use anyhow::Result;
use colored::Colorize;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{mpsc, Arc};
use std::time::{Duration, SystemTime};

pub const RELOAD_SCRIPT: &str = r#"<script>
(function() {
  let retries = 0;
  function connect() {
    const es = new EventSource('/__reload');
    es.onmessage = function(e) {
      if (e.data === 'reload') {
        window.location.reload();
      }
      retries = 0;
    };
    es.onerror = function() {
      es.close();
      if (retries < 120) {
        retries++;
        setTimeout(connect, 1000);
      }
    };
  }
  connect();
})();
</script>"#;

/// Cached state for incremental rebuilds.
struct BuildCache {
    /// source_path → (mtime, ParsedPage) — skip re-parsing unchanged files
    parse_cache: HashMap<PathBuf, (SystemTime, ParsedPage)>,
    /// page_id → RenderedPage — skip re-rendering unchanged pages
    render_cache: HashMap<PageId, RenderedPage>,
    /// page_id → content_md hash — detect content changes
    content_hashes: HashMap<PageId, u64>,
    /// page_id → tags hash — detect tag changes
    tag_hashes: HashMap<PageId, u64>,
    /// page_id → sorted backlink ids — detect backlink changes
    backlink_snapshots: HashMap<PageId, Vec<PageId>>,
    /// Number of pages in the last build
    last_page_count: usize,
    /// Whether the initial full build has completed
    initialized: bool,
}

impl BuildCache {
    fn new() -> Self {
        Self {
            parse_cache: HashMap::new(),
            render_cache: HashMap::new(),
            content_hashes: HashMap::new(),
            tag_hashes: HashMap::new(),
            backlink_snapshots: HashMap::new(),
            last_page_count: 0,
            initialized: false,
        }
    }
}

/// Simple hash for content change detection (not cryptographic).
fn hash_str(s: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}

/// Start a background thread that watches for file changes and rebuilds.
/// Increments `build_version` after each successful rebuild so SSE clients know to reload.
pub fn start_watch_rebuild(config: SiteConfig, build_version: Arc<AtomicU64>) {
    std::thread::spawn(move || {
        if let Err(e) = watch_and_rebuild_loop(&config, &build_version) {
            eprintln!("  {} File watcher error: {}", "Error".red(), e);
        }
    });
}

fn watch_and_rebuild_loop(config: &SiteConfig, build_version: &Arc<AtomicU64>) -> Result<()> {
    use notify::Watcher;

    // Channel now carries file paths so we know what changed
    let (tx, rx) = mpsc::channel::<Vec<PathBuf>>();

    let mut watcher =
        notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                if matches!(
                    event.kind,
                    notify::EventKind::Modify(_)
                        | notify::EventKind::Create(_)
                        | notify::EventKind::Remove(_)
                ) {
                    let _ = tx.send(event.paths);
                }
            }
        })
        .map_err(|e| anyhow::anyhow!("Failed to create file watcher: {}", e))?;

    // Watch graph directory (primary: "graph", fallback: "pages")
    let graph_dir = {
        let primary = config.build.input_dir.join("graph");
        if primary.exists() { primary } else { config.build.input_dir.join("pages") }
    };
    // Watch blog directory (primary: "blog", fallback: "journals")
    let blog_dir = {
        let primary = config.build.input_dir.join("blog");
        if primary.exists() { primary } else { config.build.input_dir.join("journals") }
    };

    if graph_dir.exists() {
        watcher.watch(&graph_dir, notify::RecursiveMode::Recursive)?;
    }
    if blog_dir.exists() {
        watcher.watch(&blog_dir, notify::RecursiveMode::Recursive)?;
    }

    // Watch subgraph repo directories for changes
    {
        let discovered = crate::scanner::scan(&config.build.input_dir, &config.content)?;
        let root_parsed = crate::parser::parse_all(&discovered)?;
        let subgraph_decls = crate::scanner::subgraph::discover_subgraphs(
            &root_parsed,
            &config.build.input_dir,
        );
        for decl in &subgraph_decls {
            if decl.repo_path.exists() {
                eprintln!("  {} Watching subgraph '{}': {}", "Watch".dimmed(), decl.name, decl.repo_path.display());
                watcher.watch(&decl.repo_path, notify::RecursiveMode::Recursive)?;
            }
        }
    }

    let mut cache = BuildCache::new();

    loop {
        if let Ok(paths) = rx.recv() {
            // Debounce: wait 300ms and collect all changed paths
            std::thread::sleep(Duration::from_millis(300));
            let mut changed: HashSet<PathBuf> = paths.into_iter().collect();
            while let Ok(more) = rx.try_recv() {
                changed.extend(more);
            }

            // Filter to only markdown files, excluding .git/target/node_modules/build
            changed.retain(|p| {
                let is_md = p.extension()
                    .map(|e| e == "md" || e == "markdown")
                    .unwrap_or(false);
                if !is_md {
                    return false;
                }
                // Skip changes inside directories that are never content
                let path_str = p.to_string_lossy();
                !path_str.contains("/.git/")
                    && !path_str.contains("/target/")
                    && !path_str.contains("/node_modules/")
                    && !path_str.contains("/build/")
                    && !path_str.contains("/.claude/")
            });

            if changed.is_empty() {
                continue;
            }

            let n = changed.len();
            eprintln!(
                "  {} {} file{} changed, rebuilding...",
                "Watch".yellow(),
                n,
                if n == 1 { "" } else { "s" }
            );
            let start = std::time::Instant::now();

            match incremental_rebuild(config, &mut cache, &changed) {
                Ok((rendered_count, dirty_count)) => {
                    let elapsed = start.elapsed();
                    build_version.fetch_add(1, Ordering::SeqCst);
                    eprintln!(
                        "  {} Rebuilt {}/{} pages in {:.2}s",
                        "Done".green(),
                        dirty_count,
                        rendered_count,
                        elapsed.as_secs_f64()
                    );
                }
                Err(e) => {
                    eprintln!("  {} Rebuild failed: {}", "Error".red(), e);
                }
            }
        }
    }
}

/// Incremental rebuild: selective parse → full graph → selective render → incremental output.
/// Returns (total_rendered, dirty_count).
fn incremental_rebuild(
    config: &SiteConfig,
    cache: &mut BuildCache,
    changed_paths: &HashSet<PathBuf>,
) -> Result<(usize, usize)> {
    // Step 1: Scan (always full — it's fast)
    let discovered = crate::scanner::scan(&config.build.input_dir, &config.content)?;

    // Step 2: Selective parse — only re-parse files whose mtime changed
    let mut all_parsed: Vec<ParsedPage> = Vec::new();
    let mut changed_page_ids: HashSet<PageId> = HashSet::new();

    for file in discovered.pages.iter().chain(discovered.journals.iter()) {
        let mtime = std::fs::metadata(&file.path)
            .ok()
            .and_then(|m| m.modified().ok())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        if let Some((cached_mtime, cached_page)) = cache.parse_cache.get(&file.path) {
            if *cached_mtime == mtime && !changed_paths.contains(&file.path) {
                // File unchanged — use cached parse
                all_parsed.push(cached_page.clone());
                continue;
            }
        }

        // Parse (new or changed file)
        let page = crate::parser::parse_file(file)?;
        changed_page_ids.insert(page.id.clone());
        cache.parse_cache.insert(file.path.clone(), (mtime, page.clone()));
        all_parsed.push(page);
    }

    // Non-markdown files: always re-parse (they're few and cheap)
    let non_md = crate::parser::parse_all(&crate::scanner::DiscoveredFiles {
        pages: vec![],
        journals: vec![],
        media: discovered.media.clone(),
        files: discovered.files.clone(),
    })?;
    all_parsed.extend(non_md);

    // Remove stale cache entries for deleted files
    let current_paths: HashSet<&PathBuf> = discovered
        .pages
        .iter()
        .chain(discovered.journals.iter())
        .map(|f| &f.path)
        .collect();
    cache.parse_cache.retain(|path, _| current_paths.contains(path));

    // Step 2b: Discover and merge subgraphs (same logic as build_site)
    let subgraph_decls = crate::scanner::subgraph::discover_subgraphs(
        &all_parsed,
        &config.build.input_dir,
    );
    if !subgraph_decls.is_empty() {
        let subgraph_namespaces: Vec<String> =
            subgraph_decls.iter().map(|d| d.name.clone()).collect();
        let _evicted = crate::scanner::subgraph::enforce_namespace_monopoly(
            &mut all_parsed,
            &subgraph_namespaces,
        );
        for decl in &subgraph_decls {
            let subgraph_files = crate::scanner::subgraph::scan_subgraph(decl)?;
            let declaring_page = all_parsed
                .iter()
                .find(|p| p.id == decl.declaring_page_id)
                .cloned();
            for file in &subgraph_files {
                if file.kind == crate::scanner::FileKind::Page {
                    let mut page = crate::parser::parse_file(file)?;
                    if page.id == decl.declaring_page_id {
                        if let Some(ref dp) = declaring_page {
                            page.meta.tags = dp.meta.tags.clone();
                            page.meta.aliases = dp.meta.aliases.clone();
                            page.meta.properties = dp.meta.properties.clone();
                            page.meta.public = dp.meta.public;
                            page.meta.icon = dp.meta.icon.clone();
                            page.meta.stake = dp.meta.stake;
                            if !dp.content_md.trim().is_empty() {
                                let readme_content = std::mem::take(&mut page.content_md);
                                page.content_md = dp.content_md.clone();
                                page.content_md.push_str(&format!(
                                    "\n\n---\n\n## from subgraph {}\n\n",
                                    decl.name
                                ));
                                page.content_md.push_str(&readme_content);
                            }
                            for link in &dp.outgoing_links {
                                if !page.outgoing_links.contains(link) {
                                    page.outgoing_links.push(link.clone());
                                }
                            }
                        }
                    }
                    all_parsed.push(page);
                }
            }
            if declaring_page.is_some() {
                all_parsed.retain(|p| {
                    !(p.id == decl.declaring_page_id && p.subgraph.is_none())
                });
            }
        }
    }

    // Step 3: Build graph (always full — it's fast)
    let store = crate::graph::build_graph(all_parsed)?;

    // Step 4: Determine dirty pages (need re-rendering)
    let mut dirty_ids: HashSet<PageId> = changed_page_ids.clone();

    // Pages whose content_md hash changed
    for (page_id, page) in &store.pages {
        let new_hash = hash_str(&page.content_md);
        if let Some(&old_hash) = cache.content_hashes.get(page_id) {
            if old_hash != new_hash {
                dirty_ids.insert(page_id.clone());
            }
        } else {
            // New page
            dirty_ids.insert(page_id.clone());
        }
        cache.content_hashes.insert(page_id.clone(), new_hash);
    }

    // Pages whose backlinks changed
    for (page_id, backlinks) in &store.backlinks {
        let mut sorted = backlinks.clone();
        sorted.sort();
        if let Some(old) = cache.backlink_snapshots.get(page_id) {
            if *old != sorted {
                dirty_ids.insert(page_id.clone());
            }
        }
        cache.backlink_snapshots.insert(page_id.clone(), sorted);
    }

    // Also dirty: pages whose backlinks section shows the dirty page
    let mut extra_dirty: HashSet<PageId> = HashSet::new();
    for dirty_id in &dirty_ids {
        if let Some(bl) = store.backlinks.get(dirty_id) {
            extra_dirty.extend(bl.iter().cloned());
        }
    }
    dirty_ids.extend(extra_dirty);

    // Detect structural changes (pages added/removed, tags changed) — these
    // require re-rendering synthetic pages (index, tags, blog, files, etc.).
    let structural_change = if !cache.initialized {
        true
    } else {
        // Page count changed?
        if cache.last_page_count != store.pages.len() {
            true
        } else {
            // Any dirty page had its tags changed?
            let mut tags_changed = false;
            for dirty_id in &dirty_ids {
                if let Some(page) = store.pages.get(dirty_id) {
                    let new_tag_hash = hash_str(&page.meta.tags.join(","));
                    if let Some(&old) = cache.tag_hashes.get(dirty_id) {
                        if old != new_tag_hash {
                            tags_changed = true;
                        }
                    }
                    cache.tag_hashes.insert(dirty_id.clone(), new_tag_hash);
                }
            }
            tags_changed
        }
    };
    cache.last_page_count = store.pages.len();

    // Mark synthetic pages dirty only for structural changes
    if structural_change {
        dirty_ids.insert("__structural__".to_string());
    }

    let dirty_count = dirty_ids.len();

    // Step 5: Selective render
    let dirty_ref = if cache.initialized {
        Some(&dirty_ids)
    } else {
        None // First rebuild after watcher starts: render everything
    };
    let rendered = crate::render::render_cached(
        &store,
        config,
        &mut cache.render_cache,
        dirty_ref,
    )?;
    let total = rendered.len();

    // Step 6: Output — write only what changed
    if !cache.initialized {
        // First build: full output
        crate::output::write_output(&rendered, &store, config, &discovered)?;
        cache.initialized = true;
    } else if structural_change {
        // Structural change: write all pages + regenerate indexes
        crate::output::write_incremental(&rendered, &store, config, &discovered)?;
    } else {
        // Content-only change: write just the dirty pages
        crate::output::write_dirty_pages(&rendered, &dirty_ids, config)?;
    }

    Ok((total, dirty_count))
}
