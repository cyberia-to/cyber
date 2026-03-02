use crate::config::SiteConfig;
use anyhow::Result;
use colored::Colorize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{mpsc, Arc};
use std::time::Duration;

pub const RELOAD_SCRIPT: &str = r#"<script>
(function() {
  let retries = 0;
  function connect() {
    const es = new EventSource('/__reload');
    es.onmessage = function(e) {
      if (e.data === 'reload') {
        window.location.reload();
      }
    };
    es.onerror = function() {
      es.close();
      if (retries < 30) {
        retries++;
        setTimeout(connect, 1000);
      }
    };
  }
  connect();
})();
</script>"#;

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

    let (tx, rx) = mpsc::channel();

    let mut watcher =
        notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                if matches!(
                    event.kind,
                    notify::EventKind::Modify(_)
                        | notify::EventKind::Create(_)
                        | notify::EventKind::Remove(_)
                ) {
                    let _ = tx.send(());
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

    loop {
        if rx.recv().is_ok() {
            // Debounce: wait 300ms and drain additional events
            std::thread::sleep(Duration::from_millis(300));
            while rx.try_recv().is_ok() {}

            eprintln!("  {} File change detected, rebuilding...", "Watch".yellow());
            let start = std::time::Instant::now();

            match full_rebuild(config) {
                Ok(page_count) => {
                    let elapsed = start.elapsed();
                    build_version.fetch_add(1, Ordering::SeqCst);
                    eprintln!(
                        "  {} Rebuilt {} pages in {:.2}s",
                        "Done".green(),
                        page_count,
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

/// Run the full build pipeline: scan → parse → graph → render → output.
fn full_rebuild(config: &SiteConfig) -> Result<usize> {
    let discovered = crate::scanner::scan(&config.build.input_dir, &config.content)?;
    let parsed = crate::parser::parse_all(&discovered)?;
    let store = crate::graph::build_graph(parsed)?;
    let rendered = crate::render::render_all(&store, config)?;
    let count = rendered.len();
    crate::output::write_output(&rendered, &store, config, &discovered)?;
    Ok(count)
}
