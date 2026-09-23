#!/usr/bin/env nu
# Build a local host artifact with registry lock and sibling-source provenance.
def main [--locked-sources, --previous-binary: string = ""] {
    let previous = if ($previous_binary | is-empty) { null } else { $previous_binary | path expand }
    let previous_sha256 = if $previous == null { null } else { open --raw $previous | hash sha256 }
    let root = ($env.FILE_PWD | path dirname)
    cd $root
    if $locked_sources {
        ^nu analizer/node-sources.nu $root check
        if $env.LAST_EXIT_CODE != 0 { error make {msg: "source lock verification failed"} }
    }
    ^cargo fmt --check
    if $env.LAST_EXIT_CODE != 0 { error make {msg: "format check failed"} }
    ^cargo test --locked
    if $env.LAST_EXIT_CODE != 0 { error make {msg: "tests failed"} }
    ^cargo build --release --locked
    if $env.LAST_EXIT_CODE != 0 { error make {msg: "release build failed"} }
    let metadata = (^cargo metadata --locked --format-version 1 | from json)
    let sources = ($metadata.packages | where source == null | each {|package|
        let dir = ($package.manifest_path | path dirname)
        let revision = (do { ^git -C $dir rev-parse HEAD } | complete)
        if $revision.exit_code != 0 { error make {msg: $"missing git provenance: ($dir)"} }
        let changes = (do { ^git -C $dir status --porcelain } | complete)
        if $changes.exit_code != 0 { error make {msg: $"cannot inspect source: ($dir)"} }
        {name: $package.name, version: $package.version, manifest: $package.manifest_path,
         revision: ($revision.stdout | str trim), dirty: ($changes.stdout | is-not-empty),
         changes: $changes.stdout}
    })
    let extension = if $nu.os-info.name == "windows" { ".exe" } else { "" }
    let binary = ($metadata.target_directory | path join $"release/cyber($extension)")
    with-env {CYBER_TEST_BINARY: $binary} {
        ^cargo test --locked --test node
        if $env.LAST_EXIT_CODE != 0 { error make {msg: "release binary integration failed"} }
    }
    if $previous != null {
        ^cargo run --locked --example node_storage_compat -- $previous $binary
        if $env.LAST_EXIT_CODE != 0 { error make {msg: "previous-writer compatibility failed"} }
        if (open --raw $previous | hash sha256) != $previous_sha256 {
            error make {msg: "previous binary changed during compatibility qualification"}
        }
    }
    if $locked_sources {
        ^nu analizer/node-sources.nu $root inventory
        if $env.LAST_EXIT_CODE != 0 { error make {msg: "sources changed during release validation"} }
    }
    let checksum = (open --raw $binary | hash sha256)
    mkdir dist
    cp $binary $"dist/cyber($extension)"
    $"($checksum)  cyber($extension)\n" | save --force dist/SHA256SUMS
    let compiler = (^rustc -vV)
    let target = ($compiler | lines | where $it =~ '^host: ' | first | str replace 'host: ' '')
    let inventory_sha256 = if $locked_sources { open --raw dist/soft3-dependencies.json | hash sha256 } else { null }
    {schema: "cyber/build/v1", built_at: (date now | into string), sources_locked: $locked_sources,
     version: (^$binary --version | str trim), rustc: $compiler, target: $target,
     sha256: $checksum, lock_sha256: (open --raw Cargo.lock | hash sha256),
     soft3_dependencies_sha256: $inventory_sha256,
     compatibility_previous_sha256: $previous_sha256,
     sources: $sources} | to json | save --force dist/build.json
    print $"artifact: ($root)/dist/cyber($extension)"
    print "provenance: dist/build.json; checksum: dist/SHA256SUMS"
}
