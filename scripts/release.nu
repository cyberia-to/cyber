#!/usr/bin/env nu
# Build a local host artifact with registry lock and sibling-source provenance.
def main [] {
    let root = ($env.FILE_PWD | path dirname)
    cd $root
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
    let checksum = (open --raw $binary | hash sha256)
    mkdir dist
    cp $binary $"dist/cyber($extension)"
    $"($checksum)  cyber($extension)\n" | save --force dist/SHA256SUMS
    {schema: "cyber/build/v1", built_at: (date now | into string),
     version: (^$binary --version | str trim), rustc: (^rustc -vV),
     sha256: $checksum, lock_sha256: (open --raw Cargo.lock | hash sha256),
     sources: $sources} | to json | save --force dist/build.json
    print $"artifact: ($root)/dist/cyber($extension)"
    print "provenance: dist/build.json; checksum: dist/SHA256SUMS"
}
