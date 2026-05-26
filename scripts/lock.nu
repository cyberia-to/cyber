#!/usr/bin/env nu
# lock.nu — snapshot current HEAD of every subgraph into subgraphs.toml
#
# Reads each subgraph's git HEAD and writes the commit SHA back into
# the `commit` field of the matching [[subgraph]] entry. Entries whose
# repo directory does not exist are left unchanged.
#
# Usage:
#   nu scripts/lock.nu          # write HEADs into subgraphs.toml
#   nu scripts/lock.nu --dry-run  # print what would be written, no changes

def main [
    --dry-run  # print resolved commits without modifying subgraphs.toml
] {
    let ws_root = (workspace-root)
    let ws = (open $"($ws_root)/workspace.toml")
    let root_dir = ($ws.root_dir | path expand)
    let toml_path = $"($ws_root)/subgraphs.toml"

    let data = (open $toml_path)
    let key = if ($data | get -o subgraph | is-not-empty) { "subgraph" } else { "subgraphs" }
    let entries = ($data | get $key | where ($it.name? | is-not-empty))

    let updated = ($entries | each {|d|
        let repo_path = ($root_dir | path join ($d.repo? | default $d.name))
        if ($repo_path | path exists) {
            let sha = (^git -C $repo_path rev-parse HEAD | str trim)
            if $dry_run {
                print $"  ($d.name): ($sha)"
            }
            $d | upsert commit $sha
        } else {
            if $dry_run {
                print $"  ($d.name): (ansi yellow)repo not found, skipped(ansi reset)"
            }
            $d
        }
    })

    if $dry_run {
        return
    }

    ({($key): $updated} | to toml) | save --force $toml_path
    print $"locked ($updated | length) subgraphs into ($toml_path)"
}

def workspace-root [] {
    let cwd = (pwd)
    if ($"($cwd)/workspace.toml" | path exists) {
        $cwd
    } else {
        error make {msg: "workspace.toml not found; run from the cyber repo root"}
    }
}
