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
    /// source_path → DiscoveredFile — needed by fast path to re-parse without scanning
    file_cache: HashMap<PathBuf, crate::scanner::DiscoveredFile>,
    /// page_id → RenderedPage — skip re-rendering unchanged pages
    render_cache: HashMap<PageId, RenderedPage>,
    /// page_id → content_md hash — detect content changes
    content_hashes: HashMap<PageId, u64>,
    /// page_id → tags hash — detect tag changes
    tag_hashes: HashMap<PageId, u64>,
    /// page_id → meta hash (title + aliases + icon + stake + tags) — detect frontmatter changes
    meta_hashes: HashMap<PageId, u64>,
    /// page_id → outgoing links hash — detect link changes
    link_hashes: HashMap<PageId, u64>,
    /// page_id → sorted backlink ids — detect backlink changes
    backlink_snapshots: HashMap<PageId, Vec<PageId>>,
    /// Content page IDs from the last build (excludes stubs) — detect structural changes
    last_content_page_ids: HashSet<PageId>,
    /// Namespace tree keys from last build — detect namespace parent changes
    last_namespace_keys: HashSet<String>,
    /// Whether the initial full build has completed
    initialized: bool,
    /// Cached subgraph parsed pages (rebuilt only when a subgraph file changes)
    subgraph_pages: Vec<crate::parser::ParsedPage>,
    /// Subgraph repo paths for change detection
    subgraph_repo_paths: Vec<PathBuf>,
    /// Cached graph store — reused when no structural change (avoids expensive PageRank/gravity)
    cached_store: Option<crate::graph::PageStore>,
}

impl BuildCache {
    fn new() -> Self {
        Self {
            parse_cache: HashMap::new(),
            file_cache: HashMap::new(),
            render_cache: HashMap::new(),
            content_hashes: HashMap::new(),
            tag_hashes: HashMap::new(),
            meta_hashes: HashMap::new(),
            link_hashes: HashMap::new(),
            backlink_snapshots: HashMap::new(),
            last_content_page_ids: HashSet::new(),
            last_namespace_keys: HashSet::new(),
            initialized: false,
            subgraph_pages: Vec::new(),
            subgraph_repo_paths: Vec::new(),
            cached_store: None,
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

/// Hash frontmatter metadata fields that affect other pages (title, aliases, icon, stake, tags).
fn hash_meta(page: &ParsedPage) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    page.meta.title.hash(&mut h);
    let mut aliases = page.meta.aliases.clone();
    aliases.sort();
    aliases.join(",").hash(&mut h);
    page.meta.icon.hash(&mut h);
    page.meta.stake.hash(&mut h);
    let mut tags = page.meta.tags.clone();
    tags.sort();
    tags.join(",").hash(&mut h);
    h.finish()
}

/// Hash outgoing links to detect when a page's link set changes (affects backlinks on targets).
fn hash_links(page: &ParsedPage) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    let mut sorted_links = page.outgoing_links.clone();
    sorted_links.sort();
    sorted_links.hash(&mut h);
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

    // Watch graph directory (primary: "root", fallback: "graph", then "pages")
    let graph_dir = {
        let root = config.build.input_dir.join("root");
        let graph = config.build.input_dir.join("graph");
        if root.exists() { root } else if graph.exists() { graph } else { config.build.input_dir.join("pages") }
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

    // Warm up: pre-populate subgraph cache so first incremental rebuild is fast.
    // Without this, the first file change triggers a full re-render of all 12K pages.
    {
        let discovered = crate::scanner::scan(&config.build.input_dir, &config.content)?;
        let root_parsed = crate::parser::parse_all(&discovered)?;
        let subgraph_decls = crate::scanner::subgraph::discover_subgraphs(
            &root_parsed,
            &config.build.input_dir,
        );
        for decl in &subgraph_decls {
            cache.subgraph_repo_paths.push(decl.repo_path.clone());
            let subgraph_files = crate::scanner::subgraph::scan_subgraph(decl)?;
            let declaring_page = root_parsed
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
                    cache.subgraph_pages.push(page);
                }
            }
        }
        // Pre-populate parse cache and file cache for root graph files
        for file in discovered.pages.iter().chain(discovered.journals.iter()) {
            let mtime = std::fs::metadata(&file.path)
                .ok()
                .and_then(|m| m.modified().ok())
                .unwrap_or(SystemTime::UNIX_EPOCH);
            cache.file_cache.insert(file.path.clone(), file.clone());
            if let Ok(page) = crate::parser::parse_file(file) {
                cache.parse_cache.insert(file.path.clone(), (mtime, page));
            }
        }
        // Build the full graph to snapshot page IDs (including stubs) and hashes.
        // This matches what incremental_rebuild produces, preventing false structural changes.
        {
            let mut warmup_pages: Vec<ParsedPage> = cache.parse_cache.values()
                .map(|(_, p)| p.clone())
                .collect();
            // Add non-md files
            let non_md = crate::parser::parse_all(&crate::scanner::DiscoveredFiles {
                pages: vec![],
                journals: vec![],
                media: discovered.media.clone(),
                files: discovered.files.clone(),
            })?;
            warmup_pages.extend(non_md);
            // Add subgraph pages and enforce namespace monopoly
            let subgraph_namespaces: Vec<String> =
                subgraph_decls.iter().map(|d| d.name.clone()).collect();
            crate::scanner::subgraph::enforce_namespace_monopoly(
                &mut warmup_pages,
                &subgraph_namespaces,
            );
            for decl in &subgraph_decls {
                warmup_pages.retain(|p| {
                    !(p.id == decl.declaring_page_id && p.subgraph.is_none())
                });
            }
            warmup_pages.extend(cache.subgraph_pages.clone());
            // Snapshot content page IDs and hashes BEFORE graph build.
            // This must match what incremental_rebuild hashes from all_parsed (pre-graph).
            for page in &warmup_pages {
                cache.last_content_page_ids.insert(page.id.clone());
                cache.content_hashes.insert(page.id.clone(), hash_str(&page.content_md));
                cache.tag_hashes.insert(page.id.clone(), hash_str(&page.meta.tags.join(",")));
                cache.meta_hashes.insert(page.id.clone(), hash_meta(page));
                cache.link_hashes.insert(page.id.clone(), hash_links(page));
            }
            // Build graph to get full store (with stubs, links, PageRank, etc.)
            let store = crate::graph::build_graph(warmup_pages)?;
            // Snapshot backlinks
            for (page_id, backlinks) in &store.backlinks {
                let mut sorted = backlinks.clone();
                sorted.sort();
                cache.backlink_snapshots.insert(page_id.clone(), sorted);
            }
            // Snapshot namespace tree keys
            cache.last_namespace_keys = store.namespace_tree.keys().cloned().collect();
            // Cache the store so fast path can reuse it
            cache.cached_store = Some(store);
        }

        cache.initialized = true;
    }

    loop {
        if let Ok(paths) = rx.recv() {
            // Debounce: wait 100ms and collect all changed paths
            std::thread::sleep(Duration::from_millis(100));
            let mut changed: HashSet<PathBuf> = paths.into_iter().collect();
            while let Ok(more) = rx.try_recv() {
                changed.extend(more);
            }

            // Filter to content files, excluding .git/target/node_modules/build.
            // Accept any file (not just .md) — the scanner handles file type classification.
            changed.retain(|p| {
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
            let start = std::time::Instant::now();

            // Try fast path first (skips filesystem scan for content-only edits)
            if let Some(result) = try_fast_path(config, &mut cache, &changed) {
                match result {
                    Ok((rendered_count, dirty_count)) => {
                        let elapsed = start.elapsed();
                        build_version.fetch_add(1, Ordering::SeqCst);
                        eprintln!(
                            "  {} {} file{} → fast rebuild {}/{} pages in {:.2}s",
                            "Done".green(),
                            n,
                            if n == 1 { "" } else { "s" },
                            dirty_count,
                            rendered_count,
                            elapsed.as_secs_f64()
                        );
                    }
                    Err(e) => {
                        eprintln!("  {} Fast rebuild failed: {}", "Error".red(), e);
                    }
                }
                continue;
            }

            // Full incremental rebuild (with scan)
            eprintln!(
                "  {} {} file{} changed, rebuilding...",
                "Watch".yellow(),
                n,
                if n == 1 { "" } else { "s" }
            );

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

/// Fast path for content-only edits: skip the full filesystem scan entirely.
/// Returns Some((total, dirty)) on success, None if the fast path doesn't apply.
fn try_fast_path(
    config: &SiteConfig,
    cache: &mut BuildCache,
    changed_paths: &HashSet<PathBuf>,
) -> Option<Result<(usize, usize)>> {
    // Preconditions: cache must be initialized with a cached store
    if !cache.initialized || cache.cached_store.is_none() {
        return None;
    }

    // All changed paths must be known .md files already in our caches (not new, not deleted)
    for path in changed_paths {
        if !path.exists() {
            return None; // File was deleted — need full scan
        }
        if !cache.file_cache.contains_key(path) {
            return None; // Unknown file (new file?) — need full scan
        }
        if !cache.parse_cache.contains_key(path) {
            return None; // Not previously parsed — need full scan
        }
    }

    // No subgraph files
    if changed_paths.iter().any(|p| {
        cache.subgraph_repo_paths.iter().any(|repo| p.starts_with(repo))
    }) {
        return None;
    }

    // Re-parse only the changed files
    let mut dirty_ids: HashSet<PageId> = HashSet::new();
    let mut meta_changed = false;
    let mut links_changed = false;

    for path in changed_paths {
        let file = cache.file_cache.get(path).unwrap().clone();
        let page = match crate::parser::parse_file(&file) {
            Ok(p) => p,
            Err(e) => return Some(Err(e)),
        };
        let mtime = std::fs::metadata(path)
            .ok()
            .and_then(|m| m.modified().ok())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        let page_id = page.id.clone();

        // Content hash
        let new_content = hash_str(&page.content_md);
        if cache.content_hashes.get(&page_id).copied() != Some(new_content) {
            dirty_ids.insert(page_id.clone());
        }
        cache.content_hashes.insert(page_id.clone(), new_content);

        // Meta hash
        let new_meta = hash_meta(&page);
        if cache.meta_hashes.get(&page_id).copied() != Some(new_meta) {
            dirty_ids.insert(page_id.clone());
            meta_changed = true;
        }
        cache.meta_hashes.insert(page_id.clone(), new_meta);

        // Link hash
        let new_links = hash_links(&page);
        if cache.link_hashes.get(&page_id).copied() != Some(new_links) {
            dirty_ids.insert(page_id.clone());
            links_changed = true;
        }
        cache.link_hashes.insert(page_id.clone(), new_links);

        // Tag hash
        let new_tags = hash_str(&page.meta.tags.join(","));
        cache.tag_hashes.insert(page_id.clone(), new_tags);

        // Update parse cache
        cache.parse_cache.insert(path.clone(), (mtime, page));
    }

    // If meta/links/tags changed, this is structural — bail to full rebuild
    if meta_changed || links_changed {
        // Revert: the hashes were updated but we need full rebuild.
        // The full path will recompute everything, so this is fine.
        return None;
    }

    if dirty_ids.is_empty() {
        return Some(Ok((0, 0)));
    }

    // Content-only change: update cached store in-place and re-render dirty pages
    {
        let store = cache.cached_store.as_mut().unwrap();
        for dirty_id in &dirty_ids {
            // Find the freshly-parsed page in parse_cache
            if let Some((_, new_page)) = cache.parse_cache.values()
                .find(|(_, p)| p.id == **dirty_id)
            {
                if let Some(cached_page) = store.pages.get_mut(dirty_id) {
                    cached_page.content_md = new_page.content_md.clone();
                    cached_page.meta = new_page.meta.clone();
                    cached_page.outgoing_links = new_page.outgoing_links.clone();
                }
            }
        }
    }

    let store = cache.cached_store.as_ref().unwrap();

    // Render only the dirty pages (not all pages — avoids iterating 11K cached entries)
    let dirty_count = dirty_ids.len();
    let mut rendered_dirty = Vec::with_capacity(dirty_count);
    {
        let env = match crate::render::make_template_env(config) {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        };
        for dirty_id in &dirty_ids {
            if let Some(page) = store.pages.get(dirty_id) {
                if !crate::graph::PageStore::is_page_public(page, &config.content) {
                    continue;
                }
                let rp = match crate::render::render_single_page(page, dirty_id, store, config, &env) {
                    Ok(r) => r,
                    Err(e) => return Some(Err(e)),
                };
                cache.render_cache.insert(dirty_id.clone(), rp.clone());
                rendered_dirty.push(rp);
            }
        }
    }

    // Write only dirty pages
    if let Err(e) = crate::output::write_dirty_pages(&rendered_dirty, &dirty_ids, config) {
        return Some(Err(e));
    }

    Some(Ok((dirty_count, dirty_count)))
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

        // Keep file_cache in sync for fast path
        cache.file_cache.insert(file.path.clone(), file.clone());

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
    cache.file_cache.retain(|path, _| current_paths.contains(path));

    // Step 2b: Discover and merge subgraphs — use cached pages unless a subgraph file changed
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

        // Check if any changed file is inside a subgraph repo
        let subgraph_changed = changed_paths.iter().any(|p| {
            cache.subgraph_repo_paths.iter().any(|repo| p.starts_with(repo))
        });

        if subgraph_changed || cache.subgraph_pages.is_empty() {
            // Re-scan and re-parse subgraphs (a subgraph file was modified)
            let mut new_subgraph_pages = Vec::new();
            let mut new_repo_paths = Vec::new();
            for decl in &subgraph_decls {
                new_repo_paths.push(decl.repo_path.clone());
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
                        new_subgraph_pages.push(page);
                    }
                }
                if declaring_page.is_some() {
                    all_parsed.retain(|p| {
                        !(p.id == decl.declaring_page_id && p.subgraph.is_none())
                    });
                }
            }
            cache.subgraph_pages = new_subgraph_pages;
            cache.subgraph_repo_paths = new_repo_paths;
        } else {
            // No subgraph file changed — evict declaring pages and reuse cache
            for decl in &subgraph_decls {
                let has_declaring = all_parsed.iter().any(|p| p.id == decl.declaring_page_id);
                if has_declaring {
                    all_parsed.retain(|p| {
                        !(p.id == decl.declaring_page_id && p.subgraph.is_none())
                    });
                }
            }
        }
        all_parsed.extend(cache.subgraph_pages.clone());
    }

    // Step 3: Detect content, meta, and link changes BEFORE building graph (cheap hash comparison).
    let mut dirty_ids: HashSet<PageId> = HashSet::new();
    let mut content_page_ids: HashSet<PageId> = HashSet::new();
    let mut meta_changed = false;
    let mut links_changed = false;

    for page in &all_parsed {
        content_page_ids.insert(page.id.clone());

        // Only mark pages dirty if they were actually re-parsed from a changed file.
        // Subgraph pages and unchanged pages may have non-deterministic hash diffs
        // due to merge order differences — these must not pollute dirty_ids.
        let was_reparsed = changed_page_ids.contains(&page.id);

        // Content hash — detect body text changes
        let new_hash = hash_str(&page.content_md);
        if let Some(&old_hash) = cache.content_hashes.get(&page.id) {
            if old_hash != new_hash && was_reparsed {
                dirty_ids.insert(page.id.clone());
            }
        } else if was_reparsed {
            dirty_ids.insert(page.id.clone());
        }
        cache.content_hashes.insert(page.id.clone(), new_hash);

        // Meta hash — detect frontmatter changes (title, aliases, icon, stake, tags)
        let new_meta_hash = hash_meta(page);
        if let Some(&old_meta_hash) = cache.meta_hashes.get(&page.id) {
            if old_meta_hash != new_meta_hash && was_reparsed {
                dirty_ids.insert(page.id.clone());
                meta_changed = true;
            }
        } else if cache.initialized && was_reparsed {
            meta_changed = true;
        }
        cache.meta_hashes.insert(page.id.clone(), new_meta_hash);

        // Outgoing links hash — detect link set changes
        let new_link_hash = hash_links(page);
        if let Some(&old_link_hash) = cache.link_hashes.get(&page.id) {
            if old_link_hash != new_link_hash && was_reparsed {
                dirty_ids.insert(page.id.clone());
                links_changed = true;
            }
        } else if cache.initialized && was_reparsed {
            links_changed = true;
        }
        cache.link_hashes.insert(page.id.clone(), new_link_hash);
    }

    // Check for structural change: pages added or removed, or tags changed, or meta/links changed.
    let pages_added_or_removed = content_page_ids != cache.last_content_page_ids;
    let tags_changed = dirty_ids.iter().any(|dirty_id| {
        all_parsed.iter().find(|p| &p.id == dirty_id).map(|page| {
            let new_tag_hash = hash_str(&page.meta.tags.join(","));
            let changed = cache.tag_hashes.get(dirty_id)
                .map(|&old| old != new_tag_hash)
                .unwrap_or(true);
            cache.tag_hashes.insert(dirty_id.clone(), new_tag_hash);
            changed
        }).unwrap_or(false)
    });

    // Meta or link changes escalate to structural rebuild since they affect other pages
    let structural_change = !cache.initialized
        || pages_added_or_removed
        || tags_changed
        || meta_changed
        || links_changed;


    // Step 4: Build or reuse graph store.
    // Full graph build (PageRank, gravity, etc.) is expensive — only do it for structural changes.
    if structural_change || cache.cached_store.is_none() {
        let old_namespace_keys = cache.last_namespace_keys.clone();
        let store = crate::graph::build_graph(all_parsed)?;
        cache.last_content_page_ids = content_page_ids.clone();

        // Fix 4: Compare backlink snapshots — mark pages with changed backlinks dirty
        for (page_id, backlinks) in &store.backlinks {
            let mut sorted = backlinks.clone();
            sorted.sort();
            let old_snapshot = cache.backlink_snapshots.get(page_id);
            if old_snapshot != Some(&sorted) {
                dirty_ids.insert(page_id.clone());
            }
            cache.backlink_snapshots.insert(page_id.clone(), sorted);
        }
        // Also check pages that used to have backlinks but no longer do
        let new_backlink_keys: HashSet<&PageId> = store.backlinks.keys().collect();
        let stale_backlink_pages: Vec<PageId> = cache.backlink_snapshots.keys()
            .filter(|k| !new_backlink_keys.contains(k))
            .cloned()
            .collect();
        for page_id in &stale_backlink_pages {
            if cache.backlink_snapshots.get(page_id).map(|v| !v.is_empty()).unwrap_or(false) {
                dirty_ids.insert(page_id.clone());
            }
            cache.backlink_snapshots.remove(page_id);
        }

        // Fix 3: Mark namespace parents dirty on structural changes
        for ns_key in store.namespace_tree.keys() {
            // Namespace parent pages need re-rendering (they show child lists)
            dirty_ids.insert(ns_key.clone());
        }
        // Also mark parents of removed namespaces dirty
        for old_key in &old_namespace_keys {
            if !store.namespace_tree.contains_key(old_key) {
                dirty_ids.insert(old_key.clone());
            }
        }
        cache.last_namespace_keys = store.namespace_tree.keys().cloned().collect();

        for (page_id, page) in &store.pages {
            if !cache.content_hashes.contains_key(page_id) {
                cache.content_hashes.insert(page_id.clone(), hash_str(&page.content_md));
            }
            if !cache.meta_hashes.contains_key(page_id) {
                cache.meta_hashes.insert(page_id.clone(), hash_meta(page));
            }
            if !cache.link_hashes.contains_key(page_id) {
                cache.link_hashes.insert(page_id.clone(), hash_links(page));
            }
        }
        cache.cached_store = Some(store);
    } else {
        // Content-only change: update page content in the cached store in-place.
        // Skip expensive PageRank/gravity/trikernel recomputation.
        let store = cache.cached_store.as_mut().unwrap();
        for dirty_id in &dirty_ids {
            if let Some(new_page) = all_parsed.iter().find(|p| &p.id == dirty_id) {
                if let Some(cached_page) = store.pages.get_mut(dirty_id) {
                    cached_page.content_md = new_page.content_md.clone();
                    cached_page.meta = new_page.meta.clone();
                    cached_page.outgoing_links = new_page.outgoing_links.clone();
                }
            }
        }
    }

    // Fix 5: Prune stale cache entries — only keep current page IDs
    {
        let store = cache.cached_store.as_ref().unwrap();
        let current_ids: HashSet<&PageId> = store.pages.keys().collect();
        cache.content_hashes.retain(|k, _| current_ids.contains(k));
        cache.tag_hashes.retain(|k, _| current_ids.contains(k));
        cache.meta_hashes.retain(|k, _| current_ids.contains(k));
        cache.link_hashes.retain(|k, _| current_ids.contains(k));
        cache.backlink_snapshots.retain(|k, _| current_ids.contains(k));
        cache.render_cache.retain(|k, _| current_ids.contains(k)
            || k.starts_with("__"));  // Keep synthetic page caches
    }

    if structural_change {
        dirty_ids.insert("__structural__".to_string());
    }

    let dirty_count = dirty_ids.len();
    let store = cache.cached_store.as_ref().unwrap();

    // Step 5: Selective render
    let dirty_ref = if cache.initialized {
        Some(&dirty_ids)
    } else {
        None
    };
    let rendered = crate::render::render_cached(
        store,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{PageMeta, PageKind, ParsedPage};
    use std::collections::HashMap;
    use std::path::PathBuf;

    /// Helper to build a minimal ParsedPage for testing hash functions.
    fn make_page(id: &str, title: &str, content: &str) -> ParsedPage {
        ParsedPage {
            id: id.to_string(),
            meta: PageMeta {
                title: title.to_string(),
                properties: HashMap::new(),
                tags: vec![],
                public: Some(true),
                aliases: vec![],
                date: None,
                icon: None,
                menu_order: None,
                stake: None,
            },
            kind: PageKind::Page,
            source_path: PathBuf::new(),
            namespace: None,
            subgraph: None,
            content_md: content.to_string(),
            outgoing_links: vec![],
        }
    }

    #[test]
    fn test_hash_str_deterministic() {
        let input = "hello world";
        assert_eq!(hash_str(input), hash_str(input));
        // Call multiple times to ensure stability
        let h1 = hash_str(input);
        let h2 = hash_str(input);
        let h3 = hash_str(input);
        assert_eq!(h1, h2);
        assert_eq!(h2, h3);
    }

    #[test]
    fn test_hash_str_different() {
        let h1 = hash_str("hello");
        let h2 = hash_str("world");
        assert_ne!(h1, h2, "different inputs must produce different hashes");
        // Also test subtle differences
        let h3 = hash_str("Hello");
        assert_ne!(h1, h3, "case difference must produce different hash");
    }

    #[test]
    fn test_hash_meta_detects_title_change() {
        let page_a = make_page("test", "Title A", "content");
        let page_b = make_page("test", "Title B", "content");
        assert_ne!(
            hash_meta(&page_a),
            hash_meta(&page_b),
            "different titles must produce different meta hashes"
        );
    }

    #[test]
    fn test_hash_meta_detects_icon_change() {
        let mut page_a = make_page("test", "Title", "content");
        let mut page_b = make_page("test", "Title", "content");
        page_a.meta.icon = Some("🔵".to_string());
        page_b.meta.icon = Some("🟢".to_string());
        assert_ne!(
            hash_meta(&page_a),
            hash_meta(&page_b),
            "different icons must produce different meta hashes"
        );
        // Also check None vs Some
        let page_c = make_page("test", "Title", "content");
        assert_ne!(
            hash_meta(&page_a),
            hash_meta(&page_c),
            "icon Some vs None must produce different meta hashes"
        );
    }

    #[test]
    fn test_hash_meta_detects_alias_change() {
        let mut page_a = make_page("test", "Title", "content");
        let mut page_b = make_page("test", "Title", "content");
        page_a.meta.aliases = vec!["alias1".to_string()];
        page_b.meta.aliases = vec!["alias2".to_string()];
        assert_ne!(
            hash_meta(&page_a),
            hash_meta(&page_b),
            "different aliases must produce different meta hashes"
        );
        // Same aliases → same hash
        let mut page_c = make_page("test", "Title", "content");
        page_c.meta.aliases = vec!["alias1".to_string()];
        assert_eq!(
            hash_meta(&page_a),
            hash_meta(&page_c),
            "identical aliases must produce same meta hash"
        );
    }

    #[test]
    fn test_hash_links_detects_link_change() {
        let mut page_a = make_page("test", "Title", "content");
        let mut page_b = make_page("test", "Title", "content");
        page_a.outgoing_links = vec!["link-a".to_string()];
        page_b.outgoing_links = vec!["link-b".to_string()];
        assert_ne!(
            hash_links(&page_a),
            hash_links(&page_b),
            "different link sets must produce different link hashes"
        );
    }

    #[test]
    fn test_hash_links_order_independent() {
        let mut page_a = make_page("test", "Title", "content");
        let mut page_b = make_page("test", "Title", "content");
        page_a.outgoing_links = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];
        page_b.outgoing_links = vec!["gamma".to_string(), "alpha".to_string(), "beta".to_string()];
        assert_eq!(
            hash_links(&page_a),
            hash_links(&page_b),
            "hash_links must be order-independent (it sorts internally)"
        );
    }
}
