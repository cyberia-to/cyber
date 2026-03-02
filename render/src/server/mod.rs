mod reload;

use crate::config::SiteConfig;
use anyhow::Result;
use colored::Colorize;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

pub fn serve(
    config: &SiteConfig,
    bind: &str,
    port: u16,
    live_reload: bool,
    open_browser: bool,
) -> Result<()> {
    let output_dir = config.build.output_dir.clone();
    let addr = format!("{}:{}", bind, port);
    let url = format!("http://{}", addr);

    println!(
        "{} {} → {}",
        "Serving".green().bold(),
        output_dir.display(),
        url
    );

    if live_reload {
        println!("  {} Live reload enabled", "Watch".dimmed());
    }

    let server = Arc::new(
        tiny_http::Server::http(&addr)
            .map_err(|e| anyhow::anyhow!("Failed to start server: {}", e))?,
    );

    if open_browser {
        open_url(&url);
    }

    // Build version counter — incremented after each rebuild
    let build_version = Arc::new(AtomicU64::new(0));
    let running = Arc::new(AtomicBool::new(true));

    // Start file watcher + rebuild thread
    if live_reload {
        reload::start_watch_rebuild(config.clone(), build_version.clone());
    }

    println!("  Press Ctrl+C to stop\n");

    // Ctrl+C handler
    {
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Failed to set Ctrl+C handler");
    }

    while running.load(Ordering::SeqCst) {
        match server.recv_timeout(Duration::from_millis(500)) {
            Ok(Some(request)) => {
                let url_path = request.url().to_string();
                let url_path_clean = url_path.split('?').next().unwrap_or(&url_path);

                if url_path_clean == "/__reload" {
                    // Handle SSE in a separate thread so the server keeps serving
                    let version = build_version.clone();
                    let is_running = running.clone();
                    std::thread::spawn(move || {
                        handle_sse_reload(request, &version, &is_running);
                    });
                } else {
                    // Handle regular requests in a thread to keep the main loop responsive.
                    // This prevents serialized request handling from blocking concurrent loads.
                    let dir = output_dir.clone();
                    std::thread::spawn(move || {
                        handle_request(request, &dir, live_reload);
                    });
                }
            }
            Ok(None) => {
                // Timeout — loop continues, checks running flag
            }
            Err(_) => break,
        }
    }

    println!("\n{} Server stopped.", "Bye!".green().bold());
    Ok(())
}

/// SSE long-poll: wait for build_version to change, then send "reload" event.
/// EventSource on the client will automatically reconnect after receiving.
fn handle_sse_reload(request: tiny_http::Request, version: &AtomicU64, running: &AtomicBool) {
    let current = version.load(Ordering::SeqCst);

    // Wait up to 30s for a rebuild (poll every 200ms)
    let start = Instant::now();
    while running.load(Ordering::SeqCst) && start.elapsed() < Duration::from_secs(30) {
        if version.load(Ordering::SeqCst) != current {
            // Version changed → send reload event
            let body = "data: reload\n\n";
            let response = tiny_http::Response::from_string(body)
                .with_header(
                    tiny_http::Header::from_bytes(b"Content-Type", b"text/event-stream").unwrap(),
                )
                .with_header(tiny_http::Header::from_bytes(b"Cache-Control", b"no-cache").unwrap())
                .with_header(tiny_http::Header::from_bytes(b"Connection", b"close").unwrap());
            let _ = request.respond(response);
            return;
        }
        std::thread::sleep(Duration::from_millis(200));
    }

    // Timeout keep-alive — sent as data so client onmessage fires (resets retry counter)
    let body = "data: ping\n\n";
    let response = tiny_http::Response::from_string(body)
        .with_header(tiny_http::Header::from_bytes(b"Content-Type", b"text/event-stream").unwrap())
        .with_header(tiny_http::Header::from_bytes(b"Cache-Control", b"no-cache").unwrap())
        .with_header(tiny_http::Header::from_bytes(b"Connection", b"close").unwrap());
    let _ = request.respond(response);
}

fn handle_request(request: tiny_http::Request, output_dir: &Path, inject_reload: bool) {
    let url_path = request.url().to_string();
    let url_path = url_path.split('?').next().unwrap_or(&url_path);

    // Determine file path
    let file_path = resolve_file_path(url_path, output_dir);

    if file_path.exists() {
        let content_type = guess_content_type(&file_path);
        let mut content = std::fs::read(&file_path).unwrap_or_default();

        // Inject live reload script into HTML
        if inject_reload && content_type.starts_with("text/html") {
            if let Ok(html) = String::from_utf8(content.clone()) {
                let injected =
                    html.replace("</body>", &format!("{}\n</body>", reload::RELOAD_SCRIPT));
                content = injected.into_bytes();
            }
        }

        let response = tiny_http::Response::from_data(content)
            .with_header(
                tiny_http::Header::from_bytes(b"Content-Type", content_type.as_bytes()).unwrap(),
            )
            .with_header(
                tiny_http::Header::from_bytes(
                    b"Cache-Control",
                    b"no-cache, no-store, must-revalidate",
                )
                .unwrap(),
            )
            .with_header(tiny_http::Header::from_bytes(b"Connection", b"close").unwrap());
        let _ = request.respond(response);
    } else {
        let response = tiny_http::Response::from_string("404 Not Found")
            .with_status_code(404)
            .with_header(tiny_http::Header::from_bytes(b"Content-Type", b"text/html").unwrap())
            .with_header(tiny_http::Header::from_bytes(b"Connection", b"close").unwrap());
        let _ = request.respond(response);
    }
}

fn resolve_file_path(url_path: &str, output_dir: &Path) -> PathBuf {
    if url_path == "/" || url_path.is_empty() {
        return output_dir.join("index.html");
    }

    let clean = url_path.trim_start_matches('/');
    let path = output_dir.join(clean);

    if path.is_dir() {
        path.join("index.html")
    } else if path.exists() {
        path
    } else {
        // Try adding .html
        let with_html = output_dir.join(format!("{}.html", clean));
        if with_html.exists() {
            with_html
        } else {
            // Try as directory with index.html
            let as_dir = output_dir.join(clean).join("index.html");
            if as_dir.exists() {
                as_dir
            } else {
                path
            }
        }
    }
}

fn guess_content_type(path: &Path) -> String {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8".to_string(),
        Some("css") => "text/css; charset=utf-8".to_string(),
        Some("js") => "application/javascript; charset=utf-8".to_string(),
        Some("json") => "application/json".to_string(),
        Some("xml") => "application/xml".to_string(),
        Some("png") => "image/png".to_string(),
        Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
        Some("gif") => "image/gif".to_string(),
        Some("svg") => "image/svg+xml".to_string(),
        Some("webp") => "image/webp".to_string(),
        Some("woff2") => "font/woff2".to_string(),
        Some("woff") => "font/woff".to_string(),
        Some("ico") => "image/x-icon".to_string(),
        Some("pdf") => "application/pdf".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

fn open_url(url: &str) {
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).spawn();
    }
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", url])
            .spawn();
    }
}
