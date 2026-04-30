#!/usr/bin/env nu
# sync-org.nu — reconcile GitHub org state with local filesystem and declarations
# dry-run by default; --apply executes non-destructive transitions
# see SPEC.md for the full state machine

use std/log

# ---------- entry point ----------

def main [
    --apply                # apply non-destructive transitions
    --apply-renames        # apply rename plans (not yet implemented)
    --json                 # machine-readable plan output
    --only: string         # scope to one repo
] {
    if $apply_renames {
        print "--apply-renames not yet implemented; ignoring"
    }

    let ws_root = (workspace-root)
    let ws = (load-workspace $ws_root)
    let lock = (load-lock $ws_root $ws.sync.lock_file)
    let decls = (load-declarations $ws_root)
    let org_state = (fetch-org-state $ws.org $json)
    let fs_state = (scan-filesystem ($ws.root_dir | path expand))

    let plan = (compute-plan $ws $lock $decls $org_state $fs_state $only)

    if $json and not $apply {
        print ($plan | to json)
        exit (plan-exit-code $plan)
    }

    print-plan $plan $ws

    let shadow = ($plan.transitions | where event == "shadow-conflict" | length)
    if $shadow > 0 {
        print "halt: shadow conflicts detected; resolve before --apply"
        exit 10
    }

    if not $apply {
        let muts = (actionable-transitions $plan.transitions)
        if ($muts | length) == 0 {
            exit 0
        } else {
            print "dry-run complete. re-run with --apply to execute"
            exit 1
        }
    }

    print "applying..."
    let new_lock = (apply-plan $plan $ws $org_state $lock $ws_root)
    write-lock $"($ws_root)/($ws.sync.lock_file)" $new_lock
    print "applied. lock file updated."
    exit 2
}

def actionable-transitions [transitions] {
    $transitions | where event not-in ["noop" "fetch" "workspace-self" "archived-skip"]
}

# ---------- workspace loading ----------

def workspace-root [] {
    # script lives at <root>/scripts/sync-org.nu
    # run from the cyber repo root (where workspace.toml lives).
    let cwd = (pwd)
    if ($"($cwd)/workspace.toml" | path exists) {
        $cwd
    } else {
        error make {msg: "workspace.toml not found; run from the cyber repo root (or any repo containing workspace.toml)"}
    }
}

def load-workspace [root: string] {
    open $"($root)/workspace.toml"
}

def load-lock [root: string, lock_rel: string] {
    let path = $"($root)/($lock_rel)"
    if ($path | path exists) {
        open $path
    } else {
        {schema: 1, repos: {}, orphans: {}}
    }
}

def load-declarations [root: string] {
    let dir = $"($root)/subgraphs"
    if not ($dir | path exists) { return [] }
    let files = (glob $"($dir)/*.md")
    if ($files | length) == 0 { return [] }
    $files | each {|f|
        let parsed = (parse-declaration $f)
        $parsed | merge {_path: $f}
    } | where ($it.name? | is-not-empty)
}

def parse-declaration [path: string] {
    let raw = (open --raw $path)
    let split = (split-frontmatter $raw)
    if $split.frontmatter == null { return {} }
    try { $split.frontmatter | from yaml } catch { {} }
}

# ---------- github org state ----------

def fetch-org-state [org: string, quiet: bool] {
    if not $quiet { log info $"fetching org state for ($org)..." }
    let raw = (
        ^gh api $"/orgs/($org)/repos" --paginate
            --jq '.[] | {name, visibility, archived, default_branch, ssh_url, clone_url}'
    )
    $raw
        | lines
        | where ($it | str length) > 0
        | each { from json }
        | reduce --fold {} {|it, acc| $acc | upsert $it.name $it }
}

# ---------- filesystem state ----------

def scan-filesystem [root: string] {
    if not ($root | path exists) {
        return {}
    }
    ls --all $root
        | where type == dir
        | where ($it.name | path basename) not-in [".", ".."]
        | reduce --fold {} {|it, acc|
            let name = ($it.name | path basename)
            let classification = (classify-folder $it.name)
            $acc | upsert $name $classification
        }
}

def classify-folder [path: string] {
    let has_git = ($"($path)/.git" | path exists)
    if not $has_git {
        return {kind: "plain", path: $path, remote: null}
    }
    let remote = (try {
        ^git -C $path remote get-url origin | str trim
    } catch { null })
    {kind: "git", path: $path, remote: $remote}
}

# ---------- plan computation ----------

def compute-plan [ws, lock, decls, org_state, fs_state, only] {
    let org_names = ($org_state | columns)
    let policy = $ws.subgraphs.policy

    let relevant_org_names = if $only == null or ($only | is-empty) {
        $org_names
    } else {
        $org_names | where $it == $only
    }

    # for each org repo: figure out what to do
    let org_transitions = ($relevant_org_names | each {|name|
        let org_repo = ($org_state | get $name)
        let fs_entry = (try { $fs_state | get $name } catch { null })
        let decl = ($decls | where name == $name | first)
        let lock_entry = (try { $lock | get $"repos.($name)" } catch { null })

        classify-transition $name $org_repo $fs_entry $decl $lock_entry $policy
    })

    # for each local folder not in org: flag as foreign/orphan/local-only
    let fs_names = ($fs_state | columns)
    let fs_only = ($fs_names | where $it not-in $org_names | each {|name|
        let fs_entry = ($fs_state | get $name)
        classify-fs-only $name $fs_entry
    })

    {
        org:          $ws.org
        root_dir:     ($ws.root_dir | path expand)
        transitions:  $org_transitions
        fs_orphans:   $fs_only
        summary:      (summarize $org_transitions $fs_only)
    }
}

def classify-transition [name, org_repo, fs_entry, decl, lock_entry, policy] {
    # archived handling
    if $org_repo.archived {
        if $fs_entry == null {
            return {repo: $name, event: "archived-skip", detail: "archived upstream; not cloning"}
        }
        if $decl == null or ($decl.archived? | default false) != true {
            return {repo: $name, event: "archive", detail: "mark declaration archived: true"}
        }
        return {repo: $name, event: "noop", detail: "archived, already marked"}
    }

    # not cloned
    if $fs_entry == null {
        if $decl == null {
            let vis = $org_repo.visibility
            let detail = $"clone and create declaration, visibility ($vis)"
            return {repo: $name, event: "add", detail: $detail}
        }
        return {repo: $name, event: "add-clone", detail: "declaration exists; clone missing"}
    }

    # folder exists
    if $fs_entry.kind == "plain" {
        let p = $fs_entry.path
        let detail = $"plain folder ($p) shadows org repo name"
        return {repo: $name, event: "shadow-conflict", detail: $detail}
    }

    let expected_remote_ssh = $"git@github.com:cyberia-to/($name).git"
    let expected_remote_https = $"https://github.com/cyberia-to/($name).git"
    let remote_match = (
        $fs_entry.remote == $expected_remote_ssh or
        $fs_entry.remote == $expected_remote_https
    )

    if not $remote_match {
        let r = $fs_entry.remote
        let detail = $"remote ($r), expected ($expected_remote_ssh)"
        return {repo: $name, event: "foreign-or-rename", detail: $detail}
    }

    # tracked repo; check flag reconciliation
    let decl_vis = if $decl == null { null } else { $decl.visibility? }
    let vis_drift = ($decl != null and $decl_vis != $org_repo.visibility)
    let decl_arch = if $decl == null { false } else { ($decl.archived? | default false) }
    let arch_drift = ($decl_arch != $org_repo.archived)

    if $decl == null {
        return {repo: $name, event: "adopt", detail: "clone present; create declaration stub"}
    }

    if $vis_drift {
        let new_vis = $org_repo.visibility
        let detail = $"declaration ($decl_vis), org ($new_vis)"
        return {repo: $name, event: "visibility-flip", detail: $detail}
    }

    if $arch_drift {
        return {repo: $name, event: "archive-drift", detail: "declaration archive flag mismatch"}
    }

    {repo: $name, event: "fetch", detail: "tracked; would git fetch"}
}

def classify-fs-only [name, fs_entry] {
    if $fs_entry.kind == "plain" {
        return {repo: $name, kind: "local-folder", detail: "plain directory, no org counterpart"}
    }
    if $fs_entry.remote == null {
        return {repo: $name, kind: "no-remote", detail: "git repo with no remote"}
    }
    let is_org = ($fs_entry.remote | str contains "cyberia-to/")
    if $is_org {
        let r = $fs_entry.remote
        let detail = $"cyberia-to remote but not in org list ($r)"
        {repo: $name, kind: "orphan-or-rename", detail: $detail}
    } else {
        let r = $fs_entry.remote
        let detail = $"remote points elsewhere ($r)"
        {repo: $name, kind: "foreign", detail: $detail}
    }
}

def summarize [transitions, fs_only] {
    let by_event = ($transitions | group-by event | transpose event items | each {|r| {event: $r.event, count: ($r.items | length)}})
    let fs_by_kind = ($fs_only | group-by kind | transpose kind items | each {|r| {kind: $r.kind, count: ($r.items | length)}})
    {events: $by_event, fs_orphans: $fs_by_kind}
}

# ---------- output ----------

def print-plan [plan, ws] {
    print $"workspace: ($plan.org) @ ($plan.root_dir)"
    print ""

    print "org transitions:"
    print ($plan.transitions | sort-by event | select repo event detail | table --expand)
    print ""

    if ($plan.fs_orphans | length) > 0 {
        print "filesystem entries not in org:"
        print ($plan.fs_orphans | select repo kind detail | table --expand)
        print ""
    }

    print "summary:"
    print ($plan.summary.events | table)
    if ($plan.summary.fs_orphans | length) > 0 {
        print ($plan.summary.fs_orphans | table)
    }
    print ""

    let muts = ($plan.transitions | where event not-in ["noop" "fetch" "workspace-self" "archived-skip"])
    if ($muts | length) == 0 {
        print "plan: no transitions needed (clean)"
    } else {
        print $"plan: ($muts | length) transitions pending review"
    }
}

def plan-exit-code [plan] {
    let shadow = ($plan.transitions | where event == "shadow-conflict" | length)
    if $shadow > 0 { return 10 }

    let muts = ($plan.transitions | where event not-in ["noop" "fetch" "workspace-self" "archived-skip"])
    if ($muts | length) == 0 { 0 } else { 1 }
}

# ---------- apply ----------

def apply-plan [plan, ws, org_state, lock, ws_root: string] {
    let root_dir = ($ws.root_dir | path expand)
    let subgraphs_dir = $"($ws_root)/subgraphs"

    mut repo_locks = ($lock.repos? | default {})

    for t in $plan.transitions {
        match $t.event {
            "add" => {
                let name = $t.repo
                let org_repo = ($org_state | get $name)
                apply-clone $name $org_repo $root_dir
                apply-write-stub $name $org_repo $subgraphs_dir
                $repo_locks = ($repo_locks | upsert $name (lock-entry $org_repo))
            }
            "add-clone" => {
                let name = $t.repo
                let org_repo = ($org_state | get $name)
                apply-clone $name $org_repo $root_dir
                apply-reconcile-decl $name $org_repo $subgraphs_dir
                $repo_locks = ($repo_locks | upsert $name (lock-entry $org_repo))
            }
            "adopt" => {
                let name = $t.repo
                let org_repo = ($org_state | get $name)
                apply-write-stub $name $org_repo $subgraphs_dir
                $repo_locks = ($repo_locks | upsert $name (lock-entry $org_repo))
            }
            "fetch" => {
                let name = $t.repo
                let org_repo = ($org_state | get $name)
                apply-fetch $name $root_dir
                $repo_locks = ($repo_locks | upsert $name (lock-entry $org_repo))
            }
            "archive" | "archive-drift" => {
                let name = $t.repo
                let org_repo = ($org_state | get $name)
                apply-reconcile-decl $name $org_repo $subgraphs_dir
                $repo_locks = ($repo_locks | upsert $name (lock-entry $org_repo))
            }
            "visibility-flip" => {
                let name = $t.repo
                let org_repo = ($org_state | get $name)
                apply-reconcile-decl $name $org_repo $subgraphs_dir
                $repo_locks = ($repo_locks | upsert $name (lock-entry $org_repo))
            }
            "archived-skip" | "workspace-self" | "noop" => {
                print $"skip: ($t.repo) (($t.event))"
            }
            "shadow-conflict" => {
                print $"halt: shadow conflict on ($t.repo); should have exited earlier"
            }
            "foreign-or-rename" => {
                print $"skip: ($t.repo) (foreign or rename, needs --apply-renames)"
            }
            _ => {
                print $"skip: ($t.repo) unknown event ($t.event)"
            }
        }
    }

    {
        schema: 1
        org: $ws.org
        synced_at: (date now | format date "%Y-%m-%dT%H:%M:%SZ")
        repos: $repo_locks
    }
}

def apply-clone [name: string, org_repo, root_dir: string] {
    let dest = $"($root_dir)/($name)"
    if ($dest | path exists) {
        print $"  already present: ($name)"
        return
    }
    let url = $org_repo.clone_url
    print $"  clone ($name) → ($dest)"
    ^git clone --quiet $url $dest
}

def apply-fetch [name: string, root_dir: string] {
    let path = $"($root_dir)/($name)"
    if not ($path | path exists) {
        print $"  skip fetch: ($name) not cloned"
        return
    }
    print $"  fetch ($name)"
    do { ^git -C $path fetch --all --quiet --prune } | complete | ignore
}

def apply-write-stub [name: string, org_repo, subgraphs_dir: string] {
    let path = $"($subgraphs_dir)/($name).md"
    if ($path | path exists) {
        apply-reconcile-decl $name $org_repo $subgraphs_dir
        return
    }
    print $"  write declaration: subgraphs/($name).md"
    let vis = $org_repo.visibility
    let arch = ($org_repo.archived | into string)
    let content = [
        "---"
        $"name: ($name)"
        $"repo: ($name)"
        $"visibility: ($vis)"
        $"archived: ($arch)"
        "---"
        ""
        $"# ($name)"
        ""
        "<!-- auto-generated stub. replace freely with human-written context. -->"
    ] | str join "\n"
    $content | save --force $path
}

def apply-reconcile-decl [name: string, org_repo, subgraphs_dir: string] {
    let path = $"($subgraphs_dir)/($name).md"
    if not ($path | path exists) {
        apply-write-stub $name $org_repo $subgraphs_dir
        return
    }
    let raw = (open --raw $path)
    let split = (split-frontmatter $raw)
    if $split.frontmatter == null {
        print $"  skip: ($path) has no frontmatter"
        return
    }
    mut fm = $split.frontmatter
    $fm = (ensure-yaml-key $fm "visibility" $org_repo.visibility)
    $fm = (ensure-yaml-key $fm "archived" ($org_repo.archived | into string))
    let rebuilt = $"---\n($fm)\n---\n($split.body)"
    if $rebuilt != $raw {
        print $"  reconcile declaration: subgraphs/($name).md"
        $rebuilt | save --force $path
    }
}

def split-frontmatter [raw: string] {
    let lines = ($raw | lines)
    if ($lines | length) < 2 or ($lines | get 0) != "---" {
        return {frontmatter: null, body: $raw}
    }
    let end_idx = ($lines
        | enumerate
        | skip 1
        | where item == "---"
        | get index
        | first)
    if $end_idx == null { return {frontmatter: null, body: $raw} }
    let fm_lines = ($lines | skip 1 | take ($end_idx - 1))
    let body_lines = ($lines | skip ($end_idx + 1))
    {
        frontmatter: ($fm_lines | str join "\n")
        body: ($body_lines | str join "\n")
    }
}

def ensure-yaml-key [yaml_text: string, key: string, value: string] {
    let lines = ($yaml_text | lines)
    let has_key = ($lines | any {|l|
        let trimmed = ($l | str trim)
        ($trimmed | str starts-with $"($key):")
    })
    if $has_key {
        $lines | each {|line|
            let trimmed = ($line | str trim)
            if ($trimmed | str starts-with $"($key):") {
                $"($key): ($value)"
            } else {
                $line
            }
        } | str join "\n"
    } else {
        ($lines ++ [$"($key): ($value)"]) | str join "\n"
    }
}

def lock-entry [org_repo] {
    {
        visibility: $org_repo.visibility
        archived: $org_repo.archived
        default_branch: $org_repo.default_branch
        last_seen: (date now | format date "%Y-%m-%dT%H:%M:%SZ")
    }
}

def write-lock [path: string, data] {
    $data | to toml | save --force $path
}
