#!/usr/bin/env nu
# Adds notify-cyber.yml caller workflow to all public subgraph repos.
# Run from cyber/ root: nu scripts/add-notify-workflow.nu [--dry-run]

def parse_field [content: string, field: string] {
    let matches = ($content | lines | where {|l| $l | str starts-with $"($field):"})
    if ($matches | is-empty) { return "" }
    $matches | first | str replace $"($field):" "" | str trim
}

def main [--dry-run (-n)] {
    let base = ($env.HOME | path join "cyber")
    let caller = "name: Notify cyber

on:
  push:
    branches: [master, main]

jobs:
  notify:
    uses: cyberia-to/cyber/.github/workflows/subgraph-notify.yml@master
    secrets: inherit"

    let subgraphs = (glob subgraphs/*.md | each {|f|
        let content = (open --raw $f)
        let name = (parse_field $content "name")
        let repo = (parse_field $content "repo")
        let visibility = (parse_field $content "visibility")
        let archived = (parse_field $content "archived")
        let orphan = (parse_field $content "orphan")
        let local_only = (parse_field $content "local-only")
        {
            name: $name,
            repo: (if $repo == "" { $name } else { $repo }),
            visibility: (if $visibility == "" { "public" } else { $visibility }),
            archived: ($archived == "true"),
            orphan: ($orphan == "true"),
            local_only: ($local_only == "true"),
        }
    } | where {|s| $s.name != "" and $s.name != "cyber" and $s.visibility != "private" and $s.archived != true and $s.orphan != true and $s.local_only != true })

    for s in $subgraphs {
        let repo_path = ($base | path join $s.repo)
        let workflow_dir = ($repo_path | path join ".github" "workflows")
        let workflow_file = ($workflow_dir | path join "notify-cyber.yml")

        if not ($repo_path | path exists) {
            print $"  skip  ($s.repo): not checked out locally"
            continue
        }

        if ($workflow_file | path exists) {
            print $"  ok    ($s.repo): already has notify-cyber.yml"
            continue
        }

        if $dry_run {
            print $"  add   ($s.repo): would create ($workflow_file)"
            continue
        }

        mkdir $workflow_dir
        $caller | save --force $workflow_file

        cd $repo_path
        git add .github/workflows/notify-cyber.yml
        git commit -m $"chore: add notify-cyber workflow — trigger cyber rebuild on push

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>"
        git push origin HEAD
        print $"  done  ($s.repo)"
        cd ($base | path join "cyber")
    }

    print "\nDone."
}
