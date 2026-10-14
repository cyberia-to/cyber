#!/usr/bin/env nu
# Build cyber.page: this repo is the root graph, the public subgraphs of the
# census ride along so every wiki-link resolves exactly as on localhost.
#
#   nu scripts/build.nu                                   # protocol pages alone
#   nu scripts/build.nu --census ~/cyber/cyberia-blog/subgraphs.toml --public-only
#
# The census is cyberia-blog/subgraphs.toml (read, never edited here). Each
# public entry is expected at ~/cyber/<name>; missing checkouts are skipped
# with a warning so a local build never fails on an absent sibling.

def main [
    --output: path         # output directory (default: ./build)
    --optica: path         # optica binary (default: ~/cyber/optica/target/release/optica)
    --census: path         # subgraphs.toml to mount beside the protocol pages
    --public-only          # drop private and local-only census entries
    --root-dir: path       # where sibling checkouts live (default: ~/cyber)
] {
    let root = (pwd)
    if not ($"($root)/publish.toml" | path exists) {
        error make {msg: "run from the cyber repo root"}
    }
    let optica_bin = if $optica == null {
        $"($env.HOME)/cyber/optica/target/release/optica"
    } else {
        $optica
    }
    if not ($optica_bin | path exists) {
        error make {msg: $"optica binary not found at ($optica_bin)"}
    }
    let out = if $output == null { $"($root)/build" } else { $output }

    let subgraph_args = if $census == null {
        print $"protocol pages alone → ($out)"
        []
    } else {
        let config_path = (materialize-census $census $public_only $root_dir)
        let n = (open $config_path | get subgraphs | length)
        print $"protocol pages + ($n) subgraphs → ($out)"
        ["--subgraphs" $config_path]
    }

    ^$optica_bin build $root --output $out ...$subgraph_args
    if ($"($root)/_redirects" | path exists) {
        cp $"($root)/_redirects" $"($out)/_redirects"
    }
    # the landing deck is the home of cyber.page: it replaces optica's root
    # index.html; every graph page keeps its URL underneath.
    if ($"($root)/landing/index.html" | path exists) {
        cp $"($root)/landing/index.html" $"($out)/index.html"
        cp $"($root)/landing/style.css" $"($out)/style.css"
        cp $"($root)/landing/deck.js" $"($out)/deck.js"
        if ($"($root)/landing/graph.json" | path exists) { cp $"($root)/landing/graph.json" $"($out)/graph.json" }
        print "landing → /"
    }
}

# Siblings contribute their markdown. Source code and data files render as
# pages too (cyberia.blog carries them), but on cyber.page they would be
# 1.4 GB of the 2.1 GB build and Netlify caps a deploy at 25,000 files.
def sibling-excludes [] {
    [
        "**/*.rs" "**/*.py" "**/*.ts" "**/*.tsx" "**/*.js" "**/*.mjs" "**/*.css"
        "**/*.wgsl" "**/*.tasm" "**/*.tri" "**/*.nu" "**/*.sh" "**/*.go" "**/*.c" "**/*.h"
        "**/*.json" "**/*.jsonl" "**/*.toml" "**/*.yml" "**/*.yaml" "**/*.lock"
        "**/*.csv" "**/*.csv.gz" "**/*.gz" "**/*.parquet" "**/*.bin"
        "**/.gitignore" "**/.github/**" "**/eval/data/**" "**/fixtures/**"
    ]
}

# Turn the census into optica's --subgraphs file. cyber itself is the root
# graph and cyberia-blog is the chronicle, so both stay out. Mount is
# parent/name as in the census; no subgraph enters the top nav.
def materialize-census [census: path, public_only: bool, root_dir] {
    let base = if $root_dir == null { $"($env.HOME)/cyber" } else { $root_dir }
    let data = (open $census)
    let key = if ($data | get -o subgraph | is-not-empty) { "subgraph" } else { "subgraphs" }
    let decls = ($data | get $key
        | where ($it.name? | is-not-empty)
        | where ($it.name != "cyber") and ($it.name != "cyberia-blog")
        | where ($it.archived? | default false) != true
        | where ($it.orphan? | default false) != true
        | where {|d|
            if not $public_only { true } else {
                (($d.visibility? | default "public") == "public") and (($d."local-only"? | default false) != true)
            }
        })
    let present = ($decls | each {|d|
        let path = ($base | path join $d.name)
        if not ($path | path exists) {
            print $"  skip ($d.name): no checkout at ($path)"
            null
        } else {
            let mount = if ($d.mount? | is-not-empty) { $d.mount } else if ($d.parent? | is-not-empty) { $"($d.parent)/($d.name)" } else { $d.name }
            {name: $d.name, path: $path, mount: $mount, visibility: ($d.visibility? | default "public"), menu: false, exclude: (sibling-excludes)}
        }
    } | compact)
    let config_path = $"($env.TMPDIR? | default "/tmp")/cyber-page-subgraphs.toml"
    ({subgraphs: $present} | to toml) | save --force $config_path
    $config_path
}
