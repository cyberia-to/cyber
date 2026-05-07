#!/usr/bin/env nu
# sync.nu — bootstrap a local cyberia-to workspace.
#
# Reads cyber/subgraphs/*.md, clones every declared repo as a sibling
# of cyber/ if missing, fetches if present. Run once after forking
# cyber to get the whole network locally; re-run any time to refresh.
#
# Usage:
#   nu scripts/sync.nu                # public + private (auth via gh)
#   nu scripts/sync.nu --public-only  # skip private repos
#   nu scripts/sync.nu --fetch        # only fetch (don't clone missing)

def main [
    --public-only   # skip private subgraphs
    --fetch         # only fetch existing checkouts; do not clone missing
    --org: string = "cyberia-to"
] {
    let ws_root = (workspace-root)
    let parent = ($ws_root | path dirname)

    let decls = (load-declarations $ws_root)
    let active = ($decls
        | where ($it.archived? | default false) != true
        | where ($it.orphan? | default false) != true
        | where {|d|
            if $public_only {
                ($d.visibility? | default "public") == "public"
            } else { true }
        }
    )

    print $"workspace: ($parent)"
    print $"declarations: (($active | length)) active subgraphs"
    print ""

    mut cloned = 0
    mut fetched = 0
    mut skipped = 0
    for d in $active {
        let target = ($parent | path join $d.repo)
        let result = if ($target | path exists) {
            if ($"($target)/.git" | path exists) {
                print $"▸ fetch ($d.name)"
                ^git -C $target fetch --quiet
                "fetched"
            } else {
                print $"  skip ($d.name) — directory exists but is not a git repo"
                "skipped"
            }
        } else if $fetch {
            print $"  skip ($d.name) — missing (re-run without --fetch to clone)"
            "skipped"
        } else {
            let url = $"https://github.com/($org)/($d.repo).git"
            print $"▸ clone ($d.name) ← ($url)"
            try {
                ^git clone --quiet $url $target
                "cloned"
            } catch {
                print $"  failed to clone ($d.name) — likely private. authenticate with `gh auth login` and retry"
                "skipped"
            }
        }
        if $result == "cloned" { $cloned = $cloned + 1 }
        if $result == "fetched" { $fetched = $fetched + 1 }
        if $result == "skipped" { $skipped = $skipped + 1 }
    }
    print ""
    print $"done. cloned=($cloned) fetched=($fetched) skipped=($skipped)"
}

def workspace-root [] {
    let cwd = (pwd)
    if ($"($cwd)/workspace.toml" | path exists) {
        $cwd
    } else {
        error make {msg: "workspace.toml not found; run from the cyber repo root"}
    }
}

def load-declarations [root: string] {
    let dir = $"($root)/subgraphs"
    if not ($dir | path exists) { return [] }
    let files = (glob $"($dir)/*.md")
    if ($files | length) == 0 { return [] }
    $files | each {|f| parse-declaration $f } | where ($it.name? | is-not-empty)
}

def parse-declaration [path: string] {
    let raw = (open --raw $path)
    let lines = ($raw | lines)
    if ($lines | first 1) != ["---"] { return {} }
    let body = ($lines | skip 1)
    let end_idx = ($body
        | enumerate
        | where item == "---"
        | get index
        | first)
    if $end_idx == null { return {} }
    let fm_lines = ($body | take $end_idx)
    mut fields = {}
    for line in $fm_lines {
        let kv = ($line | split row -n 2 ":")
        if ($kv | length) == 2 {
            let k = ($kv | get 0 | str trim)
            let v = ($kv | get 1 | str trim)
            $fields = ($fields | upsert $k $v)
        }
    }
    let archived = (($fields | get -i archived | default "false") == "true")
    let orphan = (($fields | get -i orphan | default "false") == "true")
    {
        name: ($fields | get -i name | default ""),
        repo: ($fields | get -i repo | default ($fields | get -i name | default "")),
        visibility: ($fields | get -i visibility | default "public"),
        archived: $archived,
        orphan: $orphan,
    }
}
