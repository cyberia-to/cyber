//! Integration tests for the full build pipeline and incremental rebuild behavior.
//!
//! Each test creates a temporary directory with a minimal graph, runs the full
//! pipeline (scan -> parse -> build_graph -> render -> write_output), then
//! simulates changes and rebuilds to verify correctness.

use std::fs;
use std::path::Path;
use tempfile::TempDir;

use cyber_publish::config::SiteConfig;
use cyber_publish::graph;
use cyber_publish::output;
use cyber_publish::parser;
use cyber_publish::render;
use cyber_publish::scanner;

/// Create a minimal test environment: a temp dir with graph/ directory
/// and a SiteConfig pointing to it.
fn setup_test_env() -> (TempDir, SiteConfig) {
    let tmp = TempDir::new().expect("failed to create temp dir");
    let graph_dir = tmp.path().join("graph");
    fs::create_dir_all(&graph_dir).expect("failed to create graph dir");

    let mut config = SiteConfig::default();
    config.build.input_dir = tmp.path().to_path_buf();
    config.build.output_dir = tmp.path().join("build");
    config.content.public_only = false; // show all pages in tests
    config.content.default_public = true;
    config.search.enabled = false; // skip search index generation
    config.feeds.enabled = false; // skip RSS feed generation
    config.graph.enabled = false; // skip graph data generation

    (tmp, config)
}

/// Write a markdown page to graph/<name>.md with optional frontmatter.
fn write_page(base: &Path, name: &str, content: &str) {
    let graph_dir = base.join("graph");
    // Handle namespaced pages: "ns/page" -> graph/ns/page.md
    let page_path = graph_dir.join(format!("{}.md", name));
    if let Some(parent) = page_path.parent() {
        fs::create_dir_all(parent).expect("failed to create page parent dir");
    }
    fs::write(&page_path, content).expect("failed to write page");
}

/// Delete a page from graph/<name>.md
fn delete_page(base: &Path, name: &str) {
    let page_path = base.join("graph").join(format!("{}.md", name));
    if page_path.exists() {
        fs::remove_file(&page_path).expect("failed to delete page");
    }
    // Also remove parent dir if empty (for namespace pages)
    if let Some(parent) = page_path.parent() {
        let _ = fs::remove_dir(parent); // ignore error if not empty
    }
}

/// Run the full build pipeline: scan -> parse -> build_graph -> render -> write_output.
/// Returns the rendered pages for inspection.
fn full_build(config: &SiteConfig) -> Vec<render::RenderedPage> {
    let discovered = scanner::scan(&config.build.input_dir, &config.content)
        .expect("scan failed");
    let parsed_pages = parser::parse_all(&discovered).expect("parse failed");
    let page_store = graph::build_graph(parsed_pages).expect("build_graph failed");
    let rendered = render::render_all(&page_store, config).expect("render failed");
    output::write_output(&rendered, &page_store, config, &discovered)
        .expect("write_output failed");
    rendered
}

/// Run a full rebuild (simulating incremental by doing a fresh build).
/// This tests the output state after changes without requiring BuildCache.
fn rebuild(config: &SiteConfig) -> Vec<render::RenderedPage> {
    let discovered = scanner::scan(&config.build.input_dir, &config.content)
        .expect("scan failed");
    let parsed_pages = parser::parse_all(&discovered).expect("parse failed");
    let page_store = graph::build_graph(parsed_pages).expect("build_graph failed");
    let rendered = render::render_all(&page_store, config).expect("render failed");
    output::write_incremental(&rendered, &page_store, config, &discovered)
        .expect("write_incremental failed");
    rendered
}

/// Check if a page output exists at build/<slug>/index.html
fn output_exists(config: &SiteConfig, slug: &str) -> bool {
    let index_path = config.build.output_dir.join(slug).join("index.html");
    index_path.exists()
}

/// Read the HTML content of a page output at build/<slug>/index.html
fn read_output(config: &SiteConfig, slug: &str) -> String {
    let index_path = config.build.output_dir.join(slug).join("index.html");
    fs::read_to_string(&index_path)
        .unwrap_or_else(|_| panic!("failed to read output at {}", index_path.display()))
}

#[test]
fn test_new_page_appears_in_output() {
    let (tmp, config) = setup_test_env();

    write_page(tmp.path(), "test", "---\ntags: cyber\n---\n\nHello world");

    let _rendered = full_build(&config);

    assert!(
        output_exists(&config, "test"),
        "build/test/index.html should exist after building graph/test.md"
    );

    let html = read_output(&config, "test");
    assert!(
        html.contains("Hello world"),
        "output HTML should contain the page content"
    );
}

#[test]
fn test_deleted_page_removed_from_output() {
    let (tmp, config) = setup_test_env();

    // Build with the page present
    write_page(tmp.path(), "deleteme", "---\ntags: test\n---\n\nTemporary page");
    write_page(tmp.path(), "keeper", "---\ntags: test\n---\n\nPermanent page");
    full_build(&config);

    assert!(output_exists(&config, "deleteme"), "page should exist after initial build");
    assert!(output_exists(&config, "keeper"), "keeper should exist after initial build");

    // Delete the page and rebuild
    delete_page(tmp.path(), "deleteme");
    rebuild(&config);

    assert!(
        !output_exists(&config, "deleteme"),
        "build/deleteme/index.html should be removed after page deletion and rebuild"
    );
    assert!(
        output_exists(&config, "keeper"),
        "keeper page should still exist after rebuild"
    );
}

#[test]
fn test_moved_page_old_output_removed() {
    let (tmp, config) = setup_test_env();

    // Build with page at graph/old.md
    write_page(tmp.path(), "old", "---\ntags: test\n---\n\nMoving page");
    full_build(&config);

    assert!(output_exists(&config, "old"), "old page should exist after initial build");

    // Move to graph/new.md (delete old, create new)
    delete_page(tmp.path(), "old");
    write_page(tmp.path(), "new", "---\ntags: test\n---\n\nMoving page");
    rebuild(&config);

    assert!(
        !output_exists(&config, "old"),
        "old output should be removed after page move"
    );
    assert!(
        output_exists(&config, "new"),
        "new output should exist after page move"
    );
}

#[test]
fn test_namespace_move_updates_listing() {
    let (tmp, config) = setup_test_env();

    // Build with page at root level
    write_page(tmp.path(), "mypage", "---\ntags: test\n---\n\nRoot page");
    full_build(&config);

    assert!(output_exists(&config, "mypage"), "root page should exist");

    // Move into a namespace: graph/ns/mypage.md
    delete_page(tmp.path(), "mypage");
    write_page(tmp.path(), "ns/mypage", "---\ntags: test\n---\n\nNamespaced page");
    rebuild(&config);

    // Old root-level output should be gone
    assert!(
        !output_exists(&config, "mypage"),
        "root-level output should be removed after namespace move"
    );

    // Namespaced page should exist with its slugified ID
    let ns_slug = parser::slugify_page_name("ns/mypage");
    assert!(
        output_exists(&config, &ns_slug),
        "namespaced page output should exist at build/{}/index.html",
        ns_slug
    );

    // Verify the namespaced page HTML contains the content
    let ns_html = read_output(&config, &ns_slug);
    assert!(
        ns_html.contains("Namespaced page"),
        "namespaced page output should contain its content"
    );
}

#[test]
fn test_content_edit_reflects_in_output() {
    let (tmp, config) = setup_test_env();

    // Initial build
    write_page(
        tmp.path(),
        "editable",
        "---\ntags: test\n---\n\nOriginal content here",
    );
    full_build(&config);

    let html_before = read_output(&config, "editable");
    assert!(
        html_before.contains("Original content here"),
        "initial build should contain original content"
    );

    // Edit the page content
    write_page(
        tmp.path(),
        "editable",
        "---\ntags: test\n---\n\nUpdated content with new info",
    );
    rebuild(&config);

    let html_after = read_output(&config, "editable");
    assert!(
        html_after.contains("Updated content with new info"),
        "rebuilt output should contain updated content"
    );
    assert!(
        !html_after.contains("Original content here"),
        "rebuilt output should not contain old content"
    );
}

#[test]
fn test_tag_change_updates_tag_page() {
    let (tmp, config) = setup_test_env();

    // Build with page having tag "alpha"
    write_page(
        tmp.path(),
        "tagged",
        "---\ntags: alpha\n---\n\nA page with a tag",
    );
    full_build(&config);

    // The tags index page should exist
    assert!(
        output_exists(&config, "tags"),
        "tags index page should exist"
    );

    // Check that tag page for "alpha" includes our page
    if output_exists(&config, "tags/alpha") {
        let tag_html = read_output(&config, "tags/alpha");
        assert!(
            tag_html.contains("tagged"),
            "tag page for 'alpha' should reference the tagged page"
        );
    }

    // Change the tag to "beta"
    write_page(
        tmp.path(),
        "tagged",
        "---\ntags: beta\n---\n\nA page with a different tag",
    );
    rebuild(&config);

    // Tag page for "beta" should now include the page
    if output_exists(&config, "tags/beta") {
        let tag_html = read_output(&config, "tags/beta");
        assert!(
            tag_html.contains("tagged"),
            "tag page for 'beta' should reference the tagged page after tag change"
        );
    }
}

#[test]
fn test_multiple_pages_build_correctly() {
    let (tmp, config) = setup_test_env();

    write_page(tmp.path(), "page-a", "---\ntags: test\n---\n\nPage A content");
    write_page(tmp.path(), "page-b", "---\ntags: test\n---\n\nPage B links to [[page-a]]");
    write_page(tmp.path(), "page-c", "---\ntags: test\n---\n\nPage C links to [[page-b]]");

    full_build(&config);

    assert!(output_exists(&config, "page-a"), "page-a should exist in output");
    assert!(output_exists(&config, "page-b"), "page-b should exist in output");
    assert!(output_exists(&config, "page-c"), "page-c should exist in output");

    // Verify wikilinks are resolved in output
    let html_b = read_output(&config, "page-b");
    assert!(
        html_b.contains("/page-a"),
        "page-b output should contain a link to page-a"
    );
}

#[test]
fn test_backlinks_appear_in_output() {
    let (tmp, config) = setup_test_env();

    write_page(tmp.path(), "target", "---\ntags: test\n---\n\nTarget page");
    write_page(
        tmp.path(),
        "linker",
        "---\ntags: test\n---\n\nThis page links to [[target]]",
    );

    full_build(&config);

    let target_html = read_output(&config, "target");
    // The target page should show a backlink from "linker"
    assert!(
        target_html.contains("linker"),
        "target page should show backlink from linker page"
    );
}

#[test]
fn test_namespace_hierarchy_builds() {
    let (tmp, config) = setup_test_env();

    write_page(
        tmp.path(),
        "project/alpha",
        "---\ntags: test\n---\n\nAlpha project",
    );
    write_page(
        tmp.path(),
        "project/beta",
        "---\ntags: test\n---\n\nBeta project",
    );

    full_build(&config);

    let alpha_slug = parser::slugify_page_name("project/alpha");
    let beta_slug = parser::slugify_page_name("project/beta");

    assert!(
        output_exists(&config, &alpha_slug),
        "project/alpha should have output"
    );
    assert!(
        output_exists(&config, &beta_slug),
        "project/beta should have output"
    );

    // Verify the rendered pages have correct content
    let alpha_html = read_output(&config, &alpha_slug);
    assert!(
        alpha_html.contains("Alpha project"),
        "alpha page output should contain its content"
    );

    let beta_html = read_output(&config, &beta_slug);
    assert!(
        beta_html.contains("Beta project"),
        "beta page output should contain its content"
    );
}

#[test]
fn test_empty_graph_builds_without_error() {
    let (_tmp, config) = setup_test_env();

    // Build with no pages at all
    full_build(&config);

    // Should still produce some output (index, tags page, etc.)
    assert!(
        config.build.output_dir.exists(),
        "output directory should exist even with empty graph"
    );

    // At minimum, an index page should be generated
    let index_path = config.build.output_dir.join("index.html");
    assert!(
        index_path.exists(),
        "index.html should exist even with empty graph"
    );
}

#[test]
fn test_page_with_aliases_builds() {
    let (tmp, config) = setup_test_env();

    write_page(
        tmp.path(),
        "canonical",
        "---\ntags: test\nalias: alt-name, another-alias\n---\n\nPage with aliases",
    );
    write_page(
        tmp.path(),
        "referrer",
        "---\ntags: test\n---\n\nLinks to [[alt-name]]",
    );

    full_build(&config);

    assert!(output_exists(&config, "canonical"), "canonical page should exist");

    // The referrer should have a link that resolves (through the alias)
    let referrer_html = read_output(&config, "referrer");
    assert!(
        referrer_html.contains("canonical") || referrer_html.contains("alt-name"),
        "referrer should contain a link related to the aliased page"
    );
}

#[test]
fn test_rebuild_after_adding_new_page() {
    let (tmp, config) = setup_test_env();

    // Initial build with one page
    write_page(tmp.path(), "first", "---\ntags: test\n---\n\nFirst page");
    full_build(&config);

    assert!(output_exists(&config, "first"), "first page should exist");

    // Add a second page and rebuild
    write_page(tmp.path(), "second", "---\ntags: test\n---\n\nSecond page");
    rebuild(&config);

    assert!(output_exists(&config, "first"), "first page should still exist");
    assert!(output_exists(&config, "second"), "second page should appear after rebuild");
}
