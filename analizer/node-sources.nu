#!/usr/bin/env nu
# Pin and assemble the source repositories used by the default Cyber node.

def checked [command: string, args: list<string>] {
    let result = (run-external $command ...$args | complete)
    if $result.exit_code != 0 {
        error make {msg: $"($command) failed: ($result.stderr | str trim)"}
    }
    $result.stdout | str trim
}

def metadata [root: string] {
    checked cargo [metadata --locked --format-version "1" --manifest-path ($root | path join Cargo.toml)] | from json
}

def packages [root: string] {
    let parent = ($root | path dirname)
    metadata $root | get packages | where source == null | each {|p|
        {name: $p.name, version: $p.version, manifest: ($p.manifest_path | path relative-to $parent)}
    } | sort-by manifest name
}

def clean [root: string] {
    let top = (checked git [-C $root rev-parse --show-toplevel] | path expand)
    if $top != ($root | path expand) {
        error make {msg: $"expected a repository at ($root)"}
    }
    if (checked git [-C $root status --porcelain --untracked-files=normal] | is-not-empty) {
        error make {msg: $"uncommitted source in ($root); use an isolated committed checkout"}
    }
}

def read-lock [root: string] {
    let lock = (open ($root | path join sources.lock.json))
    if $lock.schema != "cyber/node-sources/1" or $lock.profile != "default-and-tests" {
        error make {msg: "unsupported node source lock"}
    }
    let names = ($lock.repositories | get directory)
    if ($names | uniq | length) != ($names | length) {
        error make {msg: "duplicate source directory"}
    }
    for package in $lock.packages {
        if $package.manifest !~ '^[a-z][a-z0-9_-]*/([A-Za-z0-9_-]+/)*Cargo\.toml$' {
            error make {msg: "invalid local package manifest"}
        }
    }
    let expected = ($lock.packages | get manifest | each {|p| $p | path split | first }
        | where $it != "cyber" | uniq | sort)
    if ($names | sort) != $expected {
        error make {msg: "source repositories do not cover the local package set"}
    }
    for repo in $lock.repositories {
        if $repo.directory !~ '^[a-z][a-z0-9_-]*$' or $repo.directory == "cyber" {
            error make {msg: "invalid source directory"}
        }
        if $repo.revision !~ '^[0-9a-f]{40}$' or $repo.url !~ '^https://github\.com/cyberia-to/[A-Za-z0-9_.-]+\.git$' {
            error make {msg: $"invalid source pin for ($repo.directory)"}
        }
    }
    $lock
}

def verify [root: string] {
    let lock = (read-lock $root)
    clean $root
    if (open --raw ($root | path join Cargo.lock) | hash sha256) != $lock.cargo_lock_sha256 {
        error make {msg: "Cargo.lock differs from the source lock"}
    }
    let parent = ($root | path dirname)
    for repo in $lock.repositories {
        let dir = ($parent | path join $repo.directory)
        clean $dir
        let actual = (checked git [-C $dir rev-parse HEAD])
        if $actual != $repo.revision {
            error make {msg: $"source revision mismatch: ($repo.directory)"}
        }
    }
    if (packages $root) != $lock.packages {
        error make {msg: "resolved local packages differ from the source lock"}
    }
    print $"verified ($lock.repositories | length) dependency repositories and ($lock.packages | length) local packages"
}

# Capture records committed revisions. Checkout + check establishes that those
# revisions actually provide the resolved packages; capture alone is no gate.
def capture [root: string] {
    let local = (packages $root)
    let parent = ($root | path dirname)
    let names = ($local | get manifest | each {|p| $p | path split | first } | uniq | sort)
    let repositories = ($names | where $it != "cyber" | each {|name|
        let dir = ($parent | path join $name)
        {directory: $name,
         url: (checked git [-C $dir remote get-url origin]),
         revision: (checked git [-C $dir rev-parse HEAD])}
    })
    {schema: "cyber/node-sources/1", profile: "default-and-tests",
     cargo_lock_sha256: (open --raw ($root | path join Cargo.lock) | hash sha256),
     repositories: $repositories, packages: $local}
        | to json | save --force ($root | path join sources.lock.json)
    print "captured committed revisions; validate them with checkout and check"
}

def checkout [root: string, destination: string, mirror: string] {
    let lock = (read-lock $root)
    clean $root
    let destination = ($destination | path expand)
    if ($destination | path exists) {
        error make {msg: "destination must be new; an incomplete checkout is retained for inspection"}
    }
    let product = {directory: "cyber", url: (checked git [-C $root remote get-url origin]),
                   revision: (checked git [-C $root rev-parse HEAD])}
    mkdir $destination
    for repo in ($lock.repositories | append $product) {
        let dir = ($destination | path join $repo.directory)
        checked git [init --quiet $dir] | ignore
        checked git [-C $dir remote add origin $repo.url] | ignore
        let fetch_source = if ($mirror | is-empty) { $repo.url } else { $mirror | path expand | path join $repo.directory }
        checked git [-C $dir fetch --quiet --depth "1" $fetch_source $repo.revision] | ignore
        checked git [-C $dir checkout --quiet --detach FETCH_HEAD] | ignore
        print $"checked out ($repo.directory) ($repo.revision | str substring 0..7)"
    }
    verify ($destination | path join cyber)
}

def main [
    graph_path: string
    action: string = "check"
    --destination: string = ""
    --mirror-root: string = ""
] {
    let root = ($graph_path | path expand)
    match $action {
        "capture" => { capture $root }
        "check" => { verify $root }
        "checkout" => {
            if ($destination | is-empty) { error make {msg: "checkout requires --destination"} }
            checkout $root $destination $mirror_root
        }
        _ => { error make {msg: "expected capture, check or checkout"} }
    }
}
