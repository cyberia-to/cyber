#!/usr/bin/env nu
# Exercise the release guard only in disposable Git checkouts.
def checked [command: string, args: list<string>] {
    let result = (run-external $command ...$args | complete)
    if $result.exit_code != 0 { error make {msg: $result.stderr} }
    $result.stdout | str trim
}

def rejected [root: string, message: string] {
    let result = (do { ^nu ($root | path join analizer/node-sources.nu) $root check } | complete)
    if $result.exit_code == 0 or not ($result.stderr | str contains $message) {
        error make {msg: $"expected rejection containing '($message)': ($result.stdout) ($result.stderr)"}
    }
    print $"rejected: ($message)"
}

def commit-fixture [root: string] {
    checked git [-C $root add --all] | ignore
    checked git [-C $root -c user.name=source-lock-test -c user.email=source-lock-test@example.invalid
        -c commit.gpgsign=false commit --quiet --allow-empty -m fixture] | ignore
}

def main [graph_path: string] {
    let source = ($graph_path | path expand)
    checked nu [($source | path join analizer/node-sources.nu) $source check] | print
    let scratch = (checked mktemp [-d])
    let destination = ($scratch | path join sources)
    checked nu [($source | path join analizer/node-sources.nu) $source checkout
        --destination $destination --mirror-root ($source | path dirname)] | ignore
    let root = ($destination | path join cyber)
    let product_revision = (checked git [-C $root rev-parse HEAD])
    let lock = (open ($root | path join sources.lock.json))
    let bbg = ($destination | path join bbg)
    let bbg_revision = ($lock.repositories | where directory == bbg | first | get revision)

    "dirty fixture" | save ($bbg | path join source-lock-test-untracked)
    rejected $root "uncommitted source"
    rm ($bbg | path join source-lock-test-untracked)

    commit-fixture $bbg
    rejected $root "source revision mismatch: bbg"
    checked git [-C $bbg checkout --quiet --detach $bbg_revision] | ignore

    "\n# changed fixture\n" | save --append ($root | path join Cargo.lock)
    commit-fixture $root
    rejected $root "Cargo.lock differs"
    checked git [-C $root checkout --quiet --detach $product_revision] | ignore

    $lock | update repositories {|it| $it.repositories | where directory != bbg }
        | to json | save --force ($root | path join sources.lock.json)
    commit-fixture $root
    rejected $root "do not cover the local package set"
    checked git [-C $root checkout --quiet --detach $product_revision] | ignore

    $lock | update packages.0.version "0.0.0"
        | to json | save --force ($root | path join sources.lock.json)
    commit-fixture $root
    rejected $root "resolved local packages differ"
    checked git [-C $root checkout --quiet --detach $product_revision] | ignore

    checked nu [($root | path join analizer/node-sources.nu) $root check] | print
    rm --recursive $scratch
    print "source guard: five negative checks and restored clean acceptance passed"
}
