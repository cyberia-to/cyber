#!/usr/bin/env nu
# Exercise publication failures in a disposable fixture built from real provenance.
def checked [command: string, args: list<string>] {
    let result = (run-external $command ...$args | complete)
    if $result.exit_code != 0 { error make {msg: $result.stderr} }
    $result.stdout | str trim
}

def rejected [script: string, root: string, action: string, expected: string, extra: list<string> = []] {
    let result = (do { ^nu $script $root $action ...$extra } | complete)
    if $result.exit_code == 0 or not ($result.stderr | str contains $expected) {
        error make {msg: $"expected rejection '($expected)': ($result.stdout) ($result.stderr)"}
    }
    print $"rejected: ($expected)"
}

def main [graph_path: string] {
    let source = ($graph_path | path expand)
    let script = ($source | path join analizer/node-release.nu)
    let scratch = (checked mktemp [-d])
    let root = ($scratch | path join cyber)
    # Keep the product Git identity while changing only ignored release fixtures.
    checked git [clone --quiet --no-hardlinks --no-checkout $source $root] | ignore
    checked git [-C $root checkout --quiet --detach (checked git [-C $source rev-parse HEAD])] | ignore
    cp --recursive ($source | path join dist) ($root | path join dist)
    let dist = ($root | path join dist)
    let build = (open ($dist | path join build.json))
    let inventory = (open --raw ($dist | path join soft3-dependencies.json))
    checked nu [$script $root package] | print

    $build | update sources_locked false | to json | save --force ($dist | path join build.json)
    rejected $script $root package "source-locked build"
    $build | to json | save --force ($dist | path join build.json)

    rm ($dist | path join soft3-dependencies.json)
    rejected $script $root package "soft3-dependencies.json"
    $inventory | save ($dist | path join soft3-dependencies.json)

    $inventory | from json | update components.0.revision ("0" | fill --width 40 --character "0")
        | to json | save --force ($dist | path join soft3-dependencies.json)
    rejected $script $root package "inventory differs from qualified build"
    $inventory | save --force ($dist | path join soft3-dependencies.json)

    $inventory | from json | update source_lock_sha256 "wrong" | to json
        | save --force ($dist | path join soft3-dependencies.json)
    $build | update soft3_dependencies_sha256 (open --raw ($dist | path join soft3-dependencies.json) | hash sha256)
        | to json | save --force ($dist | path join build.json)
    rejected $script $root package "source locks differ from inventory"
    $inventory | save --force ($dist | path join soft3-dependencies.json)
    $build | to json | save --force ($dist | path join build.json)

    "wrong executable" | save --force ($dist | path join cyber)
    rejected $script $root package "binary checksum mismatch"
    cp --force ($source | path join dist cyber) ($dist | path join cyber)

    rejected $script $root collect "tag must match" [--tag v0.0.0 --artifacts $scratch]
    checked nu [$script $root package] | print
    rm --recursive $scratch
    print "release guard: six negative checks and restored packaging passed"
}
